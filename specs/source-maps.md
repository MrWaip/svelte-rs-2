# Source Maps

## Current state
- **Working**: 0/13 use cases
- **Tests**: 0/11 green
- Last updated: 2026-04-08

## Source

- `ROADMAP.md` `## Source Maps`
- Re-audit request: 2026-04-08 (`/audit source-maps проведи повторный ресерч`)

## Syntax variants

```
compile(source, { filename })                                    -> result.js = { code, map }
compile(source, { filename, outputFilename })                    -> js.map.file = outputFilename, sources via get_source_name()
compile(source, { filename, sourcemap: <upstream> })             -> js.map merged with upstream preprocessor map
compile(source, { filename, outputFilename, sourcemap })         -> js.map merged + renamed
compile(source, { filename, css: 'external' })                   -> result.css = { code, map, hasGlobal }
compile(source, { filename, css: 'external', cssOutputFilename })-> css.map.file = cssOutputFilename
compile(source, { filename, css: 'external', sourcemap })        -> css.map merged with upstream
compile(source, { filename, css: 'injected', dev: true })        -> CSS output gains inline base64 sourceMappingURL comment
compile(source, { enableSourcemap: ... })                        -> diagnostic: option removed
compileModule(source, { filename })                              -> result.js = { code, map }, sources=['input.svelte.js'], banner ';' offset
preprocess(source, [...]) -> Processed                           -> { code, map, dependencies?, attributes? }
```

## Use cases

- [ ] `compile(...)` returns JavaScript sourcemap on `CompileResult.js` (map fields + struct type), not just a `String` — currently `svelte_codegen_client::generate` calls `Codegen::default().build(&program).code` and drops `.map`. **needs infrastructure** (test: `sourcemap_compile_js`)
- [ ] `compile(...)` returns CSS sourcemap on `CompileResult.css` including `hasGlobal` companion flag — `svelte_transform_css::transform_css` returns `String`; `svelte_css::printer::Printer` has no span tracking. **needs infrastructure** (test: `sourcemap_compile_css_external`)
- [ ] JS sourcemap carries original source in `sourcesContent` so downstream tooling doesn't need filesystem lookup (reference sets `sourceMapContent: source` on `esrap.print`). **quick fix** once map plumbing exists (test: `sourcemap_js_sources_content`)
- [ ] `CompileOptions.output_filename` rewrites JS map `file` and `sources[]` via `get_source_name(filename, output_filename, 'input.svelte')`. **moderate** (test: `sourcemap_js_output_filename`)
- [ ] `CompileOptions.css_output_filename` rewrites CSS map `file` independently of JS. **moderate** (test: `sourcemap_css_output_filename`)
- [ ] `CompileOptions.sourcemap` (upstream/preprocessor map) merged into JS output via remapping utility equivalent to `@jridgewell/remapping` — needs porting of `reference/compiler/utils/mapped_code.js::merge_with_preprocessor_map`. **needs infrastructure** (test: `sourcemap_merge_upstream_js`)
- [ ] `CompileOptions.sourcemap` also merged into CSS output, with source basename normalization and rebasing. **needs infrastructure** (test: `sourcemap_merge_upstream_css`)
- [ ] CSS printer registers node boundaries analogously to reference `addSourcemapLocation()` so emitted map is high-resolution, not skeletal — requires `svelte_css::printer` to preserve/emit spans. **needs infrastructure** (test: `sourcemap_css_granular_mappings`)
- [ ] Dev-mode injected CSS (`css === 'injected'` + `dev: true`) appends inline `/*# sourceMappingURL=data:application/json;base64,... */` comment to emitted CSS text. **moderate** once CSS map plumbing exists (test: `sourcemap_css_inline_dev`)
- [ ] `compileModule(...)` returns JS sourcemap with `sources=['input.svelte.js']` (hardcoded; `output_filename` intentionally ignored) and mappings leading with `;` to account for prepended banner comment. **needs infrastructure** (test: `sourcemap_compile_module_js`)
- [ ] Span preservation across analyze → codegen: generated OXC AST nodes currently use `Span::default()` / `SPAN` widely; without meaningful spans the emitted map degenerates to a near-empty skeleton. Audit and propagate original Svelte AST spans (~32+ call sites in builder). **needs infrastructure** (test: covered indirectly by `sourcemap_js_granular_mappings`)
- [ ] WASM result surface (`wasm_compiler::WasmCompileResult`) exposes map payloads alongside `js`/`css`. **moderate** once Rust surface is ready (test: N/A — covered by Rust tests)
- [ ] Legacy `enableSourcemap` option produces `DiagnosticKind::OptionsRemovedEnableSourcemap` warning during options validation — diagnostic variant already exists in `svelte_diagnostics` but nothing reaches it. **quick fix** (test: `sourcemap_legacy_enable_sourcemap_warning`)

