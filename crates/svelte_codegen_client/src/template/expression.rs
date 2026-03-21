//! Expression parsing, concatenation building, and emit helpers.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{LoweredTextPart, FragmentItem};
use svelte_ast::ConcatPart as AstConcatPart;
use svelte_ast::NodeId;
use svelte_analyze::ExpressionKind;

use crate::builder::{Arg, AssignLeft, TemplatePart};
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Pre-transformed expression lookup (new path)
// ---------------------------------------------------------------------------

/// Get a pre-transformed expression from ParsedExprs by NodeId.
/// Takes ownership via remove — each expression can only be consumed once.
pub(crate) fn get_node_expr<'a>(ctx: &mut Ctx<'a>, node_id: NodeId) -> Expression<'a> {
    if let Some(expr) = ctx.parsed.exprs.remove(&node_id) {
        expr
    } else {
        debug_assert!(false, "missing pre-transformed expr for node {:?}", node_id);
        ctx.b.str_expr("")
    }
}

/// Get a pre-transformed attribute expression from ParsedExprs by attribute NodeId.
/// Takes ownership via remove — each expression can only be consumed once.
pub(crate) fn get_attr_expr<'a>(ctx: &mut Ctx<'a>, attr_id: NodeId) -> Expression<'a> {
    if let Some(expr) = ctx.parsed.attr_exprs.remove(&attr_id) {
        expr
    } else {
        debug_assert!(false, "missing pre-transformed attr expr for {:?}", attr_id);
        ctx.b.str_expr("")
    }
}

/// Get a pre-transformed concat part expression from ParsedExprs.
/// Takes ownership via remove — each expression can only be consumed once.
pub(crate) fn get_concat_part_expr<'a>(ctx: &mut Ctx<'a>, attr_id: NodeId, part_idx: usize) -> Expression<'a> {
    let key = (attr_id, part_idx);
    if let Some(expr) = ctx.parsed.concat_part_exprs.remove(&key) {
        expr
    } else {
        debug_assert!(false, "missing pre-transformed concat part expr for {:?}", key);
        ctx.b.str_expr("")
    }
}



// ---------------------------------------------------------------------------
// Thunk builder — `() => expr` with rune transforms
// ---------------------------------------------------------------------------

/// Build `() => expr` from a pre-transformed expression (by NodeId).
pub(crate) fn build_node_thunk<'a>(ctx: &mut Ctx<'a>, node_id: NodeId) -> Expression<'a> {
    let expr = get_node_expr(ctx, node_id);
    let params = ctx.b.no_params();
    ctx.b.arrow_expr(params, [ctx.b.expr_stmt(expr)])
}

// ---------------------------------------------------------------------------
// Concatenation builders
// ---------------------------------------------------------------------------

pub(crate) fn build_concat<'a>(ctx: &mut Ctx<'a>, item: &FragmentItem) -> Expression<'a> {
    match item {
        FragmentItem::TextConcat { parts, .. } => build_concat_from_parts(ctx, parts),
        _ => ctx.b.str_expr(""),
    }
}

/// Try to resolve an expression tag to a compile-time known value.
fn try_resolve_known(ctx: &Ctx<'_>, nid: NodeId) -> Option<String> {
    let info = ctx.expression(nid)?;
    if let ExpressionKind::Identifier(name) = &info.kind {
        ctx.known_value(name.as_str()).map(|s| s.to_string())
    } else {
        None
    }
}

pub(crate) fn build_concat_from_parts<'a>(
    ctx: &mut Ctx<'a>,
    parts: &[LoweredTextPart],
) -> Expression<'a> {
    // Single expr: try constant propagation first
    if parts.len() == 1 {
        if let LoweredTextPart::Expr(nid) = parts[0] {
            if let Some(val) = try_resolve_known(ctx, nid) {
                return ctx.b.str_expr(&val);
            }
            return get_node_expr(ctx, nid);
        }
    }

    // Multi-part: fold known values into adjacent text
    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    for part in parts {
        match part {
            LoweredTextPart::Text(s) => {
                // Merge with previous Str part if possible
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(s);
                } else {
                    tpl_parts.push(TemplatePart::Str(s.clone()));
                }
            }
            LoweredTextPart::Expr(nid) => {
                if let Some(val) = try_resolve_known(ctx, *nid) {
                    // Fold into adjacent text
                    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                        prev.push_str(&val);
                    } else {
                        tpl_parts.push(TemplatePart::Str(val));
                    }
                } else {
                    let expr = get_node_expr(ctx, *nid);
                    tpl_parts.push(TemplatePart::Expr(expr));
                }
            }
        }
    }

    // If all parts folded into strings, emit a single string literal
    if tpl_parts.len() == 1 {
        if let TemplatePart::Str(s) = &tpl_parts[0] {
            return ctx.b.str_expr(s);
        }
    }

    ctx.b.template_parts_expr(tpl_parts)
}

