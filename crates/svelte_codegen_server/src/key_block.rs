use svelte_analyze::{BlockSemantics, KeyAsyncKind};
use svelte_ast::KeyBlock;

use crate::error::{CodegenError, Result};
use crate::fragment::FragmentParent;
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn key_block(&mut self, block: &'a KeyBlock) -> Result<()> {
        let is_async = match self.analysis.block_semantics(block.id) {
            BlockSemantics::Key(sem) => !matches!(sem.async_kind, KeyAsyncKind::Sync),
            _ => return Err(CodegenError::Unsupported(block.id, "key block")),
        };

        if is_async {
            self.push_text("<!--[-->");
        }
        self.push_text("<!---->");
        let body =
            self.child_statements(|cg| cg.fragment(block.fragment, FragmentParent::Block))?;
        let block_stmt = self.b.block_stmt(body);
        self.push_stmt(block_stmt);
        self.push_text("<!---->");
        if is_async {
            self.push_text("<!--]-->");
        }
        Ok(())
    }
}
