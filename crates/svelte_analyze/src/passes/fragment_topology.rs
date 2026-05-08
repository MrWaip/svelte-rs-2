use smallvec::SmallVec;

use svelte_ast::{AstStore, Component, FragmentRole, Node, NodeId};

use crate::types::data::AnalysisData;

pub(crate) fn build(component: &Component, data: &mut AnalysisData) {
    let _ = component;
    if !data.script.blocker_data.has_async {
        return;
    }
    let map = std::mem::take(&mut data.template.expression_tags_by_fragment);
    for (idx, slot) in map.iter().enumerate() {
        let Some(expr_ids) = slot else { continue };
        let mut blockers = SmallVec::<[u32; 2]>::new();
        for &id in expr_ids {
            if let Some(d) = data.expression_data(id) {
                for &blk in d.blockers.iter() {
                    blockers.push(blk);
                }
            }
        }
        if !blockers.is_empty() {
            blockers.sort_unstable();
            blockers.dedup();
            data.template
                .insert_fragment_blockers_by_id(svelte_ast::FragmentId(idx as u32), blockers);
        }
    }
    data.template.expression_tags_by_fragment = map;
}

pub fn fragment_items(store: &AstStore, fragment_id: svelte_ast::FragmentId) -> Vec<NodeId> {
    let fragment = store.fragment(fragment_id);
    let inside_head = fragment.role == FragmentRole::SvelteHeadBody;
    let nodes: &[NodeId] = if fragment.role == FragmentRole::NamedSlot
        && fragment.nodes.len() == 1
        && matches!(store.get(fragment.nodes[0]), Node::SvelteFragmentLegacy(_))
    {
        let Node::SvelteFragmentLegacy(el) = store.get(fragment.nodes[0]) else {
            unreachable!()
        };
        &store.fragment(el.fragment).nodes
    } else {
        &fragment.nodes
    };
    nodes
        .iter()
        .copied()
        .filter(|&id| match store.get(id) {
            Node::SnippetBlock(_) => false,
            Node::ConstTag(_) | Node::DebugTag(_) => false,
            Node::Element(el) if inside_head && el.name == "title" => false,
            _ => true,
        })
        .collect()
}
