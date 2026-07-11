use svelte_analyze::ExpressionSemantics;
use svelte_ast::{Comment, ExpressionTag};
use svelte_ast_builder::Arg;

use crate::error::Result;
use crate::escape::escape_text;
use crate::model::{AsyncInterpolation, ServerCodegen};

impl ServerCodegen<'_> {
    pub(crate) fn comment(&mut self, comment: &Comment) {
        let data = comment.data(self.component.source.as_str());
        self.push_text(&format!("<!--{data}-->"));
    }

    pub(crate) fn expression_tag(&mut self, tag: &ExpressionTag) -> Result<()> {
        if let ExpressionSemantics::Expression(data) = self.analysis.expressions_v2.get(tag.id)
            && let Some(known) = data.declared_evaluation.known_str()
        {
            self.push_text(&escape_text(&known));
            return Ok(());
        }

        let async_emit = self.async_interpolation(tag.id);
        let awaited = matches!(async_emit, Some(AsyncInterpolation::Awaited { .. }));

        let mut expression = self.take_expression(tag.id, &tag.expression)?;
        if awaited && self.save_block_awaits {
            expression = self.save_block_await(expression);
        }
        let escaped = self.b.call_expr("$.escape", [Arg::Expr(expression)]);

        let blockers = match async_emit {
            None => {
                self.push_expr(escaped);
                return Ok(());
            }
            Some(AsyncInterpolation::Awaited { blockers })
            | Some(AsyncInterpolation::Deferred { blockers }) => blockers,
        };

        let thunk = if awaited {
            self.b.async_arrow_expr_body(escaped)
        } else {
            self.b
                .arrow_expr(self.b.no_params(), [self.b.expr_stmt(escaped)])
        };
        let push_call = self.b.call_expr("$$renderer.push", [Arg::Expr(thunk)]);

        let stmt = if blockers.is_empty() {
            self.b.expr_stmt(push_call)
        } else {
            let blockers_arr = self.b.array_expr(blockers);
            let arrow = self
                .b
                .arrow_expr(self.b.params(["$$renderer"]), [self.b.expr_stmt(push_call)]);
            self.b.call_stmt(
                "$$renderer.async",
                [Arg::Expr(blockers_arr), Arg::Expr(arrow)],
            )
        };
        self.push_stmt(stmt);
        Ok(())
    }
}
