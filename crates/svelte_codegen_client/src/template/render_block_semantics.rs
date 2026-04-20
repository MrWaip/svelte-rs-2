//! RenderTag codegen driven by Block Semantics — `{@render expr(args)}`.
//!
//! Root consumer for the `BlockSemantics::Render` slice. Reads one
//! `RenderTagBlockSemantics` answer, resolves it into a plain-data
//! [`RenderPlan`], and then dispatches to focused builders — one per
//! piece of the emitted call (argument thunks, final call shape, and
//! the optional `$.async(...)` wrapper).
//!
//! Consumer Model (see SEMANTIC_LAYER_ARCHITECTURE.md): consumer takes
//! one query per tag, resolves all decisions up front into `RenderPlan`
//! in a single pass over `sem`, then passes `&RenderPlan` (not `&sem`)
//! to each builder. Builders never re-interpret semantics. No
//! `ExpressionInfo` / `AsyncEmissionPlan` / `RenderTagPlan` reads —
//! every decision is already on the payload or the plan derived from it.

use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{Argument, Expression, Statement};
use oxc_span::GetSpan;

use svelte_analyze::{
    RenderArgLowering, RenderAsyncKind, RenderCalleeShape, RenderTagBlockSemantics,
};
use svelte_ast::NodeId;

use crate::context::Ctx;
use svelte_ast_builder::Arg;

/// Generate statements for a `{@render expr(args)}` tag.
///
/// `sem` is already resolved — the root match on
/// `block_semantics(id)` happened at the dispatch site (see
/// `traverse.rs` / `template/mod.rs`).
pub(crate) fn gen_render_block<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    sem: &RenderTagBlockSemantics,
    anchor_expr: Expression<'a>,
    is_standalone: bool,
    stmts: &mut Vec<Statement<'a>>,
) {
    // Take ownership of the pre-parsed expression and split it into
    // callee / arguments. Legacy analyze no longer pre-unwraps
    // `ChainExpression` — the chain-ness axis now lives on
    // `sem.callee_shape`, so we only need to skip the wrapper here.
    let expr = ctx
        .state
        .parsed
        .take_expr(ctx.node_expr_handle(id))
        .expect("render tag expression should be pre-parsed");
    let call = match expr {
        Expression::CallExpression(call) => call,
        Expression::ChainExpression(chain) => match chain.unbox().expression {
            oxc_ast::ast::ChainElement::CallExpression(call) => call,
            _ => return,
        },
        _ => return,
    };
    let callee_span = call.callee.span();
    let unboxed = call.unbox();
    let callee_expr = unboxed.callee;
    let arguments = unboxed.arguments;

    let plan = RenderPlan::build(ctx, id, sem, callee_span, is_standalone, anchor_expr);

    let mut memo_stmts: Vec<Statement<'a>> = Vec::new();
    let mut async_values: Vec<Expression<'a>> = Vec::new();

    let arg_thunks = build_arg_thunks(
        ctx,
        arguments,
        sem,
        &plan,
        &mut memo_stmts,
        &mut async_values,
    );
    let final_stmt = build_final_call(ctx, &plan, callee_expr, arg_thunks);

    if plan.needs_async {
        emit_async_wrapped(ctx, &plan, final_stmt, memo_stmts, async_values, stmts);
    } else if memo_stmts.is_empty() {
        stmts.push(final_stmt);
    } else {
        memo_stmts.push(final_stmt);
        stmts.push(ctx.b.block_stmt(memo_stmts));
    }
}

// ---------------------------------------------------------------------------
// Plan — pre-computed, plain-data snapshot consumed by the builders.
// ---------------------------------------------------------------------------

/// Every decision the output builders need, resolved up front from
/// `sem` and the source spans. Keeps builders on `&RenderPlan` only —
/// they never re-read `sem`, matching the Consumer Shape rule
/// (Plan → builders → emit).
struct RenderPlan<'a> {
    callee_shape: RenderCalleeShape,
    /// `true` when the render call must be wrapped in `$.async(...)`.
    needs_async: bool,
    /// Sorted, de-duplicated script-level blocker indices for the
    /// async wrapper (empty when `needs_async` is `false`).
    blockers: Vec<u32>,
    /// Number of `MemoSync` args — pre-computed so `MemoAsync` params
    /// can reference `${sync_memo_count + i}` in one pass.
    sync_memo_count: u32,
    /// Number of `MemoAsync` args — emitted as `async_values` entries
    /// and extra callback parameters.
    async_memo_count: u32,
    /// `$$anchor` vs `node` — the parameter name bound by the async
    /// callback, or used directly in the sync call when the render tag
    /// is the sole child of its fragment.
    anchor_name: &'static str,
    /// Source text of the non-dynamic callee — used to emit
    /// `fn(...)` / `fn?.(...)` for `Static` / `StaticChain`. Owned
    /// source slice from `component.source`.
    callee_text: &'a str,
    /// Inner anchor expression — the anchor passed to the render call
    /// itself (either the incoming `anchor_expr` for sync, or a
    /// reference to `anchor_name` bound by the async callback).
    inner_anchor: Expression<'a>,
    /// Outer anchor forwarded to `$.async(anchor, ...)`; only populated
    /// when `needs_async` is `true`.
    outer_anchor: Option<Expression<'a>>,
    /// `true` when this render tag is the sole child of its fragment —
    /// triggers the trailing `$.next()` statement after async wrapping.
    is_standalone: bool,
}

