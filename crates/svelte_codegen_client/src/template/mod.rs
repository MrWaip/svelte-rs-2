//! Template and DOM traversal code generation.

pub(crate) mod attributes;
pub(crate) mod component;
pub(crate) mod each_block;
pub(crate) mod element;
pub(crate) mod expression;
pub(crate) mod html;
pub(crate) mod if_block;
pub(crate) mod render_tag;
pub(crate) mod snippet;
pub(crate) mod traverse;

use oxc_ast::ast::Statement;

use svelte_analyze::{ContentType, FragmentItem, FragmentKey};

use crate::builder::Arg;
use crate::context::Ctx;

use element::process_element;
use expression::{emit_template_effect, emit_text_update, emit_trailing_next, static_text_of};
use html::{element_html, fragment_html};
use component::gen_component;
use if_block::gen_if_block;
use each_block::gen_each_block;
use render_tag::gen_render_tag;
use traverse::traverse_items;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Returns `(hoisted, body, snippet_stmts, hoistable_snippets)` for the root fragment.
/// `snippet_stmts` are instance-level snippets that go inside the function body.
/// `hoistable_snippets` are module-level snippet declarations.
pub fn gen_root_fragment<'a>(ctx: &mut Ctx<'a>) -> (Vec<Statement<'a>>, Vec<Statement<'a>>, Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let key = FragmentKey::Root;
    let ct = ctx
        .analysis
        .content_types
        .get(&key)
        .copied()
        .unwrap_or(ContentType::Empty);

    // Consume "root" name for all content types to keep numbering consistent
    let tpl_name = ctx.gen_ident("root");

    // Collect snippet IDs and pre-consume gen_ident slots for parameter names
    // to avoid collisions (e.g., badge's `text` param reserves the "text" counter)
    let mut instance_snippet_ids = Vec::new();
    let mut hoistable_snippet_ids = Vec::new();
    for node in &ctx.component.fragment.nodes {
        if let svelte_ast::Node::SnippetBlock(block) = node {
            if let Some(params) = ctx.analysis.snippet_params.get(&block.id) {
                for param in params {
                    ctx.gen_ident(param);
                }
            }
            if ctx.analysis.hoistable_snippets.contains(&block.id) {
                hoistable_snippet_ids.push(block.id);
            } else {
                instance_snippet_ids.push(block.id);
            }
        }
    }

    // Generate snippet bodies: instance first, then hoistable (ordering matters for ident numbering)
    let mut snippet_stmts: Vec<Statement<'a>> = Vec::new();
    for id in instance_snippet_ids {
        snippet_stmts.push(snippet::gen_snippet_block(ctx, id));
    }
    let mut hoistable_snippets: Vec<Statement<'a>> = Vec::new();
    for id in hoistable_snippet_ids {
        hoistable_snippets.push(snippet::gen_snippet_block(ctx, id));
    }

    let mut hoisted = Vec::new();
    let mut body = Vec::new();

    match ct {
        ContentType::Empty => {}
        ContentType::StaticText => gen_root_static_text(ctx, &mut body),
        ContentType::DynamicText => gen_root_dynamic_text(ctx, &mut body),
        ContentType::SingleElement => gen_root_single_element(ctx, &tpl_name, &mut hoisted, &mut body),
        ContentType::SingleBlock => gen_root_single_block(ctx, &mut body),
        ContentType::Mixed => gen_root_mixed(ctx, &tpl_name, &mut hoisted, &mut body),
    }

    (hoisted, body, snippet_stmts, hoistable_snippets)
}

// ---------------------------------------------------------------------------
// Root strategies
// ---------------------------------------------------------------------------

fn gen_root_static_text<'a>(ctx: &mut Ctx<'a>, body: &mut Vec<Statement<'a>>) {
    let lf = ctx
        .analysis
        .lowered_fragments
        .get(&FragmentKey::Root)
        .unwrap();
    let text = static_text_of(&lf.items[0]);
    let name = ctx.gen_ident("text");
    body.push(ctx.b.call_stmt("$.next", []));
    let call = if text.is_empty() {
        ctx.b.call_expr("$.text", [])
    } else {
        ctx.b.call_expr("$.text", [Arg::Str(text)])
    };
    body.push(ctx.b.var_stmt(&name, call));
    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident("$$anchor"), Arg::Ident(&name)],
    ));
}

fn gen_root_dynamic_text<'a>(ctx: &mut Ctx<'a>, body: &mut Vec<Statement<'a>>) {
    let item = {
        let lf = ctx
            .analysis
            .lowered_fragments
            .get(&FragmentKey::Root)
            .unwrap();
        lf.items[0].clone()
    };
    let name = ctx.gen_ident("text");
    body.push(ctx.b.call_stmt("$.next", []));
    body.push(ctx.b.var_stmt(&name, ctx.b.call_expr("$.text", [])));

    emit_text_update(ctx, &item, &name, body);

    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident("$$anchor"), Arg::Ident(&name)],
    ));
}

