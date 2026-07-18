use oxc_ast::ast::{AssignmentOperator, AssignmentTarget, Expression, Statement};
use oxc_span::SPAN;
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;
use svelte_component_semantics::{SymbolId, walk_bindings};

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn const_tag(&mut self, id: NodeId) -> Result<()> {
        let sem = match self.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(id, "const tag")),
        };
        if !matches!(sem.async_kind, ConstTagAsyncKind::Sync) {
            return Err(CodegenError::Unsupported(id, "async const tag"));
        }
        let stmt = self
            .js_arena
            .take_stmt(sem.decl_node_id)
            .ok_or(CodegenError::MissingExpression(id))?;
        self.push_stmt(stmt);
        Ok(())
    }

    pub(crate) fn emit_const_tags_async(&mut self, ids: &[NodeId]) -> Result<()> {
        let promises_name = self.gen_ident("promises");
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let sem: ConstTagBlockSemantics = match self.analysis.block_semantics(id) {
                BlockSemantics::ConstTag(s) => s.clone(),
                _ => continue,
            };
            let (symbol, target, value) = self.take_const_parts(id, sem.decl_node_id)?;

            self.push_stmt(self.b.let_stmt(&target));

            let target_atom = self.b.ast.atom(&target);
            let assignment_target = AssignmentTarget::AssignmentTargetIdentifier(
                self.b
                    .alloc(self.b.ast.identifier_reference(SPAN, target_atom)),
            );
            let assignment = self.b.ast.expression_assignment(
                SPAN,
                AssignmentOperator::Assign,
                assignment_target,
                value,
            );

            let body = match &sem.async_kind {
                ConstTagAsyncKind::Awaited { blockers } => {
                    self.push_blocker_thunk(blockers, &mut thunks);
                    self.b.async_thunk(assignment)
                }
                ConstTagAsyncKind::Deferred { blockers } => {
                    self.push_blocker_thunk(blockers, &mut thunks);
                    self.b.thunk(assignment)
                }
                ConstTagAsyncKind::Sync => self.b.thunk(assignment),
            };
            thunks.push(body);
            let thunk_idx = (thunks.len() - 1) as u32;
            self.const_tag_blockers
                .insert(symbol, (promises_name.clone(), thunk_idx));
        }

        if !thunks.is_empty() {
            let run = self
                .b
                .call_expr("$$renderer.run", [Arg::Expr(self.b.array_expr(thunks))]);
            self.push_stmt(self.b.var_stmt(&promises_name, run));
        }
        Ok(())
    }

    fn take_const_parts(
        &mut self,
        id: NodeId,
        decl_id: OxcNodeId,
    ) -> Result<(SymbolId, String, Expression<'a>)> {
        let stmt = self
            .js_arena
            .take_stmt(decl_id)
            .ok_or(CodegenError::MissingExpression(id))?;
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return Err(CodegenError::Unsupported(id, "const tag declaration"));
        };
        if decl.declarations.is_empty() {
            return Err(CodegenError::Unsupported(id, "const tag declarator"));
        }
        let declarator = decl.declarations.remove(0);

        let mut targets: Vec<(SymbolId, bool)> = Vec::new();
        walk_bindings(&declarator.id, |v| {
            targets.push((v.symbol, v.path.is_empty() && !v.is_rest));
        });
        let [(symbol, true)] = targets.as_slice() else {
            return Err(CodegenError::Unsupported(id, "destructured async const"));
        };
        let symbol = *symbol;
        let name = self.analysis.scoping.symbol_name(symbol).to_string();
        let value = declarator.init.ok_or(CodegenError::MissingExpression(id))?;
        Ok((symbol, name, value))
    }

    pub(crate) fn push_blocker_thunk(&self, blockers: &[u32], thunks: &mut Vec<Expression<'a>>) {
        if blockers.is_empty() {
            return;
        }
        let expr = if blockers.len() == 1 {
            self.blocker_member(blockers[0])
        } else {
            let elements: Vec<Expression<'a>> = blockers
                .iter()
                .map(|&idx| self.blocker_member(idx))
                .collect();
            self.b
                .call_expr("Promise.all", [Arg::Expr(self.b.array_expr(elements))])
        };
        thunks.push(self.b.thunk(expr));
    }
}
