//! Element processing (attributes, children).

use oxc_ast::ast::Statement;

use svelte_analyze::{ContentType, FragmentKey};
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::attributes::{process_attr, process_attrs_spread, process_class_directives, process_style_directives};
use super::each_block::gen_each_block;
use super::expression::{build_concat, emit_trailing_next};
use super::traverse::traverse_items;

/// Process an element's attributes and children.
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
    let ct = ctx
        .analysis
        .content_types
        .get(&child_key)
        .copied()
        .unwrap_or(ContentType::Empty);

    // $.remove_input_defaults for <input> elements with bind or dynamic value attribute
    if ctx.analysis.needs_input_defaults.contains(&el_id) {
        init.push(ctx.b.call_stmt(
            "$.remove_input_defaults",
            [Arg::Ident(el_name)],
        ));
    }

    // Attributes — spread path or per-attribute path
    if ctx.analysis.element_has_spread.contains(&el_id) {
        let el_clone = el.clone_without_fragment();
        process_attrs_spread(ctx, &el_clone, el_name, init, after_update);
    } else {
        let attr_count = el.attributes.len();
        let mut attr_dynamic = Vec::with_capacity(attr_count);
        for idx in 0..attr_count {
            attr_dynamic.push(ctx.analysis.dynamic_attrs.contains(&(el_id, idx)));
        }
        let el = ctx.element(el_id);
        let tag = el.name.clone();
        for (idx, attr) in el.attributes.iter().enumerate() {
            process_attr(ctx, attr, el_id, idx, el_name, &tag, attr_dynamic[idx], init, update, after_update);
        }
    }

    // Class directives
    let el = ctx.element(el_id);
    process_class_directives(ctx, &el.clone_without_fragment(), el_name, init, update);

    // Style directives
    let el = ctx.element(el_id);
    process_style_directives(ctx, &el.clone_without_fragment(), el_name, init, update);

    // Children
    let has_state = ctx.analysis.fragment_has_dynamic_children.contains(&child_key);
    match ct {
        ContentType::Empty | ContentType::StaticText => {}

        ContentType::DynamicText if !has_state => {
            // textContent shortcut
            let items: Vec<_> = ctx
                .analysis
                .lowered_fragments
                .get(&child_key)
                .unwrap()
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

        ContentType::DynamicText => {
            let text_name = ctx.gen_ident("text");
            let items: Vec<_> = ctx
                .analysis
                .lowered_fragments
                .get(&child_key)
                .unwrap()
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

        ContentType::SingleBlock if matches!(
            ctx.lowered_fragment(&child_key).items.first(),
            Some(svelte_analyze::FragmentItem::EachBlock(_))
        ) => {
            // Controlled each block: element itself is the anchor, no $.child() traversal
            let each_id = ctx.lowered_fragment(&child_key).first_each_block_id().unwrap();
            gen_each_block(ctx, each_id, ctx.b.rid_expr(el_name), true, init);
            init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));
        }

        ContentType::SingleElement | ContentType::SingleBlock | ContentType::Mixed => {
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
}

/// Check if a fragment item needs a DOM variable.
/// Elements use precomputed `elements_needing_var` from the analysis phase.
pub(crate) fn item_needs_var(item: &svelte_analyze::FragmentItem, ctx: &Ctx<'_>) -> bool {
    match item {
        svelte_analyze::FragmentItem::TextConcat { has_expr, .. } => *has_expr,
        svelte_analyze::FragmentItem::Element(id) => ctx.analysis.elements_needing_var.contains(id),
        svelte_analyze::FragmentItem::ComponentNode(_) | svelte_analyze::FragmentItem::IfBlock(_) | svelte_analyze::FragmentItem::EachBlock(_) | svelte_analyze::FragmentItem::RenderTag(_) | svelte_analyze::FragmentItem::HtmlTag(_) | svelte_analyze::FragmentItem::KeyBlock(_) => {
            true
        }
    }
}
