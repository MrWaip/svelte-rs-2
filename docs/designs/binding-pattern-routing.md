label: binding-pattern-routing

# PRD: binding-pattern-routing — centralized exhaustive declaration unfold

Child PRD of the `reactivity-semantics` cluster. Sibling to `destructure-patterns`: that one owns
traversal canonicalization (`walk_bindings` / `walk_assignment_targets`) and the assignment axis; this
one owns the **declaration side** — a single centralized full-depth unfold of `BindingPattern` and the
cleanup of the analysis payload. At one point it **refines** `destructure-patterns`: its invariant
"analysis does not change" is revoked on the declaration axis — the emit-form and the pattern-tree
mirror are stripped from the payload.

The Original for the "what": `original/compiler/` — `extract_paths` (two-pass pattern path parsing) and
the unfold of rune/props/legacy declarations. The architecture is ours.

## Purpose

A single compiler answer to "what does each destructuring declaration unfold into", valid for all
pattern forms and all reactive kinds. Today the declaration unfold is smeared across independent
one-level entries — a separate path for `$state`/`$derived`-destructure, for `$props()`, for
legacy-state, plus its own one-level parsing of template contexts in codegen (each-item, await
value/error, snippet params, `{@const}`, the `let:` carrier). Each entry covers only part of the forms:
nesting is lost, keys and defaults are handled inconsistently. Specifically in codegen the each-block
context is parsed one level deep and silently drops the nested pattern (`{#each items as [a,[b,c]]}`).

The write side is already reduced to five exhaustive dispatchers over `reference_semantics`; the
declaration side has no centralized unfold. This PRD establishes a **single full-depth recursive
traversal** that both stages flow through.

## Kind map (what we cover) — fork-set from the Original

Declaration axis, dispatch by domain kind down to the leaf (`declarator_semantics` /
`binding_semantics`):

- `$state` / `$state.raw`-destructure — at the leaf, the choice "signal carrier vs lowering to plain"
  by the binding's domain semantics (`OptimizedRune`);
- `$derived` / `$derived.by`-destructure — sync vs async (async cannot be extracted from the pattern, it
  stays a domain fact);
- `$props()` — object / rest;
- legacy `export let`-destructure;
- `{@const}` declaration;
- `let:` directive (carrier);
- each-item, including nested (`{#each items as [a, [b, c]]}`);
- await value / error (`{#await … then { a, [b] }}`);
- snippet params.

Every kind — against the full `BindingPattern` form checklist (`bindings-and-references` §4): leaf ·
object prop · alias · computed key · nested object/array · array hole · object-rest · array-rest ·
default on a leaf · default on an intermediate node.

Also applies to standalone `.svelte.js` / `.svelte.ts` modules (`compile_module`): the routing of JS
declarators is the same; there are no template contexts there, so the codegen side does not touch them.

## Public API

Analysis exposes no new extension points — the three existing ones remain (`binding_semantics`,
`declarator_semantics`, `reference_semantics`) plus the canonical traversal `walk_bindings` (owned by
`destructure-patterns`). `declarator_semantics` becomes the **single declaration-kind axis**: analysis
records a kind on every declarator — real (props, legacy, `$state`/`$derived`-destructure) and synthetic
template-context (each-item, await value/error, snippet param, `{@const}`, `let:`). This enriches the
existing query, it is not a new extension point. Per-leaf semantics (`binding_semantics`) stays for the
"signal carrier vs plain" choice at the leaf; it no longer carries the declaration kind.

The payload form changes, not the set of points:

- `DeclaratorSemantics` carries the domain declaration kind (including the rune destructure kinds and the
  synthetic template ones), but **without pattern-form mirrors** — the leaf list and the
  has-rest flag leave the variants: the consumer obtains them by traversal. What remains is the domain
  that traversal cannot recover (declaration kind `const` / `let` / `var`, sync/async for derived).
- `DerivedEmit` loses the destructured-form variants (`DestructuredInlineSource` /
  `DestructuredInlinePropsSource` / `DestructuredBoxedSync` / `DestructuredBoxedAsync`); the domain
  `Sync` / `Async` remains. The unfold form is decided by the consumer: destructuring is taken from
  traversing the LHS, the source kind from inspecting the initializer, "source = the whole rest-props"
  from a direct `binding_semantics` query on the source symbol.