impl<'a> RenderPlan<'a> {
    fn build(
        ctx: &mut Ctx<'a>,
        id: NodeId,
        sem: &RenderTagBlockSemantics,
        callee_span: oxc_span::Span,
        is_standalone: bool,
        anchor_expr: Expression<'a>,
    ) -> Self {
        let tag = ctx.render_tag(id);
        let full_source = ctx.query.component.source_text(tag.expression_span);
        // `call.callee.span()` is 0-based relative to the pre-parsed
        // expression — `full_source` is the same slice.
        let callee_text = &full_source[callee_span.start as usize..callee_span.end as usize];

        let (needs_async, blockers) = match &sem.async_kind {
            RenderAsyncKind::Sync => (false, Vec::new()),
            RenderAsyncKind::Async { blockers } => (true, blockers.to_vec()),
        };

        let mut sync_memo_count: u32 = 0;
        let mut async_memo_count: u32 = 0;
        for arg in &sem.args {
            match arg {
                RenderArgLowering::MemoSync => sync_memo_count += 1,
                RenderArgLowering::MemoAsync => async_memo_count += 1,
                _ => {}
            }
        }

        let anchor_name = if is_standalone { "$$anchor" } else { "node" };
        let (inner_anchor, outer_anchor) = if needs_async {
            (ctx.b.rid_expr(anchor_name), Some(anchor_expr))
        } else {
            (anchor_expr, None)
        };

        Self {
            callee_shape: sem.callee_shape,
            needs_async,
            blockers,
            sync_memo_count,
            async_memo_count,
            anchor_name,
            callee_text,
            inner_anchor,
            outer_anchor,
            is_standalone,
        }
    }
}

// ---------------------------------------------------------------------------
// Per-argument builder.
// ---------------------------------------------------------------------------

/// One ARM per `RenderArgLowering` variant. `memo_stmts` accumulates
/// `let $N = $.derived(...)` declarations emitted for every `MemoSync`
/// argument; `async_values` accumulates the expressions that feed the
/// `$.async(..., [async_values], ...)` array. The two share the same
/// `$N` identifier pool — sync first (0..sync_memo_count), async second
/// (sync_memo_count..sync+async) — so the callback parameters line up
/// with the reads inside the thunks.
fn build_arg_thunks<'a, 'alloc>(
    ctx: &mut Ctx<'a>,
    arguments: OxcVec<'alloc, Argument<'a>>,
    sem: &RenderTagBlockSemantics,
    plan: &RenderPlan<'a>,
    memo_stmts: &mut Vec<Statement<'a>>,
    async_values: &mut Vec<Expression<'a>>,
) -> Vec<Arg<'a, 'static>> {
    let mut thunks: Vec<Arg<'a, 'static>> = Vec::with_capacity(sem.args.len());
    let mut sync_seen: u32 = 0;
    let mut async_seen: u32 = 0;
    for (arg, arg_sem) in arguments.into_iter().zip(sem.args.iter()) {
        let arg_expr = arg.into_expression();
        match *arg_sem {
            RenderArgLowering::PropSource { sym } => {
                let name = ctx.symbol_name(sym);
                thunks.push(Arg::Expr(ctx.b.rid_expr(name)));
            }
            RenderArgLowering::MemoSync => {
                let name = format!("${sync_seen}");
                sync_seen += 1;
                let thunk = ctx.b.thunk(arg_expr);
                let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                memo_stmts.push(ctx.b.let_init_stmt(&name, derived));
                let name_ref = ctx.b.alloc_str(&name);
                let get = ctx.b.call_expr("$.get", [Arg::Ident(name_ref)]);
                let read_thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(get)]);
                thunks.push(Arg::Expr(read_thunk));
            }
            RenderArgLowering::MemoAsync => {
                async_values.push(ctx.b.clone_expr(&arg_expr));
                let param_name = format!("${}", plan.sync_memo_count + async_seen);
                async_seen += 1;
                let param_ref = ctx.b.alloc_str(&param_name);
                let get = ctx.b.call_expr("$.get", [Arg::Ident(param_ref)]);
                let read_thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(get)]);
                thunks.push(Arg::Expr(read_thunk));
            }
            RenderArgLowering::Plain => {
                let thunk = ctx
                    .b
                    .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(arg_expr)]);
                thunks.push(Arg::Expr(thunk));
            }
        }
    }
    thunks
}

// ---------------------------------------------------------------------------
// Final call builder.
// ---------------------------------------------------------------------------

