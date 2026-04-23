use oxc_ast::ast::{Expression, Statement};
use svelte_ast_builder::Arg;

use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    #[allow(clippy::too_many_arguments)]
    pub(in super::super) fn emit_async_call_stmt(
        &mut self,
        has_await: bool,
        blockers: &[u32],
        anchor: Expression<'a>,
        node_param: &str,
        condition_param: &str,
        thunk: Option<Expression<'a>>,
        inner_stmts: Vec<Statement<'a>>,
    ) -> Result<Statement<'a>> {
        let blockers_expr = if blockers.is_empty() {
            self.ctx.b.empty_array_expr()
        } else {
            self.ctx.b.promises_array(blockers)
        };
        let async_values = if has_await {
            let Some(thunk) = thunk else {
                return super::super::CodegenError::unexpected_child(
                    "async thunk",
                    "missing for await plan",
                );
            };
            Arg::Expr(self.ctx.b.array_expr([thunk]))
        } else {
            Arg::Expr(self.ctx.b.void_zero_expr())
        };
        let callback_params = if has_await {
            self.ctx.b.params([node_param, condition_param])
        } else {
            self.ctx.b.params([node_param])
        };
        let callback = self.ctx.b.arrow_block_expr(callback_params, inner_stmts);
        Ok(self.ctx.b.call_stmt(
            "$.async",
            [
                Arg::Expr(anchor),
                Arg::Expr(blockers_expr),
                async_values,
                Arg::Expr(callback),
            ],
        ))
    }
}
