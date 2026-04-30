# Custom Elements

## Current state
- **Working**: 11/17 use cases
- **Tests**: 13/16 green
- Last updated: 2026-04-30

## Source

- `ROADMAP.md` `Custom Elements`
- User request: `/audit Custom Elements`

## Syntax variants

`<svelte:options customElement="my-element" />`
`<svelte:options customElement={null} />`
`<svelte:options customElement={{ tag: "my-element" }} />`
`<svelte:options customElement={{ shadow: "open" }} />`
`<svelte:options customElement={{ shadow: "none" }} />`
`<svelte:options customElement={{ shadow: { mode: "open", delegatesFocus: true } }} />`
`<svelte:options customElement={{ props: { foo: { type: "String", reflect: true, attribute: "foo" } } }} />`
`<svelte:options customElement={{ extend: wrap }} />`
`<svelte:options customElement={{ tag: "my-element", props: { count: { reflect: true } } }} />`
`<svelte:options customElement={{ tag: "my-element", props: { count: { attribute: "data-count" } } }} />`
`<svelte:options customElement={{ tag: "my-element", props: { count: { type: "Number" } } }} />`
`let { active = false } = $props();`
`let { count: total } = $props();`
`let props = $props();`
`let { x, ...rest } = $props();`
`$host()`

## Use cases

- [x] `customElement="tag-name"` emits `customElements.define(tag, $.create_custom_element(...))`.
- [x] `customElement={{ tag }}` object form resolves the tag from the parsed config and defines the element.
- [x] `customElement={{ shadow: "open" }}` emits the default open shadow-root config.
- [x] `customElement={{ shadow: "none" }}` omits the shadow-root config argument.
- [x] `customElement={{ shadow: ShadowRootInit }}` forwards the full object instead of collapsing it to `{ mode: "open" }`.
- [x] `customElement={{ props }}` emits explicit prop metadata including `attribute`, `reflect`, and `type`.
- [x] Boolean-valued prop fallbacks infer `type: "Boolean"` for uncovered CE props.
- [ ] Prop aliases from `$props()` destructuring are exposed under the public prop name and still get CE accessor/export wrapping in the generated component.
- [x] Exported functions are listed in the custom-element accessor/export array.
- [x] Omitting `tag` emits the constructor expression without `customElements.define(...)`.
- [x] `customElement={{ extend }}` forwards the wrapper extension expression to `$.create_custom_element(...)`.
- [ ] CE mode injects component CSS into JS/shadow-root output even without inline `css="injected"`.
- [x] `$host()` is available when compiling as a custom element, and CE rest-prop lowering excludes `$$host`.
- [ ] Object-form `customElement` validation for `props` shape, allowed `type` values, and `shadow` value parity still lives in analyze-only extraction instead of the parser path used by the reference compiler.
- [ ] Compile-option `customElement: true` (without an inline `<svelte:options customElement>`) emits the `$.create_custom_element(App, propsMeta, [], exports, { mode: "open" })` constructor call at module scope. (test: `diagnose_runes_dev_ce_benchmark`)
- [ ] CE/legacy instance-script exports (`export const`, `export function`) appear on `$$exports` as `get NAME() { return NAME; }` accessors plus a trailing `...$.legacy_api()` spread instead of plain shorthand properties. (test: `diagnose_runes_dev_ce_benchmark`)
- [ ] CE rest-prop lowering `$.rest_props($$props, [...keys], "rest")` includes `"$$host"` in the excluded-key list and passes the destructured-binding label `"rest"` as the trailing argument. (test: `diagnose_runes_dev_ce_benchmark`)

## Out of scope

- SSR behavior for custom elements
- Browser/runtime lifecycle semantics of the generated wrapper class
- Migration-guide caveats that do not change compiler output or diagnostics in this repo

## Reference

- Reference docs: `reference/docs/07-misc/04-custom-elements.md`
- Reference docs: `reference/docs/05-special-elements/07-svelte-options.md`
- Reference docs: `reference/docs/02-runes/08-$host.md`
- Reference parser: `reference/compiler/phases/1-parse/read/options.js`
- Reference compile-option merge: `reference/compiler/index.js`
- Reference analyze: `reference/compiler/phases/2-analyze/index.js`
- Reference analyze warning: `reference/compiler/phases/2-analyze/visitors/VariableDeclarator.js`
- Reference client transform: `reference/compiler/phases/3-transform/client/transform-client.js`
- Rust AST config types: `crates/svelte_ast/src/lib.rs`
- Rust parser extraction: `crates/svelte_parser/src/svelte_elements.rs`
- Rust CE tag validation: `crates/svelte_parser/src/lib.rs`
- Rust parsed CE config model: `crates/svelte_parser/src/types.rs`
- Rust CE config extraction: `crates/svelte_analyze/src/utils/ce_config.rs`
- Rust analyzer execution pass: `crates/svelte_analyze/src/passes/executor.rs`
- Rust analyzer warnings: `crates/svelte_analyze/src/validate/mod.rs`
- Rust CE codegen: `crates/svelte_codegen_client/src/custom_element.rs`
- Rust compiler entry: `crates/svelte_compiler/src/lib.rs`
- Compiler integration tests: `tasks/compiler_tests/test_v3.rs`

## Test cases

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
- [x] Parser tests for custom-element tag/null compatibility and analyzer tests for `$props()` custom-element warnings, missing compile-flag warnings, and `$host()` placement
- [ ] `custom_element_css_default_injected` — ignored as `missing: custom-element default CSS injection (compiler/codegen)` — effort: moderate
- [ ] `custom_element_prop_alias` — ignored as `missing: aliased prop accessors in custom elements (analyze/codegen)` — effort: moderate
- [x] `custom_element_shadow_object`
- [ ] `diagnose_runes_dev_ce_benchmark`