Consumer machinery (not analysis API):

- **Two entry points (doors), one per stage** — the sole unfold entry in its crate:
  - transit (`svelte_transform`): public `rewrite_binding_declarations(stmts)` — walks a statement list
    and, per `VariableDeclaration`, calls `rewrite_declaration(decl) -> Vec<Statement>` which iterates
    **all** declarators and dispatches each by `declarator_semantics`;
  - codegen (`svelte_codegen_client`): public `emit_binding_pattern(declarator)`.
  Each is an exhaustive `match` over `declarator_semantics`, dispatching to a **private** kind handler;
  the kind handler is not visible from outside and is never called directly. The shared full-depth
  traversal is the existing `walk_bindings`; there is no shared unfold procedure across both stages
  (the outputs are incompatible — see invariant 5). Each kind handler calls `walk_bindings` itself and
  assembles its own output.

  **Approach revision (transit door, recorded after slice 4).** The transit door is no longer a
  per-`VariableDeclaration` in-place mutation (`rewrite_binding_pattern(declarator)`); it is a
  **statement-returning** door at the statement-list level. Forced by **async `$derived`-destructure**:
  at the top level its leaves depend on a shared `$$d` temp, so the Original (and our parity target)
  emits **one block** inside `$.run([async () => { … }])`; our codegen `$.run` builder splits independent
  top-level declarators into separate async arrows, which breaks the temp dependency. A block statement
  cannot be produced by mutating a `VariableDeclaration`'s declarator list in place, so the door returns
  `Vec<Statement>` instead. Consequences: (a) async-derived is a **branch inside the same door** (a
  block at top level, a declarator list inside a function) — there is no satellite async statement pass;
  (b) the door iterates **all** declarators of a declaration (sync kinds accumulate into one grouped
  declaration via a `flush`, async-derived flushes the pending group then emits its own block), so the
  earlier single-declarator `[0]` assumption and its reliance on `split_top_level_multi_declarators` are
  gone. The shared leaf unfold (`unfold_carrier_access` + `walk_bindings`) is identical for sync and
  async; only the framing differs (declarator vs assignment-in-block). The codegen door
  (`emit_binding_pattern`) is unaffected.
- Dumb shared access-form builders (e.g. the `$.to_array` carrier expression) live in the existing
  `svelte_emit_builders` and are pulled in by the "second consumer → extract" rule. No new crate.

## Architectural invariants

1. **One centralized unfold per stage.** In its crate — exactly one public door
   (`rewrite_binding_declarations` in transit, `emit_binding_pattern` in codegen); the kind handlers are
   private and reachable only through it. The full-depth traversal in both is the shared `walk_bindings`.
   No parallel one-level parsing in the script/template traversals. The transit door iterates all
   declarators of a declaration and returns `Vec<Statement>` (see "Approach revision" above): async
   `$derived`-destructure is a branch inside it, not a separate statement pass.
2. **Dispatch by kind — single via `declarator_semantics`, exhaustive in both stages.** Analysis records
   the domain kind on every declaration declarator — the real JS script declarator and the synthetic
   template-context declarator (each-item, await value/error, snippet param, `{@const}`, `let:`). Both
   stage entries dispatch by `declarator_semantics` into a kind handler; a new kind = a compile error in
   the `match` at **all** entries, exactly as adding a `reference_semantics` variant breaks the five
   write dispatchers. This is the "smart analyzer / dumb consumer" discipline: the kind classification is
   given by analysis, the consumer only matches — it does not re-derive the kind from rune detection and
   leaf semantics on its side.
3. **The consumer assembles the form.** Analysis carries neither emit-form nor a pattern-tree mirror. The
   structure (paths, defaults, rest, computed keys, aliases, excluded keys for object-rest) is taken from
   `walk_bindings`; the wrapper choice (carrier-array vs native destructuring inside `$.derived` vs
   passthrough) is a local structural decision of the consumer.
4. **Semantics — only the hard-to-recover domain.** Test criterion: "is this recoverable by traversing
   `BindingPattern`?". Yes → the consumer takes it from `walk_bindings`, we do not put it in the payload.
   No (needs whole-component analysis: leaf reactivity, mutations, async-ness, mode) → the fact lives in
   per-id semantics.
