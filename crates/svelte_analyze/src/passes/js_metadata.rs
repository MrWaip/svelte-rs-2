use oxc_ast::ast::*;
use oxc_ast_visit::{walk, Visit};
use oxc_semantic::ScopeFlags;
use svelte_ast::NodeId;

use crate::scope::ComponentScoping;
use crate::walker::{TemplateVisitor, VisitContext};

// ---------------------------------------------------------------------------
// JsMetadataVisitor — JS scope registration for template expressions/statements
//
// Registers scopes for ALL JS scope-creating constructs (arrows, functions,
// for-loops, blocks, catch clauses) in ComponentScoping, mirroring what
// OXC SemanticBuilder does. This gives 100% JS scoping coverage.
// ---------------------------------------------------------------------------

pub(crate) struct JsMetadataVisitor;

impl TemplateVisitor for JsMetadataVisitor {
    fn visit_js_expression(
        &mut self,
        _id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        let mut collector = ExpressionScopeCollector {
            scoping: &mut ctx.data.scoping,
            scope: ctx.scope,
        };
        collector.visit_expression(expr);
    }

    fn visit_js_statement(
        &mut self,
        _id: NodeId,
        stmt: &Statement<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        let mut collector = ExpressionScopeCollector {
            scoping: &mut ctx.data.scoping,
            scope: ctx.scope,
        };
        collector.visit_statement(stmt);
    }
}

// ---------------------------------------------------------------------------
// ExpressionScopeCollector — full JS scope registration
//
// Handles the same scope-creating constructs as oxc_semantic::SemanticBuilder:
// arrow functions, function expressions, block statements, for/for-in/for-of
// loops, and catch clauses. OXC walk_* functions ensure visit_binding_identifier
// fires for all params/variables in child scopes.
// ---------------------------------------------------------------------------

struct ExpressionScopeCollector<'s> {
    scoping: &'s mut ComponentScoping,
    scope: oxc_semantic::ScopeId,
}

impl ExpressionScopeCollector<'_> {
    fn enter_scope(&mut self) -> oxc_semantic::ScopeId {
        let parent = self.scope;
        self.scope = self.scoping.add_child_scope(parent);
        self.scoping.register_expr_local_scope(self.scope);
        parent
    }
}

impl<'a> Visit<'a> for ExpressionScopeCollector<'_> {
    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression<'a>) {
        let parent = self.enter_scope();
        expr.scope_id.set(Some(self.scope));
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

    fn visit_binding_identifier(&mut self, ident: &BindingIdentifier<'a>) {
        self.scoping.add_binding(self.scope, ident.name.as_str());
    }
}
