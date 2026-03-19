//! Template and DOM traversal code generation.

pub(crate) mod attributes;
pub(crate) mod await_block;
pub(crate) mod component;
pub(crate) mod const_tag;
pub(crate) mod debug_tag;
pub(crate) mod each_block;
pub(crate) mod element;
pub(crate) mod expression;
pub(crate) mod html;
pub(crate) mod if_block;
pub(crate) mod html_tag;
pub(crate) mod key_block;
pub(crate) mod render_tag;
pub(crate) mod snippet;
pub(crate) mod svelte_boundary;
pub(crate) mod svelte_element;
pub(crate) mod svelte_body;
pub(crate) mod svelte_document;
pub(crate) mod svelte_head;
pub(crate) mod svelte_window;
pub(crate) mod title_element;
pub(crate) mod traverse;

use oxc_ast::ast::Statement;

use svelte_analyze::{ContentStrategy, FragmentItem, FragmentKey};
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use element::process_element;
use expression::{emit_template_effect, emit_text_update, emit_trailing_next};
use html::{element_html, fragment_html};
use await_block::gen_await_block;
use component::gen_component;
use if_block::gen_if_block;
use each_block::gen_each_block;
use html_tag::gen_html_tag;
use key_block::gen_key_block;
use render_tag::gen_render_tag;
use const_tag::emit_const_tags;
use debug_tag::emit_debug_tags;
use svelte_boundary::gen_svelte_boundary;
use svelte_element::gen_svelte_element;
use traverse::traverse_items;

