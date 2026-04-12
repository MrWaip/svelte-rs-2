# Analyze Maturity Plan

Execution plan for `crates/svelte_analyze/src`.

This file is for implementation agents. Do not generalize it into a broad architecture rewrite.

If a desired change is not explicitly listed here, stop after the current slice and report the gap instead of inventing a new direction.

## Goal

Make these analyzer decisions canonical and reusable:

1. expression role
2. bind target semantics
3. fragment semantics needed by downstream readers
4. validator inputs as named facts instead of inline rediscovery

The target outcome is simple:

- analyzer computes the meaning once
- validators and downstream code read the meaning through named accessors
- fewer call sites combine raw flags and side tables manually

## Out Of Scope

- a generic query framework in the first slice
- a new lowering IR
- client runtime policy in `svelte_analyze`
- CSS analyzer or CSS prune refactors
- a broad rewrite of all validators at once
- splitting every test file up front
- SSR work

## Rules For The Agent

1. Start with concrete accessors and types, not abstract frameworks.
2. Do not add `analysis.query()` or `analysis.exprs()/elements()/fragments()` as a first step.
3. Do not move runtime helper choice, emission phase choice, or client-only policy into `svelte_analyze`.
4. Do not replace low-level fact tables if the new work can layer on top of them.
5. Do not split large files before the new facts they should consume actually exist.
6. If one slice uncovers a missing prerequisite, finish the current safe part and report the prerequisite.
7. Do not force all passes onto one traversal style.
8. Do not rewrite special-owner passes onto a shared visitor just for uniformity.
9. Manual recursion is allowed only where the pass owns scope or lowering order, or needs custom enter/leave control that shared helpers would obscure.
10. If a pass only asks local JS subtree questions, prefer a shared JS query helper over another one-off OXC visitor.

## Execution Order

Run the work in this order:

1. `ExprRole`
2. `BindTargetSemantics`
3. concrete fragment accessors
4. validator cleanup on top of those facts
5. invariants and semantic tests
6. string rediscovery reduction
7. pass ownership contracts
8. shared fragment traversal layer
9. shared JS query helpers
10. analyzer debug dump
11. recovery contracts

This order is intentional:

- slice 1 removes the most obvious multi-flag guessing
- slice 2 removes repeated bind-specific branching and string checks
- slice 3 gives downstream code stable fragment answers without a generic query layer
- slice 4 becomes much safer once the facts already exist
- slice 5 locks the new contracts in place
- slice 6 removes remaining semantic meaning hidden in ad hoc string checks
- slice 7 makes future pass changes harder to misplace
- slice 8 removes repeated hand-written fragment recursion from non-owner passes
- slice 9 removes repeated one-off OXC micro-visitors for the same local questions
- slice 10 gives humans and agents a direct way to inspect analyzer state
- slice 11 makes the new query surface safe on broken source

## Slice 1: `ExprRole`

### Problem

Expression meaning is currently spread across low-level facts from:

- [passes/js_analyze/expression_info.rs](crates/svelte_analyze/src/passes/js_analyze/expression_info.rs)
- [passes/js_analyze/dynamicity.rs](crates/svelte_analyze/src/passes/js_analyze/dynamicity.rs)
- [passes/js_analyze/needs_context.rs](crates/svelte_analyze/src/passes/js_analyze/needs_context.rs)
- [passes/js_analyze/render_tags.rs](crates/svelte_analyze/src/passes/js_analyze/render_tags.rs)
- [passes/js_analyze/async_blockers.rs](crates/svelte_analyze/src/passes/js_analyze/async_blockers.rs)

Call sites can end up reconstructing one semantic concept from several booleans.

### Required Deliverables

1. Add `ExprRole` in [types/data/expr.rs](crates/svelte_analyze/src/types/data/expr.rs).
2. The enum must start with this exact minimum set:
   - `Static`
   - `DynamicPure`
   - `DynamicWithContext`
   - `Async`
   - `RenderTag`
3. Store `ExprRole` in analyzer-owned data next to existing expression facts.
4. Add `AnalysisData::expr_role(node_id) -> Option<ExprRole>`.
5. Compute `ExprRole` inside existing analyze passes. Do not compute it in codegen.
6. Replace touched analyze-side call sites that currently combine 3 or more expression flags to infer one high-level meaning.

### Primary Files

