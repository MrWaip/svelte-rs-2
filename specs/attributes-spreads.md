# Attributes & Spreads

## Current state
- **Working**: 42/45 use cases
- **Tests**: 52/52 green
- Last updated: 2026-05-23

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
- [x] Concatenated `class="..."` template parts skip the `?? ""` fallback for expressions that always coerce to a string in template-literal context — `LogicalExpression` (`&&`, `||`, `??`), `ConditionalExpression`, `BinaryExpression`, and string/number literals — matching reference output `${cond && "b"}` instead of `${(cond && "b") ?? ""}` (test: `class_concat_logical_and_string`)
- [x] Shorthand attributes inside an `$.attribute_effect(...)` spread object emit `name: name()` for `$.prop`-wrapped locals and `name: $$props.name` for props with no local binding; bare shorthand stays for plain locals (test: `attribute_effect_shorthand_prop_unwrap`)
- [x] Attribute and directive names containing `_` parse correctly across regular attrs, `class:`, `style:`, `on:`, `use:`, `bind:` — scanner allows `_` in the attribute-identifier predicate (tests: `attribute_name_with_underscore`, `diagnose_class_directive_name_with_underscore`)
- [x] `class:` directives whose value is a non-trivial expression (e.g. `CallExpression` like `Boolean(x)`) hoist the entire class-directives object into the `template_effect` deps array — `template_effect(($0) => { classes = set_class(div, 1, "", null, classes, $0); ... }, [() => ({ ... })])` — instead of inlining the object literal directly into the `set_class(...)` call (test: `diagnose_class_directive_call_in_template_effect`)
- [x] Когда у одного элемента одновременно есть динамический memo-атрибут (например, `href={f(x)}`) и динамический текст-узел (`{g(y)}`), оба update-вызова попадают в общий `$.template_effect(...)`. Атрибут-сеттер должен идти первым в теле эффекта, текстовый сеттер — вторым, и массив deps должен следовать той же последовательности (атрибут перед текстом). Сейчас `emit_template_effect_with_memo` в `crates/svelte_codegen_client/src/codegen/effect.rs` дописывает memo-attr setters в `callback_body` *после* `regular_updates` (где уже лежит `set_text`), и deps выстраиваются shared-first → memo-after, в результате порядок инвертирован относительно reference. Owning layer: 4 codegen (test: `template_effect_attr_before_text_order`)
- [x] Element with only literal-string `style:name="value"` directives emits one eager `$.set_style(el, <static-style>, {}, { name: "value", ... })` call in init instead of grouping into a `$.template_effect` with a `let styles` cache var. Sibling reactive setters on the same element stay inside their own `template_effect`. Test: `diagnose_style_directive_literal_skips_template_effect_grouping`.
- [x] `style:name="..."` directive whose quoted value contains only literal-expression interpolations (template like `"{18}px"`, `"{0}"`, `"{"red"}"` — no identifier/state references) folds at compile time into a constant string and emits one eager `$.set_style(el, "", {}, { name: "18px", ... })` in init, matching the bare-literal directive shape. Currently we treat the template as reactive and wrap in `$.template_effect(() => styles = $.set_style(..., { name: \`${18}px\` }))`. Owning layer: 4 codegen (attribute lowering — should classify directive value as static when every interpolation expression is a literal). Test: `diagnose_style_directive_quoted_literal_interp_static`.
- [x] `style:name={expr}` whose expression is non-trivial (call, logical op, member chain, etc.) hoists the style-directives object into the `template_effect` deps array — `template_effect(($0) => styles = $.set_style(el, "", styles, $0), [() => ({ name: ($.deep_read_state(...), $.untrack(() => expr)) })])` — instead of inlining the object literal as the fourth arg of `set_style`. Mirrors the `class:` rule on line 58 but for style directives. Owning layer: 4 codegen (`emit_template_effect_with_memo` + style-directive lowering). Test: `diagnose_style_directive_complex_expression_hoists_to_memo_deps`.
- [x] `attribute_effect` spread argument whose expression is a `CallExpression` (e.g. `{...mapClasses('a', ...classes)}` alongside a `class:`/`style:` directive) hoists the call into the `attribute_effect` deps array — `attribute_effect(div, ($0) => ({ ...$0, [$.CLASS]: {...} }), [() => mapClasses('a', ...classes)])` — instead of inlining `...mapClasses(...)` as a property of the spread object literal. Mirrors the `class:` rule on line 58 / style rule on line 62 but for the spread-of-call argument of `attribute_effect`. Owning layer: 4 codegen (attribute lowering — `attribute_effect` emit in `crates/svelte_codegen_client/src/template/attributes.rs`). Test: `diagnose_attribute_effect_spread_call_memo`.
- [x] Regular-element expression-attribute whose value is a `CallExpression` always hoists the call into a `$.template_effect` deps array as a sync dep thunk — `template_effect(($0) => $.set_attribute(el, name, $0), [() => call()])` (legacy mode wraps identifier callees with `$.untrack`). On an element with additional dynamic setters, the call-attribute setter and dep join the shared `template_effect` body/deps array under the existing line-59 source-order rule. Tests: `diagnose_style_attr_call_groups_in_template_effect_memo`, `attr_call_value_hoists_to_template_effect_memo`.
- [x] `class:name={expr}` in legacy mode where `expr` is a non-reactive read of the `$$slots` map (e.g. `class:foo={$$slots.bar}`) emits one eager `$.set_class(el, 1, "", null, {}, { foo: $$slots.bar })` at template init — no `let classes;` cache var and no enclosing `$.template_effect(...)`. `$$slots` is a build-time-resolved `sanitize_slots($$props)` object whose membership cannot change after init, so reference treats it as static. Owning layer: 3.A.3 `ExpressionSemantics` — false-positive в `ExprFacts.has_store_ref` для `$$slots` (предикат `name.starts_with('$') && name.len() > 1` ловит legacy-sanitized идентификатор `$$slots`, у которого membership фиксируется на init и реактивности нет; downstream `derive::is_dynamic_template` в Member-ветке OR-ит `has_store_ref` и помечает class-directive динамичной). Test: `diagnose_class_directive_slots_member_legacy`.
- [x] Regular `<input>` with both a `bind:<prop>` directive and a `{...spread}` (identifier or call) emits the spread through `$.attribute_effect(input, () => ({ … }), void 0, void 0, void 0, void 0, true)` with the trailing `is_input = true` flag and omits the explicit `$.remove_input_defaults(input)` call entirely — defaults-reset is delegated to the runtime `set_attributes`/`attribute_effect` path. Reference branch: `RegularElement.js:186-190` (`has_spread → should_remove_defaults = true`, no explicit call emitted). Today `crates/svelte_codegen_client/src/codegen/containers/element.rs:123` pushes `remove_input_defaults` unconditionally for input+bind and the spread-emit site never passes the trailing `true`, so defaults reset twice or get skipped by the runtime spread path. Owning layer: 4 codegen (`element.rs` defaults-reset gate + `attribute_effect` emit-site argument list — extend the existing line-40 `<input>` + `needs_input_defaults` matrix to the spread branch). Test: `diagnose_legacy_input_bind_spread_omits_remove_defaults`.
- [x] Regular element с `{...spread}`, `use:action` и `bind:this={ref}` эмитит элемент-setup в порядке `$.attribute_effect(...)` → `$.action(...)` → `$.bind_this(...)`. Сегодня `crates/svelte_codegen_client/src/codegen/attributes/spread_attr.rs` в первом проходе `emit_attr_spread_full` сразу вызывает `emit_bind_directive` для `bind:this` (стейтмент уходит в `pending_element_init` до `$.attribute_effect` и до второго прохода, который эмитит `$.action`). Reference удерживает `bind_this` последним среди setup-стейтментов элемента независимо от исходного порядка атрибутов. Owning layer: 4 codegen (`spread_attr.rs` — `bind:this` нужно переместить во второй проход после use/transition/animate, как и в non-spread-ветке `dispatch.rs`). Test: `diagnose_spread_bind_this_action_order`.
- [x] `$.remove_textarea_child(textarea)` эмитится в element-setup до обработки атрибутов при любом из трёх reference-триггеров: `<textarea {...rest} />` (только spread), `<textarea bind:value ... />` (с/без spread), `<textarea value={expr} />` (динамический non-text `value`-атрибут). Реализовано в `crates/svelte_codegen_client/src/codegen/containers/element.rs::emit_element_html` через гейт `needs_textarea_content_reset(attributes, has_spread)` рядом с `remove_input_defaults`; дубль из `bind/mod.rs` снят. Tests: `diagnose_textarea_bind_value_with_spread_init_order`, `diagnose_textarea_spread_only_emits_remove_child`, `diagnose_textarea_dynamic_value_attribute_emits_remove_child`.
- [x] Element with both a dynamic `style="..."` attribute (interpolation or expression) AND one or more `style:` directives merges into a single `$.set_style(el, <style-attr-value>, styles, { <directives> })` call inside `$.template_effect`, instead of emitting two separate `$.set_style` calls (one for the attribute, one for the directives bucket). Reference passes the style-attribute value as the second positional argument to the same `set_style` that carries the directives object, so there is exactly one call per element. Owning layer: 4 codegen (`crates/svelte_codegen_client/src/codegen/attributes/style_directive.rs` + `regular.rs` — when an element has both, the regular-attribute setter for `style` must be suppressed and its value forwarded into `emit_set_style_call` as the `style_attr` arg). Test: `diagnose_style_attr_dynamic_with_style_directive_merges_set_style`.
- [x] Static `muted` attribute on any element (in practice `<video>` / `<audio>`) must be lowered to a runtime property assignment `el.muted = true;` and removed from the static `from_html` template. Reference handles this in `build_element_attribute_update` (`RegularElement.js:584-587`) as a Firefox quirk — the boolean HTML attribute is unreliable for autoplay, so reference routes ALL `muted` attributes through property-set, even literal-true. Today our codegen keeps `muted=""` in the static template and emits no runtime assignment. Owning layer: 3.A.4 `AttributeSemantics` — extend the property/special-value matrix (sibling of `value`/`checked`/`selected`/`disabled`/`readonly` at line 40) to classify `muted` as `must-be-property` regardless of whether the value is a static boolean literal, identifier expression, or interpolation; codegen consumes the classification by skipping template emission and writing the property assignment. Test: `diagnose_video_muted_static_attribute_lowers_to_property`.
- [ ] Static-template-bypass for `muted` attribute applies to `ConcatenationAttribute` form (`muted="{x}{y}"`) — deferred: same `MustBeProperty` routing needed, but value path goes through `emit_attr_concatenation` (codegen handler in `dispatch.rs:147`), separate consumer site from `BooleanAttribute` / `StringAttribute` static branches closed by `diagnose_video_muted_static_attribute_lowers_to_property`.
- [x] Regular element с `class:` директивой (или статическим `class="..."` + директивой) и legacy `on:event={call(...)}` (выражение оборачивается в `var <name> = $.derived(...)`) эмитит element-setup в порядке `var event_handler = $.derived(...)` → `let classes;`, а не наоборот. Reference (`reference/compiler/phases/3-transform/client/visitors/RegularElement.js`) визитит `OnDirective` в первой петле `other_directives` до атрибутов и `build_set_class`, поэтому derived-стейтменты `on:` всегда идут до cache-var класса. У нас в `crates/svelte_codegen_client/src/codegen/attributes/dispatch.rs` добавлен pre-pass по `OnDirectiveLegacy`, активный только когда class требует state и нет `use:` директивы (use-ветка кладёт `$.event(...)` в `pending_element_init` и сохраняет source-order сама). Owning layer: 4 codegen. Test: `diagnose_class_directive_legacy_event_handler_derived_order`.
- [x] Static empty `class=""` attribute on a regular element is elided from the `from_html` static template — reference emits `<div>x</div>` for `<div class="">x</div>`, our codegen renders `<div class="">x</div>`. Applies only to literal empty-string `class` (no expression, no concatenation, no class directives, no scoped-css class). Other attributes with empty literal values are kept as-is by reference. Owning layer: 4 codegen — `crates/svelte_codegen_client/src/codegen/data_structures/template.rs:122-158` (`stringify`) renders `class=""` unconditionally; the empty-class branch in `crates/svelte_codegen_client/src/codegen/attributes/dispatch.rs:188-196` already detects the case but still calls `set_attribute("class", Some(""))` instead of dropping both the template entry and the call. Test: `empty_class_attribute_static_elided`.
- [x] Regular-element attribute value `CallExpression` with **no captured reactive references and a pure-shape callee** (e.g. `style={Math.random()}`, `title={Date.now()}`) emits a single eager setter at init: `$.set_style(div, Math.random())`. Combined with literal `style:`-directives (`<div style={Math.random()} style:color="red">`), reference merges into one eager `$.set_style(div, Math.random(), {}, { color: "red" })` in init — no `let styles`, no `$.template_effect`. Owning layer: 3.A.3 `ExpressionSemantics` — `ExprKind::Call` carries a `dynamic: bool` field set from `!references.is_empty() || has_impure_call` (impure = callee is not a globally-rooted identifier/member chain, mirroring reference's `is_pure(callee)`); `SpreadElement` is impure-by-definition. `Call { dynamic: false }` then routes through the non-memo / non-dynamic-attr branches in dynamism, attribute-semantics, and codegen's `needs_memo` predicates the same way `KnownLiteral` does. Tests: `diagnose_attr_call_no_references_init`, `diagnose_style_attr_call_no_references_with_directive_init`.

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
- [x] `class_concat_logical_and_string`
- [x] `element_attr_import_meta_template_effect`
- [x] `attribute_effect_shorthand_prop_unwrap`
- [x] `diagnose_class_directive_call_in_template_effect`
- [x] `template_effect_attr_before_text_order`
- [x] `diagnose_style_directive_literal_skips_template_effect_grouping`
- [x] `diagnose_style_directive_quoted_literal_interp_static`
- [x] `diagnose_style_directive_complex_expression_hoists_to_memo_deps`
- [x] `diagnose_class_directive_slots_member_legacy`
- [x] `diagnose_attribute_effect_spread_call_memo`
- [x] `diagnose_style_attr_call_groups_in_template_effect_memo`
- [x] `attr_call_value_hoists_to_template_effect_memo`
- [x] `diagnose_legacy_input_bind_spread_omits_remove_defaults`
- [x] `diagnose_spread_bind_this_action_order`
- [x] `diagnose_textarea_bind_value_with_spread_init_order`
- [x] `diagnose_textarea_spread_only_emits_remove_child`
- [x] `diagnose_textarea_dynamic_value_attribute_emits_remove_child`
- [x] `diagnose_style_attr_dynamic_with_style_directive_merges_set_style`
- [x] `diagnose_attr_call_no_references_init`
- [x] `diagnose_style_attr_call_no_references_with_directive_init`
- [x] `diagnose_video_muted_static_attribute_lowers_to_property`
- [x] `diagnose_class_directive_legacy_event_handler_derived_order`
- [x] `empty_class_attribute_static_elided`
