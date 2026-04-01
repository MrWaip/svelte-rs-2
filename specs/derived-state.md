# $derived / $derived.by

## Current state

**Updated: 2026-04-01**

Core `$derived` and `$derived.by` are fully implemented for simple identifier bindings (sync and async), class fields, nested functions, and dev mode. 21 existing tests all pass.

**Gaps found:**
1. Sync destructured `$derived(expr)` — not implemented (codegen skips non-Identifier patterns in `wrap_derived_thunks`)
2. Sync destructured `$derived.by(fn)` — same gap
3. `derived_invalid_export` diagnostic — defined but never emitted from analyze
4. `state_referenced_locally` warning — not emitted for derived bindings
5. `$.save()` for nested async derived (`function_depth > 1`) — unknown, no test

**Next:** Add test cases for gaps, then fix starting with sync destructured `$derived`.

## Source

ROADMAP.md — `$derived` rune (core reactivity)

## Use cases

### Implemented
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

### In scope
- [ ] Sync destructured `$derived(expr)` where arg is plain Identifier (no intermediate var)
- [ ] Sync destructured `$derived(expr)` where arg is NOT plain Identifier (intermediate `$$d` var)
- [ ] Sync destructured `$derived.by(fn)` (intermediate `$$d` var)
- [ ] `derived_invalid_export` diagnostic when `export`ing derived binding
- [ ] `state_referenced_locally` warning for derived bindings read at same function depth

### Deferred
- `$.save()` for nested async derived (`function_depth > 1`) — needs async infrastructure
- `rune_invalid_usage` in non-runes mode — broader runes validation scope

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

### Planned
- `derived_destructured_object` — sync destructured `$derived` with object pattern
- `derived_destructured_array` — sync destructured `$derived` with array pattern
- `derived_destructured_by` — sync destructured `$derived.by` with object pattern
