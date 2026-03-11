//! DOM traversal for Mixed fragments — assigns variables, builds sibling calls.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentItem;

use crate::builder::{Arg, AssignLeft, AssignRight};
use crate::context::Ctx;

use super::each_block::gen_each_block;
use super::element::{item_needs_var, process_element};
use super::expression::parts_are_dynamic;
use super::if_block::gen_if_block;
use super::render_tag::gen_render_tag;

/// Traverse lowered items, assign DOM variables, generate init/update statements.
///
/// Returns the trailing `sibling_offset` (unprocessed static siblings after the last var).
pub(crate) fn traverse_items<'a>(
    ctx: &mut Ctx<'a>,
    items: &[FragmentItem],
    first_child_expr: Expression<'a>,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    hoisted: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) -> usize {
    let mut prev_expr: Option<Expression<'a>> = Some(first_child_expr);
    let mut prev_ident: Option<String> = None;
    let mut sibling_offset: usize = 0;

    for item in items {
        let needs_var = item_needs_var(item, ctx);
        // is_text matches Svelte's `sequence.length === 1`: a standalone {expression}
        // with no surrounding text nodes in the same sequence
        let is_text = matches!(item, FragmentItem::TextConcat { parts } if parts.len() == 1);

        if needs_var {
            // Build the expression to get this node's DOM reference
            let node_expr: Expression<'a> = if let Some(ident) = &prev_ident {
                build_sibling_call(ctx, SiblingPrev::Ident(ident), sibling_offset, is_text)
            } else {
                let fc = prev_expr.take().unwrap();
                if sibling_offset == 0 {
                    fc
                } else {
                    build_sibling_call(ctx, SiblingPrev::Expr(fc), sibling_offset, is_text)
                }
            };

            match item {
                FragmentItem::TextConcat { parts } => {
                    let name = ctx.gen_ident("text");
                    init.push(ctx.b.var_stmt(&name, node_expr));
                    prev_ident = Some(name.clone());
                    sibling_offset = 1;

                    let is_dyn = parts_are_dynamic(parts, ctx);
                    if is_dyn {
                        let expr =
                            super::expression::build_concat_from_parts(ctx, parts);
                        update.push(ctx.b.call_stmt(
                            "$.set_text",
                            [Arg::Ident(&name), Arg::Expr(expr)],
                        ));
                    } else {
                        let expr =
                            super::expression::build_concat_from_parts(ctx, parts);
                        init.push(ctx.b.assign_stmt(
                            AssignLeft::StaticMember(
                                ctx.b.static_member(ctx.b.rid_expr(&name), "nodeValue"),
                            ),
                            AssignRight::Expr(expr),
                        ));
                    }
                }

                FragmentItem::Element(el_id) => {
                    let el_name_str = ctx.element(*el_id).name.clone();
                    let el_name = ctx.gen_ident(&el_name_str);
                    init.push(ctx.b.var_stmt(&el_name, node_expr));
                    prev_ident = Some(el_name.clone());
                    sibling_offset = 1;
                    process_element(ctx, *el_id, &el_name, init, update, hoisted, after_update);
                }

                FragmentItem::IfBlock(id) => {
                    let node_name = ctx.gen_ident("node");
                    init.push(ctx.b.var_stmt(&node_name, node_expr));
                    prev_ident = Some(node_name.clone());
                    sibling_offset = 1;

                    let stmts = gen_if_block(ctx, *id, ctx.b.rid_expr(&node_name));
                    init.push(ctx.b.block_stmt(stmts));
                }

                FragmentItem::EachBlock(id) => {
                    let node_name = ctx.gen_ident("node");
                    init.push(ctx.b.var_stmt(&node_name, node_expr));
                    prev_ident = Some(node_name.clone());
                    sibling_offset = 1;
                    gen_each_block(ctx, *id, ctx.b.rid_expr(&node_name), init);
                }

                FragmentItem::RenderTag(id) => {
                    let node_name = ctx.gen_ident("node");
                    init.push(ctx.b.var_stmt(&node_name, node_expr));
                    prev_ident = Some(node_name.clone());
                    sibling_offset = 1;
                    gen_render_tag(ctx, *id, ctx.b.rid_expr(&node_name), init);
                }
            }
        } else {
            sibling_offset += 1;
        }
    }

    sibling_offset
}

// ---------------------------------------------------------------------------
// Sibling call builder
// ---------------------------------------------------------------------------

enum SiblingPrev<'a, 'b> {
    Ident(&'b str),
    Expr(Expression<'a>),
}

/// Build `$.sibling(prev, offset?, is_text?)` call.
fn build_sibling_call<'a>(
    ctx: &Ctx<'a>,
    prev: SiblingPrev<'a, '_>,
    offset: usize,
    is_text: bool,
) -> Expression<'a> {
    let prev_arg = match prev {
        SiblingPrev::Ident(name) => Arg::Ident(name),
        SiblingPrev::Expr(expr) => Arg::Expr(expr),
    };

    let mut args: Vec<Arg<'a, '_>> = vec![prev_arg];

    if is_text {
        // Text nodes always need explicit offset to reach the is_text flag
        args.push(Arg::Num(offset as f64));
        args.push(Arg::Bool(true));
    } else if offset != 1 {
        args.push(Arg::Num(offset as f64));
    }

    ctx.b.call_expr("$.sibling", args)
}
