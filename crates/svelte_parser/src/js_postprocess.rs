use std::cell::Cell;
use std::mem;

use oxc_allocator::{Allocator, Box as OxcBox};
use oxc_ast::ast::{
    AccessorProperty, ArrowFunctionExpression, AssignmentTarget, BindingPattern, BlockStatement,
    CallExpression, CatchParameter, ChainElement, ChainExpression, Class, ClassBody, ClassElement,
    DoWhileStatement, EmptyStatement, Expression, ForInStatement, ForOfStatement, ForStatement,
    FormalParameter, FormalParameterRest, FormalParameters, Function, FunctionBody, FunctionType,
    IfStatement, ImportDeclarationSpecifier, MethodDefinition, MethodDefinitionType, NewExpression,
    NullLiteral, Program, PropertyDefinition, PropertyDefinitionType, SimpleAssignmentTarget,
    Statement, StaticBlock, TSModuleBlock, TSModuleDeclaration, TSType, TSTypeAnnotation,
    TSTypeParameterDeclaration, TSTypeParameterInstantiation, TaggedTemplateExpression,
    VariableDeclarator, WhileStatement, match_member_expression,
};
use oxc_ast_visit::{VisitMut, walk_mut};
use oxc_span::{GetSpan, SPAN, Span};
use oxc_syntax::node::NodeId;
use oxc_syntax::scope::ScopeFlags;

pub(crate) struct JsPostprocessor<'a> {
    alloc: &'a Allocator,
    delta: i64,
    strip_ts: bool,
}

impl<'a> JsPostprocessor<'a> {
    fn new(alloc: &'a Allocator, delta: i64, strip_ts: bool) -> Self {
        Self {
            alloc,
            delta,
            strip_ts,
        }
    }

