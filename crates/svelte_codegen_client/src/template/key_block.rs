//! KeyBlock code generation — `{#key expr}...{/key}`.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::async_plan::AsyncEmissionPlan;
use super::gen_fragment;

/// Generate `$.key(anchor, () => expr, ($$anchor) => { ... })`.
pub(crate) fn gen_key_block<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let async_plan = AsyncEmissionPlan::for_node(ctx, id);
    let needs_async = async_plan.needs_async();
    let span_start = ctx.key_block(id).span.start;

    let body = gen_fragment(ctx, FragmentKey::KeyBlockBody(id));
    let body_fn = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);

    if needs_async {
        let expression = super::expression::get_node_expr(ctx, id);

        // Inside $.async callback: $.key(node, () => $.get($$key), body_fn)
        let key_thunk = ctx.b.thunk(ctx.b.call_expr("$.get", [Arg::Ident("$$key")]));
        let key_call = ctx.b.call_expr(
            "$.key",
            [Arg::Ident("node"), Arg::Expr(key_thunk), Arg::Expr(body_fn)],
        );
        let key_stmt = super::add_svelte_meta(ctx, key_call, span_start, "key");

        let async_thunk = async_plan.async_thunk(ctx, expression);
        stmts.push(async_plan.wrap_async_block(
            ctx,
            anchor,
            "node",
            "$$key",
            async_thunk,
            vec![key_stmt],
        ));
    } else {
        let key_thunk = super::expression::build_node_thunk(ctx, id);

        let key_call = ctx.b.call_expr(
            "$.key",
            [Arg::Expr(anchor), Arg::Expr(key_thunk), Arg::Expr(body_fn)],
        );
        stmts.push(super::add_svelte_meta(ctx, key_call, span_start, "key"));
    }
}
