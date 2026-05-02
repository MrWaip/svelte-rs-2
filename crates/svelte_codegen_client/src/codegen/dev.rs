use oxc_ast::ast::{Expression, Statement};
use svelte_ast_builder::Arg;

use super::Codegen;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn add_svelte_meta(
        &self,
        expression: Expression<'a>,
        span_start: u32,
        block_type: &str,
    ) -> Statement<'a> {
        self.add_svelte_meta_with_extra(expression, span_start, block_type, None)
    }

    pub(in crate::codegen) fn add_svelte_meta_with_extra(
        &self,
        expression: Expression<'a>,
        span_start: u32,
        block_type: &str,
        extra: Option<Expression<'a>>,
    ) -> Statement<'a> {
        if !self.ctx.state.dev {
            return self.ctx.b.expr_stmt(expression);
        }
        let (line, col) = self.ctx.state.line_index.line_col(span_start);
        let thunk = self
            .ctx
            .b
            .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(expression)]);
        let mut args = vec![
            Arg::Expr(thunk),
            Arg::Str(block_type.to_string()),
            Arg::Ident(self.ctx.state.name),
            Arg::Num(line as f64),
            Arg::Num(col as f64),
        ];
        if let Some(extra) = extra {
            args.push(Arg::Expr(extra));
        }
        self.ctx.b.call_stmt("$.add_svelte_meta", args)
    }
}
