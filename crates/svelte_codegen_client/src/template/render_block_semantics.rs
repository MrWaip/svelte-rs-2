//! RenderTag codegen driven by Block Semantics — `{@render expr(args)}`.
//!
//! Root consumer for the `BlockSemantics::Render` slice. Reads one
//! `RenderTagBlockSemantics` answer and dispatches to focused builders
//! for each piece of the emitted call: argument thunks, final call
//! shape, and the optional `$.async(...)` wrapper.
//!
//! Consumer Model (see SEMANTIC_LAYER_ARCHITECTURE.md): consumer takes
//! one query per tag, matches once on `callee_shape` and on each
//! `args[i]` variant. No `ExpressionInfo` / `AsyncEmissionPlan` /
//! `RenderTagPlan` reads — every decision is already on the payload.
//!
//! Plan → builders → emit form is closely modeled on `each_block.rs`.
//! The Plan snapshot is light here (the semantics answer is already
//! plain-data), so "Plan" is implicit: `sem` IS the plan.

use oxc_ast::ast::{Expression, Statement};
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
    let needs_async = matches!(sem.async_kind, RenderAsyncKind::Async { .. });

    // When async-wrapped, the inner render call uses the callback
    // parameter as anchor; `anchor_expr` is forwarded into
    // `$.async(...)` as the outer anchor.
    let anchor_name = if is_standalone { "$$anchor" } else { "node" };
    let (inner_anchor, outer_anchor) = if needs_async {
        (ctx.b.rid_expr(anchor_name), Some(anchor_expr))
    } else {
        (anchor_expr, None)
    };

    let tag = ctx.render_tag(id);
    let full_source = ctx.query.component.source_text(tag.expression_span);

    // Take ownership of the pre-parsed expression. Legacy render tag
    // classification no longer pre-unwraps `ChainExpression`, so we
    // handle both shapes on the take side. The `is_chain` axis is
    // already folded into `sem.callee_shape` — no need to re-read it.
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

    // Callee text for `Static` / `StaticChain` — read from original
    // source so non-dynamic normal-binding identifiers keep their
    // untransformed form.
    let callee_span = call.callee.span();
    let callee_text = &full_source[callee_span.start as usize..callee_span.end as usize];

    let unboxed = call.unbox();
    let callee_expr = unboxed.callee;

    // Legacy `Memoizer` numbers sync and async memos in one shared
    // `$0, $1, ...` pool — sync first, async second. Compute the
    // sync-memo count up front so async thunks can reference
    // `${sync_count + async_index}` in one pass.
    let sync_memo_count = sem
        .args
        .iter()
        .filter(|a| matches!(**a, RenderArgLowering::MemoSync))
        .count() as u32;

    let mut memo_stmts: Vec<Statement<'a>> = Vec::new();
    let mut async_values: Vec<Expression<'a>> = Vec::new();
    let mut async_memo_count: u32 = 0;
    let mut sync_memo_seen: u32 = 0;

    let arg_thunks = build_arg_thunks(
        ctx,
        unboxed.arguments,
        sem,
        sync_memo_count,
        &mut memo_stmts,
        &mut async_values,
        &mut sync_memo_seen,
        &mut async_memo_count,
    );

    let final_stmt = match sem.callee_shape {
        RenderCalleeShape::Dynamic | RenderCalleeShape::DynamicChain => {
            let callee_arg = build_dynamic_callee(
                ctx,
                callee_expr,
                matches!(sem.callee_shape, RenderCalleeShape::DynamicChain),
            );
            let mut snippet_args: Vec<Arg<'a, '_>> =
                vec![Arg::Expr(inner_anchor), Arg::Expr(callee_arg)];
            snippet_args.extend(arg_thunks);
            ctx.b.call_stmt("$.snippet", snippet_args)
        }
        RenderCalleeShape::StaticChain => {
            let callee_expr = ctx.b.rid_expr(callee_text);
            let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(inner_anchor)];
            all_args.extend(arg_thunks);
            let call_expr = ctx.b.maybe_call_expr(callee_expr, all_args);
            ctx.b.expr_stmt(call_expr)
        }
        RenderCalleeShape::Static => {
            let mut all_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(inner_anchor)];
            all_args.extend(arg_thunks);
            ctx.b.call_stmt(callee_text, all_args)
        }
    };

    match &sem.async_kind {
        RenderAsyncKind::Async { blockers } => {
            emit_async_wrapped(
                ctx,
                stmts,
                outer_anchor.expect("needs_async path always allocates outer_anchor"),
                blockers,
                async_values,
                memo_stmts,
                final_stmt,
                anchor_name,
                sync_memo_count,
                async_memo_count,
                is_standalone,
            );
        }
        RenderAsyncKind::Sync => {
            if memo_stmts.is_empty() {
                stmts.push(final_stmt);
            } else {
                memo_stmts.push(final_stmt);
                stmts.push(ctx.b.block_stmt(memo_stmts));
            }
        }
    }
}

