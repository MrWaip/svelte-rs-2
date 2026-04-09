# ComponentNode

## Current state
- **Working**: 12/12 component-tag use cases
- **Completed slice:** component event dev-mode handler parity in `svelte_codegen_client`
- **Done in this slice:** component `on:` handlers now reuse the shared event-handler builder for dev `$.apply(...)` wrapping and call memoization, component `$$events` preserves duplicate same-name handlers via arrays like the reference, and regular component calls now emit dev `$.add_svelte_meta(...)` with `componentTag`
- **Next:** feature complete for current `component-node` spec; if more reference gaps are found later, add them as new unchecked use cases before resuming
- **Verification:** `just test-case component_events`, `just test-case component_events_dev_apply`, and `just test-compiler` passed on 2026-04-09
- Last updated: 2026-04-09

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
- [x] `on:` directives on components serialize into `$$events`, including dev-mode shared-handler wrapping parity (tests: `component_events`, `component_events_dev_apply`)
- [x] Child nodes with `slot="name"` serialize into named `$$slots.<name>` instead of default children (tests: `component_named_slot`)
- [x] Runes-mode dotted or stateful component references lower through `$.component(...)` (tests: `component_dynamic_dotted`)
- [x] Analyze emits component-specific validation/warnings for invalid directives and attribute edge cases (component-tag directives/modifiers, direct attribute-name/value checks, duplicate named slots, and explicit default-slot conflicts)

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
- [x] `component_events_dev_apply`
- [x] `component_named_slot`
- [x] `component_dynamic_dotted`
- [x] analyzer unit tests: component invalid directive, component `on:` modifier validation, component illegal colon warning, component unquoted attribute sequence
- [x] analyzer unit tests: duplicate named slot, explicit default-slot conflict, distinct named slots, whitespace-only default-slot false-positive guard
