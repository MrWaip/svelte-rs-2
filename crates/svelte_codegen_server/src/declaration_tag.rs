use svelte_ast::NodeId;

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn declaration_tag(&mut self, id: NodeId) -> Result<()> {
        let svelte_ast::Node::DeclarationTag(tag) = self.component.store.get(id) else {
            return Err(CodegenError::Unsupported(id, "declaration tag"));
        };
        let stmt = self
            .js_arena
            .take_stmt(tag.declaration.id())
            .ok_or(CodegenError::MissingExpression(id))?;
        self.push_stmt(stmt);
        Ok(())
    }
}
