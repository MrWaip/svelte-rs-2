//! IfBlock codegen driven by Block Semantics — `{#if}` / `{:else if}`
//! / `{:else}`.
//!
//! Root consumer for the `BlockSemantics::If` slice. Reads one
//! [`IfBlockSemantics`] answer — which already carries the flattened
//! branch chain, per-branch condition lowering shape, and root async
//! wrapper decision — resolves the spans into a plain-data
//! [`IfPlan`], and then dispatches to focused builders for each piece
//! of the emitted code.
//!
//! Consumer Model (see SEMANTIC_LAYER_ARCHITECTURE.md): the consumer
//! never walks the `alternate` chain itself, never reads
//! `ExpressionInfo`, and never calls `AsyncEmissionPlan::for_node` —
//! those decisions live on the payload.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{
    FragmentKey, IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch, IfConditionKind,
};
use svelte_ast::NodeId;

use crate::context::Ctx;
use svelte_ast_builder::Arg;

use super::async_plan::wrap_async_block;
use super::expression::get_node_expr;
use super::gen_fragment;

/// Generate statements for an `{#if ...}` block.
///
/// `sem` is already resolved: the root consumer match happened at the
/// dispatch site (see `template/mod.rs` / `template/traverse.rs`). This
/// function never inspects the `alternate` chain itself — flattening is
/// pre-baked on `sem.branches`.
pub(crate) fn gen_if_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    sem: &IfBlockSemantics,
    anchor: Expression<'a>,
) -> Vec<Statement<'a>> {
    let plan = IfPlan::build(ctx, block_id, sem);

    let mut stmts: Vec<Statement<'a>> = Vec::new();
    let branch_names = build_consequent_and_derived(ctx, sem, &plan, &mut stmts);
    let alt_name = build_alternate_arrow(ctx, sem, &mut stmts);
    let if_chain = build_if_chain(ctx, sem, &plan, &branch_names, alt_name.as_deref());
    emit_if_call(ctx, block_id, sem, &plan, if_chain, anchor, stmts)
}

// ---------------------------------------------------------------------------
// Plan
// ---------------------------------------------------------------------------

/// Decisions resolved up front from `sem`. Builders operate on
/// `&IfPlan` so they never re-interpret semantic variants.
struct IfPlan {
    /// `true` when the root test expression literally contains `await`.
    /// The root branch reads through `$.get($$condition)` and the
    /// expression becomes the async wrapper's thunk.
    root_has_await: bool,
    /// `true` when `sem.async_kind` is `Async`. Both await and
    /// blocker-driven async fall under this flag.
    needs_async: bool,
    /// Sorted, de-duplicated script-level blocker indices for the async
    /// wrapper. Empty when `needs_async` is `false`.
    blockers: Vec<u32>,
    /// Forwarded to the third argument of `$.if(...)` so the runtime
    /// marks the branch with `EFFECT_TRANSPARENT` for transitions.
    is_elseif_root: bool,
    /// `span.start` of the root IfBlock — used by `$.add_svelte_meta`.
    span_start: u32,
}

impl IfPlan {
    fn build(ctx: &Ctx<'_>, block_id: NodeId, sem: &IfBlockSemantics) -> Self {
        let block = ctx.query.if_block(block_id);
        let (needs_async, root_has_await, blockers) = match &sem.async_kind {
            IfAsyncKind::Sync => (false, false, Vec::new()),
            IfAsyncKind::Async {
                root_has_await,
                blockers,
            } => (true, *root_has_await, blockers.to_vec()),
        };
        Self {
            root_has_await,
            needs_async,
            blockers,
            is_elseif_root: sem.is_elseif_root,
            span_start: block.span.start,
        }
    }
}

// ---------------------------------------------------------------------------
// Consequent arrows + optional `$.derived` memos
// ---------------------------------------------------------------------------