- [types/data/expr.rs](crates/svelte_analyze/src/types/data/expr.rs)
- [types/data/analysis.rs](crates/svelte_analyze/src/types/data/analysis.rs)
- [passes/js_analyze/expression_info.rs](crates/svelte_analyze/src/passes/js_analyze/expression_info.rs)
- [passes/js_analyze/dynamicity.rs](crates/svelte_analyze/src/passes/js_analyze/dynamicity.rs)
- [passes/js_analyze/needs_context.rs](crates/svelte_analyze/src/passes/js_analyze/needs_context.rs)
- [passes/js_analyze/render_tags.rs](crates/svelte_analyze/src/passes/js_analyze/render_tags.rs)
- [passes/js_analyze/async_blockers.rs](crates/svelte_analyze/src/passes/js_analyze/async_blockers.rs)
- [tests.rs](crates/svelte_analyze/src/tests.rs)

### Before / After

Bad call-site shape:

```rust
if info.is_dynamic && info.needs_context && !info.has_await {
    ...
}
```

Target shape:

```rust
if analysis.expr_role(id) == Some(ExprRole::DynamicWithContext) {
    ...
}
```

### Do Not Do

- do not delete `ExpressionInfo`
- do not create a generic `ExprQueries` framework in this slice
- do not add client-only variants such as update-phase or runtime-helper categories
- do not replace every old call site immediately; only replace the touched repeated patterns

### Acceptance

- `ExprRole` is analyzer-owned and accessible through `AnalysisData`
- at least one repeated multi-flag decision is replaced by `expr_role`
- touched code no longer reconstructs “dynamic with context” or equivalent via several booleans

### Validation

- `just test-analyzer`
- add analyzer tests for at least:
  - static expression
  - dynamic pure expression
  - dynamic expression needing context
  - async expression
  - render tag expression

## Slice 2: `BindTargetSemantics`

### Problem

Bind meaning is still partly spread across:

- bind name string checks
- element-name checks
- special-host branching
- caller-side interpretation of low-level facts

This makes `bind:` logic easy to duplicate incorrectly.

### Required Deliverables

1. Add analyzer-owned bind semantics type in the data layer.
2. The type must answer these three questions directly:
   - what host family owns this bind target
   - what bind property family this is
   - whether the target must be mutable/reactive for the bind to make sense
3. The recommended shape is:

```rust
pub enum BindHostKind {
    Element,
    Window,
    Document,
    Body,
}

pub enum BindPropertyKind {
    Value,
    Checked,
    Group,
    Files,
    Media,
    Dimension,
    This,
    ContentEditable,
    Other,
}

pub struct BindTargetSemantics {
    pub host: BindHostKind,
    pub property: BindPropertyKind,
    pub requires_mutable_target: bool,
}
```

The exact type names may differ, but the stored meaning must match this shape.

4. Compute the semantics in [passes/bind_semantics.rs](crates/svelte_analyze/src/passes/bind_semantics.rs).
5. Keep [passes/binding_properties.rs](crates/svelte_analyze/src/passes/binding_properties.rs) as a static rule source, not a second semantic model.
6. Add `AnalysisData::bind_target_semantics(node_id) -> Option<&BindTargetSemantics>` or an equivalent accessor.
7. Replace touched analyze-side bind logic with the new accessor instead of raw string matching.

### Primary Files

- [passes/bind_semantics.rs](crates/svelte_analyze/src/passes/bind_semantics.rs)
- [passes/binding_properties.rs](crates/svelte_analyze/src/passes/binding_properties.rs)
- [types/data/analysis.rs](crates/svelte_analyze/src/types/data/analysis.rs)
- [types/data/mod.rs](crates/svelte_analyze/src/types/data/mod.rs)
- [passes/template_validation.rs](crates/svelte_analyze/src/passes/template_validation.rs)
- [tests.rs](crates/svelte_analyze/src/tests.rs)

### Before / After

Bad shape:

- caller matches `"value"` or `"checked"`
- caller checks if element name is `"input"` or `"svelte:window"`
- caller separately decides whether the target must be mutable

Target shape:

- caller asks for `bind_target_semantics(node_id)`
- caller branches on one typed answer

### Do Not Do

- do not move special-host runtime behavior into analyze
- do not encode client helper names or event names in bind semantics
- do not leave touched bind consumers doing new string checks outside the bind pass

### Acceptance

