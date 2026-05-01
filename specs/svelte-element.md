# Svelte Element

## Current state
- **Working**: 14/14 use cases
- **Tests**: 21/21 green
- Last updated: 2026-05-01

## Source

- ROADMAP Special Elements item: `<svelte:element>`
- Request: `/audit <svelte:element>`

## Syntax variants

- `<svelte:element this={tag} />`
- `<svelte:element this={tag}></svelte:element>`
- `<svelte:element this={tag}>children</svelte:element>`
- `<svelte:element this={null} />`
- `<svelte:element this={"div"} />`
- `<svelte:element this="div" />`
- `<svelte:element this={tag} bind:this={el} />`
- `<svelte:element this={tag} onclick={handler} />`
- `<svelte:element this={tag} on:click={handler} />`
- `<svelte:element this={tag} {...props} />`
- `<svelte:element this={tag} class:active={cond} />`
- `<svelte:element this={tag} style:color={value} />`
- `<svelte:element this={tag} xmlns="http://www.w3.org/2000/svg" />`
- `<svelte:element this={tag} xmlns={ns} />`
- `<svelte:element this={await getTag()} />`
- `<svelte:element />`
- `<svelte:element this />`

## Use cases

- [x] Dynamic tag expressions parse and compile to `$.element` (tests: `svelte_element_basic`, `svelte_element_self_closing`, `svelte_element_children_expr`, `svelte_element_in_if`)
- [x] Nullish tags skip rendering without breaking surrounding output (test: `svelte_element_null_tag`)
- [x] Generic attributes, spreads, and event handlers are applied through the dynamic-element path (tests: `svelte_element_attributes`, `svelte_element_spread`, `svelte_element_onclick`)
- [x] `bind:this` is supported on `<svelte:element>` (test: `svelte_element_bind`)
- [x] Class and style directives route through the dedicated dynamic-element helpers (tests: `svelte_element_class_directive`, `svelte_element_style_directive`, `svelte_element_static_class_attr`)
- [x] Static `xmlns="http://www.w3.org/2000/svg"` flips the namespace flag passed to `$.element` (test: `svelte_element_xmlns`)
- [x] Async tag expressions are lowered through the async wrapper path (test: `async_svelte_element`)
- [x] Legacy string `this="div"` still compiles compatibly (test: `svelte_element_static_tag`)
- [x] Dynamic `xmlns={ns}` is forwarded as the runtime namespace thunk argument to `$.element` (test: `svelte_element_dynamic_xmlns`)
- [x] Dev mode emits `$.validate_dynamic_element_tag` before creating the element (test: `svelte_element_dev_invalid_tag`)
- [x] Dev mode emits `$.validate_void_dynamic_element` when a dynamic element with children could become void (test: `svelte_element_dev_void_children`)
- [x] Analyzer emits `svelte_element_missing_this` error when `this` attribute is absent (test: `svelte_element_missing_this`)
- [x] Analyzer emits `svelte_element_missing_this` error for bare boolean `this` (test: `svelte_element_missing_this_boolean`)
- [x] Analyzer emits `svelte_element_invalid_this` warning for string-literal `this="..."` while still compiling (test: `svelte_element_invalid_this_string`; existing `svelte_element_static_tag` remains green)

## Out of scope

- SSR output parity
- Legacy `<svelte:element this={'slot'} />` ownership moved to `specs/legacy-slots.md`
- Full CSS prune/scoping parity beyond `<svelte:element>`-specific behavior

## Reference

- Reference compiler:
  - `reference/docs/05-special-elements/06-svelte-element.md`
  - `reference/docs/07-misc/07-v5-migration-guide.md`
  - `reference/compiler/phases/1-parse/state/element.js`
  - `reference/compiler/phases/2-analyze/visitors/SvelteElement.js`
  - `reference/compiler/phases/3-transform/client/visitors/SvelteElement.js`
  - `reference/compiler/warnings.js`
  - `reference/compiler/errors.js`
- Our implementation:
  - `crates/svelte_parser/src/svelte_elements.rs`
  - `crates/svelte_parser/src/attr_convert.rs`
  - `crates/svelte_analyze/src/passes/template_validation.rs`
  - `crates/svelte_codegen_client/src/template/svelte_element.rs`
  - `crates/svelte_codegen_client/src/template/attributes.rs`
  - `tasks/compiler_tests/test_v3.rs`

## Test cases

- [x] `svelte_element_basic`
- [x] `svelte_element_self_closing`
- [x] `svelte_element_attributes`
- [x] `svelte_element_spread`
- [x] `svelte_element_onclick`
- [x] `svelte_element_bind`
- [x] `svelte_element_null_tag`
- [x] `svelte_element_xmlns`
- [x] `svelte_element_children_expr`
- [x] `svelte_element_in_if`
- [x] `svelte_element_class_directive`
- [x] `svelte_element_style_directive`
- [x] `svelte_element_static_class_attr`
- [x] `svelte_element_static_tag`
- [x] `async_svelte_element`
- [x] `svelte_element_dynamic_xmlns`
- [x] `svelte_element_dev_invalid_tag`
- [x] `svelte_element_dev_void_children`
- [x] `svelte_element_missing_this` (diagnostic)
- [x] `svelte_element_missing_this_boolean` (diagnostic)
- [x] `svelte_element_invalid_this_string` (diagnostic)
