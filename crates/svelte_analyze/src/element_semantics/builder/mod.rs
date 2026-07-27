mod async_kind;
mod boundary;
mod legacy_slot;
mod value_role;

use svelte_ast::{
    AstStore, Attribute, Component, ComponentLikeView, Element, FragmentId, FragmentRole, Node,
    NodeId,
};

use smallvec::SmallVec;

use super::{
    ComponentElementSemantics, ElementPropertyReset, ElementReplayEvent, ElementSemantics,
    ElementSemanticsStore, ElementValueRole, RegularElementSemantics, SvelteElementSemantics,
};
use compact_str::CompactString;

use crate::types::data::{AnalysisData, JsAst, NamespaceKind};

pub(crate) fn build(
    component: &Component,
    parsed: &JsAst<'_>,
    data: &AnalysisData,
    source: &str,
    node_count: u32,
) -> ElementSemanticsStore {
    let expressions = &data.expressions_v2;
    let mut store = ElementSemanticsStore::new(node_count);
    for fragment in component.store.iter_fragments() {
        for &node_id in fragment.nodes.iter() {
            let node = component.store.get(node_id);
            match node {
                Node::SvelteBoundary(el) => {
                    let boundary = boundary::classify(component, parsed, data, el);
                    store.set(el.id, ElementSemantics::Boundary(boundary));
                }
                Node::Element(el)
                    if el.name == "title" && title_inside_head(&component.store, fragment.id) =>
                {
                    store.set(el.id, ElementSemantics::HeadTitle);
                }
                Node::Element(el) => {
                    let name = match data.namespace(el.id).unwrap_or(NamespaceKind::Html) {
                        NamespaceKind::Html => CompactString::new(el.name.to_ascii_lowercase()),
                        NamespaceKind::Svg
                        | NamespaceKind::MathMl
                        | NamespaceKind::ForeignObject
                        | NamespaceKind::AnnotationXml => CompactString::new(el.name.as_str()),
                    };
                    let value_role = value_role::classify(component, data, el);
                    let mut value_nodes: SmallVec<[NodeId; 2]> = SmallVec::new();
                    match &value_role {
                        ElementValueRole::Option { value, .. } => value_nodes.extend(*value),
                        ElementValueRole::TextareaValue { .. } => {
                            value_nodes = value_role::content_value_nodes(component, data, el);
                        }
                        ElementValueRole::Plain
                        | ElementValueRole::Select { .. }
                        | ElementValueRole::ContentEditable { .. }
                        | ElementValueRole::RawText
                        | ElementValueRole::RichContainer => {}
                    }
                    let async_kind = async_kind::from_attributes(
                        expressions,
                        data.blocker_data(),
                        &async_kind::HtmlAttributeSite {
                            attributes: &data.attributes,
                            element_name: &name,
                            siblings: &el.attributes,
                            source,
                            value_nodes,
                        },
                    );
                    let replay_events = replay_events(data, el);
                    let opaque_content =
                        data.elements.flags.is_customizable_select(el.id) || name == "noscript";
                    let property_reset = element_property_reset(el);
                    let is_script = name == "script";
                    store.set(
                        el.id,
                        ElementSemantics::RegularElement(RegularElementSemantics {
                            name,
                            async_kind,
                            value_role,
                            replay_events,
                            opaque_content,
                            property_reset,
                            is_script,
                        }),
                    );
                }
                Node::SvelteElement(el) => {
                    let tag_async_kind = async_kind::from_tag(expressions, el.id);
                    let attributes_async_kind = async_kind::from_svelte_element_attributes(
                        expressions,
                        data.blocker_data(),
                        &async_kind::HtmlAttributeSite {
                            attributes: &data.attributes,
                            element_name: "svelte:element",
                            siblings: &el.attributes,
                            source,
                            value_nodes: SmallVec::new(),
                        },
                    );
                    store.set(
                        el.id,
                        ElementSemantics::SvelteElement(SvelteElementSemantics {
                            tag_async_kind,
                            attributes_async_kind,
                        }),
                    );
                }
                Node::SlotElementLegacy(el) => {
                    let async_kind =
                        async_kind::from_slot(expressions, data.blocker_data(), &el.attributes);
                    let slot = legacy_slot::classify_slot(el, component, source, async_kind);
                    store.set(el.id, ElementSemantics::LegacySlot(slot));
                }
                _ => {
                    if let Some(view) = node.as_component_like() {
                        let ComponentLikeView {
                            id,
                            attributes,
                            fragment,
                            ..
                        } = view;
                        let name_id = match node {
                            Node::SvelteSelf(_) => None,
                            _ => Some(id),
                        };
                        let async_kind = async_kind::from_component(
                            expressions,
                            data.blocker_data(),
                            attributes,
                            name_id,
                        );
                        let legacy_slots = legacy_slot::classify_component_slots(
                            attributes, fragment, component, source,
                        );
                        store.set(
                            id,
                            ElementSemantics::Component(ComponentElementSemantics {
                                async_kind,
                                legacy_slots,
                            }),
                        );
                    }
                }
            }
        }
    }
    store
}

fn element_property_reset(el: &Element) -> ElementPropertyReset {
    let has_attribute = |name: &str| el.attributes.iter().any(|a| a.name() == Some(name));
    if has_attribute("dir") {
        return ElementPropertyReset::Dir;
    }
    if el.name == "img" && has_attribute("loading") {
        return ElementPropertyReset::LazyLoadingImg;
    }
    ElementPropertyReset::None
}

fn title_inside_head(store: &AstStore, fragment_id: FragmentId) -> bool {
    let mut current = fragment_id;
    loop {
        let fragment = store.fragment(current);
        if fragment.role == FragmentRole::SvelteHeadBody {
            return true;
        }
        let Some(owner) = fragment.owner else {
            return false;
        };
        let Some(parent) = store.node_fragment(owner) else {
            return false;
        };
        current = parent;
    }
}

fn is_load_error_element(name: &str) -> bool {
    matches!(
        name,
        "body" | "embed" | "iframe" | "img" | "link" | "object" | "script" | "style" | "track"
    )
}

fn push_replay_event(events: &mut SmallVec<[ElementReplayEvent; 2]>, event: ElementReplayEvent) {
    if !events.contains(&event) {
        events.push(event);
    }
}

fn replay_events(_data: &AnalysisData, el: &Element) -> SmallVec<[ElementReplayEvent; 2]> {
    let mut events: SmallVec<[ElementReplayEvent; 2]> = SmallVec::new();
    if !is_load_error_element(el.name.as_str()) {
        return events;
    }
    for attr in &el.attributes {
        match attr {
            Attribute::SpreadAttribute(_) | Attribute::UseDirective(_) => {
                push_replay_event(&mut events, ElementReplayEvent::Load);
                push_replay_event(&mut events, ElementReplayEvent::Error);
            }
            _ => match attr.name() {
                Some("onload") => push_replay_event(&mut events, ElementReplayEvent::Load),
                Some("onerror") => push_replay_event(&mut events, ElementReplayEvent::Error),
                _ => {}
            },
        }
    }
    events
}
