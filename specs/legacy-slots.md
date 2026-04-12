# Legacy slots

## Current state
- **Working**: 29/29 use cases
- **Tests**: 54/54 green
- Last updated: 2026-04-12

## Source
- `ROADMAP.md` legacy item: `<slot>` + `let:` + `<svelte:fragment>` + `slot attribute`
- User request: `$audit <slot> + let: + <svelte:fragment> + slot attribute`

## Syntax variants
```svelte
<slot />
<slot name="footer" />
<slot>fallback</slot>
<slot name="footer">fallback</slot>
<slot item={entry} />
<slot {item} />
<slot {...props} />
<svelte:options customElement="my-element" /><slot />
<svelte:options customElement="my-element" /><slot name="actions" />
{#if $$slots.description}<slot name="description" />{/if}
<Component>default slot content</Component>
<Component let:item>default slot content</Component>
<Component let:item={processed}>default slot content</Component>
<Component><div slot="item">named slot content</div></Component>
<Component><div slot="item" let:item>{item.text}</div></Component>
<Component><svelte:fragment slot="item">named slot content</svelte:fragment></Component>
<Component><svelte:fragment slot="item" let:item>{item.text}</svelte:fragment></Component>
<Component><Child slot="footer" /></Component>
<Child slot="footer" let:item>{item}</Child>
```

## Use cases

- [x] Dedicated AST/parser infrastructure exists for legacy slot shapes instead of generic `Element`/attribute payloads
  - [x] `<slot>` is represented as a dedicated AST node at parse time instead of a generic `Element` (tests: `legacy_slot_element_converts_to_dedicated_node`, `slot_named_fallback`, `warn_slot_deprecated`)
  - [x] Analyze/codegen consume the dedicated `<slot>` AST node instead of re-discovering slot semantics from generic lowered `Element` assumptions (tests: `slot_element_legacy_root_fragment_uses_dedicated_lowered_item`, `legacy_slot_dev_mixed`, `warn_slot_deprecated`, `slot_named_fallback`)
  - [x] `<svelte:fragment>` is represented as a dedicated AST node at parse time instead of a generic `Element` (tests: `legacy_svelte_fragment_converts_to_dedicated_node`, `svelte_fragment_named_slot`)
  - [x] Analyze/codegen consume the dedicated `<svelte:fragment>` AST node instead of relying on generic lowered `Element` assumptions (tests: `component_named_slot_mapping_uses_svelte_fragment_legacy_wrapper_id`, `svelte_fragment_named_slot`)
  - [x] `let:` is represented as a dedicated AST directive at parse time instead of a generic attribute/directive payload (tests: `let_directive_legacy_without_expression`, `let_directive_legacy_with_expression`, `let_directive_legacy_converts_to_dedicated_attribute`)
- [x] Default component children lower to `children` plus `$$slots.default` for legacy child-content interop (tests: `component_children`, `component_element_children`)
- [x] Default `<slot>` lowers to `$.slot(..., "default", {}, fallback)` and keeps optional fallback content intact (test: warn_slot_deprecated)
- [x] Named `<slot name="...">` lowers correctly with fallback content (test: slot_named_fallback)
- [x] Direct child elements with `slot="..."` lower into parent `$$slots` entries (test: component_named_slot)
- [x] Direct child `<svelte:fragment slot="...">` lowers into parent `$$slots` entries without wrapper DOM (test: svelte_fragment_named_slot)
- [x] Child components with `slot="..."` participate in named-slot grouping instead of receiving a plain `slot` prop (tests: component_child_slot_attribute, svelte_self_slot)
- [x] Default-slot bindings remain scoped to the default slot and are not visible inside named-slot content, matching the Svelte 4 migration note (test: component_default_slot_bindings_do_not_leak_into_named_slot_scope)
- [x] `<slot>` emits slot props from attributes/spreads instead of always passing `{}` while excluding `name` from the props object, lowering spreads through `$.spread_props(...)`, and memoizing dynamic call-valued props through the legacy slot prelude when needed (tests: slot_props_default, slot_props_spread, slot_props_dynamic_state, slot_props_dynamic_call)
- [x] Parent default-slot `let:` directives lower to derived reads from `$$slotProps` inside the generated slot function, including alias form `let:item={processed}` (tests: component_default_slot_let, component_default_slot_let_alias)
- [x] Named-slot `let:` directives on direct child elements lower inside the generated named-slot function, including object destructuring and multiple `let:` directives on the same element (tests: component_named_slot_let_element, component_named_slot_let_element_destructure, component_named_slot_let_element_multiple)
- [x] Named-slot `let:` directives on `<svelte:fragment>` lower inside the generated named-slot function, including object destructuring (tests: component_named_slot_let_fragment, component_named_slot_let_fragment_destructure)
- [x] Direct `$$slots` reads lower through sanitized legacy slot bindings instead of unresolved raw identifiers
  - [x] Template direct `$$slots` reads lower through `$.sanitize_slots($$props)` so conditional checks like `$$slots.description` work in template code, including the reference compiler's untracked read wrapper (tests: `legacy_slots_if`, `legacy_slots_template_reads_require_sanitized_slots_binding`, `legacy_slot_elements_do_not_require_sanitized_slots_binding`)
  - [x] Instance-script direct `$$slots` reads inject `$.sanitize_slots($$props)` and keep the `$$props` function parameter when script reads are the only legacy special-variable consumer (tests: `legacy_slots_script_basic`, `legacy_slots_script_reads_require_sanitized_slots_binding`)
