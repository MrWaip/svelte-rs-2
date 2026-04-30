# ComponentNode

## Current state
- **Working**: 12/14 use cases
- **Tests**: 19/21 green
- Last updated: 2026-04-30

## Source

- User request: `/audit component`

## Syntax variants

- `<Component />`
- `<Component></Component>`
- `<Component foo="x" bar={expr} {...spread} />`
- `<Component bind:x={value} bind:this={ref} />`
- `<Component on:done={handler} />`
- `<Component>{@snippet children()}</Component>`
- `<foo.bar />`
- `<registry_name.Widget />`
- `<registry.Widget />`
- `<Derived_1 bind:this={refs[1]} />`
- `{#if cond}{@const Const_0 = Widget}<Const_0 bind:this={refs[0]} />{/if}`

## Use cases

- [x] Basic uppercase component tag lowers to `Component($$anchor, {})`
- [x] Non-self-closing component tag lowers the same as self-closing form
- [x] String, boolean, expression, shorthand, concatenation, and spread props preserve order (tests: `component_props`, `component_spread_props`)
- [x] `bind:this` on components lowers through `$.bind_this(...)` (tests: `component_bind_this`, `component_bind_this_variants`)
- [x] Non-`this` component bindings lower to getter/setter props (tests: `component_bind_prop_forward`)
- [x] Snippet children and snippet props lower correctly (tests: `component_snippet_prop`, `component_snippet_with_children`, `component_multiple_snippets`, `component_snippet_only`)
- [x] Complex expression props memoize when needed (tests: `component_prop_has_call`, `component_prop_has_call_multi`, `component_prop_has_call_mixed`, `component_prop_memo_state`)
- [ ] Inline callback component props that mutate `$state` (for example `onclick={() => count++}`) should stay as direct callback expressions instead of being memoized through derived getter wrappers (test: `diagnose_component_onclick_state`)
- [x] `on:` directives on components serialize into `$$events`, including dev-mode shared-handler wrapping parity (tests: `component_events`, `component_events_dev_apply`)
- [x] Runes-mode dotted or stateful component references lower through `$.component(...)`, including dotted roots whose first segment is a lowercase JS identifier containing `_` or `$` and dynamic refs read from `$$props` (tests: `component_dynamic_dotted`, `component_dynamic_dotted_identifier_root`, `component_dynamic_props_access`)
- [x] Runes-mode dotted dynamic component refs whose root binding is non-normal use the same template binding read semantics as ordinary template reads, including `$props()`-backed roots like `<registry.Widget />` (test: `component_dynamic_dotted_props_root`)
- [x] Runes-mode local component bindings whose tag names include `_` or digits, including `<Derived_1>` from `$derived(Widget)` and `<Const_0>` from `{@const}`, parse and lower through the dynamic-component path with `bind:this` (test: `component_local_underscored_bind_this`)
- [x] Analyze emits component-specific validation/warnings for invalid directives and attribute edge cases (component-tag directives/modifiers plus direct attribute-name/value checks)
- [ ] Dev-mode default `children` slot snippet passed to a component is wrapped with `$.wrap_snippet(App, ($$anchor, $$slotProps) => { ... })` (test: `diagnose_runes_dev_ce_benchmark`)

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
- [x] `component_bind_this`
- [x] `component_bind_prop_forward`
- [x] `component_snippet_prop`
- [x] `component_snippet_with_children`
- [x] `component_multiple_snippets`
- [x] `component_spread_props`
- [x] `component_events`
- [x] `component_events_dev_apply`
- [x] `component_dynamic_dotted`
- [x] `component_dynamic_dotted_identifier_root`
- [x] `component_dynamic_props_access`
- [x] `component_dynamic_dotted_props_root`
- [x] `component_local_underscored_bind_this`
- [ ] `diagnose_component_onclick_state`
- [x] analyzer unit tests: component invalid directive, component `on:` modifier validation, component illegal colon warning, component unquoted attribute sequence
- [x] `component_invalid_directive_use`
- [x] `component_on_modifier_only_allows_once`
- [ ] `diagnose_runes_dev_ce_benchmark`
