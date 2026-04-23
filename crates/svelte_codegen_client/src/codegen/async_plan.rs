use oxc_ast::ast::{Expression, Statement};

use crate::context::Ctx;
use svelte_ast_builder::Arg;

#[allow(clippy::too_many_arguments)]
pub(in crate::codegen) fn emit_async_call_stmt<'a>(
    ctx: &mut Ctx<'a>,
    has_await: bool,
    blockers: &[u32],
    anchor: Expression<'a>,
    node_param: &str,
    condition_param: &str,
    thunk: Option<Expression<'a>>,
    inner_stmts: Vec<Statement<'a>>,
) -> Statement<'a> {
    let blockers_expr = if blockers.is_empty() {
        ctx.b.empty_array_expr()
    } else {
        ctx.b.promises_array(blockers)
    };
    let async_values = if has_await {
        Arg::Expr(
            ctx.b
                .array_expr([thunk.unwrap_or_else(|| ctx.b.void_zero_expr())]),
        )
    } else {
        Arg::Expr(ctx.b.void_zero_expr())
    };
    let callback_params = if has_await {
        ctx.b.params([node_param, condition_param])
    } else {
        ctx.b.params([node_param])
    };
    let callback = ctx.b.arrow_block_expr(callback_params, inner_stmts);
    ctx.b.call_stmt(
        "$.async",
        [
            Arg::Expr(anchor),
            Arg::Expr(blockers_expr),
            async_values,
            Arg::Expr(callback),
        ],
    )
}
