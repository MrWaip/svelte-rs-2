# Destructuring & class fields

## Current state
- **Complete**: All use cases implemented and covered by passing compiler tests.
- `$state` / `$state.raw` destructuring, `$state` / `$derived` class fields, and all sync `$derived` destructuring cases pass.
- `$derived.by` class fields with constructor assignment fixed: bare placeholder declaration is now skipped in `rewrite_class_body`, and constructor-assigned fields are pre-emitted at the top of the class body (matching reference output order).
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

1. [x] `$state` object destructuring (covered, test: `state_destructure`)
2. [x] `$state` array destructuring (covered, test: `state_destructure`)
3. [x] `$state.raw` object destructuring (covered, test: `state_raw_destructure_object`)
4. [x] `$state.raw` array destructuring (covered, test: `state_raw_destructure_array`)
5. [x] Sync `$derived(expr)` object destructuring (test: `derived_destructured_object`)
6. [x] Sync `$derived(expr)` array/rest destructuring (test: `derived_destructured_array`)
7. [x] Sync `$derived.by(fn)` destructuring (test: `derived_destructured_by`)
8. [x] `$state` public class field (covered, tests: `state_class_field`, `state_class_multiple`)
9. [x] `$state` private class field (covered, test: `state_private_class_field`)
10. [x] `$state` constructor assignment (covered, tests: `state_class_constructor`, `state_class_field_constructor_assign`, `state_class_constructor_proxy`)
11. [x] `$state.raw` class field (covered, tests: `state_raw_class_field`, `state_class_raw_field`)
12. [x] `$derived` public class field (covered, test: `derived_class_field`)
13. [x] `$derived.by` class fields including constructor-assigned public fields (test: `derived_by_class_fields`)

## Reference

- Reference compiler:
  - `reference/docs/02-runes/02-$state.md` — destructuring caveat, class-field forms, constructor assignment
  - `reference/docs/02-runes/03-$derived.md` — destructured `$derived` semantics and class-field support
  - `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js` — destructured `$state` / `$derived` lowering
  - `reference/compiler/phases/3-transform/client/visitors/ClassBody.js` — class-field lowering
  - `reference/compiler/utils/ast.js` — `extract_paths`

- Our code:
  - `crates/svelte_codegen_client/src/script/state.rs` — destructuring expansion, async derived destructuring, class-field rewrite
  - `crates/svelte_codegen_client/src/script/traverse/derived.rs` — `wrap_derived_thunks`, currently identifier-only for sync `$derived`
  - `crates/svelte_codegen_client/src/script/traverse/runes.rs` — base rune init rewrites
  - `tasks/compiler_tests/test_v3.rs` — compiler test inventory and ignores
  - `tasks/compiler_tests/cases2/derived_by_class_fields/` — focused missing class-field case from this audit

## Tasks

### Codegen: sync `$derived` destructuring
1. [x] `wrap_derived_thunks` skips non-Identifier bindings — already handled by earlier expansion pass.
2. [x] Sync `$derived` destructuring lowered correctly for object, array, rest patterns.
3. [x] Both lowering modes implemented: direct per-path and intermediate `$$d`.

### Codegen: `$derived.by` class fields
4. [x] Constructor-assigned public `$derived.by` fields: bare placeholder declaration skipped in `rewrite_class_body`; backing pre-emitted at top of class body.
5. [x] Mixed public/private `$derived.by` fields in one class work correctly.

### Tests
6. [x] `derived_by_class_fields` unignored and passing.
7. [x] `derived_destructured_object`, `derived_destructured_array`, `derived_destructured_by` passing.

## Implementation order

1. Fix `derived_by_class_fields` so the class-field matrix is no longer ambiguous.
2. Add a reusable sync destructuring helper for `$derived`.
3. Unignore the three destructured `$derived` tests once the helper matches reference output.

## Discovered bugs

### BUG-1: Constructor-assigned public `$derived.by` field leaves the original declaration behind — FIXED
- **Test**: `derived_by_class_fields` — passing
- **Root cause**: Bare `total;` declaration had no rune init so `is_rune_prop` was false and it was copied through unchanged. Also, the private backing was emitted just before the constructor instead of at the top of the class body.
- **Fix**: Pre-scan identifies constructor-assigned fields (not in `body_rune_names`); they are pre-emitted before the main loop. Bare placeholder declarations are skipped in the main loop.

## Test cases

### Existing passing coverage
- `state_destructure`
- `state_raw_destructure_object`
- `state_raw_destructure_array`
- `state_class_field`
- `state_private_class_field`
- `state_class_constructor`
- `state_class_multiple`
- `state_class_field_constructor_assign`
- `state_class_constructor_proxy`
- `state_raw_class_field`
- `state_class_raw_field`
- `derived_class_field`

### Missing / ignored coverage
- `derived_destructured_object`
- `derived_destructured_array`
- `derived_destructured_by`
- `derived_by_class_fields`
