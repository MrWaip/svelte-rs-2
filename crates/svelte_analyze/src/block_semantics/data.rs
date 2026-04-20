use crate::scope::SymbolId;
use bitflags::bitflags;
use smallvec::SmallVec;
use svelte_ast::NodeId;
use svelte_component_semantics::OxcNodeId;

/// Semantic answer for one template block `NodeId`. Every non-block node
/// gets [`BlockSemantics::NonSpecial`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum BlockSemantics {
    /// Node is not a control-flow block.
    #[default]
    NonSpecial,
    /// `{#each items as item}...{/each}`
    Each(EachBlockSemantics),
    /// `{#await promise}...{:then v}...{:catch e}...{/await}`
    Await(AwaitBlockSemantics),
    /// `{#snippet row(x)}...{/snippet}`
    Snippet(SnippetBlockSemantics),
    /// `{@const doubled = x * 2}`
    ConstTag(ConstTagBlockSemantics),
    /// `{@render row(item)}`
    Render(RenderTagBlockSemantics),
    /// `{#if cond}...{:else if c2}...{:else}...{/if}` — root only;
    /// flattened `{:else if}` branches are tombstoned to `NonSpecial`.
    If(IfBlockSemantics),
    /// `{#key expr}...{/key}`
    Key(KeyBlockSemantics),
}

/// `{#each items as item, i (key)}...{/each}` — shape of the three
/// introducers plus block flavor and async/collection lowering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EachBlockSemantics {
    /// The `as <pattern>` introducer.
    pub item: EachItemKind,
    /// The `, <index>` introducer.
    pub index: EachIndexKind,
    /// The `(<key>)` key expression.
    pub key: EachKeyKind,
    /// Default vs. `bind:group` lowering.
    pub flavor: EachFlavor,
    /// Runtime `$.each(...)` flag bits. `IS_CONTROLLED` is OR'd in by
    /// codegen, not stored here.
    pub each_flags: EachFlags,
    /// A binding inside the body shadows an outer binding of the same name.
    /// Example: `{#each items as items}` — inner `items` shadows outer.
    pub shadows_outer: bool,
    /// Async lowering decision for the collection expression.
    pub async_kind: EachAsyncKind,
    /// Whether the collection read is thunk-wrapped or passed directly.
    pub collection_kind: EachCollectionKind,
}

/// How the collection expression lowers at the `$.each` call site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachCollectionKind {
    /// Wrap in a thunk. Example: `{#each items as x}` → `() => items`.
    Regular,
    /// Root is a prop-source getter; pass directly. Example:
    /// `{#each items as x}` where `items` comes from `$props()`.
    PropSource,
}

/// How the collection expression interacts with async.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachAsyncKind {
    /// Plain `{#each items as x}` — no `await`, no async-gated symbols.
    Sync,
    /// Expression has `await` or references async-gated symbols. Example:
    /// `{#each await fetchItems() as x}`.
    Async {
        /// Literal `await` in the expression.
        has_await: bool,
        /// Sorted, deduplicated blocker indices.
        blockers: SmallVec<[u32; 2]>,
    },
}

bitflags! {
    /// Runtime flag bits for an `{#each}` block. Matches reference
    /// `constants.js`: `ITEM_REACTIVE=1`, `INDEX_REACTIVE=2`,
    /// `ANIMATED=8`, `ITEM_IMMUTABLE=16`.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct EachFlags: u8 {
        const ITEM_REACTIVE  = 1;
        const INDEX_REACTIVE = 2;
        const ANIMATED       = 8;
        const ITEM_IMMUTABLE = 16;
    }
}

/// The `as <pattern>` introducer of `{#each}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachItemKind {
    /// `{#each items}` — no `as` binding.
    NoBinding,
    /// `{#each items as item}` — identifier introducer.
    Identifier(SymbolId),
    /// `{#each items as { a, b }}` / `{#each items as [a, b]}`.
    Pattern(OxcNodeId),
}

/// The `, <index>` introducer of `{#each}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachIndexKind {
    /// `{#each items as x}` — no index.
    Absent,
    /// `{#each items as x, i}` — index declared, with usage facts.
    Declared {
        sym: SymbolId,
        /// Body references `i`.
        used_in_body: bool,
        /// Key expression references `i`, e.g. `(i)`.
        used_in_key: bool,
    },
}

/// The `(<key>)` key expression of `{#each}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EachKeyKind {
    /// `{#each items as x}` — unkeyed, positional.
    Unkeyed,
    /// `{#each items as item (item)}` — key is the item identifier itself.
    KeyedByItem,
    /// `{#each items as x (x.id)}` — arbitrary key expression.
    KeyedByExpr(OxcNodeId),
}

/// High-level lowering flavor of `{#each}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachFlavor {
    /// Default `$.each(...)` lowering.
    Regular,
    /// Body has `bind:group` — uses the group-index path. Example:
    /// `{#each opts as o}<input type="radio" bind:group={x} value={o}>{/each}`.
    BindGroup,
}