- touched bind consumers stop rediscovering host/property meaning from strings
- mutable-target requirements are read through a named accessor
- legality checks still belong to validation, not to bind data construction

### Validation

- `just test-analyzer`
- add analyzer tests for at least:
  - regular element `bind:value`
  - input `bind:checked`
  - `bind:this`
  - one window bind
  - one document or body bind

## Slice 3: Concrete Fragment Accessors

### Problem

Fragment-related answers exist, but a reader can still end up combining several internal structures to answer simple questions.

Do not solve this with a generic fragment framework first. Add the exact accessors that are already needed.

### Required Deliverables

Add direct `AnalysisData` accessors for these exact questions:

1. `node_fragment(node_id) -> Option<FragmentKey>`
2. `fragment_content_strategy(key) -> ContentStrategy`
3. `fragment_has_dynamic_children(key) -> bool`
4. `lowered_fragment(key) -> Option<&LoweredFragment>`
5. `fragment_blockers(key) -> &[u32]`

If some of these already exist under different names, reuse and normalize them instead of creating duplicates.

### Primary Files

- [types/data/analysis.rs](crates/svelte_analyze/src/types/data/analysis.rs)
- [types/data/fragments.rs](crates/svelte_analyze/src/types/data/fragments.rs)
- [types/data/fragment_facts.rs](crates/svelte_analyze/src/types/data/fragment_facts.rs)
- [types/data/template_data.rs](crates/svelte_analyze/src/types/data/template_data.rs)
- [passes/lower.rs](crates/svelte_analyze/src/passes/lower.rs)
- [passes/content_types.rs](crates/svelte_analyze/src/passes/content_types.rs)
- [tests.rs](crates/svelte_analyze/src/tests.rs)

### Do Not Do

- do not invent `FragmentQueries` in this slice
- do not add client-only fragment staging or output ordering
- do not move lowering policy into analyze

### Acceptance

- a touched reader can ask these fragment questions without opening unrelated tables directly
- there is one obvious accessor for each of the listed fragment questions

### Validation

- `just test-analyzer`
- add analyzer tests for at least:
  - static fragment
  - dynamic text fragment
  - fragment with lowered output
  - fragment with blockers

## Slice 4: Validator Cleanup On Top Of Canonical Facts

### Problem

The large validator files are hard to shrink safely until the required semantic facts already exist as named accessors.

This slice is not “split everything”. This slice is “split only the rules that can now read named facts”.

### Required Deliverables

1. Refactor [validate/runes.rs](crates/svelte_analyze/src/validate/runes.rs) first.
2. Split it into smaller modules only after `ExprRole` and any required accessors already exist.
3. Minimum split target:
   - `placement`
   - `calls`
   - `effects`
   - `props`
4. Keep the top-level entry point and diagnostic behavior stable.
5. In [passes/template_validation.rs](crates/svelte_analyze/src/passes/template_validation.rs), only move rules that can directly benefit from `BindTargetSemantics` or the fragment accessors from slice 3.
6. Do not do a full `template_validation` module tree rewrite in this slice.

### Primary Files

- [validate/runes.rs](crates/svelte_analyze/src/validate/runes.rs)
- [validate/mod.rs](crates/svelte_analyze/src/validate/mod.rs)
- [passes/template_validation.rs](crates/svelte_analyze/src/passes/template_validation.rs)
- [passes/template_validation/a11y.rs](crates/svelte_analyze/src/passes/template_validation/a11y.rs)
- [tests.rs](crates/svelte_analyze/src/tests.rs)

### Do Not Do

- do not split validators before the facts they should consume exist
- do not change diagnostic kinds or spans as part of structural cleanup
- do not create a second semantic model inside validators

### Acceptance

- touched validator rules read named analyzer facts instead of combining raw tables and flags
- `validate/runes.rs` is smaller and split by rule family
- `template_validation.rs` only shrinks where the new facts actually help

### Validation

- `just test-diagnostics`
- targeted diagnostic parity cases under `tasks/diagnostic_tests`

## Slice 5: Invariants And Semantic Tests

### Problem

Analyzer refactors are still protected mostly by output snapshots. That is too indirect for this kind of structural work.

### Required Deliverables

1. Add an analyzer invariant helper used in tests.
2. Start with these exact checks:
   - every node with `ExprRole::RenderTag` still has matching expression facts
   - every stored bind semantics entry belongs to a real indexed template node
   - every lowered fragment key is valid and reachable
   - every derived symbol dependency list refers to valid `SymbolId`s
