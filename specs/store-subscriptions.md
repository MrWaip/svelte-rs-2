# $store subscriptions

## Current state
- **Working**: 17/20 use cases
- **Tests**: 18/19 green
- Last updated: 2026-04-29

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

- [x] Basic top-level store subscription in template and script reads (test: `store_basic`)
- [x] Direct writable store assignment rewrites to `$.store_set` (test: `store_write`)
- [x] Compound writable store assignment rewrites to `$.store_set(current op value)` (test: `store_compound_template`)
- [x] Update-expression rewrites to `$.update_store` / `$.update_pre_store` (test: `store_update_template`)
- [x] Deep member assignment rewrites to `$.store_mutate` (test: `store_deep_mutation`)
- [x] Deep member update rewrites to `$.store_mutate` (test: `store_deep_update`)
- [x] Runtime plan marks store setup without forcing component context by default
- [x] Dev-mode store setup validates the underlying store with `$.validate_store` (test: `store_validate_dev`)
- [x] Store cleanup runs after `$.pop(...)` without becoming unreachable when stores and component return values coexist (test: `store_validate_dev`)
- [x] Each-block with store-backed collection sets correct flags (`EACH_ITEM_REACTIVE`, no `EACH_ITEM_IMMUTABLE`) (test: `store_each_invalidate`)
- [ ] Reassigning a store binding unsubscribes the prior subscription with `$.store_unsub` — legacy-only (test: `store_reassign_unsub`)
- [ ] `{#each $items as item}` mutations invalidate the backing store with `$.invalidate_store` — legacy-only (test: `store_each_invalidate`)
- [x] Component/store binding codegen marks store-backed bindings with `$.mark_store_binding` (test: `store_mark_binding`)
- [x] Analyzer rejects subscriptions to stores not declared at component top level (test: `validate_store_invalid_scoped_subscription`)
- [x] Analyzer warns when `$name` shadows a rune (`store_rune_conflict`) (test: `validate_store_rune_conflict`)
- [x] Rune self-declarations like `let state = $state("")` must not emit a false-positive `store_rune_conflict` warning (diagnostic test: `state_rune_no_store_rune_conflict`)
- [ ] When a local binding `state` is initialized with a `$state(...)` rune call, OTHER `$state(...)` calls in the same scope must not emit a `store_rune_conflict` warning either (diagnostic test: `state_rune_no_conflict_with_other_rune_calls`)
- [x] Analyzer rejects `$store` reads inside `<script module>` (test: `validate_store_invalid_subscription_in_module`)
- [x] Module compilation rejects `$store` reads outside `.svelte` components
- [x] Analyzer emits store-specific diagnostics (`StoreInvalidScopedSubscription`, `StoreInvalidSubscription`, `StoreInvalidSubscriptionModule`, `StoreRuneConflict`)

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

## Test cases

- [x] `store_basic`
- [x] `store_write`
- [x] `store_assign_template`
- [x] `store_compound_template`
- [x] `store_update_template`
- [x] `store_deep_mutation`
- [x] `store_deep_update`
- [x] `store_validate_dev`
- [x] `store_reassign_unsub`
- [x] `store_each_invalidate`
- [x] `store_mark_binding`
- [x] `state_rune_no_store_rune_conflict`
- [ ] `state_rune_no_conflict_with_other_rune_calls`
- [x] `validate_store_invalid_scoped_subscription` (analyzer unit test)
- [x] `validate_store_invalid_subscription_in_module` (diagnostic parity test)
- [x] `validate_store_rune_conflict` (analyzer unit test)
- [x] `analyze_module_reports_store_invalid_subscription_module` (analyzer unit test)
- [x] `module_store_subscription_reports_module_diagnostic_for_js` (`compile_module()` unit test)
- [x] `module_store_subscription_reports_module_diagnostic_for_ts` (`compile_module()` unit test)
- [x] e2e smoke (legacy): `smoke_legacy_reactive_mutations_all` — covers every assignment + update operator (`=`, `+=`, `-=`, `++`, `--`, `++` prefix, `--` prefix, `&&=`, `||=`, `??=`) for `$store` identifier and deep `$store.x` member targets — including static (`obj.x`), computed string (`obj["x"]`), and computed dynamic (`obj[key]`) member access — in both script body and template expressions in legacy mode, including legacy coarse-wrap parity (`($store(), $.untrack(() => …))`).
- [x] e2e smoke (runes): `smoke_runes_reactive_mutations_all` — covers every assignment + update operator for `$store` identifier and deep `$store.x` member targets — including static (`obj.x`), computed string (`obj["x"]`), and computed dynamic (`obj[key]`) member access — in both script body and template expressions in runes mode.
