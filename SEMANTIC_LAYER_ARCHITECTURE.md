# Semantic Layer Architecture

Working document for the compile-pipeline semantic layer beyond reactivity.
This file is not a migration plan — it records the constraints shared across
four semantic clusters before inventorying old ownership.

Reactive meaning has its own document: `REACTIVITY_ARCHITECTURE.md`. Read it
first. Everything here assumes its Consumer Model, Identity Model, Storage
Content Rule, Lowering Boundary, and Dependency Boundary hold.

## Goal

Every semantic cluster in the compile pipeline should answer the same kind of
question as `reactivity_semantics` does for declarations and references:

what does this template unit mean, and what operation does the consumer need
to perform for it?

Template consumers (`svelte_transform`, `svelte_codegen_client`) should stop
assembling meaning out of scattered walker passes, `AnalysisData` flags, and
AST re-inspection. Each cluster owns one semantic contract for one template
unit kind, returned by one query keyed by a stable node identity.

## Shared Principles

Identical in spirit to the reactivity contract. Each cluster is held to the
same bar.

- **Single source of truth.** Exactly one module in `crates/svelte_analyze/src/`
  owns the cluster's semantic meaning. Other passes may contribute raw facts
  but must not stay parallel owners.
- **Consumer Model.** Consumer code takes one query per unit, pattern-matches
  once, and may read AST only as emission payload after the branch is chosen.
- **Root Consumer Migration Rule.** Migration starts at the root consumer
  point for one unit kind. New branch selected by one top-level match on the
  semantic answer; legacy fallback untouched at the same point. No secondary
  semantic lookups inside the migrated branch. No local pseudo-semantic enums
  in consumers. No AST-based reconstruction of meaning in the migrated branch.
- **Identity Model.** Cluster-facing keys are node-oriented:
  - Block: block `NodeId` (the `{#...}` / `{@...}` tag node)
  - Attribute: attribute `NodeId`
  - ElementShape: element-node `NodeId`
  - Async: unit `NodeId` owning the async behavior

  `SymbolId` is internal only. Strings are never public keys.
- **Storage Content Rule.** Cluster answers carry only `NodeId`, `OxcNodeId`,
  `ReferenceId`, `SymbolId`, enum variants, bools, numeric payloads. No
  `String`, `Box<str>`, `&str`, `Cow<str>` in stored facts or answers. Text is
  resolved at consumption time via identity-keyed lookups.
- **Lowering Boundary.** Answers may encode "what operation is required" but
  not "which runtime helper to call" / "which builder recipe to invoke".
- **Dependency Boundary.** Each cluster builds on `ComponentSemantics` and AST,
  not on legacy Svelte-specific classification tables. Concretely: a cluster
  builder's signature accepts only AST (`&Component` and `&ParserResult` —
  the latter is the parser's pre-parsed JS store for template spans and is
  considered part of the AST surface, not a separate cluster),
  `ComponentSemantics`, and — where genuinely needed —
  `ReactivitySemantics`. It never accepts `&AnalysisData` (full or partial
  borrow), never reads other clusters' side-tables, and never reaches into
  legacy fields like `EachContextIndex`, `TemplateSemanticsData`,
  `ExpressionInfo`. Pipeline code assembles the builder's output into
  `AnalysisData`; the builder itself does not see it.
- **Escalation On Missing Facts.** If during a cluster's implementation the
  allowed inputs (AST + `ComponentSemantics` + `ReactivitySemantics`) turn
  out to be insufficient for some specific fact, the builder author does
  **not** silently widen the dependency surface. The kind's work stops and
  the gap is surfaced to the human: which fact, why it's not reachable,
  which options are on the table. Accepted resolutions include: (1) moving
  the fact into `ComponentSemantics` if it is generic; (2) lifting it from
  AST into the cluster builder if it is the cluster's own; (3) admitting a
  narrow dependency on another cluster with explicit justification; or
  (4) re-opening the cluster decomposition. Quiet AnalysisData reads are
  not an option.
