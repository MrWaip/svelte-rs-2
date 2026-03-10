//! Element processing (attributes, children).

use oxc_ast::ast::Statement;

use svelte_analyze::{ContentType, FragmentKey};
use svelte_ast::{Attribute, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

use super::attributes::{has_spread, process_attr, process_attrs_spread};
use super::expression::{build_concat, emit_trailing_next, item_is_dynamic};
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

    // $.remove_input_defaults for <input> elements with any bind directive
    let is_input = el.name == "input";
    let has_bind = el
        .attributes
        .iter()
        .any(|a| matches!(a, Attribute::BindDirective(_)));
    if is_input && has_bind {
        init.push(ctx.b.call_stmt(
            "$.remove_input_defaults",
            [Arg::Ident(el_name)],
        ));
    }

    // Attributes — spread path or per-attribute path
    if has_spread(el) {
        let el_clone = el.clone_without_fragment();
        process_attrs_spread(ctx, &el_clone, el_name, init, after_update);
    } else {
        let attr_count = el.attributes.len();
        let mut attr_dynamic = Vec::with_capacity(attr_count);
        for idx in 0..attr_count {
            attr_dynamic.push(ctx.analysis.dynamic_attrs.contains(&(el_id, idx)));
        }
        let el = ctx.element(el_id);
        for (idx, attr) in el.attributes.iter().enumerate() {
            process_attr(ctx, attr, el_name, attr_dynamic[idx], init, update, after_update);
        }
    }

    // Children
    let has_state = has_dynamic_children(ctx, el_id);
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
            init.push(ctx.b.var_stmt(
                &text_name,
                ctx.b.call_expr("$.child", [Arg::Ident(el_name), Arg::Bool(true)]),
            ));
            init.push(ctx.b.call_stmt("$.reset", [Arg::Ident(el_name)]));

            let items: Vec<_> = ctx
                .analysis
                .lowered_fragments
                .get(&child_key)
                .unwrap()
                .items
                .clone();
            let expr = build_concat(ctx, &items[0]);
            update.push(ctx.b.call_stmt(
                "$.set_text",
                [Arg::Ident(&text_name), Arg::Expr(expr)],
            ));
        }

        ContentType::SingleElement | ContentType::SingleBlock | ContentType::Mixed => {
            let child_items: Vec<_> = ctx
                .analysis
                .lowered_fragments
                .get(&child_key)
                .unwrap()
                .items
                .clone();

            let first_is_text = child_items.first().is_some_and(|item| {
                matches!(item, svelte_analyze::FragmentItem::TextConcat { parts } if parts.len() == 1)
            });
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

fn has_dynamic_children(ctx: &Ctx<'_>, el_id: NodeId) -> bool {
    let key = FragmentKey::Element(el_id);
    let Some(lf) = ctx.analysis.lowered_fragments.get(&key) else {
        return false;
    };
    lf.items.iter().any(|item| item_is_dynamic(item, ctx))
}

/// Check if an element needs a DOM variable during traversal.
pub(crate) fn element_needs_var(ctx: &Ctx<'_>, id: NodeId) -> bool {
    if ctx.analysis.node_needs_ref.contains(&id) {
        return true;
    }
    let el = ctx.element(id);
    let has_runtime_attrs = el
        .attributes
        .iter()
        .any(|a| !matches!(a, Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)));
    if has_runtime_attrs {
        return true;
    }
    let key = FragmentKey::Element(id);
    let ct = ctx
        .analysis
        .content_types
        .get(&key)
        .copied()
        .unwrap_or(ContentType::Empty);
    match ct {
        ContentType::Empty | ContentType::StaticText => false,
        ContentType::DynamicText | ContentType::SingleBlock => true,
        ContentType::SingleElement | ContentType::Mixed => {
            let Some(lf) = ctx.analysis.lowered_fragments.get(&key) else {
                return false;
            };
            lf.items.iter().any(|item| item_needs_var(item, ctx))
        }
    }
}

/// Check if a fragment item needs a DOM variable.
pub(crate) fn item_needs_var(item: &svelte_analyze::FragmentItem, ctx: &Ctx<'_>) -> bool {
    match item {
        svelte_analyze::FragmentItem::TextConcat { parts } => {
            parts
                .iter()
                .any(|p| matches!(p, svelte_analyze::ConcatPart::Expr(_)))
        }
        svelte_analyze::FragmentItem::Element(id) => element_needs_var(ctx, *id),
        svelte_analyze::FragmentItem::IfBlock(_) | svelte_analyze::FragmentItem::EachBlock(_) => {
            true
        }
    }
}
