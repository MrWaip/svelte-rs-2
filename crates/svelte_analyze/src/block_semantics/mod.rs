pub(crate) mod builder;
pub mod data;

pub use builder::build;
pub use data::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
    BlockSemantics, ConstTagBlockSemantics, DeclarationTagBlockSemantics, EachAsyncKind,
    EachBlockSemantics, EachCollection, EachCollectionSource, EachFlags, EachFlavor, EachIndexKind,
    EachItemKind, EachKeyKind, FragmentDeclarationAsyncKind, HtmlTagAsyncKind, HtmlTagNamespace,
    HtmlTagSemantics, IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch, IfConditionKind,
    KeyAsyncKind, KeyBlockSemantics, RenderArgKind, RenderAsyncKind, RenderCallKind,
    RenderTagBlockSemantics, SnippetBlockSemantics, SnippetParam, SnippetPlacement, SnippetSlotKey,
};

use crate::scope::SymbolId;
use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

#[derive(Debug, Default, Clone)]
pub struct BlockSemanticsStore {
    entries: FxHashMap<u32, BlockSemantics>,
    each_index_sym_to_block: FxHashMap<SymbolId, NodeId>,
}

impl BlockSemanticsStore {
    pub(crate) fn new(node_count: u32) -> Self {
        let cap = node_count as usize / 8;
        Self {
            entries: FxHashMap::with_capacity_and_hasher(cap, Default::default()),
            each_index_sym_to_block: FxHashMap::default(),
        }
    }

    pub fn get(&self, id: NodeId) -> &BlockSemantics {
        self.entries
            .get(&id.0)
            .unwrap_or(&BlockSemantics::NonSpecial)
    }

    pub(crate) fn set(&mut self, id: NodeId, value: BlockSemantics) {
        self.entries.insert(id.0, value);
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

    pub(crate) fn set_snippet_placement(&mut self, id: NodeId, placement: SnippetPlacement) {
        if let Some(BlockSemantics::Snippet(sem)) = self.entries.get_mut(&id.0) {
            sem.placement = placement;
        }
    }
}
