# <svelte:options>

## Current state
- **Working**: 14/18 use cases
- **Missing**: 4 use cases
- **Next**: Fix option precedence first (`runes={false}`), then wire legacy `accessors`/`immutable`, then thread `preserveWhitespace` through lowering/codegen.
- Last updated: 2026-04-07

## Source

- `ROADMAP.md` `<svelte:options>`
- User request: `/audit <svelte:options>`

## Syntax variants

`<svelte:options runes={true} />`
`<svelte:options runes={false} />`
`<svelte:options namespace="html" />`
`<svelte:options namespace="svg" />`
`<svelte:options namespace="mathml" />`
`<svelte:options namespace="http://www.w3.org/2000/svg" />`
`<svelte:options css="injected" />`
`<svelte:options customElement="my-element" />`
`<svelte:options customElement={null} />`
`<svelte:options customElement={{ tag: "my-element" }} />`
`<svelte:options customElement={{ shadow: "open" }} />`
`<svelte:options customElement={{ shadow: "none" }} />`
`<svelte:options customElement={{ shadow: { mode: "open", delegatesFocus: true } }} />`
`<svelte:options customElement={{ props: { foo: { type: "String", reflect: true, attribute: "foo" } } }} />`
`<svelte:options customElement={{ extend: wrap }} />`
`<svelte:options preserveWhitespace={true} />`
`<svelte:options preserveWhitespace={false} />`
`<svelte:options immutable={true} />`
`<svelte:options immutable={false} />`
`<svelte:options accessors={true} />`
`<svelte:options accessors={false} />`
`<svelte:options tag="my-element" />`

## Use cases

- [x] `runes={true}` forces runes mode for an otherwise plain component.
- [ ] `runes={false}` overrides compile-time runes mode and enables legacy component semantics.
- [x] `namespace="html"` parses and behaves as the default HTML namespace.
- [x] `namespace="svg"` switches root template creation to SVG mode.
- [x] `namespace="mathml"` switches root template creation to MathML mode.
- [x] `css="injected"` injects component CSS instead of returning external CSS.
- [x] `customElement="tag-name"` wraps the component in `$.create_custom_element(...)` and defines the tag.
- [x] `customElement={{ tag, shadow: "open" }}` emits an open shadow root config.
- [x] `customElement={{ tag, shadow: "none" }}` omits the shadow root config.
- [x] `customElement={{ tag, props }}` emits explicit custom-element props metadata.
- [x] `customElement={{ tag, extend }}` forwards the custom element wrapper expression.
- [x] `customElement={{ shadow: ... }}` without `tag` emits the constructor without `customElements.define(...)`.
- [x] `customElement={null}` is ignored for Svelte 4 compatibility.
- [ ] `preserveWhitespace={true|false}` affects template whitespace lowering and output.
- [ ] `accessors={true|false}` affects legacy component accessor generation when runes are off.
- [ ] `immutable={true|false}` affects legacy equality/reactivity behavior when runes are off.
- [x] Runes-mode warnings are emitted for deprecated `accessors` and `immutable`, and inline `customElement` warns when the compile flag is missing.
- [x] Parser diagnostics cover unknown attributes, invalid values, deprecated `tag`, invalid custom-element names, and forbidden children.

## Out of scope

- SSR-specific `<svelte:options>` behavior
- CLI/API-level compile option validation outside inline component options
- Legacy Svelte 4 AST compatibility beyond the explicitly supported `customElement={null}` path

## Reference

- Reference docs: `reference/docs/05-special-elements/07-svelte-options.md`
- Reference docs: `reference/docs/07-misc/04-custom-elements.md`
- Reference docs: `reference/docs/07-misc/07-v5-migration-guide.md`
- Reference parser: `reference/compiler/phases/1-parse/read/options.js`
- Reference parser extraction: `reference/compiler/phases/1-parse/index.js`
- Reference analyze warnings: `reference/compiler/phases/2-analyze/index.js`
- Reference client transform: `reference/compiler/phases/3-transform/client/transform-client.js`
- Rust AST storage: `crates/svelte_ast/src/lib.rs`
- Rust parser extraction: `crates/svelte_parser/src/svelte_elements.rs`
- Rust parser tests: `crates/svelte_parser/src/tests.rs`
- Rust analyze pass: `crates/svelte_analyze/src/passes/executor.rs`
- Rust analyze warnings: `crates/svelte_analyze/src/validate/mod.rs`
- Rust CE config extraction: `crates/svelte_analyze/src/utils/ce_config.rs`
- Rust compiler entry: `crates/svelte_compiler/src/lib.rs`
- Rust codegen namespace selection: `crates/svelte_codegen_client/src/template/mod.rs`
- Rust codegen CE emission: `crates/svelte_codegen_client/src/custom_element.rs`
- Compiler integration tests: `tasks/compiler_tests/test_v3.rs`

## Tasks

- Parser: keep `SvelteOptions` extraction and diagnostics aligned with the reference parser, including static-value restrictions and custom-element object validation.
- Analyze: merge inline `runes`, `accessors`, `immutable`, and `customElement` semantics into `AnalysisData` instead of relying only on top-level `CompileOptions`.
- Analyze: thread `preserveWhitespace` through lowering/template execution so whitespace trimming follows inline options.
- Codegen: consume analysis-owned legacy option state for accessor generation and immutable-mode behavior.
- Compiler: define precedence between compile options and inline `<svelte:options>` once and pass resolved values into analyze/codegen consistently.
- Tests: keep parser/analyzer unit tests for diagnostics and add compiler snapshot tests for option-driven behavior changes.

## Implementation order

1. Resolve option precedence in `svelte_compiler` / analyze entry points.
2. Thread resolved runes / legacy flags / whitespace into analyze.
3. Update codegen and lowering to use analysis-owned resolved options.
4. Unignore failing compiler tests and add any remaining validation coverage.

## Discovered bugs

- OPEN: `component.options.runes` is parsed but never merged into `AnalyzeOptions`; `compile()` always uses `CompileOptions.runes.unwrap_or(true)`.
- OPEN: `component.options.preserve_whitespace` is parsed but not consumed by analyze or codegen.
- OPEN: `component.options.accessors` and `component.options.immutable` only participate in warning validation; legacy behavior still follows top-level compile options only.

## Test cases

- Existing passing compiler cases: `svelte_options_basic`, `css_injected`, `namespace_svg`, `namespace_mathml`, `custom_element_props`, `custom_element_props_config`, `custom_element_boolean_default`, `custom_element_exports`, `custom_element_shadow_none`, `custom_element_object_full`, `custom_element_shadow_open`, `custom_element_extend`, `custom_element_no_tag`, `host_basic`, `host_props_rest`
- Existing unit coverage: parser tests for namespace/html/css/customElement/null compatibility/diagnostics and analyzer tests for warning behavior
- Added during this audit: `svelte_options_runes_false_override`, `svelte_options_accessors_legacy`, `svelte_options_immutable_legacy`, `svelte_options_preserve_whitespace`
- Failing audit cases:
- `svelte_options_runes_false_override` — ignored as `missing: inline runes=false override precedence (compiler/analyze)` — effort: moderate
- `svelte_options_accessors_legacy` — ignored as `missing: inline accessors option in legacy mode (codegen)` — effort: moderate
- `svelte_options_immutable_legacy` — ignored as `missing: inline immutable option in legacy mode (analyze/codegen)` — effort: moderate
- `svelte_options_preserve_whitespace` — ignored as `missing: inline preserveWhitespace option plumbing (analyze/codegen)` — effort: needs infrastructure
