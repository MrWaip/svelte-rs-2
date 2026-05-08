mod collector;
mod derive;
mod walker;

use super::ExpressionSemanticsStore;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::ComponentScoping;
use crate::types::data::{BlockerData, ExpressionInfo, JsAst, PickledAwaitOffsets};
use crate::types::node_table::NodeTable;
use svelte_ast::Component;
use svelte_component_semantics::ComponentSemantics;

pub fn build<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    scoping: &ComponentScoping,
    expressions: &NodeTable<ExpressionInfo>,
    has_class_state_fields: bool,
    blockers: &BlockerData,
    pickled: &PickledAwaitOffsets,
    node_count: u32,
) -> ExpressionSemanticsStore {
    let mut store = ExpressionSemanticsStore::new(node_count);
    walker::populate(
        component,
        parsed,
        semantics,
        reactivity,
        scoping,
        expressions,
        has_class_state_fields,
        blockers,
        pickled,
        &mut store,
    );
    store
}
