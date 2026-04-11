//! Expression parsing, concatenation building, and emit helpers.

use oxc_ast::ast::{Expression, Statement};
use oxc_ast_visit::VisitMut;
use rustc_hash::FxHashSet;

use svelte_analyze::ExpressionKind;
use svelte_analyze::{
    ExprHandle, ExprSite, ExpressionInfo, FragmentItem, FragmentKey, LoweredTextPart,
};
use svelte_ast::ConcatPart as AstConcatPart;
use svelte_ast::NodeId;

use crate::builder::{Arg, AssignLeft, TemplatePart};
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Pre-transformed expression lookup (handle-based)
// ---------------------------------------------------------------------------

/// Get a pre-transformed expression by handle.
/// Takes ownership via remove — each expression can only be consumed once.
pub(crate) fn take_expr<'a>(ctx: &mut Ctx<'a>, handle: ExprHandle) -> Expression<'a> {
    if let Some(expr) = ctx.state.parsed.take_expr(handle) {
        expr
    } else {
        debug_assert!(false, "missing pre-transformed expr at handle {:?}", handle);
        ctx.b.str_expr("")
    }
}

/// Get a pre-transformed expression from ParsedExprs by NodeId.
pub(crate) fn get_node_expr<'a>(ctx: &mut Ctx<'a>, node_id: NodeId) -> Expression<'a> {
    let mut expr = take_expr(ctx, ctx.node_expr_handle(node_id));
    finalize_await_exprs(ctx, Some(node_id), &mut expr);
    expr = maybe_wrap_legacy_coarse_expr(ctx, expr, ctx.expression(node_id));
    expr
}

pub(crate) fn get_attr_expr<'a>(ctx: &mut Ctx<'a>, attr_id: NodeId) -> Expression<'a> {
    let mut expr = take_expr(ctx, ctx.attr_expr_handle(attr_id));
    finalize_await_exprs(ctx, Some(attr_id), &mut expr);
    expr = maybe_wrap_legacy_coarse_expr(ctx, expr, ctx.attr_expression(attr_id));
    expr
}

/// Get a pre-transformed concat part expression by parser-owned handle.
pub(crate) fn get_concat_part_expr<'a>(ctx: &mut Ctx<'a>, handle: ExprHandle) -> Expression<'a> {
    let mut expr = take_expr(ctx, handle);
    finalize_await_exprs(ctx, None, &mut expr);
    expr
}

struct AwaitExprFinalizer<'c, 'a> {
    ctx: &'c Ctx<'a>,
    ignore_node: Option<NodeId>,
}

impl<'a> VisitMut<'a> for AwaitExprFinalizer<'_, 'a> {
    fn visit_expression(&mut self, expr: &mut Expression<'a>) {
        oxc_ast_visit::walk_mut::walk_expression(self, expr);

        let Expression::AwaitExpression(await_expr) = expr else {
            return;
        };

        let ignored = self
            .ignore_node
            .is_some_and(|id| self.ctx.is_ignored(id, "await_reactivity_loss"));

        let arg = self.ctx.b.move_expr(&mut await_expr.argument);
        if self.ctx.is_pickled_await(await_expr.span.start) {
            let save_call = self.ctx.b.call_expr("$.save", [Arg::Expr(arg)]);
            let awaited = self.ctx.b.await_expr(save_call);
            *expr = self
                .ctx
                .b
                .call_expr_callee(awaited, std::iter::empty::<Arg<'a, '_>>());
        } else if self.ctx.state.dev && !ignored {
            let track_call = self
                .ctx
                .b
                .call_expr("$.track_reactivity_loss", [Arg::Expr(arg)]);
            let awaited = self.ctx.b.await_expr(track_call);
            *expr = self
                .ctx
                .b
                .call_expr_callee(awaited, std::iter::empty::<Arg<'a, '_>>());
        } else {
            await_expr.argument = arg;
        }
    }
}

