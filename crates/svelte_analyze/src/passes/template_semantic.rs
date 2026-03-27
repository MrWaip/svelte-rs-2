use oxc_ast::ast::*;
use oxc_ast_visit::{walk, Visit};
use oxc_semantic::{
    NodeId as OxcNodeId, Reference as OxcReference, ReferenceFlags as OxcReferenceFlags,
    ScopeFlags,
};
use svelte_ast::{BindDirective, NodeId};

use crate::scope::ComponentScoping;
use crate::walker::{ParentKind, TemplateVisitor, VisitContext};

// ---------------------------------------------------------------------------
// TemplateSemanticVisitor — mini-SemanticBuilder for template JS
//
// Mirrors OXC SemanticBuilder for expressions/statements inside the template:
// 1. Creates scopes for JS constructs (arrows, functions, for-loops, etc.)
// 2. Registers bindings with set_symbol_id on BindingIdentifier AST nodes
// 3. Creates OXC References with proper ReferenceFlags (Read/Write/ReadWrite)
// 4. Resolves references via find_binding → set_reference_id on AST nodes
//
// All data writes into the same Scoping as the script block.
// ---------------------------------------------------------------------------

pub(crate) struct TemplateSemanticVisitor;

impl TemplateVisitor for TemplateSemanticVisitor {
    fn visit_bind_directive(&mut self, dir: &BindDirective, ctx: &mut VisitContext<'_>) {
        if !dir.shorthand {
            return;
        }
        // Shorthand bind:name — no parsed expression, create Write ref by name
        if let Some(sym_id) = ctx.data.scoping.find_binding(ctx.scope, dir.name.as_str()) {
            let mut reference =
                OxcReference::new(OxcNodeId::DUMMY, ctx.scope, OxcReferenceFlags::Write);
            reference.set_symbol_id(sym_id);
            let ref_id = ctx.data.scoping.create_reference(reference);
            ctx.data.scoping.add_resolved_reference(sym_id, ref_id);
        }
    }

    fn visit_js_expression(
        &mut self,
        _id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        // bind:value={name} — top-level identifier gets Write reference
        let bind_write = ctx.parent().is_some_and(|p| p.kind == ParentKind::BindDirective)
            && matches!(expr, Expression::Identifier(_));
        let mut collector = SemanticCollector {
            scoping: &mut ctx.data.scoping,
            scope: ctx.scope,
            current_ref_flags: if bind_write {
                Some(OxcReferenceFlags::Write)
            } else {
                None
            },
        };
        collector.visit_expression(expr);
    }

    fn visit_js_statement(
        &mut self,
        _id: NodeId,
        stmt: &Statement<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        let mut collector = SemanticCollector {
            scoping: &mut ctx.data.scoping,
            scope: ctx.scope,
            current_ref_flags: None,
        };
        collector.visit_statement(stmt);
    }
}

// ---------------------------------------------------------------------------
// SemanticCollector — OXC Visit that creates scopes, bindings, and references
// ---------------------------------------------------------------------------

struct SemanticCollector<'s> {
    scoping: &'s mut ComponentScoping,
    scope: oxc_semantic::ScopeId,
    /// OXC-style reference flag propagation. Set before visiting assignment
    /// targets, consumed in visit_identifier_reference. None = default Read.
    current_ref_flags: Option<OxcReferenceFlags>,
}

impl SemanticCollector<'_> {
    fn enter_scope(&mut self) -> oxc_semantic::ScopeId {
        let parent = self.scope;
        self.scope = self.scoping.add_child_scope(parent);
        parent
    }
}

impl<'a> Visit<'a> for SemanticCollector<'_> {
    // -- Scopes --

    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression<'a>) {
        let parent = self.scope;
        // Reuse pre-set scope (e.g. snippet body scope from template_scoping)
        if let Some(existing) = expr.scope_id.get() {
            self.scope = existing;
        } else {
            self.scope = self.scoping.add_child_scope(parent);
            expr.scope_id.set(Some(self.scope));
        }
        walk::walk_arrow_function_expression(self, expr);
        self.scope = parent;
    }

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        let parent = self.enter_scope();
        func.set_scope_id(self.scope);
        walk::walk_function(self, func, flags);
        self.scope = parent;
    }

    fn visit_block_statement(&mut self, stmt: &BlockStatement<'a>) {
        let parent = self.enter_scope();
        walk::walk_block_statement(self, stmt);
        self.scope = parent;
    }

    fn visit_for_statement(&mut self, stmt: &ForStatement<'a>) {
        let parent = self.enter_scope();
        walk::walk_for_statement(self, stmt);
        self.scope = parent;
    }

    fn visit_for_in_statement(&mut self, stmt: &ForInStatement<'a>) {
        let parent = self.enter_scope();
        walk::walk_for_in_statement(self, stmt);
        self.scope = parent;
    }

    fn visit_for_of_statement(&mut self, stmt: &ForOfStatement<'a>) {
        let parent = self.enter_scope();
        walk::walk_for_of_statement(self, stmt);
        self.scope = parent;
    }

    fn visit_catch_clause(&mut self, clause: &CatchClause<'a>) {
        let parent = self.enter_scope();
        walk::walk_catch_clause(self, clause);
        self.scope = parent;
    }

    // -- Bindings --

    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        let sym_id = self.scoping.add_binding(self.scope, ident.name.as_str());
        ident.set_symbol_id(sym_id);
    }

    // -- References --

    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let flags = self.current_ref_flags.take().unwrap_or(OxcReferenceFlags::Read);
        let mut reference = OxcReference::new(OxcNodeId::DUMMY, self.scope, flags);
        if let Some(sym_id) = self.scoping.find_binding(self.scope, ident.name.as_str()) {
            reference.set_symbol_id(sym_id);
            let ref_id = self.scoping.create_reference(reference);
            self.scoping.add_resolved_reference(sym_id, ref_id);
            ident.set_reference_id(ref_id);
        } else {
            let ref_id = self.scoping.create_reference(reference);
            ident.set_reference_id(ref_id);
        }
    }

    // -- Flag propagation (mirrors SemanticBuilder) --

    fn visit_assignment_expression(&mut self, assign: &AssignmentExpression<'a>) {
        if !assign.operator.is_assign() {
            // Compound: +=, -=, etc. → LHS is ReadWrite
            self.current_ref_flags = Some(OxcReferenceFlags::read_write());
        }
        walk::walk_assignment_expression(self, assign);
    }

    fn visit_simple_assignment_target(&mut self, it: &SimpleAssignmentTarget<'a>) {
        if !self.current_ref_flags.as_ref().is_some_and(|f| f.is_write()) {
            self.current_ref_flags = Some(OxcReferenceFlags::Write);
        }
        walk::walk_simple_assignment_target(self, it);
    }

    fn visit_update_expression(&mut self, upd: &UpdateExpression<'a>) {
        self.current_ref_flags = Some(OxcReferenceFlags::read_write());
        walk::walk_update_expression(self, upd);
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        // Root object is Read even in write position (obj.x = 1 reads obj)
        self.current_ref_flags = None;
        walk::walk_member_expression(self, expr);
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &AssignmentTargetPropertyIdentifier<'a>,
    ) {
        self.current_ref_flags = Some(OxcReferenceFlags::Write);
        self.visit_identifier_reference(&it.binding);
        if let Some(init) = &it.init {
            self.visit_expression(init);
        }
    }
}