/// Per-argument dispatch: one ARM per `RenderArgLowering` variant.
/// Sync/async memo numbers share the same `$N` pool — sync first (0..
/// sync_memo_count), async second (sync_memo_count..sync+async).
#[allow(clippy::too_many_arguments)]
fn build_arg_thunks<'a, 'alloc>(
    ctx: &mut Ctx<'a>,
    arguments: oxc_allocator::Vec<'alloc, oxc_ast::ast::Argument<'a>>,
    sem: &RenderTagBlockSemantics,
    sync_memo_count: u32,
    memo_stmts: &mut Vec<Statement<'a>>,
    async_values: &mut Vec<Expression<'a>>,
    sync_memo_seen: &mut u32,
    async_memo_count: &mut u32,
) -> Vec<Arg<'a, 'static>> {
    let mut thunks: Vec<Arg<'a, 'static>> = Vec::with_capacity(sem.args.len());
    for (arg, arg_sem) in arguments.into_iter().zip(sem.args.iter()) {
        let arg_expr = arg.into_expression();
        match *arg_sem {
            RenderArgLowering::PropSource { sym } => {
                let name = ctx.symbol_name(sym);
                thunks.push(Arg::Expr(ctx.b.rid_expr(name)));
            }
            RenderArgLowering::MemoSync => {
                let idx = *sync_memo_seen;
                *sync_memo_seen += 1;
                let name = format!("${idx}");
                let thunk = ctx.b.thunk(arg_expr);
                let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                memo_stmts.push(ctx.b.let_init_stmt(&name, derived));
                let name_ref = ctx.b.alloc_str(&name);
                let get = ctx.b.call_expr("$.get", [Arg::Ident(name_ref)]);
                let read_thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(get)]);
                thunks.push(Arg::Expr(read_thunk));
            }
            RenderArgLowering::MemoAsync => {
                let idx = *async_memo_count;
                *async_memo_count += 1;
                async_values.push(ctx.b.clone_expr(&arg_expr));
                let param_name = format!("${}", sync_memo_count + idx);
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

#[allow(clippy::too_many_arguments)]
fn emit_async_wrapped<'a>(
    ctx: &mut Ctx<'a>,
    stmts: &mut Vec<Statement<'a>>,
    outer_anchor: Expression<'a>,
    blockers: &[u32],
    async_values: Vec<Expression<'a>>,
    mut memo_stmts: Vec<Statement<'a>>,
    final_stmt: Statement<'a>,
    anchor_name: &str,
    sync_memo_count: u32,
    async_memo_count: u32,
    is_standalone: bool,
) {
    memo_stmts.push(final_stmt);

    let blockers_expr = if blockers.is_empty() {
        ctx.b.void_zero_expr()
    } else {
        // Each script-level blocker index encodes as `$$promises[idx]`
        // at the call site — matching the existing `TemplateMemoState`
        // encoding used by every other async-wrapped template effect.
        ctx.b.array_expr(blockers.iter().map(|&idx| {
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

    let callback_params = if async_memo_count > 0 {
        let mut names = vec![anchor_name.to_string()];
        for i in 0..async_memo_count {
            names.push(format!("${}", sync_memo_count + i));
        }
        ctx.b.params(names.iter().map(|s| s.as_str()))
    } else {
        ctx.b.params([anchor_name])
    };
    let callback = ctx.b.arrow_block_expr(callback_params, memo_stmts);

    stmts.push(ctx.b.call_stmt(
        "$.async",
        [
            Arg::Expr(outer_anchor),
            Arg::Expr(blockers_expr),
            Arg::Expr(async_values_expr),
            Arg::Expr(callback),
        ],
    ));
    if is_standalone {
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
