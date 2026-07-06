use svelte_analyze::ExpressionSemantics;
use svelte_ast::{Comment, ExpressionTag};
use svelte_ast_builder::Arg;

use crate::error::Result;
use crate::escape::escape_text;
use crate::model::ServerCodegen;

impl ServerCodegen<'_> {
    pub(crate) fn comment(&mut self, comment: &Comment) {
        let data = comment.data(self.component.source.as_str());
        self.push_text(&format!("<!--{data}-->"));
    }

    pub(crate) fn expression_tag(&mut self, tag: &ExpressionTag) -> Result<()> {
        match self.analysis.expressions_v2.get(tag.id) {
            ExpressionSemantics::Expression(data) => {
                if let Some(known) = data.evaluation.known_str() {
                    self.push_text(&escape_text(&known));
                    return Ok(());
                }
            }
            ExpressionSemantics::NonSpecial => {}
        }
        let expression = self.take_expression(tag.id, &tag.expression)?;
        let escaped = self.b.call_expr("$.escape", [Arg::Expr(expression)]);
        self.push_expr(escaped);
        Ok(())
    }
}