/// Build the render call itself. Each `callee_shape` variant chooses
/// one shape and never falls through to another — legacy "is_dynamic"
/// / "is_chain" bit-checks are already folded into the enum.
fn build_final_call<'a>(
    ctx: &mut Ctx<'a>,
    plan: &RenderPlan<'a>,
    callee_expr: Expression<'a>,
    arg_thunks: Vec<Arg<'a, 'static>>,
) -> Statement<'a> {
    let inner_anchor = ctx.b.clone_expr(&plan.inner_anchor);
    match plan.callee_shape {
        RenderCalleeShape::Dynamic | RenderCalleeShape::DynamicChain => {
            let callee_arg = build_dynamic_callee(
                ctx,
                callee_expr,
                matches!(plan.callee_shape, RenderCalleeShape::DynamicChain),
            );
            let mut snippet_args: Vec<Arg<'a, '_>> =
                vec![Arg::Expr(inner_anchor), Arg::Expr(callee_arg)];
            snippet_args.extend(arg_thunks);
            ctx.b.call_stmt("$.snippet", snippet_args)
        }
        RenderCalleeShape::StaticChain => {
            let callee_expr = ctx.b.rid_expr(plan.callee_text);
            let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(inner_anchor)];
            all_args.extend(arg_thunks);
            let call_expr = ctx.b.maybe_call_expr(callee_expr, all_args);
            ctx.b.expr_stmt(call_expr)
        }
        RenderCalleeShape::Static => {
            let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(inner_anchor)];
            all_args.extend(arg_thunks);
            ctx.b.call_stmt(plan.callee_text, all_args)
        }
    }
}

fn build_dynamic_callee<'a>(
    ctx: &mut Ctx<'a>,
    callee_expr: Expression<'a>,
    is_chain: bool,
) -> Expression<'a> {
    if is_chain {
        let coalesced = ctx
            .b
            .logical_coalesce(callee_expr, ctx.b.rid_expr("$.noop"));
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(coalesced)])
    } else {
        ctx.b.thunk(callee_expr)
    }
}

// ---------------------------------------------------------------------------
// Async wrapper.
// ---------------------------------------------------------------------------

/// Wrap the final call (plus any leading `$.derived` memo bindings) in
/// `$.async(anchor, [blockers], [async_values], (anchor, $N...) => { ... })`
/// and append a trailing `$.next()` when the tag is standalone.
fn emit_async_wrapped<'a>(
    ctx: &mut Ctx<'a>,
    plan: &RenderPlan<'a>,
    final_stmt: Statement<'a>,
    mut memo_stmts: Vec<Statement<'a>>,
    async_values: Vec<Expression<'a>>,
    stmts: &mut Vec<Statement<'a>>,
) {
    memo_stmts.push(final_stmt);

    // Script-level blocker index → `$$promises[idx]`. Matches the
    // encoding used by every other async-wrapped template effect
    // (`TemplateMemoState::blockers_expr`).
    let blockers_expr = if plan.blockers.is_empty() {
        ctx.b.void_zero_expr()
    } else {
        ctx.b.array_expr(plan.blockers.iter().map(|&idx| {
            ctx.b
                .computed_member_expr(ctx.b.rid_expr("$$promises"), ctx.b.num_expr(idx as f64))
        }))
    };

    let async_values_expr = if async_values.is_empty() {
        ctx.b.void_zero_expr()
    } else {
        let thunks: Vec<Expression<'a>> = async_values
            .into_iter()
            .map(|expr| async_value_thunk(ctx, expr))
            .collect();
        ctx.b.array_expr(thunks)
    };

    let callback_params = if plan.async_memo_count > 0 {
        let mut names = vec![plan.anchor_name.to_string()];
        for i in 0..plan.async_memo_count {
            names.push(format!("${}", plan.sync_memo_count + i));
        }
        ctx.b.params(names.iter().map(|s| s.as_str()))
    } else {
        ctx.b.params([plan.anchor_name])
    };
    let callback = ctx.b.arrow_block_expr(callback_params, memo_stmts);

    let outer_anchor = plan
        .outer_anchor
        .as_ref()
        .map(|e| ctx.b.clone_expr(e))
        .expect("needs_async path always allocates outer_anchor");

    stmts.push(ctx.b.call_stmt(
        "$.async",
        [
            Arg::Expr(outer_anchor),
            Arg::Expr(blockers_expr),
            Arg::Expr(async_values_expr),
            Arg::Expr(callback),
        ],
    ));
    if plan.is_standalone {
        stmts.push(ctx.b.call_stmt("$.next", std::iter::empty::<Arg>()));
    }
}

/// Async-value thunk matching legacy `TemplateMemoState::async_values_expr`:
/// when the argument is literally an `AwaitExpression`, unwrap it and
/// emit a sync arrow over the inner expression (the value resolves via
/// the wrapper's `$.async` loop). Otherwise emit an `async () => expr`
/// whose returned promise feeds the wrapper.
fn async_value_thunk<'a>(ctx: &mut Ctx<'a>, expr: Expression<'a>) -> Expression<'a> {
    if let Expression::AwaitExpression(await_expr) = expr {
        let inner = await_expr.unbox().argument;
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(inner)])
    } else {
        ctx.b.async_arrow_expr_body(expr)
    }
}
