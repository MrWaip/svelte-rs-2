# Source Maps

## Current state
- **Working**: 12/13 use cases
- **Tests**: 9/10 green
- Last updated: 2026-05-03

## Source

- `ROADMAP.md` `## Source Maps`
- Re-audit request: 2026-05-03 (`/audit source maps с нуля проведи аудит`)

## How It Works

Source maps are produced as a side-effect of two independent codegen passes:

1. **JS map.** `svelte_codegen_client::generate` / `generate_module` invoke `oxc_codegen::Codegen` over the OXC `Program`. The JS map's resolution is bounded by the `Span`s attached to OXC nodes — every node currently uses `oxc_span::SPAN` (zero), so even a fully wired sourcemap pipeline emits a near-empty skeleton until Svelte AST spans are propagated through `svelte_ast_builder`.
2. **CSS map.** `svelte_transform_css::transform_css` walks the CSS AST and prints via `svelte_css::printer::Printer`. The printer must register input-source byte boundaries at every node (analogue of MagicString's `addSourcemapLocation`) for the emitted map to carry granular segments.

`compile_module` differs from `compile`: it hardcodes `sources=['input.svelte.js']`, ignores `output_filename`, prepends a banner comment to the code, and prepends a single `;` to the `mappings` VLQ string to shift all segments down one line for the banner.

## Syntax variants

```
compile(source, { filename })                                    -> result.js = { code, map }
compile(source, { filename, outputFilename })                    -> js.map.file = outputFilename, sources via get_source_name()
compile(source, { filename, css: 'external' })                   -> result.css = { code, map, hasGlobal }
compile(source, { filename, css: 'external', cssOutputFilename })-> css.map.file = cssOutputFilename
compile(source, { filename, css: 'injected', dev: true })        -> CSS gains inline base64 sourceMappingURL comment
compile(source, { enableSourcemap: ... })                        -> diagnostic: option removed (component compile only)
compileModule(source, { filename })                              -> result.js = { code, map }, sources=['input.svelte.js'], banner ';' offset
```

## Use cases

- [x] `CompileResult` carries map-bearing fields: `js: JsOutput { code, map }` and `css: CssOutput { code, map, has_global }`. Defined in `crates/svelte_sourcemap/src/lib.rs`, re-exported from `svelte_compiler`. (test: `sourcemap_compile_js` via `sourcemap_basic`)
- [x] `CompileOptions` extended with `output_filename`, `css_output_filename`, `enable_sourcemap`, `#[serde(skip)] sourcemap_kind: SourcemapKind`. `ModuleCompileOptions` mirrors only `enable_sourcemap` and `sourcemap_kind` (correctly drops `output_filename`).
- [x] `svelte_codegen_client::generate` wires `oxc_codegen::Codegen` with `with_source_text(input)` and `source_map_path` option; captures `CodegenReturn.map`; `compile()` now passes `options.sourcemap_kind` (was hardcoded `SourcemapKind::None`). (test: `sourcemap_compile_js` via `sourcemap_basic`)
- [x] JS map carries original source in `sourcesContent[0]` via `Sourcemap::attach_sources_content` invoked from `sourcemap_finalize::finalize_js`. (test: `sourcemap_js_sources_content` via `sourcemap_sources_content`)
- [x] `output_filename` rewrites JS map `file` and `sources[0]` via `svelte_sourcemap::get_source_name(filename, output_filename)` analogue — relative path (`get_relative_path`) when `output_filename` set, basename (`get_basename`) otherwise. `Sourcemap::set_source_name` writes both `file` and `sources[0]` identically (matches reference esrap default `file == sourceMapSource`). String-only path ops, split on `[/\\]`. (test: `sourcemap_js_output_filename` via `sourcemap_output_filename_relative` + `sourcemap_output_filename_absent`)
- [x] `compile_module` JS map `sources[0]` via `get_basename(filename)` analogue (fallback `"input.svelte.js"` when `filename` is empty or `"(unknown)"` sentinel); `output_filename` deliberately ignored (`ModuleCompileOptions` has no such field). `Sourcemap::set_source_name` writes both `file` and `sources[0]`. (test: `sourcemap_compile_module_js` via `sourcemap_compile_module_basename` + `sourcemap_compile_module_fallback`)
- [x] Span propagation: original Svelte AST spans flow into generated OXC nodes so the JS map has non-skeletal mappings. Done via `svelte_parser::span_shift::SpanShifter` (a `VisitMut` over `oxc_ast_visit::visit_span`) applied at the parser boundary in every `parse_*_with_alloc`. Invariant: OXC `Span` returned by `svelte_parser` is an absolute byte offset into the full `.svelte` source. No builder-API migration was needed — synthetic nodes legitimately keep the zero `SPAN` (no source location, no mapping emitted by codegen). (test: `sourcemap_js_granular_mappings`)
  - [x] Element-identifier mapping (parser + AST): `name_span: Span` field added to `Element` and `ComponentNode` in `crates/svelte_ast/src/lib.rs`; `SvelteElement.tag_span` already covers that role for `<svelte:element>`. `crates/svelte_parser/src/lib.rs` and `handlers.rs` fill `name_span` from `tag.name_span` at all four element-construction sites (self-closing & closing-tag, RegularElement & ComponentNode). Parser unit tests `element_name_span_points_at_tag_name` and `component_name_span_points_at_tag_name` cover the contract.
  - [ ] Element-identifier mapping (codegen consumer): thread `name_span` into the `BindingIdentifier` of generated locals (`var button = ...`) at the relevant codegen sites. Currently emit goes through `anchor.rs::emit_pre_anchor` → `b.var_stmt(node_name, expr)` and the AST-node `name_span` is lost on the way. Closing requires extending `PreAnchor` with `Option<Span>` and threading it through fragment-prepare → emit; non-trivial multi-site refactor in `crates/svelte_codegen_client/src/codegen/{fragment,anchor}.rs`.
  - [x] Component block container span: synthetic component `function App($$anchor) { ... }` body block now carries `instance_script.content_span` (assigned at the single `b.function_decl(..., body_span)` site in `crates/svelte_codegen_client/src/lib.rs`). Function-bracket positions in generated JS map back into the `<script>` content. (test: `tasks/compiler_tests/sourcemap_cases/sourcemap_component_block_span` + `test_sourcemap.rs::sourcemap_component_block_span`)
- [x] CSS printer (`svelte_css::printer::Printer::print_with_sourcemap`) tracks output line/column and emits a sourcemap alongside the printed CSS; `svelte_transform_css::transform_css_with_sourcemap` returns `(code, SourceMap)` and is wired in `compile()` when `!inject_styles && sourcemap_kind.is_enabled()`. (test: `sourcemap_compile_css_external` via `sourcemap_css_external`)
  - [x] Walk every CSS AST node and register both `start` and `end` source offsets at the printer. `Printer::register_token_end(span)` complement `register_token` and is invoked after each `Rule`/`AtRule`/`Declaration` finishes printing, so the map carries token pairs per node rather than only block boundaries. (test: `sourcemap_css_granular_mappings` via `tasks/compiler_tests/sourcemap_cases/sourcemap_css_granular`)
  - [x] `svelte_transform_css::transform_css_with_sourcemap` returns `(code, map)`; `has_global` propagated separately via existing `analysis.css.has_global` data.
- [x] `css_output_filename` rewrites CSS map `file` field independently of JS; falls back to `filename` when unset. Implemented in `svelte_compiler::sourcemap_finalize::finalize_css(map, css_output_filename, filename)`, called from `compile()` after `transform_css_with_sourcemap`. (test: `sourcemap_css_output_filename` via `sourcemap_css_output_filename_set` + `sourcemap_css_output_filename_fallback`)
- [x] Dev-mode injected CSS (`css === 'injected'` + `dev: true`): emitted CSS gains trailing `\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,<map> */` (OXC `SourceMap::to_data_url()` includes `charset=utf-8`). Implemented in `compile()` CSS branch — at `inject_styles && dev && sourcemap_kind.is_enabled()` builds map via `transform_css_with_sourcemap`, applies `compact_css_for_injection`, appends `Sourcemap::to_inline_comment()`. The compiled CSS string lands in `const $$css = { hash, code }` consumed by `$.append_styles`. (test: `sourcemap_css_inline_dev` via `sourcemap_css_inline_dev_appends` + `sourcemap_css_inline_dev_skipped_when_not_dev`)
- [x] Legacy `enable_sourcemap` option triggers `DiagnosticKind::OptionsRemovedEnableSourcemap` warning during `compile()` option validation (`svelte_compiler::validate_compile_options`). `compile_module` does not call this validator and silently no-ops the option. (test: `sourcemap_legacy_enable_sourcemap_warning` via existing `tasks/diagnostic_tests/cases/options/options_removed_enable_sourcemap`)
- [x] WASM (`wasm_compiler::WasmCompileResult { js: WasmJsOutput { code, map }, css: WasmCssOutput { code, map, has_global }, ... }`) and NAPI (`napi_compiler::NativeCompileResult { js: NativeJsOutput { code, map }, css: NativeCssOutput { code, map, has_global }, ... }`) surfaces carry `map: Option<String>` (`SourceMap::to_json_string()` payload).
- [ ] `compile_module` banner `/* <basename> generated by Svelte v<VERSION> */\n` prepended to `js.code` and single `;` prepended to `mappings` VLQ string to offset all segments down one line. **Blocker**: `tasks/generate_test_cases/src/main.rs` reformats reference output through OXC `Codegen` (lines 82-89), which strips line comments — existing `cases2/module_*/case-svelte.js` snapshots therefore lack the banner. Closure requires either an infrastructure change in `generate_test_cases` (raw reference output without OXC reformat for `*.svelte.js` cases) or a documented deviation. (test: pending — depends on banner pipeline decision)

## Out of scope

- SSR-specific sourcemap behavior (SSR pipeline not yet ported)
- `print(ast) -> { code, map }` public AST-printer API — we don't expose a Svelte-AST printer
- Bundler/plugin-specific map consumption after compile
- Browser DevTools UX beyond emitting correct Source Map v3 payloads
- `preprocess()` own output pipeline and upstream preprocessor sourcemap merge — we do not implement `preprocess`, and we do not consume its produced map. `CompileOptions.sourcemap` field is dropped; bundler/plugin chains must merge upstream maps externally if they need full chain mapping.
- `customElement: true` forces `css: 'injected'` — handled by existing customElement logic; map behavior follows the injected branch automatically

## Reference

### Svelte
- `original/compiler/index.js` — `compile`, `compileModule` entry points; result assembly
- `original/compiler/types/index.d.ts` — `CompileResult`, `CompileOptions`, `SourceMap` public types
- `original/compiler/validate-options.js` — line 78 `cssOutputFilename`, line 102 `outputFilename`, line 120 `enableSourcemap` warn-removed
- `original/compiler/warnings.js` — line 569-571 `options_removed_enable_sourcemap`
- `original/compiler/phases/3-transform/index.js` — line 37 `get_source_name` for js, lines 39-44 esrap `print` with `sourceMapContent`/`sourceMapSource`, lines 88-107 `transform_module` banner + `';'` mapping prefix
- `original/compiler/phases/3-transform/css/index.js` — lines 29-73 `render_stylesheet`, lines 56-63 `MagicString.generateMap`, lines 68-70 dev inline sourceMappingURL
- `original/compiler/phases/3-transform/client/visitors/shared/fragment.js` — line 118 `b.id(name, node.name_loc)` element-identifier span
- `original/compiler/phases/3-transform/client/transform-client.js` — line 404 `component_block.loc = instance.loc`
- `original/compiler/utils/builders.js` — line 268-273 `b.id(name, loc)` accepting optional `SourceLocation`
- `original/compiler/utils/mapped_code.js` — `get_source_name` (452-454), `get_basename` (443-445), `get_relative_path` (423-437)

### Our code
- `crates/svelte_compiler/src/lib.rs:9` — `CompileResult` (currently bare `js`/`css: Option<String>`)
- `crates/svelte_compiler/src/options.rs:10` — `CompileOptions` (no map-related fields)
- `crates/svelte_compiler/src/options.rs:125` — `ModuleCompileOptions` (no map-related fields)
- `crates/svelte_codegen_client/src/lib.rs` — `build_codegen_output` plumbs `SourcemapKind` through `oxc_codegen::Codegen` with `with_source_text(component.source)` and `with_source_path(filename)`
- `crates/svelte_parser/src/span_shift.rs` — `SpanShifter` (`VisitMut` over `visit_span`) plus `wrapper_delta(absolute_start, leading_ws, prefix_len)` helper applied in every `parse_*_with_alloc`. Snippet decl uses two deltas (`NAME_PREFIX_LEN=6`, `PARAMS_PREFIX_LEN=9`) because its wrapper `const NAME = PARAMS => {}` splices ` = ` between source segments.
- `crates/svelte_transform_css/src/lib.rs:9` — `transform_css*` returning `String`
- `crates/svelte_css/src/printer.rs` — `Printer` with no position tracking
- `crates/svelte_diagnostics/src/lib.rs:458` — `OptionsRemovedEnableSourcemap` variant defined; no emit site
- `crates/wasm_compiler/src/lib.rs:23` — `WasmCompileResult` no map fields
- `crates/napi_compiler/src/lib.rs:20` — `NativeCompileResult` no map fields
- `Cargo.toml` workspace — `oxc_sourcemap` and `oxc_ast_visit` available as workspace deps

## Test cases

- [x] `sourcemap_compile_js` (`tasks/compiler_tests/sourcemap_cases/sourcemap_basic` + `test_sourcemap.rs::sourcemap_basic`)
- [x] `sourcemap_compile_css_external` (`tasks/compiler_tests/sourcemap_cases/sourcemap_css_external` + `test_sourcemap.rs::sourcemap_css_external`)
- [x] `sourcemap_js_sources_content` (`tasks/compiler_tests/sourcemap_cases/sourcemap_sources_content` + `test_sourcemap.rs::sourcemap_sources_content`)
- [x] `sourcemap_js_output_filename` (`tasks/compiler_tests/sourcemap_cases/sourcemap_output_filename` + `test_sourcemap.rs::sourcemap_output_filename_relative` + `test_sourcemap.rs::sourcemap_output_filename_absent`)
- [ ] `sourcemap_js_granular_mappings`
- [x] `sourcemap_css_output_filename` (`tasks/compiler_tests/sourcemap_cases/sourcemap_css_output_filename` + `test_sourcemap.rs::sourcemap_css_output_filename_set` + `test_sourcemap.rs::sourcemap_css_output_filename_fallback`)
- [x] `sourcemap_css_granular_mappings` (`tasks/compiler_tests/sourcemap_cases/sourcemap_css_granular` + `test_sourcemap.rs::sourcemap_css_granular_mappings`)
- [x] `sourcemap_css_inline_dev` (`tasks/compiler_tests/sourcemap_cases/sourcemap_css_inline_dev` + `test_sourcemap.rs::sourcemap_css_inline_dev_appends` + `test_sourcemap.rs::sourcemap_css_inline_dev_skipped_when_not_dev`)
- [x] `sourcemap_compile_module_js` (`tasks/compiler_tests/sourcemap_cases/sourcemap_compile_module` + `test_sourcemap.rs::sourcemap_compile_module_basename` + `test_sourcemap.rs::sourcemap_compile_module_fallback`)
- [x] `sourcemap_legacy_enable_sourcemap_warning` (`tasks/diagnostic_tests/cases/options/options_removed_enable_sourcemap`)
