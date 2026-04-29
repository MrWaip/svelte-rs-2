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
- **Consumer shape: Plan → builders → emit.** Inside a migrated branch the
  consumer body stays readable by resolving all decisions up front into a
  plain-data `Plan` struct (one pass over `sem` + source spans), then
  dispatching to small focused builders — one per piece of output (e.g.
  collection, key, fragment, fallback) — and finally emitting the call.
  Builders receive `&Plan`, not `&sem`, so they never re-interpret
  semantics. Reference implementation:
  [`crates/svelte_codegen_client/src/template/each_block.rs`](crates/svelte_codegen_client/src/template/each_block.rs)
  (`gen_each_block` / `EachPlan`). This is the form every migrated kind
  consumer should converge to.
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
- **`OxcNodeId` is the sole AST hook.** Cluster payloads carry `OxcNodeId`
  for any reference into JS AST; consumers resolve the statement /
  expression on demand through `JsAst::expr(id)` / `JsAst::stmt(id)`.
  The historical `StmtHandle` / `ExprHandle` parser-side indices (and the
  `ParserResult.stmts` / `.exprs` tables they pointed into) have been
  removed — `JsAst` is now keyed by `OxcNodeId` directly. Do not
  reintroduce a parallel handle system layered over the same AST: a
  second identity forces consumers to mix two systems and turns
  payloads into JSON-shuffling thin wrappers.
- **No binding-pattern repack.** `BindingPattern` subtrees
  (`$props`, `$state`, `$derived`, `{@const}`, `{#snippet}` params,
  `{#each … as pat}`, `{#await … then pat}`, `let:` directives, etc.)
  are **not** re-shaped into cluster-local DTOs. Every attempt — a flat
  `leaves: Vec<SymbolId>`, a `destructured: bool`, a parallel per-leaf
  struct — is a JSON repack of the OXC AST plus `ComponentSemantics`
  data that is already reachable by `OxcNodeId`. The cluster payload
  carries the `OxcNodeId` of the pattern (or its owning node) and
  nothing else; the consumer walks the pattern on demand via the shared
  `svelte_component_semantics::pattern::walk_bindings(&pat, |visit| …)`
  helper. This is the single traversal for every destructuring site in
  the pipeline — do not add a second one, do not cache its output.
  Rationale: the OXC `BindingPattern` is already the canonical form
  (symbols are attached to `BindingIdentifier.symbol_id`, defaults are
  attached to `AssignmentPattern.right`, rest is `ObjectPattern.rest` /
  `ArrayPattern.rest`). Any parallel structure must prove it carries
  a *classification* that AST does not — not a copy of AST data.
- **Lowering Boundary.** Answers may encode "what operation is required" but
  not "which runtime helper to call" / "which builder recipe to invoke".
- **Composite answers.** A cluster builder is expected to compose facts
  from multiple allowed inputs (AST, `ComponentSemantics`,
  `ReactivitySemantics`, analyzer-output tables) into one high-level
  decision that the consumer can read as a single variant or bit. The
  consumer must not reassemble the same decision from scattered
  low-level facts. Example: `EachFlags::ITEM_REACTIVE` is computed by
  the Block Semantics builder from a composition of
  "collection expression references an external binding" (scope check
  via `ComponentSemantics`), "expression has a store dependency" (via
  `ReactivitySemantics`), "key is the item identifier" (own payload),
  and "runes mode" — the codegen consumer sees one bit on the payload,
  never the four underlying facts.
- **Dependency Boundary.** Each cluster builds on `ComponentSemantics` and AST,
  not on legacy Svelte-specific classification tables. Concretely: a cluster
  builder's signature accepts only AST (`&Component` and `&JsAst` —
  the latter is the parser's pre-parsed JS store for template spans and is
  considered part of the AST surface, not a separate cluster),
  `ComponentSemantics`, `ReactivitySemantics`, and narrow analyzer-output
  tables that carry **generic** (non-cluster) facts consumed by multiple
  clusters — e.g. `BlockerData` (script-level async analysis: which
  symbols are blocked by which await barriers). These are distinguished
  from cluster side-tables (`EachContextIndex`, `TemplateSemanticsData`,
  `ExpressionInfo`) which must not be read. A builder never accepts
  `&AnalysisData` (full or partial borrow) and never reaches into the
  excluded legacy surfaces. Pipeline code assembles the builder's output
  into `AnalysisData`; the builder itself does not see it.
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

