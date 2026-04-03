# $props / $bindable

## Current state
- **Working**: 8/15 use cases covered with passing compiler tests
- **Partial**: 3/15 use cases have runtime/codegen support but incomplete analyzer parity or focused coverage
- **Validation landed**: `$bindable` placement/arity, `$props` placement/duplicate/arity, `$props.id` placement/duplicate/arity, props pattern validation (computed keys, `$$` names, nested destructures) — all 7 analyzer unit tests passing
- **Remaining**: `props_illegal_name` for MemberExpression rest prop access, `custom_element_props_identifier` warning, focused compiler cases for renamed props
- **Next**: add `props_illegal_name` MemberExpression check (needs binding kind tracking), then add focused compiler cases for alias + fallback edge cases
- Last updated: 2026-04-03

## Source

ROADMAP.md — `$props` / `$bindable`

## Syntax variants

- `let { foo } = $props()`
- `let { foo = 1 } = $props()`
- `let { foo: local = 1 } = $props()`
- `let { foo, ...rest } = $props()`
- `const props = $props()`
- `let { value = $bindable() } = $props()`
- `let { value = $bindable('fallback') } = $props()`
- `const id = $props.id()`

## Use cases

### Covered
- [x] Basic destructured props source: `let { x, y = 10 } = $props()` (`props_basic`)
- [x] Rest props lowering: `let { x, ...rest } = $props()` (`props_rest`)
- [x] Identifier pattern: `const props = $props()` (`props_identifier_basic`, `props_identifier_await_expression`)
- [x] Non-bindable fallback values including lazy defaults (`props_lazy_default`)
- [x] Local mutation of a prop source produces updatable local state (`props_mutated`)
- [x] `$bindable()` defaults inside `$props()` destructuring (`props_bindable`, `props_mixed`)
- [x] Proxy wrapping for bindable object/array defaults (`tag_bindable_proxy`)
- [x] Bindable prop forwarding through component bindings (`component_bind_prop_forward`, `push_binding_group_order`)
- [x] `$props.id()` basic lowering (`props_id_basic`, `props_id_with_props`)

### Partial
- [~] Renamed props work structurally in `script_info` and codegen, but there is no focused compiler case for aliasing plus fallback/bindability
- [~] Custom element `$props()` paths are covered for happy paths, but the warning-only branch for non-destructured/rest declarations without explicit `customElement.props` is untested
- [~] `$props.id()` basic lowering works (`props_id_basic`, `props_id_with_props`), but analyze still lacks focused placement/duplicate parity with reference `$props()` validation

### Missing
- [x] `$bindable()` validation in analyze:
  `bindable_invalid_location` and argument-count checks — DONE
- [x] `$props()` validation in analyze:
  `props_invalid_placement`, `props_duplicate`, and rune argument-count checks — DONE
- [x] `$props.id()` validation in analyze:
  `props_id_invalid_placement`, duplicate detection with `$props()`, and zero-argument enforcement — DONE
- [x] `$props()` pattern validation in analyze:
  `props_invalid_pattern` and `props_invalid_identifier` — DONE
- [ ] `props_illegal_name` for MemberExpression access on rest props (needs binding kind tracking)
- [ ] Custom-element warning parity:
  `custom_element_props_identifier` is defined but not emitted

### Deferred
- `$$props` / `$$restProps` legacy compatibility is out of scope for this Svelte 5 rune audit
- SSR behavior for props is out of scope

## Reference

### Reference compiler files
- `reference/docs/02-runes/05-$props.md`
- `reference/docs/02-runes/06-$bindable.md`
- `reference/compiler/phases/2-analyze/visitors/CallExpression.js` — `$props`, `$props.id`, `$bindable` placement and arity validation
- `reference/compiler/phases/2-analyze/visitors/VariableDeclarator.js` — props pattern validation, bindable default stripping, custom-element warning
- `reference/compiler/phases/2-analyze/visitors/MemberExpression.js` — `props_illegal_name`
- `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`
- `reference/compiler/phases/3-transform/client/transform-client.js`
- `reference/compiler/phases/3-transform/client/utils.js`

### Our files
- `crates/svelte_analyze/src/utils/script_info.rs` — structural extraction of props declarations/defaults
- `crates/svelte_analyze/src/passes/post_resolve.rs` — `PropsAnalysis` construction and bindable/runtime-plan flags
- `crates/svelte_analyze/src/passes/js_analyze/needs_context.rs` — marks props/rest access as context-sensitive
- `crates/svelte_analyze/src/validate/runes.rs` — currently validates `$state`/`$derived` only
- `crates/svelte_codegen_client/src/script/props.rs` — `$.prop`, `$.rest_props`, bindable default proxying
- `crates/svelte_codegen_client/src/script/traverse/statement_passes.rs` — props declaration replacement/removal
- `crates/svelte_diagnostics/src/lib.rs` — already contains the missing `$props`/`$bindable` diagnostics and warning codes

## Tasks

### analyze
1. [x] Extend `validate/runes.rs` for `$bindable` arity and placement parity with reference `CallExpression.js`
2. [x] Extend `validate/runes.rs` for `$props` arity, top-level placement, and duplicate detection across `$props()` and `$props.id()`
3. [x] Extend `validate/runes.rs` for `$props.id` top-level identifier-only placement and zero-argument validation
4. [x] Add props-pattern validation for computed keys, nested patterns, and `$$` names
5. [ ] Add `custom_element_props_identifier` warning emission in the appropriate analyze pass

### tests
6. [ ] Keep the new analyzer tests added by this audit and make them pass
7. [ ] Add focused compiler cases for renamed props and custom-element warning behavior once validation parity lands

## Implementation order

1. Port `$bindable` validation first because it is local and isolated
2. Port `$props` placement/duplicate checks next because they gate all remaining patterns
3. Port props-pattern/custom-element warning logic from `VariableDeclarator.js`
4. Add or refresh compiler cases after analyzer validation is correct

## Discovered bugs

- FIXED: `validate/runes.rs` now emits `$props`/`$bindable`/`$props.id` diagnostics (placement, arity, duplicate, pattern validation)

## Test cases

### Existing passing coverage
- `props_basic`
- `props_rest`
- `props_identifier_basic`
- `props_identifier_await_expression`
- `props_lazy_default`
- `props_mutated`
- `props_bindable`
- `props_mixed`
- `tag_bindable_proxy`
- `component_bind_prop_forward`
- `push_binding_group_order`
- `props_id_basic`
- `props_id_with_props`

### Added by this audit
- analyze unit tests for `bindable_invalid_location`
- analyze unit tests for `rune_invalid_arguments_length` on `$bindable`
- analyze unit tests for `props_invalid_placement`
- analyze unit tests for `props_duplicate`
- analyze unit tests for `$props.id()` duplicate handling against `$props()`
- analyze unit tests for `props_id_invalid_placement`
- analyze unit tests for `props_invalid_pattern`
