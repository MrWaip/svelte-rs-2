use rustc_hash::FxHashSet;

use oxc_ast::ast::{
    ArrowFunctionExpression, ClassElement, Expression, FunctionBody,
    ImportDeclarationSpecifier, MethodDefinitionType,
    PropertyDefinitionType, Statement, VariableDeclarator,
};
use oxc_span::GetSpan;
use oxc_traverse::{Traverse, TraverseCtx};

use svelte_parser::RuneKind;

use crate::builder::{Arg, Builder};

use super::{FunctionInfo, PropKind, ScriptTransformer};

/// Post-traverse: wrap `$.derived(expr)` → `$.derived(() => expr)` for $derived runes.
pub(super) fn wrap_derived_thunks<'a>(
    b: &Builder<'a>,
    program: &mut oxc_ast::ast::Program<'a>,
    pending: &FxHashSet<oxc_semantic::SymbolId>,
) {
    use oxc_ast::ast::Statement;
    for stmt in program.body.iter_mut() {
        if let Statement::VariableDeclaration(decl) = stmt {
            for declarator in decl.declarations.iter_mut() {
                let sym_id = match &declarator.id {
                    oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                        match id.symbol_id.get() {
                            Some(s) => s,
                            None => continue,
                        }
                    }
                    _ => continue,
                };
                if !pending.contains(&sym_id) {
                    continue;
                }
                if let Some(Expression::CallExpression(call)) = &mut declarator.init {
                    if !call.arguments.is_empty() {
                        let mut dummy = oxc_ast::ast::Argument::from(b.cheap_expr());
                        std::mem::swap(&mut call.arguments[0], &mut dummy);
                        let arg_expr = dummy.into_expression();
                        let thunk = b.thunk(arg_expr);
                        call.arguments[0] = oxc_ast::ast::Argument::from(thunk);
                    }
                }
            }
        }
    }
}

/// Wrap a non-simple default expression for lazy evaluation.
pub(super) fn wrap_lazy<'a>(b: &Builder<'a>, expr: Expression<'a>) -> Expression<'a> {
    // Zero-arg call foo() → use callee directly (already lazy)
    if let Expression::CallExpression(call) = &expr {
        if call.arguments.is_empty() {
            if let Expression::Identifier(_) = &call.callee {
                return b.clone_expr(&call.callee);
            }
        }
    }
    // Otherwise wrap: () => expr
    b.arrow_expr(b.no_params(), [b.expr_stmt(expr)])
}

/// Check if an expression is a `$inspect(...)`, `$inspect(...).with(...)`, or `$inspect.trace(...)` call.
fn is_inspect_call(expr: &Expression) -> bool {
    match expr {
        Expression::CallExpression(call) => {
            if let Expression::Identifier(id) = &call.callee {
                if id.name.as_str() == "$inspect" {
                    return true;
                }
            }
            if let Expression::StaticMemberExpression(member) = &call.callee {
                // $inspect(...).with(...)
                if member.property.name.as_str() == "with" {
                    if let Expression::CallExpression(inner) = &member.object {
                        if let Expression::Identifier(id) = &inner.callee {
                            return id.name.as_str() == "$inspect";
                        }
                    }
                }
                // $inspect.trace(...)
                if member.property.name.as_str() == "trace" {
                    if let Expression::Identifier(id) = &member.object {
                        return id.name.as_str() == "$inspect";
                    }
                }
            }
            false
        }
        _ => false,
    }
}

/// Check if an expression is specifically a `$inspect.trace(...)` call.
fn is_inspect_trace_call(expr: &Expression) -> bool {
    if let Expression::CallExpression(call) = expr {
        if let Expression::StaticMemberExpression(member) = &call.callee {
            if member.property.name.as_str() == "trace" {
                if let Expression::Identifier(id) = &member.object {
                    return id.name.as_str() == "$inspect";
                }
            }
        }
    }
    false
}

/// Sanitize a filename for use in trace labels by inserting a zero-width space
/// after each `/` to prevent devtools from treating it as a clickable link.
pub(crate) fn sanitize_location(filename: &str) -> String {
    filename.replace('/', "/\u{200b}")
}

