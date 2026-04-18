//! DOM traversal for Mixed fragments — assigns variables, builds sibling calls.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::{FragmentItem, LoweredTextPart};

use svelte_ast_builder::{Arg, AssignLeft};
use crate::context::Ctx;

use super::await_block::gen_await_block;
use super::component::gen_component;
use super::each_block::gen_each_block;
use super::element::{item_needs_var, process_element};
use super::element_ident_prefix;
use super::expression::{parts_are_dynamic, MemoAttr};
use super::html_tag::gen_html_tag;
use super::if_block::gen_if_block;
use super::key_block::gen_key_block;
use super::render_tag::gen_render_tag;
use super::slot::{emit_slot_call, is_legacy_slot_element};

/// Traverse lowered items, assign DOM variables, generate init/update statements.
///
/// Returns `(trailing_sibling_offset, const_tag_blocker_exprs)`.
pub(crate) fn traverse_items<'a>(
    ctx: &mut Ctx<'a>,
    items: &[FragmentItem],
    first_child_expr: Expression<'a>,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    hoisted: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
    memo_attrs: &mut Vec<MemoAttr<'a>>,
) -> usize {
    let mut prev_expr: Option<Expression<'a>> = Some(first_child_expr);
    let mut prev_ident: Option<String> = None;
    let mut sibling_offset: usize = 0;

    for item in items {
        let needs_var = item_needs_var(item, ctx);
        // is_text matches Svelte's `sequence.length === 1`: a standalone {expression}
        // with no surrounding text nodes in the same sequence
        let is_text = item.is_standalone_expr();

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
                FragmentItem::TextConcat { parts, .. } => {
                    let name = ctx.gen_ident("text");
                    init.push(ctx.b.var_stmt(&name, node_expr));
                    sibling_offset = 1;

                    let is_dyn = parts_are_dynamic(parts, ctx);
                    if is_dyn && !ctx.bound_contenteditable {
                        // Collect const-tag blockers into ctx for downstream template_effect
                        for part in parts {
                            if let LoweredTextPart::Expr(id) = part {
                                let exprs = ctx.const_tag_blocker_exprs(*id);
                                ctx.pending_const_blockers.extend(exprs);
                            }
                        }
                        let expr = super::expression::build_concat_from_parts(ctx, parts);
                        update.push(
                            ctx.b
                                .call_stmt("$.set_text", [Arg::Ident(&name), Arg::Expr(expr)]),
                        );
                    } else {
                        // Static text or bound_contenteditable: nodeValue= in init
                        let expr = super::expression::build_concat_from_parts(ctx, parts);
                        init.push(ctx.b.assign_stmt(
                            AssignLeft::StaticMember(
                                ctx.b.static_member(ctx.b.rid_expr(&name), "nodeValue"),
                            ),
                            expr,
                        ));
                    }
                    prev_ident = Some(name);
                }

                FragmentItem::Element(el_id) => {
                    let el_name_str = ctx.element(*el_id).name.clone();
                    let el_name = ctx.gen_ident(&element_ident_prefix(&el_name_str));
                    init.push(ctx.b.var_stmt(&el_name, node_expr));
                    sibling_offset = 1;
                    process_element(
                        ctx,
                        *el_id,
                        &el_name,
                        init,
                        update,
                        hoisted,
                        after_update,
                        memo_attrs,
                    );
                    prev_ident = Some(el_name);
                }

                FragmentItem::SlotElementLegacy(el_id) => {
                    debug_assert!(is_legacy_slot_element(ctx, *el_id));
                    let node_name = ctx.gen_ident("node");
                    init.push(ctx.b.var_stmt(&node_name, node_expr));
                    sibling_offset = 1;
                    emit_slot_call(ctx, *el_id, &node_name, init);
                    prev_ident = Some(node_name);
                }

                FragmentItem::SvelteFragmentLegacy(_) => {
                    unreachable!("transparent legacy fragment items are flattened before traversal")
                }

                FragmentItem::ComponentNode(_)
                | FragmentItem::IfBlock(_)
                | FragmentItem::EachBlock(_)
                | FragmentItem::RenderTag(_)
                | FragmentItem::HtmlTag(_)
                | FragmentItem::KeyBlock(_)
                | FragmentItem::SvelteElement(_)
                | FragmentItem::SvelteBoundary(_)
                | FragmentItem::AwaitBlock(_) => {
                    let node_name = ctx.gen_ident("node");
                    init.push(ctx.b.var_stmt(&node_name, node_expr));
                    sibling_offset = 1;
                    let anchor = ctx.b.rid_expr(&node_name);
                    match item {
                        FragmentItem::ComponentNode(id) => {
                            gen_component(ctx, *id, anchor, init, true)
                        }
                        FragmentItem::IfBlock(id) => {
                            let stmts = gen_if_block(ctx, *id, anchor);
                            init.push(ctx.b.block_stmt(stmts));
                        }
                        FragmentItem::EachBlock(id) => {
                            gen_each_block(ctx, *id, anchor, false, init)
                        }
                        FragmentItem::RenderTag(id) => {
                            gen_render_tag(ctx, *id, anchor, false, init)
                        }
                        FragmentItem::HtmlTag(id) => gen_html_tag(ctx, *id, anchor, false, init),
                        FragmentItem::KeyBlock(id) => gen_key_block(ctx, *id, anchor, init),
                        FragmentItem::SvelteElement(id) => {
                            super::svelte_element::gen_svelte_element(ctx, *id, anchor, init)
                        }
                        FragmentItem::SvelteBoundary(id) => {
                            super::svelte_boundary::gen_svelte_boundary(ctx, *id, anchor, init)
                        }
                        FragmentItem::AwaitBlock(id) => gen_await_block(ctx, *id, anchor, init),
                        _ => unreachable!(),
                    }
                    prev_ident = Some(node_name);
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
