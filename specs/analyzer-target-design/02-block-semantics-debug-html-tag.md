# BlockSemantics: `DebugTag` + `HtmlTag` варианты + `hydration_html_changed_ignored`

## Parent

`specs/analyzer-target-design.md`

## What to build

`BlockSemantics` доделывается двумя вариантами: `DebugTag` и `HtmlTag`. Логически они block-теги (форма `{@...}`), и должны попадать под единый contract «codegen, держащий block-узел: `data.blocks.get(block_id)` — один enum».

Payload каждого нетривиален:

- `DebugTagSemantics { identifiers: SmallVec<[NodeId; 4]>, dev_only: bool }`
- `HtmlTagSemantics { expression_node_id, parent_strategy, hydration_html_changed_ignored: bool }`

`hydration_html_changed_ignored` — узкий пред-вычисленный fact от `<!-- svelte-ignore hydration_html_changed -->`. Codegen `emit_html_tag` читает это поле напрямую вместо `view.is_ignored(id, "hydration_html_changed")` (Decision 48).

Codegen `emit_debug_tag` / `emit_html_tag` мигрируют со старых side-tables (`DebugTagData`, `is_ignored` лукап по string-коду) на матч по варианту `BlockSemantics`.

`DebugTagData` удаляется.

Decision basis: §9 + §21 + §48.

## Acceptance criteria

- [ ] `BlockSemantics::DebugTag` вариант существует с полным payload
- [ ] `BlockSemantics::HtmlTag` вариант существует с полным payload, включая `hydration_html_changed_ignored`
- [ ] Codegen `emit_debug_tag` читает `data.blocks.get(id)` вместо `DebugTagData`
- [ ] Codegen `emit_html_tag` читает `hydration_html_changed_ignored` поле вместо `is_ignored(..., "hydration_html_changed")`
- [ ] `DebugTagData` side-table удалён
- [ ] 0 вхождений `is_ignored(..., "hydration_html_changed")` в codegen
- [ ] Юнит-тесты на `BlockSemanticsBuilder` для обоих вариантов
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] `SEMANTIC_LAYER_ARCHITECTURE.md` обновлён (out-of-scope строки 162–163 → in-scope)

## Blocked by

None — can start immediately.
