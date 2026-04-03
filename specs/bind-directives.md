# bind:*

## Current state
- **Working**: all listed client-side bind codegen and analyzer validation use cases are implemented and covered, including getter/setter pairs, bind diagnostics, attribute-coupled validation, rest-pattern warnings, and focused compiler coverage for `<textarea bind:value>` plus resize-observer bindings
- **Missing**: no known gaps in the current client-side scope
- **Next**: feature complete for the current `bind:*` scope; keep parity checks in `/qa` and revisit only if new ignored compiler cases or validation mismatches appear
- Last updated: 2026-04-03

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

- `[x]` `bind:value` on `<input>` and `<select>` including shorthand and function bindings
  Existing tests: `bind_directives`, `bind_directives_extended`, `bind_function_value`, `bind_select_value`
- `[x]` `bind:value` on `<textarea>`
  Existing tests: `bind_textarea_value`, `textarea_child_value_dynamic`
- `[x]` `bind:checked`, `bind:group`, and `bind:files`
  Existing tests: `bind_directives_extended`, `bind_function_checked`, `bind_group_*`, `bind_files`, `push_binding_group_order`
- `[x]` Contenteditable bindings: `bind:innerHTML`, `bind:innerText`, `bind:textContent`
  Existing tests: `bind_content_editable`, `bind_contenteditable_flag`, `bind_multiple_on_element`
- `[x]` Element size bindings: `bind:clientWidth`, `bind:clientHeight`, `bind:offsetWidth`, `bind:offsetHeight`
  Existing test: `bind_element_size`
- `[x]` Resize observer bindings: `bind:contentRect`, `bind:contentBoxSize`, `bind:borderBoxSize`, `bind:devicePixelContentBoxSize`
  Existing tests: `bind_resize_observer`, `bind_resize_observer_border_box_size`, `bind_resize_observer_device_pixel_content_box_size`
- `[x]` `bind:this` on elements, components, `<svelte:element>`, and getter/setter sequence form
  Existing tests: `bind_this`, `bind_this_sequence`, `component_bind_this`, `component_bind_this_variants`, `svelte_element_bind`
- `[x]` Media read/write bindings
  Existing tests: `bind_media_rw`, `bind_media_ro`, `bind_media_property`, `bind_img`
- `[x]` `<svelte:window>` and `<svelte:document>` bindings
  Existing tests: `svelte_window_bind_scroll`, `svelte_window_bind_size`, `svelte_window_reactive`, `svelte_window_bind_online`, `svelte_window_combined`, `svelte_document_bindings`, `svelte_document_combined`
- `[x]` `bind:focused`
  Existing test: `bind_focused`
- `[x]` Bind validation parity in analyze:
  `bind_invalid_name`, `bind_invalid_target`, `bind_invalid_expression`, `bind_invalid_parens`, `bind_invalid_value`, `bind_group_invalid_expression`, `bind_group_invalid_snippet_parameter`
- `[x]` Attribute validation coupled to bindings:
  `attribute_contenteditable_missing`, `attribute_contenteditable_dynamic`, `attribute_invalid_type`, `attribute_invalid_multiple`
- `[x]` Runes-mode validation for binding each-item arguments
  `each_item_invalid_assignment`
- `[x]` Warning parity for rest-pattern each bindings
  `bind_invalid_each_rest`

## Reference

- `reference/compiler/phases/bindings.js` — canonical binding property matrix
- `reference/compiler/phases/2-analyze/visitors/BindDirective.js` — analyzer validation and group-binding metadata rules
- `reference/compiler/errors.js` — bind and attribute diagnostic definitions
- `reference/compiler/warnings.js` — `bind_invalid_each_rest`
- `reference/compiler/phases/3-transform/client/visitors/BindDirective.js` — reference client transform surface
- `crates/svelte_parser/src/scanner/mod.rs` — parser support for `BindDirective`
- `crates/svelte_analyze/src/passes/template_semantic.rs` — bind expressions participate in template semantic analysis
- `crates/svelte_analyze/src/passes/bind_semantics.rs` — bind/group metadata currently precomputed for codegen
- `crates/svelte_analyze/src/tests.rs` — analyzer validation coverage, including ignored gaps
- `crates/svelte_codegen_client/src/template/bind.rs` — regular element bind codegen
- `crates/svelte_codegen_client/src/template/component.rs` — component `bind:this` and prop binding codegen
- `crates/svelte_codegen_client/src/template/svelte_window.rs` — `<svelte:window>` binding codegen
- `crates/svelte_codegen_client/src/template/svelte_document.rs` — `<svelte:document>` binding codegen
- `tasks/compiler_tests/cases2/` — current positive coverage for bind codegen parity

## Tasks

1. `[x]` Port reference analyzer validation from `BindDirective.js` into `svelte_analyze`
   Files: `crates/svelte_analyze/src/validate/*` and any required shared template-validation helpers
2. `[x]` Add analyzer tests for missing bind diagnostics and warnings
   Start with invalid name/target/expression/value plus contenteditable/input/select validation
3. `[x]` Add focused compiler cases for positive-but-uncovered bindings
   Start with `<textarea bind:value>` and resize-observer `borderBoxSize` / `devicePixelContentBoxSize`
4. `[x]` Re-audit ROADMAP bindings after validation and test coverage land

## Implementation order

1. Bind diagnostics in analyze
2. Attribute-coupled validation (`contenteditable`, static `type`, static `multiple`)
3. Runes/each-specific validation and warnings
4. Positive coverage backfill for uncovered binding names

## Discovered bugs

- FIXED: `crates/svelte_analyze` bind diagnostics and warnings now hit active validation paths in `template_validation`
- FIXED: focused compiler coverage exists for `<textarea bind:value>` and the resize-observer bindings `borderBoxSize` / `devicePixelContentBoxSize`

## Test cases

- Existing compiler cases:
  `bind_content_editable`, `bind_contenteditable_flag`, `bind_directives`, `bind_directives_extended`, `bind_element_size`, `bind_files`, `bind_focused`, `bind_function_checked`, `bind_function_value`, `bind_group_each`, `bind_group_each_var`, `bind_group_each_var_keyed`, `bind_group_keyed_each`, `bind_group_nested_each`, `bind_group_radio_basic`, `bind_group_value_attr`, `bind_img`, `bind_media_property`, `bind_media_ro`, `bind_media_rw`, `bind_multiple_on_element`, `bind_property`, `bind_resize_observer`, `bind_resize_observer_border_box_size`, `bind_resize_observer_device_pixel_content_box_size`, `bind_select_value`, `bind_textarea_value`, `bind_this`, `bind_this_sequence`, `bind_use_deferral`, `component_bind_prop_forward`, `component_bind_this`, `component_bind_this_variants`, `push_binding_group_order`, `svelte_document_bindings`, `svelte_element_bind`, `svelte_window_bind_online`, `svelte_window_bind_scroll`, `svelte_window_bind_size`, `textarea_child_value_dynamic`
- Analyzer coverage:
  `validate_bind_invalid_name`, `validate_bind_invalid_name_with_special_element_candidates`, `validate_bind_invalid_target`, `validate_bind_invalid_expression`, `validate_bind_invalid_parens`, `validate_bind_invalid_value`, `validate_bind_group_invalid_expression`, `validate_bind_group_invalid_snippet_parameter`, `validate_bind_invalid_each_rest`, `validate_attribute_contenteditable_missing`, `validate_attribute_contenteditable_dynamic`, `validate_attribute_invalid_type`, `validate_attribute_invalid_multiple`, `validate_bind_member_expression_no_error`, `validate_bind_getter_setter_no_error`
