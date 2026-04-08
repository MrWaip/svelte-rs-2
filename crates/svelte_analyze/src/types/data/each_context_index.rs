use bitflags::bitflags;
use compact_str::CompactString;
use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

use super::NodeTable;
use crate::scope::SymbolId;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct EachBlockFlags: u8 {
        const KEY_USES_INDEX = 1 << 0;
        const IS_DESTRUCTURED = 1 << 1;
        const BODY_USES_INDEX = 1 << 2;
        const KEY_IS_ITEM = 1 << 3;
        const HAS_ANIMATE = 1 << 4;
        const NEEDS_COLLECTION_ID = 1 << 5;
        const CONTAINS_GROUP_BINDING = 1 << 6;
    }
}

#[derive(Clone, Default)]
pub struct EachBlockEntry {
    flags: EachBlockFlags,
    context_name: Option<CompactString>,
    index_sym: Option<SymbolId>,
    key_node_id: Option<NodeId>,
}

pub struct EachContextIndex {
    entries: NodeTable<EachBlockEntry>,
    index_sym_to_block: FxHashMap<SymbolId, NodeId>,
}

impl EachContextIndex {
    pub fn new(node_count: u32) -> Self {
        Self {
            entries: NodeTable::new(node_count),
            index_sym_to_block: FxHashMap::default(),
        }
    }

    pub fn record_context_name(
        &mut self,
        block_id: NodeId,
        context_name: impl Into<CompactString>,
    ) {
        let context_name = context_name.into();
        if context_name != "$$item" {
            self.entries.get_or_default(block_id).context_name = Some(context_name);
        }
    }

    pub fn record_index_sym(&mut self, block_id: NodeId, sym: SymbolId) {
        self.index_sym_to_block.insert(sym, block_id);
        self.entries.get_or_default(block_id).index_sym = Some(sym);
    }

    pub fn record_key_node_id(&mut self, block_id: NodeId, node_id: NodeId) {
        self.entries.get_or_default(block_id).key_node_id = Some(node_id);
    }

    pub fn mark_key_uses_index(&mut self, block_id: NodeId) {
        self.entries
            .get_or_default(block_id)
            .flags
            .insert(EachBlockFlags::KEY_USES_INDEX);
    }

    pub fn mark_destructured(&mut self, block_id: NodeId) {
        self.entries
            .get_or_default(block_id)
            .flags
            .insert(EachBlockFlags::IS_DESTRUCTURED);
    }

    pub fn mark_body_uses_index(&mut self, block_id: NodeId) {
        self.entries
            .get_or_default(block_id)
            .flags
            .insert(EachBlockFlags::BODY_USES_INDEX);
    }

    pub fn mark_key_is_item(&mut self, block_id: NodeId) {
        self.entries
            .get_or_default(block_id)
            .flags
            .insert(EachBlockFlags::KEY_IS_ITEM);
    }

    pub fn mark_has_animate(&mut self, block_id: NodeId) {
        self.entries
            .get_or_default(block_id)
            .flags
            .insert(EachBlockFlags::HAS_ANIMATE);
    }

    pub fn index_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.entries.get(id).and_then(|entry| entry.index_sym)
    }

    pub fn mark_contains_group_binding(&mut self, block_id: NodeId) {
        self.entries
            .get_or_default(block_id)
            .flags
            .insert(EachBlockFlags::CONTAINS_GROUP_BINDING);
    }

    pub fn block_for_index_sym(&self, sym: SymbolId) -> Option<NodeId> {
        self.index_sym_to_block.get(&sym).copied()
    }

    pub fn key_node_id(&self, id: NodeId) -> Option<NodeId> {
        self.entries.get(id).and_then(|entry| entry.key_node_id)
    }

    pub fn key_uses_index(&self, id: NodeId) -> bool {
        self.entries
            .get(id)
            .is_some_and(|entry| entry.flags.contains(EachBlockFlags::KEY_USES_INDEX))
    }

    pub fn is_destructured(&self, id: NodeId) -> bool {
        self.entries
            .get(id)
            .is_some_and(|entry| entry.flags.contains(EachBlockFlags::IS_DESTRUCTURED))
    }

    pub fn body_uses_index(&self, id: NodeId) -> bool {
        self.entries
            .get(id)
            .is_some_and(|entry| entry.flags.contains(EachBlockFlags::BODY_USES_INDEX))
    }

    pub fn key_is_item(&self, id: NodeId) -> bool {
        self.entries
            .get(id)
            .is_some_and(|entry| entry.flags.contains(EachBlockFlags::KEY_IS_ITEM))
    }

    pub fn has_animate(&self, id: NodeId) -> bool {
        self.entries
            .get(id)
            .is_some_and(|entry| entry.flags.contains(EachBlockFlags::HAS_ANIMATE))
    }

    pub fn context_name(&self, id: NodeId) -> &str {
        self.entries
            .get(id)
            .and_then(|entry| entry.context_name.as_deref())
            .unwrap_or("$$item")
    }
    pub fn mark_needs_collection_id(&mut self, block_id: NodeId) {
        self.entries
            .get_or_default(block_id)
            .flags
            .insert(EachBlockFlags::NEEDS_COLLECTION_ID);
    }

    pub fn needs_collection_id(&self, id: NodeId) -> bool {
        self.entries
            .get(id)
            .is_some_and(|entry| entry.flags.contains(EachBlockFlags::NEEDS_COLLECTION_ID))
    }

    pub fn contains_group_binding(&self, id: NodeId) -> bool {
        self.entries
            .get(id)
            .is_some_and(|entry| entry.flags.contains(EachBlockFlags::CONTAINS_GROUP_BINDING))
    }
}
