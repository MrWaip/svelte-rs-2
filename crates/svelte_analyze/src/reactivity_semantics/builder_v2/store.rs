use oxc_span::GetSpan;
use rustc_hash::FxHashMap;
use svelte_component_semantics::ReferenceId;

use super::super::data::{BindingSemantics, ReferenceFacts, StoreBindingSemantics};
use crate::scope::SymbolId;
use crate::types::data::AnalysisData;

pub(super) fn collect_store_declarations(data: &mut AnalysisData) {
    if data.script.is_standalone_module {
        return;
    }
    let root = data.scoping.root_scope_id();

    let mut candidates: Vec<(u32, String, Vec<ReferenceId>)> = Vec::new();

    let mut grouped: FxHashMap<String, (u32, Vec<ReferenceId>)> = FxHashMap::default();
    for (sym, ref_id) in data.scoping.store_candidate_refs().to_vec() {
        let base_name = data.scoping.symbol_name(sym);
        let dollar_name = format!("${}", base_name);
        let span_start = reference_span_start(data, ref_id);
        let entry = grouped
            .entry(dollar_name)
            .or_insert((span_start, Vec::new()));
        entry.0 = entry.0.min(span_start);
        entry.1.push(ref_id);
    }
    for (dollar_name, (span, refs)) in grouped {
        candidates.push((span, dollar_name, refs));
    }

    let unresolved_dollar_refs: Vec<(String, u32, Vec<ReferenceId>)> = data
        .scoping
        .root_unresolved_references()
        .iter()
        .filter_map(|(name, refs)| {
            let s = name.as_str();
            if !s.starts_with('$') || s.len() <= 1 || s.starts_with("$$") {
                return None;
            }
            if is_literal_rune_reference(data, s) {
                return None;
            }
            Some((s.to_string(), earliest_span(data, refs), refs.clone()))
        })
        .collect();
    for (dollar_name, span, refs) in unresolved_dollar_refs {
        candidates.push((span, dollar_name, refs));
    }

    candidates.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let mut seen: FxHashMap<String, (SymbolId, SymbolId)> = FxHashMap::default();

    for (_, dollar_name, refs) in candidates {
        let base_name = &dollar_name[1..];

        let (_base_sym, store_sym) = if let Some(pair) = seen.get(&dollar_name) {
            *pair
        } else {
            let base_sym = match data.scoping.find_binding(root, base_name) {
                Some(s) => s,
                None => {
                    if data.scoping.find_binding_in_any_scope(base_name).is_some() {
                        continue;
                    }
                    data.scoping.add_synthetic_binding(root, base_name)
                }
            };
            let store_sym = match data.scoping.find_binding(root, &dollar_name) {
                Some(s) => s,
                None => data.scoping.add_synthetic_binding(root, &dollar_name),
            };
            if data.reactivity.binding_facts(store_sym).is_none() {
                data.reactivity.record_store_binding(
                    store_sym,
                    StoreBindingSemantics {
                        base_symbol: base_sym,
                        store_symbol: store_sym,
                    },
                );
            }
            seen.insert(dollar_name.clone(), (base_sym, store_sym));
            (base_sym, store_sym)
        };

        for ref_id in refs {
            record_store_reference(data, store_sym, ref_id);
        }
    }
}

fn is_literal_rune_reference(data: &AnalysisData, dollar_name: &str) -> bool {
    if !data.reactivity.uses_runes() {
        return false;
    }
    if !svelte_ast::is_rune_name(dollar_name) {
        return false;
    }
    if base_is_svelte_store_rune_import(data, dollar_name) {
        return true;
    }
    if rune_named_base_is_store(data, &dollar_name[1..]) {
        return false;
    }
    true
}

fn base_is_svelte_store_rune_import(data: &AnalysisData, dollar_name: &str) -> bool {
    let Some(rune_import) = data.reactivity.svelte_store_rune_import() else {
        return false;
    };
    let root = data.scoping.root_scope_id();
    data.scoping.find_binding(root, &dollar_name[1..]) == Some(rune_import)
}

fn rune_named_base_is_store(data: &AnalysisData, base_name: &str) -> bool {
    let root = data.scoping.root_scope_id();
    let Some(base_sym) = data.scoping.find_binding(root, base_name) else {
        return false;
    };
    match data.reactivity.binding_semantics(base_sym) {
        BindingSemantics::Prop(_) | BindingSemantics::LegacyBindableProp(_) => base_name != "props",
        BindingSemantics::NonReactive
        | BindingSemantics::LegacyPropsObject
        | BindingSemantics::MaybeReactive => true,
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Unresolved => false,
    }
}

fn earliest_span(data: &AnalysisData, refs: &[ReferenceId]) -> u32 {
    refs.iter()
        .map(|&ref_id| reference_span_start(data, ref_id))
        .min()
        .unwrap_or(0)
}

fn reference_span_start(data: &AnalysisData, ref_id: ReferenceId) -> u32 {
    let node_id = data.scoping.get_reference(ref_id).node_id();
    data.scoping
        .js_kind(node_id)
        .map(|kind| kind.span().start)
        .unwrap_or(0)
}

fn record_store_reference(data: &mut AnalysisData, store_sym: SymbolId, ref_id: ReferenceId) {
    let reference = data.scoping.get_reference(ref_id);
    let facts = if reference.is_read() && reference.is_write() {
        ReferenceFacts::StoreUpdate { symbol: store_sym }
    } else if reference.is_write() {
        ReferenceFacts::StoreWrite { symbol: store_sym }
    } else if reference.is_read() {
        ReferenceFacts::StoreRead { symbol: store_sym }
    } else {
        return;
    };
    data.reactivity.record_reference_semantics(ref_id, facts);
}
