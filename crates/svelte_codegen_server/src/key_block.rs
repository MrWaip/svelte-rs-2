use svelte_analyze::{BlockSemantics, KeyAsyncKind};
use svelte_ast::KeyBlock;

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn key_block(
        &mut self,
        block: &'a KeyBlock,
        preserve_whitespace: bool,
    ) -> Result<()> {
        match self.analysis.block_semantics(block.id) {
            BlockSemantics::Key(sem) => {
                if !matches!(sem.async_kind, KeyAsyncKind::Sync) {
                    return Err(CodegenError::Unsupported(block.id, "async key block"));
                }
            }
            _ => return Err(CodegenError::Unsupported(block.id, "key block")),
        }

        self.push_text("<!---->");
        let body = self.child_statements(|cg| {
            cg.fragment(block.fragment, FragmentParent::Block, preserve_whitespace)
        })?;
        let block_stmt = self.b.block_stmt(body);
        self.push_stmt(block_stmt);
        self.push_text("<!---->");
        Ok(())
    }
}
