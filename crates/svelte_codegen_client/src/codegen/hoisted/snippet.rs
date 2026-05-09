use svelte_analyze::BlockSemantics;
use svelte_ast::NodeId;

use super::super::data_structures::{EmitState, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_hoisted_snippet(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let sem = match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::Snippet(s) => s.clone(),
            _ => return CodegenError::unexpected_block_semantics(id, "Snippet expected"),
        };
        self.emit_snippet_block(state, ctx, id, sem)
    }

    pub(in crate::codegen) fn emit_local_snippet_block(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
    ) -> Result<()> {
        if state.skip_snippets {
            return Ok(());
        }
        let sem = match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::Snippet(s) => s.clone(),
            _ => return CodegenError::unexpected_block_semantics(id, "Snippet expected"),
        };
        let stmt = self.build_snippet_const(id, &sem)?;
        let block = self.ctx.b.block_stmt(vec![stmt]);
        state.init.push(block);
        Ok(())
    }

    pub(in crate::codegen) fn emit_inline_snippet_block(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
    ) -> Result<()> {
        if state.skip_snippets {
            return Ok(());
        }
        let sem = match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::Snippet(s) => s.clone(),
            _ => return CodegenError::unexpected_block_semantics(id, "Snippet expected"),
        };
        let stmt = self.build_snippet_const(id, &sem)?;
        state.init.push(stmt);
        Ok(())
    }
}