fn gen_root_single_element<'a>(
    ctx: &mut Ctx<'a>,
    tpl_name: &str,
    hoisted: &mut Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    let el_id = {
        let lf = ctx
            .analysis
            .lowered_fragments
            .get(&FragmentKey::Root)
            .unwrap();
        match lf.items[0] {
            FragmentItem::Element(id) => id,
            _ => unreachable!(),
        }
    };

    let el = ctx.element(el_id);
    let html = element_html(ctx, el);
    hoisted.push(ctx.b.var_stmt(
        tpl_name,
        ctx.b
            .call_expr("$.from_html", [Arg::Expr(ctx.b.template_str_expr(&html))]),
    ));

    let el_name_str = ctx.element(el_id).name.clone();
    let el_name = ctx.gen_ident(&el_name_str);
    body.push(ctx.b.var_stmt(&el_name, ctx.b.call_expr(tpl_name, [])));

    let mut init = Vec::new();
    let mut update = Vec::new();
    let mut after_update = Vec::new();
    process_element(ctx, el_id, &el_name, &mut init, &mut update, hoisted, &mut after_update);
    body.extend(init);
    emit_template_effect(ctx, update, body);
    body.extend(after_update);
    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident("$$anchor"), Arg::Ident(&el_name)],
    ));
}

fn gen_root_single_block<'a>(ctx: &mut Ctx<'a>, body: &mut Vec<Statement<'a>>) {
    let item = {
        let lf = ctx
            .analysis
            .lowered_fragments
            .get(&FragmentKey::Root)
            .unwrap();
        lf.items[0].clone()
    };

    // RenderTag / ComponentNode at root: call directly with $$anchor, no wrapping
    match item {
        FragmentItem::RenderTag(id) => {
            gen_render_tag(ctx, id, ctx.b.rid_expr("$$anchor"), body);
            return;
        }
        FragmentItem::ComponentNode(id) => {
            gen_component(ctx, id, ctx.b.rid_expr("$$anchor"), body);
            return;
        }
        _ => {}
    }

    let frag = ctx.gen_ident("fragment");
    let node = ctx.gen_ident("node");
    body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr("$.comment", [])));
    body.push(ctx.b.var_stmt(
        &node,
        ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]),
    ));

    match item {
        FragmentItem::IfBlock(id) => {
            let stmts = gen_if_block(ctx, id, ctx.b.rid_expr(&node));
            body.push(ctx.b.block_stmt(stmts));
        }
        FragmentItem::EachBlock(id) => {
            gen_each_block(ctx, id, ctx.b.rid_expr(&node), false, body);
        }
        _ => unreachable!(),
    }

    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident("$$anchor"), Arg::Ident(&frag)],
    ));
}

fn gen_root_mixed<'a>(
    ctx: &mut Ctx<'a>,
    tpl_name: &str,
    hoisted: &mut Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    let items: Vec<_> = {
        let lf = ctx
            .analysis
            .lowered_fragments
            .get(&FragmentKey::Root)
            .unwrap();
        lf.items.clone()
    };

    let starts_text = matches!(items.first(), Some(FragmentItem::TextConcat { .. }));
    if starts_text {
        body.push(ctx.b.call_stmt("$.next", []));
    }

    let html = fragment_html(ctx, FragmentKey::Root);
    hoisted.push(ctx.b.var_stmt(
        tpl_name,
        ctx.b.call_expr(
            "$.from_html",
            [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(1.0)],
        ),
    ));

    let frag = ctx.gen_ident("fragment");
    body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr(tpl_name, [])));

    let first_child = ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]);
    let mut init = Vec::new();
    let mut update = Vec::new();
    let mut after_update = Vec::new();
    let trailing = traverse_items(
        ctx,
        &items,
        first_child,
        &mut init,
        &mut update,
        hoisted,
        &mut after_update,
    );
    emit_trailing_next(ctx, trailing, &mut init);
    body.extend(init);
    emit_template_effect(ctx, update, body);
    body.extend(after_update);
    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident("$$anchor"), Arg::Ident(&frag)],
    ));
}

// ---------------------------------------------------------------------------
// Fragment codegen (used for if/each/nested fragments)
// ---------------------------------------------------------------------------

