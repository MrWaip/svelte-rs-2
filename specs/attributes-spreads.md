# Attributes & Spreads

## Current state
- **Working**: 26/27 use cases
- **Tests**: 32/33 green
- Last updated: 2026-05-13

## Source

- ROADMAP bucket: `Attributes & Spreads`
- Related specs with overlapping ownership:
  - `specs/element.md`
  - `specs/component-node.md`
  - `specs/legacy-slots.md`
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

- [x] Static, boolean, expression, concatenation, and shorthand attributes on regular elements compile (test: `element_attributes`)
- [x] Regular-element spread attributes preserve source order with surrounding attrs (test: `spread_attribute`)
- [x] `class:name` directives on regular elements compile (test: `class_directive`)
- [x] `style:name` directives, concat values, and `|important` compile (tests: `style_directive`, `style_directive_concat`, `style_directive_important`, `style_directive_string`)
- [x] `class={object}` and `class={[...]}` lower through `$.clsx(...)` (tests: `class_object`, `class_array`, `class_expr_with_directives`)
- [x] Dynamic `style` attributes compile for string/object inputs (tests: `style_attr_dynamic`, `style_attr_object`)
- [x] `<svelte:element>` supports plain attrs, spreads, `class:` and `style:` (tests: `svelte_element_attributes`, `svelte_element_spread`, `svelte_element_class_directive`, `svelte_element_style_directive`)
- [x] Form-element special cases for dynamic textarea children and `<option>{expr}</option>` are covered by focused compiler cases (tests: `textarea_child_value_dynamic`, `option_expr_child_value`)
- [x] Regular non-spread dynamic attrs follow the reference property/special-value update matrix instead of always falling through `$.set_attribute(...)` — covers `value`, `checked`, `selected`, and DOM-property attrs like `disabled` / `readonly` across expression, concatenation, and shorthand forms; `<input>` variants also set `needs_input_defaults` when required, and non-HTML namespace cases stay aligned with current reference lowering (tests: `input_dynamic_special_attrs`, `svg_dynamic_special_attrs`, `diagnose_props_bindable_icon_component`)
- [x] Spread attributes compose with `class={...}` / `class:*` through a single `$.attribute_effect(...)` shape (test: `spread_class_directive`)
- [x] Spread attributes compose with `style={...}` / `style:*` through a single `$.attribute_effect(...)` shape (test: `spread_style_directive`)
- [x] Regular-element `autofocus` lowers through `$.autofocus(...)` (test: `element_autofocus`)
- [x] `attribute_invalid_name` — error for names starting with digit/dash/dot or containing illegal chars
- [x] `attribute_invalid_event_handler` — error for `on*` attrs with string/concatenation values
- [x] `attribute_duplicate` — parser layer (`attr_convert.rs`); HTMLAttribute + BindDirective share key space; `this` excluded
- [x] `attribute_unquoted_sequence` — analyzer rejects unquoted concatenation values like `foo=a{value}` consistently across components, regular elements, custom elements, and `<svelte:element>` (tests: `component_attribute_unquoted_sequence_errors`, `regular_element_attribute_unquoted_sequence_errors`, `custom_element_attribute_unquoted_sequence_errors`, `svelte_element_attribute_unquoted_sequence_errors`)
- [x] `attribute_quoted` — warning for quoted single-expr on component or custom element (runes mode); `visit_component_node` added
- [x] Form-element validation ownership is split across neighboring specs — `textarea_invalid_content` is done here; customizable `select` / `optgroup` / `selectedcontent` paths are tracked in `specs/element.md`; remaining bind-sensitive attribute validations are tracked in `specs/bind-directives.md`
- [x] Event attribute validation specifics are owned and completed in `specs/events.md`
- [x] Binding-driven attribute diagnostics (`attribute_invalid_type`, `attribute_invalid_multiple`, contenteditable) are tracked and completed in `specs/bind-directives.md`
- [x] A11y attribute warnings are owned by `specs/a11y-warnings.md`
- [x] Legacy slot-attribute validation ownership moved to `specs/legacy-slots.md`
- [x] Regular-element attribute whose value is a member-access chain rooted in a `MetaProperty` (e.g. `<a href={import.meta.env.VITE_X}>`) lowers through `$.template_effect(() => $.set_attribute(el, name, expr))` instead of an eager `$.set_attribute(...)` outside the effect. Reference treats any `MetaProperty`-rooted read as runtime-evaluated. Owning layer: 3.A.3 `ExpressionSemantics`. The classifier already has the right output shape — `ExprKind::SimpleRead { reactive: true }` for `TopLevelForm::Member`. The single missing input is a builder-private fact in `ExprFacts` (peer of `has_call` / `has_await` / `has_store_ref`) marking that the peeled root is `MetaProperty`; `derive::is_dynamic_template` ORs it into the Member/Call dynamism check, `update_aggregates` notes `ContextSignal::IMPORT_OR_PROP_MEMBER` so `$.push/$.pop` emit. Consumers (`references_need_wrap` in `attribute_semantics`, `is_dynamic_element_attr` in `dynamism.rs`) read the canonical `data.kind` reactive flag — no sidecar boolean on `ExpressionData`. Test: `element_attr_import_meta_template_effect`.
- [ ] Concatenated `class="..."` template parts skip the `?? ""` fallback for expressions that always coerce to a string in template-literal context — `LogicalExpression` (`&&`, `||`, `??`), `ConditionalExpression`, `BinaryExpression`, and string/number literals — matching reference output `${cond && "b"}` instead of `${(cond && "b") ?? ""}` (test: `class_concat_logical_and_string`)
- [x] Shorthand attributes inside an `$.attribute_effect(...)` spread object emit `name: name()` for `$.prop`-wrapped locals and `name: $$props.name` for props with no local binding; bare shorthand stays for plain locals (test: `attribute_effect_shorthand_prop_unwrap`)
- [x] Attribute and directive names containing `_` parse correctly across regular attrs, `class:`, `style:`, `on:`, `use:`, `bind:` — scanner allows `_` in the attribute-identifier predicate (tests: `attribute_name_with_underscore`, `diagnose_class_directive_name_with_underscore`)
- [x] `class:` directives whose value is a non-trivial expression (e.g. `CallExpression` like `Boolean(x)`) hoist the entire class-directives object into the `template_effect` deps array — `template_effect(($0) => { classes = set_class(div, 1, "", null, classes, $0); ... }, [() => ({ ... })])` — instead of inlining the object literal directly into the `set_class(...)` call (test: `diagnose_class_directive_call_in_template_effect`)

