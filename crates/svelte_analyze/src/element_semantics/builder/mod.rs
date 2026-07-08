mod async_kind;
mod boundary;
mod legacy_slot;

use svelte_ast::{Component, ComponentLikeView, Node};

use super::{
    ElementSemantics, ElementSemanticsStore, RegularElementSemantics, SvelteElementSemantics,
};
use crate::expression_semantics::ExpressionSemanticsStore;
use crate::types::data::JsAst;

pub(crate) fn build(
    component: &Component,
    parsed: &JsAst<'_>,
    expressions: &ExpressionSemanticsStore,
    source: &str,
    node_count: u32,
) -> ElementSemanticsStore {
    let mut store = ElementSemanticsStore::new(node_count);
    for fragment in component.store.iter_fragments() {
        for &node_id in fragment.nodes.iter() {
            let node = component.store.get(node_id);
            match node {
                Node::SvelteBoundary(el) => {
                    let boundary = boundary::classify(component, parsed, el);
                    store.set(el.id, ElementSemantics::Boundary(boundary));
                }
                Node::Element(el) => {
                    if let Some(async_kind) =
                        async_kind::from_attributes(expressions, &el.attributes)
                    {
                        store.set(
                            el.id,
                            ElementSemantics::RegularElement(RegularElementSemantics {
                                async_kind,
                            }),
                        );
                    }
                }
                Node::SvelteElement(el) => {
                    if let Some(async_kind) = async_kind::from_tag(expressions, el.id) {
                        store.set(
                            el.id,
                            ElementSemantics::SvelteElement(SvelteElementSemantics { async_kind }),
                        );
                    }
                }
                Node::SlotElementLegacy(el) => {
                    let slot = legacy_slot::classify_slot(el, component, source);
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
                        let slots = legacy_slot::classify_component_slots(
                            attributes, fragment, component, source,
                        );
                        store.set(id, ElementSemantics::LegacyComponentSlots(slots));
                    }
                }
            }
        }
    }
    store
}
