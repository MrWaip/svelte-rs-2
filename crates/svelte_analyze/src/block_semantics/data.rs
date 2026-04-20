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
use svelte_ast::NodeId;
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
    /// `{@render expr(args...)}` tag.
    Render(RenderTagBlockSemantics),
    /// `{#if ...} ... {:else if ...} ... {:else} ... {/if}` block (root
    /// only — flattened `{:else if}` IfBlocks are tombstoned to
    /// [`BlockSemantics::NonSpecial`] by the builder).
    If(IfBlockSemantics),
    /// `{#key expr} ... {/key}` block.
    Key(KeyBlockSemantics),
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
///
/// Structural information about destructured parameters (form, keys,
/// indexes, defaults, rest) lives in the OXC `BindingPattern` reached
/// through `pattern_id`. Codegen walks it directly at emit time and
/// classifies defaults inline via `is_simple_expression` — there is
/// deliberately no parallel per-leaf Svelte-side shape here.
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
    /// inside the body. `pattern_id` is the `OxcNodeId` of the outer
    /// `BindingPattern`; codegen walks that subtree to emit the
    /// destructuring, including `$.to_array` intermediates for arrays
    /// (per-level state that doesn't fit a flat leaf model).
    Pattern { pattern_id: OxcNodeId },
}

/// `{@const <pattern> = <init>}` — declaration-shape answer.
///
/// Per-symbol read strategy for the bindings themselves (how each
/// introduced name is read inside downstream expressions — `$.get` vs
/// `$.safe_get` vs plain) lives in
/// `reactivity_semantics::ConstDeclarationSemantics::ConstTag`. This
/// payload answers only declaration-shape questions: where the pattern
/// and init live in the AST and how the init interacts with async.
///
/// Pattern bindings / destructure flavour / init expression are read on
/// demand in the consumer via `ComponentSemantics::js_storage().kind(id)`
/// + `walk_bindings(&pattern, ...)` — the payload stays minimal per the
/// Parser-handle ban (no `StmtHandle` / `ExprHandle` in cluster answers).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstTagBlockSemantics {
    /// `OxcNodeId` of the `VariableDeclaration` backing the tag
    /// (`const <pattern> = <init>`). Consumers resolve the statement
    /// through `ComponentSemantics::js_kind(id)` when they need the
    /// binding pattern or init expression.
    pub decl_node_id: OxcNodeId,
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

// ---------------------------------------------------------------------------
// RenderTag
// ---------------------------------------------------------------------------

/// `{@render expr(args...)}` — lowering-shape answer.
///
/// Identity is the RenderTag block `NodeId`. The inner `CallExpression`
/// is reached on demand via the tag's expression span through existing
/// `ComponentSemantics` resolution — no handle is carried here (per the
/// Parser-handle ban).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderTagBlockSemantics {
    /// Composite of "reactive callee?" × "optional chain?" — one
    /// 4-way enum so the consumer dispatches the call shape with a
    /// single match.
    pub callee_shape: RenderCalleeShape,
    /// `Some(sym)` when the callee is a plain `Identifier` resolving
    /// to a binding; `None` for member / computed / chain-head
    /// callees. Read by CSS pruning to narrow the set of possible
    /// snippets this render tag can target without re-resolving the
    /// callee identifier. Orthogonal to `callee_shape` — shape
    /// decides lowering, `callee_sym` decides downstream analysis.
    pub callee_sym: Option<SymbolId>,
    /// Per-argument lowering answer, in source order. Length matches
    /// the `CallExpression.arguments` length exactly (spread is
    /// rejected at analyze time, so every arg is an `Expression`).
    /// Consumer zips with the CallExpression's argument slice — no
    /// `OxcNodeId` per element needed.
    pub args: SmallVec<[RenderArgLowering; 4]>,
    /// Async wrapper decision for the whole render call. Mirrors
    /// `AwaitWrapper` / `EachAsyncKind::Async` in shape: either sync
    /// (direct emission) or wrapped in `$.async(...)`.
    pub async_kind: RenderAsyncKind,
}