/// One `const consequent_i = ($$anchor) => { ...body... };` per branch,
/// interleaved with `const d_i = $.derived(() => test);` for `Memo`
/// branches (matches the reference compiler's emission order).
///
/// Returns one entry per branch holding the consequent identifier and,
/// for `Memo` branches, the derived identifier — `None` when the
/// branch is `Raw` or `AsyncParam`.
fn build_consequent_and_derived<'a>(
    ctx: &mut Ctx<'a>,
    sem: &IfBlockSemantics,
    plan: &IfPlan,
    stmts: &mut Vec<Statement<'a>>,
) -> Vec<BranchNames> {
    let mut out: Vec<BranchNames> = Vec::with_capacity(sem.branches.len());
    for (i, branch) in sem.branches.iter().enumerate() {
        let body = gen_fragment(ctx, FragmentKey::IfConsequent(branch.block_id));
        let consequent_name = ctx.gen_ident("consequent");
        let arrow = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);
        stmts.push(ctx.b.var_stmt(&consequent_name, arrow));

        let derived_name = match branch.condition {
            IfConditionKind::Memo => Some(emit_condition_derived(ctx, branch, stmts)),
            IfConditionKind::Raw => None,
            IfConditionKind::AsyncParam => {
                // Root async condition is read via `$.get($$condition)` —
                // no per-branch derived, and no expression read here
                // (the expression becomes the wrapper's async_values
                // thunk, consumed by `emit_if_call`).
                debug_assert!(i == 0 && plan.root_has_await);
                None
            }
        };
        out.push(BranchNames {
            consequent: consequent_name,
            derived: derived_name,
        });
    }
    out
}

struct BranchNames {
    consequent: String,
    derived: Option<String>,
}

fn emit_condition_derived<'a>(
    ctx: &mut Ctx<'a>,
    branch: &IfBranch,
    stmts: &mut Vec<Statement<'a>>,
) -> String {
    let expr = get_node_expr(ctx, branch.block_id);
    // `$.derived(() => expr)` — must be an arrow preserving the call
    // inside, not `thunk()` (which strips no-arg calls like `foo()` back
    // to `foo`).
    let thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(expr)]);
    let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
    let name = ctx.gen_ident("d");
    stmts.push(ctx.b.var_stmt(&name, derived));
    name
}

// ---------------------------------------------------------------------------
// Final `{:else}` alternate arrow
// ---------------------------------------------------------------------------

/// Emit `const alternate = ($$anchor) => { ...else body... };` when
/// `sem.final_alternate` is a fragment. Returns the identifier name for
/// the chain builder to call `$$render(alternate, -1)` with.
fn build_alternate_arrow<'a>(
    ctx: &mut Ctx<'a>,
    sem: &IfBlockSemantics,
    stmts: &mut Vec<Statement<'a>>,
) -> Option<String> {
    let IfAlternate::Fragment {
        last_branch_block_id,
    } = sem.final_alternate
    else {
        return None;
    };
    let body = gen_fragment(ctx, FragmentKey::IfAlternate(last_branch_block_id));
    let name = ctx.gen_ident("alternate");
    let arrow = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);
    stmts.push(ctx.b.var_stmt(&name, arrow));
    Some(name)
}

// ---------------------------------------------------------------------------
// Nested `if / else if / else` chain
// ---------------------------------------------------------------------------

