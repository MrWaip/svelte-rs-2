use crate::codegen::expr::coarse_wrap;
use svelte_analyze::{KeyAsyncKind, KeyBlockSemantics};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;
use svelte_emit_builders::runes::rune_get;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::effect::suspending_block_thunk;
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_key_block(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: KeyBlockSemantics,
    ) -> Result<()> {
        let span_start = self.ctx.query.key_block(id).span.start;
        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

        let fragment = self.ctx.query.key_block(id).fragment;
        let mut inner_ctx = ctx.child_of_block(
            self.ctx,
            fragment,
            FragmentAnchor::callback_param("$$anchor", false),
        );
        inner_ctx.in_block_callback = true;
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, fragment)?;
        let body_stmts = self.pack_callback_body(inner_state, "$$anchor")?;
        let body_fn = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body_stmts);

        match &sem.async_kind {
            KeyAsyncKind::Awaited { blockers } | KeyAsyncKind::Deferred { blockers } => {
                let blockers = blockers.to_vec();
                let async_thunk = match &sem.async_kind {
                    KeyAsyncKind::Awaited { .. } => {
                        let suspension = self.ctx.expression_suspension(id);
                        let expr = self.take_node_expr(id)?;
                        Some(suspending_block_thunk(self.ctx, expr, suspension))
                    }
                    KeyAsyncKind::Deferred { .. } | KeyAsyncKind::Sync => None,
                };
                let key_thunk = self.ctx.b.thunk(rune_get(&self.ctx.b, "$$key"));
                let key_call = self.ctx.b.call_expr(
                    "$.key",
                    [
                        Arg::Ident(&anchor_node),
                        Arg::Expr(key_thunk),
                        Arg::Expr(body_fn),
                    ],
                );
                let key_stmt = self.add_svelte_meta(key_call, span_start, "key");
                let anchor_expr = self.ctx.b.rid_expr(&anchor_node);
                let wrapped = self.emit_async_call_stmt(
                    &blockers,
                    anchor_expr,
                    &anchor_node,
                    "$$key",
                    async_thunk,
                    vec![key_stmt],
                )?;
                state.init.push(wrapped);
                Ok(())
            }
            KeyAsyncKind::Sync => {
                let key_expr = self.take_node_expr(id)?;
                let key_expr = coarse_wrap(self.ctx, key_expr, self.ctx.expression_data(id));
                let key_thunk = self.ctx.b.thunk(key_expr);

                let key_call = self.ctx.b.call_expr(
                    "$.key",
                    [
                        Arg::Ident(&anchor_node),
                        Arg::Expr(key_thunk),
                        Arg::Expr(body_fn),
                    ],
                );
                state
                    .init
                    .push(self.add_svelte_meta(key_call, span_start, "key"));
                Ok(())
            }
        }
    }
}