/// Four non-overlapping lowering shapes of the `{@render}` callee.
///
/// "Dynamic" means the callee identifier resolves to a reactive
/// binding (prop / rune / store / contextual); equivalent to the
/// reference compiler's `binding?.kind !== 'normal'`. "Chain" means
/// the tag is written `{@render fn?.(...)}` with an optional call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderCalleeShape {
    /// Normal binding, non-chain. Lowers to `callee(anchor, ...thunks)`.
    Static,
    /// Normal binding, optional chain. Lowers to
    /// `callee?.(anchor, ...thunks)` (`b.maybe_call`).
    StaticChain,
    /// Reactive binding. Lowers to
    /// `$.snippet(anchor, () => callee, ...thunks)`.
    Dynamic,
    /// Reactive binding, optional chain. Lowers to
    /// `$.snippet(anchor, () => callee ?? $.noop, ...thunks)`.
    DynamicChain,
}

/// Composite per-argument lowering. Collapses the reference
/// compiler's four memoizer cases (prop-source pass-through, sync
/// memo, async memo, plain thunk) into a single enum so the consumer
/// never reassembles the decision from `has_call` / `has_await` /
/// binding-kind facts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderArgLowering {
    /// Argument is a single `Identifier` that resolves to a
    /// `$props()` source binding. Codegen passes the getter
    /// identifier directly, with no thunk wrapping — the prop getter
    /// already has the right shape.
    PropSource { sym: SymbolId },
    /// Expression contains a call but no `await`. Codegen wraps the
    /// argument in a local `$.derived(() => <arg>)` memo and passes
    /// `() => $.get(memo)` as the thunk. Memo names are synthesised at
    /// emit time (`$0`, `$1`, ...).
    MemoSync,
    /// Expression contains an `await`. The argument becomes an entry
    /// in the async wrapper's `async_values` array; inside the
    /// callback it's read as `$.get($$async_id_k)`. The callback
    /// param order is the source order of `MemoAsync` entries in
    /// `RenderTagBlockSemantics.args`.
    MemoAsync,
    /// Plain `() => <arg>` thunk — no memo, no prop pass-through.
    Plain,
}

/// Async wrapper decision for the whole render call. Mirrors
/// `AwaitWrapper` in shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderAsyncKind {
    /// No argument has `await`, no dependency has a blocker. Emit
    /// the render statement(s) directly.
    Sync,
    /// Wrap the render statement(s) in
    /// `$.async(anchor, blockers, async_values, (anchor, $$async_id_0, ...) => { ... })`.
    Async {
        /// Sorted, de-duplicated blocker indices (from
        /// `BlockerData::symbol_blockers`) unioned across every
        /// argument's dependency set. Callee blockers are not
        /// included — the reference compiler does not route the
        /// callee expression through the memoizer.
        blockers: SmallVec<[u32; 2]>,
    },
}

// ---------------------------------------------------------------------------
// IfBlock
// ---------------------------------------------------------------------------

/// `{#if expr} ... {:else if expr} ... {:else} ... {/if}` — the full
/// branch chain after elseif flattening.
///
/// Identity: the **root** IfBlock `NodeId`. IfBlocks that the builder
/// absorbs as flattened branches carry [`BlockSemantics::NonSpecial`] so
/// the codegen dispatcher never wakes them up. An elseif that stopped
/// flattening (because it introduces new blockers or its own `await`)
/// gets its own `BlockSemantics::If` payload: it becomes the alternate
/// fragment's first child and codegen re-dispatches through the normal
/// fragment-walk path.
///
/// Reactive meaning of the branch test expressions stays in
/// `reactivity_semantics` (one `ReferenceId` per identifier); the
/// transformer rewrites references before codegen sees them. This
/// payload answers only the branch-layout and async-shape questions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfBlockSemantics {
    /// One entry per branch in source order: the root `{#if}` first,
    /// then each flattened `{:else if}`. Length >= 1.
    pub branches: SmallVec<[IfBranch; 2]>,
    /// Disposition of the final alternate fragment.
    ///
    /// `None` when the last branch has no `alternate`.
    /// `Fragment { last_branch_block_id }` when a concrete `{:else}`
    /// fragment exists — the consumer emits it as a fragment-rendering
    /// arrow. This also covers "alternate is a non-flattenable nested
    /// IfBlock": the nested IfBlock carries its own root payload and
    /// normal fragment codegen dispatches through it from inside the
    /// alternate arrow.
    pub final_alternate: IfAlternate,
    /// `true` iff the root IfBlock carries `IfBlock.elseif == true` —
    /// i.e. this payload describes a non-flattened elseif that is being
    /// re-dispatched as its own root. Consumer forwards as the third
    /// argument to `$.if(...)` so the runtime marks the branch as
    /// `EFFECT_TRANSPARENT` for transition purposes.
    pub is_elseif_root: bool,
    /// Root-level async wrapper decision. Folded from the root branch's
    /// test expression only — flattening ensures no absorbed branch
    /// adds new await/blockers.
    pub async_kind: IfAsyncKind,
}

