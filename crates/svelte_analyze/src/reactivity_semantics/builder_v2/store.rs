//! Cluster C3: `$store` subscription detection.
//!
//! Identifies identifier references named `$foo` whose base name (`foo`)
//! resolves to a root-scope binding. The base symbol is recorded as a store
//! declaration and every resolved reference is classified as a store
//! read/write/update.

use svelte_component_semantics::ReferenceId;

use super::super::data::{StoreDeclarationSemantics, V2ReferenceFacts};
use crate::scope::SymbolId;
use crate::types::data::AnalysisData;
use crate::utils::script_info::is_rune_name;

pub(super) fn collect_store_declarations(data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();
    let entries: Vec<(SymbolId, Vec<ReferenceId>)> = data
        .scoping
        .root_unresolved_references()
        .iter()
        .filter_map(|(name, refs)| {
            let sym = store_candidate_symbol(data, root, name.as_str())?;
            Some((sym, refs.clone()))
        })
        .collect();

    for (sym, ref_ids) in entries {
        let node_id = data.scoping.symbol_declaration(sym);
        if data.reactivity.declaration_facts_v2(node_id).is_none() {
            data.reactivity.record_symbol_declaration_root(sym, node_id);
            data.reactivity.record_store_declaration_v2(
                node_id,
                StoreDeclarationSemantics { base_symbol: sym },
            );
        }
        for ref_id in ref_ids {
            let reference = data.scoping.get_reference(ref_id);
            let is_read = reference.is_read();
            let is_write = reference.is_write();
            let facts = if is_read && is_write {
                V2ReferenceFacts::StoreUpdate { symbol: sym }
            } else if is_write {
                V2ReferenceFacts::StoreWrite { symbol: sym }
            } else if is_read {
                V2ReferenceFacts::StoreRead { symbol: sym }
            } else {
                continue;
            };
            data.reactivity.record_reference_semantics_v2(ref_id, facts);
        }
    }
}

fn store_candidate_symbol(
    data: &AnalysisData,
    root: oxc_semantic::ScopeId,
    unresolved_name: &str,
) -> Option<SymbolId> {
    let store_name = store_base_name(unresolved_name)?;
    data.scoping.find_binding(root, store_name)
}

fn store_base_name(unresolved_name: &str) -> Option<&str> {
    (unresolved_name.starts_with('$')
        && unresolved_name.len() > 1
        && !unresolved_name.starts_with("$$")
        && !is_rune_name(unresolved_name))
    .then_some(&unresolved_name[1..])
}
