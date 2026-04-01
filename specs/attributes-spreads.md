# Attributes & Spreads

## Current state
- **Working**: 8/13 use cases are covered by existing compiler cases
- **Missing**: analyze-side attribute validation/warnings, spread composition parity for `class`/`style`, and the regular-element `autofocus` helper path
- **Next**: fix the three failing compiler cases added during this audit (`element_autofocus`, `spread_class_directive`, `spread_style_directive`), then port analyze-owned validation instead of adding more codegen-side special cases
- Last updated: 2026-04-01

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

- `[x]` Static, boolean, expression, concatenation, and shorthand attributes on regular elements compile
  Existing tests: `element_attributes`
- `[x]` Regular-element spread attributes preserve source order with surrounding attrs
  Existing tests: `spread_attribute`
- `[x]` `class:name` directives on regular elements compile
  Existing tests: `class_directive`
- `[x]` `style:name` directives, concat values, and `|important` compile
  Existing tests: `style_directive`, `style_directive_concat`, `style_directive_important`, `style_directive_string`
- `[x]` `class={object}` and `class={[...]}` lower through `$.clsx(...)`
  Existing tests: `class_object`, `class_array`, `class_expr_with_directives`
- `[x]` Dynamic `style` attributes compile for string/object inputs
  Existing tests: `style_attr_dynamic`, `style_attr_object`
- `[x]` `<svelte:element>` supports plain attrs, spreads, `class:` and `style:`
  Existing tests: `svelte_element_attributes`, `svelte_element_spread`, `svelte_element_class_directive`, `svelte_element_style_directive`
- `[x]` Form-element special cases for dynamic textarea children and `<option>{expr}</option>` are covered by focused compiler cases
  Existing tests: `textarea_child_value_dynamic`, `option_expr_child_value`
- `[~]` Spread attributes coexist with plain attrs, but spread composition with `class={...}` / `class:*` is not covered and likely diverges from reference `$.attribute_effect(...)` shape
  Added during this audit: `spread_class_directive`
- `[~]` Spread attributes coexist with plain attrs, but spread composition with `style={...}` / `style:*` is not covered and likely diverges from reference `$.attribute_effect(...)` shape
  Added during this audit: `spread_style_directive`
- `[ ]` Regular-element `autofocus` should lower through `$.autofocus(...)` instead of a generic attribute setter
  Added during this audit: `element_autofocus`
- `[ ]` Analyze-side attribute validation/warnings are mostly absent
  Missing today: `attribute_duplicate`, `attribute_invalid_name`, `attribute_unquoted_sequence`, `attribute_illegal_colon`, `attribute_quoted`, `slot_attribute_invalid`, `slot_attribute_invalid_placement`
- `[ ]` Form-element validation and remaining special handling are incomplete
  Missing today: `textarea_invalid_content`, customizable `select` / `optgroup` / `selectedcontent` paths, and the remaining bind-sensitive attribute validations tracked in `specs/bind-directives.md`

### Deferred

- `[ ]` Event attribute validation specifics
  Tracked in `specs/events.md`
- `[ ]` Binding-driven attribute diagnostics such as `attribute_invalid_type`, `attribute_invalid_multiple`, and contenteditable checks
  Tracked in `specs/bind-directives.md`
- `[ ]` A11y-only attribute warnings beyond structural parity
  Tracked under diagnostics work rather than this spec

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

## Tasks

1. `[ ]` Port analyze-owned generic attribute validation and warnings
   Files: `crates/svelte_analyze/src/validate/mod.rs` plus new template validation modules
   Scope: duplicate/invalid/unquoted/quoted/slot-placement validation and warnings
   Effort: needs infrastructure
2. `[ ]` Align spread + `class` / `style` composition with reference `$.attribute_effect(...)`
   Files: `crates/svelte_codegen_client/src/template/attributes.rs`, `crates/svelte_codegen_client/src/template/element.rs`, `crates/svelte_codegen_client/src/template/svelte_element.rs`
   Effort: moderate
3. `[ ]` Port the regular-element `autofocus` helper path
   Files: `crates/svelte_codegen_client/src/template/attributes.rs`
   Effort: quick fix
4. `[ ]` Finish remaining form-element attribute ownership in analyze/codegen order
   Files: shared with `specs/element.md` and `specs/bind-directives.md`
   Effort: needs infrastructure

## Implementation order

1. Fix the three bounded compiler cases from this audit to close the obvious codegen gaps.
2. Add analyze-side template validation for generic attribute diagnostics and slot placement.
3. Revisit remaining form-element-specific validations and customizable select behavior after the generic validation path exists.

## Discovered bugs

- OPEN: `crates/svelte_analyze/src/validate/mod.rs` validates runes only; generic template attribute validation is absent.
- OPEN: `crates/svelte_codegen_client/src/template/attributes.rs` has no dedicated `autofocus` lowering path and treats it like a generic attribute.
- OPEN: `element_autofocus` currently emits `$.set_attribute(input, "autofocus", enabled)` instead of reference `$.autofocus(input, enabled)`.
- OPEN: `spread_class_directive` currently emits separate `$.attribute_effect(...props)` plus `$.set_class(...)` calls instead of a single `$.attribute_effect(..., [$.CLASS])` shape.
- OPEN: `spread_style_directive` panics in codegen with `missing pre-transformed expr at handle ExprHandle(1)`, so spread + `style:` is currently broken rather than merely mismatched.

## Test cases

- Existing covered compiler cases:
  - `element_attributes`
  - `spread_attribute`
  - `class_directive`
  - `class_object`
  - `class_array`
  - `class_expr_with_directives`
  - `style_directive`
  - `style_directive_concat`
  - `style_directive_important`
  - `style_directive_string`
  - `style_attr_dynamic`
  - `style_attr_object`
  - `svelte_element_attributes`
  - `svelte_element_spread`
  - `svelte_element_class_directive`
  - `svelte_element_style_directive`
  - `textarea_child_value_dynamic`
  - `option_expr_child_value`
- Added during this audit:
  - `element_autofocus` (fails: generic `set_attribute` vs `$.autofocus`)
  - `spread_class_directive` (fails: split emission vs single `$.attribute_effect`)
  - `spread_style_directive` (fails: codegen panic on spread + `style:`)
- Recommended next command:
  - `port specs/attributes-spreads.md`
