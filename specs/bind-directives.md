# bind:*

## Current state
- **Working**: 26/26 use cases
- **Tests**: 73/73 green
- Last updated: 2026-05-01

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
- [x] `<option value={expr}>` (Expression): currently falls through the generic dynamic-attribute path; reference routes through `build_element_special_value_attribute` with the `__value` cache (init `{}`) + effect wrapping `option.value = (option.__value = expr) ?? ""`. (test: `option_expr_value`, S)
- [x] `<option value="prefix-{expr}">` (Concatenation): same routing but `is_defined` evaluates to true so no `??` wrap, and concat is interpolated via template literal. Multi-option coalescing into single `template_effect` is part of this slice. (test: `option_concat_value`, `option_expr_value_multi`)
- [x] `bind:value` on `<textarea>`
  Existing tests: `bind_textarea_value`, `textarea_child_value_dynamic`
- [x] `bind:checked`, `bind:group`, and `bind:files`
  Existing tests: `bind_directives_extended`, `bind_function_checked`, `bind_group_*`, `bind_files`, `push_binding_group_order`
- [x] `bind:group` with auto-subscribed stores in same component: the synthesized `const binding_group = [];` declaration must be emitted AFTER the store getter constants and `const [$$stores, $$cleanup] = $.setup_stores();` (reference order = `store_setup` → `store_init` → `group_binding_declarations`). Currently emitted before them. (test: `bind_group_order_with_stores`, S)
- [x] Regular-element `bind:checked` targeting a `$bindable` prop source from `$props()` passes the prop accessor directly to `$.bind_checked(...)` instead of lowering through rune getter/setter closures (test: `props_bindable_checkbox_disabled_shorthand_ts`)
- [x] Contenteditable bindings: `bind:innerHTML`, `bind:innerText`, `bind:textContent`
  Existing tests: `bind_content_editable`, `bind_contenteditable_flag`, `bind_multiple_on_element`
- [x] Element size bindings: `bind:clientWidth`, `bind:clientHeight`, `bind:offsetWidth`, `bind:offsetHeight`
  Existing test: `bind_element_size`
- [x] Resize observer bindings: `bind:contentRect`, `bind:contentBoxSize`, `bind:borderBoxSize`, `bind:devicePixelContentBoxSize`
  Existing tests: `bind_resize_observer`, `bind_resize_observer_border_box_size`, `bind_resize_observer_device_pixel_content_box_size`
- [x] `bind:this` on elements, components, `<svelte:element>`, and getter/setter sequence form
  Existing tests: `bind_this`, `bind_this_sequence`, `component_bind_this`, `component_bind_this_variants`, `svelte_element_bind`
- [x] `bind:this` on a regular element with children and a class/style directive must be emitted after the element's `$.reset(...)`
  Existing test: `bind_this_with_children_and_class_directive`
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
- [x] Dev-mode component prop bindings validate ownership via `$$ownership_validator.binding(...)` — analyze flag `output.needs_component_bind_ownership` raised on PropSource bind in dev+runes; codegen wraps component call with `$$ownership_validator.binding(name, Comp, source)` prefix inside a BlockStatement. (test: `bind_component_prop_dev_ownership`)
- [x] Dev-mode component prop bindings to non-bindable plain `prop` (not `$bindable`) also emit `$$ownership_validator.binding(...)`. Reference fires for both `bindable_prop` and plain `prop`. Already covered: analyze promotes non-bindable prop to `PropBindingKind::Source { bindable: false }` when used as bind target, so the existing `PropSource` mode path catches it. (test: `bind_component_plain_prop_dev_ownership`)
- [x] Dev-mode `<svelte:component>` prop bindings emit `$$ownership_validator.binding(name, intermediate_name, source)` using the synthesized intermediate ident inside the dynamic component callback, not the literal tag name. (test: `bind_dynamic_component_dev_ownership`)
- [x] Dev-mode `$$ownership_validator.binding(...)` honors `<!-- svelte-ignore ownership_invalid_binding -->` on the binding directive — when ignored, analyze sets `requires_ownership_emit = false` on the `ComponentPropKind::Bind` and codegen skips both the validator var decl and the binding stmt. (test: `bind_component_dev_ownership_ignore`)
- [x] Component prop bindings with explicit identifier source (`<Comp bind:value={foo}>` where local var `foo` ≠ prop name `value`) — analyze stores trimmed `expr_text` as `expr_name` for simple-identifier expressions and uses it for both binding-semantics lookup and codegen source ident. (test: `bind_component_explicit_source`)
- [x] Dev-mode element bind helpers (`$.bind_value`, `$.bind_checked`, `$.bind_group`, `$.bind_select_value`, `$.bind_content_editable`, `$.bind_volume`, `$.bind_paused`, `$.bind_element_size`) take named `function get() {...}` / `function set($$value) {...}` declarations as get/set callbacks instead of arrow expressions. Implemented at shorthand bind lowering in `transform/template_entry.rs`: in dev mode synthesize `named_function_expr("get", …)` / `named_function_expr("set", …)` instead of `b.thunk` / `b.arrow_expr`. (test: `bind_value_dev_named_fns`)

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
- [x] `bind_this_with_children_and_class_directive`
- [x] `bind_use_deferral`
- [x] `component_bind_prop_forward`
- [x] `component_bind_this`
- [x] `component_bind_this_variants`
- [x] `push_binding_group_order`
- [x] `bind_group_order_with_stores`
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
- [x] `validate_bind_plain_let_is_valid`
- [x] `validate_bind_getter_setter_without_parens`
- [x] `validate_bind_group_invalid_expression`
- [x] `validate_bind_sequence_reports_all_relevant_errors`
- [x] `validate_bind_group_invalid_snippet_parameter`
- [x] `validate_bind_invalid_each_rest`
- [x] `validate_bind_checked_radio_target`
- [x] `validate_bind_files_wrong_input_type`
- [x] `validate_attribute_contenteditable_missing`
- [x] `validate_attribute_contenteditable_dynamic`
- [x] `validate_attribute_invalid_type`
- [x] `validate_attribute_invalid_multiple`
- [x] `validate_bind_member_expression_no_error`
- [x] `validate_bind_getter_setter_no_error`
- [x] `bind_select_static_option_value`
- [x] `option_expr_value`
- [x] `option_concat_value`
- [x] `option_expr_value_multi`
- [x] `bind_value_dev_named_fns`
- [x] `bind_component_prop_dev_ownership`
- [x] `bind_component_plain_prop_dev_ownership`
- [x] `bind_dynamic_component_dev_ownership`
- [x] `bind_component_dev_ownership_ignore`
- [x] `bind_component_explicit_source`
