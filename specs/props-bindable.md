# $props / $bindable

## Current state
- **Working**: 20/22 use cases
- **Completed (2026-04-03)**: added compiler-level pipeline tests for `$props.id()` validation edge cases (`compile_props_id_invalid_placement`, `compile_props_id_duplicate_with_props`) in `crates/svelte_compiler/src/tests.rs`
- **Completed (2026-04-11)**: identifier-pattern `$props()` bindings no longer trigger a false-positive `store_rune_conflict`; analyzer store validation now excludes props-owned bindings and the focused parity case `props_identifier_no_store_rune_conflict` passes alongside the existing positive `store_rune_conflict` analyzer test.
- **Completed (2026-04-12)**: `$props()` / `$props.id()` in `<script module>` now emit `props_invalid_placement` / `props_id_invalid_placement` via analyzer validation, with focused diagnostic parity coverage (`validate_props_invalid_placement_in_module_script`, `validate_props_id_invalid_placement_in_module_script`).
- **Completed (2026-04-12)**: module-script `$props()` / `$props.id()` validation now flows through `validate/runes.rs` instead of a parallel module-only validator, preserving reference parity for module invalid-placement behavior while avoiding duplicated call-walker logic.
- **Completed (2026-04-12)**: follow-up review cleanup deduplicated `RuneValidator` initialization for instance/module entrypoints while keeping module placement diagnostics and test coverage unchanged.
- **In progress (2026-04-12)**: closing the remaining dev-mode ownership mutation parity gap for computed identifier member paths. Static member writes and string-literal computed paths already wrap prop / bindable-prop assignment and update expressions with `$$ownership_validator.mutation(...)`, and emit `$.create_ownership_validator($$props)` when needed.
- **Next**: Complete computed identifier member-path ownership mutation parity; `$$props` / `$$restProps` legacy compatibility remains out of scope for this run.
- Last updated: 2026-04-12

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

- [x] Basic destructured props source: `let { x, y = 10 } = $props()` (test: `props_basic`)
- [x] Rest props lowering: `let { x, ...rest } = $props()` (test: `props_rest`)
- [x] Identifier pattern: `const props = $props()` (tests: `props_identifier_basic`, `props_identifier_await_expression`)
- [x] Non-bindable fallback values including lazy defaults (test: `props_lazy_default`)
- [x] Local mutation of a prop source produces updatable local state (test: `props_mutated`)
- [x] `$bindable()` defaults inside `$props()` destructuring (tests: `props_bindable`, `props_mixed`)
- [x] Proxy wrapping for bindable object/array defaults (test: `tag_bindable_proxy`)
- [x] Bindable prop forwarding through component bindings (tests: `component_bind_prop_forward`, `push_binding_group_order`)
- [x] Renamed/aliased props (test: `props_renamed`): `let { foo: local = 'default' } = $props()` uses prop key in `$.prop()` call
- [x] Renamed + bindable props (test: `props_renamed_bindable`): `let { value: local = $bindable('fallback') } = $props()`
- [x] `$props.id()` basic lowering (tests: `props_id_basic`, `props_id_with_props`)
- [x] `$props.id()` validation edge cases covered by compiler-level pipeline tests
- [x] `$bindable()` validation: `bindable_invalid_location` and argument-count checks
- [x] `$props()` validation: `props_invalid_placement`, `props_duplicate`, and rune argument-count checks
- [x] Identifier-pattern `$props()` bindings like `let props = $props()` must not emit a false-positive `store_rune_conflict` warning (diagnostic test: `props_identifier_no_store_rune_conflict`)
- [x] `$props()` and `$props.id()` rejected inside `<script module>` — reference: `ast_type !== 'instance'` check in `CallExpression.js`
- [x] `$props.id()` validation: `props_id_invalid_placement`, duplicate detection with `$props()`, zero-argument enforcement
- [x] `$props()` pattern validation: `props_invalid_pattern` and `props_invalid_identifier`
- [x] `props_illegal_name` for MemberExpression access on rest props
- [x] Custom-element warning: `custom_element_props_identifier` for identifier/rest `$props()` in custom elements
- [ ] Dev-mode ownership mutation validation for prop / bindable-prop member writes via `$$ownership_validator.mutation(...)` (partial: assignment/update member writes covered; computed-path cases still open)
- [ ] `$$props` / `$$restProps` legacy compatibility (Tier 7)