/// Build the `$$render` callback body: a bottom-up nested
/// `if / else if / else` chain where each branch test reads via
/// [`IfConditionKind`].
fn build_if_chain<'a>(
    ctx: &mut Ctx<'a>,
    sem: &IfBlockSemantics,
    plan: &IfPlan,
    branch_names: &[BranchNames],
    alt_name: Option<&str>,
) -> Statement<'a> {
    let mut else_clause: Option<Statement<'a>> = alt_name.map(|name| {
        ctx.b
            .call_stmt("$$render", [Arg::Ident(name), Arg::Num(-1.0)])
    });

    for (i, branch) in sem.branches.iter().enumerate().rev() {
        let test = branch_condition_expr(ctx, branch, plan, branch_names[i].derived.as_deref());
        let render_args: Vec<Arg<'a, '_>> = if i == 0 {
            vec![Arg::Ident(&branch_names[i].consequent)]
        } else {
            vec![Arg::Ident(&branch_names[i].consequent), Arg::Num(i as f64)]
        };
        let then_stmt = ctx.b.call_stmt("$$render", render_args);
        let if_stmt = ctx.b.if_stmt(test, then_stmt, else_clause.take());
        else_clause = Some(if_stmt);
    }

    else_clause.expect("IfBlockSemantics guarantees at least one branch")
}

/// Lower one branch's test expression to the shape dictated by its
/// [`IfConditionKind`]. For the root branch under async, the condition
/// reads through `$$condition` — the raw expression has already been
/// consumed as the wrapper's async thunk (see `emit_if_call`).
fn branch_condition_expr<'a>(
    ctx: &mut Ctx<'a>,
    branch: &IfBranch,
    plan: &IfPlan,
    derived_name: Option<&str>,
) -> Expression<'a> {
    match branch.condition {
        IfConditionKind::AsyncParam => {
            debug_assert!(plan.root_has_await);
            ctx.b.call_expr("$.get", [Arg::Ident("$$condition")])
        }
        IfConditionKind::Memo => {
            let name = derived_name
                .expect("Memo branch must have a derived ident from build_consequent_and_derived");
            ctx.b.call_expr("$.get", [Arg::Ident(name)])
        }
        IfConditionKind::Raw => get_node_expr(ctx, branch.block_id),
    }
}

// ---------------------------------------------------------------------------
// Emit
// ---------------------------------------------------------------------------

/// Compose `$.if(...)` — wrapping in `$.async(...)` when the root is
/// async. `decls` already contains the consequent arrows + any derived
/// bindings + the optional alternate arrow, in source order. Returns
/// the full statement list the caller wraps in a block or inlines.
fn emit_if_call<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    _sem: &IfBlockSemantics,
    plan: &IfPlan,
    render_body_stmt: Statement<'a>,
    anchor: Expression<'a>,
    mut decls: Vec<Statement<'a>>,
) -> Vec<Statement<'a>> {
    let render_fn = ctx.b.arrow(ctx.b.params(["$$render"]), [render_body_stmt]);

    if plan.needs_async {
        // Reuse the anchor's ident name for the `$.async` callback
        // parameter so the callback param shadows the outer variable
        // (`node`, `node_1`, …). Fall back to a generated `node` when
        // the anchor is a non-identifier expression.
        let node_name = match &anchor {
            Expression::Identifier(id) => id.name.to_string(),
            _ => ctx.gen_ident("node"),
        };
        let if_anchor = ctx.b.rid_expr(&node_name);
        let mut if_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(if_anchor), Arg::Arrow(render_fn)];
        if plan.is_elseif_root {
            if_args.push(Arg::Bool(true));
        }
        let if_call = ctx.b.call_expr("$.if", if_args);
        decls.push(super::add_svelte_meta(ctx, if_call, plan.span_start, "if"));

        let async_thunk = if plan.root_has_await {
            let expr = get_node_expr(ctx, block_id);
            Some(ctx.b.async_thunk(expr))
        } else {
            None
        };
        vec![wrap_async_block(
            ctx,
            plan.root_has_await,
            &plan.blockers,
            anchor,
            &node_name,
            "$$condition",
            async_thunk,
            decls,
        )]
    } else {
        let mut if_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor), Arg::Arrow(render_fn)];
        if plan.is_elseif_root {
            if_args.push(Arg::Bool(true));
        }
        let if_call = ctx.b.call_expr("$.if", if_args);
        decls.push(super::add_svelte_meta(ctx, if_call, plan.span_start, "if"));
        decls
    }
}
