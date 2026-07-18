use svelte_ast::NodeId;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
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
}
