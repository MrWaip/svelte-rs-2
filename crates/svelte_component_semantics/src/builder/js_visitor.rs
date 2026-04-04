use compact_str::CompactString;
use oxc_ast::ast::*;
use oxc_ast_visit::{walk, Visit};
use oxc_syntax::reference::ReferenceFlags;
use oxc_syntax::scope::{ScopeFlags, ScopeId};
use oxc_syntax::symbol::SymbolFlags;

use crate::reference::Reference;
use crate::storage::ComponentSemantics;
use crate::symbol::SymbolOwner;

/// OXC Visit that walks a JS Program and registers scopes, bindings, and
/// references into `ComponentSemantics`.
///
/// Forked from OXC SemanticBuilder (v0.117.0):
/// - Scope/binding/reference logic: https://github.com/oxc-project/oxc/blob/crates_v0.117.0/crates/oxc_semantic/src/builder.rs
/// - Binding rules per declaration kind: https://github.com/oxc-project/oxc/blob/crates_v0.117.0/crates/oxc_semantic/src/binder.rs
/// - Storage types: https://github.com/oxc-project/oxc/blob/crates_v0.117.0/crates/oxc_semantic/src/scoping.rs
pub struct JsSemanticVisitor<'s> {
    semantics: &'s mut ComponentSemantics,
    scope: ScopeId,
    /// Flag propagation for assignment targets / update expressions.
    current_ref_flags: ReferenceFlags,
    /// Current binding flags — set by visit_variable_declaration, consumed by
    /// visit_binding_identifier inside the pattern walk.
    binding_flags: Option<(ScopeId, SymbolFlags)>,
    /// When true, created references are tagged as template references.
    template_mode: bool,
    /// Owner tag for symbols created by this visitor.
    owner: SymbolOwner,
    /// Per-scope-depth stack of unresolved references.
    unresolved_stack: Vec<Vec<(CompactString, oxc_syntax::reference::ReferenceId)>>,
    /// Offset added to every OxcNodeId read from AST nodes.
    /// Used to make module script NodeIds unique relative to instance script.
    node_id_offset: u32,
    /// Max OxcNodeId seen during this traversal (before offset).
    /// Used to compute offset for subsequent programs.
    max_node_id_seen: u32,
}

impl<'s> JsSemanticVisitor<'s> {
    pub fn new(semantics: &'s mut ComponentSemantics, scope: ScopeId, owner: SymbolOwner) -> Self {
        Self {
            semantics,
            scope,
            current_ref_flags: ReferenceFlags::empty(),
            binding_flags: None,
            template_mode: false,
            owner,
            unresolved_stack: vec![Vec::new()],
            node_id_offset: 0,
            max_node_id_seen: 0,
        }
    }

    /// Create a visitor with a NodeId offset. Used for module script so its
    /// NodeIds don't collide with instance script.
    pub fn new_with_offset(
        semantics: &'s mut ComponentSemantics,
        scope: ScopeId,
        owner: SymbolOwner,
        node_id_offset: u32,
    ) -> Self {
        Self {
            semantics,
            scope,
            current_ref_flags: ReferenceFlags::empty(),
            binding_flags: None,
            template_mode: false,
            owner,
            unresolved_stack: vec![Vec::new()],
            node_id_offset,
            max_node_id_seen: 0,
        }
    }

    /// Create a visitor in template mode — all created references will be
    /// tagged as template references. Symbols get `Template` owner.
    pub fn new_template(semantics: &'s mut ComponentSemantics, scope: ScopeId) -> Self {
        Self {
            semantics,
            scope,
            current_ref_flags: ReferenceFlags::empty(),
            binding_flags: None,
            template_mode: true,
            owner: SymbolOwner::Template,
            unresolved_stack: vec![Vec::new()],
            node_id_offset: 0,
            max_node_id_seen: 0,
        }
    }

    /// Remap an OxcNodeId from the AST by applying the offset, and track max seen.
    fn remap_node_id(&mut self, raw: oxc_syntax::node::NodeId) -> oxc_syntax::node::NodeId {
        let raw_val = raw.index() as u32;
        if raw_val > self.max_node_id_seen {
            self.max_node_id_seen = raw_val;
        }
        if self.node_id_offset == 0 {
            raw
        } else {
            oxc_syntax::node::NodeId::from_usize((raw_val + self.node_id_offset) as usize)
        }
    }

    /// The max OxcNodeId seen during traversal (after offset).
    /// Used by the builder to compute the next offset.
    pub(crate) fn max_node_id(&self) -> u32 {
        self.max_node_id_seen + self.node_id_offset
    }

    /// Enable template mode — references will be tagged as template references.
    pub fn set_template_mode(&mut self, template: bool) {
        self.template_mode = template;
    }