- **Traversal Budget.** Each cluster's builder is held to a fixed number of
  walks per component: **1 walk of the instance script** (OXC Visit), **1
  walk of the module script** (OXC Visit), and **1 walk of the template**
  (Svelte AST). Nested sub-walks over template-owned statement/expression
  subtrees are permitted inside the template walk when the fact cannot be
  answered from `ComponentSemantics` directly, but must stay single-pass
  per subtree. Re-walking the same nodes within the cluster is forbidden.
  Facts already available via `ComponentSemantics` (references, scopes,
  symbol facts) are read, not re-derived — and the read does not count
  against the traversal budget.
- **Reactive meaning stays in `reactivity_semantics`.** When a cluster answer
  refers to an expression or identifier, it carries only the `ReferenceId`
  of the relevant reference. The transformer rewrites that reference from
  `reference_semantics(reference_id)` before codegen sees the result. Clusters
  here do not classify reactive meaning themselves.
- **Pre-Code Shape Check.** Before a migrated consumer path is implemented,
  the author writes a short pseudo-sketch: root point, one query, one match,
  migrated branch, fallback. If the sketch is not linear, the contract is not
  ready — extend the analyzer first.
- **Sparse storage, total API.** Missing entries normalize to the cluster's
  `NonSpecial` variant; `None` does not leak. `Unresolved` stays distinct from
  non-special.
- **Unit-scoped migration.** A cluster is not migrated in one shot. The unit
  is one kind inside the cluster (one block kind, one attribute kind, one
  element shape) closed end-to-end: builder → answer variant → consumer
  migration → old consumer-side assembly removed → tests green.

## Block Semantics

Scope: template control-flow and binding-introducing blocks whose lowering is
owned by template semantics proper.

In scope:
- `{#each}`, `{#if}`, `{#await}`, `{#key}`, `{#snippet}`, `{@const}`,
  `{@render}`

Out of scope:
- `{@html}`, `{@debug}` — direct emission, no block semantics
- Special elements — they live in ElementShape Semantics

Identity: block `NodeId`.

Draft answer shape:

```rust
pub enum BlockSemantics {
    NonSpecial,

    Each(EachBlockSemantics),
    If(IfBlockSemantics),
    Await(AwaitBlockSemantics),
    Key(KeyBlockSemantics),
    Snippet(SnippetBlockSemantics),
    ConstTag(ConstTagSemantics),
    Render(RenderTagSemantics),

    Unresolved,
}
```

Payloads carry introducer identities only. Context-binding reactive meaning
stays in `reactivity_semantics` (introducer identity in Block; declaration +
reference semantics in reactivity). This keeps the legacy-reactivity rule for
`{#each}` context-variable mutation resolvable from one side of the boundary.

## Attribute Semantics

Scope: attributes, bindings, and directives attached to any element-shape
(HTML, `<svelte:element>`, component-shaped). One cluster covers attribute-
shaped AST across element kinds.

Identity: attribute `NodeId`.

Draft answer shape:

```rust
pub enum AttributeSemantics {
    HtmlProperty(HtmlPropertySemantics),
    HtmlClass(HtmlClassSemantics),
    HtmlStyle(HtmlStyleSemantics),
    HtmlClassDirective(HtmlClassDirectiveSemantics),
    HtmlStyleDirective(HtmlStyleDirectiveSemantics),
    HtmlBind(HtmlBindSemantics),
    HtmlSpread(HtmlSpreadSemantics),
    Event(EventSemantics),

    ComponentProp(ComponentPropSemantics),
    ComponentBind(ComponentBindSemantics),
    ComponentSpread(ComponentSpreadSemantics),
    ComponentSnippetAttachment(ComponentSnippetSemantics),

    BoundaryHandler(BoundaryHandlerSemantics),
    BoundarySlotAttachment(BoundarySlotSemantics),

    Use(UseDirectiveSemantics),
    Transition(TransitionDirectiveSemantics),
    Animate(AnimateDirectiveSemantics),
    Attach(AttachSemantics),

    Unresolved,
}
```

Notes:
- Consumer sees one variant per attribute regardless of whether the attribute
  sits on HTML, `<svelte:element>`, or a component-shaped element.
- Spread and expression variants carry a `ReferenceId`. Reactive meaning of
  the spread/expression operand (including legacy `$$props` / `$$restProps`)
  is resolved via `reference_semantics(reference_id)` and rewritten by the
  transformer. Attribute cluster does not know about legacy props bags.
- `Event` covers both `on:click` (legacy directive syntax) and `onclick`
  (modern attribute syntax). The variant payload encodes which syntax form,
  because the two have different modifier/delegation semantics even after
  they resolve to the same DOM event.

