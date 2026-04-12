# Legacy slots

## Current state
- **Working**: 7/19 closure items
- **Completed (2026-04-12)**: absorbed direct `$$slots` runtime-surface ownership from the removed `legacy-special-vars.md` audit and kept the focused compiler parity case under this spec
- **Confirmed gap (2026-04-12)**: Rust preserves direct `$$slots.description` reads but never injects the reference compiler's `const $$slots = $.sanitize_slots($$props)` helper, and it misses the untracked-read wrapper around conditional slot presence checks
- **Next**: implement legacy slot-prop and `let:` codegen, then widen `slot="..."` ownership from element-only to component children, then close custom-element slot metadata/runtime parity
- **Blocked verification**: `just test-case <name>` currently fails before executing assertions because `crates/svelte_codegen_client/src/script/traverse/assignments.rs:250` calls `.clone()` on a non-`Clone` `Option<(String, String, Vec<Expression<'_>>)>`
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

- [x] Default component children lower to `children` plus `$$slots.default` for legacy child-content interop (tests: `component_children`, `component_element_children`)
- [x] Default `<slot>` lowers to `$.slot(..., "default", {}, fallback)` and keeps optional fallback content intact (test: warn_slot_deprecated)
- [x] Non-custom-element legacy `<slot>` keeps runes-mode deprecation warning ownership while still lowering through the legacy runtime path (test: warn_slot_deprecated)
- [x] Named `<slot name="...">` lowers correctly with fallback content (test: slot_named_fallback)
- [x] Direct child elements with `slot="..."` lower into parent `$$slots` entries (test: component_named_slot)
- [x] Direct child `<svelte:fragment slot="...">` lowers into parent `$$slots` entries without wrapper DOM (test: svelte_fragment_named_slot)
- [x] Element-child `slot="..."` diagnostics cover static-value, placement, duplicate-name, default-slot-conflict, and slotted-`{@const}` allowances (test: slots/slot_attribute_static_value_ok, slots/slot_attribute_invalid_expression_value, slots/slot_attribute_invalid_placement_root, slots/slot_attribute_invalid_placement_nested_inside_component, slots/slot_attribute_duplicate_reports_second_named_slot, slots/slot_default_duplicate_reports_implicit_default_content, slots/slot_default_duplicate_ignores_whitespace_and_other_named_slots, slots/const_tag_inside_slotted_element_is_allowed)
- [ ] `<slot>` emits slot props from attributes/spreads instead of always passing `{}` (test: slot_props_default, #[ignore], moderate)
- [ ] Parent default-slot `let:` directives lower to derived reads from `$$slotProps` inside the generated slot function (test: component_default_slot_let, #[ignore], moderate)
- [ ] Named-slot `let:` directives on direct child elements lower inside the generated named-slot function (test: component_named_slot_let_element, #[ignore], moderate)
- [ ] Named-slot `let:` directives on `<svelte:fragment>` lower inside the generated named-slot function (test: component_named_slot_let_fragment, #[ignore], moderate)
- [ ] Child components with `slot="..."` participate in named-slot grouping instead of receiving a plain `slot` prop (test: component_child_slot_attribute, #[ignore], moderate)
- [ ] Duplicate/default slot-conflict diagnostics include child components with `slot="..."` instead of only element-like wrappers (test: none yet, moderate)
- [ ] `<slot>` validation matches reference behavior for invalid `name`, reserved `name="default"`, invalid non-attribute directives, and slot/render conflicts (test: none yet, moderate)
- [ ] `<svelte:fragment>` validation matches reference behavior for invalid placement and invalid attributes other than `slot` plus optional `let:` (test: none yet, moderate)
- [ ] `let:` invalid-placement diagnostics match the reference owner matrix for default slots, named slots, and slotted child components (test: none yet, needs infrastructure)
- [ ] Default-slot bindings remain scoped to the default slot and are not visible inside named-slot content, matching the Svelte 4 migration note (test: none yet, needs infrastructure)
- [ ] Direct `$$slots` reads lower through `$.sanitize_slots($$props)` so conditional checks like `$$slots.description` work in component/template code; current Rust preserves the read but omits the sanitized binding and misses the reference compiler's untracked read wrapper (test: `legacy_slots_if`, `#[ignore]`, moderate)
- [ ] Custom-element `<slot>` and named `<slot name="...">` are lowered to CE slot calls and emitted in the wrapper slot-name array (test: custom_element_slots, #[ignore], needs infrastructure)

## Out of scope

- Snippet interop beyond the legacy slot conflict diagnostics already referenced above
- SSR slot generation

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
- [ ] slot_props_default
- [ ] component_default_slot_let
- [ ] component_named_slot_let_element
- [ ] component_named_slot_let_fragment
- [ ] component_child_slot_attribute
- [ ] legacy_slots_if
- [ ] custom_element_slots
