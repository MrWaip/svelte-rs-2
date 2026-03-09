//! IfBlock code generation.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{FragmentItem, FragmentKey};
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::expression::parse_expr;
use super::gen_fragment;

/// Generate statements for an `{#if ...}` block.
/// Returns statements that should be wrapped in a block `{ ... }`.
pub(crate) fn gen_if_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    elseif_arg: Option<&str>,
) -> Vec<Statement<'a>> {
    let block = ctx.if_block(block_id);
    let has_alternate = block.alternate.is_some();
    let test_span = block.test_span;

    let consequent_key = FragmentKey::IfConsequent(block_id);
    let alternate_key = FragmentKey::IfAlternate(block_id);

    let mut stmts = Vec::new();

    // Detect elseif before generating alternate
    let alt_is_elseif = has_alternate && {
        let lf = ctx.analysis.lowered_fragments.get(&alternate_key);
        lf.is_some_and(|lf| {
            lf.items.len() == 1
                && matches!(&lf.items[0], FragmentItem::IfBlock(id) if {
                    ctx.if_block(*id).elseif
                })
        })
    };

    // Consequent: body first, then name (bottom-up naming)
    let cons_body = gen_fragment(ctx, consequent_key);
    let cons_name = ctx.gen_ident("consequent");
    let cons_fn = ctx.b.arrow_expr(ctx.b.params(["$$anchor"]), cons_body);
    stmts.push(ctx.b.var_stmt(&cons_name, cons_fn));

    // Alternate fn (if any)
    let alt_name: Option<String> = if has_alternate {
        if alt_is_elseif {
            // Name BEFORE body — outer alternate name must precede inner names
            let alt_name = ctx.gen_ident("alternate");
            let nested_id = {
                let lf = ctx.analysis.lowered_fragments.get(&alternate_key).unwrap();
                match &lf.items[0] {
                    FragmentItem::IfBlock(id) => *id,
                    _ => unreachable!(),
                }
            };
            // Consume "root" for the implicit SingleBlock wrapper that elseif bypasses
            ctx.gen_ident("root");
            let nested_stmts =
                gen_if_block(ctx, nested_id, ctx.b.rid_expr("$$anchor"), Some("$$elseif"));
            let alt_body = vec![ctx.b.block_stmt(nested_stmts)];
            let alt_fn = ctx
                .b
                .arrow_expr(ctx.b.params(["$$anchor", "$$elseif"]), alt_body);
            stmts.push(ctx.b.var_stmt(&alt_name, alt_fn));
            Some(alt_name)
        } else {
            // Body first, then name
            let alt_body = gen_fragment(ctx, alternate_key);
            let alt_name = ctx.gen_ident("alternate");
            let alt_fn = ctx.b.arrow_expr(ctx.b.params(["$$anchor"]), alt_body);
            stmts.push(ctx.b.var_stmt(&alt_name, alt_fn));
            Some(alt_name)
        }
    } else {
        None
    };

    // Render fn: ($$render) => { if (test) $$render(cons); else $$render(alt, false); }
    let test = parse_expr(ctx, test_span);
    let then_stmt = ctx.b.call_stmt("$$render", [Arg::Ident(&cons_name)]);
    let else_stmt = alt_name
        .as_ref()
        .map(|an| ctx.b.call_stmt("$$render", [Arg::Ident(an), Arg::Bool(false)]));
    let if_stmt = ctx.b.if_stmt(test, then_stmt, else_stmt);
    let render_fn = ctx.b.arrow(ctx.b.params(["$$render"]), [if_stmt]);

    // $.if(anchor, render_fn[, $$elseif])
    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor), Arg::Arrow(render_fn)];
    if let Some(earg) = elseif_arg {
        args.push(Arg::Ident(earg));
    }
    stmts.push(ctx.b.call_stmt("$.if", args));

    stmts
}
