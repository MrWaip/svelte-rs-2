//! Populate `template.const_tags.syms` — the leaf-SymbolId list per
//! `{@const}` tag. Used by codegen (`svelte_codegen_client`) to wire up
//! const-tag bindings without re-resolving names at emission time.
//!
//! Reactive meaning for the bindings themselves lives in `ReactivitySemantics`
//! (see `reactivity_semantics::builder_v2::compute_const_tag_reactivity`).
//! This pass only builds the tag→syms index; it does not touch declaration
//! semantics.

use crate::types::data::AnalysisData;

pub(crate) fn run(data: &mut AnalysisData) {
    let pairs: Vec<_> = data
        .template
        .const_tags
        .by_fragment
        .iter()
        .filter_map(|(frag_key, tag_ids)| {
            let scope = data.scoping.fragment_scope(frag_key)?;
            Some((scope, tag_ids.clone()))
        })
        .collect();
    for (scope, tag_ids) in pairs {
        for tag_id in tag_ids {
            let Some(names) = data.template.const_tags.names(tag_id).cloned() else {
                continue;
            };
            let mut syms = Vec::new();
            for name in &names {
                if let Some(sym_id) = data.scoping.find_binding(scope, name) {
                    syms.push(sym_id);
                }
            }
            if !syms.is_empty() {
                data.template.const_tags.syms.insert(tag_id, syms);
            }
        }
    }
}
