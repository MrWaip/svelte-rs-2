use oxc_ast::ast::Expression;
use svelte_analyze::{BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics};
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft};

use crate::codegen::binding_pattern::{BindingPatternOutput, BindingPatternSource};
use crate::context::Ctx;

use super::super::data_structures::{EmitState, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_hoisted_const_tag(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let sem = match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(s) => s.clone(),
            _ => return CodegenError::unexpected_block_semantics(id, "ConstTag expected"),
        };

        self.emit_const_tag_sync(state, id, sem)
    }

    fn emit_const_tag_sync(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
        sem: ConstTagBlockSemantics,
    ) -> Result<()> {
        let out = self.emit_binding_pattern(sem.decl_node_id, BindingPatternSource::ConstTag { id })?;
        let BindingPatternOutput::ConstTagDerived(d) = out else {
            return CodegenError::unexpected_child("const tag derived", "statements");
        };
        state.init.push(self.ctx.b.const_stmt(d.target, d.derived));
        if self.ctx.state.dev && d.simple {
            state
                .init
                .push(self.ctx.b.call_stmt("$.get", [Arg::Ident(d.target)]));
        }
        Ok(())
    }

    pub(in crate::codegen) fn emit_const_tags_async_batch(
        &mut self,
        state: &mut EmitState<'a>,
        ids: &[NodeId],
    ) -> Result<()> {
        let promises_name = self.ctx.gen_ident("promises");
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let sem: ConstTagBlockSemantics = match self.ctx.query.analysis.block_semantics(id) {
                BlockSemantics::ConstTag(s) => s.clone(),
                _ => continue,
            };
            let (has_await, blockers) = match &sem.async_kind {
                ConstTagAsyncKind::Async {
                    has_await,
                    blockers,
                } => (*has_await, blockers.to_vec()),
                ConstTagAsyncKind::Sync => (false, Vec::new()),
            };

            let out =
                self.emit_binding_pattern(sem.decl_node_id, BindingPatternSource::ConstTag { id })?;
            let BindingPatternOutput::ConstTagDerived(d) = out else {
                return CodegenError::unexpected_child("const tag derived", "statements");
            };

            state.init.push(self.ctx.b.let_stmt(d.target));
            build_blocker_thunks(self.ctx, &blockers, &mut thunks);

            let assignment = self
                .ctx
                .b
                .assign_expr(AssignLeft::Ident(d.target.to_string()), d.derived);
            let body = if self.ctx.state.dev && d.simple {
                let get_call = self.ctx.b.call_stmt("$.get", [Arg::Ident(d.target)]);
                let assign_stmt = self.ctx.b.expr_stmt(assignment);
                let body_stmts = vec![assign_stmt, get_call];
                if has_await {
                    self.ctx.b.async_thunk_block(body_stmts)
                } else {
                    self.ctx.b.thunk_block(body_stmts)
                }
            } else if has_await {
                self.ctx.b.async_thunk(assignment)
            } else {
                self.ctx.b.thunk(assignment)
            };
            thunks.push(body);
            let thunk_idx = thunks.len() - 1;
            for sym_id in d.symbols {
                self.ctx
                    .const_tag_blockers
                    .insert(sym_id, (promises_name.clone(), thunk_idx));
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

}

fn build_blocker_thunks<'a>(
    ctx: &mut Ctx<'a>,
    blockers: &[u32],
    thunks: &mut Vec<Expression<'a>>,
) {
    if blockers.is_empty() {
        return;
    }
    if blockers.len() == 1 {
        let member = ctx.b.computed_member_expr(
            ctx.b.rid_expr("$$promises"),
            ctx.b.num_expr(blockers[0] as f64),
        );
        let promise_access = ctx.b.static_member_expr(member, "promise");
        thunks.push(ctx.b.thunk(promise_access));
    } else {
        let arr_elements: Vec<Expression<'a>> = blockers
            .iter()
            .map(|&idx| {
                ctx.b
                    .computed_member_expr(ctx.b.rid_expr("$$promises"), ctx.b.num_expr(idx as f64))
            })
            .collect();
        let arr = ctx.b.array_expr(arr_elements);
        let wait_call = ctx.b.call_expr("$.wait", [Arg::Expr(arr)]);
        thunks.push(ctx.b.thunk(wait_call));
    }
}