    fn dummy_expr(&self) -> Expression<'a> {
        Expression::NullLiteral(OxcBox::new_in(
            NullLiteral {
                span: SPAN,
                node_id: Cell::new(NodeId::DUMMY),
            },
            self.alloc,
        ))
    }

    fn take_expr(&self, expr: &mut Expression<'a>) -> Expression<'a> {
        mem::replace(expr, self.dummy_expr())
    }

    fn unwrap_ts_inner(&self, expr: &mut Expression<'a>) {
        loop {
            let taken = self.take_expr(expr);
            let inner = match taken {
                Expression::TSAsExpression(b) => b.unbox().expression,
                Expression::TSSatisfiesExpression(b) => b.unbox().expression,
                Expression::TSNonNullExpression(b) => b.unbox().expression,
                Expression::TSTypeAssertion(b) => b.unbox().expression,
                Expression::TSInstantiationExpression(b) => b.unbox().expression,
                other => {
                    *expr = other;
                    return;
                }
            };
            *expr = inner;
        }
    }

    fn strip_expression_wrappers(&self, node: &mut Expression<'a>) {
        if node.is_typescript_syntax() {
            self.unwrap_ts_inner(node);
            return;
        }
        if let Expression::ParenthesizedExpression(paren) = node
            && paren.expression.is_typescript_syntax()
        {
            self.unwrap_ts_inner(&mut paren.expression);
            if matches!(paren.expression, Expression::ChainExpression(_)) {
                return;
            }
            let inner = self.take_expr(&mut paren.expression);
            *node = inner;
        }
    }

    fn strip_chain_element_wrappers(&self, node: &mut ChainElement<'a>) {
        if let ChainElement::TSNonNullExpression(ts) = node {
            let inner_expr = self.take_expr(ts.expression.get_inner_expression_mut());
            *node = match inner_expr {
                Expression::CallExpression(c) => ChainElement::CallExpression(c),
                expr @ match_member_expression!(Expression) => {
                    ChainElement::from(expr.into_member_expression())
                }
                _ => {
                    unreachable!("TSNonNullExpression inside ChainExpression must wrap member/call")
                }
            };
        }
    }

    fn flatten_chain_spine(&self, element: &mut ChainElement<'a>) {
        match element {
            ChainElement::StaticMemberExpression(m) => self.flatten_chain_object(&mut m.object),
            ChainElement::ComputedMemberExpression(m) => self.flatten_chain_object(&mut m.object),
            ChainElement::PrivateFieldExpression(m) => self.flatten_chain_object(&mut m.object),
            ChainElement::CallExpression(c) => self.flatten_chain_object(&mut c.callee),
            ChainElement::TSNonNullExpression(_) => {}
        }
    }

    fn flatten_chain_object(&self, expr: &mut Expression<'a>) {
        if let Expression::ParenthesizedExpression(paren) = expr
            && matches!(paren.expression, Expression::ChainExpression(_))
        {
            let inner = self.take_expr(&mut paren.expression);
            *expr = inner;
        }
        match expr {
            Expression::StaticMemberExpression(m) => self.flatten_chain_object(&mut m.object),
            Expression::ComputedMemberExpression(m) => self.flatten_chain_object(&mut m.object),
            Expression::PrivateFieldExpression(m) => self.flatten_chain_object(&mut m.object),
            Expression::CallExpression(c) => self.flatten_chain_object(&mut c.callee),
            _ => {}
        }
    }

    fn strip_simple_assignment_target(&self, node: &mut SimpleAssignmentTarget<'a>) {
        let Some(expr) = node.get_expression_mut() else {
            return;
        };
        let inner = self.take_expr(expr.get_inner_expression_mut());
        match inner {
            Expression::Identifier(id) => {
                *node = SimpleAssignmentTarget::AssignmentTargetIdentifier(id);
            }
            expr @ match_member_expression!(Expression) => {
                *node = SimpleAssignmentTarget::from(expr.into_member_expression());
            }
            _ => {}
        }
    }

    fn strip_assignment_target(&self, node: &mut AssignmentTarget<'a>) {
        let Some(expr) = node.get_expression_mut() else {
            return;
        };
        let inner = self.take_expr(expr.get_inner_expression_mut());
        match inner {
            Expression::Identifier(id) => {
                *node = AssignmentTarget::AssignmentTargetIdentifier(id);
            }
            expr @ match_member_expression!(Expression) => {
                *node = AssignmentTarget::from(expr.into_member_expression());
            }
            _ => {}
        }
    }

    fn replace_ts_only_body_with_empty(&self, stmt: &mut Statement<'a>) {
        if is_pure_ts_type_statement(stmt) {
            let span = stmt.span();
            *stmt = Statement::EmptyStatement(OxcBox::new_in(
                EmptyStatement {
                    span,
                    node_id: Cell::new(NodeId::DUMMY),
                },
                self.alloc,
            ));
        }
    }

    fn filter_statements(&self, stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>) {
        for stmt in stmts.iter_mut() {
            match stmt {
                Statement::ImportDeclaration(import) => {
                    if let Some(specs) = &mut import.specifiers {
                        specs.retain(|spec| {
                            !matches!(spec, ImportDeclarationSpecifier::ImportSpecifier(s) if s.import_kind.is_type())
                        });
                    }
                }
                Statement::ExportNamedDeclaration(export) if export.declaration.is_none() => {
                    export.specifiers.retain(|spec| !spec.export_kind.is_type());
                }
                _ => {}
            }
        }

        stmts.retain(|stmt| {
            if is_pure_ts_type_statement(stmt) {
                return false;
            }
            match stmt {
                Statement::ImportDeclaration(import) => {
                    if import.import_kind.is_type() {
                        return false;
                    }
                    import.specifiers.as_ref().is_none_or(|s| !s.is_empty())
                }
                Statement::ExportNamedDeclaration(export) => {
                    if export.export_kind.is_type() {
                        return false;
                    }
                    export.declaration.is_some() || !export.specifiers.is_empty()
                }
                _ => true,
            }
        });
    }
}

fn is_pure_ts_type_statement(stmt: &Statement<'_>) -> bool {
    use oxc_ast::ast::Declaration;
    match stmt {
        Statement::TSTypeAliasDeclaration(_)
        | Statement::TSInterfaceDeclaration(_)
        | Statement::TSImportEqualsDeclaration(_)
        | Statement::TSExportAssignment(_)
        | Statement::TSNamespaceExportDeclaration(_) => true,
        Statement::TSModuleDeclaration(m) => !ts_module_has_runtime_node(m),
        Statement::TSGlobalDeclaration(g) => !ts_module_block_has_runtime_node(&g.body),
        Statement::VariableDeclaration(d) => d.declare,
        Statement::FunctionDeclaration(f) => is_pure_ts_function(f),
        Statement::ClassDeclaration(c) => c.declare,
        Statement::ExportNamedDeclaration(e) => match &e.declaration {
            Some(Declaration::TSTypeAliasDeclaration(_))
            | Some(Declaration::TSInterfaceDeclaration(_))
            | Some(Declaration::TSImportEqualsDeclaration(_)) => true,
            Some(Declaration::TSModuleDeclaration(m)) => !ts_module_has_runtime_node(m),
            Some(Declaration::TSGlobalDeclaration(g)) => !ts_module_block_has_runtime_node(&g.body),
            Some(Declaration::VariableDeclaration(v)) => v.declare,
            Some(Declaration::FunctionDeclaration(f)) => is_pure_ts_function(f),
            Some(Declaration::ClassDeclaration(c)) => c.declare,
            _ => false,
        },
        _ => false,
    }
}

