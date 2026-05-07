# Phase 1 narrow builders consolidation

## Parent

`specs/analyzer-target-design.md`

## What to build

Финализация Phase 1 (scope & scripts) согласно spec'у. Цель — пять единиц с явным узким контрактом каждого builder-а:

1. **`ComponentSemanticsBuilder`** — остаётся как есть (живёт в crate `svelte_component_semantics`).
2. **`resolve_component_name()`** — util-функция (не builder), считает финальное имя резолвом конфликтов с symbols + reserved keywords. `passes/finalize_component_name` удаляется.
3. **`ScriptDeclarationsBuilder`** — узкий: `props_declaration`, `exports`, `props_id`. Полный `ScriptInfo` god-struct исчезает; внутренние поля либо переезжают в `ComponentSemantics` (если про symbols), либо удаляются. `passes/enrich_script_info` удаляется.
4. **`BlockerDataBuilder`** — async barriers, per-symbol map, `await_reactivity_loss_ignored` (последнее уже в #08).
5. **`CustomElementInfoBuilder`** — `is_custom_element_target`, `custom_element_compile_flag`, `ce_config` (parsed `<svelte:options customElement>`), `custom_element_slot_names`.

Дополнительно удаляются как самостоятельные сущности (нет внешних потребителей):

- `ProxyStateInits`
- `has_class_state_fields` (как top-level фактъ)
- `has_store_member_mutations` (как top-level факт)

Эти три были derivation steps для `RuntimePlan` — пересчитываются inline в `RuntimePlanBuilder` (cluster #07).

После слайса Phase 1 — линейный список из 5 явных вызовов в `analyze()`, читаемых сверху вниз.

Decision basis: §12 + §25.

## Acceptance criteria

- [ ] `ScriptDeclarationsBuilder` существует, узкий (`props_declaration`, `exports`, `props_id`)
- [ ] `ScriptInfo` god-struct исчез
- [ ] `passes/enrich_script_info` удалён
- [ ] `passes/finalize_component_name` удалён; `resolve_component_name()` — util-функция
- [ ] `BlockerDataBuilder` стабилен и содержит `await_reactivity_loss_ignored`
- [ ] `CustomElementInfoBuilder` существует с полным набором полей
- [ ] `ProxyStateInits` удалён
- [ ] `has_class_state_fields` удалён как top-level факт
- [ ] `has_store_member_mutations` удалён как top-level факт
- [ ] `analyze()` Phase 1 — линейный список 5 builder-вызовов
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `08-ignore-data-privatization.md` (для `BlockerDataBuilder` финализации с `await_reactivity_loss_ignored`)