**Not a parallel cluster with its own identity.** Async is a decoration
over the other three clusters. Per-kind async facts ride inside the
owning cluster's payload, not behind a separate `async_semantics(id)`
query. This is the architectural refinement that came out of the
EachBlock slice: the original draft split Async into its own enum and
own query surface; in practice every async question is "how does this
*specific* block / attribute / element lower under async?", which is
precisely a field on that unit's semantic payload.

Scope:
- Pickled await expressions inside template / attribute / component-prop
  expressions.
- Async blockers and barrier placement.
- `{#await}` lowering differences under async mode.
- Top-level `await` in `<script>` (async-mode only).
- `$state.eager`, `$state.snapshot`, `$effect.pending` async interactions.

Out of scope:
- Non-async `{#await}` — owned by Block Semantics.
- Regular reactive state lifecycle — owned by `reactivity_semantics`.

Layout:
- Each cluster defines its own per-kind async payload variant. Example
  (already landed for `{#each}`): `EachBlockSemantics.async_kind:
  EachAsyncKind { Sync | Async { has_await, blockers } }`. Similar fields
  land on `IfBlockSemantics`, `AwaitBlockSemantics`, `HtmlBindSemantics`,
  `ComponentPropSemantics`, etc. when their slices migrate.
- Top-level `await` in `<script>` and the async-mode runtime harness
  stay as analyzer-wide output (e.g. `BlockerData`), not a cluster
  payload — they are component-global facts, not per-unit.
- No public `async_semantics(id)` query exists. Consumers read the
  async field from the cluster payload they already have in hand.

Identity: still the unit `NodeId` — but the answer lives on that unit's
cluster payload, not in a separate store.

Migration order: async is no longer "migrated last" as a distinct
cluster. Async fields are added to each cluster's payload during that
cluster's own slice, because without them the cluster's consumer still
has to route through legacy `AsyncEmissionPlan` / `ExpressionInfo` and
the Root Consumer Migration Rule fails. Each cluster gains its async
field end-to-end with the rest of its payload.

## Migration Order

Fixed: **Block → Attribute → ElementShape.**

Async is no longer a fourth step — see "Async Semantics" above. Each
cluster grows its own async payload field during its own slice.

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
- `ExpressionInfo` — the legacy per-expression bag-of-facts
  (`has_store_ref`, `has_await`, `ref_symbols`, etc.) that consumers
  previously assembled into semantic decisions. Replaced by
  `reactivity_semantics` for per-reference classification and by the
  owning cluster's payload for higher-level answers.
- Attribute dynamism / ExpressionInfo bit combinations re-derived in consumers
- Element-kind AST dispatch in template traversal
- Async-specific side tables (`AsyncEmissionPlan`, pickled-await bookkeeping)

Already removed:
- `FragmentItem` / `LoweredFragment` — the codegen-side dispatcher
  that conflated lowering facts with node-kind discrimination has been
  deleted. Codegen now walks fragment children through
  `svelte_codegen_client::codegen::fragment::prepare` (emit-time
  hoisting + whitespace normalization + `ContentStrategy`) without
  duplicating Block / ElementShape kind dispatch.

Deletion is gradual and per-kind; parallel ownership is contained by the
`#[deprecated]` warning.

## Historical: `FragmentItem` killed

The earlier prerequisite slice — replacing the analyze-side
`LoweredFragment { items: Vec<FragmentItem> }` dispatcher with a
codegen-side fragment plan that does not duplicate Block / ElementShape
node-kind discrimination — has landed.

