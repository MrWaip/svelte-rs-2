use oxc_ast::ast::{Expression, Statement};
use oxc_ast_visit::Visit;
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::reference::ReferenceFlags;
use oxc_syntax::scope::ScopeId;
use oxc_syntax::symbol::SymbolId;
use svelte_ast::FragmentKey;

use super::js_visitor::JsSemanticVisitor;
use crate::reference::Reference;
use crate::storage::ComponentSemantics;
use crate::symbol::SymbolOwner;

/// Context passed to a `TemplateWalker` implementation.
///
/// Provides scope management and JS semantic analysis without exposing
/// Svelte AST types — the walker implementation (in `svelte_analyze`)
/// knows about `Fragment`, `Node`, `AstStore`, etc., while this context
/// only deals with scopes and JS AST.
pub struct TemplateBuildContext<'s> {
    semantics: &'s mut ComponentSemantics,
    scope_stack: Vec<ScopeId>,
    /// Offset applied to OxcNodeIds read from template JS AST nodes.
    /// Ensures template NodeIds don't collide with script NodeIds.
    node_id_offset: u32,
    /// Counter for allocating synthetic NodeIds (shorthand refs, etc.).
    /// Starts at `node_id_offset` and increments.
    next_synthetic_node_id: u32,
}

impl<'s> TemplateBuildContext<'s> {
    pub(crate) fn new(
        semantics: &'s mut ComponentSemantics,
        root_scope: ScopeId,
        node_id_offset: u32,
    ) -> Self {
        Self {
            semantics,
            scope_stack: vec![root_scope],
            node_id_offset,
            next_synthetic_node_id: node_id_offset,
        }
    }

    /// Current scope.
    pub fn current_scope(&self) -> ScopeId {
        *self
            .scope_stack
            .last()
            .expect("scope stack must not be empty")
    }

    /// Create a child scope under the current scope and push it.
    pub fn enter_child_scope(&mut self) -> ScopeId {
        let parent = self.current_scope();
        let child = self.semantics.add_child_scope(parent);
        self.scope_stack.push(child);
        child
    }

    /// Create a child scope, register it for a fragment key, and push it.
    pub fn enter_fragment_scope(&mut self, key: FragmentKey) -> ScopeId {
        let scope = self.enter_child_scope();
        self.semantics.set_fragment_scope(key, scope);
        scope
    }

    /// Register the current scope for a fragment key without creating a new one.
    pub fn register_fragment_scope(&mut self, key: FragmentKey) {
        let scope = self.current_scope();
        self.semantics.set_fragment_scope(key, scope);
    }

    /// Pop the current scope and return to the parent.
    pub fn leave_scope(&mut self) {
        debug_assert!(self.scope_stack.len() > 1, "cannot leave root scope");
        self.scope_stack.pop();
    }

    /// Push an existing scope (e.g. a pre-created scope) onto the stack.
    pub fn enter_scope(&mut self, scope: ScopeId) {
        self.scope_stack.push(scope);
    }

    /// Run the JS semantic visitor on an expression in the current scope.
    /// References are tagged as template references. NodeIds are offset.
    pub fn visit_js_expression(&mut self, expr: &Expression<'_>) {
        let scope = self.current_scope();
        let offset = self.next_synthetic_node_id;
        let mut visitor = JsSemanticVisitor::new_with_offset(
            self.semantics,
            scope,
            SymbolOwner::Template,
            offset,
        );
        visitor.set_template_mode(true);
        visitor.visit_expression(expr);
        let max = visitor.max_node_id();
        if max >= self.next_synthetic_node_id {
            self.next_synthetic_node_id = max + 1;
        }
    }

    /// Run the JS semantic visitor on a statement in the current scope.
    /// References are tagged as template references. NodeIds are offset.
    pub fn visit_js_statement(&mut self, stmt: &Statement<'_>) {
        let scope = self.current_scope();
        let offset = self.next_synthetic_node_id;
        let mut visitor = JsSemanticVisitor::new_with_offset(
            self.semantics,
            scope,
            SymbolOwner::Template,
            offset,
        );
        visitor.set_template_mode(true);
        visitor.visit_statement(stmt);
        let max = visitor.max_node_id();
        if max >= self.next_synthetic_node_id {
            self.next_synthetic_node_id = max + 1;
        }
    }

    /// Run the JS semantic visitor on an expression, with initial reference
    /// flags (e.g. Write for `bind:value`). References are tagged as template.
    pub fn visit_js_expression_with_flags(&mut self, expr: &Expression<'_>, flags: ReferenceFlags) {
        let scope = self.current_scope();
        let offset = self.next_synthetic_node_id;
        let mut visitor = JsSemanticVisitor::new_with_offset(
            self.semantics,
            scope,
            SymbolOwner::Template,
            offset,
        );
        visitor.set_template_mode(true);
        visitor.set_reference_flags(flags);
        visitor.visit_expression(expr);
        let max = visitor.max_node_id();
        if max >= self.next_synthetic_node_id {
            self.next_synthetic_node_id = max + 1;
        }
    }

    /// Allocate a unique OxcNodeId for a synthetic template node
    /// (shorthand reference, etc.).
    pub fn alloc_node_id(&mut self) -> OxcNodeId {
        let id = self.next_synthetic_node_id;
        self.next_synthetic_node_id += 1;
        OxcNodeId::from_usize(id as usize)
    }

    /// Materialize a shorthand reference (e.g. `bind:name`, `class:name`, `style:name`).
    /// Creates a template reference with a unique NodeId and resolves it if the name is bound.
    pub fn materialize_shorthand_reference(
        &mut self,
        name: &str,
        flags: ReferenceFlags,
    ) -> Option<SymbolId> {
        let scope = self.current_scope();
        let node_id = self.alloc_node_id();
        let mut reference = Reference::new(node_id, scope, flags);

        if let Some(sym_id) = self.semantics.find_binding(scope, name) {
            reference.set_symbol_id(sym_id);
            let ref_id = self.semantics.create_template_reference(reference);
            self.semantics.add_resolved_reference(sym_id, ref_id);
            Some(sym_id)
        } else {
            let ref_id = self.semantics.create_template_reference(reference);
            self.semantics
                .add_root_unresolved_reference(compact_str::CompactString::from(name), ref_id);
            None
        }
    }

    /// The highest NodeId allocated by this context (for builder counter tracking).
    pub(crate) fn max_node_id(&self) -> u32 {
        if self.next_synthetic_node_id > self.node_id_offset {
            self.next_synthetic_node_id - 1
        } else {
            self.node_id_offset
        }
    }

    /// Direct access to semantics for operations not covered by the context API.
    pub fn semantics(&self) -> &ComponentSemantics {
        self.semantics
    }

    /// Mutable access to semantics for operations not covered by the context API.
    pub fn semantics_mut(&mut self) -> &mut ComponentSemantics {
        self.semantics
    }
}

/// Trait for walking a Svelte template tree.
///
/// Implemented by `svelte_analyze` which knows the Svelte AST types.
/// The builder calls `walk_template` and passes a `TemplateBuildContext`
/// that provides scope management and JS semantic analysis.
pub trait TemplateWalker {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_>);
}
