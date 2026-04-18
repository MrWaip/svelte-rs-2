//! TitleElement code generation — `<title>` inside `<svelte:head>`.
//!
//! Generates `$.document.title = <value>` with reactive effect wrapping.
//! When value has reactive state: wraps in `$.deferred_template_effect()`.
//! When value is non-reactive: wraps in `$.effect()`.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{FragmentKey, LoweredTextPart};
use svelte_ast::NodeId;

use svelte_ast_builder::{Arg, AssignLeft, TemplatePart};
use crate::context::Ctx;

use super::expression::{emit_effect_call, parts_are_dynamic, MemoValueRef, TemplateMemoState};

enum TitleValuePart<'a> {
    Str(String),
    Expr(Expression<'a>, /* defined */ bool),
    SyncMemo(usize, /* defined */ bool),
    AsyncMemo(usize, /* defined */ bool),
}

impl<'a> TitleValuePart<'a> {
    fn is_defined(&self) -> bool {
        match self {
            Self::Str(_) => true,
            Self::Expr(_, defined) | Self::SyncMemo(_, defined) | Self::AsyncMemo(_, defined) => {
                *defined
            }
        }
    }
}

#[derive(Default)]
struct TitleMemoizer<'a> {
    deps: TemplateMemoState<'a>,
}

impl<'a> TitleMemoizer<'a> {
    fn add_expr(&mut self, ctx: &mut Ctx<'a>, id: NodeId) -> TitleValuePart<'a> {
        self.deps.push_node_deps(ctx, id);

        if let Some(value) = try_resolve_known(ctx, id) {
            return TitleValuePart::Str(value);
        }

        let expr = super::expression::get_node_expr(ctx, id);
        let defined = super::expression::is_definitely_defined(ctx, id, &expr);
        let info = ctx
            .expression(id)
            .expect("title expression metadata should exist");
        match self
            .deps
            .add_memoized_expr(ctx, info, ctx.b.clone_expr(&expr))
        {
            Some(MemoValueRef::Sync(index)) => TitleValuePart::SyncMemo(index, defined),
            Some(MemoValueRef::Async(index)) => TitleValuePart::AsyncMemo(index, defined),
            None => TitleValuePart::Expr(expr, defined),
        }
    }

    fn has_deps(&self) -> bool {
        self.deps.has_deps()
    }
    fn param_names(&self) -> Vec<String> {
        self.deps.param_names()
    }

    fn part_expr(&self, ctx: &Ctx<'a>, part: TitleValuePart<'a>) -> Expression<'a> {
        match part {
            TitleValuePart::Str(value) => ctx.b.str_expr(&value),
            TitleValuePart::Expr(expr, _) => expr,
            TitleValuePart::SyncMemo(index, _) => self.deps.sync_param_expr(ctx, index),
            TitleValuePart::AsyncMemo(index, _) => self.deps.async_param_expr(ctx, index),
        }
    }
}

/// Emit title element statements for a fragment (called before DOM init, like const_tags).
pub(crate) fn emit_title_elements<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    stmts: &mut Vec<Statement<'a>>,
) {
    let Some(ids) = ctx.title_elements_for_fragment(&key).cloned() else {
        return;
    };
    for id in ids {
        gen_title_element(ctx, id, stmts);
    }
}