## Element Shape Semantics

Scope: element-node shapes, i.e. the decision the consumer currently makes
by pattern-matching on AST node kind. One cluster folds that dispatch into
one semantic query keyed by element `NodeId`.

Identity: element `NodeId` (AST node id of the `Element` / `ComponentNode` /
`SvelteElement` / `SvelteBoundary` / `SvelteWindow` / etc. node).

Draft answer shape:

```rust
pub enum ElementShapeSemantics {
    Html(HtmlElementSemantics),
    SvelteElement(SvelteElementSemantics),

    Component(ComponentInvocationSemantics),
    DynamicComponent(DynamicComponentSemantics),
    SelfComponent(SelfComponentSemantics),

    Boundary(BoundaryElementSemantics),

    SpecialTarget(SpecialTargetSemantics),

    Unresolved,
}
```

`SpecialTargetSemantics` variants: `Head`, `Window`, `Document`, `Body`.

Notes:
- `<svelte:element>` lives here as its own variant because the element-shape
  decision (static vs dynamic tag, namespace derivation) is the distinguishing
  concern. Per-attribute classification on `<svelte:element>` stays in
  Attribute Semantics under the same HTML-attached variants.
- `<svelte:boundary>` is an element-shape — not a block — because its lowering
  shape is element-with-props-and-children, and its attributes classify like
  any other attached attribute (via Attribute Semantics).
- `<svelte:options>` is compile-time-only and does not appear in this enum.
- `<svelte:fragment>` legacy and `<slot>` legacy are owned by their legacy
  spec; not surfaced here.

## Async Semantics

Scope: everything gated behind the `experimental.async` compile flag.

In scope:
- Pickled await expressions inside template / attribute / component-prop
  expressions.
- Async blockers and barrier placement.
- `{#await}` lowering differences under async mode.
- Top-level `await` in `<script>` (async-mode only).
- `$state.eager`, `$state.snapshot`, `$effect.pending` async interactions.

Out of scope:
- Non-async `{#await}` — owned by Block Semantics.
- Regular reactive state lifecycle — owned by `reactivity_semantics`.

Identity: unit `NodeId` of the construct owning the async behavior (block,
await expression, attribute expression, component-prop expression).

Draft answer shape:

```rust
pub enum AsyncSemantics {
    NonAsync,

    AwaitBlock(AwaitBlockAsyncSemantics),
    TopLevelAwait(TopLevelAwaitSemantics),
    PickledAwait(PickledAwaitSemantics),
    Blocker(AsyncBlockerSemantics),

    Unresolved,
}
```

Async is migrated last because it decorates units already stabilized by the
other three clusters.

## Migration Order

Fixed: **Block → Attribute → ElementShape → Async.**

Rationale:

1. **Block first.** Blocks introduce contextual bindings (`each` item/index,
   `await` value/error, snippet params). Downstream clusters need stable
   introducer identities to reference block-scoped bindings without AST
   reconstruction.
2. **Attribute second.** The largest consumer surface and the most walker-
   fragmented one. A stable attribute contract unblocks consolidation of the
   biggest hot-zone and feeds ElementShape (component props are attribute-
   shaped).
3. **ElementShape third.** Collapses the AST-node-kind dispatch in consumers
   into one semantic dispatch. Builds on stable Attribute identity for
   per-attribute lookups inside each element-shape variant.
4. **Async last.** Async is a decoration layer on top of the other three;
   each cluster must already expose stable node-id identity for async to
   attach without reaching back into AST.

## Migration Unit Within A Cluster

A cluster is migrated **one kind at a time, end-to-end**. Example for Block:
EachBlock through the full loop (builder support → answer variant → consumer
root migrated → old consumer-side assembly for EachBlock removed → tests
green), then IfBlock, then AwaitBlock, etc.

Valid migration unit:
- builder support for one kind
- public answer variant for that kind
- at least one real consumer migrated to the new answer for that kind
- old consumer-side meaning assembly for that kind removed
- tests covering the migrated path

## Deprecation Policy

