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
    // 1. Collect all branches by walking the elseif chain
    let mut branches: Vec<Branch> = Vec::new();
    let mut final_alternate: Option<FragmentKey> = None;
    let mut current = block_id;

    loop {
        let block = ctx.if_block(current);
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
            let nested_id = ctx.lowered_fragment(&alternate_key).first_if_block_id().unwrap();
            current = nested_id;
            continue;
        } else {
            final_alternate = Some(alternate_key);
            break;
        }
    }

    let mut stmts = Vec::new();

    // 2. Generate all arrow functions at the same scope level.
    // Naming order: body (gen_fragment) BEFORE name (gen_ident) for each branch.
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

    // 2b. Pre-compute has_call memoization for each branch condition.
    // When a condition has a function call AND reactive references, wrap in $.derived.
    let mut derived_names: Vec<Option<String>> = Vec::new();
    for branch in branches.iter() {
        let needs_memo = ctx.analysis.needs_expr_memoization(branch.block_id);
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

    // 3. Build the if/else-if/else chain (bottom-up)
    let num_branches = branches.len();

    // Start from the final else (if any)
    let mut else_clause: Option<Statement<'a>> = alt_name
        .as_ref()
        .map(|an| ctx.b.call_stmt("$$render", [Arg::Ident(an), Arg::Num(-1.0)]));

    // Build from last branch to first
    for i in (0..num_branches).rev() {
        let test = if let Some(ref derived_name) = derived_names[i] {
            ctx.b.call_expr("$.get", [Arg::Ident(derived_name)])
        } else {
            get_node_expr(ctx, branches[i].block_id)
        };
        let render_args: Vec<Arg<'a, '_>> = if i == 0 {
            // First branch: no second argument
            vec![Arg::Ident(&branch_names[i])]
        } else {
            // Middle branches: pass index
            vec![Arg::Ident(&branch_names[i]), Arg::Num(i as f64)]
        };
        let then_stmt = ctx.b.call_stmt("$$render", render_args);
        let if_stmt = ctx.b.if_stmt(test, then_stmt, else_clause.take());
        else_clause = Some(if_stmt);
    }

    let render_body_stmt = else_clause.unwrap();
    let render_fn = ctx.b.arrow(ctx.b.params(["$$render"]), [render_body_stmt]);

    // 4. Single $.if() call, wrapped with add_svelte_meta in dev mode
    let span_start = ctx.if_block(block_id).span.start;
    let args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor), Arg::Arrow(render_fn)];
    let if_call = ctx.b.call_expr("$.if", args);
    stmts.push(super::add_svelte_meta(ctx, if_call, span_start, "if"));

    stmts
}