/// Generate `$.document.title = <value>` wrapped in an effect.
pub(crate) fn gen_title_element<'a>(ctx: &mut Ctx<'a>, id: NodeId, stmts: &mut Vec<Statement<'a>>) {
    let key = FragmentKey::Element(id);
    let lf = ctx.lowered_fragment(&key);
    let items = lf.items.clone();

    // Build the value expression and check dynamism from the title's lowered children.
    // Title children are lowered as a single TextConcat item (text + expression tags).
    let (value, has_state, mut memoizer, single_dynamic_expr) =
        if let Some(svelte_analyze::FragmentItem::TextConcat { parts, .. }) = items.first() {
            let is_dyn = parts_are_dynamic(parts, ctx)
                || parts.iter().any(|part| match part {
                    LoweredTextPart::Expr(id) => {
                        ctx.expr_has_await(*id) || ctx.expr_has_blockers(*id)
                    }
                    _ => false,
                });
            let single_dynamic = parts.len() == 1 && matches!(parts[0], LoweredTextPart::Expr(_));
            let (expr, memoizer) = build_title_value(ctx, parts);
            (expr, is_dyn, memoizer, single_dynamic)
        } else {
            // Empty title or no text content
            (ctx.b.str_expr(""), false, TitleMemoizer::default(), false)
        };

    // For a single dynamic expression, apply `?? ""` fallback.
    // build_concat_from_parts already handles this for template literals (multi-part),
    // but for a single expression we need explicit nullish coalescing.
    // Static values don't need the fallback since they're known strings.
    let value = if has_state && single_dynamic_expr {
        ctx.b.logical_coalesce(value, ctx.b.str_expr(""))
    } else {
        value
    };

    // Build: $.document.title = value
    let doc_expr = ctx.b.static_member_expr(ctx.b.rid_expr("$"), "document");
    let title_member = ctx.b.static_member(doc_expr, "title");
    let assignment = ctx
        .b
        .assign_stmt(AssignLeft::StaticMember(title_member), value);

    // Wrap in effect
    if has_state {
        // Reactive: $.deferred_template_effect(() => { $.document.title = value }, sync?, async?, blockers?)
        let params = memoizer.param_names();
        let arrow = if params.is_empty() {
            ctx.b.thunk_block(vec![assignment])
        } else {
            ctx.b.arrow_block_expr(
                ctx.b.params(params.iter().map(|name| name.as_str())),
                vec![assignment],
            )
        };
        if memoizer.has_deps() {
            emit_effect_call(
                ctx,
                "$.deferred_template_effect",
                arrow,
                &mut memoizer.deps,
                stmts,
            );
        } else {
            stmts.push(
                ctx.b
                    .call_stmt("$.deferred_template_effect", [Arg::Expr(arrow)]),
            );
        }
    } else {
        // Static: $.effect(() => { $.document.title = value })
        let arrow = ctx.b.thunk_block(vec![assignment]);
        stmts.push(ctx.b.call_stmt("$.effect", [Arg::Expr(arrow)]));
    }
}

fn build_title_value<'a>(
    ctx: &mut Ctx<'a>,
    parts: &[LoweredTextPart],
) -> (Expression<'a>, TitleMemoizer<'a>) {
    let mut memoizer = TitleMemoizer::default();
    let mut built_parts: Vec<TitleValuePart<'a>> = Vec::new();

    for part in parts {
        match part {
            LoweredTextPart::TextSpan(_) | LoweredTextPart::TextOwned(_) => {
                push_title_text(
                    &mut built_parts,
                    part.text_value(&ctx.query.component.source).unwrap(),
                );
            }
            LoweredTextPart::Expr(id) => {
                let value_part = memoizer.add_expr(ctx, *id);
                match value_part {
                    TitleValuePart::Str(value) => push_title_text(&mut built_parts, &value),
                    other => built_parts.push(other),
                }
            }
        }
    }

    let value = if built_parts.len() == 1 {
        memoizer.part_expr(ctx, built_parts.pop().unwrap())
    } else {
        let template_parts = built_parts.into_iter().map(|part| match part {
            TitleValuePart::Str(value) => TemplatePart::Str(value),
            other => {
                let defined = other.is_defined();
                TemplatePart::Expr(memoizer.part_expr(ctx, other), defined)
            }
        });
        ctx.b.template_parts_expr(template_parts)
    };

    (value, memoizer)
}

fn push_title_text<'a>(parts: &mut Vec<TitleValuePart<'a>>, text: &str) {
    if let Some(TitleValuePart::Str(prev)) = parts.last_mut() {
        prev.push_str(text);
    } else {
        parts.push(TitleValuePart::Str(text.to_string()));
    }
}

fn try_resolve_known(ctx: &Ctx<'_>, nid: NodeId) -> Option<String> {
    let info = ctx.expression(nid)?;
    info.identifier_name()
        .and_then(|name| ctx.known_value(name).map(|s| s.to_string()))
}
