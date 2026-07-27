pub(crate) mod builder;
pub mod data;

pub use builder::build;
pub use data::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
    BlockSemantics, ConstTagBlockSemantics, DeclarationTagBlockSemantics, EachAsyncKind,
    EachBlockSemantics, EachCollection, EachCollectionSource, EachFlags, EachFlavor, EachIndexKind,
    EachItemKind, EachKeyKind, ExpressionBlocker, FragmentDeclarationAsyncKind, HtmlTagAsyncKind,
    HtmlTagNamespace, HtmlTagSemantics, IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch,
    IfConditionKind, KeyAsyncKind, KeyBlockSemantics, RenderArgKind, RenderAsyncKind,
    RenderCallKind, RenderTagBlockSemantics, SnippetBlockSemantics, SnippetParam, SnippetPlacement,
    SnippetSlotKey,
};

use crate::scope::SymbolId;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use svelte_ast::NodeId;

#[derive(Debug, Default, Clone)]
pub struct BlockSemanticsStore {
    entries: FxHashMap<u32, BlockSemantics>,
    each_index_sym_to_block: FxHashMap<SymbolId, NodeId>,
    fragment_declaration_groups: FxHashMap<u32, SmallVec<[NodeId; 2]>>,
    fragment_declaration_group_order: Vec<svelte_ast::FragmentId>,
}

impl BlockSemanticsStore {
    pub(crate) fn new(node_count: u32) -> Self {
        let cap = node_count as usize / 8;
        Self {
            entries: FxHashMap::with_capacity_and_hasher(cap, Default::default()),
            each_index_sym_to_block: FxHashMap::default(),
            fragment_declaration_groups: FxHashMap::default(),
            fragment_declaration_group_order: Vec::new(),
        }
    }

    pub fn fragment_declaration_group(&self, id: svelte_ast::FragmentId) -> &[NodeId] {
        self.fragment_declaration_groups
            .get(&id.0)
            .map_or(&[], SmallVec::as_slice)
    }

    pub fn fragment_declaration_group_order(&self) -> &[svelte_ast::FragmentId] {
        &self.fragment_declaration_group_order
    }

    pub(crate) fn open_fragment_declaration_group(&mut self, id: svelte_ast::FragmentId) {
        self.fragment_declaration_group_order.push(id);
    }

    pub(crate) fn set_fragment_declaration_group(
        &mut self,
        id: svelte_ast::FragmentId,
        members: SmallVec<[NodeId; 2]>,
    ) {
        self.fragment_declaration_groups.insert(id.0, members);
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
