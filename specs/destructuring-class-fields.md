# Destructuring & class fields

## Current state
- **Working**: `$state` / `$state.raw` destructuring and the main `$state` / `$derived` class-field paths are covered by passing compiler tests.
- **Missing**: 3 sync destructured `$derived` use cases are still ignored (`derived_destructured_object`, `derived_destructured_array`, `derived_destructured_by`).
- **Partial**: `$derived.by` class-field support is incomplete. A new focused test (`derived_by_class_fields`) shows constructor-assigned public fields keep the original `total;` declaration instead of fully lowering to the private backing field form.
- **Out of scope**: template destructuring features tracked by other specs (`snippet-block.md`, `each-block.md`, `const-tag.md`).
- **Next**: fix `derived_by_class_fields` first, then port sync `$derived` destructuring with a shared `extract_paths`-style helper.
- Last updated: 2026-04-01

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
5. [ ] Sync `$derived(expr)` object destructuring (ignored test: `derived_destructured_object`)
6. [ ] Sync `$derived(expr)` array/rest destructuring (ignored test: `derived_destructured_array`)
7. [ ] Sync `$derived.by(fn)` destructuring (ignored test: `derived_destructured_by`)
8. [x] `$state` public class field (covered, tests: `state_class_field`, `state_class_multiple`)
9. [x] `$state` private class field (covered, test: `state_private_class_field`)
10. [x] `$state` constructor assignment (covered, tests: `state_class_constructor`, `state_class_field_constructor_assign`, `state_class_constructor_proxy`)
11. [x] `$state.raw` class field (covered, tests: `state_raw_class_field`, `state_class_raw_field`)
12. [x] `$derived` public class field (covered, test: `derived_class_field`)
13. [~] `$derived.by` class fields: field initializers and private reads look implemented, but constructor-assigned public fields are not fully lowered (ignored test: `derived_by_class_fields`)

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
1. [ ] Extend `wrap_derived_thunks` to handle non-Identifier binding patterns instead of skipping them.
2. [ ] Introduce an `extract_paths`-style helper for sync `$derived` destructuring so object, array, rest, and default-value paths all lower consistently.
3. [ ] Match reference behavior for the two sync lowering modes:
   - direct per-path `$.derived(() => source.path)` when `$derived(expr)` can use the source expression directly
   - intermediate `$$d = $.derived(...)` plus per-path `$.derived(() => $.get($$d).path)` when needed

### Codegen: `$derived.by` class fields
4. [ ] Fix constructor-assigned public `$derived.by` fields so `rewrite_class_body` / `rewrite_constructor` remove the original public declaration when synthesizing the private backing field.
5. [ ] Confirm the same lowering works for mixed public/private `$derived.by` fields in one class without duplicate field emission.

### Tests
6. [ ] Unignore `derived_by_class_fields` after the constructor rewrite bug is fixed.
7. [ ] Unignore `derived_destructured_object`, `derived_destructured_array`, and `derived_destructured_by` after sync destructuring lands.

## Implementation order

1. Fix `derived_by_class_fields` so the class-field matrix is no longer ambiguous.
2. Add a reusable sync destructuring helper for `$derived`.
3. Unignore the three destructured `$derived` tests once the helper matches reference output.

## Discovered bugs

### BUG-1: Constructor-assigned public `$derived.by` field leaves the original declaration behind
- **Test**: `derived_by_class_fields` — ignored
- **Observed output**: Rust still emits `total;` in the class body before adding `#total` getter/setter lowering.
- **Likely area**: interaction between `rewrite_class_body` and `rewrite_constructor` in `crates/svelte_codegen_client/src/script/state.rs`

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
