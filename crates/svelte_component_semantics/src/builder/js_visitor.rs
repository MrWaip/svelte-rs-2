use compact_str::CompactString;
use oxc_ast::AstKind;
use oxc_ast::ast::*;
use oxc_ast_visit::{Visit, walk};
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::reference::ReferenceFlags;
use oxc_syntax::scope::{ScopeFlags, ScopeId};
use oxc_syntax::symbol::SymbolFlags;

use crate::reference::Reference;
use crate::storage::ComponentSemantics;
use crate::symbol::SymbolOwner;

pub struct JsSemanticVisitor<'s, 'a> {
    semantics: &'s mut ComponentSemantics<'a>,
    scope: ScopeId,

    current_ref_flags: ReferenceFlags,

    binding_flags: Option<(ScopeId, SymbolFlags)>,

    template_mode: bool,

    owner: SymbolOwner,

    unresolved_stack: Vec<Vec<(CompactString, oxc_syntax::reference::ReferenceId)>>,

    current_node_id: OxcNodeId,

    next_node_id: u32,

    start_node_id: u32,
}

impl<'s, 'a> JsSemanticVisitor<'s, 'a> {
    pub fn new(
        semantics: &'s mut ComponentSemantics<'a>,
        scope: ScopeId,
        owner: SymbolOwner,
    ) -> Self {
        Self {
            semantics,
            scope,
            current_ref_flags: ReferenceFlags::empty(),
            binding_flags: None,
            template_mode: false,
            owner,
            unresolved_stack: vec![Vec::new()],
            current_node_id: OxcNodeId::DUMMY,
            next_node_id: 0,
            start_node_id: 0,
        }
    }

    pub fn new_with_offset(
        semantics: &'s mut ComponentSemantics<'a>,
        scope: ScopeId,
        owner: SymbolOwner,
        next_node_id: u32,
    ) -> Self {
        Self {
            semantics,
            scope,
            current_ref_flags: ReferenceFlags::empty(),
            binding_flags: None,
            template_mode: false,
            owner,
            unresolved_stack: vec![Vec::new()],
            current_node_id: OxcNodeId::DUMMY,
            next_node_id,
            start_node_id: next_node_id,
        }
    }

    pub fn new_template(semantics: &'s mut ComponentSemantics<'a>, scope: ScopeId) -> Self {
        Self {
            semantics,
            scope,
            current_ref_flags: ReferenceFlags::empty(),
            binding_flags: None,
            template_mode: true,
            owner: SymbolOwner::Template,
            unresolved_stack: vec![Vec::new()],
            current_node_id: OxcNodeId::DUMMY,
            next_node_id: 0,
            start_node_id: 0,
        }
    }

    fn alloc_node_id(&mut self) -> OxcNodeId {
        let node_id = OxcNodeId::from_usize(self.next_node_id as usize);
        self.next_node_id += 1;
        node_id
    }

    pub(crate) fn max_node_id(&self) -> u32 {
        if self.next_node_id > self.start_node_id {
            self.next_node_id - 1
        } else {
            self.start_node_id
        }
    }

    pub fn set_template_mode(&mut self, template: bool) {
        self.template_mode = template;
    }

    pub fn set_reference_flags(&mut self, flags: ReferenceFlags) {
        self.current_ref_flags = flags;
    }

    fn enter_scope(&mut self, flags: ScopeFlags) -> ScopeId {
        let parent = self.scope;
        self.scope = self.semantics.add_scope(parent, flags);
        self.unresolved_stack.push(Vec::new());
        parent
    }

    fn leave_scope(&mut self, parent: ScopeId) {
        self.resolve_references_for_current_scope();
        self.scope = parent;
    }

    fn resolve_references_for_current_scope(&mut self) {
        let unresolved = self.unresolved_stack.pop().unwrap_or_default();
        if unresolved.is_empty() {
            return;
        }

        let mut remaining = Vec::new();
        for (name, ref_id) in unresolved {
            if let Some(sym_id) = self.semantics.scopes.get_binding(self.scope, &name) {
                self.semantics
                    .get_reference_mut(ref_id)
                    .set_symbol_id(sym_id);
                self.semantics.add_resolved_reference(sym_id, ref_id);
            } else {
                remaining.push((name, ref_id));
            }
        }

        if let Some(parent_level) = self.unresolved_stack.last_mut() {
            parent_level.extend(remaining);
        } else {
            for (name, ref_id) in remaining {
                self.semantics.add_root_unresolved_reference(name, ref_id);
            }
        }
    }

