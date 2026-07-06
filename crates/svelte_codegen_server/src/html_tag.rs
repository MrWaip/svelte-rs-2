use svelte_ast::HtmlTag;
use svelte_ast_builder::Arg;

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn html_tag(&mut self, tag: &'a HtmlTag) -> Result<()> {
        if self.expression_is_async(tag.id) {
            return Err(CodegenError::Unsupported(tag.id, "async html tag"));
        }
        let expr = self.take_expression(tag.id, &tag.expression)?;
        let html = self.b.call_expr("$.html", [Arg::Expr(expr)]);
        self.push_expr(html);
        Ok(())
    }
}
