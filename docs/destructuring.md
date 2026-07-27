# PRD: Destructuring

label: destructuring
topics: destructuring, destructure patterns, BindingPattern, AssignmentTarget, walk_bindings, walk_assignment_targets, destructure declaration unfold, destructure assignment, object-rest/array-rest, default/alias/computed key, props destructure/alias, binding_origin_key, OriginKind, each-item/await/snippet/const/let destructure, writeback, each-item writeback place

Child PRD of `reactivity-semantics`, sibling to `state-rune`. One compiler answer to "how do I reach the
value of each binding/target of a destructuring pattern, and what do I do with it" across three axes:
declaration (`BindingPattern`), assignment (`AssignmentTarget`), and the `$props()` key form (how a prop
binding is identified in `$$props`).

The rule that shapes all three: one canonical traversal per axis, one centralized unfold entry per stage,
and no destructuring structure in analysis — analysis publishes flat per-id facts, the consumer
reconstructs the pattern by traversal. Applies equally to components and to standalone
`.svelte.js` / `.svelte.ts` modules (`compile_module`); template contexts (each-item, await, snippet,
`{@const}`, `let:`) exist only in components.

The Original for the "what": `original/compiler/` — `extract_paths`, `build_assignment`, the
declaration/props unfold. The architecture is ours.

## Pattern traversal (foundation)

Two canonical traversals live in `ComponentSemantics`, one per axis — the **only** sanctioned way to walk a
pattern. A manual `match` over the pattern tree is allowed to recognize a specific form, never to walk it.

- **`walk_bindings`** — `BindingPattern` traversal (declaration axis). Full-depth recursive; nesting is
  never lost. Per leaf it yields the leaf symbol, the path from the root, the default at a step, the rest
  flag, and the excluded keys for object-rest.
- **`walk_assignment_targets`** — `AssignmentTarget` traversal (write axis), same yield shape. Target
  identity is a reference id (identifier) or the member's root reference; the write axis carries no binding
  symbols.
- **`walk_assignment_target_idents`** — a thin filter over `walk_assignment_targets` for consumers that
  only need plain identifier targets (e.g. legacy `$:` dependency tracking).

Form checklist every traversal must handle (`bindings-and-references` §4): leaf · object prop · alias ·
computed key · nested object/array · array hole · object-rest · array-rest · default on a leaf · default
on an intermediate node.

## Declaration axis

One centralized full-depth unfold per stage: every destructuring declaration — script and template —
routes through a single exhaustive entry, so nesting is never dropped and the N independent one-level
parsers are gone.

### Two doors (one per stage)

- **Transform** (`svelte_transform_client`): `rewrite_binding_declarations` — the sole public entry. Walks a
  statement list and, per declaration, iterates all declarators and dispatches each by
  `declarator_semantics` to a private kind handler. Serves script kinds; template kinds → `unreachable!`.
- **Codegen** (`svelte_codegen_client`): `emit_binding_pattern` — the sole public entry. Dispatches by
  `declarator_semantics` to a private kind handler. Serves template kinds; script kinds → `unreachable!`.

Kind handlers are private and reachable only through the door — nobody can bypass it. This holds for the
single-identifier form (`const a = $rune(…)`) too: analysis records `declarator_semantics` for it, so it
dispatches by kind like any declarator, the single identifier being the degenerate `walk_bindings`.

The transform door returns statements, not an in-place mutation — forced by **async `$derived`-destructure**:
at the top level its leaves depend on a shared temp, so the Original emits one block inside
`$.run([async () => { … }])`, which a declarator-list mutation cannot produce. Async-derived is thus a
branch inside the same door (a block at top level, a declarator-list declaration inside a function), not a
separate pass. Statement granularity follows the Original: one declaration per source declarator only at
the top level of the instance body; nested declarations (`let x, y;` inside a function) stay grouped.

### Single kind axis: `declarator_semantics`

`declarator_semantics` records the domain kind on **every** declarator — the real JS script declarator and
the synthetic template-context declarator (each-item, await value/error, snippet param, `{@const}`,
`let:`), which are already materialized as synthetic JS declarations in the AST. `binding_semantics` keeps
only the per-leaf "signal carrier vs plain" choice.

Kinds (dispatch down to the leaf):

- `$state` / `$state.raw`-destructure — at the leaf, "signal carrier vs lowering to plain" by domain
  semantics;
- `$derived` / `$derived.by`-destructure — sync vs async (async can't be extracted from the pattern, it
  stays a domain fact);
- `$props()` — object / rest (flat by construction, see §Known divergences);
- legacy `export let`-destructure;
- `{@const}`; `let:` carrier; each-item (incl. nested, `{#each items as [a, [b, c]]}`); await value/error
  (`{#await … then { a, [b] }}`); snippet params.

### Payload cleanup (refinement of "analysis does not change")