fn is_pure_ts_function(func: &Function<'_>) -> bool {
    func.declare || func.r#type == FunctionType::TSDeclareFunction
}

fn ts_module_has_runtime_node(decl: &TSModuleDeclaration<'_>) -> bool {
    use oxc_ast::ast::TSModuleDeclarationBody;
    let Some(body) = &decl.body else {
        return false;
    };
    match body {
        TSModuleDeclarationBody::TSModuleDeclaration(nested) => ts_module_has_runtime_node(nested),
        TSModuleDeclarationBody::TSModuleBlock(block) => ts_module_block_has_runtime_node(block),
    }
}

fn ts_module_block_has_runtime_node(block: &TSModuleBlock<'_>) -> bool {
    block
        .body
        .iter()
        .any(|stmt| !is_pure_ts_type_statement(stmt))
}

impl<'a> VisitMut<'a> for JsPostprocessor<'a> {
    fn visit_span(&mut self, it: &mut Span) {
        if self.delta != 0 {
            it.start = shift(it.start, self.delta);
            it.end = shift(it.end, self.delta);
        }
    }

    fn visit_program(&mut self, it: &mut Program<'a>) {
        if self.strip_ts {
            self.filter_statements(&mut it.body);
        }
        walk_mut::walk_program(self, it);
    }

    fn visit_block_statement(&mut self, it: &mut BlockStatement<'a>) {
        if self.strip_ts {
            self.filter_statements(&mut it.body);
        }
        walk_mut::walk_block_statement(self, it);
    }

    fn visit_function_body(&mut self, it: &mut FunctionBody<'a>) {
        if self.strip_ts {
            self.filter_statements(&mut it.statements);
        }
        walk_mut::walk_function_body(self, it);
    }

    fn visit_static_block(&mut self, it: &mut StaticBlock<'a>) {
        if self.strip_ts {
            self.filter_statements(&mut it.body);
        }
        walk_mut::walk_static_block(self, it);
    }

    fn visit_expression(&mut self, it: &mut Expression<'a>) {
        if self.strip_ts {
            self.strip_expression_wrappers(it);
        }
        walk_mut::walk_expression(self, it);
    }

    fn visit_chain_element(&mut self, it: &mut ChainElement<'a>) {
        if self.strip_ts {
            self.strip_chain_element_wrappers(it);
        }
        walk_mut::walk_chain_element(self, it);
    }

    fn visit_chain_expression(&mut self, it: &mut ChainExpression<'a>) {
        walk_mut::walk_chain_expression(self, it);
        self.flatten_chain_spine(&mut it.expression);
    }

    fn visit_simple_assignment_target(&mut self, it: &mut SimpleAssignmentTarget<'a>) {
        if self.strip_ts {
            self.strip_simple_assignment_target(it);
        }
        walk_mut::walk_simple_assignment_target(self, it);
    }

    fn visit_assignment_target(&mut self, it: &mut AssignmentTarget<'a>) {
        if self.strip_ts {
            self.strip_assignment_target(it);
        }
        walk_mut::walk_assignment_target(self, it);
    }

    fn visit_class_body(&mut self, it: &mut ClassBody<'a>) {
        if self.strip_ts {
            it.body.retain(|member| match member {
                ClassElement::PropertyDefinition(prop) => {
                    !prop.declare
                        && prop.r#type != PropertyDefinitionType::TSAbstractPropertyDefinition
                }
                ClassElement::MethodDefinition(method) => {
                    method.r#type != MethodDefinitionType::TSAbstractMethodDefinition
                }
                ClassElement::TSIndexSignature(_) => false,
                _ => true,
            });
        }
        walk_mut::walk_class_body(self, it);
    }

    fn visit_class(&mut self, it: &mut Class<'a>) {
        if self.strip_ts {
            it.type_parameters = None;
            it.super_type_arguments = None;
            it.implements.clear();
            it.r#abstract = false;
        }
        walk_mut::walk_class(self, it);
    }

    fn visit_function(&mut self, it: &mut Function<'a>, flags: ScopeFlags) {
        if self.strip_ts {
            it.type_parameters = None;
            it.return_type = None;
            it.this_param = None;
        }
        walk_mut::walk_function(self, it, flags);
    }

    fn visit_arrow_function_expression(&mut self, it: &mut ArrowFunctionExpression<'a>) {
        if self.strip_ts {
            it.type_parameters = None;
            it.return_type = None;
        }
        walk_mut::walk_arrow_function_expression(self, it);
    }

    fn visit_formal_parameter(&mut self, it: &mut FormalParameter<'a>) {
        if self.strip_ts {
            it.type_annotation = None;
            it.optional = false;
        }
        walk_mut::walk_formal_parameter(self, it);
    }

    fn visit_catch_parameter(&mut self, it: &mut CatchParameter<'a>) {
        if self.strip_ts {
            it.type_annotation = None;
        }
        walk_mut::walk_catch_parameter(self, it);
    }

    fn visit_formal_parameter_rest(&mut self, it: &mut FormalParameterRest<'a>) {
        if self.strip_ts {
            it.type_annotation = None;
        }
        walk_mut::walk_formal_parameter_rest(self, it);
    }

    fn visit_call_expression(&mut self, it: &mut CallExpression<'a>) {
        if self.strip_ts {
            it.type_arguments = None;
        }
        walk_mut::walk_call_expression(self, it);
    }

    fn visit_new_expression(&mut self, it: &mut NewExpression<'a>) {
        if self.strip_ts {
            it.type_arguments = None;
        }
        walk_mut::walk_new_expression(self, it);
    }

    fn visit_tagged_template_expression(&mut self, it: &mut TaggedTemplateExpression<'a>) {
        if self.strip_ts {
            it.type_arguments = None;
        }
        walk_mut::walk_tagged_template_expression(self, it);
    }

    fn visit_property_definition(&mut self, it: &mut PropertyDefinition<'a>) {
        if self.strip_ts {
            it.type_annotation = None;
            it.accessibility = None;
            it.readonly = false;
            it.r#override = false;
            it.optional = false;
            it.definite = false;
        }
        walk_mut::walk_property_definition(self, it);
    }

    fn visit_accessor_property(&mut self, it: &mut AccessorProperty<'a>) {
        if self.strip_ts {
            it.type_annotation = None;
            it.accessibility = None;
            it.r#override = false;
            it.definite = false;
        }
        walk_mut::walk_accessor_property(self, it);
    }

    fn visit_method_definition(&mut self, it: &mut MethodDefinition<'a>) {
        if self.strip_ts {
            it.accessibility = None;
            it.r#override = false;
            it.optional = false;
        }
        walk_mut::walk_method_definition(self, it);
    }

    fn visit_variable_declarator(&mut self, it: &mut VariableDeclarator<'a>) {
        if self.strip_ts {
            it.type_annotation = None;
            it.definite = false;
        }
        walk_mut::walk_variable_declarator(self, it);
    }

    fn visit_if_statement(&mut self, it: &mut IfStatement<'a>) {
        if self.strip_ts {
            self.replace_ts_only_body_with_empty(&mut it.consequent);
            if let Some(alt) = it.alternate.as_mut() {
                self.replace_ts_only_body_with_empty(alt);
            }
        }
        walk_mut::walk_if_statement(self, it);
    }

    fn visit_for_statement(&mut self, it: &mut ForStatement<'a>) {
        if self.strip_ts {
            self.replace_ts_only_body_with_empty(&mut it.body);
        }
        walk_mut::walk_for_statement(self, it);
    }

    fn visit_for_in_statement(&mut self, it: &mut ForInStatement<'a>) {
        if self.strip_ts {
            self.replace_ts_only_body_with_empty(&mut it.body);
        }
        walk_mut::walk_for_in_statement(self, it);
    }

    fn visit_for_of_statement(&mut self, it: &mut ForOfStatement<'a>) {
        if self.strip_ts {
            self.replace_ts_only_body_with_empty(&mut it.body);
        }
        walk_mut::walk_for_of_statement(self, it);
    }

    fn visit_while_statement(&mut self, it: &mut WhileStatement<'a>) {
        if self.strip_ts {
            self.replace_ts_only_body_with_empty(&mut it.body);
        }
        walk_mut::walk_while_statement(self, it);
    }

    fn visit_do_while_statement(&mut self, it: &mut DoWhileStatement<'a>) {
        if self.strip_ts {
            self.replace_ts_only_body_with_empty(&mut it.body);
        }
        walk_mut::walk_do_while_statement(self, it);
    }

    fn visit_ts_type(&mut self, it: &mut TSType<'a>) {
        if !self.strip_ts {
            walk_mut::walk_ts_type(self, it);
        }
    }

    fn visit_ts_type_annotation(&mut self, it: &mut TSTypeAnnotation<'a>) {
        if !self.strip_ts {
            walk_mut::walk_ts_type_annotation(self, it);
        }
    }

    fn visit_ts_type_parameter_declaration(&mut self, it: &mut TSTypeParameterDeclaration<'a>) {
        if !self.strip_ts {
            walk_mut::walk_ts_type_parameter_declaration(self, it);
        }
    }

    fn visit_ts_type_parameter_instantiation(&mut self, it: &mut TSTypeParameterInstantiation<'a>) {
        if !self.strip_ts {
            walk_mut::walk_ts_type_parameter_instantiation(self, it);
        }
    }
}

