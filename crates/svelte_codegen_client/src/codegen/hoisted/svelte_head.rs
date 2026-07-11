use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::{EmitState, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_hoisted_svelte_head(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let head_fragment = match self.ctx.query.component.store.get(id) {
            svelte_ast::Node::SvelteHead(head) => head.fragment,
            _ => return Ok(()),
        };
        let inner_ctx = ctx.child_of_svelte_head(self.ctx, head_fragment);
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, head_fragment)?;
        let body = self.pack_callback_body(inner_state, "$$anchor")?;
        let body_fn = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body);
        let hash_str = svelte_analyze::head_hash(self.ctx.state.filename);
        state.init.push(
            self.ctx
                .b
                .call_stmt("$.head", [Arg::Str(hash_str), Arg::Expr(body_fn)]),
        );
        Ok(())
    }
}
