use oxc_ast::ast::{Expression, Statement};
use oxc_ast_visit::Visit;
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::reference::ReferenceFlags;
use oxc_syntax::scope::ScopeId;
use oxc_syntax::symbol::SymbolId;
use svelte_ast::FragmentId;

use super::js_visitor::JsSemanticVisitor;
use crate::storage::ComponentSemantics;
use crate::symbol::SymbolOwner;

/// Context passed to a `TemplateWalker` implementation.
///
/// Provides scope management and JS semantic analysis without exposing
/// Svelte AST types — the walker implementation (in `svelte_analyze`)
/// knows about `Fragment`, `Node`, `AstStore`, etc., while this context
/// only deals with scopes and JS AST.
pub struct TemplateBuildContext<'s, 'a> {
    semantics: &'s mut ComponentSemantics<'a>,
    scope_stack: Vec<ScopeId>,
    /// Offset applied to OxcNodeIds read from template JS AST nodes.
    /// Ensures template NodeIds don't collide with script NodeIds.
    node_id_offset: u32,
    /// Counter for allocating synthetic NodeIds (shorthand refs, etc.).
    /// Starts at `node_id_offset` and increments.
    next_synthetic_node_id: u32,
}

impl<'s, 'a> TemplateBuildContext<'s, 'a> {
    pub(crate) fn new(
        semantics: &'s mut ComponentSemantics<'a>,
        root_scope: ScopeId,
        node_id_offset: u32,
    ) -> Self {
        // OxcNodeId(0) is reserved as DUMMY sentinel — never use it as a
        // real id. If the offset would land at 0 (script-less component),
        // bump to 1 so the first allocation produces a non-DUMMY id.
        let safe_offset = node_id_offset.max(1);
        Self {
            semantics,
            scope_stack: vec![root_scope],
            node_id_offset: safe_offset,
            next_synthetic_node_id: safe_offset,
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

    /// Create a child scope, register it for a fragment id, and push it.
    pub fn enter_fragment_scope_by_id(&mut self, id: FragmentId) -> ScopeId {
        let scope = self.enter_child_scope();
        self.semantics.set_fragment_scope_by_id(id, scope);
        scope
    }

    /// Register the current scope for a fragment id without creating a new one.
    pub fn register_fragment_scope_by_id(&mut self, id: FragmentId) {
        let scope = self.current_scope();
        self.semantics.set_fragment_scope_by_id(id, scope);
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

    /// Run the JS semantic visitor on a template expression. Binds
    /// `expr_ref.oxc_id` to the visitor's root node id so consumers can later
    /// look up the expression via `expr_ref.id()`. References are tagged as
    /// template references.
    pub fn visit_js_expression(&mut self, expr_ref: &svelte_ast::ExprRef, expr: &Expression<'a>) {
        let id = OxcNodeId::from_usize(self.next_synthetic_node_id as usize);
        expr_ref.bind(id);
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
        visitor.flush_unresolved();
        let max = visitor.max_node_id();
        if max >= self.next_synthetic_node_id {
            self.next_synthetic_node_id = max + 1;
        }
    }

    /// Run the JS semantic visitor on a template statement. Binds
    /// `stmt_ref.oxc_id` to the visitor's root node id. Same contract as
    /// `visit_js_expression`.
    pub fn visit_js_statement(&mut self, stmt_ref: &svelte_ast::StmtRef, stmt: &Statement<'a>) {
        let id = OxcNodeId::from_usize(self.next_synthetic_node_id as usize);
        stmt_ref.bind(id);
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
        visitor.flush_unresolved();
        let max = visitor.max_node_id();
        if max >= self.next_synthetic_node_id {
            self.next_synthetic_node_id = max + 1;
        }
    }

    /// Run the JS semantic visitor on a template expression with initial
    /// reference flags (e.g. Write for `bind:value`). Binds
    /// `expr_ref.oxc_id`. References are tagged as template references.
    pub fn visit_js_expression_with_flags(
        &mut self,
        expr_ref: &svelte_ast::ExprRef,
        expr: &Expression<'a>,
        flags: ReferenceFlags,
    ) {
        let id = OxcNodeId::from_usize(self.next_synthetic_node_id as usize);
        expr_ref.bind(id);
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
        visitor.flush_unresolved();
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

    /// Mark a symbol as member-mutated without creating a write reference.
    ///
    /// Used by template walkers when a member-expression bind target
    /// (`bind:value={foo.bar}`) mutates `foo` through the member chain —
    /// OXC reads `foo` as an object-side identifier, but the template
    /// semantically writes to it. Mirrors reference compiler's
    /// `phases/scope.js::BindDirective` final pass where MemberExpression
    /// bind targets set `binding.mutated = true` on the root symbol (kept
    /// separate from `reassigned`, which fires only for Identifier bind).
    pub fn mark_symbol_member_mutated(&mut self, sym_id: SymbolId) {
        self.semantics.mark_symbol_member_mutated(sym_id);
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
    pub fn semantics(&self) -> &ComponentSemantics<'a> {
        self.semantics
    }

    /// Mutable access to semantics for operations not covered by the context API.
    pub fn semantics_mut(&mut self) -> &mut ComponentSemantics<'a> {
        self.semantics
    }
}

/// Trait for walking a Svelte template tree.
///
/// Implemented by `svelte_analyze` which knows the Svelte AST types.
/// The builder calls `walk_template` and passes a `TemplateBuildContext`
/// that provides scope management and JS semantic analysis.
pub trait TemplateWalker<'a> {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_, 'a>);
}
