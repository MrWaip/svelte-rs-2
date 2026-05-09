# `svelte_check` crate creation + validation extraction

## Parent

`specs/analyzer-target-design.md`

## What to build

**HITL slice** — публичная API surface `svelte_analyze` расширяется; каждый новый pub-метод проходит ревью (Risk 3).

Создаётся новый crate `svelte_check`, в который переезжает вся валидация:

- `validate/{runes, legacy, stores, non_reactive_update, experimental_async}` (целиком из `svelte_analyze`)
- `passes/template_validation` целиком, включая `a11y`
- `IgnoreData` сканер (`<!-- svelte-ignore -->`) — приватная структура `svelte_check`, не утекает в `AnalysisData`
- `warning_filter` логика

`ComponentChecker` (один crate-level entry point) с visitor-ами по правилам. Читает готовую `AnalysisData` через публичный API analyze. Один template walk + один script walk на checker.

Cluster builders диагностики не эмитят — нашли невалидное состояние, возвращают `Unresolved`-вариант или ассертят (баг анализатора).

`svelte_compiler` оркестрирует pipeline `parse → analyze → check → transform → codegen`.

Граница crate-ов:

- `svelte_check` зависит от `svelte_analyze`
- `svelte_analyze` НЕ зависит от `svelte_check`
- Публичный API `AnalysisData` достаточен для всех внешних потребителей без `pub(crate)` хаков

`crates/svelte_analyze/src/validate/` каталог удаляется. `passes/template_validation*` каталог удаляется (после переезда).

Performance-чувствительные потребители (бенчмарки, wasm) могут пропустить Phase 6 checker для минимального времени analyze.

Decision basis: §17 + §33 + §34 + User Story 32.

## Acceptance criteria

- [ ] Crate `crates/svelte_check/` существует
- [ ] `ComponentChecker` — public entry point
- [ ] Все правила (`runes`, `legacy`, `stores`, `non_reactive_update`, `experimental_async`, `template_validation`, `a11y`) переехали в `svelte_check`
- [ ] `IgnoreData` сканер — приватная часть `svelte_check`
- [ ] `warning_filter` логика — внутри `svelte_check`
- [ ] `svelte_compiler` оркестрирует `parse → analyze → check → transform → codegen`
- [ ] `svelte_check` зависит от `svelte_analyze`; `svelte_analyze` не зависит от `svelte_check`
- [ ] Публичный API `AnalysisData` достаточен — нет `pub(crate)` хаков для checker
- [ ] `crates/svelte_analyze/src/validate/` каталог удалён
- [ ] `passes/template_validation*` удалён из analyze
- [ ] Cluster builders не эмитят диагностики (grep по `Diagnostic` / `Severity` в cluster builders → 0)
- [ ] Per-rule unit-тесты на `ComponentChecker`
- [ ] Skipping Phase 6 checker возможно для performance-чувствительных потребителей
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена
- [ ] Каждый новый публичный pub-метод `svelte_analyze` прошёл review

## Blocked by

- `08-ignore-data-privatization.md`