## Reference

### Svelte
  - `reference/compiler/phases/2-analyze/visitors/shared/attribute.js`
  - `reference/compiler/phases/2-analyze/visitors/RegularElement.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/a11y/index.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/element.js`
  - `reference/compiler/phases/3-transform/client/visitors/RegularElement.js`
  - `reference/compiler/phases/3-transform/client/visitors/SvelteElement.js`
  - `reference/compiler/errors.js`
  - `reference/compiler/warnings.js`

### Our code
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

- [x] `element_attributes`
- [x] `spread_attribute`
- [x] `class_directive`
- [x] `class_object`
- [x] `class_array`
- [x] `class_expr_with_directives`
- [x] `style_directive`
- [x] `style_directive_concat`
- [x] `style_directive_important`
- [x] `style_directive_string`
- [x] `style_attr_dynamic`
- [x] `style_attr_object`
- [x] `svelte_element_attributes`
- [x] `svelte_element_spread`
- [x] `svelte_element_class_directive`
- [x] `svelte_element_style_directive`
- [x] `textarea_child_value_dynamic`
- [x] `option_expr_child_value`
- [x] `element_autofocus`
- [x] `input_dynamic_special_attrs`
- [x] `svg_dynamic_special_attrs`
- [x] `spread_class_directive`
- [x] `spread_style_directive`
- [x] `diagnose_props_bindable_icon_component`
- [x] `component_attribute_unquoted_sequence_errors`
- [x] `regular_element_attribute_unquoted_sequence_errors`
- [x] `custom_element_attribute_unquoted_sequence_errors`
- [x] `svelte_element_attribute_unquoted_sequence_errors`
- [ ] `class_concat_logical_and_string`
- [x] `element_attr_import_meta_template_effect`
- [x] `attribute_effect_shorthand_prop_unwrap`
- [x] `diagnose_class_directive_call_in_template_effect`