// ---------------------------------------------------------------------------
// AwaitBlock
// ---------------------------------------------------------------------------

/// `{#await p}pending{:then v}ok{:catch e}err{/await}` — branch presence,
/// introduced bindings, and the two orthogonal async lowering decisions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AwaitBlockSemantics {
    /// Pending branch (before `{:then}`). Never carries a binding.
    pub pending: AwaitBranch,
    /// `{:then [<binding>]}` branch.
    pub then: AwaitBranch,
    /// `{:catch [<binding>]}` branch.
    pub catch: AwaitBranch,
    /// Expression contains literal `await`, e.g. `{#await await p()}`.
    pub expression_has_await: bool,
    /// Whether the whole `$.await(...)` call sits inside `$.async(...)`.
    pub wrapper: AwaitWrapper,
}

/// One branch of `{#await}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitBranch {
    /// Branch not present in source.
    Absent,
    /// Branch present, optionally with a binding (never for pending).
    Present { binding: AwaitBinding },
}

/// The introducer in `{:then <binding>}` / `{:catch <binding>}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitBinding {
    /// No binding, e.g. `{:then}` / `{:catch}`.
    None,
    /// `{:then user}` / `{:catch err}`.
    Identifier(SymbolId),
    /// `{:then { a, b }}` / `{:catch [msg, code]}`.
    Pattern {
        kind: AwaitDestructureKind,
        leaves: SmallVec<[SymbolId; 4]>,
        pattern_id: OxcNodeId,
    },
}

/// Destructuring shape for [`AwaitBinding::Pattern`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AwaitDestructureKind {
    /// `{:then { a, b }}`
    Object,
    /// `{:then [a, b]}`
    Array,
}

/// How the `$.await(...)` call is wrapped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AwaitWrapper {
    /// Emit `$.await(...)` directly.
    None,
    /// Wrap in `$.async(node, [blockers], [], ...)`.
    AsyncWrap { blockers: SmallVec<[u32; 2]> },
}

// ---------------------------------------------------------------------------
// SnippetBlock
// ---------------------------------------------------------------------------

/// `{#snippet name(a, { b })}...{/snippet}` — declaration shape:
/// snippet name, hoisting decision, and parameter forms.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnippetBlockSemantics {
    /// Const name under which the snippet is declared.
    pub name: SymbolId,
    /// `true` = hoisted to module level above the component function;
    /// `false` = instance-level or nested in a parent block.
    pub hoistable: bool,
    /// Parameters in source order.
    pub params: SmallVec<[SnippetParam; 4]>,
}

/// One parameter of a `{#snippet}` declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnippetParam {
    /// `{#snippet row(item)}` — plain identifier parameter.
    Identifier { sym: SymbolId },
    /// `{#snippet row({ a, b })}` / `{#snippet row([x, y])}`.
    Pattern { pattern_id: OxcNodeId },
}

/// `{@const <pattern> = <init>}` — declaration shape and async lowering
/// of the initializer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstTagBlockSemantics {
    /// `OxcNodeId` of the backing `VariableDeclaration`.
    pub decl_node_id: OxcNodeId,
    /// Async lowering decision for the init expression.
    pub async_kind: ConstTagAsyncKind,
}

/// How a `{@const}` initializer interacts with async.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstTagAsyncKind {
    /// `{@const x = a + b}` — sync. Emits `const X = $.derived(() => init)`.
    Sync,
    /// `{@const x = await p()}` or init references async-gated symbols.
    /// Emits `let X;` plus a thunk pushed into `$.run([...])`.
    Async {
        /// Literal `await` in the init.
        has_await: bool,
        /// Sorted, deduplicated blocker indices.
        blockers: SmallVec<[u32; 2]>,
    },
}

// ---------------------------------------------------------------------------
// RenderTag
// ---------------------------------------------------------------------------

/// `{@render expr(args...)}` — call-site lowering shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderTagBlockSemantics {
    /// Reactive-callee × optional-chain, collapsed into 4 variants.
    pub callee_shape: RenderCalleeShape,
    /// `Some(sym)` for plain `{@render name(...)}` where `name` resolves
    /// to a binding. Used by CSS pruning.
    pub callee_sym: Option<SymbolId>,
    /// Per-argument lowering, in source order. Length matches the
    /// call's argument list exactly.
    pub args: SmallVec<[RenderArgLowering; 4]>,
    /// Async wrapper decision for the whole render call.
    pub async_kind: RenderAsyncKind,
}

