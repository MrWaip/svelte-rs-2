mod collector;
mod derive;
mod walker;

use super::ExpressionSemanticsStore;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::ComponentScoping;
use crate::types::data::{BlockerData, JsAst, SnippetData};
use svelte_ast::Component;
use svelte_component_semantics::ComponentSemantics;

#[allow(clippy::too_many_arguments)]
pub fn build<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    scoping: &ComponentScoping,
    snippets: &SnippetData,
    has_class_state_fields: bool,
    blockers: &BlockerData,
    runes_mode: svelte_ast::RunesMode,
    node_count: u32,
    dev: bool,
) -> ExpressionSemanticsStore {
    let mut store = ExpressionSemanticsStore::new(node_count);
    walker::populate(
        component,
        parsed,
        semantics,
        reactivity,
        scoping,
        snippets,
        has_class_state_fields,
        blockers,
        runes_mode,
        &mut store,
        dev,
    );
    store
}
