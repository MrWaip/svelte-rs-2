# Filename-Derived Component Name

## Current state
- **Working**: 6/6 use cases
- **Completed slices**: `filename-derived normalization`; `scoped component-name finalization`
- **Next**: complete
- **Notes**: final component export names now come from `CompileOptions::component_name()` candidate normalization plus analyze-time deconfliction against reserved keywords, semantic symbol names, and root unresolved references; analyze warnings and codegen share that finalized name
- Last updated: 2026-04-11

## Source

- User request: `/audit` how the original compiler handles `CompileOptions.filename`, prompted by invalid output for `src/routes/.../+page.svelte`

## Syntax variants

```text
compile(source, { filename: "src/routes/foo/+page.svelte" })
compile(source, { filename: "src/routes/foo/+layout.svelte" })
compile(source, { filename: "src/routes/foo/index.svelte" })
compile(source, { filename: "src/routes/foo/123-widget.svelte" })
compile(source, { name: "custom-name", filename: "src/routes/foo/+page.svelte" })
compile(source, { rootDir: "/repo", filename: "/repo/src/routes/foo/+page.svelte" })
```

## Use cases

- [x] Explicit `CompileOptions.name` overrides filename, and the final exported function name goes through identifier sanitization and deconfliction against reserved words and semantic conflicts. (tests: `component_name_explicit_sanitized`, `compile_explicit_name_reserved_word_is_deconflicted`, `compile_explicit_name_conflict_is_deconflicted`)
- [x] Plain filename derives the component name from basename without `.svelte`, then uppercases the first character (`counter.svelte` -> `Counter`). (test: `component_name_from_filename`)
- [x] `index.svelte` derives the name from the parent directory when that directory exists and is not `src` (`src/routes/blog/index.svelte` -> `Blog`). (tests: `component_name_index_uses_parent_dir`, `component_name_index_under_src_stays_index`)
- [x] Invalid identifier characters in derived names are sanitized before codegen (`+page.svelte` -> `_page`), and leading digits are prefixed (`123-widget.svelte` -> `_23_widget`). (tests: `component_name_filename_sanitized`, `component_name_filename_leading_digit_sanitized`, `compile_filename_derived_name_is_sanitized`)
- [x] Derived or explicit component names are deconflicted against reserved words and declarations/references visible to the root semantic scope (`class.svelte`, local `App`, root unresolved names), and analyze warnings reuse that finalized name. (tests: `compile_filename_derived_name_conflict_is_deconflicted`, `svelte_self_deprecated_uses_deconflicted_component_name`, `svelte_self_deprecated_uses_reserved_word_deconflicted_component_name`)
- [x] `rootDir` does not participate in component-name derivation; it only affects the normalized runtime `filename` used for diagnostics/dev metadata. Our compiler also ignores `root_dir` when deriving the name.

## Out of scope

- `compileModule(...)` naming for `.svelte.js` / `.svelte.ts` modules
- SSR output naming
- Source map filename rewriting

## Reference

### Svelte
- `reference/compiler/phases/2-analyze/index.js` - `get_component_name(filename)` and `module.scope.generate(options.name ?? component_name)`
- `reference/compiler/phases/scope.js` - `Scope.generate(preferred_name)` sanitization and conflict avoidance
- `reference/compiler/state.js` - `rootDir` adjustment applies to runtime `filename`, not the component-name derivation path
- `reference/compiler/validate-options.js` - `filename`, `rootDir`, and `name` option validation/defaults

### Our code
- `crates/svelte_compiler/src/options.rs` - `CompileOptions::component_name`
- `crates/svelte_compiler/src/lib.rs` - derived name passed directly into analyze/codegen
- `crates/svelte_codegen_client/src/lib.rs` - exported function declaration uses `ctx.state.name` verbatim
- `tasks/compiler_tests/test_v3.rs` - snapshot harness currently forces `name: Some("App")`

## Test cases

- [x] `component_name_explicit_sanitized`
- [x] `component_name_from_filename`
- [x] `component_name_index_uses_parent_dir`
- [x] `component_name_index_under_src_stays_index`
- [x] `component_name_filename_sanitized`
- [x] `component_name_filename_leading_digit_sanitized`
- [x] `compile_filename_derived_name_is_sanitized`
- [x] `compile_explicit_name_reserved_word_is_deconflicted`
- [x] `compile_explicit_name_conflict_is_deconflicted`
- [x] `compile_filename_derived_name_conflict_is_deconflicted`
- [x] `svelte_self_deprecated_uses_deconflicted_component_name`
- [x] `svelte_self_deprecated_uses_reserved_word_deconflicted_component_name`
