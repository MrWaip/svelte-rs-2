# ComponentNode

## Current state
- **Working**: 11/12 component-tag use cases
- **Next:** implement component-specific validation/warnings for invalid directives and attribute edge cases in `svelte_analyze`
- Last updated: 2026-04-03

## Source

- User request: `/audit component`

## Syntax variants

- `<Component />`
- `<Component></Component>`
- `<Component foo="x" bar={expr} {...spread} />`
- `<Component bind:x={value} bind:this={ref} />`
- `<Component on:done={handler} />`
- `<Component>{@snippet children()}</Component>`
- `<Component><div slot="footer" /></Component>`
- `<foo.bar />`

## Use cases

- [x] Basic uppercase component tag lowers to `Component($$anchor, {})`
- [x] Non-self-closing component tag lowers the same as self-closing form
- [x] String, boolean, expression, shorthand, concatenation, and spread props preserve order (tests: `component_props`, `component_spread_props`)
- [x] `bind:this` on components lowers through `$.bind_this(...)` (tests: `component_bind_this`, `component_bind_this_variants`)
- [x] Non-`this` component bindings lower to getter/setter props (tests: `component_bind_prop_forward`)
- [x] Default children lower to `children` prop plus `$$slots.default` (tests: `component_children`, `component_element_children`)
- [x] Snippet children and snippet props lower correctly (tests: `component_snippet_prop`, `component_snippet_with_children`, `component_multiple_snippets`, `component_snippet_only`)
- [x] Complex expression props memoize when needed (tests: `component_prop_has_call`, `component_prop_has_call_multi`, `component_prop_has_call_mixed`, `component_prop_memo_state`)
- [x] `on:` directives on components serialize into `$$events` (tests: `component_events`)
- [x] Child nodes with `slot="name"` serialize into named `$$slots.<name>` instead of default children (tests: `component_named_slot`)
- [x] Runes-mode dotted or stateful component references lower through `$.component(...)` (tests: `component_dynamic_dotted`)
- [ ] Analyze emits component-specific validation/warnings for invalid directives and attribute edge cases

## Reference

- Reference analyze:
  - `reference/compiler/phases/2-analyze/visitors/Component.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/component.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/attribute.js`
- Reference client transform:
  - `reference/compiler/phases/3-transform/client/visitors/Component.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- Rust implementation:
  - `crates/svelte_parser/src/lib.rs`
  - `crates/svelte_analyze/src/passes/template_scoping.rs`
  - `crates/svelte_analyze/src/passes/element_flags.rs`
  - `crates/svelte_codegen_client/src/template/component.rs`
  - `crates/svelte_codegen_client/src/template/traverse.rs`
  - `crates/svelte_diagnostics/src/lib.rs`
  - `tasks/compiler_tests/cases2/component_*`

## Test cases

- [x] `component_basic`
- [x] `component_non_self_closing`
- [x] `component_props`
- [x] `component_children`
- [x] `component_element_children`
- [x] `component_bind_this`
- [x] `component_bind_prop_forward`
- [x] `component_snippet_prop`
- [x] `component_snippet_with_children`
- [x] `component_multiple_snippets`
- [x] `component_spread_props`
- [x] `component_events`
- [x] `component_named_slot`
- [x] `component_dynamic_dotted`
