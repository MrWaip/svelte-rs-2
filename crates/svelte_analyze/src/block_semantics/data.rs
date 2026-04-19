//! Block Semantics — answer shapes for template control-flow blocks.
//!
//! Consumer Model (see SEMANTIC_LAYER_ARCHITECTURE.md): one query per block
//! `NodeId`, returning exactly one variant. Consumers pattern-match at the
//! root dispatch point and never re-inspect the AST to reconstruct meaning.
//!
//! Storage Content Rule: only `NodeId` / `OxcNodeId` / `ReferenceId` /
//! `SymbolId`, enum variants, bools and numeric payloads. Never `String` /
//! `&str` / `CompactString` — names are read from the source via spans at
//! consume-time, not stored on the payload.

use crate::scope::SymbolId;
use bitflags::bitflags;
use smallvec::SmallVec;
use svelte_component_semantics::OxcNodeId;

/// One answer for one block `NodeId`. `NonSpecial` is the neutral value
/// returned for every node that is not a control-flow block; the store
/// never returns `None`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum BlockSemantics {
    /// Node is not a control-flow block in the sense owned by this cluster.
    #[default]
    NonSpecial,
    /// `{#each ... as ...}` block.
    Each(EachBlockSemantics),
    // Placeholders for later slices — payload lands when each kind is
    // migrated end-to-end. Keeping them here shapes the public enum so
    // consumers can already switch on it exhaustively once migrated.
    // TODO(block-semantics): If payload.
    If,
    // TODO(block-semantics): Await payload.
    Await,
    // TODO(block-semantics): Key payload.
    Key,
    // TODO(block-semantics): Snippet payload.
    Snippet,
    // TODO(block-semantics): ConstTag payload.
    ConstTag,
    // TODO(block-semantics): Render payload.
    Render,
}

/// `{#each items as <item>[, <index>] [(<key>)]}` — the identities of the
/// three introducers plus the block's high-level flavor.
///
/// Reactive meaning of `item`/`index` bindings stays in `reactivity_semantics`
/// (queried via the `SymbolId` carried here). The AST node of a destructured
/// pattern or key expression is read by the consumer via
/// `ComponentSemantics.js_storage()` using the `OxcNodeId` carried here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EachBlockSemantics {
    pub item: EachItemKind,
    pub index: EachIndexKind,
    pub key: EachKeyKind,
    pub flavor: EachFlavor,
    /// Runtime `$.each(...)` flags, pre-computed by the Block Semantics
    /// builder. Covers `ITEM_REACTIVE`, `INDEX_REACTIVE`, `ANIMATED`, and
    /// `ITEM_IMMUTABLE`. Does **not** include `IS_CONTROLLED` — that bit
    /// is decided at the codegen call site (element-anchor vs. comment
    /// anchor) and OR'd in there.
    pub each_flags: EachFlags,
    /// Some binding introduced in the each body shadows an outer binding
    /// of the same name. Forces the runtime to thread the collection
    /// through an extra parameter so shadowed reads resolve correctly.
    pub shadows_outer: bool,
    /// Async lowering decision for this each-block's collection. See
    /// [`EachAsyncKind`]. Async is treated as a decoration on top of
    /// the block — per SEMANTIC_LAYER_ARCHITECTURE.md — so it rides in
    /// the block's semantic payload rather than a separate query.
    pub async_kind: EachAsyncKind,
    /// Lowering shape of the collection read. Resolved from
    /// `reactivity_semantics` on the root identifier of the collection
    /// expression. Codegen uses this to choose between a thunk-wrapped
    /// read and a direct prop-getter call; it never re-queries reactivity
    /// for this answer.
    pub collection_kind: EachCollectionKind,
}

/// How the each-block's collection expression lowers at the call site.
///
/// The answer is keyed off the *root identifier* of the expression
/// (after walking through member accesses and parentheses). Non-
/// identifier roots — literals, calls, `this`, etc. — fall into
/// `Regular`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachCollectionKind {
    /// Wrap the expression in a thunk `() => <expr>` before passing
    /// it to `$.each(...)`.
    Regular,
    /// Root is a prop-source getter — pass the identifier directly
    /// without thunk-wrapping, since the getter is already a function.
    PropSource,
}

/// How the each-block's collection expression interacts with async.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachAsyncKind {
    /// Collection expression has no `await` and references no
    /// async-gated symbols. Codegen uses the ordinary `$.each(...)`
    /// shape without a `$.async` wrapper.
    Sync,
    /// Collection expression has an `await` and/or references
    /// async-gated symbols (blockers). Codegen wraps the `$.each(...)`
    /// call inside `$.async(anchor, [blockers], ..., (node, cond) => {...})`.
    Async {
        /// `await` token literally present in the collection expression.
        has_await: bool,
        /// Sorted, de-duplicated blocker indices (from
        /// `BlockerData::symbol_blockers`) collected over every
        /// identifier reference in the collection expression.
        blockers: SmallVec<[u32; 2]>,
    },
}

bitflags! {
    /// Pre-computed intrinsic runtime flags for an `{#each}` block.
    /// Bit layout matches Svelte runtime constants (see reference
    /// `constants.js`): `ITEM_REACTIVE=1`, `INDEX_REACTIVE=2`,
    /// `ANIMATED=8`, `ITEM_IMMUTABLE=16`. Bit `4` (`IS_CONTROLLED`) is
    /// deliberately left to the codegen call site.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct EachFlags: u8 {
        const ITEM_REACTIVE  = 1;
        const INDEX_REACTIVE = 2;
        const ANIMATED       = 8;
        const ITEM_IMMUTABLE = 16;
    }
}

/// The `as <pattern>` introducer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachItemKind {
    /// `{#each items}` — no `as` binding at all.
    NoBinding,
    /// `{#each items as item}` — single identifier introducer.
    Identifier(SymbolId),
    /// `{#each items as { a, b }}` / `{#each items as [a, b]}` — destructured.
    /// The `OxcNodeId` points at the `BindingPattern` node; the consumer
    /// reads it via `ComponentSemantics.js_storage().kind(id)`.
    Pattern(OxcNodeId),
}

/// The `, <index>` introducer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachIndexKind {
    /// No index binding.
    Absent,
    /// `, i` — declared index identifier, with usage facts.
    Declared {
        sym: SymbolId,
        /// At least one expression in the body references the index symbol.
        used_in_body: bool,
        /// The key expression references the index symbol.
        used_in_key: bool,
    },
}

/// The `(<key>)` key expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachKeyKind {
    /// No `(...)` — block is unkeyed; runtime uses positional index.
    Unkeyed,
    /// `(item)` — key is the item identifier itself. Optimized path:
    /// the key function can be elided in some lowerings.
    KeyedByItem,
    /// `(<expr>)` — any other key expression. The `OxcNodeId` points at
    /// the key expression node.
    KeyedByExpr(OxcNodeId),
}

/// High-level lowering flavor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachFlavor {
    /// Default `$.each(...)` lowering.
    Regular,
    /// Body contains at least one `bind:group` directive — needs the
    /// group-index lowering path.
    BindGroup,
}