fn shift(value: u32, delta: i64) -> u32 {
    let shifted = value as i64 + delta;
    debug_assert!(
        shifted >= 0 && shifted <= u32::MAX as i64,
        "span shift out of range: value={value} delta={delta} shifted={shifted}",
    );
    shifted as u32
}

fn shift_comments(comments: &mut [oxc_ast::Comment], delta: i64) {
    for comment in comments {
        comment.span.start = shift(comment.span.start, delta);
        comment.span.end = shift(comment.span.end, delta);
        comment.attached_to = shift(comment.attached_to, delta);
    }
}

pub(crate) fn process_program<'a>(
    alloc: &'a Allocator,
    program: &mut Program<'a>,
    delta: i64,
    strip_ts: bool,
) {
    normalize_empty_import_specifiers(program);
    if delta == 0 && !strip_ts {
        return;
    }
    let mut v = JsPostprocessor::new(alloc, delta, strip_ts);
    v.visit_program(program);
    if delta != 0 {
        shift_comments(&mut program.comments, delta);
    }
    if strip_ts {
        relocate_orphaned_comments(program);
    }
}

fn normalize_empty_import_specifiers(program: &mut Program<'_>) {
    for stmt in &mut program.body {
        let Statement::ImportDeclaration(import) = stmt else {
            continue;
        };
        if import.import_kind.is_type() {
            continue;
        }
        if matches!(&import.specifiers, Some(specs) if specs.is_empty()) {
            import.specifiers = None;
        }
    }
}

