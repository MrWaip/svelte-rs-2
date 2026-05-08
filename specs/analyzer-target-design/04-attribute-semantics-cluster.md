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

Payload содержит NodeId expression-а; expression-факты для **attribute/directive value-expressions** считаются builder-ом и хранятся либо как поля payload, либо в расширенном `ExpressionSemantics` store (scope расширяется с fragment-child ExpressionTag до всех attribute/directive expression-сайтов в этом спринте — см. cleanup ниже).

Codegen attribute-pipeline переключается на `data.attributes.get(id)` единым матчем.

`BindSemanticsData`, `TemplateSemanticsData`, `DirectiveModifierFlags` растворяются в payload. `bind_*`/`attr_*`-helper-ы исчезают из `Ctx`/`AnalysisData`.

### Cleanup, унаследованный из spec 03

Spec 03 ограничил scope `ExpressionSemantics` fragment-child ExpressionTag-сайтами. Полная ликвидация старой инфраструктуры expression-фактов (для attribute/directive value-expressions) делается **здесь**:

- Расширение scope `ExpressionSemanticsBuilder` до всех attribute/directive expression-сайтов (или эквивалентное поглощение этих фактов в `AttributeSemantics` payload).
- Удаление `crates/svelte_analyze/src/types/data/expr.rs` (`ExpressionInfo`, `ExpressionKind`, `ExprRole`, `ExprSite`, `ExprDeps`).
- Удаление `crates/svelte_analyze/src/types/data/pickled_await_offsets.rs` (`PickledAwaitOffsets`); transform `template_rewrites.rs` лукапит per AwaitExpression NodeId.
- Удаление `crates/svelte_analyze/src/passes/js_analyze/expression_info.rs`.
- Удаление `merge_concat_expression_info` и shadow-aggregation в `collect_symbols.rs`.
- Удаление helper-методов `expr_*` (`expr_deps`, `expr_role`, `expr_is_async`, `expr_has_await`, `expr_has_blockers`) с `Ctx`/`AnalysisData`.
- Удаление `attr_expression(id)`-accessor.
- Удаление `is_pickled_await(offset: u32)`-accessor.

Decision basis: §19 + §52 + (унаследовано из §18, §24).

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
- [ ] `ExpressionInfo`, `ExpressionKind`, `ExprRole`, `ExprSite`, `ExprDeps` удалены (унаследовано из spec 03)
- [ ] `PickledAwaitOffsets` удалён; transform `template_rewrites.rs` лукапит через NodeId (унаследовано из spec 03)
- [ ] 0 helper-методов с префиксом `expr_` на `Ctx`/`AnalysisData` (унаследовано из spec 03)
- [ ] 0 чтений `ExpressionInfo` (как типа) из codegen и transform (унаследовано из spec 03)
- [ ] `merge_concat_expression_info` удалён (унаследовано из spec 03)
- [ ] Юнит-тесты на каждый variant `AttributeSemanticsBuilder`
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `03-expression-semantics-cluster.md`