## Reference

- `reference/docs/02-runes/05-$props.md`
- `reference/docs/02-runes/06-$bindable.md`
- `reference/compiler/phases/2-analyze/visitors/CallExpression.js` — `$props`, `$props.id`, `$bindable` placement and arity validation
- `reference/compiler/phases/2-analyze/visitors/VariableDeclarator.js` — props pattern validation, bindable default stripping, custom-element warning
- `reference/compiler/phases/2-analyze/visitors/MemberExpression.js` — `props_illegal_name`
- `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`
- `reference/compiler/phases/3-transform/client/transform-client.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/utils.js` — `validate_mutation`, `$$ownership_validator.mutation`
- `reference/compiler/phases/3-transform/client/utils.js`
- `crates/svelte_analyze/src/utils/script_info.rs` — structural extraction of props declarations/defaults
- `crates/svelte_analyze/src/passes/post_resolve.rs` — `PropsAnalysis` construction and bindable/runtime-plan flags
- `crates/svelte_analyze/src/passes/js_analyze/needs_context.rs` — marks props/rest access as context-sensitive
- `crates/svelte_analyze/src/validate/runes.rs` — currently validates `$state`/`$derived` only
- `crates/svelte_codegen_client/src/script/props.rs` — `$.prop`, `$.rest_props`, bindable default proxying
- `crates/svelte_codegen_client/src/script/traverse/statement_passes.rs` — props declaration replacement/removal
- `crates/svelte_diagnostics/src/lib.rs` — already contains the missing `$props`/`$bindable` diagnostics and warning codes

## Test cases

- [x] `props_basic`
- [x] `props_rest`
- [x] `props_identifier_basic`
- [x] `props_identifier_await_expression`
- [x] `props_lazy_default`
- [x] `props_mutated`
- [x] `props_bindable`
- [x] `props_mixed`
- [x] `tag_bindable_proxy`
- [x] `component_bind_prop_forward`
- [x] `push_binding_group_order`
- [x] `props_id_basic`
- [x] `props_id_with_props`
- [x] `props_renamed`
- [x] `props_renamed_bindable`
- [x] analyze unit: `bindable_invalid_location`
- [x] analyze unit: `rune_invalid_arguments_length` on `$bindable`
- [x] analyze unit: `props_invalid_placement`
- [x] analyze unit: `props_duplicate`
- [x] analyze unit: `$props.id()` duplicate handling against `$props()`
- [x] analyze unit: `props_id_invalid_placement`
- [x] analyze unit: `props_invalid_pattern`
- [x] analyze unit: `props_illegal_name` MemberExpression on rest_prop (3 tests)
- [x] analyze unit: `custom_element_props_identifier` warning (4 tests)
- [x] analyze unit: `validate_props_identifier_no_store_rune_conflict`
- [x] `props_identifier_no_store_rune_conflict`
- [x] diagnostic parity: `validate_props_invalid_placement_in_module_script`
- [x] diagnostic parity: `validate_props_id_invalid_placement_in_module_script`
- [x] diagnostic parity: `validate_props_invalid_arguments_in_module_script`
- [x] diagnostic parity: `validate_props_id_invalid_arguments_in_module_script`
- [x] compiler unit: `compile_dev_props_member_mutation_uses_ownership_validator`
- [x] compiler unit: `compile_dev_bindable_prop_member_mutation_uses_prop_alias`
- [x] compiler unit: `compile_dev_bindable_prop_member_update_uses_ownership_validator`
