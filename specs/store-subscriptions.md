# $store subscriptions

## Current state
- **Working**: 13/16 client use cases
- **Fixed this session**:
  - `$.mark_store_binding()` in component bind getter for store-backed bindings
  - `store_invalid_scoped_subscription` diagnostic for nested-scope store refs
  - `store_rune_conflict` warning for `$name` that shadows a rune
- **Missing**: 3 — `$.store_unsub` (legacy-only), `$.invalidate_store` (legacy-only), `$store` in `<script module>` / module compilation
- **Next**: legacy-mode features when non-runes infrastructure is added; module-level diagnostics when dual-script AST is available
- Last updated: 2026-04-03

## Source

- ROADMAP item: `$store` subscriptions
- User request: `/audit $store`

## Syntax variants

- Store read in script: `$count`
- Store read in template: `{$count}`
- Direct assignment: `$count = value`
- Compound assignment: `$count += value`
- Update expressions: `$count++`, `--$count`
- Deep mutation: `$store.foo = value`, `$store.foo++`
- Store usage inside event handlers and template expressions
- Store-driven each expressions: `{#each $items as item}`
- Store values passed into component props / bindings
- Invalid nested or non-top-level store declarations
- Invalid store reads in module context or `.svelte.js`/`.svelte.ts` module compilation

## Use cases

- [x] Basic top-level store subscription in template and script reads
- [x] Direct writable store assignment rewrites to `$.store_set`
- [x] Compound writable store assignment rewrites to `$.store_set(current op value)`
- [x] Update-expression rewrites to `$.update_store` / `$.update_pre_store`
- [x] Deep member assignment rewrites to `$.store_mutate`
- [x] Deep member update rewrites to `$.store_mutate`
- [x] Runtime plan marks store setup without forcing component context by default
- [x] Dev-mode store setup validates the underlying store with `$.validate_store`
- [x] Store cleanup runs after `$.pop(...)` without becoming unreachable when stores and component return values coexist
- [x] Each-block with store-backed collection sets correct flags (`EACH_ITEM_REACTIVE`, no `EACH_ITEM_IMMUTABLE`)
- [ ] Reassigning a store binding unsubscribes the prior subscription with `$.store_unsub` (legacy-only)
- [ ] `{#each $items as item}` mutations invalidate the backing store with `$.invalidate_store` (legacy-only)
- [x] Component/store binding codegen marks store-backed bindings with `$.mark_store_binding`
- [x] Analyzer rejects subscriptions to stores not declared at component top level
- [x] Analyzer warns when `$name` shadows a rune (`store_rune_conflict`)
- [ ] Analyzer rejects `$store` reads inside `<script module>`
- [ ] Module compilation rejects `$store` reads outside `.svelte` components

## Reference

- Reference compiler docs:
  - `reference/docs/06-runtime/01-stores.md`
- Reference compiler analyze:
  - `reference/compiler/phases/2-analyze/index.js`
  - `reference/compiler/errors.js`
  - `reference/compiler/warnings.js`
- Reference compiler client transform:
  - `reference/compiler/phases/3-transform/client/transform-client.js`
  - `reference/compiler/phases/3-transform/client/visitors/Program.js`
  - `reference/compiler/phases/3-transform/client/visitors/EachBlock.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/utils.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/declarations.js`
- Reference compiler server transform:
  - `reference/compiler/phases/3-transform/server/transform-server.js`
  - `reference/compiler/phases/3-transform/server/visitors/AssignmentExpression.js`
  - `reference/compiler/phases/3-transform/server/visitors/UpdateExpression.js`
  - `reference/compiler/phases/3-transform/server/visitors/shared/utils.js`
- Rust implementation:
  - `crates/svelte_analyze/src/utils/script_info.rs`
  - `crates/svelte_analyze/src/passes/collect_symbols.rs`
  - `crates/svelte_analyze/src/passes/post_resolve.rs`
  - `crates/svelte_analyze/src/scope.rs`
  - `crates/svelte_analyze/src/validate/mod.rs`
  - `crates/svelte_transform/src/lib.rs`
  - `crates/svelte_transform/src/rune_refs.rs`
  - `crates/svelte_codegen_client/src/lib.rs`
  - `crates/svelte_codegen_client/src/script/traverse/assignments.rs`
  - `crates/svelte_codegen_client/src/script/traverse/runes.rs`
  - `crates/svelte_diagnostics/src/lib.rs`
  - `tasks/compiler_tests/cases2/store_*`

## Tasks

- [ ] Analyze: add a dedicated store validation pass after scoping resolves store candidates
  - Emit `StoreInvalidScopedSubscription`, `StoreInvalidSubscription`, `StoreInvalidSubscriptionModule`, and `StoreRuneConflict`
  - Decide whether module-context enforcement belongs in `analyze` vs `analyze_module`, but keep diagnostics sourced from analysis data rather than codegen heuristics
- [ ] Analyze: preserve enough dependency metadata for template codegen to know when an each block or component binding depends on a store subscription
- [ ] Client codegen: emit `$.validate_store` in dev-mode store setup
- [ ] Client codegen: preserve store cleanup when `$.pop` must return exports in dev/custom-element-style paths
- [ ] Client codegen: emit `$.store_unsub` when a subscribed store binding is reassigned
- [ ] Client codegen: emit `$.invalidate_store` for each blocks whose collection depends on a store subscription
- [ ] Client codegen: emit `$.mark_store_binding` for component/attribute binding paths that reference store subscriptions
- [ ] Tests: keep existing happy-path store snapshots, add focused missing snapshots for each missing client behavior, and cover diagnostics in analyzer/unit tests if compiler snapshots cannot express them cleanly

## Implementation order

1. Add analyzer diagnostics for scoped/module misuse and rune ambiguity.
2. Port dev-mode `validate_store` because it is isolated and already captured by a focused failing compiler case.
3. Port reassignment teardown and each-block invalidation because both need explicit codegen ownership over store lifetimes.
4. Port component binding marking once component/binding paths are stable enough to compare with reference output.
5. Add module-analysis coverage for non-component store usage.

## Discovered bugs

- FIXED: client codegen sets up store subscriptions but never emits `$.validate_store` in dev mode.
- FIXED: when stores coexist with `$.pop($$exports)`, generated cleanup is emitted after `return` and becomes unreachable.
- OPEN: the analyzer validation entrypoint only runs rune validation today, so store-specific diagnostics are currently absent.
- OPEN: no Rust codegen path currently emits `$.invalidate_store` or `$.store_unsub` (legacy-only features).

## Test cases

- Existing covered compiler cases:
  - `store_basic`
  - `store_write`
  - `store_assign_template`
  - `store_compound_template`
  - `store_update_template`
  - `store_deep_mutation`
  - `store_deep_update`
  - `store_validate_dev` — dev-mode `$.validate_store` + correct `$$cleanup()` ordering
  - `store_reassign_unsub` — store variable reassignment in runes mode (no legacy state promotion)
  - `store_each_invalidate` — each block with store-backed collection, correct flags
  - `store_mark_binding` — `bind:value={$count}` on component with `$.mark_store_binding()` in getter
- Analyzer unit tests:
  - `validate_store_invalid_scoped_subscription` — `$count` where `count` declared in nested function
  - `validate_store_rune_conflict` — local `state` binding + `$state()` call warns about ambiguity
