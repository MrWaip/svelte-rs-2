//! KeyBlock codegen driven by Block Semantics — `{#key expr}...{/key}`.
//!
//! Root consumer for the `BlockSemantics::Key` slice. Reads one
//! [`KeyBlockSemantics`] answer — which already carries the root async
//! wrapper decision — resolves the span into a plain-data [`KeyPlan`],
//! and then dispatches to focused builders for the body arrow and the
//! final `$.key(...)` call (optionally wrapped in `$.async(...)`).
//!
//! Consumer Model (see SEMANTIC_LAYER_ARCHITECTURE.md): the consumer
//! never calls `AsyncEmissionPlan::for_node` and never reads
//! `ExpressionInfo` — the async decision lives on the payload.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{FragmentKey, KeyAsyncKind, KeyBlockSemantics};
use svelte_ast::NodeId;

use crate::context::Ctx;
use svelte_ast_builder::Arg;

use super::async_plan::wrap_async_block;
use super::expression::{build_node_thunk, get_node_expr};
use super::gen_fragment;

/// Generate statements for a `{#key ...}` block.
///
/// `sem` is already resolved: the root consumer match happened at the
/// dispatch site (see `template/mod.rs` / `template/traverse.rs`).
pub(crate) fn gen_key_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    sem: &KeyBlockSemantics,
    anchor: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let plan = KeyPlan::build(ctx, block_id, sem);

    let body = gen_fragment(ctx, FragmentKey::KeyBlockBody(block_id));
    let body_fn = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);

    if plan.needs_async {
        emit_async_wrapped(ctx, block_id, &plan, anchor, body_fn, stmts);
    } else {
        stmts.push(build_sync_key_call(ctx, block_id, &plan, anchor, body_fn));
    }
}

// ---------------------------------------------------------------------------
// Plan
// ---------------------------------------------------------------------------

/// Decisions resolved up front from `sem`. Builders operate on
/// `&KeyPlan` so they never re-interpret semantic variants.
struct KeyPlan {
    /// `true` when `sem.async_kind` is `Async`. Both await and
    /// blocker-driven async fall under this flag.
    needs_async: bool,
    /// `true` when the key expression literally contains `await`. Adds
    /// the `$$key` callback parameter and feeds the async-values thunk.
    has_await: bool,
    /// Sorted, de-duplicated script-level blocker indices for the async
    /// wrapper. Empty when `needs_async` is `false`.
    blockers: Vec<u32>,
    /// `span.start` of the root KeyBlock — used by `$.add_svelte_meta`.
    span_start: u32,
}

impl KeyPlan {
    fn build(ctx: &Ctx<'_>, block_id: NodeId, sem: &KeyBlockSemantics) -> Self {
        let block = ctx.query.key_block(block_id);
        let (needs_async, has_await, blockers) = match &sem.async_kind {
            KeyAsyncKind::Sync => (false, false, Vec::new()),
            KeyAsyncKind::Async {
                has_await,
                blockers,
            } => (true, *has_await, blockers.to_vec()),
        };
        Self {
            needs_async,
            has_await,
            blockers,
            span_start: block.span.start,
        }
    }
}

// ---------------------------------------------------------------------------
// Sync path
// ---------------------------------------------------------------------------

/// Direct `$.key(anchor, () => expr, body_fn)` emission, wrapped in
/// `$.add_svelte_meta` for dev.
fn build_sync_key_call<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    plan: &KeyPlan,
    anchor: Expression<'a>,
    body_fn: Expression<'a>,
) -> Statement<'a> {
    let key_thunk = build_node_thunk(ctx, block_id);
    let key_call = ctx.b.call_expr(
        "$.key",
        [Arg::Expr(anchor), Arg::Expr(key_thunk), Arg::Expr(body_fn)],
    );
    super::add_svelte_meta(ctx, key_call, plan.span_start, "key")
}

// ---------------------------------------------------------------------------
// Async path
// ---------------------------------------------------------------------------

/// Emit `$.async(anchor, [blockers], async_values, (node, $$key?) => { $.key(node, () => $.get($$key), body_fn) })`.
/// Inside the callback the key is always read through `$$key` — the
/// raw expression becomes the async-values thunk only when
/// `has_await` is true.
fn emit_async_wrapped<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    plan: &KeyPlan,
    anchor: Expression<'a>,
    body_fn: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    // Inside the async callback: read the (awaited) key via $$key.
    let key_thunk = ctx.b.thunk(ctx.b.call_expr("$.get", [Arg::Ident("$$key")]));
    let key_call = ctx.b.call_expr(
        "$.key",
        [Arg::Ident("node"), Arg::Expr(key_thunk), Arg::Expr(body_fn)],
    );
    let key_stmt = super::add_svelte_meta(ctx, key_call, plan.span_start, "key");

    let async_thunk = if plan.has_await {
        let expr = get_node_expr(ctx, block_id);
        Some(ctx.b.async_thunk(expr))
    } else {
        None
    };

    stmts.push(wrap_async_block(
        ctx,
        plan.has_await,
        &plan.blockers,
        anchor,
        "node",
        "$$key",
        async_thunk,
        vec![key_stmt],
    ));
}
