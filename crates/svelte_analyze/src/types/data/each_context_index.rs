use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

use super::{NodeBitSet, NodeTable};
use crate::scope::SymbolId;

pub struct EachContextIndex {
    context_names: NodeTable<String>,
    index_syms: NodeTable<SymbolId>,
    index_sym_to_block: FxHashMap<SymbolId, NodeId>,
    key_node_ids: NodeTable<NodeId>,
    key_uses_index: NodeBitSet,
    is_destructured: NodeBitSet,
    body_uses_index: NodeBitSet,
    key_is_item: NodeBitSet,
    has_animate: NodeBitSet,
    bind_this_context: NodeTable<Vec<String>>,
    parent_each_blocks: NodeTable<Vec<NodeId>>,
    contains_group_binding: NodeBitSet,
    needs_collection_id: NodeBitSet,
}

impl EachContextIndex {
    pub fn new(node_count: u32) -> Self {
        Self {
            context_names: NodeTable::new(node_count),
            index_syms: NodeTable::new(node_count),
            index_sym_to_block: FxHashMap::default(),
            key_node_ids: NodeTable::new(node_count),
            key_uses_index: NodeBitSet::new(node_count),
            is_destructured: NodeBitSet::new(node_count),
            body_uses_index: NodeBitSet::new(node_count),
            key_is_item: NodeBitSet::new(node_count),
            has_animate: NodeBitSet::new(node_count),
            bind_this_context: NodeTable::new(node_count),
            parent_each_blocks: NodeTable::new(node_count),
            contains_group_binding: NodeBitSet::new(node_count),
            needs_collection_id: NodeBitSet::new(node_count),
        }
    }

    pub fn record_context_name(&mut self, block_id: NodeId, context_name: String) {
        self.context_names.insert(block_id, context_name);
    }

    pub fn record_index_sym(&mut self, block_id: NodeId, sym: SymbolId) {
        self.index_sym_to_block.insert(sym, block_id);
        self.index_syms.insert(block_id, sym);
    }

    pub fn record_key_node_id(&mut self, block_id: NodeId, node_id: NodeId) {
        self.key_node_ids.insert(block_id, node_id);
    }

    pub fn mark_key_uses_index(&mut self, block_id: NodeId) {
        self.key_uses_index.insert(block_id);
    }

    pub fn mark_destructured(&mut self, block_id: NodeId) {
        self.is_destructured.insert(block_id);
    }

    pub fn mark_body_uses_index(&mut self, block_id: NodeId) {
        self.body_uses_index.insert(block_id);
    }

    pub fn mark_key_is_item(&mut self, block_id: NodeId) {
        self.key_is_item.insert(block_id);
    }

    pub fn mark_has_animate(&mut self, block_id: NodeId) {
        self.has_animate.insert(block_id);
    }

    pub fn set_bind_this_context(&mut self, attr_id: NodeId, each_vars: Vec<String>) {
        self.bind_this_context.insert(attr_id, each_vars);
    }

    pub fn set_parent_each_blocks(&mut self, attr_id: NodeId, parent_eaches: Vec<NodeId>) {
        self.parent_each_blocks.insert(attr_id, parent_eaches);
    }

    pub fn mark_contains_group_binding(&mut self, block_id: NodeId) {
        self.contains_group_binding.insert(block_id);
    }

    pub fn index_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.index_syms.get(id).copied()
    }

    pub fn block_for_index_sym(&self, sym: SymbolId) -> Option<NodeId> {
        self.index_sym_to_block.get(&sym).copied()
    }

    pub fn key_node_id(&self, id: NodeId) -> Option<NodeId> {
        self.key_node_ids.get(id).copied()
    }

    pub fn key_uses_index(&self, id: NodeId) -> bool {
        self.key_uses_index.contains(&id)
    }

    pub fn is_destructured(&self, id: NodeId) -> bool {
        self.is_destructured.contains(&id)
    }

    pub fn body_uses_index(&self, id: NodeId) -> bool {
        self.body_uses_index.contains(&id)
    }

    pub fn key_is_item(&self, id: NodeId) -> bool {
        self.key_is_item.contains(&id)
    }

    pub fn has_animate(&self, id: NodeId) -> bool {
        self.has_animate.contains(&id)
    }

    pub fn context_name(&self, id: NodeId) -> &str {
        self.context_names.get(id).map_or("$$item", |s| s.as_str())
    }

    pub fn bind_this_context(&self, id: NodeId) -> Option<&Vec<String>> {
        self.bind_this_context.get(id)
    }

    pub fn parent_each_blocks(&self, id: NodeId) -> Option<&Vec<NodeId>> {
        self.parent_each_blocks.get(id)
    }

    pub fn contains_group_binding(&self, id: NodeId) -> bool {
        self.contains_group_binding.contains(&id)
    }

    pub fn mark_needs_collection_id(&mut self, block_id: NodeId) {
        self.needs_collection_id.insert(block_id);
    }

    pub fn needs_collection_id(&self, id: NodeId) -> bool {
        self.needs_collection_id.contains(&id)
    }
}
