//! EachBlock code generation.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::get_node_expr;
use super::gen_fragment;

// Each block flags (from Svelte constants)
const EACH_ITEM_REACTIVE: u32 = 1;
// const EACH_INDEX_REACTIVE: u32 = 2;
const EACH_IS_CONTROLLED: u32 = 4;
// const EACH_IS_ANIMATED: u32 = 8;
const EACH_ITEM_IMMUTABLE: u32 = 16;

/// Generate statements for an `{#each ...}` block.
pub(crate) fn gen_each_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    is_controlled: bool,
    body: &mut Vec<Statement<'a>>,
) {
    let block = ctx.each_block(block_id);
    let body_key = FragmentKey::EachBody(block_id);
    let expr_span = block.expression_span;
    let context_span = block.context_span;
    let context_name = ctx.component.source_text(context_span).to_string();

    // Compute flags
    let mut flags: u32 = EACH_ITEM_IMMUTABLE;

    // If the collection expression references any variable, items need reactive wrapping
    let expr_has_refs = ctx.analysis.expressions.get(&block_id)
        .is_some_and(|info| !info.references.is_empty());
    if expr_has_refs {
        flags |= EACH_ITEM_REACTIVE;
    }
    if is_controlled {
        flags |= EACH_IS_CONTROLLED;
    }

    let expr_source = ctx.component.source_text(expr_span).trim();
    let collection_fn = if ctx.prop_sources.contains(expr_source) {
        // Prop getter is already a function — pass directly without thunk
        ctx.b.rid_expr(expr_source)
    } else {
        let collection = get_node_expr(ctx, block_id);
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(collection)])
    };

    // Track each context variable for $.get() wrapping in body expressions
    let item_reactive = (flags & EACH_ITEM_REACTIVE) != 0;
    if item_reactive {
        ctx.each_vars.insert(context_name.clone());
    }

    let frag_body = gen_fragment(ctx, body_key);

    if item_reactive {
        ctx.each_vars.remove(&context_name);
    }

    let frag_fn = ctx
        .b
        .arrow_expr(ctx.b.params(["$$anchor", &context_name]), frag_body);

    body.push(ctx.b.call_stmt(
        "$.each",
        [
            Arg::Expr(anchor),
            Arg::Num(flags as f64),
            Arg::Expr(collection_fn),
            Arg::IdentRef(ctx.b.rid("$.index")),
            Arg::Expr(frag_fn),
        ],
    ));
}
