//! TitleElement code generation — `<title>` inside `<svelte:head>`.
//!
//! Generates `$.document.title = <value>` with reactive effect wrapping.
//! When value has reactive state: wraps in `$.deferred_template_effect()`.
//! When value is non-reactive: wraps in `$.effect()`.

use oxc_ast::ast::Statement;

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::{Arg, AssignLeft, AssignRight};
use crate::context::Ctx;

use super::expression::{build_concat_from_parts, parts_are_dynamic};

/// Generate `$.document.title = <value>` wrapped in an effect.
pub(crate) fn gen_title_element<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    stmts: &mut Vec<Statement<'a>>,
) {
    let key = FragmentKey::Element(id);
    let lf = ctx.lowered_fragment(&key);
    let items = lf.items.clone();

    // Build the value expression and check dynamism from the title's lowered children.
    // Title children are lowered as a single TextConcat item (text + expression tags).
    let (value, has_state) = if let Some(svelte_analyze::FragmentItem::TextConcat { parts, .. }) = items.first() {
        let is_dyn = parts_are_dynamic(parts, ctx);
        let expr = build_concat_from_parts(ctx, parts);
        (expr, is_dyn)
    } else {
        // Empty title or no text content
        (ctx.b.str_expr(""), false)
    };

    // For a single expression, apply `?? ""` fallback.
    // build_concat_from_parts already handles this for template literals (multi-part),
    // but for a single expression we need explicit nullish coalescing.
    let value = if items.first().is_some_and(|item| {
        matches!(item, svelte_analyze::FragmentItem::TextConcat { parts, .. } if parts.len() == 1 && matches!(parts[0], svelte_analyze::ConcatPart::Expr(_)))
    }) {
        // Single expression: value ?? ""
        ctx.b.logical_coalesce(value, ctx.b.str_expr(""))
    } else {
        value
    };

    // Build: $.document.title = value
    let doc_expr = ctx.b.static_member_expr(ctx.b.rid_expr("$"), "document");
    let title_member = ctx.b.static_member(doc_expr, "title");
    let assignment = ctx.b.assign_stmt(AssignLeft::StaticMember(title_member), AssignRight::Expr(value));

    // Wrap in effect
    if has_state {
        // Reactive: $.deferred_template_effect(() => { $.document.title = value })
        let arrow = ctx.b.thunk_block(vec![assignment]);
        stmts.push(ctx.b.call_stmt("$.deferred_template_effect", [Arg::Expr(arrow)]));
    } else {
        // Static: $.effect(() => { $.document.title = value })
        let arrow = ctx.b.thunk_block(vec![assignment]);
        stmts.push(ctx.b.call_stmt("$.effect", [Arg::Expr(arrow)]));
    }
}
