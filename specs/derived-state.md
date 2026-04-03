# $derived / $derived.by

## Current state

**Updated: 2026-04-03**

Core `$derived` and `$derived.by` are now fully implemented for simple identifier bindings (sync and async), destructuring, class fields, nested functions, dev mode, nested async save-path lowering, and non-runes invalid-usage validation.

**Done this session:**
1. Added compiler coverage for nested async `$derived(await ...)` and nested async destructured `$derived(await ...)`, matching the reference compiler's `$.save(...)` path for `function_depth > 1`
2. Wired `config.json.runes` through the compiler test harness and added a passing compiler diagnostic case for non-runes `$derived` invalid usage
3. Added analyzer coverage for `rune_invalid_usage` in both simple and destructured non-runes forms, confirmed the diagnostic stays gated off in runes mode, and kept compiler coverage for the `$derived` non-runes diagnostic path

**Next:** feature complete for the current `$derived` / `$derived.by` scope

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

## Tasks

### Codegen: Sync destructured $derived
- **File:** `crates/svelte_codegen_client/src/script/traverse/derived.rs`
- Extend `wrap_derived_thunks` to handle destructured patterns (ObjectPattern/ArrayPattern)
- Reference: `VariableDeclaration.js:227-288` — Path 4 (Identifier arg, no intermediate) and Path 5 (non-Identifier arg, intermediate `$$d`)
- Need `extract_paths` equivalent for destructuring into individual `$.derived(() => path)` declarations

### Analyze: derived_invalid_export diagnostic
- **File:** `crates/svelte_analyze/src/validate/runes.rs` (or export validation pass)
- Emit `DerivedInvalidExport` when an `ExportNamedDeclaration` contains a derived binding
- Reference: `ExportNamedDeclaration.js:40-42`

### Analyze: state_referenced_locally warning for derived
- **File:** `crates/svelte_analyze/src/` (identifier visitor or validation)
- Emit `StateReferencedLocally` when a derived binding is read at the same function depth as its declaration
- Reference: `Identifier.js:117`

## Test cases

### Existing (all pass)
- `derived_basic`, `derived_by`, `derived_by_inside_function`
- `derived_class_field`, `derived_dynamic`, `derived_in_nested_function`
- `derived_inside_function`, `derived_local_signal_get`, `derived_nested_getter`
- `derived_shorthand_property`, `tag_derived_basic`, `tag_derived_by`
- `state_constructor_read_derived`
- `event_handler_derived_with_class_directives`, `event_handler_derived_with_class_object`
- `async_derived_basic`, `async_derived_destructured`
- `async_derived_dev`, `async_derived_dev_ignored`, `async_derived_dev_ignored_destructured`
- `async_const_derived_chain`
- `async_derived_nested_function`
- `async_derived_nested_function_destructured`
- `derived_non_runes_invalid_usage`
- `validate_derived_rune_invalid_usage_in_non_runes_mode`
- `validate_derived_destructured_rune_invalid_usage_in_non_runes_mode`
- `validate_derived_rune_allowed_in_runes_mode`

### Planned
- `derived_destructured_object` — sync destructured `$derived` with object pattern
- `derived_destructured_array` — sync destructured `$derived` with array pattern
- `derived_destructured_by` — sync destructured `$derived.by` with object pattern

### Completed in this session
- `derived_destructured_object`
- `derived_destructured_array`
- `derived_destructured_by`
- `derived_invalid_export` analyzer diagnostic
- `state_referenced_locally` warning for derived bindings
- `async_derived_nested_function`
- `async_derived_nested_function_destructured`
- `derived_non_runes_invalid_usage`
- `rune_invalid_usage` analyzer validation in non-runes mode