/// Generate statements for a fragment, destined for `($$anchor) => { ... }`.
pub(crate) fn gen_fragment<'a>(ctx: &mut Ctx<'a>, key: FragmentKey) -> Vec<Statement<'a>> {
    let ct = ctx
        .analysis
        .content_types
        .get(&key)
        .copied()
        .unwrap_or(ContentType::Empty);

    // Consume "root" name for all content types to keep numbering consistent
    let tpl_name = ctx.gen_ident("root");

    let mut body: Vec<Statement<'a>> = Vec::new();

    match ct {
        ContentType::Empty => {}
        ContentType::StaticText => {
            let text = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                static_text_of(&lf.items[0])
            };
            let name = ctx.gen_ident("text");
            let call = if text.is_empty() {
                ctx.b.call_expr("$.text", [])
            } else {
                ctx.b.call_expr("$.text", [Arg::Str(text)])
            };
            body.push(ctx.b.var_stmt(&name, call));
            body.push(ctx.b.call_stmt(
                "$.append",
                [Arg::Ident("$$anchor"), Arg::Ident(&name)],
            ));
        }
        ContentType::DynamicText => {
            let item = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                lf.items[0].clone()
            };
            let name = ctx.gen_ident("text");
            body.push(ctx.b.var_stmt(&name, ctx.b.call_expr("$.text", [])));
            emit_text_update(ctx, &item, &name, &mut body);
            body.push(ctx.b.call_stmt(
                "$.append",
                [Arg::Ident("$$anchor"), Arg::Ident(&name)],
            ));
        }
        ContentType::SingleElement => {
            let el_id = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                match lf.items[0] {
                    FragmentItem::Element(id) => id,
                    _ => unreachable!(),
                }
            };

            let el = ctx.element(el_id);
            let html = element_html(ctx, el);

            let el_name_str = ctx.element(el_id).name.clone();
            let el_name = ctx.gen_ident(&el_name_str);

            let mut init = Vec::new();
            let mut update = Vec::new();
            let mut sub_hoisted = Vec::new();
            let mut after_update = Vec::new();
            process_element(
                ctx,
                el_id,
                &el_name,
                &mut init,
                &mut update,
                &mut sub_hoisted,
                &mut after_update,
            );
            ctx.module_hoisted.extend(sub_hoisted);

            // Hoist AFTER children so inner templates come first (bottom-up order)
            ctx.module_hoisted.push(ctx.b.var_stmt(
                &tpl_name,
                ctx.b
                    .call_expr("$.from_html", [Arg::Expr(ctx.b.template_str_expr(&html))]),
            ));

            let mut result = Vec::new();
            result.push(ctx.b.var_stmt(&el_name, ctx.b.call_expr(&tpl_name, [])));
            result.extend(init);
            emit_template_effect(ctx, update, &mut result);
            result.extend(after_update);
            result.push(ctx.b.call_stmt(
                "$.append",
                [Arg::Ident("$$anchor"), Arg::Ident(&el_name)],
            ));
            return result;
        }
        ContentType::SingleBlock => {
            let item = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                lf.items[0].clone()
            };

            // RenderTag / ComponentNode: call directly with $$anchor
            // Still consume a "fragment" ident for consistent numbering
            match item {
                FragmentItem::RenderTag(id) => {
                    ctx.gen_ident("fragment");
                    gen_render_tag(ctx, id, ctx.b.rid_expr("$$anchor"), &mut body);
                }
                FragmentItem::ComponentNode(id) => {
                    ctx.gen_ident("fragment");
                    gen_component(ctx, id, ctx.b.rid_expr("$$anchor"), &mut body);
                }
                _ => {
                    let frag = ctx.gen_ident("fragment");
                    let node = ctx.gen_ident("node");
                    body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr("$.comment", [])));
                    body.push(ctx.b.var_stmt(
                        &node,
                        ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]),
                    ));
                    match item {
                        FragmentItem::IfBlock(id) => {
                            let stmts = gen_if_block(ctx, id, ctx.b.rid_expr(&node));
                            body.push(ctx.b.block_stmt(stmts));
                        }
                        FragmentItem::EachBlock(id) => {
                            gen_each_block(ctx, id, ctx.b.rid_expr(&node), false, &mut body);
                        }
                        _ => unreachable!(),
                    }
                    body.push(ctx.b.call_stmt(
                        "$.append",
                        [Arg::Ident("$$anchor"), Arg::Ident(&frag)],
                    ));
                }
            }
        }
        ContentType::Mixed => {
            let items: Vec<_> = {
                let lf = ctx.analysis.lowered_fragments.get(&key).unwrap();
                lf.items.clone()
            };

            let starts_text = matches!(items.first(), Some(FragmentItem::TextConcat { .. }));
            if starts_text {
                body.push(ctx.b.call_stmt("$.next", []));
            }

            let html = fragment_html(ctx, key);
            let tpl_stmt = ctx.b.var_stmt(
                &tpl_name,
                ctx.b.call_expr(
                    "$.from_html",
                    [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(1.0)],
                ),
            );

            let frag = ctx.gen_ident("fragment");
            body.push(ctx.b.var_stmt(&frag, ctx.b.call_expr(&tpl_name, [])));

            let first = ctx.b.call_expr("$.first_child", [Arg::Ident(&frag)]);
            let mut init = Vec::new();
            let mut update = Vec::new();
            let mut sub_hoisted = Vec::new();
            let mut after_update = Vec::new();
            let trailing = traverse_items(
                ctx,
                &items,
                first,
                &mut init,
                &mut update,
                &mut sub_hoisted,
                &mut after_update,
            );
            ctx.module_hoisted.extend(sub_hoisted);
            // Push own template AFTER inner templates (bottom-up order)
            ctx.module_hoisted.push(tpl_stmt);
            emit_trailing_next(ctx, trailing, &mut init);
            body.extend(init);
            emit_template_effect(ctx, update, &mut body);
            body.extend(after_update);
            body.push(ctx.b.call_stmt(
                "$.append",
                [Arg::Ident("$$anchor"), Arg::Ident(&frag)],
            ));
            return body;
        }
    }

    body
}
