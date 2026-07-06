mod async_kind;
mod boundary;

use svelte_ast::{Component, Node};

use super::{
    ElementSemantics, ElementSemanticsStore, RegularElementSemantics, SvelteElementSemantics,
};
use crate::expression_semantics::ExpressionSemanticsStore;
use crate::types::data::JsAst;

pub(crate) fn build(
    component: &Component,
    parsed: &JsAst<'_>,
    expressions: &ExpressionSemanticsStore,
    node_count: u32,
) -> ElementSemanticsStore {
    let mut store = ElementSemanticsStore::new(node_count);
    for fragment in component.store.iter_fragments() {
        for &node_id in fragment.nodes.iter() {
            match component.store.get(node_id) {
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
                _ => {}
            }
        }
    }
    store
}
