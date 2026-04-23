use svelte_ast::NodeId;

use super::super::data_structures::{EmitState, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_hoisted_svelte_window(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        self.emit_special_target(state, ctx, id, None)?;
        Ok(())
    }

    pub(super) fn emit_hoisted_svelte_document(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        self.emit_special_target(state, ctx, id, None)?;
        Ok(())
    }

    pub(super) fn emit_hoisted_svelte_body(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        self.emit_special_target(state, ctx, id, None)?;
        Ok(())
    }
}
