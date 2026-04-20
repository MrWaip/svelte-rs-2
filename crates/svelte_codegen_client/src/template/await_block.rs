//! AwaitBlock code generation.
//!
//! `gen_await_block` is a thin orchestrator: it reads the pre-computed
//! [`AwaitBlockSemantics`] and dispatches to focused builders for each
//! piece of the `$.await(...)` call (expression, pending / then / catch
//! callbacks, trailing-null trimming).
//!
//! The reference compiler additionally wraps `$.await(...)` in `$.async`
//! when the expression references blocker symbols; `AwaitBlockSemantics`
//! already carries the [`AwaitWrapper`] decision but this consumer
//! deliberately mirrors today's Rust behavior (no wrap), matching
//! existing snapshots. The wrapper field stays on the payload so the
//! future fix lands as a consumer-side change only.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::scope::SymbolId;
use svelte_analyze::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, FragmentKey,
};
use svelte_ast::NodeId;

use crate::context::Ctx;
use svelte_ast_builder::Arg;

use super::expression::{build_node_thunk, get_node_expr};
use super::gen_fragment;

/// Generate statements for an `{#await ...}` block.
///
/// `sem` is already resolved: the root consumer match happened before
/// this call (see SEMANTIC_LAYER_ARCHITECTURE.md — Root Consumer Migration).
pub(crate) fn gen_await_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    sem: &AwaitBlockSemantics,
    anchor: Expression<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    let plan = AwaitPlan::from_sem(sem);

    let expression = build_expression(ctx, block_id, &plan);
    let then_fn = build_then_fn(ctx, block_id, &sem.then, &plan);
    let catch_fn = build_catch_fn(ctx, block_id, &sem.catch, &plan);
    let pending_fn = build_pending_fn(ctx, block_id, &plan);

    emit_await_call(
        ctx, block_id, &plan, anchor, expression, pending_fn, then_fn, catch_fn, body,
    );
}

// ---------------------------------------------------------------------------
// Plan
// ---------------------------------------------------------------------------

/// Decisions resolved up front from `sem`. Keeps builders free of
/// semantic re-interpretation.
struct AwaitPlan {
    has_pending: bool,
    has_then: bool,
    has_catch: bool,
    /// Expression thunk must be `async () => await <expr>` vs `() => <expr>`.
    expression_has_await: bool,
}

impl AwaitPlan {
    fn from_sem(sem: &AwaitBlockSemantics) -> Self {
        Self {
            has_pending: matches!(sem.pending, AwaitBranch::Present { .. }),
            has_then: matches!(sem.then, AwaitBranch::Present { .. }),
            has_catch: matches!(sem.catch, AwaitBranch::Present { .. }),
            expression_has_await: sem.expression_has_await,
        }
    }
}

// ---------------------------------------------------------------------------
// Expression
// ---------------------------------------------------------------------------

/// `() => <expr>` — or `async () => <expr>` when the expression literally
/// contains an `await`. Visiting the expression first preserves the
/// reference compiler's scope-ordering (matters for template variable
/// numbering).
fn build_expression<'a>(ctx: &mut Ctx<'a>, block_id: NodeId, plan: &AwaitPlan) -> Expression<'a> {
    if plan.expression_has_await {
        let expr = get_node_expr(ctx, block_id);
        ctx.b.async_thunk(expr)
    } else {
        build_node_thunk(ctx, block_id)
    }
}

// ---------------------------------------------------------------------------
// Branch callbacks
// ---------------------------------------------------------------------------

fn build_then_fn<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    branch: &AwaitBranch,
    plan: &AwaitPlan,
) -> Expression<'a> {
    if plan.has_then {
        build_branch_callback(ctx, FragmentKey::AwaitThen(block_id), binding_of(branch))
    } else if plan.has_catch {
        // Trim-aware placeholder: emitted only if a later arg forces it.
        ctx.b.void_zero_expr()
    } else {
        ctx.b.null_expr()
    }
}

fn build_catch_fn<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    branch: &AwaitBranch,
    plan: &AwaitPlan,
) -> Expression<'a> {
    if plan.has_catch {
        build_branch_callback(ctx, FragmentKey::AwaitCatch(block_id), binding_of(branch))
    } else {
        ctx.b.null_expr()
    }
}