3. Add semantic tests that assert analyzer facts directly, not only generated JS.
4. Keep these tests in [tests.rs](crates/svelte_analyze/src/tests.rs) unless the touched diff becomes hard to read.
5. Only split test files after the semantic helpers and first fact tests exist.

### Primary Files

- [tests.rs](crates/svelte_analyze/src/tests.rs)
- [types/data/analysis.rs](crates/svelte_analyze/src/types/data/analysis.rs)
- any touched files from slices 1-4

### Do Not Do

- do not start by rearranging the whole test tree
- do not write only output snapshots for these slices
- do not add weak invariants that simply restate one field equals itself

### Acceptance

- new analyzer facts have direct tests
- invariants fail fast when side tables drift apart
- analyzer cleanup is no longer validated only through final JS diffs

### Validation

- `just test-analyzer`
- any targeted compiler or diagnostic tests needed for touched code

## Slice 6: String Rediscovery Reduction

### Problem

Some semantic decisions are still inferred from raw strings outside the places where string-based classification is acceptable.

Allowed string-based zones:

- parser-side syntax extraction
- static rule tables

Problematic zones:

- downstream semantic logic that could already be driven by `SymbolId`, `NodeId`, or typed analyzer facts

### Required Deliverables

1. Add a short module-level contract comment in each touched file that still uses semantic string matching.
2. Treat these files as the primary cleanup targets:
   - [passes/collect_symbols.rs](crates/svelte_analyze/src/passes/collect_symbols.rs)
   - [utils/script_info.rs](crates/svelte_analyze/src/utils/script_info.rs)
   - [passes/binding_properties.rs](crates/svelte_analyze/src/passes/binding_properties.rs)
3. Keep parser-style string checks in `script_info` only where they are purely syntactic.
4. Replace touched analyzer-side semantic decisions in `collect_symbols` and later passes with:
   - `SymbolId` lookups
   - typed facts from earlier slices
   - static rule table lookups wrapped by named helpers
5. Add at least one regression test involving shadowed names, so equal identifier text is no longer enough to produce the same semantic outcome.

### Do Not Do

- do not try to eliminate every string literal in one pass
- do not move parser syntax detection out of `utils/script_info.rs` if it is still purely syntactic
- do not replace static rule tables with dynamic logic

### Acceptance

- touched downstream semantic code no longer decides meaning from raw identifier strings when an id-based or typed-fact path exists
- acceptable string-based zones are explicit and narrow
- at least one shadowing-style regression is covered by tests

### Validation

- `just test-analyzer`
- targeted tests for shadowing and store/rune-related classification where touched

## Slice 7: Pass And Traversal Ownership Contracts

### Problem

The pass graph exists, but “which pass owns which facts” is still too easy to learn by reading code instead of by reading an explicit contract.

That makes future analyzer work drift toward “I will just write this fact here because I already have the data”.

### Required Deliverables

1. Add an ownership contract section in [passes/mod.rs](crates/svelte_analyze/src/passes/mod.rs).
2. For each major pass touched by this plan, document:
   - primary inputs read
   - primary analyzer facts written
   - facts it is not allowed to own
   - traversal family it belongs to
3. At minimum, write ownership notes for:
   - `expression_info`
   - `dynamicity`
   - `bind_semantics`
   - `content_types`
   - `lower`
   - `template_validation`
4. Document these traversal families explicitly:
   - manual template recursion for scope/binding construction
   - `TemplateVisitor` for template-local passes
   - OXC `Visit` for JS subtree analysis
   - fragment/data traversal helpers for lowered or indexed analyses
