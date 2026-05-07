# `ScriptRuneCalls` → side-output `ReactivitySemantics` + transform на `rune_calls()`

## Parent

`specs/analyzer-target-design.md`

## What to build

`ScriptRuneCalls` (per-OxcNodeId rune-classification map) перестаёт быть top-level полем `AnalysisData` и становится приватной структурой внутри `ReactivitySemantics`. Появляется новый pub-метод `ReactivitySemantics::rune_calls()` (либо узкий `rune_kind_at(node) -> Option<RuneKind>`).

`svelte_transform` через этот accessor делает classification всех rune-зависимых сайтов: class-state-fields в class body, `this.x = $state(...)` в конструкторах, rune-инициализаторов в declarators, `$inspect`-calls. Локальные AST-walks с `rune_kind_from_expr`-помощниками внутри transform-а уходят целиком.

Старый pass `passes/js_analyze/script_runes.rs` удаляется. Один pass классифицирует rune-call-узлы, не два независимых.

Decision basis: §13 (pipeline) + §49 (transform single-match).

## Acceptance criteria

- [ ] `ReactivitySemantics::rune_calls()` (или эквивалентный accessor) — публичный API
- [ ] `ScriptRuneCalls` исчез как top-level поле `AnalysisData`
- [ ] `passes/js_analyze/script_runes.rs` удалён
- [ ] 0 AST-classification walks в `svelte_transform` для rune-распознавания (grep по `rune_kind_from_expr` / аналогам в transform — 0 строк)
- [ ] Transform classification class-state-fields, `this.x = $state(...)`, rune-init declarators, `$inspect`-calls идёт через accessor
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена для этой миграционной единицы

## Blocked by

None — can start immediately.
