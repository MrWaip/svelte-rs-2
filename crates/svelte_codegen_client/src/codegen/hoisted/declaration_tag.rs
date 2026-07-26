use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_analyze::{BlockSemantics, FragmentDeclarationAsyncKind};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;
use svelte_component_semantics::walk_bindings;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};
use super::const_tag::build_blocker_thunks;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_declaration_tags(
        &mut self,
        state: &mut EmitState<'a>,
        ids: &[NodeId],
    ) -> Result<()> {
        let mut grouped: Vec<NodeId> = Vec::new();
        for &id in ids {
            if !self.ctx.state.experimental_async {
                self.emit_hoisted_declaration_tag(state, id)?;
                continue;
            }
            match self.declaration_tag_async_kind(id) {
                FragmentDeclarationAsyncKind::Sync => {
                    self.emit_hoisted_declaration_tag(state, id)?
                }
                FragmentDeclarationAsyncKind::Awaited { .. }
                | FragmentDeclarationAsyncKind::Deferred { .. } => grouped.push(id),
            }
        }
        if !grouped.is_empty() {
            self.emit_declaration_tags_async_batch(state, &grouped)?;
        }
        Ok(())
    }

    fn declaration_tag_async_kind(&self, id: NodeId) -> FragmentDeclarationAsyncKind {
        match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::DeclarationTag(sem) => sem.async_kind.clone(),
            _ => FragmentDeclarationAsyncKind::Sync,
        }
    }

    pub(in crate::codegen) fn emit_hoisted_declaration_tag(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
    ) -> Result<()> {
        let svelte_ast::Node::DeclarationTag(tag) = self.ctx.query.component.store.get(id) else {
            return CodegenError::unexpected_child("declaration tag", "statements");
        };
        let Some(stmt) = self.ctx.state.parsed.take_stmt(tag.declaration.id()) else {
            return CodegenError::unexpected_child("declaration tag statement", "statements");
        };
        state.init.push(stmt);
        Ok(())
    }

    fn emit_declaration_tags_async_batch(
        &mut self,
        state: &mut EmitState<'a>,
        ids: &[NodeId],
    ) -> Result<()> {
        let promises_name = self.ctx.gen_ident("promises");
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let async_kind = self.declaration_tag_async_kind(id);

            let decl_stmt_id = {
                let svelte_ast::Node::DeclarationTag(tag) = self.ctx.query.component.store.get(id)
                else {
                    return CodegenError::unexpected_child("declaration tag", "statements");
                };
                tag.declaration.id()
            };
            let Some(stmt) = self.ctx.state.parsed.take_stmt(decl_stmt_id) else {
                return CodegenError::unexpected_child("declaration tag statement", "statements");
            };
            let Statement::VariableDeclaration(mut decl) = stmt else {
                state.init.push(stmt);
                continue;
            };

            match &async_kind {
                FragmentDeclarationAsyncKind::Awaited { blockers }
                | FragmentDeclarationAsyncKind::Deferred { blockers } => {
                    build_blocker_thunks(self.ctx, blockers, &mut thunks);
                }
                FragmentDeclarationAsyncKind::Sync => {}
            }
            let is_awaited = matches!(async_kind, FragmentDeclarationAsyncKind::Awaited { .. });

            let mut symbols: Vec<SymbolId> = Vec::new();
            let mut assignments: Vec<Expression<'a>> = Vec::new();
            for declarator in decl.declarations.drain(..) {
                walk_bindings(&declarator.id, |v| {
                    let name = self.ctx.query.view.symbol_name(v.symbol).to_string();
                    if symbols.contains(&v.symbol) {
                        return;
                    }
                    symbols.push(v.symbol);
                    state
                        .init
                        .push(self.ctx.b.let_stmt(self.ctx.b.alloc_str(&name)));
                });

                let Some(init) = declarator.init else {
                    continue;
                };
                let Some(target) = self
                    .ctx
                    .b
                    .binding_pattern_to_assignment_target(declarator.id)
                else {
                    return CodegenError::unexpected_child(
                        "declaration tag pattern",
                        "assignment target",
                    );
                };
                assignments.push(self.ctx.b.assign_expr_raw(target, init));
            }

            let thunk = self.build_declaration_thunk(assignments, is_awaited);
            thunks.push(thunk);
            let thunk_idx = thunks.len() - 1;
            for sym in symbols {
                self.ctx
                    .const_tag_blockers
                    .insert(sym, (promises_name.clone(), thunk_idx));
            }
        }

        if !thunks.is_empty() {
            let thunks_array = self.ctx.b.array_expr(thunks);
            let run_call = self.ctx.b.call_expr("$.run", [Arg::Expr(thunks_array)]);
            state
                .init
                .push(self.ctx.b.var_stmt(&promises_name, run_call));
        }
        Ok(())
    }

    fn build_declaration_thunk(
        &mut self,
        mut assignments: Vec<Expression<'a>>,
        is_awaited: bool,
    ) -> Expression<'a> {
        if assignments.len() == 1 {
            let single = assignments.remove(0);
            if is_awaited {
                return self.ctx.b.async_arrow_expr_body(single);
            }
            return self.ctx.b.thunk(single);
        }
        let stmts: Vec<Statement<'a>> = assignments
            .into_iter()
            .map(|expr| self.ctx.b.expr_stmt(expr))
            .collect();
        if is_awaited {
            return self.ctx.b.async_thunk_block(stmts);
        }
        self.ctx.b.thunk_block(stmts)
    }
}
