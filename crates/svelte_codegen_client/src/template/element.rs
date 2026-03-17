//! Element processing (attributes, children).

use oxc_ast::ast::Statement;

use svelte_analyze::{ContentStrategy, FragmentKey};
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::attributes::{process_attr, process_attrs_spread, process_class_attribute_and_directives, process_style_directives};
use super::each_block::gen_each_block;
use super::expression::{build_concat, emit_trailing_next};
use super::traverse::traverse_items;

/// Process an element's attributes and children.
///
/// Statement ordering matches the Svelte reference compiler's two-state model:
/// - Simple attributes ($.set_attribute, etc.) → parent state → before children
/// - Directives ($.attach, $.action, $.bind_*) → element_state → after children
///
/// Merge order: simple attrs → children init → directive init → updates → after_update
pub(crate) fn process_element<'a>(
    ctx: &mut Ctx<'a>,
    el_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    hoisted: &mut Vec<Statement<'a>>,
    after_update: &mut Vec<Statement<'a>>,
) {
    let el = ctx.element(el_id);
    let child_key = FragmentKey::Element(el_id);
    let ct = ctx.content_type(&child_key);

    // $.remove_input_defaults for <input> elements with bind or dynamic value attribute
    if ctx.needs_input_defaults(el_id) {
        init.push(ctx.b.call_stmt(
            "$.remove_input_defaults",
            [Arg::Ident(el_name)],
        ));
    }

    // --- Attributes (simple attrs go into init/update directly; directives into separate buffers) ---
    // Directive init/after_update are collected separately and merged after children.
    let mut directive_init: Vec<Statement<'a>> = Vec::new();
    let mut directive_after_update: Vec<Statement<'a>> = Vec::new();

    if ctx.has_spread(el_id) {
        let el_clone = el.clone_without_fragment();
        process_attrs_spread(ctx, &el_clone, el_name, init, after_update);
    } else {
        let attr_count = el.attributes.len();
        let mut attr_dynamic = Vec::with_capacity(attr_count);
        for idx in 0..attr_count {
            attr_dynamic.push(ctx.is_dynamic_attr(el_id, idx));
        }
        let el = ctx.element(el_id);
        let tag = el.name.clone();
        for (idx, attr) in el.attributes.iter().enumerate() {
            process_attr(ctx, attr, el_id, idx, el_name, &tag, attr_dynamic[idx], init, update, &mut directive_init, &mut directive_after_update);
        }
    }

    // Class attribute and/or class directives
    let el = ctx.element(el_id);
    process_class_attribute_and_directives(ctx, &el.clone_without_fragment(), el_name, init, update);

    // Style directives
    let el = ctx.element(el_id);
    process_style_directives(ctx, &el.clone_without_fragment(), el_name, init, update);

    // --- Children ---
    let has_state = ctx.has_dynamic_children(&child_key);
    match ct {
        ContentStrategy::Empty | ContentStrategy::Static(_) => {}

        ContentStrategy::Dynamic { has_elements: false, has_blocks: false, .. } if !has_state => {
            // textContent shortcut
            let items: Vec<_> = ctx
                .lowered_fragment(&child_key)
                .items
                .clone();
            let expr = build_concat(ctx, &items[0]);
            init.push(ctx.b.assign_stmt(
                crate::builder::AssignLeft::StaticMember(
                    ctx.b.static_member(ctx.b.rid_expr(el_name), "textContent"),
                ),
                crate::builder::AssignRight::Expr(expr),
            ));
        }

        ContentStrategy::Dynamic { has_elements: false, has_blocks: false, .. } => {
            let text_name = ctx.gen_ident("text");
            let items: Vec<_> = ctx
                .lowered_fragment(&child_key)
                .items
                .clone();
            // is_text only for standalone {expression} (single Expr part, no surrounding text)
            let is_text = items.first().is_some_and(|item| item.is_standalone_expr());
            let child_call = if is_text {
                ctx.b.call_expr("$.child", [Arg::Ident(el_name), Arg::Bool(true)])
            } else {
                ctx.b.call_expr("$.child", [Arg::Ident(el_name)])
            };
            init.push(ctx.b.var_stmt(&text_name, child_call));
            init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));

            let expr = build_concat(ctx, &items[0]);
            update.push(ctx.b.call_stmt(
                "$.set_text",
                [Arg::Ident(&text_name), Arg::Expr(expr)],
            ));
        }

        ContentStrategy::SingleBlock(id) if ctx.is_each_block(id) => {
            // Controlled each block: element itself is the anchor, no $.child() traversal
            gen_each_block(ctx, id, ctx.b.rid_expr(el_name), true, init);
            init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
        }

        ContentStrategy::SingleElement(_) | ContentStrategy::SingleBlock(_) | ContentStrategy::Dynamic { .. } => {
            // Clone needed: traverse_items borrows ctx mutably
            let child_items: Vec<_> = ctx.lowered_fragment(&child_key).items.clone();

            let first_is_text = child_items.first().is_some_and(|item| item.is_standalone_expr());
            let first_child = if first_is_text {
                ctx.b.call_expr("$.child", [Arg::Ident(el_name), Arg::Bool(true)])
            } else {
                ctx.b.call_expr("$.child", [Arg::Ident(el_name)])
            };
            let mut child_init = Vec::new();
            let mut child_update = Vec::new();
            let trailing = traverse_items(
                ctx,
                &child_items,
                first_child,
                &mut child_init,
                &mut child_update,
                hoisted,
                after_update,
            );

            emit_trailing_next(ctx, trailing, &mut child_init);
            if !child_init.is_empty() {
                child_init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
            }

            init.extend(child_init);
            update.extend(child_update);
        }
    }

    // --- Merge directive statements after children (matching Svelte's element_state merge) ---
    init.extend(directive_init);
    after_update.extend(directive_after_update);
}

/// Check if a fragment item needs a DOM variable.
/// Elements use precomputed `element_flags.needs_var` from the analysis phase.
pub(crate) fn item_needs_var(item: &svelte_analyze::FragmentItem, ctx: &Ctx<'_>) -> bool {
    match item {
        svelte_analyze::FragmentItem::TextConcat { has_expr, .. } => *has_expr,
        svelte_analyze::FragmentItem::Element(id) => ctx.needs_var(*id),
        svelte_analyze::FragmentItem::ComponentNode(_) | svelte_analyze::FragmentItem::IfBlock(_) | svelte_analyze::FragmentItem::EachBlock(_) | svelte_analyze::FragmentItem::RenderTag(_) | svelte_analyze::FragmentItem::HtmlTag(_) | svelte_analyze::FragmentItem::KeyBlock(_) | svelte_analyze::FragmentItem::SvelteElement(_) => {
            true
        }
    }
}