    pub(crate) fn flush_unresolved(&mut self) {
        while self.unresolved_stack.len() > 1 {
            self.resolve_references_for_current_scope();
        }

        if let Some(root_unresolved) = self.unresolved_stack.pop() {
            for (name, ref_id) in root_unresolved {
                if let Some(sym_id) = self.semantics.scopes.get_binding(self.scope, &name) {
                    self.semantics
                        .get_reference_mut(ref_id)
                        .set_symbol_id(sym_id);
                    self.semantics.add_resolved_reference(sym_id, ref_id);
                } else {
                    self.semantics.add_root_unresolved_reference(name, ref_id);
                }
            }
        }
    }

    fn reference_identifier(&mut self, ident: &IdentifierReference<'a>) {
        let flags = {
            let flags = self.current_ref_flags;
            self.current_ref_flags = ReferenceFlags::empty();
            if flags.is_empty() {
                ReferenceFlags::Read
            } else {
                flags
            }
        };
        let reference = Reference::new(self.current_node_id, self.scope, flags);
        let ref_id = if self.template_mode {
            self.semantics.create_template_reference(reference)
        } else {
            self.semantics.create_reference(reference)
        };
        ident.reference_id.set(Some(ref_id));

        let name = ident.name.as_str();
        if let Some(sym_id) = self.semantics.find_binding(self.scope, name) {
            self.semantics
                .get_reference_mut(ref_id)
                .set_symbol_id(sym_id);
            self.semantics.add_resolved_reference(sym_id, ref_id);
        } else if let Some(base) = store_candidate_base(name) {
            let root = self.semantics.root_scope_id();
            if let Some(sym_id) = self.semantics.find_binding(root, base) {
                self.semantics
                    .get_reference_mut(ref_id)
                    .set_symbol_id(sym_id);
                self.semantics.add_resolved_reference(sym_id, ref_id);
                self.semantics.add_store_candidate_ref(sym_id, ref_id);
            } else if let Some(current_level) = self.unresolved_stack.last_mut() {
                current_level.push((CompactString::from(name), ref_id));
            }
        } else if let Some(current_level) = self.unresolved_stack.last_mut() {
            current_level.push((CompactString::from(name), ref_id));
        }
    }

    fn var_scope(&self) -> ScopeId {
        self.semantics.find_function_scope(self.scope)
    }