- [x] Custom-element `<slot>` and named `<slot name="...">` are lowered to CE slot calls and emitted in the wrapper slot-name array (test: custom_element_slots)
- [x] Non-custom-element legacy `<slot>` keeps runes-mode deprecation warning ownership while still lowering through the legacy runtime path (test: warn_slot_deprecated)
- [x] Element-child `slot="..."` diagnostics cover static-value, placement, duplicate-name, default-slot-conflict, and slotted-`{@const}` allowances (test: slots/slot_attribute_static_value_ok, slots/slot_attribute_invalid_expression_value, slots/slot_attribute_invalid_placement_root, slots/slot_attribute_invalid_placement_nested_inside_component, slots/slot_attribute_duplicate_reports_second_named_slot, slots/slot_default_duplicate_reports_implicit_default_content, slots/slot_default_duplicate_ignores_whitespace_and_other_named_slots, slots/const_tag_inside_slotted_element_is_allowed)
- [x] Duplicate/default slot-conflict diagnostics include child components with `slot="..."` instead of only element-like wrappers (tests: `slots/slot_attribute_duplicate_component_child_reports_second_named_slot`, `slots/slot_default_duplicate_component_child_reports_slotted_component_conflict`)
- [x] `<slot>` validation matches reference behavior for invalid `name`, reserved `name="default"`, invalid non-attribute directives, and slot/render conflicts
  - [x] `<slot name={expr}>` and other non-text `name` forms report `slot_element_invalid_name` (test: `slots/slot_element_invalid_name_dynamic`)
  - [x] `<slot name="default">` reports `slot_element_invalid_name_default` (test: `slots/slot_element_invalid_name_default`)
  - [x] `<slot>` rejects non-attribute directives other than spread and `let:` (test: `slots/slot_element_invalid_attribute_class`)
  - [x] `<slot>` and `{@render ...}` conflicts report `slot_snippet_conflict` (test: `slots/slot_snippet_conflict`)
- [x] `<svelte:fragment>` validation matches reference behavior for invalid placement and invalid attributes other than `slot` plus optional `let:`
  - [x] Root or otherwise non-component `<svelte:fragment>` reports `svelte_fragment_invalid_placement` (test: `slots/svelte_fragment_invalid_placement_root`)
  - [x] `<svelte:fragment>` rejects attributes other than `slot` plus optional `let:` (test: `slots/svelte_fragment_invalid_attribute_class`)
- [x] `let:` invalid-placement diagnostics match the reference owner matrix for default slots, named slots, and slotted child components
  - [x] `let:` on `<svelte:window>` reports `let_directive_invalid_placement` instead of a generic illegal-attribute diagnostic (test: `slots/let_directive_invalid_placement_svelte_window`)
  - [x] `let:` on `<svelte:body>` reports `let_directive_invalid_placement` instead of a generic illegal-attribute diagnostic (test: `slots/let_directive_invalid_placement_svelte_body`)

## Out of scope

- Snippet interop beyond the legacy slot conflict diagnostics already referenced above
- SSR slot generation
- Shared legacy special-variable deconfliction tracked by `specs/legacy-export-let.md`