## Out of scope

- SSR-specific sourcemap behavior (SSR pipeline not yet ported)
- `print(ast) -> { code, map }` public AST printer API — we don't expose a Svelte-AST printer
- Bundler/plugin-specific map consumption after compile
- Browser DevTools UX beyond emitting correct Source Map v3 payloads
- `preprocess()` own output pipeline — we don't implement `preprocess` itself; only merging its incoming map via `CompileOptions.sourcemap`

## Reference

### Svelte
- `reference/compiler/index.js` — `compile` / `compileModule` entry, result assembly
- `reference/compiler/types/index.d.ts` — `CompileResult`, `CompileOptions`, `SourceMap` public types
- `reference/compiler/validate-options.js` — legacy `enableSourcemap` removal, `outputFilename`/`cssOutputFilename` validation
- `reference/compiler/phases/3-transform/index.js` — JS map generation via `esrap.print`, `sourceMapContent`/`sourceMapSource`, `merge_with_preprocessor_map`, module banner offset
- `reference/compiler/phases/3-transform/css/index.js` — CSS map generation via MagicString, `addSourcemapLocation`, `hasGlobal`, dev-mode inline map
- `reference/compiler/utils/mapped_code.js` — `MappedCode`, `combine_sourcemaps`, `sourcemap_add_offset`, `apply_preprocessor_sourcemap`, `get_source_name`, `merge_with_preprocessor_map`
- `reference/compiler/preprocess/index.js` — preprocessor map accumulation (informational; we don't port `preprocess` itself)

### Our code
- `crates/svelte_compiler/src/lib.rs` — `CompileResult`, `compile`, `compile_module`
- `crates/svelte_compiler/src/options.rs` — `CompileOptions`, `ModuleCompileOptions` (missing map-related fields)
- `crates/svelte_codegen_client/src/lib.rs` — `generate`, `generate_module` (both drop `CodegenReturn.map`)
- `crates/svelte_transform_css/src/lib.rs` — CSS transform returning plain `String`
- `crates/svelte_css/src/printer.rs` — CSS printer without span tracking
- `crates/wasm_compiler/src/lib.rs` — `WasmCompileResult` without map fields
- `crates/svelte_diagnostics/src/lib.rs` — `DiagnosticKind::OptionsRemovedEnableSourcemap` variant (unused)

## Test cases

- [ ] `sourcemap_compile_js`
- [ ] `sourcemap_compile_css_external`
- [ ] `sourcemap_js_sources_content`
- [ ] `sourcemap_js_output_filename`
- [ ] `sourcemap_css_output_filename`
- [ ] `sourcemap_merge_upstream_js`
- [ ] `sourcemap_merge_upstream_css`
- [ ] `sourcemap_css_granular_mappings`
- [ ] `sourcemap_css_inline_dev`
- [ ] `sourcemap_compile_module_js`
- [ ] `sourcemap_legacy_enable_sourcemap_warning`