- `DeclaratorSemantics` carries the domain kind but **no pattern-form mirror** (leaf list, has-rest flag
  leave the variants — the consumer traverses). What stays is the non-recoverable domain: `const`/`let`/`var`,
  sync/async for derived.
- `DerivedEmit` keeps only `Sync` / `Async` and loses the destructured-form variants.

## Assignment axis

Destructuring assignment (`AssignmentTarget` on the LHS). The parity fork it closes: in runes mode a
destructuring assignment to signals must not write past the signal, else the write bypasses the signal and
the UI does not update.

Forms (fork-set from the Original):

- array/object pattern as target; RHS literal vs RHS identifier (wrapper form);
- default on a leaf and on an intermediate node (`[a = 5]`, `[{ a } = {}]`) — `$.fallback`;
- array-rest (`[a, ...b]`) — `$.to_array` without a count + `.slice(n)`; object-rest (`{ a, ...rest }`) —
  `$.exclude_from_object`;
- nesting (`[[a]]`, `[{ a }]`), array holes; computed key (`{ [k]: a }`);
- mixed targets (`[obj.x, a]`) — a member target next to a signal target;
- value position (`let x = ([a] = arr)`) — `return` the RHS from the wrapper;
- async (`[a, b] = [1, await …]`, `[a = await …]`) — async wrapper + `await`, lazy `$.fallback`;
- guard: no target reactive (`[x, y] = [1, 2]` from locals) — pattern left untouched;
- guard: a single identifier (`a = 5`) — the ordinary write dispatcher, outside the unfold.

Each target routes through the five write dispatchers over `reference_semantics`; the unfold engages only
if at least one target is reactive (signal / store / derived / legacy-state). Analysis does not change on
this axis — destructuring-assignment leaves are already classified write. Legacy mode is served by the same
unfold, dictated by the target's semantics, not a separate branch.

## Props key axis

One answer to "how is this prop binding identified in `$$props`", for every `BindingProperty` key form
under a `$props()` ObjectPattern:

- shorthand (`let { foo } = $props()`) — alias = binding name;
- ident key (`let { foo: x } = $props()`);
- string literal that is a valid JS identifier (`let { 'foo': x } = $props()`);
- non-standard string literal (`let { 'a-b': x } = $props()`);
- numeric literal (`let { 0: x } = $props()`).

A computed key (`let { [expr]: x } = $props()`) is a separate validation error, out of scope.

### Public API

- **`binding_origin_key`** (`ComponentSemantics`) — resolves "how is this binding identified at its
  destructure source". Given a binding symbol, returns a domain-neutral alias (name string) plus
  `OriginKind`, from a walk-up over the declaration node's parents to the nearest `BindingProperty`. Serves
  every destructure family, not just `$props()`.
- **`OriginKind`** — `Ident | String | Numeric`. Source of truth for the analysis variant choice
  (`Ident` → static, `String | Numeric` → computed) and the literal form in the Source-declarator emit.
- **`ReferenceSemantics::PropRead` / `PropReferenceSemantics`** (`ReactivitySemantics`) — `Source` (thunk
  call by local name), `NonSourceStatic` (`$$props.<ident>`), `NonSourceComputed` (`$$props[<literal>]`).
  Payload identity is the binding symbol only; no strings, no node pointers.

## Architectural invariants

1. **One traversal per axis.** `walk_bindings` (declaration), `walk_assignment_targets` (write). No manual
   `match` over the pattern tree for traversal — only to recognize a form (`bindings-and-references` §5).
2. **One centralized unfold entry per stage.** Declaration: one public door per crate
   (`rewrite_binding_declarations`, `emit_binding_pattern`), private handlers behind it. Assignment: a
   single write-side entry. No parallel one-level parsing in the script/template traversals.
3. **Dispatch by kind — exhaustive.** `declarator_semantics` is the single declaration-kind axis, dispatched
   by both doors; the assignment axis dispatches through the five write dispatchers over
   `reference_semantics`. A new kind/variant is a compile error everywhere (no wildcard `_`).
4. **Form is the consumer's, not analysis's.** Analysis carries no emit-form and no pattern-tree mirror; the
   consumer takes structure (paths, defaults, rest, computed keys, aliases, excluded keys) from the
   traversal and picks the wrapper (carrier-array vs native `$.derived` vs passthrough; sequence vs wrapper
   function vs async wrapper). This **refines** "analysis does not change": on the declaration axis the
   emit-form/tree mirror were stripped from `DeclaratorSemantics`/`DerivedEmit`; on the assignment axis
   analysis is unchanged (write leaves already marked).
5. **Semantics — only the non-recoverable domain fact.** Recoverable by traversing the pattern → the
   consumer takes it from the traversal; needs whole-component analysis (leaf reactivity, mutations,
   async-ness, mode) → a flat per-id fact keyed by the binding symbol. Identity by id; the alias comes from
   `binding_origin_key` on the spot.
