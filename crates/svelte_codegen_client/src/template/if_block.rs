//! IfBlock code generation — flattened elseif chains.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

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
    let has_await = ctx.expr_has_await(block_id);
    let needs_async = has_await || ctx.expr_has_blockers(block_id);

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
        let alt_is_elseif = ctx.query.raw().is_elseif_alt(current);

        if alt_is_elseif {
            let nested_id = ctx
                .query
                .lowered_fragment(&alternate_key)
                .first_if_block_id()
                .unwrap();
            current = nested_id;
            continue;
        } else {
            final_alternate = Some(alternate_key);
            break;
        }
    }

    // Build the expression for the root condition (needed before stmts consume it)
    let expression = if needs_async {
        Some(get_node_expr(ctx, block_id))
    } else {
        None
    };

    let mut stmts = Vec::new();

    // 2. Generate all arrow functions at the same scope level.
    let mut branch_names: Vec<String> = Vec::new();

    for branch in branches.iter() {
        let body = gen_fragment(ctx, branch.consequent_key);
        let name = ctx.gen_ident("consequent");
        let arrow = ctx.b.arrow_block_expr(ctx.b.params(["$$anchor"]), body);
        stmts.push(ctx.b.var_stmt(&name, arrow));
        branch_names.push(name);
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

    // 2b. Pre-compute memoization for each branch condition.
    let mut derived_names: Vec<Option<String>> = Vec::new();
    for (i, branch) in branches.iter().enumerate() {
        if i == 0 && has_await {
            // Root async condition: resolved via $.get($$condition) inside $.async callback
            derived_names.push(None);
        } else {
            let needs_memo = ctx.query.raw().needs_expr_memoization(branch.block_id);
            if needs_memo {
                let expr = get_node_expr(ctx, branch.block_id);
                let thunk = ctx.b.thunk(expr);
                let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                let name = ctx.gen_ident("d");
                stmts.push(ctx.b.var_stmt(&name, derived));
                derived_names.push(Some(name));
            } else {
                derived_names.push(None);
            }
        }
    }

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
    let span_start = ctx.query.if_block(block_id).span.start;

    if needs_async {
        // Inside $.async callback: use "node" param as anchor
        let if_anchor = ctx.b.rid_expr("node");
        let if_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(if_anchor), Arg::Arrow(render_fn)];
        let if_call = ctx.b.call_expr("$.if", if_args);
        stmts.push(super::add_svelte_meta(ctx, if_call, span_start, "if"));

        let async_thunk = if has_await {
            Some(ctx.b.async_thunk(expression.unwrap()))
        } else {
            None
        };
        vec![ctx.gen_async_block(
            block_id,
            anchor,
            has_await,
            async_thunk,
            "$$condition",
            stmts,
        )]
    } else {
        let if_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor), Arg::Arrow(render_fn)];
        let if_call = ctx.b.call_expr("$.if", if_args);
        stmts.push(super::add_svelte_meta(ctx, if_call, span_start, "if"));
        stmts
    }
}
