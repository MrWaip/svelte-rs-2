use svelte_analyze::{FragmentKey, KeyAsyncKind, KeyBlockSemantics};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_key_block(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: KeyBlockSemantics,
    ) -> Result<()> {
        let (needs_async, has_await, blockers) = match &sem.async_kind {
            KeyAsyncKind::Sync => (false, false, Vec::new()),
            KeyAsyncKind::Async {
                has_await,
                blockers,
            } => (true, *has_await, blockers.to_vec()),
        };

        let span_start = self.ctx.query.key_block(id).span.start;
        let anchor_node = self.comment_anchor_node_name(state, ctx)?;

        let key = FragmentKey::KeyBlockBody(id);
        let inner_ctx = ctx.child_of_block(
            key,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, key)?;
        let body_stmts = self.pack_callback_body(inner_state, "$$anchor")?;
        let body_fn = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body_stmts);

        if needs_async {
            let async_thunk = if has_await {
                let expr = self.take_node_expr(id)?;
                Some(self.ctx.b.async_thunk(expr))
            } else {
                None
            };
            let key_thunk = self
                .ctx
                .b
                .thunk(self.ctx.b.call_expr("$.get", [Arg::Ident("$$key")]));
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
                has_await,
                &blockers,
                anchor_expr,
                &anchor_node,
                "$$key",
                async_thunk,
                vec![key_stmt],
            )?;
            state.init.push(wrapped);
            return Ok(());
        }

        let key_expr = self.take_node_expr(id)?;
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