    /// Set initial reference flags (e.g. Write for bind:value context).
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

    /// Try to resolve unresolved references in the current scope against its
    /// bindings. Remaining unresolved refs are pushed to the parent level.
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
            // We're at root — dump remaining to root_unresolved
            for (name, ref_id) in remaining {
                self.semantics.add_root_unresolved_reference(name, ref_id);
            }
        }
    }

    /// Flush any remaining unresolved references at the end of traversal.
    /// Called automatically when the visitor is dropped or after visit_program.
    pub(crate) fn flush_unresolved(&mut self) {
        while self.unresolved_stack.len() > 1 {
            self.resolve_references_for_current_scope();
        }
        // Flush root level
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

    fn resolve_reference_flags(&mut self) -> ReferenceFlags {
        let flags = self.current_ref_flags;
        self.current_ref_flags = ReferenceFlags::empty();
        if flags.is_empty() {
            ReferenceFlags::Read
        } else {
            flags
        }
    }

    fn reference_identifier(&mut self, ident: &IdentifierReference<'_>) {
        let flags = self.resolve_reference_flags();
        let node_id = self.remap_node_id(ident.node_id.get());
        let reference = Reference::new(node_id, self.scope, flags);
        let ref_id = if self.template_mode {
            self.semantics.create_template_reference(reference)
        } else {
            self.semantics.create_reference(reference)
        };
        ident.reference_id.set(Some(ref_id));

        // Try immediate resolution via parent-chain walk
        if let Some(sym_id) = self.semantics.find_binding(self.scope, ident.name.as_str()) {
            self.semantics
                .get_reference_mut(ref_id)
                .set_symbol_id(sym_id);
            self.semantics.add_resolved_reference(sym_id, ref_id);
        } else {
            // Defer to scope-exit resolution (handles forward references)
            if let Some(current_level) = self.unresolved_stack.last_mut() {
                current_level.push((CompactString::from(ident.name.as_str()), ref_id));
            }
        }
    }

    fn var_scope(&self) -> ScopeId {
        self.semantics.find_function_scope(self.scope)
    }
}

impl<'a> Visit<'a> for JsSemanticVisitor<'_> {
    // =========================================================
    // Program — flush unresolved after traversal
    // =========================================================

    fn visit_program(&mut self, program: &Program<'a>) {
        walk::walk_program(self, program);
        self.flush_unresolved();
    }

    // =========================================================
    // Scopes
    // =========================================================

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        // Function declaration: bind name in current (parent) scope
        if func.is_declaration() {
            if let Some(ident) = &func.id {
                self.binding_flags = Some((self.scope, SymbolFlags::Function));
                self.visit_binding_identifier(ident);
            }
        }

        let parent = self.enter_scope(flags | ScopeFlags::Function);
        func.set_scope_id(self.scope);

        // Function expression: bind name in own scope (for recursion)
        if func.is_expression() {
            if let Some(ident) = &func.id {
                self.binding_flags = Some((self.scope, SymbolFlags::Function));
                self.visit_binding_identifier(ident);
            }
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
        // Class declaration: bind name in current scope
        if class.is_declaration() {
            if let Some(ident) = &class.id {
                self.binding_flags = Some((self.scope, SymbolFlags::Class));
                self.visit_binding_identifier(ident);
            }
        }

        // Class body gets its own StrictMode scope
        let parent = self.enter_scope(ScopeFlags::StrictMode);
        class.scope_id.set(Some(self.scope));

        // Class expression: bind name in own scope
        if class.is_expression() {
            if let Some(ident) = &class.id {
                self.binding_flags = Some((self.scope, SymbolFlags::Class));
                self.visit_binding_identifier(ident);
            }
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

    // =========================================================
    // Bindings
    // =========================================================

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

    /// The leaf binding visitor — all binding patterns eventually reach here.
    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        if let Some((scope, flags)) = self.binding_flags {
            let node_id = self.remap_node_id(ident.node_id.get());
            let sym_id = self.semantics.symbols.create_symbol(
                CompactString::from(ident.name.as_str()),
                ident.span,
                flags,
                scope,
                node_id,
                self.owner,
            );
            self.semantics.scopes.add_binding(
                scope,
                CompactString::from(ident.name.as_str()),
                sym_id,
            );
            ident.symbol_id.set(Some(sym_id));
        }
    }

    // =========================================================
    // References
    // =========================================================

    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        self.reference_identifier(ident);
    }

    // =========================================================
    // Flag propagation
    // =========================================================

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        if !expr.operator.is_assign() {
            self.current_ref_flags = ReferenceFlags::read_write();
        }
        walk::walk_assignment_expression(self, expr);
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
