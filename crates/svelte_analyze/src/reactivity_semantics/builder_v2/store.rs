use svelte_component_semantics::ReferenceId;

use super::super::data::{ReferenceFacts, StoreBindingSemantics};
use crate::scope::SymbolId;
use crate::types::data::AnalysisData;

pub(super) fn collect_store_declarations(data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();

    let resolved: Vec<(SymbolId, ReferenceId)> = data.scoping.store_candidate_refs().to_vec();

    let unresolved: Vec<(SymbolId, Vec<ReferenceId>)> = data
        .scoping
        .root_unresolved_references()
        .iter()
        .filter_map(|(name, refs)| {
            let sym = unresolved_store_base(name.as_str(), data, root)?;
            Some((sym, refs.clone()))
        })
        .collect();

    for (sym, ref_id) in resolved {
        record_store(data, sym, ref_id);
    }
    for (sym, ref_ids) in unresolved {
        for ref_id in ref_ids {
            record_store(data, sym, ref_id);
        }
    }
}

fn record_store(data: &mut AnalysisData, sym: SymbolId, ref_id: ReferenceId) {
    if data.reactivity.binding_facts(sym).is_none() {
        data.reactivity
            .record_store_binding(sym, StoreBindingSemantics { base_symbol: sym });
    }
    let reference = data.scoping.get_reference(ref_id);
    let facts = if reference.is_read() && reference.is_write() {
        ReferenceFacts::StoreUpdate { symbol: sym }
    } else if reference.is_write() {
        ReferenceFacts::StoreWrite { symbol: sym }
    } else if reference.is_read() {
        ReferenceFacts::StoreRead { symbol: sym }
    } else {
        return;
    };
    data.reactivity.record_reference_semantics(ref_id, facts);
}

fn unresolved_store_base(
    name: &str,
    data: &AnalysisData,
    root: oxc_semantic::ScopeId,
) -> Option<SymbolId> {
    if !name.starts_with('$')
        || name.len() <= 1
        || name.starts_with("$$")
        || svelte_ast::is_rune_name(name)
    {
        return None;
    }
    data.scoping.find_binding(root, &name[1..])
}
