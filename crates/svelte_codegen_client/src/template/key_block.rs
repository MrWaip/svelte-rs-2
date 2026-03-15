//! KeyBlock code generation — `{#key expr}...{/key}`.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::gen_fragment;

/// Generate `$.key(anchor, () => expr, ($$anchor) => { ... })`.
pub(crate) fn gen_key_block<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let key_thunk = super::expression::build_node_thunk(ctx, id);

    let body = gen_fragment(ctx, FragmentKey::KeyBlockBody(id));
    let body_fn = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);

    stmts.push(ctx.b.call_stmt(
        "$.key",
        [Arg::Expr(anchor), Arg::Expr(key_thunk), Arg::Expr(body_fn)],
    ));
}
