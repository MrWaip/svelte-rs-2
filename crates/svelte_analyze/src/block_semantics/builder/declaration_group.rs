use smallvec::SmallVec;
use svelte_ast::{Component, FragmentId, Node, NodeId};

use super::super::{
    BlockSemantics, BlockSemanticsStore, ConstTagBlockSemantics, DeclarationTagBlockSemantics,
    FragmentDeclarationAsyncKind,
};

pub(super) fn populate(component: &Component, store: &mut BlockSemanticsStore) {
    for index in 0..component.store.fragments_len() {
        populate_fragment(component, FragmentId(index), store);
    }
}

fn populate_fragment(component: &Component, fragment: FragmentId, store: &mut BlockSemanticsStore) {
    let mut group_open = false;
    for &node_id in component.store.fragment_nodes(fragment) {
        if !matches!(
            component.store.get(node_id),
            Node::ConstTag(_) | Node::DeclarationTag(_)
        ) {
            continue;
        }
        let Some(async_kind) = declaration_async_kind(store, node_id) else {
            continue;
        };
        if async_kind.is_async() {
            group_open = true;
            continue;
        }
        if !group_open {
            continue;
        }
        defer_declaration(store, node_id);
    }
}

fn declaration_async_kind(
    store: &BlockSemanticsStore,
    node_id: NodeId,
) -> Option<FragmentDeclarationAsyncKind> {
    match store.get(node_id) {
        BlockSemantics::ConstTag(sem) => Some(sem.async_kind.clone()),
        BlockSemantics::DeclarationTag(sem) => Some(sem.async_kind.clone()),
        _ => None,
    }
}

fn defer_declaration(store: &mut BlockSemanticsStore, node_id: NodeId) {
    let deferred = FragmentDeclarationAsyncKind::Deferred {
        blockers: SmallVec::new(),
    };
    let updated = match store.get(node_id) {
        BlockSemantics::ConstTag(sem) => BlockSemantics::ConstTag(ConstTagBlockSemantics {
            async_kind: deferred,
            ..sem.clone()
        }),
        BlockSemantics::DeclarationTag(_) => {
            BlockSemantics::DeclarationTag(DeclarationTagBlockSemantics {
                async_kind: deferred,
            })
        }
        _ => return,
    };
    store.set(node_id, updated);
}