fn finalize_await_exprs<'a>(ctx: &Ctx<'a>, ignore_node: Option<NodeId>, expr: &mut Expression<'a>) {
    let mut finalizer = AwaitExprFinalizer { ctx, ignore_node };
    finalizer.visit_expression(expr);
}

fn maybe_wrap_legacy_coarse_expr<'a>(
    ctx: &Ctx<'a>,
    expr: Expression<'a>,
    info: Option<&ExpressionInfo>,
) -> Expression<'a> {
    let Some(info) = info else {
        return expr;
    };
    if ctx.query.runes() {
        return expr;
    }
    let needs_wrap = info.has_call
        || matches!(
            info.kind,
            ExpressionKind::MemberExpression | ExpressionKind::Assignment
        );
    if !needs_wrap {
        return expr;
    }

    let mut seq_parts: Vec<Expression<'a>> = Vec::new();
    for &sym in &info.ref_symbols {
        let is_prop_source = ctx.query.scoping().is_prop_source(sym);
        let is_template = ctx.query.scoping().is_template_declaration(sym);
        let is_import = ctx.query.scoping().is_import(sym);
        let is_rest_prop = ctx.query.scoping().is_rest_prop(sym);
        if !(is_prop_source || is_template || is_import || is_rest_prop) {
            continue;
        }

        let getter = if is_prop_source {
            ctx.b.call_expr(
                ctx.query.symbol_name(sym),
                std::iter::empty::<Arg<'a, '_>>(),
            )
        } else {
            ctx.b.rid_expr(ctx.query.symbol_name(sym))
        };
        let getter = ctx.b.call_expr("$.deep_read_state", [Arg::Expr(getter)]);
        seq_parts.push(getter);
    }

    if seq_parts.is_empty() {
        return expr;
    }
    let mut iter = seq_parts.into_iter();
    let mut sequence = iter
        .next()
        .unwrap_or_else(|| panic!("legacy coarse expression should have deps"));
    for next in iter.chain(std::iter::once(
        ctx.b.call_expr("$.untrack", [Arg::Expr(ctx.b.thunk(expr))]),
    )) {
        sequence = ctx.b.seq_expr([sequence, next]);
    }
    sequence
}

// ---------------------------------------------------------------------------
// Thunk builder — `() => expr` with rune transforms
// ---------------------------------------------------------------------------

/// Build `() => expr` from a pre-transformed expression (by NodeId).
pub(crate) fn build_node_thunk<'a>(ctx: &mut Ctx<'a>, node_id: NodeId) -> Expression<'a> {
    let expr = get_node_expr(ctx, node_id);
    ctx.b.thunk(expr)
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