`#[deprecated(note = "...")]` is the **opening** step of each kind migration,
applied to the old API surface for that kind before consumer rewrites begin.
Rationale: new code paths added during the migration must see the warning
and pick the new API; old call sites get flagged for mechanical cleanup. This
is stricter than the reactivity migration (which deferred deprecation until
after real consumer migration) and is chosen because the kinds here are
numerous and independently migratable — the warning is the safety rail that
keeps parallel ownership contained.

## Consolidation Principle

Reducing the number of template walker passes is a side-effect of this
migration, not a target on its own. Per cluster, the builder is expected to
fold work that is already logically co-located into one pass; cross-cluster
walker merges are only done when they fall out naturally from the new
ownership. By the end of the four clusters, the expectation is a substantial
reduction in walker passes, not a pre-declared count.

## Legacy Svelte 4 Interactions

Four of the open legacy specs reach into the semantic layer. The architecture
must not force them to invent a second classification system.

- **Legacy reactivity system** (`specs/legacy-reactivity-system.md`). All
  declaration/reference reactive classification stays in `reactivity_semantics`.
  Block Semantics exposes the identity of `{#each}` context bindings
  (introducer-side), which is exactly what the legacy-reactivity upgrade rule
  needs to read. No Block-side contract extension is required.
- **Legacy `export let` props + `$$props` / `$$restProps`**
  (`specs/legacy-export-let.md`). Legacy prop classification and synthetic
  legacy binding meaning (`$$props`, `$$restProps`, `$$slots`, `$$events`,
  `$$legacy`) are reactivity territory. Reference-site rewrites to
  `$.legacy_rest_props(...)` / sanitized `$$props` reads are handled by the
  transformer from `reference_semantics(reference_id)`. Attribute Semantics
  sees only generic Spread / Expression variants carrying a `ReferenceId` —
  it does not know the operand is a legacy props bag.
- **`$:` reactive assignments** (`specs/legacy-reactive-assignments.md`).
  Script-level. Not visible to any cluster here.
- **`on:event` legacy directive** (`specs/events.md`). Attribute Semantics
  `Event` variant carries the syntax form explicitly because legacy `on:` and
  modern `onclick` have different modifier/delegation rules.

General rule: legacy-specific marker types use explicit `Legacy` naming so
removal is mechanical (`grep LEGACY(svelte4)` → delete sites → compile),
matching the isolation constraint from `specs/legacy-reactivity-system.md:22-24`.
They never live inside non-legacy variant payloads.

`<svelte:self>` and `<svelte:component>` are ElementShape variants already;
they deprecate alongside Svelte 4 but fit cleanly into the shape enum.

## Documentation Scope

This document is the **only** doc for the semantic layer migration. It grows
in place as clusters land: each cluster adds its finalized answer shape, the
actual payload fields, and its migration state directly here. No per-cluster
spec files are created under `specs/` unless one cluster overflows its
session; in that case a per-cluster spec may be split off to track in-flight
state.

## Deprecated Surface

Surfaces marked deprecated as the first step of each kind migration:
- Block-specific `AnalysisData` classifications (per-kind, as each migrates)
- Attribute dynamism / ExpressionInfo bit combinations re-derived in consumers
- Element-kind AST dispatch in template traversal
- Async-specific side tables (`AsyncEmissionPlan`, pickled-await bookkeeping)

Deletion is gradual and per-kind; parallel ownership is contained by the
`#[deprecated]` warning.

## Prerequisite: Kill `FragmentItem`

The codegen consumer path in `svelte_codegen_client` does not walk the
Svelte AST directly. It walks `LoweredFragment { items: Vec<FragmentItem> }`
produced by `svelte_analyze`, where `FragmentItem` is an enum that
discriminates template items by AST node kind:

```rust
pub enum FragmentItem {
    Element(NodeId),
    ComponentNode(NodeId),
    IfBlock(NodeId),
    EachBlock(NodeId),
    AwaitBlock(NodeId),
    KeyBlock(NodeId),
    RenderTag(NodeId),
    HtmlTag(NodeId),
    SvelteElement(NodeId),
    SvelteBoundary(NodeId),
    SlotElementLegacy(NodeId),
    SvelteFragmentLegacy(NodeId),
    TextConcat { parts: Vec<LoweredTextPart>, has_expr: bool },
}
```

This enum conflates two very different things:

