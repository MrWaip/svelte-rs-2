# Attributes & Spreads

## Current state
- **Working**: core attribute/spread compilation paths are covered, including unquoted-sequence validation parity across components, regular elements, custom elements, and `<svelte:element>`.
- **Current slice completed**: `attribute_unquoted_sequence` parity outside components. The scanner already emitted unquoted concatenations as `ConcatenationAttribute`; analyze now rejects them consistently across all relevant attribute owners.
- **Spec ownership status**: no further implementation-owned slice remains here; related remaining work is tracked in neighboring specs (`events`, `bind-directives`, `element`, `a11y-warnings`).
- **Non-goals for this completed run**: parser/scanner changes, bind-coupled diagnostics owned by `specs/bind-directives.md`, customizable `select` / `optgroup` / `selectedcontent` handling owned by `specs/element.md`, and a11y warnings.
- **Constraint**: Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.
- Last updated: 2026-04-09

## Source

- ROADMAP bucket: `Attributes & Spreads`
- Related specs with overlapping ownership:
  - `specs/element.md`
  - `specs/component-node.md`
  - `specs/events.md`
  - `specs/bind-directives.md`
  - `specs/css-pipeline.md`
- Request: `/audit Attributes & Spreads`

## Syntax variants

- Regular element attributes: `<div foo="x" bar={expr} baz />`
- Concatenated and shorthand attrs: `<div title="x {y}" {y} />`
- Spread attrs: `<div a="x" {...props} b={y} {...rest} />`
- `class` forms: `class="foo"`, `class={expr}`, `class={[...]}`, `class={{...}}`, `class:name`
- `style` forms: `style="x: y"`, `style={expr}`, `style={{...}}`, `style:name`, `style:name|important`
- Form-element-sensitive attrs: `<textarea>{expr}</textarea>`, `<option>{expr}</option>`, `<input autofocus={expr}>`
- Dynamic tag parity: `<svelte:element this={tag} ... />`

## Use cases

- `[x]` Static, boolean, expression, concatenation, and shorthand attributes on regular elements compile (test: `element_attributes`)
- `[x]` Regular-element spread attributes preserve source order with surrounding attrs (test: `spread_attribute`)
- `[x]` `class:name` directives on regular elements compile (test: `class_directive`)
- `[x]` `style:name` directives, concat values, and `|important` compile (tests: `style_directive`, `style_directive_concat`, `style_directive_important`, `style_directive_string`)
- `[x]` `class={object}` and `class={[...]}` lower through `$.clsx(...)` (tests: `class_object`, `class_array`, `class_expr_with_directives`)
- `[x]` Dynamic `style` attributes compile for string/object inputs (tests: `style_attr_dynamic`, `style_attr_object`)
- `[x]` `<svelte:element>` supports plain attrs, spreads, `class:` and `style:` (tests: `svelte_element_attributes`, `svelte_element_spread`, `svelte_element_class_directive`, `svelte_element_style_directive`)
- `[x]` Form-element special cases for dynamic textarea children and `<option>{expr}</option>` are covered by focused compiler cases (tests: `textarea_child_value_dynamic`, `option_expr_child_value`)
- `[x]` Spread attributes compose with `class={...}` / `class:*` through a single `$.attribute_effect(...)` shape (test: `spread_class_directive`)
- `[x]` Spread attributes compose with `style={...}` / `style:*` through a single `$.attribute_effect(...)` shape (test: `spread_style_directive`)
- `[x]` Regular-element `autofocus` lowers through `$.autofocus(...)` (test: `element_autofocus`)
- `[x]` `attribute_invalid_name` — error for names starting with digit/dash/dot or containing illegal chars
- `[x]` `attribute_invalid_event_handler` — error for `on*` attrs with string/concatenation values
- `[x]` `attribute_duplicate` — parser layer (`attr_convert.rs`); HTMLAttribute + BindDirective share key space; `this` excluded
- `[x]` `attribute_unquoted_sequence` — analyzer rejects unquoted concatenation values like `foo=a{value}` consistently across components, regular elements, custom elements, and `<svelte:element>` (tests: `component_attribute_unquoted_sequence_errors`, `regular_element_attribute_unquoted_sequence_errors`, `custom_element_attribute_unquoted_sequence_errors`, `svelte_element_attribute_unquoted_sequence_errors`)
- `[x]` `attribute_quoted` — warning for quoted single-expr on component or custom element (runes mode); `visit_component_node` added
- `[x]` `slot_attribute_invalid` — placement done; value check (non-StringAttribute when parent is ComponentNode) done
- `[x]` Form-element validation ownership is split across neighboring specs — `textarea_invalid_content` is done here; customizable `select` / `optgroup` / `selectedcontent` paths are tracked in `specs/element.md`; remaining bind-sensitive attribute validations are tracked in `specs/bind-directives.md`
- `[x]` Event attribute validation specifics are owned and completed in `specs/events.md`
- `[x]` Binding-driven attribute diagnostics (`attribute_invalid_type`, `attribute_invalid_multiple`, contenteditable) are tracked and completed in `specs/bind-directives.md`
- `[x]` A11y attribute warnings are owned by `specs/a11y-warnings.md`

## Reference

- Reference compiler:
  - `reference/compiler/phases/2-analyze/visitors/shared/attribute.js`
  - `reference/compiler/phases/2-analyze/visitors/RegularElement.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/a11y/index.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/element.js`
  - `reference/compiler/phases/3-transform/client/visitors/RegularElement.js`
  - `reference/compiler/phases/3-transform/client/visitors/SvelteElement.js`
  - `reference/compiler/errors.js`
  - `reference/compiler/warnings.js`
- Our implementation:
  - `crates/svelte_parser/src/attr_convert.rs`
  - `crates/svelte_parser/src/scanner/mod.rs`
  - `crates/svelte_analyze/src/passes/element_flags.rs`
  - `crates/svelte_analyze/src/passes/bind_semantics.rs`
  - `crates/svelte_analyze/src/validate/mod.rs`
  - `crates/svelte_codegen_client/src/template/attributes.rs`
  - `crates/svelte_codegen_client/src/template/element.rs`
  - `crates/svelte_codegen_client/src/template/svelte_element.rs`
  - `tasks/compiler_tests/cases2/*attribute*`
  - `tasks/compiler_tests/cases2/class_*`
  - `tasks/compiler_tests/cases2/style_*`

## Test cases

- `[x]` `element_attributes`
- `[x]` `spread_attribute`
- `[x]` `class_directive`
- `[x]` `class_object`
- `[x]` `class_array`
- `[x]` `class_expr_with_directives`
- `[x]` `style_directive`
- `[x]` `style_directive_concat`
- `[x]` `style_directive_important`
- `[x]` `style_directive_string`
- `[x]` `style_attr_dynamic`
- `[x]` `style_attr_object`
- `[x]` `svelte_element_attributes`
- `[x]` `svelte_element_spread`
- `[x]` `svelte_element_class_directive`
- `[x]` `svelte_element_style_directive`
- `[x]` `textarea_child_value_dynamic`
- `[x]` `option_expr_child_value`
- `[x]` `element_autofocus`
- `[x]` `spread_class_directive`
- `[x]` `spread_style_directive`
- `[x]` `component_attribute_unquoted_sequence_errors`
- `[x]` `regular_element_attribute_unquoted_sequence_errors`
- `[x]` `custom_element_attribute_unquoted_sequence_errors`
- `[x]` `svelte_element_attribute_unquoted_sequence_errors`