5. Record this initial classification directly in the contract:
   - special-owner manual recursion:
     - [passes/build_component_semantics.rs](crates/svelte_analyze/src/passes/build_component_semantics.rs)
     - [passes/lower.rs](crates/svelte_analyze/src/passes/lower.rs)
   - `TemplateVisitor` family:
     - [passes/collect_symbols.rs](crates/svelte_analyze/src/passes/collect_symbols.rs)
     - [passes/reactivity.rs](crates/svelte_analyze/src/passes/reactivity.rs)
     - [passes/element_flags.rs](crates/svelte_analyze/src/passes/element_flags.rs)
     - [passes/bind_semantics.rs](crates/svelte_analyze/src/passes/bind_semantics.rs)
     - [passes/content_types.rs](crates/svelte_analyze/src/passes/content_types.rs)
     - [passes/template_side_tables.rs](crates/svelte_analyze/src/passes/template_side_tables.rs)
     - [passes/template_validation.rs](crates/svelte_analyze/src/passes/template_validation.rs)
     - [passes/js_analyze/render_tags.rs](crates/svelte_analyze/src/passes/js_analyze/render_tags.rs)
   - OXC `Visit` family:
     - [passes/mark_runes.rs](crates/svelte_analyze/src/passes/mark_runes.rs)
     - [passes/js_analyze/](crates/svelte_analyze/src/passes/js_analyze/)
     - nested visitors in `template_side_tables`
     - nested visitors in `template_validation`
     - `ResolvedRefCollector` in `collect_symbols`
   - fragment/data traversal family:
     - [passes/css_prune.rs](crates/svelte_analyze/src/passes/css_prune.rs)
     - [passes/css_prune_index.rs](crates/svelte_analyze/src/passes/css_prune_index.rs)
     - [passes/post_resolve.rs](crates/svelte_analyze/src/passes/post_resolve.rs)
     - lowered/data scans inside [passes/content_types.rs](crates/svelte_analyze/src/passes/content_types.rs)
     - [passes/executor.rs](crates/svelte_analyze/src/passes/executor.rs)
5. Add short `Writes:` and `Traversal:` comments or equivalent module-level notes in the touched pass files themselves.
6. For every touched pass in this slice, add one sentence explaining why its current traversal family is the correct one.
7. When a new analyzer-owned fact is introduced by this plan, name its owning pass in the contract.

### Do Not Do

- do not redesign the pass registry API for this slice
- do not add a generic metadata framework for pass ownership
- do not document every pass in the crate if it is untouched
- do not declare one traversal family “correct” for all passes

### Acceptance

- a new contributor can answer “who owns this fact?” from `passes/mod.rs` plus touched pass headers
- a new contributor can answer “why does this pass use this traversal style?” from the same contract
- touched facts introduced by this plan have one obvious owning pass
- touched passes have one explicit traversal family with a reason, not just a traversal API name
- future slices in this plan do not need to guess where a fact should be written

### Validation

- no separate test command required
- verify touched docs stay in sync with the implemented writes in the same diff

## Slice 8: Shared Fragment Traversal Layer

### Problem

Several non-owner passes recursively walk fragment trees by hand even though they are answering the same kind of question:

- recurse through `Fragment` children
- keep track of `FragmentKey`
- descend into child fragments of block-like or element-like nodes

This is currently repeated in places like:

- [passes/template_side_tables.rs](crates/svelte_analyze/src/passes/template_side_tables.rs)
- [passes/css_prune.rs](crates/svelte_analyze/src/passes/css_prune.rs)
- parts of [passes/lower.rs](crates/svelte_analyze/src/passes/lower.rs)

The first concrete duplicates to target are:

- `collect_fragment_facts_in` in `template_side_tables`
- `collect_rich_content_facts_in` in `template_side_tables`
- `collect_css_prune_edges_in_fragment` in `css_prune`
- `collect_const_tags_in` in `lower`, only if it fits without pulling main lowering into the helper

### Required Deliverables

1. Add one shared fragment-recursion helper under `crates/svelte_analyze/src/passes/` or another local analyzer-only module.
2. The helper must walk:
   - a `Fragment`
   - its owning `FragmentKey`
   - child fragments reachable through element-like and block-like nodes
3. Migrate at least two non-owner fragment-recursive passes to the helper.
4. The first migration targets should be chosen from:
   - fragment fact collection in `template_side_tables`
   - rich content fact collection in `template_side_tables`
   - CSS prune edge collection in `css_prune`
   - const-tag fragment collection in `lower`, only if it fits cleanly
5. Keep special-owner traversals out of scope for this helper in the first pass.
6. The helper may be callback-based or closure-based, but it must stay narrow:
   - enter current fragment
   - inspect current node
   - descend into child fragments with the next `FragmentKey`
7. Keep the first version analyzer-local. Do not make it a generic crate-wide traversal abstraction.

### Primary Files

