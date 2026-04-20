use bitflags::bitflags;
use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

use super::NodeTable;
use crate::scope::SymbolId;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct EachBlockFlags: u8 {
        const IS_DESTRUCTURED = 1 << 1;
    }
}

#[derive(Clone, Default)]
pub struct EachBlockEntry {
    flags: EachBlockFlags,
    index_sym: Option<SymbolId>,
    key_node_id: Option<NodeId>,
}

#[deprecated(
    note = "use AnalysisData::block_semantics(id) -> BlockSemantics::Each(...) \
            from crates/svelte_analyze/src/block_semantics/. This legacy side \
            table will be removed once all consumers have been migrated."
)]
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

    pub fn record_index_sym(&mut self, block_id: NodeId, sym: SymbolId) {
        self.index_sym_to_block.insert(sym, block_id);
        self.entries.get_or_default(block_id).index_sym = Some(sym);
    }

    pub fn record_key_node_id(&mut self, block_id: NodeId, node_id: NodeId) {
        self.entries.get_or_default(block_id).key_node_id = Some(node_id);
    }

    pub fn mark_destructured(&mut self, block_id: NodeId) {
        self.entries
            .get_or_default(block_id)
            .flags
            .insert(EachBlockFlags::IS_DESTRUCTURED);
    }

    pub fn index_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.entries.get(id).and_then(|entry| entry.index_sym)
    }

    pub fn block_for_index_sym(&self, sym: SymbolId) -> Option<NodeId> {
        self.index_sym_to_block.get(&sym).copied()
    }

    pub fn key_node_id(&self, id: NodeId) -> Option<NodeId> {
        self.entries.get(id).and_then(|entry| entry.key_node_id)
    }

    pub fn is_destructured(&self, id: NodeId) -> bool {
        self.entries
            .get(id)
            .is_some_and(|entry| entry.flags.contains(EachBlockFlags::IS_DESTRUCTURED))
    }
}