/// Whether the given post-transform expression is statically known to be
/// defined (non-null, non-undefined). When true, `build_template_chunk` may
/// skip the `?? ""` wrap that protects against `"null"`/`"undefined"`
/// leaking into interpolated text.
///
/// Currently narrow: covers only `{#each}` index identifiers whose transform
/// kept them as bare identifiers (non-keyed blocks). In keyed blocks the
/// transform wraps the index in `$.get(idx)`, which produces a
/// `CallExpression` — the reference compiler's `scope.evaluate(...)` treats
/// that as unknown, so we must keep the `?? ""` fallback. Broadening this to
/// literals and numeric operations is tracked as a follow-up in
/// `specs/each-block.md`.
pub(crate) fn is_definitely_defined(ctx: &Ctx<'_>, nid: NodeId, expr: &Expression<'_>) -> bool {
    // Only trust the classification when transform left the expression as a
    // bare identifier. Once wrapped in a call (`$.get(...)`), the call result
    // type is unknown to the scope evaluator.
    if !matches!(expr, Expression::Identifier(_)) {
        return false;
    }
    let Some(info) = ctx.expression(nid) else {
        return false;
    };
    if !matches!(info.kind, ExpressionKind::Identifier(_)) {
        return false;
    }
    if info.ref_symbols.len() != 1 {
        return false;
    }
    ctx.is_each_index_sym(info.ref_symbols[0])
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
            LoweredTextPart::TextSpan(_) | LoweredTextPart::TextOwned(_) => {
                let s = part.text_value(&ctx.query.component.source).unwrap();
                // Merge with previous Str part if possible
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(s);
                } else {
                    tpl_parts.push(TemplatePart::Str(s.to_string()));
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
                    let defined = is_definitely_defined(ctx, *nid, &expr);
                    tpl_parts.push(TemplatePart::Expr(expr, defined));
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
    _attr_id: NodeId,
    parts: &[AstConcatPart],
) -> Expression<'a> {
    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    for part in parts {
        match part {
            AstConcatPart::Static(s) => push_template_str(&mut tpl_parts, s.clone()),
            AstConcatPart::Dynamic { span, .. } => {
                let expr = ctx
                    .state
                    .parsed
                    .expr_handle(span.start)
                    .map(|handle| get_concat_part_expr(ctx, handle))
                    .unwrap_or_else(|| ctx.b.str_expr(""));
                if let Some(value) = literal_concat_part_value(&expr) {
                    push_template_str(&mut tpl_parts, value);
                } else {
                    tpl_parts.push(TemplatePart::Expr(expr, false));
                }
            }
        }
    }

    if tpl_parts.len() == 1 {
        if let TemplatePart::Str(value) = &tpl_parts[0] {
            return ctx.b.str_expr(value);
        }
    }

    ctx.b.template_parts_expr(tpl_parts)
}

fn push_template_str<'a>(tpl_parts: &mut Vec<TemplatePart<'a>>, value: String) {
    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
        prev.push_str(&value);
    } else {
        tpl_parts.push(TemplatePart::Str(value));
    }
}

fn literal_concat_part_value(expr: &Expression<'_>) -> Option<String> {
    match expr {
        Expression::StringLiteral(lit) => Some(lit.value.as_str().to_string()),
        Expression::NumericLiteral(lit) => Some(lit.value.to_string()),
        Expression::BooleanLiteral(lit) => Some(lit.value.to_string()),
        Expression::NullLiteral(_) => Some(String::new()),
        _ => None,
    }
}

pub(crate) enum MemoValueRef {
    Sync(usize),
    Async(usize),
}

#[derive(Default)]
pub(crate) struct TemplateMemoState<'a> {
    sync_values: Vec<Expression<'a>>,
    async_values: Vec<Expression<'a>>,
    blockers: Vec<u32>,
    extra_blockers: Vec<Expression<'a>>,
}

impl<'a> TemplateMemoState<'a> {
    pub(crate) fn push_script_blocker(&mut self, idx: u32) {
        if !self.blockers.contains(&idx) {
            self.blockers.push(idx);
        }
    }

    pub(crate) fn push_expr_info(&mut self, ctx: &Ctx<'a>, info: &ExpressionInfo) {
        for sym in &info.ref_symbols {
            if let Some(idx) = ctx.symbol_blocker(*sym) {
                self.push_script_blocker(idx);
            }
            if let Some(expr) = ctx.const_tag_symbol_blocker_expr(*sym) {
                self.extra_blockers.push(expr);
            }
        }
    }

    pub(crate) fn push_node_deps(&mut self, ctx: &mut Ctx<'a>, id: NodeId) {
        let blockers = ctx
            .expr_deps(ExprSite::Node(id))
            .map(|deps| deps.blockers)
            .unwrap_or_default();
        for idx in blockers {
            self.push_script_blocker(idx);
        }
        self.extra_blockers.extend(ctx.const_tag_blocker_exprs(id));
    }

