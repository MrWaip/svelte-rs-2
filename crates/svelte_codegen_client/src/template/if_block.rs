//! IfBlock code generation — flattened elseif chains.

use oxc_ast::ast::{Expression, Statement};
use svelte_span::Span;

use svelte_analyze::{FragmentItem, FragmentKey};
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::parse_expr;
use super::gen_fragment;

/// Collected branch info before code generation.
struct Branch {
    test_span: Span,
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
        let test_span = block.test_span;

        branches.push(Branch {
            test_span,
            consequent_key: FragmentKey::IfConsequent(current),
        });

        if !has_alternate {
            break;
        }

        let alternate_key = FragmentKey::IfAlternate(current);
        let alt_is_elseif = {
            let lf = ctx.analysis.lowered_fragments.get(&alternate_key);
            lf.is_some_and(|lf| {
                lf.items.len() == 1
                    && matches!(&lf.items[0], FragmentItem::IfBlock(id) if {
                        ctx.if_block(*id).elseif
                    })
            })
        };

        if alt_is_elseif {
            let nested_id = {
                let lf = ctx.analysis.lowered_fragments.get(&alternate_key).unwrap();
                match &lf.items[0] {
                    FragmentItem::IfBlock(id) => *id,
                    _ => unreachable!(),
                }
            };
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
    // For elseif branches (index > 0), consume gen_ident("root") for the skipped SingleBlock wrapper.
    let mut branch_names: Vec<String> = Vec::new();

    for (i, branch) in branches.iter().enumerate() {
        if i > 0 {
            // Consume "root" for the implicit SingleBlock wrapper that elseif bypasses
            ctx.gen_ident("root");
        }
        let body = gen_fragment(ctx, branch.consequent_key);
        let name = ctx.gen_ident("consequent");
        let arrow = ctx.b.arrow_expr(ctx.b.params(["$$anchor"]), body);
        stmts.push(ctx.b.var_stmt(&name, arrow));
        branch_names.push(name);
    }

    let alt_name = if let Some(alt_key) = final_alternate {
        let body = gen_fragment(ctx, alt_key);
        let name = ctx.gen_ident("alternate");
        let arrow = ctx.b.arrow_expr(ctx.b.params(["$$anchor"]), body);
        stmts.push(ctx.b.var_stmt(&name, arrow));
        Some(name)
    } else {
        None
    };

    // 3. Build the if/else-if/else chain (bottom-up)
    let num_branches = branches.len();

    // Start from the final else (if any)
    let mut else_clause: Option<Statement<'a>> = alt_name
        .as_ref()
        .map(|an| ctx.b.call_stmt("$$render", [Arg::Ident(an), Arg::Num(-1.0)]));

    // Build from last branch to first
    for i in (0..num_branches).rev() {
        let test = parse_expr(ctx, branches[i].test_span);
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

    // 4. Single $.if() call
    let args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor), Arg::Arrow(render_fn)];
    stmts.push(ctx.b.call_stmt("$.if", args));

    stmts
}