/// Compute 1-based line and column from source text and byte offset.
pub(crate) fn compute_line_col(source: &str, offset: u32) -> (usize, usize) {
    let offset = offset as usize;
    let bytes = source.as_bytes();
    let mut line = 1;
    let mut col = 0;
    for i in 0..offset.min(bytes.len()) {
        if bytes[i] == b'\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

impl<'a> Traverse<'a, ()> for ScriptTransformer<'_, 'a> {
    fn enter_class_body(
        &mut self,
        node: &mut oxc_ast::ast::ClassBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let info = self.scan_class_state_fields(node);
        self.class_state_stack.push(info);
    }

    fn exit_class_body(
        &mut self,
        node: &mut oxc_ast::ast::ClassBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.body.retain(|member| match member {
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

        let Some(info) = self.class_state_stack.pop() else { return };
        if info.fields.is_empty() {
            return;
        }
        self.rewrite_class_body(node, &info);
    }

    fn enter_function(
        &mut self,
        node: &mut oxc_ast::ast::Function<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_parameters = None;
            node.return_type = None;
            node.this_param = None;
        }
        let name = node.id.as_ref()
            .map(|id| id.name.to_string())
            .or_else(|| self.next_arrow_name.take());
        self.function_info_stack.push(FunctionInfo {
            is_async: node.r#async,
            name,
            span_start: node.span.start,
        });
    }

    fn exit_function(
        &mut self,
        _node: &mut oxc_ast::ast::Function<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.function_info_stack.pop();
    }

    fn enter_arrow_function_expression(
        &mut self,
        node: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_parameters = None;
            node.return_type = None;
        }
        let name = self.next_arrow_name.take();
        self.function_info_stack.push(FunctionInfo {
            is_async: node.r#async,
            name,
            span_start: node.span.start,
        });
    }

    fn exit_arrow_function_expression(
        &mut self,
        _node: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.function_info_stack.pop();
    }

    fn exit_function_body(
        &mut self,
        body: &mut FunctionBody<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if !self.dev {
            return;
        }
        let Some(Statement::ExpressionStatement(es)) = body.statements.first() else {
            return;
        };
        if !is_inspect_trace_call(&es.expression) {
            return;
        }

        let info = self.function_info_stack.last()
            .unwrap_or_else(|| panic!("$inspect.trace() outside function"));

        // Extract explicit label argument if present
        let trace_stmt = body.statements.remove(0);
        let Statement::ExpressionStatement(es) = trace_stmt else { unreachable!() };
        let Expression::CallExpression(call) = es.unbox().expression else { unreachable!() };
        let mut call = call.unbox();

        let label_expr = if !call.arguments.is_empty() {
            // Explicit label: $inspect.trace("custom") → use the argument directly
            let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
            std::mem::swap(&mut call.arguments[0], &mut dummy);
            dummy.into_expression()
        } else {
            // Auto-label: "funcName (line:col)"
            let func_name = info.name.as_deref().unwrap_or("trace");
            let full_offset = self.script_content_start + info.span_start;
            let (line, col) = compute_line_col(self.component_source, full_offset);
            let sanitized = sanitize_location(self.filename);
            let label = format!("{func_name} ({sanitized}:{line}:{col})");
            self.b.str_expr(&label)
        };
        let label_thunk = self.b.thunk(label_expr);

        let is_async = info.is_async;

        // Take remaining statements, build body thunk
        let remaining: Vec<Statement<'a>> = body.statements.drain(..).collect();
        let body_thunk = if is_async {
            self.b.async_thunk_block(remaining)
        } else {
            self.b.thunk_block(remaining)
        };

        // Build: return [await] $.trace(label, bodyThunk)
        let trace_call = self.b.call_expr("$.trace", [
            Arg::Expr(label_thunk),
            Arg::Expr(body_thunk),
        ]);
        let return_expr = if is_async {
            self.b.await_expr(trace_call)
        } else {
            trace_call
        };
        body.statements.push(self.b.return_stmt(return_expr));

        self.has_tracing = true;
    }

    fn exit_statements(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Strip TypeScript declarations and type-only imports/exports
        if self.is_ts {
            // Remove individual type specifiers from imports/exports
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

            // Remove entire TS-only statements
            stmts.retain(|stmt| match stmt {
                Statement::TSTypeAliasDeclaration(_)
                | Statement::TSInterfaceDeclaration(_)
                | Statement::TSModuleDeclaration(_)
                | Statement::TSEnumDeclaration(_) => false,
                Statement::VariableDeclaration(decl) if decl.declare => false,
                Statement::FunctionDeclaration(func) if func.declare => false,
                Statement::ClassDeclaration(class) if class.declare => false,
                Statement::ImportDeclaration(import) if import.import_kind.is_type() => false,
                Statement::ExportNamedDeclaration(export) if export.export_kind.is_type() => false,
                Statement::ExportAllDeclaration(export) if export.export_kind.is_type() => false,
                // Remove imports/exports left with no specifiers after type-specifier filtering
                Statement::ImportDeclaration(import) => {
                    import.specifiers.as_ref().is_none_or(|s| !s.is_empty())
                }
                Statement::ExportNamedDeclaration(export) => {
                    export.declaration.is_some() || !export.specifiers.is_empty()
                }
                _ => true,
            });
        }

        // Strip `export` keyword: ExportNamedDeclaration → inner declaration
        // (only for component scripts; module compilation preserves exports)
        if self.strip_exports {
            let mut i = 0;
            while i < stmts.len() {
                if let Statement::ExportNamedDeclaration(_) = &stmts[i] {
                    let stmt = stmts.remove(i);
                    if let Statement::ExportNamedDeclaration(export) = stmt {
                        if let Some(decl) = export.unbox().declaration {
                            stmts.insert(i, Statement::from(decl));
                            i += 1;
                        }
                        // else: `export { x }` form — just remove
                    }
                } else {
                    i += 1;
                }
            }
        }

        // Prod strip: $inspect.trace() → remove entirely; $inspect()/$inspect().with() → 2 EmptyStatements
        if !self.dev {
            let mut i = 0;
            while i < stmts.len() {
                if let Statement::ExpressionStatement(es) = &stmts[i] {
                    if is_inspect_trace_call(&es.expression) {
                        stmts.remove(i);
                        continue;
                    }
                    if is_inspect_call(&es.expression) {
                        stmts[i] = Statement::EmptyStatement(self.b.ast.alloc_empty_statement(oxc_span::SPAN));
                        stmts.insert(i + 1, Statement::EmptyStatement(self.b.ast.alloc_empty_statement(oxc_span::SPAN)));
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }
        }

        // Strip $props.id() declarations (regenerated at top of component fn_body)
        stmts.retain(|stmt| {
            if let Statement::VariableDeclaration(decl) = stmt {
                if Self::is_props_id_declaration(decl) {
                    return false;
                }
            }
            true
        });

        // Expand destructured $state/$state.raw declarations
        self.expand_state_destructuring(stmts);

        // Replace $props() destructuring
        if self.props_gen.is_none() {
            return;
        }

        let mut idx = None;
        for (j, stmt) in stmts.iter().enumerate() {
            if let Statement::VariableDeclaration(decl) = stmt {
                if Self::is_props_declaration(decl) {
                    idx = Some(j);
                    break;
                }
            }
        }

        let Some(j) = idx else { return };

        let replacement = self.gen_props_statements();
        stmts.remove(j);
        for (k, stmt) in replacement.into_iter().enumerate() {
            stmts.insert(j + k, stmt);
        }
    }

    fn enter_formal_parameter(
        &mut self,
        node: &mut oxc_ast::ast::FormalParameter<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.readonly = false;
            node.r#override = false;
        }
    }

    fn enter_catch_parameter(
        &mut self,
        node: &mut oxc_ast::ast::CatchParameter<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
        }
    }

    fn enter_call_expression(
        &mut self,
        node: &mut oxc_ast::ast::CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_arguments = None;
        }
        // Capture callee text for arrow/function arguments (used by $inspect.trace auto-label).
        // e.g. foo(() => { $inspect.trace(); }) → label "foo(...)"
        let has_fn_arg = node.arguments.iter().any(|arg| {
            matches!(arg, oxc_ast::ast::Argument::ArrowFunctionExpression(_)
                | oxc_ast::ast::Argument::FunctionExpression(_))
        });
        if has_fn_arg {
            let start = (self.script_content_start + node.callee.span().start) as usize;
            let end = (self.script_content_start + node.callee.span().end) as usize;
            if end <= self.component_source.len() {
                let callee_text = &self.component_source[start..end];
                self.next_arrow_name = Some(format!("{callee_text}(...)"));
            }
        }
    }

    fn enter_new_expression(
        &mut self,
        node: &mut oxc_ast::ast::NewExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_arguments = None;
        }
    }

    fn enter_tagged_template_expression(
        &mut self,
        node: &mut oxc_ast::ast::TaggedTemplateExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_arguments = None;
        }
    }

    fn enter_class(
        &mut self,
        node: &mut oxc_ast::ast::Class<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_parameters = None;
            node.super_type_arguments = None;
            node.implements.clear();
            node.r#abstract = false;
        }
    }

    fn enter_property_definition(
        &mut self,
        node: &mut oxc_ast::ast::PropertyDefinition<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.readonly = false;
            node.r#override = false;
            node.optional = false;
            node.definite = false;
        }
    }

    fn enter_accessor_property(
        &mut self,
        node: &mut oxc_ast::ast::AccessorProperty<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.accessibility = None;
            node.r#override = false;
            node.definite = false;
        }
    }

    fn enter_object_property(
        &mut self,
        node: &mut oxc_ast::ast::ObjectProperty<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Capture property key name for arrow/function values (used by $inspect.trace auto-label).
        // e.g. { handler: () => { $inspect.trace(); } } → label "handler"
        if !node.computed {
            let is_fn_value = matches!(&node.value,
                Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_));
            if is_fn_value || node.method {
                if let oxc_ast::ast::PropertyKey::StaticIdentifier(id) = &node.key {
                    self.next_arrow_name = Some(id.name.to_string());
                }
            }
        }
    }

    fn enter_method_definition(
        &mut self,
        node: &mut oxc_ast::ast::MethodDefinition<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.accessibility = None;
            node.r#override = false;
            node.optional = false;
        }
    }

    fn enter_variable_declarator(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.is_ts {
            node.type_annotation = None;
            node.definite = false;
        }
        // Capture variable name for arrow functions (used by $inspect.trace auto-label)
        if let Some(Expression::ArrowFunctionExpression(_)) = &node.init {
            if let oxc_ast::ast::BindingPattern::BindingIdentifier(id) = &node.id {
                self.next_arrow_name = Some(id.name.to_string());
            }
        }

        let rune_info = match &node.id {
            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                self.rune_for_binding(id)
            }
            _ => return,
        };

        let Some((kind, mutated)) = rune_info else {
            return;
        };

        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);

        if let Expression::CallExpression(mut call) = init_expr {
            match kind {
                RuneKind::Derived => {
                    // $derived(expr) → $.derived(() => expr)
                    // Just rewrite callee here; thunk wrapping happens post-traverse
                    // to avoid OXC scope_id issues with new arrow nodes.
                    call.callee = self.b.rid_expr("$.derived");
                    if let oxc_ast::ast::BindingPattern::BindingIdentifier(bid) = &node.id {
                        if let Some(sym_id) = bid.symbol_id.get() {
                            self.derived_pending.insert(sym_id);
                        }
                    }
                    node.init = Some(Expression::CallExpression(call));
                }
                RuneKind::DerivedBy => {
                    // $derived.by(fn) → $.derived(fn)
                    call.callee = self.b.rid_expr("$.derived");
                    node.init = Some(Expression::CallExpression(call));
                }
                RuneKind::State | RuneKind::StateRaw => {
                    if mutated {
                        call.callee = self.b.rid_expr("$.state");

                        if call.arguments.is_empty() {
                            let void_zero = self.b.ast.expression_unary(
                                oxc_span::SPAN,
                                oxc_ast::ast::UnaryOperator::Void,
                                self.b.num_expr(0.0),
                            );
                            call.arguments.push(void_zero.into());
                        } else if kind == RuneKind::State {
                            // Wrap proxyable args (arrays/objects) in $.proxy()
                            let needs_proxy = call.arguments[0].as_expression()
                                .is_some_and(|e| Self::should_proxy(e));
                            if needs_proxy {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut call.arguments[0], &mut dummy);
                                let inner = dummy.into_expression();
                                let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                                call.arguments[0] = oxc_ast::ast::Argument::from(proxied);
                            }
                        }

                        let state_expr = Expression::CallExpression(call);
                        // In dev mode, wrap $.state() in $.tag() for debugging
                        node.init = if self.dev {
                            let var_name = match &node.id {
                                oxc_ast::ast::BindingPattern::BindingIdentifier(id) => id.name.as_str(),
                                _ => "state",
                            };
                            Some(self.b.call_expr("$.tag", [Arg::Expr(state_expr), Arg::Str(var_name.to_string())]))
                        } else {
                            Some(state_expr)
                        };
                    } else {
                        let value = if call.arguments.is_empty() {
                            self.b.ast.expression_unary(
                                oxc_span::SPAN,
                                oxc_ast::ast::UnaryOperator::Void,
                                self.b.num_expr(0.0),
                            )
                        } else {
                            let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                            std::mem::swap(&mut call.arguments[0], &mut dummy);
                            dummy.into_expression()
                        };
                        let value = if kind == RuneKind::State && Self::should_proxy(&value) {
                            self.b.call_expr("$.proxy", [Arg::Expr(value)])
                        } else {
                            value
                        };
                        node.init = Some(value);
                    }
                }
                RuneKind::StateEager => {
                    // $state.eager(expr) → $.eager(() => expr)
                    let arg = call.arguments.remove(0).into_expression();
                    node.init = Some(self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(arg))]));
                }
                RuneKind::EffectPending => {
                    // $effect.pending() → $.eager($.pending)
                    let pending_call = self.b.call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
                    node.init = Some(self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(pending_call))]));
                }
                _ => {
                    // Other rune kinds — put back the call unchanged
                    node.init = Some(Expression::CallExpression(call));
                }
            }
        }
    }

    fn enter_expression(&mut self, node: &mut Expression<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        // Strip TS expression wrappers (as, satisfies, non-null, type assertion, instantiation)
        if self.is_ts {
            loop {
                match node {
                    Expression::TSAsExpression(_)
                    | Expression::TSSatisfiesExpression(_)
                    | Expression::TSNonNullExpression(_)
                    | Expression::TSTypeAssertion(_)
                    | Expression::TSInstantiationExpression(_) => {
                        let inner = match self.b.move_expr(node) {
                            Expression::TSAsExpression(ts) => ts.unbox().expression,
                            Expression::TSSatisfiesExpression(ts) => ts.unbox().expression,
                            Expression::TSNonNullExpression(ts) => ts.unbox().expression,
                            Expression::TSTypeAssertion(ts) => ts.unbox().expression,
                            Expression::TSInstantiationExpression(ts) => ts.unbox().expression,
                            _ => unreachable!(),
                        };
                        *node = inner;
                    }
                    _ => break,
                }
            }
        }
        match node {
            Expression::AssignmentExpression(_) => {
                self.transform_assignment(node, ctx);
            }
            Expression::UpdateExpression(_) => {
                self.transform_update(node, ctx);
            }
            Expression::CallExpression(_) => {
                let Expression::CallExpression(call) = node else { return };

                // $host() → $$props.$$host (entire expression replacement, not callee rename)
                if let Expression::Identifier(id) = &call.callee {
                    if id.name.as_str() == "$host" {
                        *node = self.b.static_member_expr(
                            self.b.rid_expr("$$props"),
                            "$$host",
                        );
                        return;
                    }
                }

                // $state.eager(expr) → $.eager(() => expr)
                if let Expression::StaticMemberExpression(member) = &call.callee {
                    if let Expression::Identifier(obj) = &member.object {
                        match (obj.name.as_str(), member.property.name.as_str()) {
                            ("$state", "eager") => {
                                let Expression::CallExpression(mut call) = self.b.move_expr(node) else { unreachable!() };
                                let arg = call.arguments.remove(0).into_expression();
                                *node = self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(arg))]);
                                return;
                            }
                            ("$effect", "pending") => {
                                let pending_call = self.b.call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
                                *node = self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(pending_call))]);
                                return;
                            }
                            _ => {}
                        }
                    }
                }

                let new_callee = match &call.callee {
                    Expression::Identifier(id) if id.name.as_str() == "$effect" => {
                        Some("$.user_effect")
                    }
                    Expression::StaticMemberExpression(member) => {
                        if let Expression::Identifier(obj) = &member.object {
                            match (obj.name.as_str(), member.property.name.as_str()) {
                                ("$effect", "pre") => Some("$.user_pre_effect"),
                                ("$effect", "root") => Some("$.effect_root"),
                                ("$state", "snapshot") => Some("$.snapshot"),
                                ("$effect", "tracking") => Some("$.effect_tracking"),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(callee_name) = new_callee {
                    let Expression::CallExpression(call) = node else { unreachable!() };
                    call.callee = self.b.rid_expr(callee_name);
                }
            }
            Expression::Identifier(id) => {
                // Check props first
                if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                    match prop_kind {
                        PropKind::Source => {
                            let name = id.name.as_str().to_string();
                            *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                        }
                        PropKind::NonSource(prop_name) => {
                            *node = self.b.static_member_expr(
                                self.b.rid_expr("$$props"),
                                &prop_name,
                            );
                        }
                    }
                    return;
                }
                // Store subscription read: $count → $count()
                // Only transform original source identifiers (with reference_id),
                // not synthetic ones we create during transformation.
                let id_name = id.name.as_str();
                if id.reference_id.get().is_some() && self.component_scoping.is_store_ref(id_name) {
                    let name = id_name.to_string();
                    *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                    return;
                }
                // Regular rune check
                let Some((kind, mutated)) = self.rune_for_ref(id) else {
                    return;
                };
                let needs_get = mutated
                    || kind.is_derived();
                if needs_get {
                    let name = id.name.as_str().to_string();
                    *node = svelte_transform::rune_refs::make_rune_get(self.b.ast.allocator, &name);
                }
            }
            // Private field read wrapping handled in exit_expression to avoid infinite re-entry
            _ => {}
        }
    }

    fn exit_expression(&mut self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        // Private state field read: this.#field → $.get(this.#field)
        // Done in exit to avoid infinite re-entry (enter would re-visit the created node).
        if let Expression::PrivateFieldExpression(pfe) = node {
            if matches!(&pfe.object, Expression::ThisExpression(_))
                && self.is_private_state_field(pfe.field.name.as_str())
            {
                let field_expr = self.b.move_expr(node);
                *node = self.b.call_expr("$.get", [Arg::Expr(field_expr)]);
                return;
            }
        }

        if self.dev {
            if let Some(replacement) = self.transform_inspect(node) {
                *node = replacement;
                return;
            }
            // await expr → (await $.track_reactivity_loss(expr))()
            if let Expression::AwaitExpression(await_expr) = node {
                let arg = self.b.move_expr(&mut await_expr.argument);
                let track_call = self.b.call_expr("$.track_reactivity_loss", [Arg::Expr(arg)]);
                let awaited = self.b.await_expr(track_call);
                *node = self.b.call_expr_callee(awaited, std::iter::empty::<Arg<'a, '_>>());
            }
        }
    }
}

