use svelte_analyze::{BlockSemantics, HtmlTagAsyncKind};
use svelte_ast::HtmlTag;
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn html_tag(&mut self, tag: &'a HtmlTag) -> Result<()> {
        let async_blockers = match self.analysis.block_semantics(tag.id) {
            BlockSemantics::HtmlTag(sem) => match &sem.async_kind {
                HtmlTagAsyncKind::Sync => None,
                HtmlTagAsyncKind::Awaited { blockers } => Some(blockers.clone()),
                HtmlTagAsyncKind::Deferred { .. } => {
                    return Err(CodegenError::Unsupported(tag.id, "deferred html tag"));
                }
            },
            _ => return Err(CodegenError::Unsupported(tag.id, "html tag")),
        };
        let mut expr = self.take_expression(tag.id, &tag.expression)?;
        match async_blockers {
            None => {
                let html = self.b.call_expr("$.html", [Arg::Expr(expr)]);
                self.push_expr(html);
            }
            Some(blockers) => {
                expr = self.save_block_await(expr);
                let html = self.b.call_expr("$.html", [Arg::Expr(expr)]);
                let push = self.b.call_stmt("$$renderer.push", [Arg::Expr(html)]);
                let wrapped = self.wrap_async_block(vec![push], &blockers);
                self.push_stmt(wrapped);
            }
        }
        Ok(())
    }
}