    pub(crate) fn add_memoized_expr(
        &mut self,
        ctx: &Ctx<'a>,
        info: &ExpressionInfo,
        expr: Expression<'a>,
    ) -> Option<MemoValueRef> {
        self.push_expr_info(ctx, info);

        if !(info.has_call || info.has_await) || info.ref_symbols.is_empty() {
            return None;
        }

        if info.has_await {
            let index = self.async_values.len();
            self.async_values.push(expr);
            Some(MemoValueRef::Async(index))
        } else {
            let index = self.sync_values.len();
            self.sync_values.push(expr);
            Some(MemoValueRef::Sync(index))
        }
    }

    pub(crate) fn has_deps(&self) -> bool {
        !self.sync_values.is_empty()
            || !self.async_values.is_empty()
            || !self.blockers.is_empty()
            || !self.extra_blockers.is_empty()
    }

    pub(crate) fn has_sync_values(&self) -> bool {
        !self.sync_values.is_empty()
    }

    pub(crate) fn has_async_values(&self) -> bool {
        !self.async_values.is_empty()
    }

    pub(crate) fn has_blockers(&self) -> bool {
        !self.blockers.is_empty() || !self.extra_blockers.is_empty()
    }

    pub(crate) fn param_names(&self) -> Vec<String> {
        let total = self.sync_values.len() + self.async_values.len();
        (0..total).map(|i| format!("${i}")).collect()
    }

    pub(crate) fn sync_param_expr(&self, ctx: &Ctx<'a>, index: usize) -> Expression<'a> {
        ctx.b.rid_expr(&format!("${index}"))
    }

    pub(crate) fn async_param_expr(&self, ctx: &Ctx<'a>, index: usize) -> Expression<'a> {
        ctx.b
            .rid_expr(&format!("${}", self.sync_values.len() + index))
    }

    pub(crate) fn sync_values_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        if self.sync_values.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(
                self.sync_values
                    .drain(..)
                    .map(|expr| ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(expr)])),
            )
        }
    }

    pub(crate) fn async_values_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        if self.async_values.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(
                self.async_values
                    .drain(..)
                    .map(|expr| async_value_thunk(ctx, expr)),
            )
        }
    }

    pub(crate) fn blockers_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        let mut all_blockers: Vec<Expression<'a>> = self
            .blockers
            .iter()
            .map(|&idx| {
                ctx.b
                    .computed_member_expr(ctx.b.rid_expr("$$promises"), ctx.b.num_expr(idx as f64))
            })
            .collect();
        all_blockers.extend(self.extra_blockers.drain(..));
        if all_blockers.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(all_blockers)
        }
    }
}

pub(crate) fn emit_effect_call<'a>(
    ctx: &Ctx<'a>,
    effect_name: &str,
    callback: Expression<'a>,
    deps: &mut TemplateMemoState<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    if !deps.has_deps() {
        body.push(ctx.b.call_stmt(effect_name, [Arg::Expr(callback)]));
        return;
    }

    let has_sync_values = deps.has_sync_values();
    let has_async_values = deps.has_async_values();
    let has_blockers = deps.has_blockers();
    let mut args = vec![Arg::Expr(callback)];
    if has_sync_values || has_async_values || has_blockers {
        args.push(Arg::Expr(deps.sync_values_expr(ctx)));
    }
    if has_async_values || has_blockers {
        args.push(Arg::Expr(deps.async_values_expr(ctx)));
    }
    if has_blockers {
        args.push(Arg::Expr(deps.blockers_expr(ctx)));
    }
    body.push(ctx.b.call_stmt(effect_name, args));
}