impl<'a> ScriptTransformer<'_, 'a> {
    /// Transform `$inspect(args)` → `$.inspect(thunk, inspector, true)`
    /// and `$inspect(args).with(cb)` → `$.inspect(thunk, inspector)`.
    ///
    /// Called in exit_expression (bottom-up), so for `.with()` the inner
    /// `$inspect(...)` has already been transformed to `$.inspect(...)`.
    fn transform_inspect(&self, node: &mut Expression<'a>) -> Option<Expression<'a>> {
        let Expression::CallExpression(outer_call) = node else { return None };

        // Case 1: $.inspect(thunk, inspector, true).with(cb)
        // The inner call was already transformed from $inspect → $.inspect by exit_expression
        if let Expression::StaticMemberExpression(member) = &outer_call.callee {
            if member.property.name.as_str() == "with" {
                if let Expression::CallExpression(inner_call) = &member.object {
                    if let Expression::Identifier(id) = &inner_call.callee {
                        if id.name.as_str() == "$.inspect" {
                            let Expression::CallExpression(outer_call) = node else { unreachable!() };

                            let cb = if outer_call.arguments.is_empty() {
                                self.b.rid_expr("undefined")
                            } else {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut outer_call.arguments[0], &mut dummy);
                                dummy.into_expression()
                            };

                            // Take the thunk from the already-transformed inner call
                            let Expression::StaticMemberExpression(member) = self.b.move_expr(&mut outer_call.callee) else { unreachable!() };
                            let member = member.unbox();
                            let Expression::CallExpression(inner_call) = member.object else { unreachable!() };
                            let mut inner_call = inner_call.unbox();

                            // First arg of inner $.inspect is the thunk
                            let thunk = {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut inner_call.arguments[0], &mut dummy);
                                dummy.into_expression()
                            };

                            let inspector = self.build_inspect_arrow(cb);
                            return Some(self.b.call_expr("$.inspect", [
                                Arg::Expr(thunk),
                                Arg::Expr(inspector),
                            ]));
                        }
                    }
                }
            }
        }

        // Case 2: plain $inspect(args)
        if let Expression::Identifier(id) = &outer_call.callee {
            if id.name.as_str() == "$inspect" {
                let Expression::CallExpression(call) = node else { unreachable!() };
                let inspect_args: Vec<Expression<'a>> = call.arguments.drain(..)
                    .map(|a| a.into_expression())
                    .collect();

                let thunk = self.build_inspect_thunk(inspect_args);
                let console_log = self.b.static_member_expr(self.b.rid_expr("console"), "log");
                let log_call = self.b.call_expr_callee(console_log, [
                    Arg::Spread(self.b.rid_expr("$$args")),
                ]);
                let inspector = self.b.arrow_expr(
                    self.b.rest_params("$$args"),
                    [self.b.expr_stmt(log_call)],
                );

                return Some(self.b.call_expr("$.inspect", [
                    Arg::Expr(thunk),
                    Arg::Expr(inspector),
                    Arg::Bool(true),
                ]));
            }
        }

        None
    }

    /// Build `() => [arg1, arg2, ...]` thunk for inspect args.
    fn build_inspect_thunk(&self, args: Vec<Expression<'a>>) -> Expression<'a> {
        let array_args: Vec<Arg<'a, '_>> = args.into_iter().map(Arg::Expr).collect();
        let array = self.b.array_from_args(array_args);
        self.b.arrow_expr(self.b.no_params(), [self.b.expr_stmt(array)])
    }

    /// Build `(...$$args) => cb(...$$args)` arrow for inspect callback.
    fn build_inspect_arrow(&self, cb: Expression<'a>) -> Expression<'a> {
        let call = self.b.call_expr_callee(cb, [
            Arg::Spread(self.b.rid_expr("$$args")),
        ]);
        self.b.arrow_expr(
            self.b.rest_params("$$args"),
            [self.b.expr_stmt(call)],
        )
    }

    fn transform_assignment(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        let Expression::AssignmentExpression(assign) = node else {
            return;
        };

        if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
            if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                if matches!(prop_kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);
                    *node = self.b.call_expr(&name, [Arg::Expr(right)]);
                    return;
                }
            }
            // Store subscription assignment: $count = val → $.store_set(count, val)
            // Compound: $count += val → $.store_set(count, $count() + val)
            let id_name = id.name.as_str();
            if let Some(base) = self.component_scoping.store_base_name(id_name) {
                let base_name: &str = self.b.alloc_str(base);
                let dollar_name: &str = self.b.alloc_str(id_name);
                let right = self.b.move_expr(&mut assign.right);

                let value = if assign.operator.is_assign() {
                    right
                } else {
                    // Read current value via thunk call: $count()
                    let current = self.b.call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                    if let Some(bin_op) = assign.operator.to_binary_operator() {
                        self.b.ast.expression_binary(oxc_span::SPAN, current, bin_op, right)
                    } else if let Some(log_op) = assign.operator.to_logical_operator() {
                        self.b.ast.expression_logical(oxc_span::SPAN, current, log_op, right)
                    } else {
                        unreachable!("all compound assignment operators are either binary or logical")
                    }
                };

                *node = self.b.call_expr("$.store_set", [
                    Arg::Ident(base_name),
                    Arg::Expr(value),
                ]);
                return;
            }
            if let Some((kind, mutated)) = self.rune_for_ref(id) {
                if mutated {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);

                    // Expand compound assignments: value += x → $.set(value, $.get(value) + x)
                    let value = if assign.operator.is_assign() {
                        right
                    } else {
                        let left_get = svelte_transform::rune_refs::make_rune_get(self.b.ast.allocator, &name);
                        if let Some(bin_op) = assign.operator.to_binary_operator() {
                            self.b.ast.expression_binary(oxc_span::SPAN, left_get, bin_op, right)
                        } else if let Some(log_op) = assign.operator.to_logical_operator() {
                            self.b.ast.expression_logical(oxc_span::SPAN, left_get, log_op, right)
                        } else {
                            unreachable!("all compound assignment operators are either binary or logical")
                        }
                    };

                    let needs_proxy = kind != RuneKind::StateRaw && Self::should_proxy(&value);
                    *node = svelte_transform::rune_refs::make_rune_set(self.b.ast.allocator, &name, value, needs_proxy);
                    return;
                }
            }
        }

        // Deep store mutation: $store.field = val → $.store_mutate(store, ...)
        if let Some((root_name, base)) = self.extract_assign_member_store_root(&assign.left) {
            let root_name = root_name.to_string();
            let base_name = base.to_string();
            let alloc = self.b.ast.allocator;
            // Replace root identifier in the member chain with $.untrack($store)
            svelte_transform::rune_refs::replace_expr_root_in_assign_target(
                &mut assign.left,
                svelte_transform::rune_refs::make_untrack(alloc, &root_name),
            );
            // Take modified assignment as the mutation expression
            let mutation = self.b.move_expr(node);
            let untracked = svelte_transform::rune_refs::make_untrack(alloc, &root_name);
            *node = svelte_transform::rune_refs::make_store_mutate(alloc, &base_name, mutation, untracked);
        }
    }

    fn transform_update(&self, node: &mut Expression<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };

        if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
            if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                if matches!(prop_kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let fn_name = if upd.prefix { "$.update_pre_prop" } else { "$.update_prop" };
                    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&name)];
                    if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                        args.push(Arg::Num(-1.0));
                    }
                    *node = self.b.call_expr(fn_name, args);
                    return;
                }
            }
            // Store subscription update: $count++ → $.update_store(count, $count())
            // ++$count → $.update_pre_store(count, $count())
            // $count-- → $.update_store(count, $count(), -1)
            let id_name = id.name.as_str();
            if let Some(base) = self.component_scoping.store_base_name(id_name) {
                let base_name: &str = self.b.alloc_str(base);
                let dollar_name: &str = self.b.alloc_str(id_name);
                let fn_name = if upd.prefix { "$.update_pre_store" } else { "$.update_store" };
                let thunk_call = self.b.call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                let mut args: Vec<Arg<'a, '_>> = vec![
                    Arg::Ident(base_name),
                    Arg::Expr(thunk_call),
                ];
                if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                    args.push(Arg::Num(-1.0));
                }
                *node = self.b.call_expr(fn_name, args);
                return;
            }
            if let Some((_, mutated)) = self.rune_for_ref(id) {
                if mutated {
                    let name = id.name.as_str().to_string();
                    let is_increment = upd.operator == oxc_ast::ast::UpdateOperator::Increment;
                    *node = svelte_transform::rune_refs::make_rune_update(
                        self.b.ast.allocator, &name, upd.prefix, is_increment,
                    );
                    return;
                }
            }
        }

        // Deep store update: $store.count++ → $.store_mutate(store, ...)
        if let Some((root_name, base)) = self.extract_simple_member_store_root(&upd.argument) {
            let root_name = root_name.to_string();
            let base_name = base.to_string();
            let alloc = self.b.ast.allocator;
            svelte_transform::rune_refs::replace_expr_root_in_simple_target(
                &mut upd.argument,
                svelte_transform::rune_refs::make_untrack(alloc, &root_name),
            );
            let mutation = self.b.move_expr(node);
            let untracked = svelte_transform::rune_refs::make_untrack(alloc, &root_name);
            *node = svelte_transform::rune_refs::make_store_mutate(alloc, &base_name, mutation, untracked);
        }
    }
}
