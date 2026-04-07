# Custom Elements

## Current state
- **Working**: 10/15 use cases
- **Missing**: 5 use cases
- **Next**: Fix CE slot lowering/runtime metadata first, then CE default CSS injection, then aliased-prop CE accessors, then `customElement.shadow` object support. Parser/analyzer object-form validation can follow after the runtime-visible gaps.
- Last updated: 2026-04-07

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
`<slot />`
`<slot name="actions" />`
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
- [ ] `customElement={{ shadow: ShadowRootInit }}` forwards the full object instead of collapsing it to `{ mode: "open" }`.
- [x] `customElement={{ props }}` emits explicit prop metadata including `attribute`, `reflect`, and `type`.
- [x] Boolean-valued prop fallbacks infer `type: "Boolean"` for uncovered CE props.
- [ ] Prop aliases from `$props()` destructuring are exposed under the public prop name and still get CE accessor/export wrapping in the generated component.
- [x] Exported functions are listed in the custom-element accessor/export array.
- [ ] `<slot>` and named `<slot name="...">` are lowered to CE slot calls and emitted in the wrapper slot-name array.
- [x] Omitting `tag` emits the constructor expression without `customElements.define(...)`.
- [x] `customElement={{ extend }}` forwards the wrapper extension expression to `$.create_custom_element(...)`.
- [ ] CE mode injects component CSS into JS/shadow-root output even without inline `css="injected"`.
- [x] `$host()` is available when compiling as a custom element, and CE rest-prop lowering excludes `$$host`.
- [ ] Object-form `customElement` validation for `props` shape, allowed `type` values, and `shadow` value parity still lives in analyze-only extraction instead of the parser path used by the reference compiler.

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

## Tasks

- Parser: move object-form `customElement` structural validation closer to the reference parse phase instead of accepting any object expression span.
- Analyze: preserve parser-owned CE config facts in `AnalysisData` without re-deciding structural validity from raw JS nodes.
- Analyze: keep CE-specific warnings (`options_missing_custom_element`, `$props()` identifier/rest warnings, `$host()` placement) aligned with reference behavior.
- Codegen: emit `ShadowRootInit` object literals verbatim when supplied, instead of the current `open|none` enum collapse.
- Codegen: keep CE props, slots, exports, tag/no-tag, and extend behavior covered by compiler snapshots.
- Tests: maintain compiler snapshot coverage for runtime-visible CE output and keep parser/analyzer unit tests for diagnostics-only behavior.

## Implementation order

1. Expand parser/analyze data model to represent object-form `shadow` and validated CE object fields.
2. Update CE codegen to emit the richer shadow config and continue sourcing `extend` from the parser-owned expression.
3. Add parser/analyzer unit coverage for invalid object-form CE configs that compiler snapshots cannot express.
4. Unignore failing compiler CE parity cases once the data model and codegen are fixed.

## Discovered bugs

- OPEN: `crates/svelte_parser/src/svelte_elements.rs` accepts any object-form `customElement={...}` and defers validation, unlike the reference parser which validates `tag`, `props`, and `shadow` structure during parse.
- OPEN: `crates/svelte_parser/src/types.rs` only models `shadow` as `Open | None`, so `ShadowRootInit` object syntax cannot round-trip into codegen.
- OPEN: `crates/svelte_analyze/src/utils/ce_config.rs` silently ignores unsupported object properties instead of surfacing reference-equivalent diagnostics.
- OPEN: CE `<slot>` content is still emitted as literal DOM `<slot>` nodes and `$.create_custom_element(..., [], ...)` instead of reference `$.slot(...)` calls plus slot-name metadata.
- OPEN: CE mode does not auto-promote component CSS into injected JS/shadow-root output unless inline `css="injected"` is set explicitly.
- OPEN: Aliased CE props (`let { count: total } = $props()`) do not receive the reference custom-element accessor/export wrapper even though the metadata key resolves to `count`.

## Test cases

- Existing passing compiler cases: `custom_element_props`, `custom_element_props_config`, `custom_element_boolean_default`, `custom_element_exports`, `custom_element_shadow_none`, `custom_element_object_full`, `custom_element_shadow_open`, `custom_element_extend`, `custom_element_no_tag`, `host_basic`, `host_props_rest`
- Existing unit coverage: parser tests for CE tag/null compatibility and analyzer tests for `$props()` CE warnings, missing compile-flag warnings, and `$host()` placement
- Added during this audit: `custom_element_prop_alias`, `custom_element_slots`, `custom_element_css_default_injected`, `custom_element_shadow_object`
- Failing audit cases:
- `custom_element_slots` — ignored as `missing: custom-element slot lowering and slot metadata emission (analyze/codegen)` — effort: needs infrastructure
- `custom_element_css_default_injected` — ignored as `missing: custom-element default CSS injection (compiler/codegen)` — effort: moderate
- `custom_element_prop_alias` — ignored as `missing: aliased prop accessors in custom elements (analyze/codegen)` — effort: moderate
- `custom_element_shadow_object` — ignored as `missing: ShadowRootInit object emission for customElement.shadow (analyze/codegen)` — effort: moderate