fn async_value_thunk<'a>(ctx: &Ctx<'a>, expr: Expression<'a>) -> Expression<'a> {
    if let Expression::AwaitExpression(await_expr) = expr {
        let inner = await_expr.unbox().argument;
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(inner)])
    } else {
        ctx.b.async_arrow_expr_body(expr)
    }
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
        // Collect script-level blockers and const-tag blockers
        let mut blockers: Vec<u32> = Vec::new();
        let mut extra_blockers: Vec<oxc_ast::ast::Expression<'a>> = Vec::new();
        if let FragmentItem::TextConcat { parts, .. } = item {
            for part in parts {
                if let LoweredTextPart::Expr(id) = part {
                    let deps = ctx
                        .expr_deps(ExprSite::Node(*id))
                        .unwrap_or_else(|| panic!("missing expression deps for {:?}", id));
                    for idx in deps.blockers {
                        if !blockers.contains(&idx) {
                            blockers.push(idx);
                        }
                    }
                    extra_blockers.extend(ctx.const_tag_blocker_exprs(*id));
                }
            }
            blockers.sort_unstable();
        }
        let set = ctx
            .b
            .call_stmt("$.set_text", [Arg::Ident(node_name), Arg::Expr(expr)]);
        emit_template_effect_with_blockers(ctx, vec![set], blockers, extra_blockers, body);
    } else {
        body.push(ctx.b.assign_stmt(
            AssignLeft::StaticMember(ctx.b.static_member(ctx.b.rid_expr(node_name), "nodeValue")),
            expr,
        ));
    }
}

/// Emit `$.template_effect(callback, sync_values?, async_values?, blockers?)`.
/// `script_blockers`: indices into `$$promises` (script-level async).
/// `extra_blockers`: pre-built expressions like `promises[M]` (const-tag blockers).
pub(crate) fn emit_template_effect_with_blockers<'a>(
    ctx: &mut Ctx<'a>,
    update: Vec<Statement<'a>>,
    script_blockers: Vec<u32>,
    extra_blockers: Vec<oxc_ast::ast::Expression<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    if update.is_empty() {
        return;
    }
    let eff = ctx.b.arrow_expr(ctx.b.no_params(), update);
    let mut deps = TemplateMemoState::default();
    for idx in script_blockers {
        deps.push_script_blocker(idx);
    }
    deps.extra_blockers.extend(extra_blockers);
    emit_effect_call(ctx, "$.template_effect", eff, &mut deps, body);
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
        FragmentItem::Element(id)
        | FragmentItem::ComponentNode(id)
        | FragmentItem::IfBlock(id)
        | FragmentItem::EachBlock(id)
        | FragmentItem::RenderTag(id)
        | FragmentItem::HtmlTag(id)
        | FragmentItem::KeyBlock(id)
        | FragmentItem::SvelteElement(id)
        | FragmentItem::SvelteBoundary(id)
        | FragmentItem::AwaitBlock(id) => ctx.is_dynamic(*id),
    }
}

pub(crate) fn item_has_local_blockers(item: &FragmentItem, ctx: &Ctx<'_>) -> bool {
    let FragmentItem::TextConcat { parts, .. } = item else {
        return false;
    };
    parts.iter().any(|part| {
        let LoweredTextPart::Expr(id) = part else {
            return false;
        };
        ctx.expression(*id).is_some_and(|info| {
            info.ref_symbols
                .iter()
                .any(|sym| ctx.const_tag_symbol_blocker_expr(*sym).is_some())
        })
    })
}

pub(crate) fn build_fragment_local_blockers<'a>(
    ctx: &Ctx<'a>,
    key: &FragmentKey,
) -> Vec<Expression<'a>> {
    let mut out = Vec::new();
    let mut seen_syms = FxHashSet::default();
    let items = &ctx.lowered_fragment(key).items;
    for item in items {
        let FragmentItem::TextConcat { parts, .. } = item else {
            continue;
        };
        for part in parts {
            let LoweredTextPart::Expr(id) = part else {
                continue;
            };
            if let Some(info) = ctx.expression(*id) {
                for sym in &info.ref_symbols {
                    if seen_syms.insert(*sym) {
                        if let Some(expr) = ctx.const_tag_symbol_blocker_expr(*sym) {
                            out.push(expr);
                        }
                    }
                }
            }
        }
    }
    out
}

