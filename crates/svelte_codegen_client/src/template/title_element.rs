//! TitleElement code generation — `<title>` inside `<svelte:head>`.
//!
//! Generates `$.document.title = <value>` with reactive effect wrapping.
//! When value has reactive state: wraps in `$.deferred_template_effect()`.
//! When value is non-reactive: wraps in `$.effect()`.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{ExpressionKind, FragmentKey, LoweredTextPart};
use svelte_ast::NodeId;

use crate::builder::{Arg, AssignLeft, TemplatePart};
use crate::context::Ctx;

use super::expression::parts_are_dynamic;

enum TitleValuePart<'a> {
    Str(String),
    Expr(Expression<'a>),
    SyncMemo(usize),
    AsyncMemo(usize),
}

#[derive(Default)]
struct TitleMemoizer<'a> {
    sync_values: Vec<Expression<'a>>,
    async_values: Vec<Expression<'a>>,
    blockers: Vec<u32>,
    extra_blockers: Vec<Expression<'a>>,
}

impl<'a> TitleMemoizer<'a> {
    fn push_blockers(&mut self, ctx: &mut Ctx<'a>, id: NodeId) {
        for idx in ctx.analysis.expression_blockers(id) {
            if !self.blockers.contains(&idx) {
                self.blockers.push(idx);
            }
        }
        self.extra_blockers.extend(ctx.const_tag_blocker_exprs(id));
    }

    fn add_expr(&mut self, ctx: &mut Ctx<'a>, id: NodeId) -> TitleValuePart<'a> {
        self.push_blockers(ctx, id);

        if let Some(value) = try_resolve_known(ctx, id) {
            return TitleValuePart::Str(value);
        }

        let expr = super::expression::get_node_expr(ctx, id);
        if ctx.analysis.needs_expr_memoization(id) {
            if ctx.expr_has_await(id) {
                let index = self.async_values.len();
                self.async_values.push(expr);
                TitleValuePart::AsyncMemo(index)
            } else {
                let index = self.sync_values.len();
                self.sync_values.push(expr);
                TitleValuePart::SyncMemo(index)
            }
        } else {
            TitleValuePart::Expr(expr)
        }
    }

    fn has_deps(&self) -> bool {
        !self.sync_values.is_empty()
            || !self.async_values.is_empty()
            || !self.blockers.is_empty()
            || !self.extra_blockers.is_empty()
    }

    fn has_sync_values(&self) -> bool {
        !self.sync_values.is_empty()
    }

    fn has_async_values(&self) -> bool {
        !self.async_values.is_empty()
    }

    fn has_blockers(&self) -> bool {
        !self.blockers.is_empty() || !self.extra_blockers.is_empty()
    }

    fn sync_values_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        if self.sync_values.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(self.sync_values.drain(..).map(|expr| ctx.b.thunk(expr)))
        }
    }

    fn async_values_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        if self.async_values.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(self.async_values.drain(..).map(|expr| ctx.b.async_thunk(expr)))
        }
    }

    fn blockers_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        let mut all_blockers: Vec<Expression<'a>> = self.blockers.iter()
            .map(|&idx| {
                ctx.b.computed_member_expr(
                    ctx.b.rid_expr("$$promises"),
                    ctx.b.num_expr(idx as f64),
                )
            })
            .collect();
        all_blockers.extend(self.extra_blockers.drain(..));
        if all_blockers.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(all_blockers)
        }
    }

    fn param_names(&self) -> Vec<String> {
        let total = self.sync_values.len() + self.async_values.len();
        (0..total).map(|i| format!("${i}")).collect()
    }

    fn part_expr(&self, ctx: &Ctx<'a>, part: TitleValuePart<'a>) -> Expression<'a> {
        match part {
            TitleValuePart::Str(value) => ctx.b.str_expr(&value),
            TitleValuePart::Expr(expr) => expr,
            TitleValuePart::SyncMemo(index) => ctx.b.rid_expr(&format!("${index}")),
            TitleValuePart::AsyncMemo(index) => {
                ctx.b.rid_expr(&format!("${}", self.sync_values.len() + index))
            }
        }
    }
}

/// Emit title element statements for a fragment (called before DOM init, like const_tags).
pub(crate) fn emit_title_elements<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    stmts: &mut Vec<Statement<'a>>,
) {
    let Some(ids) = ctx.analysis.title_elements.by_fragment(&key).cloned() else {
        return;
    };
    for id in ids {
        gen_title_element(ctx, id, stmts);
    }
}

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
    let (value, has_state, mut memoizer, single_dynamic_expr) = if let Some(svelte_analyze::FragmentItem::TextConcat { parts, .. }) = items.first() {
        let is_dyn = parts_are_dynamic(parts, ctx) || parts.iter().any(|part| {
            match part {
                LoweredTextPart::Expr(id) => ctx.expr_has_await(*id) || ctx.expr_has_blockers(*id),
                _ => false,
            }
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
    let assignment = ctx.b.assign_stmt(AssignLeft::StaticMember(title_member), value);

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
            let has_sync_values = memoizer.has_sync_values();
            let has_async_values = memoizer.has_async_values();
            let has_blockers = memoizer.has_blockers();
            let mut args = vec![Arg::Expr(arrow)];
            if has_sync_values || has_async_values || has_blockers {
                args.push(Arg::Expr(memoizer.sync_values_expr(ctx)));
            }
            if has_async_values || has_blockers {
                args.push(Arg::Expr(memoizer.async_values_expr(ctx)));
            }
            if has_blockers {
                args.push(Arg::Expr(memoizer.blockers_expr(ctx)));
            }
            stmts.push(ctx.b.call_stmt("$.deferred_template_effect", args));
        } else {
            stmts.push(ctx.b.call_stmt("$.deferred_template_effect", [Arg::Expr(arrow)]));
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
                push_title_text(&mut built_parts, part.text_value(&ctx.component.source).unwrap());
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
            other => TemplatePart::Expr(memoizer.part_expr(ctx, other)),
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
    if let ExpressionKind::Identifier(name) = &info.kind {
        ctx.known_value(name.as_str()).map(|s| s.to_string())
    } else {
        None
    }
}
