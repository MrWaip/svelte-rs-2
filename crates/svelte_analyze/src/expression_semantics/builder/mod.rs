mod collector;
mod derive;
mod walker;

use super::ExpressionSemanticsStore;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::ComponentScoping;
use crate::types::data::{BlockerData, JsAst};
use svelte_ast::Component;
use svelte_component_semantics::ComponentSemantics;

pub fn build<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    scoping: &ComponentScoping,
    has_class_state_fields: bool,
    blockers: &BlockerData,
    node_count: u32,
) -> ExpressionSemanticsStore {
    let mut store = ExpressionSemanticsStore::new(node_count);
    walker::populate(
        component,
        parsed,
        semantics,
        reactivity,
        scoping,
        has_class_state_fields,
        blockers,
        &mut store,
    );
    store
}