5. **Sharing boundary — the traversal plus the dumb builders.** What is shared is the traversal
   `walk_bindings` (in `ComponentSemantics`) and the dumb form builders in `svelte_emit_builders`. There
   is no shared unfold procedure across both stages, nor a "dispatch trait": the outputs of transit
   (a `Vec<Statement>` of rewritten declarations / blocks) and codegen (block runtime) are incompatible.
   Each stage is its own door with
   its own `match`; only the traversal and the form builders are shared.
6. **Refinement of `destructure-patterns`.** Its invariant "analysis does not change" holds on the
   assignment axis but is revoked on the declaration axis: the emit-form and the tree mirror are stripped
   from the payload — precisely because they violate "no emit-form in analysis" and "flat per-id
   semantics".

## Unfold (client emit)

The heart of the unfold in each door is traversing `BindingPattern` via `walk_bindings`: at each leaf the
kind handler queries `binding_semantics(symbol)` and assembles the form itself from the leaf's domain
meaning plus the path structure (`path`, `is_rest`, default at a step, excluded keys for object-rest).
The recursion runs full-depth — nested patterns are no longer lost. Leaf forms:

- reactive leaf (signal carrier) — unfold via a `$.to_array` carrier with reuse of the temp array by
  path prefix and a chain of `$.derived`;
- native destructuring inside `$.derived` with a projection — for forms the Original does not unfold into
  a carrier;
- passthrough — for a non-reactive leaf (lowering to plain).

Two doors, both dispatch by `declarator_semantics`; foreign kinds → named arms in `unreachable!`, no
catch-all `_`:

- **Transit** (`rewrite_binding_declarations` → `rewrite_declaration`) serves script declarations.
  Iterates all declarators; `match` over `declarator_semantics` of each → private kind handler
  (`rewrite_state`, the sync `rewrite_derived`, `rewrite_async_derived`, `rewrite_props`,
  `rewrite_legacy_state`), which sets the access root and traverses the pattern. Output — `Vec<Statement>`:
  sync kinds accumulate into a grouped declaration, async-derived emits its own block (top level) or
  declarator-list declaration (inside a function). Template kinds → `unreachable!`.
- **Codegen** (`emit_binding_pattern`) serves template contexts. `match` over `declarator_semantics` of
  the synthetic context declarator → private kind handler (each / await / snippet / `{@const}` /
  `let:`), which sets the element access root (synthetic `$$item`, the await value/error, the snippet
  param) and traverses the pattern. Output — block runtime. Script kinds → `unreachable!`.

## Rejected alternatives

Recorded so they are not re-litigated in later sessions.

- **One shared entry for both stages.** Rejected: the outputs are incompatible — transit mutates the JS
  declarator in place, codegen emits block runtime. A single entry point would break the stage boundary.
  In its place — two doors (`rewrite_binding_declarations` / `emit_binding_pattern`), with only the
  `walk_bindings` traversal shared.
- **Pull template contexts into transit for the sake of a single entry.** Rejected: it moves block
  runtime emit into transit, breaking "transit mutates JS / codegen emits runtime" for cosmetics.
- **Heterogeneous dispatch: transit by `declarator_semantics`, `$state`/`$derived` by leaf semantics,
  template ones at the block emitter call site.** Rejected: it smears the declaration kind across three
  axes, yields no single exhaustive `match`, and forces the consumer to re-derive the kind from rune
  detection and leaf semantics (analysis in the consumer). The decision — analysis records the kind on
  every declarator, both entries dispatch by `declarator_semantics`. The synthetic template-context
  declarator (each-item, await, snippet, `{@const}`) is a kind carrier on par with the real one;
  `EachItemKind` / `ContextualBindingSemantics` stay for per-leaf semantics, not for the declaration kind.
- **A shared dispatch trait contract between stages.** Rejected: the per-stage emit bodies differ,
  there is nothing to share; "centralized + recursive" is guaranteed by the shared `walk_bindings` in
  each door, and "exhaustive" by the `match` over `DeclaratorSemantics` in both doors (foreign kinds →
  `unreachable!`).
