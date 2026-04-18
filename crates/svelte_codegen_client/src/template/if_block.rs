//! IfBlock code generation — flattened elseif chains.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{ExprSite, FragmentKey};
use svelte_ast::NodeId;

use svelte_ast_builder::Arg;
use crate::context::Ctx;

use super::async_plan::AsyncEmissionPlan;
use super::expression::get_node_expr;
use super::gen_fragment;

/// Collected branch info before code generation.
struct Branch {
    block_id: NodeId,
    consequent_key: FragmentKey,
}

/// Generate statements for an `{#if ...}` block.
/// Flattens elseif chains into a single `$.if()` call.
pub(crate) fn gen_if_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
) -> Vec<Statement<'a>> {
    let async_plan = AsyncEmissionPlan::for_node(ctx, block_id);
    let has_await = async_plan.has_await();
    let needs_async = async_plan.needs_async();

    // 1. Collect all branches by walking the elseif chain
    let mut branches: Vec<Branch> = Vec::new();
    let mut final_alternate: Option<FragmentKey> = None;
    let mut current = block_id;

    loop {
        let block = ctx.query.if_block(current);
        let has_alternate = block.alternate.is_some();

        branches.push(Branch {
            block_id: current,
            consequent_key: FragmentKey::IfConsequent(current),
        });

        if !has_alternate {
            break;
        }

        let alternate_key = FragmentKey::IfAlternate(current);
        let alt_is_elseif = ctx.is_elseif_alt(current);

        if alt_is_elseif {
            let nested_id = ctx
                .query
                .lowered_fragment(&alternate_key)
                .first_if_block_id()
                .unwrap();

            // Don't flatten if the else-if has await or introduces blockers
            // not present in the parent — it needs its own $.async() wrapper.
            let nested_plan = AsyncEmissionPlan::for_node(ctx, nested_id);
            let can_flatten = !nested_plan.has_await() && {
                let parent_plan = AsyncEmissionPlan::for_node(ctx, current);
                parent_plan.blockers_contain_all(nested_plan.blockers())
            };
            if !can_flatten {
                final_alternate = Some(alternate_key);
                break;
            }

            current = nested_id;
            continue;
        }

        final_alternate = Some(alternate_key);
        break;
    }

    // Build the expression for the root condition async thunk (only when has_await).
    // Blocker-only async doesn't need the expression here — it uses the raw
    // condition in the $.if callback.
    let expression = if has_await {
        Some(get_node_expr(ctx, block_id))
    } else {
        None
    };

    let mut stmts = Vec::new();

    // 2. Generate consequent arrows interleaved with derived declarations per branch.
    let mut branch_names: Vec<String> = Vec::new();
    let mut derived_names: Vec<Option<String>> = Vec::new();

    for (i, branch) in branches.iter().enumerate() {
        let body = gen_fragment(ctx, branch.consequent_key);
        let name = ctx.gen_ident("consequent");
        let arrow = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);
        stmts.push(ctx.b.var_stmt(&name, arrow));
        branch_names.push(name);

        // Derived memoization (emitted right after its consequent, matching reference order)
        if i == 0 && has_await {
            // Root async condition: resolved via $.get($$condition) inside $.async callback
            derived_names.push(None);
        } else {
            let needs_memo = ctx
                .expr_deps(ExprSite::Node(branch.block_id))
                .is_some_and(|deps| deps.needs_memo);
            if needs_memo {
                let expr = get_node_expr(ctx, branch.block_id);
                // Always wrap in arrow for $.derived() — do not use thunk() which
                // strips no-arg calls (e.g. is_even() -> is_even), but $.derived
                // requires the call to be preserved inside the arrow.
                let thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(expr)]);
                let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                let name = ctx.gen_ident("d");
                stmts.push(ctx.b.var_stmt(&name, derived));
                derived_names.push(Some(name));
            } else {
                derived_names.push(None);
            }
        }
    }

    let alt_name = if let Some(alt_key) = final_alternate {
        let body = gen_fragment(ctx, alt_key);
        let name = ctx.gen_ident("alternate");
        let arrow = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);
        stmts.push(ctx.b.var_stmt(&name, arrow));
        Some(name)
    } else {
        None
    };

    // 3. Build the if/else-if/else chain (bottom-up)
    let num_branches = branches.len();

    let mut else_clause: Option<Statement<'a>> = alt_name.as_ref().map(|an| {
        ctx.b
            .call_stmt("$$render", [Arg::Ident(an), Arg::Num(-1.0)])
    });

    for i in (0..num_branches).rev() {
        let test = if i == 0 && has_await {
            ctx.b.call_expr("$.get", [Arg::Ident("$$condition")])
        } else if let Some(ref derived_name) = derived_names[i] {
            ctx.b.call_expr("$.get", [Arg::Ident(derived_name)])
        } else {
            get_node_expr(ctx, branches[i].block_id)
        };
        let render_args: Vec<Arg<'a, '_>> = if i == 0 {
            vec![Arg::Ident(&branch_names[i])]
        } else {
            vec![Arg::Ident(&branch_names[i]), Arg::Num(i as f64)]
        };
        let then_stmt = ctx.b.call_stmt("$$render", render_args);
        let if_stmt = ctx.b.if_stmt(test, then_stmt, else_clause.take());
        else_clause = Some(if_stmt);
    }

    let render_body_stmt = else_clause.unwrap();
    let render_fn = ctx.b.arrow(ctx.b.params(["$$render"]), [render_body_stmt]);

    // 4. $.if() call + optional $.async() wrapping
    let block = ctx.query.if_block(block_id);
    let span_start = block.span.start;
    let is_elseif = block.elseif;

    if needs_async {
        // Reuse the anchor's identifier name for the $.async callback parameter,
        // so the callback param shadows the outer variable (node, node_1, etc.).
        let node_name = match &anchor {
            Expression::Identifier(id) => id.name.to_string(),
            _ => ctx.gen_ident("node"),
        };
        let if_anchor = ctx.b.rid_expr(&node_name);
        let mut if_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(if_anchor), Arg::Arrow(render_fn)];
        if is_elseif {
            if_args.push(Arg::Bool(true));
        }
        let if_call = ctx.b.call_expr("$.if", if_args);
        stmts.push(super::add_svelte_meta(ctx, if_call, span_start, "if"));

        let async_thunk = expression.and_then(|expr| async_plan.async_thunk(ctx, expr));
        vec![async_plan.wrap_async_block(
            ctx,
            anchor,
            &node_name,
            "$$condition",
            async_thunk,
            stmts,
        )]
    } else {
        let mut if_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor), Arg::Arrow(render_fn)];
        if is_elseif {
            if_args.push(Arg::Bool(true));
        }
        let if_call = ctx.b.call_expr("$.if", if_args);
        stmts.push(super::add_svelte_meta(ctx, if_call, span_start, "if"));
        stmts
    }
}
