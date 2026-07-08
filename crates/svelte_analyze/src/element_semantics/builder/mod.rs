mod async_kind;
mod boundary;
mod legacy_slot;
mod value_role;

use svelte_ast::{Component, ComponentLikeView, Node};

use super::{
    ElementSemantics, ElementSemanticsStore, ElementValueRole, RegularElementSemantics,
    SvelteElementSemantics,
};
use crate::types::data::{AnalysisData, JsAst};

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
                    let boundary = boundary::classify(component, parsed, el);
                    store.set(el.id, ElementSemantics::Boundary(boundary));
                }
                Node::Element(el) => {
                    let async_kind = async_kind::from_attributes(expressions, &el.attributes);
                    let value_role = value_role::classify(data, el);
                    if !async_kind.is_sync() || value_role != ElementValueRole::Plain {
                        store.set(
                            el.id,
                            ElementSemantics::RegularElement(RegularElementSemantics {
                                async_kind,
                                value_role,
                            }),
                        );
                    }
                }
                Node::SvelteElement(el) => {
                    let async_kind = async_kind::from_tag(expressions, el.id);
                    if !async_kind.is_sync() {
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
