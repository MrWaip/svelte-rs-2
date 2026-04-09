# <svelte:options>

## Current state
- **Working**: 18/18 use cases
- **Done this session**: inline `<svelte:options accessors={true} />`, `immutable={true}`, and `preserveWhitespace={true}` now resolve through the compiler/analyze/codegen pipeline instead of only affecting diagnostics. Analyze owns the resolved legacy flags and whitespace mode, runtime-plan/codegen use them for legacy `$.push`/`$.pop`, accessor exports, immutable prop flags/coarse-grained reads, and lowering skips whitespace cleanup when requested.
- **Done previously**: inline `<svelte:options runes={false} />` overrides compile-time `CompileOptions.runes` before analysis starts, so the component follows legacy semantics even when the API forced runes mode.
- **Missing**: 0 use cases
- **Next**: no open implementation work in this spec
- Last updated: 2026-04-09

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
- [x] `runes={false}` overrides compile-time runes mode and enables legacy component semantics.
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
- [x] `preserveWhitespace={true|false}` affects template whitespace lowering and output.
- [x] `accessors={true|false}` affects legacy component accessor generation when runes are off.
- [x] `immutable={true|false}` affects legacy equality/reactivity behavior when runes are off.
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

## Test cases

- [x] `svelte_options_basic`
- [x] `css_injected`
- [x] `namespace_svg`
- [x] `namespace_mathml`
- [x] `custom_element_props`
- [x] `custom_element_props_config`
- [x] `custom_element_boolean_default`
- [x] `custom_element_exports`
- [x] `custom_element_shadow_none`
- [x] `custom_element_object_full`
- [x] `custom_element_shadow_open`
- [x] `custom_element_extend`
- [x] `custom_element_no_tag`
- [x] `host_basic`
- [x] `host_props_rest`
- [x] Parser tests for namespace/html/css/customElement/null compatibility and diagnostics, plus analyzer warning coverage
- [x] `svelte_options_runes_false_override`
- [x] `svelte_options_accessors_legacy`
- [x] `svelte_options_immutable_legacy`
- [x] `svelte_options_preserve_whitespace`
