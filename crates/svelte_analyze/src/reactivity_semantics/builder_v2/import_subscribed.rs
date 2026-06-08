use super::super::data::ReferenceFacts;
use crate::types::data::AnalysisData;

pub(super) fn classify_import_subscribed_reads(data: &mut AnalysisData<'_>) {
    let store_pairs: Vec<(
        svelte_component_semantics::SymbolId,
        svelte_component_semantics::SymbolId,
    )> = data
        .reactivity
        .iter_store_bindings()
        .filter_map(|(_, store)| {
            data.scoping
                .is_import(store.base_symbol)
                .then_some((store.base_symbol, store.store_symbol))
        })
        .collect();

    for (base_sym, store_symbol) in store_pairs {
        let ref_ids: Vec<_> = data.scoping.get_resolved_reference_ids(base_sym).to_vec();
        for ref_id in ref_ids {
            if data.reactivity.reference_facts(ref_id).is_some() {
                continue;
            }
            let reference = data.scoping.get_reference(ref_id);
            if !reference.is_read() {
                continue;
            }
            data.reactivity.record_reference_semantics(
                ref_id,
                ReferenceFacts::ImportSubscribedRead { store_symbol },
            );
        }
    }
}
