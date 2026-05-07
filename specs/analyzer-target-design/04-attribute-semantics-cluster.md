# `AttributeSemantics` cluster

## Parent

`specs/analyzer-target-design.md`

## What to build

Новый кластер `AttributeSemantics`. Идентичность — `NodeId` attribute-узла. Любая работа с атрибутом начинается с одного варианта enum `data.attributes.get(attr_id)`.

Варианты payload (минимально):

- `HtmlBind { target, each_context_vars, ... }`
- `Event { handler_mode, modifiers, ... }`
- `ComponentProp`
- `ComponentBind`
- `ComponentSpread`
- `Use`
- `Transition`
- `Animate`
- `Attach`
- `BoundaryHandler`

Композиции, которые сейчас собираются `&&`-цепочкой (`bind_target_semantics(id)` + `attr_expression(id)` + `event_modifiers(id)` и т.д.), уезжают в analyzer как поля payload. `each_context_vars` лежит явным полем в `HtmlBind` payload-е (Decision 9 user-story) — `parent_each_blocks(id)` исчезает как helper.

Payload содержит NodeId expression-а; expression-факты тянутся из `ExpressionSemantics` (cluster #03).

Codegen attribute-pipeline переключается на `data.attributes.get(id)` единым матчем.

`BindSemanticsData`, `TemplateSemanticsData`, `DirectiveModifierFlags` растворяются в payload. `bind_*`/`attr_*`-helper-ы исчезают из `Ctx`/`AnalysisData`.

Decision basis: §19 + §52.

## Acceptance criteria

- [ ] `AttributeSemanticsBuilder` существует и регистрируется в Phase 4 после `ExpressionSemanticsBuilder`
- [ ] `AttributeSemanticsStore` — top-level поле `AnalysisData::attributes`
- [ ] Все варианты payload (HtmlBind, Event, ComponentProp/Bind/Spread, Use, Transition, Animate, Attach, BoundaryHandler) реализованы
- [ ] Codegen attribute-pipeline читает `data.attributes.get(id)` единым матчем
- [ ] `BindSemanticsData` удалён
- [ ] `TemplateSemanticsData` удалён
- [ ] `DirectiveModifierFlags` удалён
- [ ] 0 helper-методов с префиксами `bind_` / `attr_` на `Ctx`/`AnalysisData`
- [ ] `parent_each_blocks(id)` исчез — `each_context_vars` поле payload
- [ ] Юнит-тесты на каждый variant `AttributeSemanticsBuilder`
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `03-expression-semantics-cluster.md`
