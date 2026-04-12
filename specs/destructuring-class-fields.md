# Destructuring & class fields

## Current state
- **Working**: 13/13 use cases
- **Tests**: 16/16 green
- Last updated: 2026-04-03

## Source

ROADMAP.md — runes/script item `Destructuring & class fields`

## Syntax variants

- Script destructuring:
  - `let { a, b } = $state(expr)`
  - `let [a, b, ...rest] = $state(expr)`
  - `let { a, b } = $state.raw(expr)`
  - `let { a, b } = $derived(expr)`
  - `let [a, b, ...rest] = $derived(expr)`
  - `let { a, b } = $derived.by(fn)`
- Class fields:
  - `field = $state(expr)` / `field = $state.raw(expr)`
  - `#field = $state(expr)` / `#field = $derived.by(fn)`
  - `field = $derived(expr)` / `field = $derived.by(fn)`
  - `this.field = $state(expr)` / `this.field = $derived.by(fn)` as the first constructor assignment

## Use cases

- [x] `$state` object destructuring (test: `state_destructure`)
- [x] `$state` array destructuring (test: `state_destructure`)
- [x] `$state.raw` object destructuring (test: `state_raw_destructure_object`)
- [x] `$state.raw` array destructuring (test: `state_raw_destructure_array`)
- [x] Sync `$derived(expr)` object destructuring (test: `derived_destructured_object`)
- [x] Sync `$derived(expr)` array/rest destructuring (test: `derived_destructured_array`)
- [x] Sync `$derived.by(fn)` destructuring (test: `derived_destructured_by`)
- [x] `$state` public class field (tests: `state_class_field`, `state_class_multiple`)
- [x] `$state` private class field (test: `state_private_class_field`)
- [x] `$state` constructor assignment (tests: `state_class_constructor`, `state_class_field_constructor_assign`, `state_class_constructor_proxy`)
- [x] `$state.raw` class field (tests: `state_raw_class_field`, `state_class_raw_field`)
- [x] `$derived` public class field (test: `derived_class_field`)
- [x] `$derived.by` class fields including constructor-assigned public fields (test: `derived_by_class_fields`)

## Reference

- `reference/docs/02-runes/02-$state.md` — destructuring caveat, class-field forms, constructor assignment
- `reference/docs/02-runes/03-$derived.md` — destructured `$derived` semantics and class-field support
- `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js` — destructured `$state` / `$derived` lowering
- `reference/compiler/phases/3-transform/client/visitors/ClassBody.js` — class-field lowering
- `reference/compiler/utils/ast.js` — `extract_paths`
- `crates/svelte_codegen_client/src/script/state.rs` — destructuring expansion, async derived destructuring, class-field rewrite
- `crates/svelte_codegen_client/src/script/traverse/derived.rs` — `wrap_derived_thunks`, currently identifier-only for sync `$derived`
- `crates/svelte_codegen_client/src/script/traverse/runes.rs` — base rune init rewrites
- `tasks/compiler_tests/test_v3.rs` — compiler test inventory and ignores
- `tasks/compiler_tests/cases2/derived_by_class_fields/` — focused missing class-field case from this audit

## Test cases

- [x] `state_destructure`
- [x] `state_raw_destructure_object`
- [x] `state_raw_destructure_array`
- [x] `state_class_field`
- [x] `state_private_class_field`
- [x] `state_class_constructor`
- [x] `state_class_multiple`
- [x] `state_class_field_constructor_assign`
- [x] `state_class_constructor_proxy`
- [x] `state_raw_class_field`
- [x] `state_class_raw_field`
- [x] `derived_class_field`
- [x] `derived_destructured_object`
- [x] `derived_destructured_array`
- [x] `derived_destructured_by`
- [x] `derived_by_class_fields`