    fn declare_implicit_legacy_reactive_bindings(&mut self, program: &Program<'a>) {
        for stmt in &program.body {
            let Statement::LabeledStatement(labeled) = stmt else {
                continue;
            };
            if labeled.label.name != "$" {
                continue;
            }
            let Statement::ExpressionStatement(es) = &labeled.body else {
                continue;
            };
            let Some(assign) = unwrap_assignment_expression(&es.expression) else {
                continue;
            };
            if !matches!(assign.operator, AssignmentOperator::Assign) {
                continue;
            }
            match &assign.left {
                AssignmentTarget::AssignmentTargetIdentifier(id) => {
                    self.declare_implicit_target_ident(id.as_ref());
                }
                AssignmentTarget::ObjectAssignmentTarget(obj) => {
                    for prop in &obj.properties {
                        if let AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(
                            shorthand,
                        ) = prop
                        {
                            self.declare_implicit_target_ident(&shorthand.binding);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn declare_implicit_target_ident(&mut self, id: &IdentifierReference<'a>) {
        let name = id.name.as_str();
        if name.starts_with('$') {
            return;
        }
        if self.semantics.find_binding(self.scope, name).is_some() {
            return;
        }
        self.semantics.add_binding(
            self.scope,
            name,
            id.span,
            SymbolFlags::empty(),
            id.node_id.get(),
            SymbolOwner::Synthetic,
        );
    }
}

impl<'s, 'a> Visit<'a> for JsSemanticVisitor<'s, 'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        let node_id = self.alloc_node_id();
        kind.set_node_id(node_id);
        let parent_id = (self.current_node_id != OxcNodeId::DUMMY).then_some(self.current_node_id);
        self.semantics
            .record_js_node(node_id, kind, self.scope, parent_id);
        self.current_node_id = node_id;
    }

    fn leave_node(&mut self, _kind: AstKind<'a>) {
        self.current_node_id = self
            .semantics
            .js_parent_id(self.current_node_id)
            .unwrap_or(OxcNodeId::DUMMY);
    }

    fn visit_program(&mut self, program: &Program<'a>) {
        walk::walk_program(self, program);
        if matches!(self.owner, SymbolOwner::InstanceScript) {
            self.declare_implicit_legacy_reactive_bindings(program);
        }
        self.flush_unresolved();
    }

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        if func.is_declaration()
            && let Some(ident) = &func.id
        {
            self.binding_flags = Some((self.scope, SymbolFlags::Function));
            self.visit_binding_identifier(ident);
        }

        let parent = self.enter_scope(flags | ScopeFlags::Function);
        func.set_scope_id(self.scope);

        if func.is_expression()
            && let Some(ident) = &func.id
        {
            self.binding_flags = Some((self.scope, SymbolFlags::Function));
            self.visit_binding_identifier(ident);
        }

        walk::walk_function(self, func, flags);
        self.leave_scope(parent);
    }

    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression<'a>) {
        let parent = self.enter_scope(ScopeFlags::Arrow | ScopeFlags::Function);
        expr.scope_id.set(Some(self.scope));
        walk::walk_arrow_function_expression(self, expr);
        self.leave_scope(parent);
    }

    fn visit_class(&mut self, class: &Class<'a>) {
        if class.is_declaration()
            && let Some(ident) = &class.id
        {
            self.binding_flags = Some((self.scope, SymbolFlags::Class));
            self.visit_binding_identifier(ident);
        }

        let parent = self.enter_scope(ScopeFlags::StrictMode);
        class.scope_id.set(Some(self.scope));

        if class.is_expression()
            && let Some(ident) = &class.id
        {
            self.binding_flags = Some((self.scope, SymbolFlags::Class));
            self.visit_binding_identifier(ident);
        }

        walk::walk_class(self, class);
        self.leave_scope(parent);
    }

    fn visit_block_statement(&mut self, stmt: &BlockStatement<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        stmt.scope_id.set(Some(self.scope));
        walk::walk_block_statement(self, stmt);
        self.leave_scope(parent);
    }

    fn visit_switch_statement(&mut self, stmt: &SwitchStatement<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        stmt.scope_id.set(Some(self.scope));
        walk::walk_switch_statement(self, stmt);
        self.leave_scope(parent);
    }

    fn visit_for_statement(&mut self, stmt: &ForStatement<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        stmt.scope_id.set(Some(self.scope));
        walk::walk_for_statement(self, stmt);
        self.leave_scope(parent);
    }

    fn visit_for_in_statement(&mut self, stmt: &ForInStatement<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        stmt.scope_id.set(Some(self.scope));
        walk::walk_for_in_statement(self, stmt);
        self.leave_scope(parent);
    }

    fn visit_for_of_statement(&mut self, stmt: &ForOfStatement<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        stmt.scope_id.set(Some(self.scope));
        walk::walk_for_of_statement(self, stmt);
        self.leave_scope(parent);
    }

    fn visit_catch_clause(&mut self, clause: &CatchClause<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        clause.scope_id.set(Some(self.scope));
        walk::walk_catch_clause(self, clause);
        self.leave_scope(parent);
    }

    fn visit_ts_type_alias_declaration(&mut self, decl: &TSTypeAliasDeclaration<'a>) {
        self.binding_flags = Some((self.scope, SymbolFlags::TypeAlias));
        self.visit_binding_identifier(&decl.id);
        self.binding_flags = None;

        let parent = self.enter_scope(ScopeFlags::empty());
        decl.scope_id.set(Some(self.scope));
        walk::walk_ts_type_alias_declaration(self, decl);
        self.leave_scope(parent);
    }

    fn visit_ts_interface_declaration(&mut self, decl: &TSInterfaceDeclaration<'a>) {
        self.binding_flags = Some((self.scope, SymbolFlags::Interface));
        self.visit_binding_identifier(&decl.id);
        self.binding_flags = None;

        let parent = self.enter_scope(ScopeFlags::empty());
        decl.scope_id.set(Some(self.scope));
        walk::walk_ts_interface_declaration(self, decl);
        self.leave_scope(parent);
    }

    fn visit_static_block(&mut self, block: &StaticBlock<'a>) {
        let parent = self.enter_scope(ScopeFlags::ClassStaticBlock | ScopeFlags::Function);
        block.scope_id.set(Some(self.scope));
        walk::walk_static_block(self, block);
        self.leave_scope(parent);
    }

    fn visit_with_statement(&mut self, stmt: &WithStatement<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        stmt.scope_id.set(Some(self.scope));
        walk::walk_with_statement(self, stmt);
        self.leave_scope(parent);
    }

    fn visit_ts_module_declaration(&mut self, decl: &TSModuleDeclaration<'a>) {
        let parent = self.enter_scope(ScopeFlags::TsModuleBlock);
        decl.scope_id.set(Some(self.scope));
        walk::walk_ts_module_declaration(self, decl);
        self.leave_scope(parent);
    }

    fn visit_ts_global_declaration(&mut self, decl: &TSGlobalDeclaration<'a>) {
        let parent = self.enter_scope(ScopeFlags::TsModuleBlock);
        decl.scope_id.set(Some(self.scope));
        walk::walk_ts_global_declaration(self, decl);
        self.leave_scope(parent);
    }

    fn visit_ts_enum_declaration(&mut self, decl: &TSEnumDeclaration<'a>) {
        let flags = if decl.r#const {
            SymbolFlags::ConstEnum
        } else {
            SymbolFlags::RegularEnum
        };
        self.binding_flags = Some((self.scope, flags));
        self.visit_binding_identifier(&decl.id);
        self.binding_flags = None;
        walk::walk_ts_enum_declaration(self, decl);
    }

    fn visit_ts_enum_body(&mut self, body: &TSEnumBody<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        body.scope_id.set(Some(self.scope));
        walk::walk_ts_enum_body(self, body);
        self.leave_scope(parent);
    }

    fn visit_ts_conditional_type(&mut self, ty: &TSConditionalType<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        ty.scope_id.set(Some(self.scope));
        walk::walk_ts_conditional_type(self, ty);
        self.leave_scope(parent);
    }

    fn visit_ts_call_signature_declaration(&mut self, decl: &TSCallSignatureDeclaration<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        decl.scope_id.set(Some(self.scope));
        walk::walk_ts_call_signature_declaration(self, decl);
        self.leave_scope(parent);
    }

    fn visit_ts_method_signature(&mut self, sig: &TSMethodSignature<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        sig.scope_id.set(Some(self.scope));
        walk::walk_ts_method_signature(self, sig);
        self.leave_scope(parent);
    }

    fn visit_ts_construct_signature_declaration(
        &mut self,
        decl: &TSConstructSignatureDeclaration<'a>,
    ) {
        let parent = self.enter_scope(ScopeFlags::empty());
        decl.scope_id.set(Some(self.scope));
        walk::walk_ts_construct_signature_declaration(self, decl);
        self.leave_scope(parent);
    }

    fn visit_ts_function_type(&mut self, ty: &TSFunctionType<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        ty.scope_id.set(Some(self.scope));
        walk::walk_ts_function_type(self, ty);
        self.leave_scope(parent);
    }

    fn visit_ts_constructor_type(&mut self, ty: &TSConstructorType<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        ty.scope_id.set(Some(self.scope));
        walk::walk_ts_constructor_type(self, ty);
        self.leave_scope(parent);
    }

    fn visit_ts_mapped_type(&mut self, ty: &TSMappedType<'a>) {
        let parent = self.enter_scope(ScopeFlags::empty());
        ty.scope_id.set(Some(self.scope));
        walk::walk_ts_mapped_type(self, ty);
        self.leave_scope(parent);
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration<'a>) {
        let (scope, flags) = match decl.kind {
            VariableDeclarationKind::Var => (self.var_scope(), SymbolFlags::FunctionScopedVariable),
            VariableDeclarationKind::Let => (self.scope, SymbolFlags::BlockScopedVariable),
            VariableDeclarationKind::Const
            | VariableDeclarationKind::Using
            | VariableDeclarationKind::AwaitUsing => (
                self.scope,
                SymbolFlags::BlockScopedVariable | SymbolFlags::ConstVariable,
            ),
        };
        self.binding_flags = Some((scope, flags));
        walk::walk_variable_declaration(self, decl);
        self.binding_flags = None;
    }

    fn visit_formal_parameter(&mut self, param: &FormalParameter<'a>) {
        self.binding_flags = Some((self.scope, SymbolFlags::FunctionScopedVariable));
        walk::walk_formal_parameter(self, param);
        self.binding_flags = None;
    }

    fn visit_binding_rest_element(&mut self, elem: &BindingRestElement<'a>) {
        walk::walk_binding_rest_element(self, elem);
    }

    fn visit_catch_parameter(&mut self, param: &CatchParameter<'a>) {
        self.binding_flags = Some((
            self.scope,
            SymbolFlags::FunctionScopedVariable | SymbolFlags::CatchVariable,
        ));
        walk::walk_catch_parameter(self, param);
        self.binding_flags = None;
    }

    fn visit_import_declaration(&mut self, decl: &ImportDeclaration<'a>) {
        let is_type = decl.import_kind.is_type();
        if let Some(specifiers) = &decl.specifiers {
            for spec in specifiers {
                let (ident, spec_is_type) = match spec {
                    ImportDeclarationSpecifier::ImportSpecifier(s) => {
                        (&s.local, s.import_kind.is_type())
                    }
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => (&s.local, false),
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => (&s.local, false),
                };
                let flags = if is_type || spec_is_type {
                    SymbolFlags::TypeImport
                } else {
                    SymbolFlags::Import
                };
                self.binding_flags = Some((self.scope, flags));
                self.visit_binding_identifier(ident);
            }
        }
        self.binding_flags = None;
    }

    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        let kind = AstKind::BindingIdentifier(self.alloc(ident));
        self.enter_node(kind);
        if let Some((scope, flags)) = self.binding_flags {
            let sym_id = self.semantics.symbols.create_symbol(
                CompactString::from(ident.name.as_str()),
                ident.span,
                flags,
                scope,
                self.current_node_id,
                self.owner,
            );
            self.semantics.scopes.add_binding(
                scope,
                CompactString::from(ident.name.as_str()),
                sym_id,
            );
            ident.symbol_id.set(Some(sym_id));
        }
        self.visit_span(&ident.span);
        self.leave_node(kind);
    }

    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let kind = AstKind::IdentifierReference(self.alloc(ident));
        self.enter_node(kind);
        self.reference_identifier(ident);
        self.visit_span(&ident.span);
        self.leave_node(kind);
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        if !expr.operator.is_assign() {
            self.current_ref_flags = ReferenceFlags::read_write();
        }
        walk::walk_assignment_expression(self, expr);
        if let Some(sym) = assignment_target_member_root_symbol(self.semantics, &expr.left) {
            self.semantics.mark_symbol_member_mutated(sym);
        }
    }

    fn visit_simple_assignment_target(&mut self, target: &SimpleAssignmentTarget<'a>) {
        if !self.current_ref_flags.is_write() {
            self.current_ref_flags = ReferenceFlags::Write;
        }
        walk::walk_simple_assignment_target(self, target);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
        self.current_ref_flags = ReferenceFlags::read_write();
        walk::walk_update_expression(self, expr);
        if let Some(sym) =
            simple_assignment_target_member_root_symbol(self.semantics, &expr.argument)
        {
            self.semantics.mark_symbol_member_mutated(sym);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        self.current_ref_flags = ReferenceFlags::empty();
        walk::walk_member_expression(self, expr);
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &AssignmentTargetPropertyIdentifier<'a>,
    ) {
        self.current_ref_flags = ReferenceFlags::Write;
        self.visit_identifier_reference(&it.binding);
        if let Some(init) = &it.init {
            self.visit_expression(init);
        }
    }
}