/// Check if template HTML needs `importNode` (flag bit 2).
/// `<video>` requires `importNode` instead of `cloneNode`.
fn needs_import_node(html: &str) -> bool {
    html.contains("<video")
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Returns `(hoisted, body, snippet_stmts, hoistable_snippets)` for the root fragment.
/// `snippet_stmts` are instance-level snippets that go inside the function body.
/// `hoistable_snippets` are module-level snippet declarations.
pub fn gen_root_fragment<'a>(ctx: &mut Ctx<'a>) -> (Vec<Statement<'a>>, Vec<Statement<'a>>, Vec<Statement<'a>>, Vec<Statement<'a>>) {
    let key = FragmentKey::Root;
    let ct = ctx.content_type(&key);

    // Consume "root" name for all content types to keep numbering consistent
    let tpl_name = ctx.gen_ident("root");

    // Collect snippet IDs and pre-consume gen_ident slots for parameter names
    // to avoid collisions (e.g., badge's `text` param reserves the "text" counter)
    let mut instance_snippet_ids = Vec::new();
    let mut hoistable_snippet_ids = Vec::new();
    for node in &ctx.component.fragment.nodes {
        if let svelte_ast::Node::SnippetBlock(block) = node {
            if let Some(params) = ctx.analysis.snippets.params(block.id) {
                for param in params {
                    ctx.gen_ident(param);
                }
            }
            if ctx.is_snippet_hoistable(block.id) {
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

    emit_const_tags(ctx, key, &mut body);
    emit_debug_tags(ctx, key, &mut body);

    // Generate svelte:window events/bindings — go to init (before template)
    let svelte_window_ids: Vec<_> = ctx.component.fragment.nodes.iter()
        .filter_map(|n| n.as_svelte_window().map(|w| w.id))
        .collect();
    for id in svelte_window_ids {
        svelte_window::gen_svelte_window(ctx, id, &mut body);
    }

    // Generate svelte:document events/bindings — go to init (before template)
    let svelte_document_ids: Vec<_> = ctx.component.fragment.nodes.iter()
        .filter_map(|n| n.as_svelte_document().map(|d| d.id))
        .collect();
    for id in svelte_document_ids {
        svelte_document::gen_svelte_document(ctx, id, &mut body);
    }

    // Generate svelte:body events/actions — go to init (before template)
    let svelte_body_ids: Vec<_> = ctx.component.fragment.nodes.iter()
        .filter_map(|n| n.as_svelte_body().map(|b| b.id))
        .collect();
    for id in svelte_body_ids {
        svelte_body::gen_svelte_body(ctx, id, &mut body);
    }

    // Collect SvelteHead IDs — $.head() calls are generated after main template init
    let svelte_head_ids: Vec<_> = ctx.component.fragment.nodes.iter()
        .filter_map(|n| n.as_svelte_head().map(|h| h.id))
        .collect();

    match ct {
        ContentStrategy::Empty => {}
        ContentStrategy::Static(ref text) => gen_root_static_text(ctx, text, &mut body),
        ContentStrategy::DynamicText => gen_root_dynamic_text(ctx, &mut body),
        ContentStrategy::SingleElement(el_id) => gen_root_single_element(ctx, el_id, &tpl_name, &mut hoisted, &mut body),
        ContentStrategy::SingleBlock(ref kind) => gen_root_single_block(ctx, kind, &mut body),
        ContentStrategy::Mixed { .. } => gen_root_mixed(ctx, &tpl_name, &mut hoisted, &mut body),
    }

    // Generate $.head() calls for <svelte:head> nodes.
    // In the Svelte reference, hoisted nodes are visited first and push to init[],
    // while the template var decl is unshifted to init[0]. So $.head() ends up
    // after the template init but before $.append().
    if !svelte_head_ids.is_empty() {
        // Find the $.append() call at the end and insert before it
        let insert_pos = body.len().saturating_sub(
            if matches!(ct, ContentStrategy::SingleElement(_) | ContentStrategy::DynamicText | ContentStrategy::Mixed { .. }) { 1 } else { 0 }
        );
        let mut head_stmts = Vec::new();
        for id in svelte_head_ids {
            svelte_head::gen_svelte_head(ctx, id, &mut head_stmts);
        }
        // Insert head stmts before the append
        let tail: Vec<_> = body.drain(insert_pos..).collect();
        body.extend(head_stmts);
        body.extend(tail);
    }

    (hoisted, body, snippet_stmts, hoistable_snippets)
}

// ---------------------------------------------------------------------------
// Root strategies
// ---------------------------------------------------------------------------

fn gen_root_static_text<'a>(ctx: &mut Ctx<'a>, text: &str, body: &mut Vec<Statement<'a>>) {
    let name = ctx.gen_ident("text");
    body.push(ctx.b.call_stmt("$.next", []));
    let call = if text.is_empty() {
        ctx.b.call_expr("$.text", [])
    } else {
        ctx.b.call_expr("$.text", [Arg::Str(text.to_string())])
    };
    body.push(ctx.b.var_stmt(&name, call));
    body.push(ctx.b.call_stmt(
        "$.append",
        [Arg::Ident("$$anchor"), Arg::Ident(&name)],
    ));
}

fn gen_root_dynamic_text<'a>(ctx: &mut Ctx<'a>, body: &mut Vec<Statement<'a>>) {
    // Clone needed: emit_text_update borrows ctx mutably
    let item = ctx.lowered_fragment(&FragmentKey::Root).items[0].clone();
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
    el_id: NodeId,
    tpl_name: &str,
    hoisted: &mut Vec<Statement<'a>>,
    body: &mut Vec<Statement<'a>>,
) {

    let el = ctx.element(el_id);
    let html = element_html(ctx, el);
    let from_html = if needs_import_node(&html) {
        ctx.b.call_expr("$.from_html", [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(2.0)])
    } else {
        ctx.b.call_expr("$.from_html", [Arg::Expr(ctx.b.template_str_expr(&html))])
    };
    hoisted.push(ctx.b.var_stmt(tpl_name, from_html));

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

fn gen_root_single_block<'a>(ctx: &mut Ctx<'a>, item: &FragmentItem, body: &mut Vec<Statement<'a>>) {

    // RenderTag / ComponentNode at root: call directly with $$anchor, no wrapping
    match item {
        FragmentItem::RenderTag(id) => {
            gen_render_tag(ctx, *id, ctx.b.rid_expr("$$anchor"), body);
            return;
        }
        FragmentItem::ComponentNode(id) => {
            gen_component(ctx, *id, ctx.b.rid_expr("$$anchor"), body);
            return;
        }
        FragmentItem::TitleElement(id) => {
            title_element::gen_title_element(ctx, *id, body);
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
            let stmts = gen_if_block(ctx, *id, ctx.b.rid_expr(&node));
            body.push(ctx.b.block_stmt(stmts));
        }
        FragmentItem::EachBlock(id) => {
            gen_each_block(ctx, *id, ctx.b.rid_expr(&node), false, body);
        }
        FragmentItem::HtmlTag(id) => {
            gen_html_tag(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        FragmentItem::KeyBlock(id) => {
            gen_key_block(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        FragmentItem::SvelteElement(id) => {
            gen_svelte_element(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        FragmentItem::SvelteBoundary(id) => {
            gen_svelte_boundary(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        FragmentItem::AwaitBlock(id) => {
            gen_await_block(ctx, *id, ctx.b.rid_expr(&node), body);
        }
        _ => unreachable!("SingleBlock should not contain Element or TextConcat"),
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
    // Clone needed: traverse_items borrows ctx mutably
    let items: Vec<_> = ctx.lowered_fragment(&FragmentKey::Root).items.clone();

    let starts_text = matches!(items.first(), Some(FragmentItem::TextConcat { .. }));
    if starts_text {
        body.push(ctx.b.call_stmt("$.next", []));
    }

    let html = fragment_html(ctx, FragmentKey::Root);
    let flags = if needs_import_node(&html) { 3.0 } else { 1.0 };
    hoisted.push(ctx.b.var_stmt(
        tpl_name,
        ctx.b.call_expr(
            "$.from_html",
            [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(flags)],
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
    let ct = ctx.content_type(&key);

    // Consume "root" name for all content types to keep numbering consistent
    let tpl_name = ctx.gen_ident("root");

    let mut body: Vec<Statement<'a>> = Vec::new();

    emit_const_tags(ctx, key, &mut body);
    emit_debug_tags(ctx, key, &mut body);

    match ct {
        ContentStrategy::Empty => {}
        ContentStrategy::Static(ref text) => {
            let name = ctx.gen_ident("text");
            let call = if text.is_empty() {
                ctx.b.call_expr("$.text", [])
            } else {
                ctx.b.call_expr("$.text", [Arg::Str(text.clone())])
            };
            body.push(ctx.b.var_stmt(&name, call));
            body.push(ctx.b.call_stmt(
                "$.append",
                [Arg::Ident("$$anchor"), Arg::Ident(&name)],
            ));
        }
        ContentStrategy::DynamicText => {
            // Clone needed: emit_text_update borrows ctx mutably
            let item = ctx.lowered_fragment(&key).items[0].clone();
            let name = ctx.gen_ident("text");
            body.push(ctx.b.var_stmt(&name, ctx.b.call_expr("$.text", [])));
            emit_text_update(ctx, &item, &name, &mut body);
            body.push(ctx.b.call_stmt(
                "$.append",
                [Arg::Ident("$$anchor"), Arg::Ident(&name)],
            ));
        }
        ContentStrategy::SingleElement(el_id) => {
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
            let from_html = if needs_import_node(&html) {
                ctx.b.call_expr("$.from_html", [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(2.0)])
            } else {
                ctx.b.call_expr("$.from_html", [Arg::Expr(ctx.b.template_str_expr(&html))])
            };
            ctx.module_hoisted.push(ctx.b.var_stmt(&tpl_name, from_html));

            // Prepend const tags from body, then element code
            body.push(ctx.b.var_stmt(&el_name, ctx.b.call_expr(&tpl_name, [])));
            body.extend(init);
            emit_template_effect(ctx, update, &mut body);
            body.extend(after_update);
            body.push(ctx.b.call_stmt(
                "$.append",
                [Arg::Ident("$$anchor"), Arg::Ident(&el_name)],
            ));
            return body;
        }
        ContentStrategy::SingleBlock(ref item) => {
            // RenderTag / ComponentNode: call directly with $$anchor
            // Still consume a "fragment" ident for consistent numbering
            match item {
                FragmentItem::RenderTag(id) => {
                    ctx.gen_ident("fragment");
                    gen_render_tag(ctx, *id, ctx.b.rid_expr("$$anchor"), &mut body);
                }
                FragmentItem::ComponentNode(id) => {
                    ctx.gen_ident("fragment");
                    gen_component(ctx, *id, ctx.b.rid_expr("$$anchor"), &mut body);
                }
                FragmentItem::TitleElement(id) => {
                    ctx.gen_ident("fragment");
                    title_element::gen_title_element(ctx, *id, &mut body);
                }
                FragmentItem::Element(_) | FragmentItem::TextConcat { .. } => {
                    unreachable!("SingleBlock should not contain Element or TextConcat")
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
                            let stmts = gen_if_block(ctx, *id, ctx.b.rid_expr(&node));
                            body.push(ctx.b.block_stmt(stmts));
                        }
                        FragmentItem::EachBlock(id) => {
                            gen_each_block(ctx, *id, ctx.b.rid_expr(&node), false, &mut body);
                        }
                        FragmentItem::HtmlTag(id) => {
                            gen_html_tag(ctx, *id, ctx.b.rid_expr(&node), &mut body);
                        }
                        FragmentItem::KeyBlock(id) => {
                            gen_key_block(ctx, *id, ctx.b.rid_expr(&node), &mut body);
                        }
                        FragmentItem::SvelteElement(id) => {
                            gen_svelte_element(ctx, *id, ctx.b.rid_expr(&node), &mut body);
                        }
                        FragmentItem::SvelteBoundary(id) => {
                            gen_svelte_boundary(ctx, *id, ctx.b.rid_expr(&node), &mut body);
                        }
                        FragmentItem::AwaitBlock(id) => {
                            gen_await_block(ctx, *id, ctx.b.rid_expr(&node), &mut body);
                        }
                        _ => unreachable!("SingleBlock should not contain Element, TextConcat, RenderTag, ComponentNode, or TitleElement here"),
                    }
                    body.push(ctx.b.call_stmt(
                        "$.append",
                        [Arg::Ident("$$anchor"), Arg::Ident(&frag)],
                    ));
                }
            }
        }
        ContentStrategy::Mixed { .. } => {
            // Clone needed: traverse_items borrows ctx mutably
            let items: Vec<_> = ctx.lowered_fragment(&key).items.clone();

            let starts_text = matches!(items.first(), Some(FragmentItem::TextConcat { .. }));
            if starts_text {
                body.push(ctx.b.call_stmt("$.next", []));
            }

            let html = fragment_html(ctx, key);
            let flags = if needs_import_node(&html) { 3.0 } else { 1.0 };
            let tpl_stmt = ctx.b.var_stmt(
                &tpl_name,
                ctx.b.call_expr(
                    "$.from_html",
                    [Arg::Expr(ctx.b.template_str_expr(&html)), Arg::Num(flags)],
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