- **A second dispatch axis by emit form (carrier / native / passthrough).** Rejected: the form is not an
  analysis axis — the consumer assembles it from the leaf's domain facts. Keeping the form in the payload
  = "emit-form of semantics".
- **Keep the `DerivedEmit` form variants and the form mirrors in `DeclaratorSemantics`.** Rejected: this
  is exactly the carrier of the anti-pattern the feature removes (see invariant 6).

## Conventions and verified implementation facts

- **Names.** The doors are `rewrite_binding_declarations` (transit; per-declaration helper
  `rewrite_declaration -> Vec<Statement>`) and `emit_binding_pattern` (codegen), both public, the sole
  entry in their crate. The kind handlers are private: transit — `rewrite_<kind>` (`rewrite_state`,
  `rewrite_derived`, `rewrite_async_derived`, `rewrite_props`, `rewrite_legacy_state`, …), codegen —
  `emit_<kind>`. Not `handle_*`. Foreign kinds — a named `unreachable!`, no `_` arm (otherwise a new
  kind would not break the `match`).

- **Migration marking.** Old entries are marked `#[deprecated = "superseded by binding-pattern-routing"]`
  without `#[allow]` wrappers: the deprecation gate does not fail the build — `[workspace.lints.rust]
  deprecated = "allow"` in the root `Cargo.toml` plus `-A deprecated` in `.cargo/config.toml` (verified).
  The list of remaining `#[deprecated]` is a live migration checklist; zero = migration complete.

- **Skeleton shape (slice 1).** Decided: build the new mechanism alongside the old one, migrate one
  consumer at a time. Slice 1 establishes both doors — `rewrite_binding_pattern` (transit) and
  `emit_binding_pattern` (codegen), each with an exhaustive `match` over the kind: own kinds — stub arms
  `unimplemented!()`, foreign kinds — `unreachable!`. The old entries are all marked `#[deprecated]` at
  once and keep driving behavior; the suite is green trivially. Slices 2–9 switch one consumer at a time
  onto its door (filling the arm), slice 10 deletes the old.
  The alternative "move all kinds in slice 1" is rejected: it front-loads regression risk into a slice
  that itself moves zero reds.

- **Slice order.** The red corpus under the tag is marked into four kinds (slices 2–5): each-item
  (codegen), await value/error (codegen), `$state`/`$derived`-destructure (transit), legacy state/props
  (transit). The mostly-green kinds (`$props()`-destructure, `{@const}`, `let:`, snippet params) move
  onto the shared procedure one at a time (slices 6–9); they come **after** the reds so numbers 2–5 in
  the corpus marking do not shift. Without these slices coverage is silently narrowed — the old paths of
  the green kinds cannot be deleted, invariant 1 cannot be closed. Caveat: `let:` (slice 8) is not fully
  green — its default and array-rest forms are red against a reference quirk (see the test corpus
  section), so slice 8 is green→green only on the no-default / no-array-rest forms.

- **Branch stubs.** `todo!()` MUST NOT be used — `[workspace.lints.clippy] todo = "deny"` would fail
  `just lint`. An own not-yet-done kind — `unimplemented!()` (the same loud semantics, not banned). A
  foreign kind (lives in the other stage) — `unreachable!` with an explanation: this is not a stub but
  the invariant "we don't reach here" (`unreachable` is not banned — only `todo` and `unwrap_used` are
  denied, verified in `Cargo.toml`). In slice 1 neither executes: the new mechanism is not called by
  anyone yet.

- **Dead-code in slice 1.** The not-yet-called `rewrite_binding_pattern` / `emit_binding_pattern` under
  `-D warnings` trip `dead_code`; we put `#[allow(dead_code)]` on each door until the slice where its
  first consumer switches over and removes the `allow` (for `emit_binding_pattern` — slice 2, each).

- **Codegen today (verified).** The each-block context parsing in `svelte_codegen_client` is by hand and
  one-level: on a nested element the parse returns "no name" and silently drops the node; `$.to_array`
  and index access are assembled on the spot, without `walk_bindings`. This is exactly the machinery that
  slice 2 replaces with a call to the shared recursive procedure.

## Target test corpus (`/dig`)

The cases that `/dig` registered for this PRD live under `tasks/compiler_tests/cluster_cases/*/declaration`
(213 cases total). They are the slice acceptance gate: a slice is done when its kind's `declaration`
cases go green through the new door.