fn build_pending_fn<'a>(ctx: &mut Ctx<'a>, block_id: NodeId, plan: &AwaitPlan) -> Expression<'a> {
    if !plan.has_pending {
        return ctx.b.null_expr();
    }
    let frag_body = gen_fragment(ctx, FragmentKey::AwaitPending(block_id));
    ctx.b
        .arrow_block_expr(ctx.b.params(["$$anchor"]), frag_body)
}

fn binding_of(branch: &AwaitBranch) -> &AwaitBinding {
    match branch {
        AwaitBranch::Present { binding } => binding,
        AwaitBranch::Absent => &AwaitBinding::None,
    }
}

/// Branch callback shape matching the reference compiler:
/// - no binding: `($$anchor) => { ...fragment... }`
/// - identifier: `($$anchor, name) => { ...fragment... }`
/// - destructured: `($$anchor, $$source) => { <derived declarations>; ...fragment... }`
fn build_branch_callback<'a>(
    ctx: &mut Ctx<'a>,
    fragment_key: FragmentKey,
    binding: &AwaitBinding,
) -> Expression<'a> {
    let frag_body = gen_fragment(ctx, fragment_key);
    match binding {
        AwaitBinding::None => ctx
            .b
            .arrow_block_expr(ctx.b.params(["$$anchor"]), frag_body),
        AwaitBinding::Identifier(sym) => {
            let name = ctx.symbol_name(*sym).to_string();
            ctx.b
                .arrow_block_expr(ctx.b.params(["$$anchor", &name]), frag_body)
        }
        AwaitBinding::Pattern { kind, leaves, .. } => {
            build_destructured_callback(ctx, *kind, leaves, frag_body)
        }
    }
}

/// Destructured callback: see reference `create_derived_block_argument`.
/// One `$$value = $.derived(() => { var <pattern> = $.get($$source); return { a, b }; })`
/// + one per-leaf `name = $.derived(() => $.get($$value).name)`.
fn build_destructured_callback<'a>(
    ctx: &mut Ctx<'a>,
    kind: AwaitDestructureKind,
    leaves: &[SymbolId],
    frag_body: Vec<Statement<'a>>,
) -> Expression<'a> {
    let names: Vec<String> = leaves
        .iter()
        .map(|&sym| ctx.symbol_name(sym).to_string())
        .collect();

    let mut decls: Vec<Statement<'a>> = Vec::new();

    let get_source = ctx.b.call_expr("$.get", [Arg::Ident("$$source")]);
    let destruct_stmt = match kind {
        AwaitDestructureKind::Array => ctx.b.var_array_destruct_stmt(&names, get_source),
        AwaitDestructureKind::Object => ctx.b.var_object_destruct_stmt(&names, get_source),
    };
    let return_obj = ctx.b.shorthand_object_expr(&names);
    let return_stmt = ctx.b.return_stmt(return_obj);
    let derived_fn = ctx.b.thunk_block(vec![destruct_stmt, return_stmt]);
    let derived_call = ctx.b.call_expr("$.derived", [Arg::Expr(derived_fn)]);
    decls.push(ctx.b.var_stmt("$$value", derived_call));

    for name in &names {
        let get_value = ctx.b.call_expr("$.get", [Arg::Ident("$$value")]);
        let member = ctx.b.static_member_expr(get_value, name);
        let getter_fn = ctx
            .b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(member)]);
        let per_field_derived = ctx.b.call_expr("$.derived", [Arg::Expr(getter_fn)]);
        decls.push(ctx.b.var_stmt(name, per_field_derived));
    }

    decls.extend(frag_body);
    ctx.b
        .arrow_block_expr(ctx.b.params(["$$anchor", "$$source"]), decls)
}

// ---------------------------------------------------------------------------
// Call assembly
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn emit_await_call<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    plan: &AwaitPlan,
    anchor: Expression<'a>,
    expression: Expression<'a>,
    pending_fn: Expression<'a>,
    then_fn: Expression<'a>,
    catch_fn: Expression<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    let span_start = ctx.await_block(block_id).span.start;

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Expr(anchor),
        Arg::Expr(expression),
        Arg::Expr(pending_fn),
    ];
    // Trim trailing nulls: catch after then, then if catch absent.
    if plan.has_then || plan.has_catch {
        args.push(Arg::Expr(then_fn));
    }
    if plan.has_catch {
        args.push(Arg::Expr(catch_fn));
    }

    let call = ctx.b.call_expr("$.await", args);
    body.push(super::add_svelte_meta(ctx, call, span_start, "await"));
}