6. **Generic vs domain.** Traversal and key-form recognition are generic JS (`ComponentSemantics`); the
   reactivity classification (Source / NonSourceStatic / NonSourceComputed; signal carrier vs plain) is
   domain (`ReactivitySemantics`). One named variant per decision — so the dispatcher is a flat exhaustive
   `match`, no compound conditions.
7. **Sharing boundary — the traversal plus the dumb builders.** Shared across stages: the traversals and the
   dumb access-form builders in `svelte_emit_builders` (`$.to_array` / `$.fallback` /
   `$.exclude_from_object` / carrier expressions). No shared unfold procedure and no dispatch trait — the
   stage outputs are incompatible (transform emits statements, codegen emits block runtime).
8. **The each-item writeback place is one shape both stages agree on up front.** A legacy `{#each}`
   destructured binding is written back through the place assembled from its path — and two stages emit it:
   the transform for an identifier assignment (`count = 5` in a handler), codegen for a `bind:` setter. So
   the carrier identifiers the place names are reserved once in `TransformData` before the transform runs
   and both stages read them; generating them per stage puts the same each-block behind two different
   `$$array` names. Representability is one predicate, `has_each_item_writeback_place` (a rest leaf or a
   slice-terminated path gets no place — `adr/0008`); a second copy per stage drifts from the ADR.
   A computed key prints **raw** in the place while the read prints the transformed key —
   `$.get($$item)[key] = v` next to `let value = () => $.get($$item)[key()]` — a fork of the Original's
   `extract_paths`, which builds its `update_expression` from the untransformed key.

## Client emit

**Declaration.** Each door traverses via `walk_bindings`; at each leaf the handler queries
`binding_semantics` and assembles the form from the leaf's domain meaning plus the path. Leaf forms:
reactive leaf → `$.to_array` carrier (temp-array reuse by path prefix) + a chain of `$.derived`; native
destructuring inside `$.derived` for forms the Original does not carry; passthrough for a non-reactive leaf.

**Assignment.** Through `walk_assignment_targets`; per target the access expression is assembled from the
path (`$.to_array` with element count / `.slice` for array-rest / member / computed / `$.exclude_from_object`
/ `$.fallback` for defaults). The result is wrapped as a sequence (object pattern with a simple identifier
RHS), a wrapper function, or an async wrapper (when RHS or defaults contain `await`). The setter per target
is the identifier or member write dispatcher (a signal → `$.set(.., needs_proxy)` by the ordinary
`should_proxy` rule).

**Props key.** `NonSourceStatic` → `$$props.<ident>`; `NonSourceComputed` → `$$props[<literal>]` (the
alias string; numeric key emits `$$props["0"]` per the Original); `Source` → `<binding_name>()` via the
`$.prop` result. Source declarator → `$.prop($$props, <key>, …)`, `<key>` numeric for `OriginKind::Numeric`
else string. Rest-excluded names come from `binding_origin_key`, not from re-reading the AST.

## Rejected alternatives

- **One shared entry / a shared dispatch trait across the declaration stages.** The stage outputs are
  incompatible (transform emits statements, codegen emits block runtime) — a single entry would break the
  stage boundary and there is nothing to share. In its place: two doors, only the traversal shared.
- **Pull template contexts into transform for a single entry.** Moves block-runtime emit into transform,
  breaking "transform mutates JS / codegen emits runtime" for cosmetics.
- **Heterogeneous declaration dispatch** (transform by `declarator_semantics`, runes by leaf semantics,
  template ones at the block-emitter call site). Smears the kind across three axes and forces the consumer
  to re-derive it — instead, analysis records the kind on every declarator and both doors dispatch by it.
- **A second dispatch axis by emit form** (carrier / native / passthrough). The form is not an analysis
  axis; the consumer assembles it from the leaf's domain facts.
- **A shared runtime access-builder for the assignment axis.** Deliberately avoided: each unfold assembles
  the access from the path itself — less coupling, no cross-axis path-form co-evolution.
- **Re-open the `$props()` key form in transform** (walk the key node to re-discover it). "Analysis in
  codegen" — the alias/`OriginKind` comes from `binding_origin_key`.

## Document links

- `reactivity-semantics.md` — root PRD (the five write dispatchers); `state-rune.md` — sibling.
- `analyze.md` — "smart analyzer / dumb consumer"; `transform.md`, `codegen.md`, `supporting-crates.md`
  (`svelte_emit_builders`); `component-semantics.md` — where the traversals live.
- `bindings-and-references.md` — §4 (form checklist), §5 (traversals), §6 (flat per-id semantics).
- `context.md` — §"Reactivity", §"Scopes, bindings, references", §"Emit-form of semantics" (anti-pattern).
- `original/compiler/` — `extract_paths`, `build_assignment`, `get_prop_source`, `regex_is_valid_identifier`
  for the "what".