fn store_candidate_base(name: &str) -> Option<&str> {
    if name.starts_with('$')
        && name.len() > 1
        && !name.starts_with("$$")
        && !svelte_ast::is_rune_name(name)
    {
        Some(&name[1..])
    } else {
        None
    }
}

fn assignment_target_member_root_symbol(
    semantics: &ComponentSemantics<'_>,
    target: &AssignmentTarget<'_>,
) -> Option<oxc_syntax::symbol::SymbolId> {
    match target {
        AssignmentTarget::StaticMemberExpression(m) => expression_root_symbol(semantics, &m.object),
        AssignmentTarget::ComputedMemberExpression(m) => {
            expression_root_symbol(semantics, &m.object)
        }
        _ => None,
    }
}

fn simple_assignment_target_member_root_symbol(
    semantics: &ComponentSemantics<'_>,
    target: &SimpleAssignmentTarget<'_>,
) -> Option<oxc_syntax::symbol::SymbolId> {
    match target {
        SimpleAssignmentTarget::StaticMemberExpression(m) => {
            expression_root_symbol(semantics, &m.object)
        }
        SimpleAssignmentTarget::ComputedMemberExpression(m) => {
            expression_root_symbol(semantics, &m.object)
        }
        _ => None,
    }
}

fn unwrap_assignment_expression<'r, 'a>(
    expr: &'r Expression<'a>,
) -> Option<&'r AssignmentExpression<'a>> {
    let mut current = expr;
    loop {
        match current {
            Expression::AssignmentExpression(assign) => return Some(assign),
            Expression::ParenthesizedExpression(p) => current = &p.expression,
            _ => return None,
        }
    }
}

fn expression_root_symbol(
    semantics: &ComponentSemantics<'_>,
    expr: &Expression<'_>,
) -> Option<oxc_syntax::symbol::SymbolId> {
    match expr {
        Expression::Identifier(id) => id
            .reference_id
            .get()
            .and_then(|ref_id| semantics.get_reference(ref_id).symbol_id()),
        Expression::StaticMemberExpression(m) => expression_root_symbol(semantics, &m.object),
        Expression::ComputedMemberExpression(m) => expression_root_symbol(semantics, &m.object),
        _ => None,
    }
}
