//! EachBlock code generation.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::{Attribute, Node, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::get_node_expr;
use super::gen_fragment;

// Each block flags (from Svelte constants)
const EACH_ITEM_REACTIVE: u32 = 1;
// const EACH_INDEX_REACTIVE: u32 = 2;
const EACH_IS_CONTROLLED: u32 = 4;
const EACH_IS_ANIMATED: u32 = 8;
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
    let expr_has_refs = ctx.expression(block_id)
        .is_some_and(|info| !info.references.is_empty());
    if expr_has_refs {
        flags |= EACH_ITEM_REACTIVE;
    }
    if is_controlled {
        flags |= EACH_IS_CONTROLLED;
    }

    // animate: can only appear on elements that are the sole child of a keyed each block
    if block.key_span.is_some() && block.body.nodes.iter().any(|node| {
        if let Node::Element(el) = node {
            el.attributes.iter().any(|a| matches!(a, Attribute::AnimateDirective(_)))
        } else {
            false
        }
    }) {
        flags |= EACH_IS_ANIMATED;
    }

    let index_name = block.index_span
        .map(|span| ctx.component.source_text(span).to_string());

    let expr_source = ctx.component.source_text(expr_span).trim();
    let root = ctx.analysis.scoping.root_scope_id();
    let is_prop_source = ctx.analysis.scoping.find_binding(root, expr_source)
        .is_some_and(|s| ctx.analysis.scoping.is_prop_source(s));
    let collection_fn = if is_prop_source {
        // Prop getter is already a function — pass directly without thunk
        ctx.b.rid_expr(expr_source)
    } else {
        let collection = get_node_expr(ctx, block_id);
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(collection)])
    };

    // Key function: keyed each uses (item) => key_expr, unkeyed uses $.index
    let key_fn = if let Some(key_expr) = ctx.parsed.key_exprs.remove(&block_id) {
        let key_body = ctx.b.expr_stmt(key_expr);
        ctx.b.arrow_expr(ctx.b.params([&context_name]), [key_body])
    } else {
        ctx.b.rid_expr("$.index")
    };

    // Track each-block variables for bind:this context detection
    ctx.each_vars.push(context_name.clone());
    if let Some(ref idx) = index_name {
        ctx.each_vars.push(idx.clone());
    }

    let frag_body = gen_fragment(ctx, body_key);

    // Pop each-block variables
    if index_name.is_some() {
        ctx.each_vars.pop();
    }
    ctx.each_vars.pop();

    let frag_fn = if let Some(ref idx) = index_name {
        ctx.b.arrow_block_expr(ctx.b.params(["$$anchor", &context_name, idx]), frag_body)
    } else {
        ctx.b.arrow_expr(ctx.b.params(["$$anchor", &context_name]), frag_body)
    };

    body.push(ctx.b.call_stmt(
        "$.each",
        [
            Arg::Expr(anchor),
            Arg::Num(flags as f64),
            Arg::Expr(collection_fn),
            Arg::Expr(key_fn),
            Arg::Expr(frag_fn),
        ],
    ));
}
