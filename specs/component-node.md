# ComponentNode

## Current state
- **Working**: 8/12 component-tag use cases
- **Missing**: component events via `$$events`, named-slot child grouping, and runes-mode dynamic component lowering for dotted/stateful component references
- **Next**: port analyze metadata for dynamic component tags first, then fill component codegen gaps for `$$events` and slot grouping
- Last updated: 2026-04-01

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
- [ ] `on:` directives on components serialize into `$$events`
- [ ] Child nodes with `slot="name"` serialize into named `$$slots.<name>` instead of default children
- [ ] Runes-mode dotted or stateful component references lower through `$.component(...)`
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

## Tasks

- [ ] Analyze: add component dynamicity metadata so runes-mode dotted/stateful component references choose the dynamic `$.component(...)` path
- [ ] Analyze: validate component directives/attribute edge cases and emit existing diagnostics instead of silently ignoring them
- [ ] Client codegen: collect `on:` directives on component tags into `$$events`
- [ ] Client codegen: group child nodes by `slot="name"` and emit named `$$slots` entries instead of always lowering everything into `children`
- [ ] Tests: keep the audit bounded to one focused case per missing behavior

## Implementation order

1. Dynamic component metadata in analyze.
2. `$$events` serialization for component `on:` directives.
3. Named slot child grouping in component codegen.
4. Component-specific diagnostics once behavior parity is in place.

## Discovered bugs

- OPEN: component codegen ignores `Attribute::OnDirectiveLegacy`, so `<Component on:foo={...} />` currently drops component events entirely.
- OPEN: component children are always lowered as default content; codegen does not partition children by `slot="name"`.
- OPEN: `ComponentNode` has no dynamic-component metadata, so dotted/stateful component tags cannot switch to the reference compiler's `$.component(...)` path in runes mode.

## Test cases

- Existing covered compiler cases:
  - `component_basic`
  - `component_non_self_closing`
  - `component_props`
  - `component_children`
  - `component_element_children`
  - `component_bind_this`
  - `component_bind_prop_forward`
  - `component_snippet_prop`
  - `component_snippet_with_children`
  - `component_multiple_snippets`
  - `component_spread_props`
- Added during this audit:
  - `component_events`
  - `component_named_slot`
  - `component_dynamic_dotted`
- Recommended next command:
  - `port specs/component-node.md`
