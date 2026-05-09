# `IgnoreData` privatization + `await_reactivity_loss_ignored` lift в `BlockerData`

## Parent

`specs/analyzer-target-design.md`

## What to build

Подготовка границы `analyze → check → transform`: runtime-влияющие ignore-коды поднимаются как узкие пред-вычисленные поля в payload соответствующего store/cluster, а `IgnoreData` готовится к переезду в `svelte_check` (физический переезд — в слайсе #09).

Для этого:

- `BlockerData` получает поле `await_reactivity_loss_ignored: FxHashSet<OxcNodeId>`.
- `BlockerDataBuilder` сканирует `<!-- svelte-ignore await_reactivity_loss -->` коды и заполняет это множество.
- Transform `enter_for_of_statement` читает множество вместо `is_ignored(..., "await_reactivity_loss")`.
- Поле `hydration_html_changed_ignored` уже снято в слайсе #02 (`BlockSemantics::HtmlTag.hydration_html_changed_ignored`).

После этого слайса в `svelte_codegen_client` и `svelte_transform`:

- 0 случаев `view.is_ignored(id, "<code-string>")`
- 0 чтений `IgnoreData` (как типа)

`IgnoreData` остаётся внутри analyze для остальных (не runtime-влияющих) ignore-кодов до создания `svelte_check` крейта в слайсе #09.

Decision basis: §48.

## Acceptance criteria

- [ ] `BlockerData` имеет поле `await_reactivity_loss_ignored: FxHashSet<OxcNodeId>`
- [ ] `BlockerDataBuilder` заполняет это множество из `<!-- svelte-ignore await_reactivity_loss -->`
- [ ] Transform `enter_for_of_statement` читает множество вместо `is_ignored(..., "await_reactivity_loss")`
- [ ] 0 вхождений `view.is_ignored(id, "code-string")` в `svelte_codegen_client`
- [ ] 0 вхождений `view.is_ignored(id, "code-string")` в `svelte_transform`
- [ ] 0 чтений `IgnoreData` (как типа) из `svelte_codegen_client`
- [ ] 0 чтений `IgnoreData` (как типа) из `svelte_transform`
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `02-block-semantics-debug-html-tag.md`
