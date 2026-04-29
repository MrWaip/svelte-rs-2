pub(crate) mod builder;
pub mod data;

pub use builder::build;
pub use data::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
    BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics, EachAsyncKind, EachBlockSemantics,
    EachCollectionKind, EachFlags, EachFlavor, EachIndexKind, EachItemKind, EachKeyKind,
    IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch, IfConditionKind, KeyAsyncKind,
    KeyBlockSemantics, RenderArgLowering, RenderAsyncKind, RenderCalleeShape,
    RenderTagBlockSemantics, SnippetBlockSemantics, SnippetParam,
};

use crate::scope::SymbolId;
use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

#[derive(Debug, Default, Clone)]
pub struct BlockSemanticsStore {
    entries: Vec<BlockSemantics>,
    each_index_sym_to_block: FxHashMap<SymbolId, NodeId>,
}

impl BlockSemanticsStore {
    pub fn new(node_count: u32) -> Self {
        let mut entries = Vec::with_capacity(node_count as usize);
        entries.resize_with(node_count as usize, BlockSemantics::default);
        Self {
            entries,
            each_index_sym_to_block: FxHashMap::default(),
        }
    }

    pub fn get(&self, id: NodeId) -> &BlockSemantics {
        self.entries
            .get(id.0 as usize)
            .unwrap_or(&BlockSemantics::NonSpecial)
    }

    pub(crate) fn set(&mut self, id: NodeId, value: BlockSemantics) {
        let idx = id.0 as usize;
        if idx >= self.entries.len() {
            self.entries.resize_with(idx + 1, BlockSemantics::default);
        }
        self.entries[idx] = value;
    }

    pub(crate) fn record_each_index_sym(&mut self, sym: SymbolId, block: NodeId) {
        self.each_index_sym_to_block.insert(sym, block);
    }

    pub fn block_for_each_index_sym(&self, sym: SymbolId) -> Option<NodeId> {
        self.each_index_sym_to_block.get(&sym).copied()
    }

    pub fn is_each_index_sym(&self, sym: SymbolId) -> bool {
        self.each_index_sym_to_block.contains_key(&sym)
    }

    pub(crate) fn set_snippet_hoistable(&mut self, id: NodeId, value: bool) {
        let idx = id.0 as usize;
        if let Some(BlockSemantics::Snippet(sem)) = self.entries.get_mut(idx) {
            sem.hoistable = value;
        }
    }
}
