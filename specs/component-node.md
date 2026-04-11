# ComponentNode

## Current state
- **Working**: 14/14 component-tag use cases
- **Current slice:** dotted JS-identifier component roots and shared template binding reads
- **Completed slice:** component event dev-mode handler parity in `svelte_codegen_client`
- **Done in this slice:** parser tag scanning now accepts dotted component roots whose first segment is a lowercase JS identifier containing `_` or `$`, such as `<registry_name.Widget />`, while still rejecting lowercase plain element names like `<div_foo>`; shared template binding read classification now lives in `ComponentScoping` and is reused by analyze, transform, and client codegen instead of being duplicated across phases
- **Why this slice came next:** post-implementation review found one remaining parser gap for lowercase JS-identifier dotted roots and one architecture gap where analyze and transform still duplicated the same template binding read rules
- **Repro:** `just test-case component_dynamic_dotted_identifier_root` and `just test-case component_dynamic_props_access`
- **Next:** `component-node` use cases are fully covered again; if future reference gaps are found, record them as new unchecked use cases before resuming
- **Verification:** `cargo test -p svelte_parser component_name_with_underscore`, `cargo test -p svelte_parser dotted_component_name_with_lowercase_identifier_root`, `cargo test -p svelte_parser lowercase_tag_name_with_underscore_is_rejected`, `cargo test -p svelte_analyze component_rune_bindings_are_dynamic`, `cargo test -p svelte_analyze component_prop_binding_uses_props_access_ref`, `just test-case component_dynamic_dotted_identifier_root`, `just test-case component_dynamic_props_access`, `just test-case component_dynamic_dotted`, `just test-case component_local_underscored_bind_this`, `just test-parser`, `just test-analyzer`, and `just test-compiler` passed on 2026-04-11
- Last updated: 2026-04-11

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
- `<registry_name.Widget />`
- `<Derived_1 bind:this={refs[1]} />`
- `{#if cond}{@const Const_0 = Widget}<Const_0 bind:this={refs[0]} />{/if}`

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
- [x] Runes-mode dotted or stateful component references lower through `$.component(...)`, including dotted roots whose first segment is a lowercase JS identifier containing `_` or `$` and dynamic refs read from `$$props` (tests: `component_dynamic_dotted`, `component_dynamic_dotted_identifier_root`, `component_dynamic_props_access`)
- [x] Runes-mode local component bindings whose tag names include `_` or digits, including `<Derived_1>` from `$derived(Widget)` and `<Const_0>` from `{@const}`, parse and lower through the dynamic-component path with `bind:this` (test: `component_local_underscored_bind_this`)
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
- [x] `component_dynamic_dotted_identifier_root`
- [x] `component_dynamic_props_access`
- [x] `component_local_underscored_bind_this`
- [x] analyzer unit tests: component invalid directive, component `on:` modifier validation, component illegal colon warning, component unquoted attribute sequence
- [x] analyzer unit tests: duplicate named slot, explicit default-slot conflict, distinct named slots, whitespace-only default-slot false-positive guard