- [passes/template_side_tables.rs](crates/svelte_analyze/src/passes/template_side_tables.rs)
- [passes/css_prune.rs](crates/svelte_analyze/src/passes/css_prune.rs)
- optionally one small helper use in [passes/lower.rs](crates/svelte_analyze/src/passes/lower.rs)
- one new helper module under `crates/svelte_analyze/src/passes/`

### Do Not Do

- do not rewrite [passes/build_component_semantics.rs](crates/svelte_analyze/src/passes/build_component_semantics.rs) onto this helper
- do not rewrite the main `lower_nodes` lowering traversal onto this helper
- do not force `TemplateVisitor` to serve lowered-fragment/data traversal
- do not add client-only semantics to the helper

### Acceptance

- at least two repeated fragment-recursive analyses share one traversal helper
- touched passes stop open-coding the same fragment descent rules
- touched passes no longer duplicate the same child-fragment match ladders by hand
- special-owner traversals remain separate where their scope/control-flow needs differ

### Validation

- `just test-analyzer`
- targeted analyzer tests for the touched fact families

## Slice 9: Shared JS Query Helpers

### Problem

There are several small OXC visitors that answer narrow JS subtree questions, but the same shapes keep getting reimplemented in different pass files.

Typical repeated questions:

- does this expression contain invalid assignment to each vars
- does this expression contain invalid assignment to snippet params
- collect binding names
- walk params but not function bodies

The first concrete duplicates to target are:

- `InvalidEachAssignmentVisitor` in `template_validation`
- `InvalidSnippetParamAssignmentVisitor` in `template_validation`
- local binding-name collection in `js_analyze/async_blockers`
- parameter-only marker visitors in `template_side_tables`

### Required Deliverables

1. Add one shared analyzer-local JS query module for small OXC helper visitors.
2. Move at least two repeated JS subtree queries out of large pass files into that module.
3. The first migration targets should be chosen from:
   - invalid each assignment detection in [passes/template_validation.rs](crates/svelte_analyze/src/passes/template_validation.rs)
   - invalid snippet param assignment detection in [passes/template_validation.rs](crates/svelte_analyze/src/passes/template_validation.rs)
   - local binding-name collection in [passes/js_analyze/async_blockers.rs](crates/svelte_analyze/src/passes/js_analyze/async_blockers.rs)
   - param-only marker traversal patterns in [passes/template_side_tables.rs](crates/svelte_analyze/src/passes/template_side_tables.rs)
4. Reuse existing helpers like [utils/binding_pattern.rs](crates/svelte_analyze/src/utils/binding_pattern.rs) where they already own part of the problem.
5. The first shared helpers must answer concrete questions, not expose a “visit anything” API:
   - collect binding names from binding patterns
   - detect invalid assignment to each vars
   - detect invalid assignment to snippet params
   - walk params without descending into nested function bodies

### Primary Files

- [passes/template_validation.rs](crates/svelte_analyze/src/passes/template_validation.rs)
- [passes/template_side_tables.rs](crates/svelte_analyze/src/passes/template_side_tables.rs)
- [passes/collect_symbols.rs](crates/svelte_analyze/src/passes/collect_symbols.rs)
- [passes/js_analyze/async_blockers.rs](crates/svelte_analyze/src/passes/js_analyze/async_blockers.rs)
- [utils/binding_pattern.rs](crates/svelte_analyze/src/utils/binding_pattern.rs)
- one new helper module under `crates/svelte_analyze/src/passes/` or `crates/svelte_analyze/src/utils/`

### Do Not Do

- do not move full semantic analysis into this helper module
- do not create a giant generic visitor framework
- do not merge unrelated JS analyses just because they all use OXC `Visit`

### Acceptance

- touched pass files lose at least two one-off micro-visitors or local duplicate subtree queries
- repeated JS subtree questions have one obvious helper location
- OXC `Visit` is still used, but through shared local query helpers where repetition existed
- touched files stop adding new custom visitors for the exact questions listed above

### Validation

- `just test-analyzer`
- targeted tests for the touched validation or analysis behavior

## Slice 10: Analyzer Debug Dump

### Problem

There is no single compact human-readable dump of analyzer facts for a component.

Without that, refactors are debugged indirectly through final JS or ad hoc local prints.

### Required Deliverables

1. Add a debug dump helper under `crates/svelte_analyze/src` for analyzer-owned facts.
2. The first version must dump at least:
   - `ExprRole`
   - bind target semantics
   - fragment key per relevant node
   - fragment content strategy
   - lowered fragment presence
   - derived symbol dependencies