pub(crate) fn build_attr_concat<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    parts: &[AstConcatPart],
) -> Expression<'a> {
    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    let mut dyn_idx = 0usize;
    for part in parts {
        match part {
            AstConcatPart::Static(s) => tpl_parts.push(TemplatePart::Str(s.clone())),
            AstConcatPart::Dynamic(_) => {
                let expr = get_concat_part_expr(ctx, attr_id, dyn_idx);
                tpl_parts.push(TemplatePart::Expr(expr));
                dyn_idx += 1;
            }
        }
    }
    ctx.b.template_parts_expr(tpl_parts)
}

// ---------------------------------------------------------------------------
// Emit helpers
// ---------------------------------------------------------------------------

/// Emit a text update (set_text or nodeValue assignment) depending on dynamism.
pub(crate) fn emit_text_update<'a>(
    ctx: &mut Ctx<'a>,
    item: &FragmentItem,
    node_name: &str,
    body: &mut Vec<Statement<'a>>,
) {
    let is_dyn = item_is_dynamic(item, ctx);
    let expr = build_concat(ctx, item);

    if is_dyn {
        let set = ctx.b.call_stmt("$.set_text", [Arg::Ident(node_name), Arg::Expr(expr)]);
        emit_template_effect(ctx, vec![set], body);
    } else {
        body.push(ctx.b.assign_stmt(
            AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(node_name), "nodeValue")),
            expr,
        ));
    }
}

pub(crate) fn emit_template_effect<'a>(
    ctx: &mut Ctx<'a>,
    update: Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    if update.is_empty() {
        return;
    }
    let eff = ctx.b.arrow(ctx.b.no_params(), update);
    body.push(ctx.b.call_stmt("$.template_effect", [Arg::Arrow(eff)]));
}

/// Emit `$.next()` or `$.next(N)` for trailing static siblings after the last named var.
pub(crate) fn emit_trailing_next<'a>(
    ctx: &mut Ctx<'a>,
    trailing: usize,
    stmts: &mut Vec<Statement<'a>>,
) {
    if trailing <= 1 {
        return;
    }
    let offset = trailing - 1;
    if offset == 1 {
        stmts.push(ctx.b.call_stmt("$.next", []));
    } else {
        stmts.push(ctx.b.call_stmt("$.next", [Arg::Num(offset as f64)]));
    }
}

// ---------------------------------------------------------------------------
// Analysis helpers
// ---------------------------------------------------------------------------

pub(crate) fn item_is_dynamic(item: &FragmentItem, ctx: &Ctx<'_>) -> bool {
    match item {
        FragmentItem::TextConcat { parts, .. } => parts_are_dynamic(parts, ctx),
        FragmentItem::Element(id) | FragmentItem::ComponentNode(id) | FragmentItem::IfBlock(id) | FragmentItem::EachBlock(id) | FragmentItem::RenderTag(id) | FragmentItem::HtmlTag(id) | FragmentItem::KeyBlock(id) | FragmentItem::SvelteElement(id) | FragmentItem::SvelteBoundary(id) | FragmentItem::AwaitBlock(id) | FragmentItem::TitleElement(id) => {
            ctx.is_dynamic(*id)
        }
    }
}

/// Check if a text content expression needs call memoization.
/// Requires `has_call` AND references to resolved bindings (not just rune names).
pub(crate) fn text_content_needs_memo(item: &FragmentItem, ctx: &Ctx<'_>) -> bool {
    if let FragmentItem::TextConcat { parts, .. } = item {
        return parts.iter().any(|p| {
            if let LoweredTextPart::Expr(id) = p {
                ctx.analysis.needs_expr_memoization(*id)
            } else {
                false
            }
        });
    }
    false
}

/// Emit memoized template_effect for text with `has_call` expressions:
/// `$.template_effect(($0) => $.set_text(text, $0), [() => expr])`
pub(crate) fn emit_memoized_text_effect<'a>(
    ctx: &mut Ctx<'a>,
    text_name: &str,
    expr: Expression<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    // Build the lazy getter: [() => expr]
    let thunk = ctx.b.thunk(expr);
    let getter_array = ctx.b.array_expr([thunk]);

    // Build the callback: ($0) => $.set_text(text, $0)
    let params = ctx.b.params(["$0"]);
    let set_text = ctx.b.call_stmt(
        "$.set_text",
        [Arg::Ident(text_name), Arg::Ident("$0")],
    );
    let callback = ctx.b.arrow_expr(params, [set_text]);

    body.push(ctx.b.call_stmt(
        "$.template_effect",
        [Arg::Expr(callback), Arg::Expr(getter_array)],
    ));
}

pub(crate) fn parts_are_dynamic(parts: &[LoweredTextPart], ctx: &Ctx<'_>) -> bool {
    parts.iter().any(|p| {
        if let LoweredTextPart::Expr(id) = p {
            ctx.is_dynamic(*id)
        } else {
            false
        }
    })
}