Today fragment children are walked in
`svelte_codegen_client::codegen::fragment` (`mod.rs`, `prepare.rs`,
`process_children.rs`, `types.rs`). `prepare` does the genuine
lowering work — hoist structural nodes (snippets, const-tags,
debug-tags, `<svelte:head>`, `<svelte:window>` / `<svelte:document>` /
`<svelte:body>`, head titles), trim whitespace per Svelte rules,
coalesce adjacent `Text` + `{expression}` into a single `Concat`, and
classify the result into `ContentStrategy` — and dispatches each child
through Block Semantics + AST node-kind matches without going through a
parallel discriminator.

The target form for downstream cluster migrations is therefore already
in place: each child resolves through `block_semantics(node_id)`
first, then ElementShape (today still AST node-kind based, until that
cluster lands).

## Open: Кто отдаёт семантику для ExpressionTag в шаблонах

Текстовое выражение `{expr}` внутри шаблона (не внутри блока, не внутри
атрибута, а в теле фрагмента) — пока не закреплено ни за одним кластером.

Пример:

```svelte
<p>Hello {name}!</p>
```

`{name}` — это `ExpressionTag`-узел в children параграфа. Что консьюмеру
(transform / codegen) надо знать про этот узел:

- реактивное значение выражения (→ `reactivity_semantics`, через
  `ReferenceId` идентификаторов)
- async-статус выражения: содержит ли `await`, какие script-level
  блокеры нужно дождаться прежде чем его можно эвалюэйтить
- роль: читается ли оно как контент текстового узла, как аргумент
  клсикс-композиции и т. п.

Существующие кластеры не покрывают:

- **Block Semantics** — вне scope по документу (строки 162-163): `{@html}`
  и `{@debug}` явно out of scope, `ExpressionTag` не упомянут.
- **Attribute Semantics** — только выражения внутри атрибутов
  (`class={x}`, `value={x}`, конкатенации). Свободный ExpressionTag в теле
  фрагмента туда не попадает.
- **ElementShape Semantics** — про форму элемента, не про его содержимое.

Сейчас эти факты живут в `ExpressionInfo` (per-expression bag-of-facts,
помеченный как deprecated surface — строки 432-436) плюс в методах вида
`expression_blockers(node_id)` на `AnalysisData`. Это та самая
«scattered meaning assembly», от которой Consumer Model должна увести.

Варианты, не выбрано:

1. **Новый кластер InterpolationSemantics** (peer Block / Attribute /
   ElementShape). Query `interpolation_semantics(NodeId)` возвращает
   payload с async-фактами, ролью, flags. Плюсы: симметрично остальным
   кластерам. Минусы: плодит query surface, хотя большая часть фактов
   уже есть в `reactivity_semantics(reference_id)`.

2. **Расширить Block Semantics** варинтом `ExpressionTag(sem)`. Идёт
   против строки 166 («`{@html}`, `{@debug}` — direct emission, no
   block semantics»), но ExpressionTag ведёт себя иначе чем `{@html}` —
   это основной кирпич реактивного текста, а не escape-hatch.

3. **Всё на reactivity_semantics через `ReferenceId`**. Блокеры и
   async-статус выводятся из набора references выражения через уже
   существующий `reference_semantics(reference_id)` + `BlockerData`.
   Плюсы: ни одного нового кластера. Минусы: consumer должен сам
   агрегировать per-ref факты в per-expression ответ (те же грабли,
   что у `ExpressionInfo`).

4. **Deferred до Attribute Semantics.** Сначала закрываем Attribute
   (там `ExpressionAttribute`-семантика понадобится в любом случае), а
   потом решаем — переиспользовать ту же форму для ExpressionTag в
   теле, или ввести отдельный кластер.

Блокер миграции Attribute: attribute-level expressions и text-level
expressions имеют одинаковую природу (оба ExpressionTag-подобны, оба
читают тот же `ReferenceId`). Если Attribute получит собственную
expression-семантику не оглядываясь на текст — придётся либо
дублировать её для ExpressionTag, либо возвращаться и унифицировать.

Решение: до старта Attribute Semantics миграции — зафиксировать
контракт «кто владеет семантикой одного ExpressionTag NodeId».
Варианты выше — стартовая точка, не финал.

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
