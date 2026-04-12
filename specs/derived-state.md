# $derived / $derived.by

## Current state
- **Working**: 20/20 use cases
- **Tests**: 35/37 green
- Last updated: 2026-04-03

## Source

ROADMAP.md — `$derived` rune (core reactivity)

## Use cases

- [x] Basic `$derived(expr)` → `$.derived(() => expr)`
- [x] `$derived.by(fn)` → `$.derived(fn)`
- [x] `$derived` in nested function scope
- [x] `$derived.by` in nested function scope
- [x] `$derived` class field (`area = $derived(this.width * this.height)`)
- [x] Constructor assignment `this.x = $derived(...)`
- [x] Read access rewritten to `$.get(x)`
- [x] Dev mode `$.tag($.derived(...), "name")` wrapping
- [x] Async `$derived(await expr)` → `await $.async_derived(async () => expr)`
- [x] Async destructured `$derived(await expr)` with intermediate variable
- [x] Async dev mode with label and location args
- [x] Async dev mode with `svelte-ignore await_waterfall` suppression
- [x] `@const` tag bindings treated as derived
- [x] Sync destructured `$derived(expr)` where arg is plain Identifier (no intermediate var)
- [x] Sync destructured `$derived(expr)` where arg is NOT plain Identifier (intermediate `$$d` var)
- [x] Sync destructured `$derived.by(fn)` (intermediate `$$d` var)
- [x] `derived_invalid_export` diagnostic when `export`ing derived binding
- [x] `state_referenced_locally` warning for derived bindings read at same function depth
- [x] `$.save()` for nested async derived (`function_depth > 1`)
- [x] `rune_invalid_usage` in non-runes mode

## Reference

### Reference compiler files
- `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js:192-291` — all transform paths
- `reference/compiler/phases/2-analyze/visitors/CallExpression.js:117-135` — placement validation
- `reference/compiler/phases/2-analyze/visitors/CallExpression.js:245-257` — async_deriveds detection
- `reference/compiler/phases/2-analyze/visitors/ExportNamedDeclaration.js:40-42` — derived_invalid_export
- `reference/compiler/phases/2-analyze/visitors/Identifier.js:117` — state_referenced_locally warning
- `reference/compiler/phases/2-analyze/visitors/VariableDeclarator.js:29-65` — binding.kind = 'derived'
- `reference/compiler/phases/2-analyze/visitors/shared/declarations.js:22-23` — read transform registration

### Our files
- `crates/svelte_analyze/src/types/script.rs` — `RuneKind::Derived`, `RuneKind::DerivedBy`
- `crates/svelte_analyze/src/utils/script_info.rs` — `detect_rune`, `collect_derived_refs`
- `crates/svelte_analyze/src/passes/mark_runes.rs` — `mark_script_runes`, `mark_nested_runes`
- `crates/svelte_analyze/src/scope.rs` — `Rune.derived_deps`, `is_dynamic_by_id`
- `crates/svelte_analyze/src/validate/runes.rs` — placement and argument validation
- `crates/svelte_codegen_client/src/script/traverse/runes.rs` — `rewrite_variable_rune_init`, `rewrite_identifier_expression`
- `crates/svelte_codegen_client/src/script/traverse/derived.rs` — `wrap_derived_thunks` (only handles BindingIdentifier)
- `crates/svelte_codegen_client/src/script/state.rs` — `process_async_derived_destructuring`, `gen_derived_destructure_assignments`
- `crates/svelte_diagnostics/src/lib.rs` — `DerivedInvalidExport`, `StateReferencedLocally`

## Test cases

- [x] `derived_basic`
- [x] `derived_by`
- [x] `derived_by_inside_function`
- [x] `derived_class_field`
- [x] `derived_dynamic`
- [x] `derived_in_nested_function`
- [x] `derived_inside_function`
- [x] `derived_local_signal_get`
- [x] `derived_nested_getter`
- [x] `derived_shorthand_property`
- [x] `tag_derived_basic`
- [x] `tag_derived_by`
- [x] `state_constructor_read_derived`
- [x] `event_handler_derived_with_class_directives`
- [x] `event_handler_derived_with_class_object`
- [x] `async_derived_basic`
- [x] `async_derived_destructured`
- [x] `async_derived_dev`
- [x] `async_derived_dev_ignored`
- [x] `async_derived_dev_ignored_destructured`
- [x] `async_const_derived_chain`
- [x] `async_derived_nested_function`
- [x] `async_derived_nested_function_destructured`
- [x] `derived_destructured_object`
- [x] `derived_destructured_array`
- [x] `derived_destructured_by`
- [x] `derived_non_runes_invalid_usage`
- [x] `validate_derived_rune_invalid_usage_in_non_runes_mode`
- [x] `validate_derived_destructured_rune_invalid_usage_in_non_runes_mode`
- [x] `validate_derived_rune_allowed_in_runes_mode`
- [x] `derived_invalid_export`
- [x] `validate_derived_wrong_arg_count`
- [x] `validate_derived_by_wrong_arg_count`
- [x] `validate_derived_invalid_export`
- [ ] `validate_derived_invalid_export_specifier`
- [ ] `validate_derived_invalid_default_export`
- [x] `state_referenced_locally` (derived bindings)