/// One branch of a flattened `{#if} / {:else if}` chain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfBranch {
    /// `NodeId` of the IfBlock node that authored this branch. For
    /// branch `0` this equals the payload's own root NodeId; for
    /// flattened branches it is the absorbed elseif IfBlock's NodeId.
    /// Used by the consumer to key `FragmentKey::IfConsequent(id)` and
    /// to fetch the branch's test expression through
    /// `ctx.node_expr_handle(id)`.
    pub block_id: NodeId,
    /// How the branch's test expression lowers at the `$.if` call site.
    pub condition: IfConditionKind,
}

/// Three non-overlapping lowering shapes for a branch condition.
///
/// Mirrors the reference compiler's three emission variants
/// (`reference/compiler/phases/3-transform/client/visitors/IfBlock.js`
/// lines 39-52). Collapses the legacy consumer's `has_await` /
/// `has_call` bit arithmetic into a single enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfConditionKind {
    /// Raw expression, read inline in the `$$render` callback.
    Raw,
    /// Wrap the expression in a `$.derived` memo declared alongside the
    /// consequent arrow, and read through `$.get(d)`. Chosen when the
    /// expression contains a call but no `await`.
    Memo,
    /// Root branch **only**, when the root test contains `await`. The
    /// branch reads its condition through the enclosing `$.async`
    /// wrapper's `$$condition` parameter: `test = $.get($$condition)`.
    /// The expression itself is pulled out as the async wrapper's
    /// thunk.
    AsyncParam,
}

/// Root-level async wrapper decision. Shape mirrors
/// [`EachAsyncKind`] / [`RenderAsyncKind::Async`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IfAsyncKind {
    /// No `await` in the root test and no script-level blocker on any
    /// identifier it references. Codegen emits `$.if(...)` directly.
    Sync,
    /// Root wrapped in
    /// `$.async(anchor, blockers, async_values, (node, $$condition?) => { ... })`.
    ///
    /// * `root_has_await` drives two things: the async-values array
    ///   carries the root expression's thunk, and the callback gains
    ///   the `$$condition` parameter.
    /// * `blockers` is the sorted, de-duplicated script-level blocker
    ///   index set for the root test's references.
    Async {
        root_has_await: bool,
        blockers: SmallVec<[u32; 2]>,
    },
}

/// Final-alternate disposition after flattening stops.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IfAlternate {
    /// No `{:else}` exists on the last flattened branch.
    None,
    /// A concrete `{:else}` fragment exists on the last flattened
    /// branch. Consumer emits it as a fragment-rendering arrow keyed
    /// by `FragmentKey::IfAlternate(last_branch_block_id)`.
    Fragment {
        /// `NodeId` of the IfBlock whose alternate fragment this is —
        /// i.e. the last branch's block id.
        last_branch_block_id: NodeId,
    },
}

// ---------------------------------------------------------------------------
// KeyBlock
// ---------------------------------------------------------------------------

/// `{#key expr}...{/key}` — root-level async wrapper decision.
///
/// Identity: KeyBlock `NodeId`. The body fragment is reached via
/// `FragmentKey::KeyBlockBody(id)` by the consumer — not carried on
/// the payload. The key expression itself is consumed from
/// `ParserResult` via the block's `expression_span` (existing
/// `get_node_expr` path).
///
/// Reactive meaning of identifiers inside the expression stays in
/// `reactivity_semantics`; the transformer rewrites references before
/// codegen takes ownership of the expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyBlockSemantics {
    pub async_kind: KeyAsyncKind,
}

/// Whether `{#key}` lowers directly or inside an `$.async` wrapper.
/// Shape mirrors [`EachAsyncKind`] / [`IfAsyncKind`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyAsyncKind {
    /// No `await`, no script-level blockers. Emit `$.key(...)` directly.
    Sync,
    /// Wrap in
    /// `$.async(anchor, [blockers], async_values, (node, $$key?) => { $.key(node, () => $.get($$key), body) })`.
    ///
    /// * `has_await` — literal `await` appears in the expression.
    ///   Drives the `$$key` callback parameter and the async-values
    ///   thunk.
    /// * `blockers` — sorted, de-duplicated script-level blocker
    ///   indices for every resolved identifier reference in the
    ///   expression.
    Async {
        has_await: bool,
        blockers: SmallVec<[u32; 2]>,
    },
}
