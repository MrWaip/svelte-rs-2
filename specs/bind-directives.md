# bind:*

## Current state
- **Working**: all existing bind codegen and analyzer validation use cases are implemented and covered, except dev-mode ownership validation for component prop bindings
- **Completed (2026-04-11)**: `Bindable Prop bind:checked Lowering`. `props_bindable_checkbox_disabled_shorthand_ts` now matches reference for the live gap: analyze marks bind directives that target prop sources, and regular-element `bind:checked` passes the bindable prop accessor directly to `$.bind_checked(...)`. The `{disabled}` shorthand in the same repro remains owned by `specs/attributes-spreads.md`.
- `bind_select_static_option_value` landed: any `<option value="...">` (StringAttribute) now drops the literal `value=` from the template HTML and emits `option.value = option.__value = "<lit>"` in JS init, matching the reference compiler's `needs_special_value_handling` rule. Routed via `element_needs_var` in analyze + extended `value` attr arms in `template/html.rs` and `template/attributes.rs` (reuses the existing `bind:group` `__value` emission).
- **Next:** return to dev-mode `$$ownership_validator.binding(...)` coverage for component bindings; `<option value={expr}>` (Expression / Concatenation) still emits the legacy path and should be ported separately.
- **Non-goals for the completed bindable-prop slice:** `{disabled}` attr shorthand lowering, other prop-source binds such as `bind:value`, dev-mode ownership validation, and `<option value={expr}>`.
- Last updated: 2026-04-11

## Source

User request: `$audit bind:*`

ROADMAP.md — Bindings

## Syntax variants

- `bind:name`
- `bind:name={identifier}`
- `bind:name={member.expression}`
- `bind:name={get, set}` function bindings without extra surrounding parentheses
- Element bindings on regular elements, `<svelte:window>`, `<svelte:document>`, `<svelte:element>`, and components (`bind:this`, component prop bindings)
- Group bindings inside `{#each}` and keyed `{#each}` blocks

## Use cases

- [x] `bind:value` on `<input>` and `<select>` including shorthand and function bindings
  Existing tests: `bind_directives`, `bind_directives_extended`, `bind_function_value`, `bind_select_value`
- [x] `<option value="...">` (StringAttribute) drops the literal `value=` from the template HTML and emits `option.value = option.__value = "<lit>"` JS initializer, matching reference `needs_special_value_handling`. Always-on, not gated on parent `<select bind:value>`. (test: `bind_select_static_option_value`)
- [ ] `<option value={expr}>` and `<option value="prefix-{expr}">` (Expression / Concatenation): currently fall through the generic dynamic-attribute path; reference routes them through `build_element_special_value_attribute` with the `__value` cache + effect wrapping. No failing test exists yet — port when a fixture lands. (M)
- [x] `bind:value` on `<textarea>`
  Existing tests: `bind_textarea_value`, `textarea_child_value_dynamic`
- [x] `bind:checked`, `bind:group`, and `bind:files`
  Existing tests: `bind_directives_extended`, `bind_function_checked`, `bind_group_*`, `bind_files`, `push_binding_group_order`
- [x] Regular-element `bind:checked` targeting a `$bindable` prop source from `$props()` passes the prop accessor directly to `$.bind_checked(...)` instead of lowering through rune getter/setter closures (test: `props_bindable_checkbox_disabled_shorthand_ts`)
- [x] Contenteditable bindings: `bind:innerHTML`, `bind:innerText`, `bind:textContent`
  Existing tests: `bind_content_editable`, `bind_contenteditable_flag`, `bind_multiple_on_element`
- [x] Element size bindings: `bind:clientWidth`, `bind:clientHeight`, `bind:offsetWidth`, `bind:offsetHeight`
  Existing test: `bind_element_size`
- [x] Resize observer bindings: `bind:contentRect`, `bind:contentBoxSize`, `bind:borderBoxSize`, `bind:devicePixelContentBoxSize`
  Existing tests: `bind_resize_observer`, `bind_resize_observer_border_box_size`, `bind_resize_observer_device_pixel_content_box_size`
- [x] `bind:this` on elements, components, `<svelte:element>`, and getter/setter sequence form
  Existing tests: `bind_this`, `bind_this_sequence`, `component_bind_this`, `component_bind_this_variants`, `svelte_element_bind`
- [x] Media read/write bindings
  Existing tests: `bind_media_rw`, `bind_media_ro`, `bind_media_property`, `bind_img`
- [x] `<svelte:window>` and `<svelte:document>` bindings
  Existing tests: `svelte_window_bind_scroll`, `svelte_window_bind_size`, `svelte_window_reactive`, `svelte_window_bind_online`, `svelte_window_combined`, `svelte_document_bindings`, `svelte_document_combined`
