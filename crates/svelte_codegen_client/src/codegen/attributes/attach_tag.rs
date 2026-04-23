use svelte_ast::{AttachTag, NodeId};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_attach_tag(
        &mut self,
        state: &mut EmitState<'a>,
        _owner_id: NodeId,
        owner_var: &str,
        a: &AttachTag,
    ) -> Result<()> {
        let attr_id = a.id;
        let blockers = self.attr_blockers(attr_id);

        let expr = self.take_attr_expr(attr_id)?;
        let thunk = self.ctx.b.thunk(expr);
        let stmt = self
            .ctx
            .b
            .call_stmt("$.attach", [Arg::Ident(owner_var), Arg::Expr(thunk)]);
        let stmt = self.wrap_run_after_blockers(stmt, &blockers);
        state.init.push(stmt);

        Ok(())
    }
}