Most kinds instantiate the same **18-form checklist** (one case per `BindingPattern` form):
`alias` · `array_hole` · `array_of_objects` · `array_rest` · `array_rest_nested` · `computed_key` ·
`default_intermediate_array` · `default_intermediate_object` · `default_leaf_array` · `default_leaf_object` ·
`flat_object` · `nested_array` · `nested_object` · `object_in_array_in_object` · `object_of_arrays` ·
`object_rest` · `single_array` · `string_key`.

Per kind (→ slice):

- **each-item** (→ slice 2): `each/declaration/{legacy,runes}` — 18-form checklist each (36).
- **await value/error** (→ slice 3): `await/declaration/{legacy,runes}` — 18-form checklist each (36).
- **`$state`-destructure** (→ slice 4): `runes/state/declaration` — 18-form checklist (18).
- **`$derived`-destructure** (→ slice 4): `runes/derived/declaration` — bespoke set (11):
  `array_with_object` · `boxed_nested_array` · `default_nested_array` · `derived_by_nested` ·
  `flat_object` · `nested_array` · `nested_object` · `object_of_arrays` · `rest_nested_array` ·
  `single_array` · `string_key`.
- **legacy state** (→ slice 5): `legacy/state/declaration` — 18-form checklist (18).
- **legacy props** (→ slice 5): `legacy/props/declaration` — bespoke set (11):
  `child_alias_weird_sample` · `gibberish_key_default_source` · `gibberish_key_nonsource` ·
  `identifier_key_nonsource_guard` · `main_alias_weird_sample` · `nested_array_props` ·
  `numeric_key_default_source` · `numeric_key_nonsource` · `numeric_key_rest_after` ·
  `shorthand_nonsource_guard` · `string_valid_ident_nonsource_guard`.
- **`$props()`-destructure** (→ slice 6, green): `runes/props/declaration` — bespoke set (11):
  `alias` · `bindable` · `bindable_alias` · `bindable_default` · `default_alias` · `default_leaf` ·
  `flat_object` · `mixed_rest_default` · `rest` · `single` · `string_key`. No nested forms — nested
  `$props()` is a compile error (`props_invalid_pattern`), so the form surface is flat by construction.
- **`{@const}`** (→ slice 7, green): `const_tag/declaration/{legacy,runes}` — 18-form checklist each (36).
- **`let:`** (→ slice 8): `legacy/let_directive/declaration` — 17 cases from the checklist (12 green,
  5 expected-red, ignore-tagged `binding-pattern-routing#8`). `default_leaf_object` is omitted —
  `{ a = 1 }` in a `let:` value is a reference parse error (shorthand-with-default not valid there). The
  5 reds are a reference-compiler quirk in legacy `let:`: with any default the reference yields an empty
  projection (`return {}`) and leaves the leaf reads as free variables (and even drops the
  `template_effect`), and array-rest is dropped from the projection. Our output is "more correct", so we
  diverge — matching means reproducing the reference quirk. So `let:` is **not** cleanly green: the
  no-default / no-array-rest forms are green, defaults and array-rest are red.
- **snippet params** (→ slice 9, green): `snippets/declaration/{legacy,runes}` — 18-form checklist each (36).

## Server emit

Stub — awaits `/audit`. Covers the declaration unfold on the server pipeline and how its rules differ
from the client.

## Document links

- `analyze.md` (smart analyzer / dumb codegen, BindingPattern handling), `transform.md` (the five write
  dispatchers as a template), `codegen.md`, `supporting-crates.md` (`svelte_emit_builders`).
- `context.md` — §"Reactivity", §"Scopes, bindings, references", §"Synthetic binding / declaration",
  §"Emit-form of semantics" (anti-pattern).
- `docs/bindings-and-references.md` §4 (form checklist), §5 (`walk_bindings`, traversal discipline),
  §6 (flat per-id semantics).
- `docs/reactivity-semantics.md` — the root PRD of the axis.
- `docs/designs/destructure-patterns.md` — the sibling PRD (traversals, assignment axis); this document
  refines its invariant "analysis does not change" on the declaration axis.
- `original/compiler/` — `extract_paths` and the declaration unfold for the "what".
