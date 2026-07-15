use svelte_ast::{Attribute, Component, FragmentId, Node, NodeId, SlotElementLegacy};

use super::super::{LegacyComponentSlotsSemantics, LegacyDefaultSlot, LegacySlotSemantics};

pub(super) fn classify_slot(
    el: &SlotElementLegacy,
    component: &Component,
    source: &str,
) -> LegacySlotSemantics {
    let has_fallback = !component.store.fragment(el.fragment).nodes.is_empty();
    LegacySlotSemantics {
        name: slot_name(&el.attributes, source).to_string(),
        has_fallback,
    }
}

pub(super) fn classify_component_slots(
    attributes: &[Attribute],
    fragment: FragmentId,
    component: &Component,
    source: &str,
) -> LegacyComponentSlotsSemantics {
    LegacyComponentSlotsSemantics {
        default_slot: default_slot_form(attributes, fragment, component),
        default_wrapper: lone_default_svelte_fragment(fragment, component, source),
    }
}

fn slot_name<'s>(attributes: &'s [Attribute], source: &'s str) -> &'s str {
    for attr in attributes {
        if let Attribute::StringAttribute(sa) = attr
            && sa.name == "name"
        {
            return sa.value(source);
        }
    }
    "default"
}

fn default_slot_form(
    attributes: &[Attribute],
    fragment: FragmentId,
    component: &Component,
) -> LegacyDefaultSlot {
    if has_named_attribute(attributes, "children") {
        return LegacyDefaultSlot::SlotDefault;
    }
    if fragment_has_let_svelte_fragment(fragment, component) {
        return LegacyDefaultSlot::SlotDefaultInvalid;
    }
    if has_let_directive(attributes) {
        if has_named_attribute(attributes, "slot") {
            return LegacyDefaultSlot::OwnLetDisplaced;
        }
        return LegacyDefaultSlot::SlotDefaultInvalid;
    }
    LegacyDefaultSlot::ChildrenProp
}

fn has_named_attribute(attributes: &[Attribute], target: &str) -> bool {
    for attr in attributes {
        let name = match attr {
            Attribute::StringAttribute(x) => &x.name,
            Attribute::BooleanAttribute(x) => &x.name,
            Attribute::ExpressionAttribute(x) => &x.name,
            Attribute::ConcatenationAttribute(x) => &x.name,
            _ => continue,
        };
        if name == target {
            return true;
        }
    }
    false
}

fn has_let_directive(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .any(|attr| matches!(attr, Attribute::LetDirectiveLegacy(_)))
}

fn fragment_has_let_svelte_fragment(fragment: FragmentId, component: &Component) -> bool {
    for &child_id in &component.store.fragment(fragment).nodes {
        if let Node::SvelteFragmentLegacy(el) = component.store.get(child_id)
            && has_let_directive(&el.attributes)
        {
            return true;
        }
    }
    false
}

fn lone_default_svelte_fragment(
    fragment: FragmentId,
    component: &Component,
    source: &str,
) -> Option<NodeId> {
    let mut wrapper: Option<NodeId> = None;
    for &child_id in &component.store.fragment(fragment).nodes {
        match component.store.get(child_id) {
            Node::Text(text) if is_whitespace(text.raw_value(source)) => continue,
            Node::SvelteFragmentLegacy(_) => {
                if wrapper.is_some() {
                    return None;
                }
                wrapper = Some(child_id);
            }
            _ => return None,
        }
    }
    wrapper
}

fn is_whitespace(text: &str) -> bool {
    text.chars().all(|c| c.is_ascii_whitespace())
}
