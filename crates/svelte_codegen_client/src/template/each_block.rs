//! EachBlock code generation.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::parse_expr;
use super::gen_fragment;

/// Generate statements for an `{#each ...}` block.
pub(crate) fn gen_each_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    let block = ctx.each_block(block_id);
    let body_key = FragmentKey::EachBody(block_id);
    let expr_span = block.expression_span;
    let context_span = block.context_span;

    let collection = parse_expr(ctx, expr_span);
    let collection_fn = ctx
        .b
        .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(collection)]);

    let context_name = ctx.component.source_text(context_span).to_string();
    let frag_body = gen_fragment(ctx, body_key);
    let frag_fn = ctx
        .b
        .arrow_expr(ctx.b.params(["$$anchor", &context_name]), frag_body);

    body.push(ctx.b.call_stmt(
        "$.each",
        [
            Arg::Expr(anchor),
            Arg::Num(16.0),
            Arg::Expr(collection_fn),
            Arg::IdentRef(ctx.b.rid("$.index")),
            Arg::Expr(frag_fn),
        ],
    ));
}
