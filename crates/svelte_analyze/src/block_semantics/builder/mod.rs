mod await_;
mod common;
mod const_tag;
mod each;
mod html_tag;
mod if_;
mod key;
mod render;
mod snippet;
mod walker;

use super::BlockSemanticsStore;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::types::data::{BlockerData, FragmentNamespaces, IgnoreData, JsAst};
use svelte_ast::Component;
use svelte_component_semantics::ComponentSemantics;

pub fn build(
    component: &Component,
    parsed: &JsAst<'_>,
    semantics: &ComponentSemantics<'_>,
    reactivity: &ReactivitySemantics,
    blockers: &BlockerData,
    fragment_namespaces: &FragmentNamespaces,
    ignore_data: &IgnoreData,
    dev: bool,
    node_count: u32,
) -> BlockSemanticsStore {
    let mut store = BlockSemanticsStore::new(node_count);
    walker::populate(
        component,
        parsed,
        semantics,
        reactivity,
        blockers,
        fragment_namespaces,
        ignore_data,
        dev,
        &mut store,
    );
    store
}
