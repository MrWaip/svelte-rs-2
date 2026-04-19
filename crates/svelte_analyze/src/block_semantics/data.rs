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
use crate::types::data::StmtHandle;
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
    /// `{#await ...}` block.
    Await(AwaitBlockSemantics),
    /// `{#snippet name(params)}` block.
    Snippet(SnippetBlockSemantics),
    /// `{@const <pattern> = <init>}` block.
    ConstTag(ConstTagBlockSemantics),
    // Placeholders for later slices — payload lands when each kind is
    // migrated end-to-end. Keeping them here shapes the public enum so
    // consumers can already switch on it exhaustively once migrated.
    // TODO(block-semantics): If payload.
    If,
    // TODO(block-semantics): Key payload.
    Key,
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

// ---------------------------------------------------------------------------
// AwaitBlock
// ---------------------------------------------------------------------------

/// `{#await <expr>}...{:then <binding>}...{:catch <binding>}...{/await}` —
/// the presence of each branch, its introduced binding, and the two
/// independent async-shape facts that drive lowering.
///
/// The two async facts are kept as separate fields rather than folded
/// into one enum: `expression_has_await` toggles the thunk wrapping the
/// expression, while `wrapper` decides whether the whole `$.await` call
/// must sit inside a `$.async(...)` block — two orthogonal lowering
/// decisions, not one axis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AwaitBlockSemantics {
    /// `{#await expr}...{/await}` — pending fragment. Never carries a
    /// binding, but the absence of a fragment still matters for
    /// `$.await(...)` arg trimming.
    pub pending: AwaitBranch,
    /// `{:then [<binding>]}` branch.
    pub then: AwaitBranch,
    /// `{:catch [<binding>]}` branch.
    pub catch: AwaitBranch,
    /// Expression literally contains `await` — the expression thunk must
    /// be `async () => await <expr>` rather than the plain `() => <expr>`.
    pub expression_has_await: bool,
    /// Expression references async-gated symbols (blockers) — the whole
    /// `$.await(...)` call is wrapped in `$.async(node, [blockers], [], ...)`.
    pub wrapper: AwaitWrapper,
}

/// Branch presence + optional introduced binding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitBranch {
    /// The branch does not appear in the source (no fragment, no binding).
    Absent,
    /// The branch is present. `binding` is `None` for pending (never
    /// carries one) and for `{:then}` / `{:catch}` without a parameter.
    Present { binding: AwaitBinding },
}

/// The introducer in `{:then <binding>}` / `{:catch <binding>}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitBinding {
    /// No binding declared.
    None,
    /// `{:then user}` / `{:catch err}` — simple identifier.
    Identifier(SymbolId),
    /// `{:then { a, b }}` / `{:catch [msg, code]}` — destructured
    /// pattern. The consumer reads the pattern subtree via
    /// `ComponentSemantics.js_storage()` using `pattern_id`; leaf names
    /// are looked up as `semantics.symbol_name(sym)` without re-walking
    /// the pattern.
    Pattern {
        kind: AwaitDestructureKind,
        leaves: SmallVec<[SymbolId; 4]>,
        pattern_id: OxcNodeId,
    },
}

/// Destructuring shape for `AwaitBinding::Pattern`. Intentionally a
/// cluster-local enum, symmetric to `EachKeyKind` — decoupled from the
/// legacy `DestructureKind` living in `types::data::expr` (which is
/// shared with `render_tags` and cannot be deprecated here).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AwaitDestructureKind {
    Object,
    Array,
}

/// How the `$.await(...)` call must be wrapped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitWrapper {
    /// No wrapper — emit `$.await(...)` directly.
    None,
    /// `$.async(node, [blockers], [], (node) => { $.await(...) })`. The
    /// blocker list mirrors the indices that `BlockerData` associates
    /// with identifier references in the expression, sorted and
    /// deduplicated.
    AsyncWrap { blockers: SmallVec<[u32; 2]> },
}

// ---------------------------------------------------------------------------
// SnippetBlock
// ---------------------------------------------------------------------------

/// `{#snippet name(params)}...{/snippet}` — declaration-shape answer.
///
/// Per-symbol read strategy (how `name` / `label` read inside the body
/// lowers to `name()` vs `$.get(name)`) stays in `reactivity_semantics`
/// under `ContextualDeclarationSemantics::SnippetParam`. This payload
/// covers only facts the consumer needs when emitting the
/// `const name = ($$anchor, ...) => { ... }` declaration itself:
/// the snippet's own const name, whether it hoists above the component
/// function, and each parameter's declaration form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnippetBlockSemantics {
    /// Const name under which the snippet is declared. Read by the
    /// consumer via `ComponentSemantics::symbol_name(sym)` — the source
    /// text slice is never stored.
    pub name: SymbolId,
    /// `true` — snippet lowers to a module-level declaration hoisted
    /// above the component function. `false` — instance-level (inside
    /// the component function) or locally inside a parent block.
    pub hoistable: bool,
    /// Parameters in source order.
    pub params: SmallVec<[SnippetParam; 4]>,
}