- [x] `bind:focused`
  Existing test: `bind_focused`
- [x] Bind validation parity in analyze:
  `bind_invalid_name`, `bind_invalid_target`, `bind_invalid_expression`, `bind_invalid_parens`, `bind_invalid_value`, `bind_group_invalid_expression`, `bind_group_invalid_snippet_parameter`
- [x] Attribute validation coupled to bindings:
  `attribute_contenteditable_missing`, `attribute_contenteditable_dynamic`, `attribute_invalid_type`, `attribute_invalid_multiple`
- [x] Runes-mode validation for binding each-item arguments
  `each_item_invalid_assignment`
- [x] Warning parity for rest-pattern each bindings
  `bind_invalid_each_rest`
- [ ] Dev-mode component prop bindings validate ownership via `$$ownership_validator.binding(...)`

## Reference

- `reference/compiler/phases/bindings.js` — canonical binding property matrix
- `reference/compiler/phases/2-analyze/visitors/BindDirective.js` — analyzer validation and group-binding metadata rules
- `reference/compiler/errors.js` — bind and attribute diagnostic definitions
- `reference/compiler/warnings.js` — `bind_invalid_each_rest`
- `reference/compiler/phases/3-transform/client/visitors/BindDirective.js` — reference client transform surface
- `reference/compiler/phases/3-transform/client/visitors/shared/component.js` — `$$ownership_validator.binding(...)` for component bindings
- `crates/svelte_parser/src/scanner/mod.rs` — parser support for `BindDirective`
- `crates/svelte_analyze/src/passes/template_semantic.rs` — bind expressions participate in template semantic analysis
- `crates/svelte_analyze/src/passes/bind_semantics.rs` — bind/group metadata currently precomputed for codegen
- `crates/svelte_analyze/src/tests.rs` — analyzer validation coverage, including ignored gaps
- `crates/svelte_codegen_client/src/template/bind.rs` — regular element bind codegen
- `crates/svelte_codegen_client/src/template/component.rs` — component `bind:this` and prop binding codegen
- `crates/svelte_codegen_client/src/template/svelte_window.rs` — `<svelte:window>` binding codegen
- `crates/svelte_codegen_client/src/template/svelte_document.rs` — `<svelte:document>` binding codegen
- `tasks/compiler_tests/cases2/` — current positive coverage for bind codegen parity

## Test cases

- [x] `bind_content_editable`
- [x] `bind_contenteditable_flag`
- [x] `bind_directives`
- [x] `bind_directives_extended`
- [x] `bind_element_size`
- [x] `bind_files`
- [x] `bind_focused`
- [x] `bind_function_checked`
- [x] `bind_function_value`
- [x] `bind_group_each`
- [x] `bind_group_each_var`
- [x] `bind_group_each_var_keyed`
- [x] `bind_group_keyed_each`
- [x] `bind_group_nested_each`
- [x] `bind_group_radio_basic`
- [x] `bind_group_value_attr`
- [x] `bind_img`
- [x] `bind_media_property`
- [x] `bind_media_ro`
- [x] `bind_media_rw`
- [x] `bind_multiple_on_element`
- [x] `bind_property`
- [x] `bind_resize_observer`
- [x] `bind_resize_observer_border_box_size`
- [x] `bind_resize_observer_device_pixel_content_box_size`
- [x] `bind_select_value`
- [x] `bind_textarea_value`
- [x] `bind_this`
- [x] `bind_this_sequence`
- [x] `bind_use_deferral`
- [x] `component_bind_prop_forward`
- [x] `component_bind_this`
- [x] `component_bind_this_variants`
- [x] `push_binding_group_order`
- [x] `props_bindable_checkbox_disabled_shorthand_ts`
- [x] `svelte_document_bindings`
- [x] `svelte_element_bind`
- [x] `svelte_window_bind_online`
- [x] `svelte_window_bind_scroll`
- [x] `svelte_window_bind_size`
- [x] `textarea_child_value_dynamic`
- [x] `validate_bind_invalid_name`
- [x] `validate_bind_invalid_name_with_special_element_candidates`
- [x] `validate_bind_invalid_target`
- [x] `validate_bind_invalid_expression`
- [x] `validate_bind_invalid_parens`
- [x] `validate_bind_invalid_value`
- [x] `validate_bind_group_invalid_expression`
- [x] `validate_bind_group_invalid_snippet_parameter`
- [x] `validate_bind_invalid_each_rest`
- [x] `validate_attribute_contenteditable_missing`
- [x] `validate_attribute_contenteditable_dynamic`
- [x] `validate_attribute_invalid_type`
- [x] `validate_attribute_invalid_multiple`
- [x] `validate_bind_member_expression_no_error`
- [x] `validate_bind_getter_setter_no_error`
- [x] `bind_select_static_option_value`