1. **Real lowering facts that AST does not carry.** `TextConcat` merges a
   run of adjacent `Text` / `ExpressionTag` / `Text` nodes into a single
   runtime `$.set_text` target; lowering also filters hoisted nodes
   (`SnippetBlock`, `SvelteHead`, `{@const}`, `{@debug}`, `<svelte:window>`,
   whitespace-only text) and normalizes sibling order per the reference
   compiler's whitespace rules. These are genuine additions over AST.
2. **Node-kind discrimination that Block / ElementShape Semantics now
   own.** Every `FragmentItem::*Block(id)` / `*Tag(id)` / `*Element(id)` /
   `ComponentNode(id)` / `SvelteBoundary(id)` variant is redundant with a
   one-query `block_semantics(id)` or `element_shape_semantics(id)` lookup.

The consequence: as long as `FragmentItem` is the codegen dispatcher, a
migrated cluster cannot produce a clean root consumer point. The Root
Consumer Migration Rule is violated by construction — every block-kind
emission currently starts at a `match FragmentItem::*` site, and a
`match block_semantics(id)` inside a single FragmentItem arm is a
meaningless narrowing (one-variant match inside a variant). The form the
architecture actually wants is:

```rust
for &node_id in fragment_plan {  // plan = filtered, ordered NodeIds + TextConcat pseudo-items
    match analysis.block_semantics(node_id) {
        BlockSemantics::Each(sem) => gen_each_block(ctx, node_id, sem, ...),
        BlockSemantics::If(sem)   => gen_if_block(...),
        BlockSemantics::Await(..) => gen_await_block(...),
        ...
        BlockSemantics::NonSpecial => match analysis.element_shape_semantics(node_id) {
            ElementShapeSemantics::Html(..) => process_element(...),
            ElementShapeSemantics::Component(..) => gen_component(...),
            ...
        }
    }
}
```

This is not achievable inside the Block Semantics migration. It requires
a separate initiative:

### Separate slice: **kill `FragmentItem`**

Scope:
- Reshape `LoweredFragment` to hold a plan of `NodeId`s (plus
  `TextConcat` as an explicit pseudo-node or side table) without
  duplicating node-kind discrimination.
- Replace `FragmentItem::*` matches across `svelte_codegen_client`
  (~250 call sites) and `svelte_analyze` consumers with Block / ElementShape
  Semantics queries, falling back to AST node-kind reads only where the
  cluster does not yet own the decision.
- Preserve lowering's genuine work: whitespace collapse, hoisted-node
  filtering, TextConcat merging, fragment-scoped flags consumed by
  codegen (`ContentStrategy`, `has_dynamic_children`, etc.).
- Remove `FragmentItem` once no consumer references it.

Ordering vs. the semantic clusters:
- **Precedes** end-to-end Block Semantics consumer migration. Until it
  lands, block-kind consumer migrations can only land as transitional
  `match block_semantics(id)` inserted inside the existing `FragmentItem`
  dispatcher, which is tolerated but not the target form.
- **Independent** of Attribute Semantics (attributes live on element
  nodes, not on fragment items).
- **Coordinates with** ElementShape Semantics: both need the semantic
  dispatcher in codegen; landing ElementShape and the FragmentItem kill
  in the same slice may be the simplest path.

Migration unit: not a cluster. A dedicated infrastructure slice with its
own spec. It does not add new semantic meaning; it removes a duplicated
dispatcher that blocks the cluster migrations from reaching their target
consumer shape.

Until this slice lands, the Block Semantics payload is built and unit
tested, but consumer code for block kinds either (a) reads the payload
from inside a FragmentItem arm (acceptable transitional form), or
(b) does not migrate its consumer at all and only lives as an
analyzer-side contract (preferred when no clean consumer path exists).

## Open Questions

Intentionally unresolved until per-cluster work starts:
- Exact payload fields of every draft answer variant.
- Whether ElementShape's `Html` vs `SvelteElement` is one variant with a flag
  or two — depends on how much payload overlap survives design.
- Per-component-prop identity vs per-attribute identity within
  `ComponentInvocationSemantics` (both are attribute `NodeId`s; the payload
  layout may duplicate).
- Async barrier representation for expressions embedded inside attribute and
  component-prop contexts — node identity is stable, barrier edge semantics
  are not yet shaped.
- Exact walker-consolidation steps per cluster. Declared as a side-effect
  above; concrete merges decided per session.
