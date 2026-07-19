use oxc_ast::ast::{Statement, VariableDeclarator};
use oxc_ast_visit::Visit;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use svelte_ast::{AstStore, Component, Node, NodeId};
use svelte_component_semantics::{ReferenceId, walk_bindings};

use super::RefCollector;
use crate::reactivity_semantics::data::ConstTagCycleFactLegacy;
use crate::scope::SymbolId;
use crate::types::data::{AnalysisData, JsAst};

pub(super) fn build<'a>(component: &Component, parsed: &JsAst<'a>, data: &mut AnalysisData<'a>) {
    if data.script.runes() {
        return;
    }

    let store = &component.store;
    let mut order: Vec<SmallVec<[NodeId; 4]>> = Vec::with_capacity(store.fragments_len() as usize);
    let mut cycle: Option<ConstTagCycleFactLegacy> = None;

    for fragment in store.iter_fragments() {
        let const_tags: SmallVec<[NodeId; 4]> = fragment
            .nodes
            .iter()
            .copied()
            .filter(|&id| matches!(store.get(id), Node::ConstTag(_)))
            .collect();

        if const_tags.len() <= 1 {
            order.push(const_tags);
            continue;
        }

        let mut declared: FxHashMap<SymbolId, usize> = FxHashMap::default();
        let mut first_symbol: Vec<Option<SymbolId>> = vec![None; const_tags.len()];
        for (idx, &node_id) in const_tags.iter().enumerate() {
            let Some(declarator) = first_declarator(store, parsed, node_id) else {
                continue;
            };
            walk_bindings(&declarator.id, |v| {
                declared.insert(v.symbol, idx);
                if first_symbol[idx].is_none() {
                    first_symbol[idx] = Some(v.symbol);
                }
            });
        }

        let mut deps: Vec<SmallVec<[usize; 4]>> = vec![SmallVec::new(); const_tags.len()];
        for (idx, &node_id) in const_tags.iter().enumerate() {
            let Node::ConstTag(tag) = store.get(node_id) else {
                continue;
            };
            let Some(stmt) = parsed.stmt(tag.decl.id()) else {
                continue;
            };
            let mut refs: SmallVec<[ReferenceId; 4]> = SmallVec::new();
            let mut reactive_rune_call = false;
            let mut ignored_impure = false;
            let mut collector = RefCollector {
                refs: &mut refs,
                reactive_rune_call: &mut reactive_rune_call,
                has_impure: &mut ignored_impure,
            };
            collector.visit_statement(stmt);
            for ref_id in refs {
                let Some(sym) = data.scoping.symbol_for_reference(ref_id) else {
                    continue;
                };
                let Some(&dep_idx) = declared.get(&sym) else {
                    continue;
                };
                if dep_idx != idx && !deps[idx].contains(&dep_idx) {
                    deps[idx].push(dep_idx);
                }
            }
        }

        let mut state = vec![0u8; const_tags.len()];
        let mut stack: Vec<usize> = Vec::new();
        let mut sorted: SmallVec<[NodeId; 4]> = SmallVec::new();
        let mut cycle_indices: Option<Vec<usize>> = None;
        for i in 0..const_tags.len() {
            visit(
                i,
                &deps,
                &const_tags,
                &mut state,
                &mut stack,
                &mut sorted,
                &mut cycle_indices,
            );
        }

        if cycle.is_none()
            && let Some(indices) = cycle_indices
        {
            let names = indices
                .iter()
                .filter_map(|&idx| first_symbol[idx])
                .map(|sym| data.scoping.semantics().symbol_name(sym).to_string())
                .collect::<Vec<_>>()
                .join(" → ");
            let at_node = const_tags[indices[0]];
            cycle = Some(ConstTagCycleFactLegacy { names, at_node });
        }

        order.push(sorted);
    }

    data.reactivity.set_const_tag_order_legacy(order, cycle);
}

fn first_declarator<'a, 'p>(
    store: &AstStore,
    parsed: &'p JsAst<'a>,
    node_id: NodeId,
) -> Option<&'p VariableDeclarator<'a>> {
    let Node::ConstTag(tag) = store.get(node_id) else {
        return None;
    };
    let Some(Statement::VariableDeclaration(decl)) = parsed.stmt(tag.decl.id()) else {
        return None;
    };
    decl.declarations.first()
}

fn visit(
    i: usize,
    deps: &[SmallVec<[usize; 4]>],
    const_tags: &[NodeId],
    state: &mut [u8],
    stack: &mut Vec<usize>,
    sorted: &mut SmallVec<[NodeId; 4]>,
    cycle: &mut Option<Vec<usize>>,
) {
    match state[i] {
        2 => return,
        1 => {
            if cycle.is_none() {
                let mut indices = stack.clone();
                indices.push(i);
                *cycle = Some(indices);
            }
            return;
        }
        _ => {}
    }

    state[i] = 1;
    stack.push(i);
    for &dep in &deps[i] {
        visit(dep, deps, const_tags, state, stack, sorted, cycle);
    }
    stack.pop();
    state[i] = 2;
    sorted.push(const_tags[i]);
}
