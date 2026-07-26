use smallvec::SmallVec;
use svelte_ast::{
    Attribute, Component, FragmentId, Node, NodeId, SLOT_ATTRIBUTE, SlotElementLegacy,
};

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
    let default_let_scope_owners = default_let_scope_owners(fragment, component);
    let slotted_let_owner = default_let_scope_owners
        .iter()
        .copied()
        .find(|&id| element_has_default_slot_attribute(id, component, source));
    let default_slot = default_slot_form(attributes, fragment, component, slotted_let_owner);
    LegacyComponentSlotsSemantics {
        default_slot,
        default_wrapper: lone_default_svelte_fragment(fragment, component, source),
        default_let_owner: match default_slot {
            LegacyDefaultSlot::SlotDefaultSlottedLet => slotted_let_owner,
            _ => None,
        },
        default_let_scope_owners,
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
    slotted_let_owner: Option<NodeId>,
) -> LegacyDefaultSlot {
    if has_named_attribute(attributes, "children") {
        return LegacyDefaultSlot::SlotDefault;
    }
    if fragment_has_let_svelte_fragment(fragment, component) {
        return LegacyDefaultSlot::SlotDefaultInvalid;
    }
    if has_let_directive(attributes) {
        if has_named_attribute(attributes, SLOT_ATTRIBUTE) {
            return LegacyDefaultSlot::OwnLetDisplaced;
        }
        return LegacyDefaultSlot::SlotDefaultInvalid;
    }
    if slotted_let_owner.is_some() {
        return LegacyDefaultSlot::SlotDefaultSlottedLet;
    }
    LegacyDefaultSlot::ChildrenProp
}

fn default_let_scope_owners(fragment: FragmentId, component: &Component) -> SmallVec<[NodeId; 2]> {
    let mut owners = SmallVec::new();
    for &child_id in &component.store.fragment(fragment).nodes {
        let attrs = match component.store.get(child_id) {
            Node::Element(el) => &el.attributes,
            Node::SvelteElement(el) => &el.attributes,
            _ => continue,
        };
        if has_let_directive(attrs) {
            owners.push(child_id);
        }
    }
    owners
}

fn element_has_default_slot_attribute(
    child_id: NodeId,
    component: &Component,
    source: &str,
) -> bool {
    let attrs = match component.store.get(child_id) {
        Node::Element(el) => &el.attributes,
        Node::SvelteElement(el) => &el.attributes,
        _ => return false,
    };
    attrs.iter().any(|a| match a {
        Attribute::StringAttribute(sa) => {
            sa.name == SLOT_ATTRIBUTE && sa.value(source) == "default"
        }
        _ => false,
    })
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
