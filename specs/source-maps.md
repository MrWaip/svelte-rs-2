# Source Maps

## Current state
- **Working**: 0/9 use cases
- **Partial**: 2/9 use cases
- **Missing**: 7/9 use cases
- **Next**: wire JS sourcemap output through `CompileResult`, then add source-map-aware tests before tackling CSS maps and preprocessor composition
- **Blocker**: the current compiler test harness only snapshots JS/CSS text, so the audit could not add meaningful `tasks/compiler_tests` cases for sourcemap behavior without first adding result-surface and assertion infrastructure
- Last updated: 2026-04-07

## Source

- `ROADMAP.md` `## Source Maps`
- Audit request: `$audit sourcemaps`
- Session findings: reference Svelte emits JS and CSS maps, merges preprocessor maps, and our `oxc_codegen v0.117.0` dependency already supports source-map output when spans are preserved

## Syntax variants

`compile(source, { filename }) -> result.js.code + result.js.map`
`compile(source, { filename, outputFilename }) -> result.js.map`
`compile(source, { filename, css: 'external' }) -> result.css.code + result.css.map`
`compile(source, { filename, css: 'external', cssOutputFilename }) -> result.css.map`
`compile(source, { sourcemap: <upstream map> }) -> merged result.js.map`
`compile(source, { sourcemap: <upstream map>, css: 'external' }) -> merged result.css.map`
`compileModule(source, { filename }) -> result.js.code + result.js.map`
`preprocess(...): { code, map }`
`preprocess(...): processed.code` with attached `sourceMappingURL=...`
`print(ast) -> { code, map }`

## Use cases

- [ ] `compile(...)` returns JavaScript source maps in the public result surface, not just generated JS text
- [ ] `compile(...)` returns CSS source maps for external CSS output, including the `hasGlobal` companion metadata shape used by reference Svelte
- [ ] `compileModule(...)` returns JavaScript source maps and keeps mappings correct after the prepended banner/comment offset
- [ ] `CompileOptions.sourcemap` accepts an upstream/preprocessor map and merges it into compiler-produced JS maps
- [ ] `CompileOptions.sourcemap` also merges into compiler-produced CSS maps
- [ ] `CompileOptions.outputFilename` influences JavaScript sourcemap naming/relative source paths
- [ ] `CompileOptions.cssOutputFilename` influences CSS sourcemap naming/relative source paths
- [ ] Surface the sourcemap result through the WASM API as well as the Rust API
- [ ] Add source-map-aware test fixtures/assertions for compiler output so map behavior can be regression-tested

## Out of scope

- SSR-specific sourcemap behavior
- Bundler/plugin-specific consumption of maps after compile
- Browser DevTools UX beyond emitting correct Source Map v3 payloads
- Full `print(ast)` API parity if the project does not plan to expose a public Svelte-AST printer

## Reference

- Reference compile/module options and result types: `reference/compiler/types/index.d.ts`
- Reference compile option validation: `reference/compiler/validate-options.js`
- Reference JS transform output with sourcemap naming and merge: `reference/compiler/phases/3-transform/index.js`
- Reference CSS sourcemap generation: `reference/compiler/phases/3-transform/css/index.js`
- Reference source-map merge and attached-map parsing utilities: `reference/compiler/utils/mapped_code.js`
- Reference preprocess public map surface: `reference/compiler/preprocess/public.d.ts`
- Reference parser note about script AST locations for sourcemaps: `reference/compiler/phases/1-parse/read/script.js`
- Reference Svelte AST printer docs mentioning `{ code, map }`: `reference/compiler/print/index.js`
- Local public compile result: `crates/svelte_compiler/src/lib.rs`
- Local compile/module options: `crates/svelte_compiler/src/options.rs`
- Local client codegen printer call-sites: `crates/svelte_codegen_client/src/lib.rs`
- Local CSS transform/printer pipeline: `crates/svelte_transform_css/src/lib.rs`, `crates/svelte_css/src/printer.rs`
- Local WASM result surface: `crates/wasm_compiler/src/lib.rs`
- Local diagnostics enum already containing removed `enableSourcemap`: `crates/svelte_diagnostics/src/lib.rs`

## Tasks

- API: extend `CompileOptions` with `sourcemap`, `output_filename`, and `css_output_filename` equivalents and extend `CompileResult`/WASM result types to carry map payloads
- JS codegen: switch `svelte_codegen_client::generate` and `generate_module` from returning `String` to returning code plus optional sourcemap from `oxc_codegen`
- JS codegen: preserve meaningful spans on generated OXC AST nodes so the emitted map is better than a mostly-synthetic skeleton
- CSS pipeline: introduce a map-aware CSS transform/printer result type instead of returning plain `String`
- Composition: add merge utilities for incoming preprocess/upstream maps and normalize source naming like the reference compiler
- Validation/tests: add result-level tests for map presence, source naming, merged-map paths, and representative node mappings

## Implementation order

1. JS result/API plumbing for `compile(...)`
2. JS result/API plumbing for `compileModule(...)` and WASM
3. Source-map-aware tests for JS maps
4. CSS map generation and result plumbing
5. Upstream/preprocessor map composition

## Discovered bugs

- OPEN: `oxc_codegen v0.117.0` can already emit source maps, but `svelte_codegen_client` discards that capability by returning only `.code`
- OPEN: the CSS pipeline serializes to plain `String` with no map-aware abstraction, so CSS maps need new infrastructure rather than simple plumbing
- OPEN: `DiagnosticKind::OptionsRemovedEnableSourcemap` exists locally, but the current compiler options surface does not validate or warn on legacy `enableSourcemap`

## Test cases

- Existing sourcemap-focused compiler cases: none found
- Existing result-level sourcemap tests: none found in `crates/svelte_compiler/src/tests.rs`
- Audit note: no `tasks/compiler_tests` cases were added in this run because the current harness only snapshots `js`/`css` text and cannot assert maps yet; this is tracked as a missing infrastructure use case above
