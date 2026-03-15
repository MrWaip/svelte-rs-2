//! KeyBlock code generation — `{#key expr}...{/key}`.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::get_node_expr;
use super::gen_fragment;

/// Generate `$.key(anchor, () => expr, ($$anchor) => { ... })`.
pub(crate) fn gen_key_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let body_key = FragmentKey::KeyBlockBody(block_id);

    // Build key expression thunk: () => expr
    let expression = get_node_expr(ctx, block_id);
    let key_thunk = ctx.b.arrow(ctx.b.no_params(), [ctx.b.expr_stmt(expression)]);

    // Build body function: ($$anchor) => { ... }
    let body = gen_fragment(ctx, body_key);
    let body_fn = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);

    // $.key(anchor, () => expr, ($$anchor) => { ... })
    let args: Vec<Arg<'a, '_>> = vec![
        Arg::Expr(anchor),
        Arg::Arrow(key_thunk),
        Arg::Expr(body_fn),
    ];

    stmts.push(ctx.b.call_stmt("$.key", args));
}
