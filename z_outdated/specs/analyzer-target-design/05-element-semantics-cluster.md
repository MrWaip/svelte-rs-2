# `ElementSemantics` cluster

## Parent

`specs/analyzer-target-design.md`

## What to build

Новый кластер `ElementSemantics`. Идентичность — `NodeId` element-узла. Один semantic-дисптач закрывает HTML, `<svelte:element>`, `ComponentNode`, `<svelte:boundary>`, `<svelte:head>`, и остальные специальные таргеты вместо AST-node-kind дисптача `match node`.

Варианты payload:

- `Html` — пред-вычислены поля `slot_target`, `wrapped_in_css_container`, `is_customizable_select`, `needs_textarea_value_lowering`, `is_bound_contenteditable`, `needs_var`, `needs_input_defaults`, `is_void`, `is_custom_element`, `option_synthetic_value_expr`, `creation_namespace`, `is_css_scoped`, `class_needs_state`, `has_class_attribute`, `has_class_directives`, `has_style_directives`, `has_spread`, `has_use_directive`, `has_bind_group`, `has_attribute_named(...)`, `static_class`, `static_style`, `is_dynamic`
- `SvelteElement`
- `Component` — `is_dynamic_component`, `has_component_css_props`, `component_needs_legacy_props_marker`, `component_binding_sym`, `component_snippets`, `wrapped_in_css_container`
- `DynamicComponent`
- `SelfComponent`
- `Boundary`
- `SpecialTarget` × 4 sub-variants (`<svelte:head>`, `<svelte:window>`, `<svelte:document>`, `<svelte:body>`)

Payload `Html`-варианта несёт NodeId-ы атрибутов; attribute-факты тянутся из `AttributeSemantics` (cluster #04).

Codegen element-pipeline переключается с AST-node-kind dispatch на единый матч `data.elements.get(el_id)`.

`ElementFacts`, `ElementFlags`, `TitleElementData`, `html_tag_in_svg/mathml` растворяются в payload. `is_*`/`has_*`/`needs_*`/`class_*`/`static_*`-helper-ы для element-related queries исчезают из `Ctx`/`AnalysisData`.

Decision basis: §7 + §20 + §52.

## Acceptance criteria

- [ ] `ElementSemanticsBuilder` существует и регистрируется в Phase 4 после `AttributeSemanticsBuilder`
- [ ] `ElementSemanticsStore` — top-level поле `AnalysisData::elements`
- [ ] Все варианты payload (Html, SvelteElement, Component, DynamicComponent, SelfComponent, Boundary, SpecialTarget×4) реализованы с пред-вычисленными полями
- [ ] Codegen element-pipeline читает `data.elements.get(id)` единым матчем (нет AST-node-kind dispatch)
- [ ] `ElementFacts` удалён
- [ ] `ElementFlags` удалён
- [ ] `TitleElementData` удалён
- [ ] `html_tag_in_svg` / `html_tag_in_mathml` side-tables удалены
- [ ] 0 helper-методов с префиксами `is_` / `has_` / `needs_` / `class_` / `static_` для element-related queries на `Ctx`/`AnalysisData`
- [ ] Юнит-тесты на каждый variant `ElementSemanticsBuilder`
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `04-attribute-semantics-cluster.md`