## Reference
### Svelte
- `reference/docs/99-legacy/20-legacy-slots.md`
- `reference/docs/99-legacy/21-legacy-$$slots.md`
- `reference/docs/99-legacy/22-legacy-svelte-fragment.md`
- `reference/docs/07-misc/06-v4-migration-guide.md`
- `reference/docs/07-misc/07-v5-migration-guide.md`
- `reference/docs/07-misc/04-custom-elements.md`
- `reference/compiler/phases/1-parse/state/element.js`
- `reference/compiler/utils/slot.js`
- `reference/compiler/phases/2-analyze/visitors/shared/attribute.js`
- `reference/compiler/phases/2-analyze/visitors/shared/component.js`
- `reference/compiler/phases/2-analyze/visitors/SlotElement.js`
- `reference/compiler/phases/2-analyze/visitors/Identifier.js`
- `reference/compiler/phases/2-analyze/visitors/SvelteFragment.js`
- `reference/compiler/phases/3-transform/client/visitors/Program.js`
- `reference/compiler/phases/3-transform/client/visitors/Identifier.js`
- `reference/compiler/phases/3-transform/client/visitors/SlotElement.js`
- `reference/compiler/phases/3-transform/client/visitors/LetDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- `reference/compiler/phases/3-transform/server/visitors/shared/component.js`

### Our code
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/attr_convert.rs`
- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/passes/template_validation.rs`
- `crates/svelte_analyze/src/tests.rs`
- `crates/svelte_codegen_client/src/template/slot.rs`
- `crates/svelte_codegen_client/src/template/component.rs`
- `crates/svelte_codegen_client/src/lib.rs`
- `crates/svelte_codegen_client/src/custom_element.rs`
- `tasks/compiler_tests/test_v3.rs`
- `tasks/diagnostic_tests/test_diagnostics.rs`

## Test cases

- [x] warn_slot_deprecated
- [x] component_children
- [x] component_element_children
- [x] slot_named_fallback
- [x] component_named_slot
- [x] svelte_fragment_named_slot
- [x] slots/slot_attribute_static_value_ok
- [x] slots/slot_attribute_invalid_expression_value
- [x] slots/slot_attribute_invalid_placement_root
- [x] slots/slot_attribute_invalid_placement_nested_inside_component
- [x] slots/slot_attribute_duplicate_reports_second_named_slot
- [x] slots/slot_default_duplicate_reports_implicit_default_content
- [x] slots/slot_distinct_named_slots_do_not_conflict
- [x] slots/slot_default_duplicate_ignores_whitespace_and_other_named_slots
- [x] slots/const_tag_inside_slotted_element_is_allowed
- [x] legacy_slot_element_converts_to_dedicated_node
- [x] legacy_svelte_fragment_converts_to_dedicated_node
- [x] let_directive_legacy_without_expression
- [x] let_directive_legacy_with_expression
- [x] let_directive_legacy_converts_to_dedicated_attribute
- [x] slot_element_legacy_root_fragment_uses_dedicated_lowered_item
- [x] component_named_slot_mapping_uses_svelte_fragment_legacy_wrapper_id
- [x] legacy_slot_dev_mixed
- [x] component_default_slot_bindings_do_not_leak_into_named_slot_scope
- [x] slot_props_default
- [x] slot_props_spread
- [x] slot_props_dynamic_state
- [x] slot_props_dynamic_call
- [x] component_default_slot_let
- [x] component_default_slot_let_alias
- [x] component_named_slot_let_element
- [x] component_named_slot_let_element_destructure
- [x] component_named_slot_let_element_multiple
- [x] component_named_slot_let_fragment
- [x] component_named_slot_let_fragment_destructure
- [x] component_child_slot_attribute
- [x] svelte_self_slot
- [x] legacy_slots_if
- [x] legacy_slots_script_basic
- [x] custom_element_slots
- [x] legacy_slots_template_reads_require_sanitized_slots_binding
- [x] legacy_slot_elements_do_not_require_sanitized_slots_binding
- [x] legacy_slots_script_reads_require_sanitized_slots_binding
- [x] slots/slot_attribute_duplicate_component_child_reports_second_named_slot
- [x] slots/slot_default_duplicate_component_child_reports_slotted_component_conflict
- [x] slots/slot_element_invalid_name_dynamic
- [x] slots/slot_element_invalid_name_default
- [x] slots/slot_element_invalid_attribute_class
- [x] slots/slot_snippet_conflict
- [x] slots/svelte_fragment_invalid_placement_root
- [x] slots/svelte_fragment_invalid_attribute_class
- [x] slots/let_directive_invalid_placement_svelte_window
- [x] slots/let_directive_invalid_placement_svelte_body
