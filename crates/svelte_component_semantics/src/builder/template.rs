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

pub struct TemplateBuildContext<'s, 'a> {
    semantics: &'s mut ComponentSemantics<'a>,
    scope_stack: Vec<ScopeId>,

    node_id_offset: u32,

    next_synthetic_node_id: u32,
}

impl<'s, 'a> TemplateBuildContext<'s, 'a> {
    pub(crate) fn new(
        semantics: &'s mut ComponentSemantics<'a>,
        root_scope: ScopeId,
        node_id_offset: u32,
    ) -> Self {
        let safe_offset = node_id_offset.max(1);
        Self {
            semantics,
            scope_stack: vec![root_scope],
            node_id_offset: safe_offset,
            next_synthetic_node_id: safe_offset,
        }
    }

    pub fn current_scope(&self) -> ScopeId {
        *self
            .scope_stack
            .last()
            .expect("scope stack must not be empty")
    }

    pub fn enter_child_scope(&mut self) -> ScopeId {
        let parent = self.current_scope();
        let child = self.semantics.add_child_scope(parent);
        self.scope_stack.push(child);
        child
    }

    pub fn enter_fragment_scope_by_id(&mut self, id: FragmentId) -> ScopeId {
        let scope = self.enter_child_scope();
        self.semantics.set_fragment_scope_by_id(id, scope);
        scope
    }

    pub fn register_fragment_scope_by_id(&mut self, id: FragmentId) {
        let scope = self.current_scope();
        self.semantics.set_fragment_scope_by_id(id, scope);
    }

    pub fn leave_scope(&mut self) {
        debug_assert!(self.scope_stack.len() > 1, "cannot leave root scope");
        self.scope_stack.pop();
    }

    pub fn enter_scope(&mut self, scope: ScopeId) {
        self.scope_stack.push(scope);
    }

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

    pub fn alloc_node_id(&mut self) -> OxcNodeId {
        let id = self.next_synthetic_node_id;
        self.next_synthetic_node_id += 1;
        OxcNodeId::from_usize(id as usize)
    }

    pub fn mark_symbol_member_mutated(&mut self, sym_id: SymbolId) {
        self.semantics.mark_symbol_member_mutated(sym_id);
    }

    pub(crate) fn max_node_id(&self) -> u32 {
        if self.next_synthetic_node_id > self.node_id_offset {
            self.next_synthetic_node_id - 1
        } else {
            self.node_id_offset
        }
    }

    pub fn semantics(&self) -> &ComponentSemantics<'a> {
        self.semantics
    }

    pub fn semantics_mut(&mut self) -> &mut ComponentSemantics<'a> {
        self.semantics
    }
}

pub trait TemplateWalker<'a> {
    fn walk_template(&mut self, ctx: &mut TemplateBuildContext<'_, 'a>);
}