/// One parameter of a `{#snippet}` declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnippetParam {
    /// `(name)` — identifier parameter. Lowers to the arrow argument
    /// with `= $.noop` default. User-specified defaults on top-level
    /// identifier params are deliberately not captured: the reference
    /// compiler drops them at the declaration site, so there is no
    /// lowering decision for the payload to answer.
    Identifier { sym: SymbolId },
    /// `({ a, b })` / `([x, y])` — destructured parameter. Lowers to a
    /// positional `$$argN` argument plus per-binding `let` declarations
    /// inside the body.
    Pattern {
        kind: SnippetDestructureKind,
        /// `OxcNodeId` of the outer `BindingPattern`. Consumer walks the
        /// pattern subtree from here to build member paths and clone
        /// default expressions.
        pattern_id: OxcNodeId,
        bindings: SmallVec<[SnippetPatternBinding; 4]>,
    },
}

/// One leaf identifier introduced by a destructured `SnippetParam::Pattern`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetPatternBinding {
    pub sym: SymbolId,
    pub default: SnippetDefaultKind,
    /// `true` — leaf was introduced by `...rest` in an object pattern.
    pub is_rest: bool,
}

/// Whether a snippet parameter / pattern leaf carries a user-specified
/// default, and how the default expression lowers. Classification is
/// pre-computed: the consumer never re-walks the default expression to
/// decide simple-vs-computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetDefaultKind {
    /// No user default. Identifier params get `= $.noop` at emit time;
    /// pattern leaves fall back to a plain member access.
    None,
    /// Default is a simple literal / identifier — inlines directly:
    /// `$.fallback(access, <value>)`.
    Constant,
    /// Default is an arbitrary expression — lowers through a thunk:
    /// `$.fallback(access, () => <value>, true)`.
    Computed,
}

/// Destructure flavor for `SnippetParam::Pattern`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetDestructureKind {
    Object,
    Array,
}

/// `{@const <pattern> = <init>}` — declaration-shape answer.
///
/// Per-symbol read strategy for the bindings themselves (how each
/// introduced name is read inside downstream expressions — `$.get` vs
/// `$.safe_get` vs plain) lives in
/// `reactivity_semantics::ConstDeclarationSemantics::ConstTag`. This
/// payload answers only declaration-shape questions: which symbols the
/// tag introduces, whether the pattern is destructured, how the init
/// expression interacts with async. The init expression itself is
/// cloned/taken from the pre-parsed `Statement` via `stmt_handle` at
/// emit time — the consumer never re-interprets it semantically.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstTagBlockSemantics {
    /// Bindings in source order — for destructured patterns, leaf order
    /// of the pattern. A single element means a plain-identifier tag
    /// (`{@const x = ...}`).
    pub bindings: SmallVec<[SymbolId; 2]>,
    /// `true` iff the init is a destructure pattern
    /// (`{@const { a, b } = expr}` / `{@const [x, y] = expr}`). Consumer
    /// branches on this single flag instead of inspecting `bindings.len()`.
    pub is_destructured: bool,
    /// Handle to the pre-parsed `const <pattern> = <init>;` statement.
    /// Consumer calls `ParserResult::take_stmt(handle)` at emit time to
    /// extract the init expression. Same role as
    /// `AnalysisData::snippet_stmt_handle` in the snippet slice.
    pub stmt_handle: StmtHandle,
    /// Async lowering decision for the init expression.
    pub async_kind: ConstTagAsyncKind,
}

/// How a `{@const}` init expression interacts with async.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstTagAsyncKind {
    /// No `await` and no async-gated symbol references. Codegen emits
    /// `const X = $.derived(() => init)`.
    Sync,
    /// Init contains an `await` literal and/or references async-gated
    /// symbols (script-level blockers). Codegen emits `let X;` plus a
    /// thunk pushed into a per-fragment `$.run([...])` pack; when
    /// `has_await` is true the thunk is wrapped in
    /// `$.async_derived(async () => ...)`, otherwise in
    /// `$.derived(() => ...)`.
    Async {
        /// Literal `await` appears in the init expression — chooses
        /// between `$.async_derived` and `$.derived` at emit time.
        has_await: bool,
        /// Sorted, de-duplicated blocker indices (from
        /// `BlockerData::symbol_blockers`) collected across every
        /// identifier reference in the init expression.
        blockers: SmallVec<[u32; 2]>,
    },
}
