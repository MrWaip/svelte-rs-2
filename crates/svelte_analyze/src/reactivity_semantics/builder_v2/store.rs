use oxc_span::GetSpan;
use svelte_component_semantics::ReferenceId;

use super::super::data::{ReferenceFacts, StoreBindingSemantics};
use crate::scope::SymbolId;
use crate::types::data::AnalysisData;

pub(super) fn collect_store_declarations(data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();

    if !data.reactivity.uses_runes() {
        let mut to_declare: Vec<(u32, String)> = data
            .scoping
            .root_unresolved_references()
            .iter()
            .filter_map(|(name, refs)| {
                let s = name.as_str();
                if !s.starts_with('$') || s.len() <= 1 || s.starts_with("$$") {
                    return None;
                }
                let base = &s[1..];
                if data.scoping.find_binding(root, base).is_some() {
                    return None;
                }
                Some((earliest_reference_span_start(data, refs), base.to_string()))
            })
            .collect();
        to_declare.sort_by_key(|(span_start, _)| *span_start);
        for (_, base) in to_declare {
            data.scoping.add_synthetic_binding(root, &base);
        }
    }

    let resolved: Vec<(SymbolId, ReferenceId)> = data.scoping.store_candidate_refs().to_vec();

    let mut unresolved: Vec<(u32, SymbolId, Vec<ReferenceId>)> = data
        .scoping
        .root_unresolved_references()
        .iter()
        .filter_map(|(name, refs)| {
            let sym = unresolved_store_base(name.as_str(), data, root)?;
            Some((earliest_reference_span_start(data, refs), sym, refs.clone()))
        })
        .collect();
    unresolved.sort_by_key(|(span_start, _, _)| *span_start);

    for (sym, ref_id) in resolved {
        record_store(data, sym, ref_id);
    }
    for (_, sym, ref_ids) in unresolved {
        for ref_id in ref_ids {
            record_store(data, sym, ref_id);
        }
    }
}

fn earliest_reference_span_start(data: &AnalysisData, refs: &[ReferenceId]) -> u32 {
    refs.iter()
        .filter_map(|&ref_id| {
            let node_id = data.scoping.get_reference(ref_id).node_id();
            data.scoping.js_kind(node_id).map(|kind| kind.span().start)
        })
        .min()
        .unwrap_or(0)
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

fn rune_name_shadows_store_legacy(name: &str, data: &AnalysisData) -> bool {
    !data.reactivity.uses_runes() && svelte_ast::is_rune_name(name)
}

fn unresolved_store_base(
    name: &str,
    data: &AnalysisData,
    root: oxc_semantic::ScopeId,
) -> Option<SymbolId> {
    if !name.starts_with('$') || name.len() <= 1 || name.starts_with("$$") {
        return None;
    }
    if !rune_name_shadows_store_legacy(name, data) && svelte_ast::is_rune_name(name) {
        return None;
    }
    data.scoping.find_binding(root, &name[1..])
}