/// Four non-overlapping lowering shapes of the `{@render}` callee.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderCalleeShape {
    /// `{@render row(x)}` — normal binding → `row(anchor, ...thunks)`.
    Static,
    /// `{@render row?.(x)}` — normal binding, optional chain.
    StaticChain,
    /// `{@render row(x)}` where `row` is a reactive binding (prop / rune /
    /// store). Lowers via `$.snippet(anchor, () => row, ...thunks)`.
    Dynamic,
    /// `{@render row?.(x)}` with a reactive callee.
    DynamicChain,
}

/// Lowering shape of a single `{@render}` argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderArgLowering {
    /// Arg is a bare prop-source identifier. Example: `{@render r(items)}`
    /// where `items` comes from `$props()`. Passed directly, no thunk.
    PropSource { sym: SymbolId },
    /// Arg has a call but no `await`. Example: `{@render r(map(xs))}`.
    /// Wrapped in a local `$.derived(() => arg)` memo.
    MemoSync,
    /// Arg has `await`. Example: `{@render r(await load())}`. Becomes an
    /// async-values entry, read inside the callback as `$.get($$async_id_k)`.
    MemoAsync,
    /// Plain `() => arg` thunk — no memo, no prop pass-through.
    Plain,
}

/// Async wrapper decision for the whole `{@render}` call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderAsyncKind {
    /// No arg has `await`, no dep has a blocker. Emit directly.
    Sync,
    /// Wrap in `$.async(anchor, blockers, async_values, (anchor, $$a0, ...) => {...})`.
    Async {
        /// Sorted, deduplicated blocker indices unioned across all args.
        /// Callee blockers are excluded.
        blockers: SmallVec<[u32; 2]>,
    },
}

// ---------------------------------------------------------------------------
// IfBlock
// ---------------------------------------------------------------------------

/// `{#if a}...{:else if b}...{:else}...{/if}` — full branch chain after
/// `{:else if}` flattening. Identity is the root IfBlock `NodeId`;
/// absorbed elseifs carry `NonSpecial`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfBlockSemantics {
    /// One entry per branch in source order: root `{#if}` first, then
    /// each flattened `{:else if}`. Length ≥ 1.
    pub branches: SmallVec<[IfBranch; 2]>,
    /// Disposition of the final alternate fragment.
    pub final_alternate: IfAlternate,
    /// `true` when this payload describes a non-flattened elseif being
    /// re-dispatched as its own root (marks the branch as
    /// `EFFECT_TRANSPARENT` for transitions).
    pub is_elseif_root: bool,
    /// Root-level async wrapper decision, driven by the root test only.
    pub async_kind: IfAsyncKind,
}

/// One branch of a flattened `{#if} / {:else if}` chain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfBranch {
    /// `NodeId` of the IfBlock node that authored this branch. For
    /// branch 0 this equals the payload's own root id.
    pub block_id: NodeId,
    /// Lowering shape of the branch's test expression.
    pub condition: IfConditionKind,
}

/// Three non-overlapping lowering shapes for a branch condition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfConditionKind {
    /// `{#if x > 0}` — raw expression, read inline in `$$render`.
    Raw,
    /// `{#if f(x)}` — expression has a call but no `await`. Wrapped in a
    /// `$.derived` memo, read via `$.get(d)`.
    Memo,
    /// Root-only. `{#if await check()}` — read through the enclosing
    /// `$.async` wrapper's `$$condition` parameter.
    AsyncParam,
}

/// Root-level async wrapper decision for `{#if}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IfAsyncKind {
    /// No `await`, no blocker. Emit `$.if(...)` directly.
    Sync,
    /// Root wrapped in `$.async(anchor, blockers, async_values,
    /// (node, $$condition?) => {...})`.
    Async {
        /// Root test contains literal `await` — callback gains `$$condition`.
        root_has_await: bool,
        /// Sorted, deduplicated blocker indices for the root test's refs.
        blockers: SmallVec<[u32; 2]>,
    },
}

/// Disposition of the `{:else}` fragment after flattening stops.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IfAlternate {
    /// No `{:else}` on the last flattened branch.
    None,
    /// A concrete `{:else}` fragment exists on the last flattened branch.
    Fragment {
        /// `NodeId` of the IfBlock whose alternate fragment this is.
        last_branch_block_id: NodeId,
    },
}

// ---------------------------------------------------------------------------
// KeyBlock
// ---------------------------------------------------------------------------

/// `{#key expr}...{/key}` — root-level async wrapper decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyBlockSemantics {
    pub async_kind: KeyAsyncKind,
}

/// Whether `{#key}` lowers directly or through `$.async`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyAsyncKind {
    /// `{#key x}` — emit `$.key(...)` directly.
    Sync,
    /// `{#key await load()}` — wrap in `$.async(anchor, [blockers],
    /// async_values, (node, $$key?) => { $.key(node, () => $.get($$key), body) })`.
    Async {
        /// Literal `await` in the expression.
        has_await: bool,
        /// Sorted, deduplicated blocker indices for resolved refs.
        blockers: SmallVec<[u32; 2]>,
    },
}
