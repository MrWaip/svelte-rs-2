mod collector;
mod derive;
mod walker;

use super::ExpressionSemanticsStore;
use crate::await_semantics::AwaitSemanticsStore;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::ComponentScoping;
use crate::types::data::{BlockerData, JsAst, SnippetData};
use crate::value_evaluation::ValueEvaluation;
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
    value_evaluation: &ValueEvaluation,
    has_class_state_fields: bool,
    observes_context: bool,
    blockers: &BlockerData,
    await_semantics: &AwaitSemanticsStore,
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
        value_evaluation,
        has_class_state_fields,
        blockers,
        await_semantics,
        runes_mode,
        &mut store,
        dev,
    );
    if observes_context {
        store.note_context(super::ContextSignal::SCRIPT_CONTEXT);
    }
    store
}
