# $derived / $derived.by

## Current state
- **Working**: 28/28 use cases
- **Tests**: 45/45 green
- Last updated: 2026-05-14

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
- [x] Dev mode `$.tag($.derived(...), "name")` wrapping also fires for `$derived` declarations in `.svelte.js` / `.svelte.ts` standalone modules (test: `module_dev_derived_tag`)
- [x] Async `$derived(await expr)` → `await $.async_derived(async () => expr)`
- [x] Async destructured `$derived(await expr)` with intermediate variable
- [x] Async dev mode with label and location args
- [x] Async dev mode with `svelte-ignore await_waterfall` suppression
- [x] `@const` tag bindings treated as derived
- [x] Sync destructured `$derived(expr)` where arg is plain Identifier (no intermediate var)
- [x] Sync destructured `$derived(ident)` where `ident` resolves to a `$props()`-destructured prop follows the plain-Identifier branch — per-prop `$.derived(() => $$props.<ident>.<key>)`, no `$$d` wrap. `DerivedLowering` is extended with `DestructuredInlineSource` / `DestructuredBoxedSync` / `DestructuredBoxedAsync` so analyze names the lowering shape from the raw source AST once; transform's destructure dispatch is one `match` on `DerivedLowering` (test: `derived_destructured_object_prop_source`)
- [x] Sync destructured `$derived(ident)` where `ident` resolves to a whole-object `$props()` binding (`const props = $props(); const { a, b } = $derived(props);`) gets a dedicated `DerivedLowering::DestructuredInlinePropsSource` variant — analyze detects the source symbol's `PropBindingKind::Rest`, transform substitutes `$$props` directly as the access root (no `rest_prop_member` rewrite path, no indirection through the local `props` binding); emits `$.derived(() => $$props.<key>)` (test: `derived_destructured_props_whole_source`)
- [x] Sync destructured `$derived(expr)` where arg is NOT plain Identifier (intermediate `$$d` var)
- [x] Sync destructured `$derived.by(fn)` (intermediate `$$d` var)
- [x] `derived_invalid_export` diagnostic when `export`ing derived binding
- [x] `state_referenced_locally` warning for derived bindings read at same function depth
- [x] `$.save()` for nested async derived (`function_depth > 1`)
- [x] `rune_invalid_usage` in non-runes mode
- [x] `$derived(expr)` declarator nested inside an arrow/function body in a `.svelte.ts` module wraps `expr` in `() => …` (transform recurses into `VariableDeclaration` declarator-init bodies, not only `FunctionDeclaration`); test: `module_derived_arrow_wrap_no_state_deps`
- [x] `$derived(expr)` declarator nested inside an arrow/function body in a `.svelte.ts` module still wraps `expr` in `() => …` when `expr` reads `$state` bindings — arrow wrap must happen on the original expression before state-read rewriting, so output is `$.derived(() => $.get(items).length)`, not `$.derived($.get(items).length)`; test: `module_derived_arrow_wrap_with_state_deps`
- [x] Plain assignment to a `$derived` binding (e.g. `invalid = false` inside a function) rewritten to `$.set(invalid, false)` — derived write classified via new `ReferenceSemantics::DerivedWrite` variant, transform `dispatch_identifier_assignment` routes it to a derived-specific helper that emits `$.set` without proxy wrapping; test: `derived_write_assignment`
- [x] `$derived(expr)` declarator inside a class method body in a `.svelte.ts` module wraps `expr` in `() => …` — `wrap_derived_thunks_in_stmts` descends into `Statement::ClassDeclaration` (and `ExportNamedDeclaration` wrapping one) and iterates `ClassElement::MethodDefinition` bodies; test: `module_derived_arrow_wrap_in_class_method`.
- [x] `$derived(expr)` initializer wrapped in a TypeScript type cast (`$derived(expr) as T`, and analogously `satisfies T` / non-null `!`) is still classified as a derived rune — the declaration must lower to `$.derived(() => expr)` and all reads become `$.get(status)`. Owning layer: analyze — `utils::script_info::detect_rune` only matches a bare `CallExpression`, so any TS wrapper around the call hides the rune from the classifier and the declaration falls through as a plain initializer. Generalises to other runes (`$state`, `$props`, `$derived.by`). Repro: `let status = $derived(error ? 'error' : fallback) as string;` (test: `diagnose_derived_rune_with_ts_as_cast`)

## Reference

### Reference compiler files
- `original/compiler/phases/3-transform/client/visitors/VariableDeclaration.js:192-291` — all transform paths
- `original/compiler/phases/2-analyze/visitors/CallExpression.js:117-135` — placement validation
- `original/compiler/phases/2-analyze/visitors/CallExpression.js:245-257` — async_deriveds detection
- `original/compiler/phases/2-analyze/visitors/ExportNamedDeclaration.js:40-42` — derived_invalid_export
- `original/compiler/phases/2-analyze/visitors/Identifier.js:117` — state_referenced_locally warning
- `original/compiler/phases/2-analyze/visitors/VariableDeclarator.js:29-65` — binding.kind = 'derived'
- `original/compiler/phases/2-analyze/visitors/shared/declarations.js:22-23` — read transform registration

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
- [x] `module_dev_derived_tag`
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
- [x] `derived_destructured_object_prop_source`
- [x] `derived_destructured_props_whole_source`
- [x] `derived_non_runes_invalid_usage`
- [x] `validate_derived_rune_invalid_usage_in_non_runes_mode`
- [x] `validate_derived_destructured_rune_invalid_usage_in_non_runes_mode`
- [x] `validate_derived_rune_allowed_in_runes_mode`
- [x] `derived_invalid_export`
- [x] `validate_derived_wrong_arg_count`
- [x] `validate_derived_by_wrong_arg_count`
- [x] `validate_derived_invalid_export`
- [x] `validate_derived_invalid_export_specifier`
- [x] `validate_derived_invalid_default_export`
- [x] `state_referenced_locally` (derived bindings)
- [x] `module_derived_arrow_wrap_no_state_deps`
- [x] `module_derived_arrow_wrap_with_state_deps`
- [x] `derived_write_assignment`
- [x] `diagnose_derived_rune_with_ts_as_cast`
- [x] `module_derived_arrow_wrap_in_class_method`
