//! Block Semantics cluster — single source of truth for the meaning of
//! template control-flow blocks (`{#each}`, `{#if}`, `{#await}`, `{#key}`,
//! `{#snippet}`, `{@const}`, `{@render}`). See SEMANTIC_LAYER_ARCHITECTURE.md.
//!
//! Consumers query one block `NodeId` and receive one [`BlockSemantics`]
//! variant. Out-of-bounds and non-block nodes return [`BlockSemantics::NonSpecial`] —
//! the store never leaks `None`.

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

/// Dense `NodeId`-indexed storage with a total API. Missing slots normalize
/// to [`BlockSemantics::NonSpecial`] — there is no `Option` on the public
/// surface.
///
/// A side index `index_sym -> each block NodeId` supports reverse lookup
/// (`is_each_index_sym`, `block_for_index_sym`) without scanning the main
/// vector.
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

    /// Total API — returns `&NonSpecial` for any id outside the populated range.
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

    /// Post-populate mutator for the snippet `hoistable` flag. Kept
    /// separate from `set` because hoistable is classified in a second
    /// pass after all snippet references have been collected: the
    /// populator seeds `hoistable: false` and the finalize stage flips
    /// it for top-level snippets whose body never references an
    /// instance-scope symbol.
    pub(crate) fn set_snippet_hoistable(&mut self, id: NodeId, value: bool) {
        let idx = id.0 as usize;
        if let Some(BlockSemantics::Snippet(sem)) = self.entries.get_mut(idx) {
            sem.hoistable = value;
        }
    }
}
