use svelte_analyze::{BlockSemantics, ConstTagAsyncKind};
use svelte_ast::NodeId;

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn const_tag(&mut self, id: NodeId) -> Result<()> {
        let sem = match self.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(id, "const tag")),
        };
        if !matches!(sem.async_kind, ConstTagAsyncKind::Sync) {
            return Err(CodegenError::Unsupported(id, "async const tag"));
        }
        let stmt = self
            .js_arena
            .take_stmt(sem.decl_node_id)
            .ok_or(CodegenError::MissingExpression(id))?;
        self.push_stmt(stmt);
        Ok(())
    }
}