/// Check if a text content expression needs call memoization.
/// Requires `has_call` AND references to resolved bindings (not just rune names).
pub(crate) fn text_content_needs_memo(item: &FragmentItem, ctx: &Ctx<'_>) -> bool {
    if let FragmentItem::TextConcat { parts, .. } = item {
        return parts.iter().any(|p| {
            if let LoweredTextPart::Expr(id) = p {
                ctx.expr_deps(ExprSite::Node(*id))
                    .is_some_and(|deps| deps.needs_memo)
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
    item: &FragmentItem,
    text_name: &str,
    body: &mut Vec<Statement<'a>>,
) {
    let FragmentItem::TextConcat { parts, .. } = item else {
        return;
    };
    let mut deps = TemplateMemoState::default();
    let expr = build_concat_with_memo(ctx, parts, &mut deps);

    let param_names = deps.param_names();
    let params = if param_names.is_empty() {
        ctx.b.no_params()
    } else {
        ctx.b.params(param_names.iter().map(|s| s.as_str()))
    };
    let set_text = ctx
        .b
        .call_stmt("$.set_text", [Arg::Ident(text_name), Arg::Expr(expr)]);
    let callback = ctx.b.arrow_expr(params, [set_text]);
    emit_effect_call(ctx, "$.template_effect", callback, &mut deps, body);
}

/// Attribute update that needs call memoization — expression extracted into dependency array.
pub(crate) enum MemoAttrUpdate {
    Call {
        setter_fn: &'static str,
        attr_name: Option<String>,
    },
    Assignment {
        property: String,
    },
}

pub(crate) struct MemoAttr<'a> {
    pub attr_id: NodeId,
    pub el_name: String,
    pub update: MemoAttrUpdate,
    pub expr: Expression<'a>,
}

/// Emit `$.template_effect` combining regular updates with memoized attribute expressions.
///
/// Memoized attrs get `$N` parameter placeholders; their expressions become getter thunks
/// in the dependency array.
pub(crate) fn emit_template_effect_with_memo<'a>(
    ctx: &mut Ctx<'a>,
    regular_updates: Vec<Statement<'a>>,
    memo_attrs: Vec<MemoAttr<'a>>,
    script_blockers: Vec<u32>,
    extra_blockers: Vec<Expression<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    if memo_attrs.is_empty() {
        emit_template_effect_with_blockers(
            ctx,
            regular_updates,
            script_blockers,
            extra_blockers,
            body,
        );
        return;
    }

    // Collect memo data: (param_name, el_name, update, expr)
    let memo_count = memo_attrs.len();
    let mut param_names: Vec<String> = Vec::with_capacity(memo_count);
    let mut memo_data: Vec<(NodeId, String, MemoAttrUpdate, Expression<'a>)> =
        Vec::with_capacity(memo_count);

    for (i, memo) in memo_attrs.into_iter().enumerate() {
        param_names.push(format!("${i}"));
        memo_data.push((memo.attr_id, memo.el_name, memo.update, memo.expr));
    }

    // Build getter thunks and setter stmts (needs references to param_names)
    let mut deps = TemplateMemoState::default();
    for idx in script_blockers {
        deps.push_script_blocker(idx);
    }
    deps.extra_blockers.extend(extra_blockers);
    let mut callback_body = regular_updates;

    for (i, (attr_id, el_name, update, expr)) in memo_data.into_iter().enumerate() {
        let memo_expr = ctx.b.rid_expr(&param_names[i]);
        match update {
            MemoAttrUpdate::Call {
                setter_fn,
                attr_name,
            } => {
                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(ctx.b.rid_expr(&el_name))];
                if let Some(name) = attr_name {
                    args.push(Arg::Str(name));
                }
                args.push(Arg::Expr(memo_expr));
                callback_body.push(ctx.b.call_stmt(setter_fn, args));
            }
            MemoAttrUpdate::Assignment { property } => {
                callback_body.push(ctx.b.assign_stmt(
                    AssignLeft::StaticMember(
                        ctx.b.static_member(ctx.b.rid_expr(&el_name), &property),
                    ),
                    memo_expr,
                ));
            }
        }

        let attr_deps = ctx
            .expr_deps(ExprSite::Attr(attr_id))
            .unwrap_or_else(|| panic!("memoized attribute should have expression deps"));
        deps.push_expr_info(ctx, attr_deps.info);
        if attr_deps.has_await() {
            deps.async_values.push(expr);
        } else {
            deps.sync_values.push(expr);
        }
    }

    let params = ctx.b.params(param_names.iter().map(|s| s.as_str()));
    let callback = ctx.b.arrow_expr(params, callback_body);
    emit_effect_call(ctx, "$.template_effect", callback, &mut deps, body);
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

fn build_concat_with_memo<'a>(
    ctx: &mut Ctx<'a>,
    parts: &[LoweredTextPart],
    deps: &mut TemplateMemoState<'a>,
) -> Expression<'a> {
    if parts.len() == 1 {
        if let LoweredTextPart::Expr(nid) = parts[0] {
            deps.push_node_deps(ctx, nid);
            if let Some(val) = try_resolve_known(ctx, nid) {
                return ctx.b.str_expr(&val);
            }

            let expr = get_node_expr(ctx, nid);
            let node_deps = ctx
                .expr_deps(ExprSite::Node(nid))
                .unwrap_or_else(|| panic!("missing expression deps for {:?}", nid));
            if node_deps.needs_memo {
                if node_deps.has_await() {
                    let index = deps.async_values.len();
                    deps.async_values.push(ctx.b.clone_expr(&expr));
                    return deps.async_param_expr(ctx, index);
                }

                let index = deps.sync_values.len();
                deps.sync_values.push(ctx.b.clone_expr(&expr));
                return deps.sync_param_expr(ctx, index);
            }
            return expr;
        }
    }

    let mut tpl_parts: Vec<TemplatePart<'a>> = Vec::new();
    for part in parts {
        match part {
            LoweredTextPart::TextSpan(_) | LoweredTextPart::TextOwned(_) => {
                let s = part.text_value(&ctx.query.component.source).unwrap();
                if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                    prev.push_str(s);
                } else {
                    tpl_parts.push(TemplatePart::Str(s.to_string()));
                }
            }
            LoweredTextPart::Expr(nid) => {
                deps.push_node_deps(ctx, *nid);
                if let Some(val) = try_resolve_known(ctx, *nid) {
                    if let Some(TemplatePart::Str(prev)) = tpl_parts.last_mut() {
                        prev.push_str(&val);
                    } else {
                        tpl_parts.push(TemplatePart::Str(val));
                    }
                    continue;
                }

                let expr = get_node_expr(ctx, *nid);
                let defined = is_definitely_defined(ctx, *nid, &expr);
                let node_deps = ctx
                    .expr_deps(ExprSite::Node(*nid))
                    .unwrap_or_else(|| panic!("missing expression deps for {:?}", nid));
                let expr = if node_deps.needs_memo {
                    // Memo params (`$0`, `$1`) lose the original expression
                    // shape — treat them as not-definitely-defined.
                    if node_deps.has_await() {
                        let index = deps.async_values.len();
                        deps.async_values.push(ctx.b.clone_expr(&expr));
                        deps.async_param_expr(ctx, index)
                    } else {
                        let index = deps.sync_values.len();
                        deps.sync_values.push(ctx.b.clone_expr(&expr));
                        deps.sync_param_expr(ctx, index)
                    }
                } else {
                    expr
                };
                tpl_parts.push(TemplatePart::Expr(expr, defined));
            }
        }
    }

    if tpl_parts.len() == 1 {
        if let TemplatePart::Str(s) = &tpl_parts[0] {
            return ctx.b.str_expr(s);
        }
    }

    ctx.b.template_parts_expr(tpl_parts)
}