pub(crate) fn process_expression<'a>(
    alloc: &'a Allocator,
    expr: &mut Expression<'a>,
    delta: i64,
    strip_ts: bool,
) {
    if delta == 0 && !strip_ts {
        return;
    }
    let mut v = JsPostprocessor::new(alloc, delta, strip_ts);
    v.visit_expression(expr);
}

pub(crate) fn process_statement<'a>(
    alloc: &'a Allocator,
    stmt: &mut Statement<'a>,
    delta: i64,
    strip_ts: bool,
) {
    if delta == 0 && !strip_ts {
        return;
    }
    let mut v = JsPostprocessor::new(alloc, delta, strip_ts);
    v.visit_statement(stmt);
}

pub(crate) fn process_binding_pattern<'a>(
    alloc: &'a Allocator,
    pat: &mut BindingPattern<'a>,
    delta: i64,
    strip_ts: bool,
) {
    if delta == 0 && !strip_ts {
        return;
    }
    let mut v = JsPostprocessor::new(alloc, delta, strip_ts);
    v.visit_binding_pattern(pat);
}

pub(crate) fn process_formal_parameters<'a>(
    alloc: &'a Allocator,
    params: &mut FormalParameters<'a>,
    delta: i64,
    strip_ts: bool,
) {
    if delta == 0 && !strip_ts {
        return;
    }
    let mut v = JsPostprocessor::new(alloc, delta, strip_ts);
    v.visit_formal_parameters(params);
}

pub(crate) fn wrapper_delta(absolute_start: u32, leading_ws: usize, prefix_len: i64) -> i64 {
    absolute_start as i64 + leading_ws as i64 - prefix_len
}

fn relocate_orphaned_comments(program: &mut Program<'_>) {
    let mut stmt_spans: Vec<(u32, u32)> = program
        .body
        .iter()
        .map(|s| (s.span().start, s.span().end))
        .collect();
    stmt_spans.sort_unstable_by_key(|&(start, _)| start);
    let stmt_starts: Vec<u32> = stmt_spans.iter().map(|&(start, _)| start).collect();

    program.comments.retain_mut(|comment| {
        if stmt_starts.binary_search(&comment.attached_to).is_ok() {
            return true;
        }
        let container = stmt_starts.partition_point(|&start| start <= comment.span.start);
        let nested = container > 0 && stmt_spans[container - 1].1 >= comment.span.end;
        if nested {
            return false;
        }
        let next = stmt_starts.partition_point(|&start| start < comment.span.end);
        if next < stmt_starts.len() {
            comment.attached_to = stmt_starts[next];
            true
        } else {
            false
        }
    });
}