3. Keep the output stable and sorted by `NodeId` and `SymbolId` where relevant.
4. Expose the dump in the smallest safe surface:
   - test helper
   - internal debug helper
   - or `#[cfg(test)]` utility
5. Add at least two tests that snapshot or assert against the dump for small components.

### Primary Files

- [types/data/analysis.rs](crates/svelte_analyze/src/types/data/analysis.rs)
- [tests.rs](crates/svelte_analyze/src/tests.rs)
- one new helper module in `crates/svelte_analyze/src/`

### Do Not Do

- do not make this a public stable crate API unless a later need appears
- do not dump raw AST payloads
- do not include client-only codegen state

### Acceptance

- analyzer fact inspection no longer requires reading final JS or adding one-off debug prints
- the dump covers the new canonical facts from slices 1-3
- test failures can point directly at semantic drift

### Validation

- `just test-analyzer`
- dump-based tests for at least two small focused components

## Slice 11: Recovery Contracts

### Problem

As more named accessors are added, it becomes unclear which ones are guaranteed to work on broken source and which ones may legitimately return `None`.

Mature compilers define these contracts explicitly so diagnostics and downstream readers do not panic or guess.

### Required Deliverables

1. For every accessor introduced in slices 1-3, add doc comments describing recovery behavior on invalid or partially analyzed source.
2. Use one of these explicit outcomes per accessor:
   - always available after analyze
   - available only when owning fact exists
   - returns empty slice / `None` on recovery path
3. Add tests that run analyze on invalid components and assert the accessor contract rather than relying on absence of panic.
4. Start with invalid cases relevant to this plan:
   - invalid rune placement
   - invalid bind target
   - broken fragment-producing template
5. Make touched validators rely on these safe access patterns instead of assuming full fact availability.

### Primary Files

- [types/data/analysis.rs](crates/svelte_analyze/src/types/data/analysis.rs)
- [validate/runes.rs](crates/svelte_analyze/src/validate/runes.rs)
- [passes/template_validation.rs](crates/svelte_analyze/src/passes/template_validation.rs)
- [tests.rs](crates/svelte_analyze/src/tests.rs)

### Do Not Do

- do not weaken working invariants just to make invalid-code paths “work”
- do not hide real bugs by swallowing every missing fact silently
- do not treat panic-free behavior as sufficient without an explicit accessor contract

### Acceptance

- new accessors document their recovery semantics
- invalid-source tests assert expected `None`/empty/available behavior
- touched validators can consume the new accessors safely on error paths

### Validation

- `just test-analyzer`
- `just test-diagnostics`
- targeted invalid-source cases under `tasks/diagnostic_tests` where validator behavior is touched

## Explicit Wrong Directions

If an implementation agent goes in one of these directions, it is off-plan:

- introducing a generic `query` framework before adding the concrete accessors above
- inventing a new analyzer IR to represent client lowering phases
- moving client event/action/helper policy into analyzer facts
- mass-splitting validators before canonical facts exist
- replacing `ExpressionInfo` entirely instead of layering `ExprRole` on top
- forcing all passes onto `TemplateVisitor` or one shared traversal style
- rewriting special-owner traversals just for stylistic consistency
- rewriting `build_component_semantics` or main `lower` recursion to fit a generic traversal first
- adding new hand-written fragment recursion in non-owner passes when the shared helper exists
- adding new one-off OXC micro-visitors for an already-shared JS subtree query
- touching CSS analysis as part of this work

## Stop Conditions

Stop and report instead of continuing if:

1. a required accessor would need client-only policy to answer correctly
2. a typed fact starts becoming a disguised lowering IR
3. a validator cleanup requires a new semantic fact that is not in the current slice
4. the work expands into unrelated feature completion

## Definition Of Done

This document is complete when:

- `ExprRole` exists and is used by touched consumers
- `BindTargetSemantics` exists and removes touched string-based bind rediscovery
- the listed fragment questions are answered through direct accessors
- touched validators read canonical facts instead of re-deriving meaning
- touched passes have explicit traversal-family ownership with a documented reason
- non-owner fragment-recursive passes share a traversal helper where repetition existed
- repeated local JS subtree queries have a shared helper location
- special-owner traversals remain special-owner traversals
- invariants and semantic tests protect the new analyzer contracts
