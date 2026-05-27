# `CodegenView` + `ComponentScoping` wrapper removal

## Parent

`specs/analyzer-target-design.md`

## What to build

Удаляются две обёртки, потерявшие смысл после миграции потребителей на cluster API.

**`CodegenView`** (`types/data/codegen_view.rs`). Codegen читает `AnalysisData` через публичный API кластеров напрямую — обёртка удаляется целиком. Это закрытие User Story 30.

**`ComponentScoping`** обёртка над `ComponentSemantics`. analyze / transform / codegen используют `ComponentSemantics` напрямую. Helper-методы из `ComponentScoping` либо переезжают в сам `ComponentSemantics`, либо в `TemplateTopology` / `TemplateElementIndex` где уместнее.

Дополнительно удаляется пустой `BlockAnalysis` struct (Decision 27).

Возможны регрессии перформанса если кэш в `ComponentScoping` был критичный (Risk 5) — ловится benchmark-ами / профайлингом, не блокирует слайс.

Decision basis: §27 + §28 + §29 + Risk 5.

## Acceptance criteria

- [ ] `types/data/codegen_view.rs` удалён
- [ ] 0 чтений `CodegenView` (как типа) во всех crate-ах
- [ ] `ComponentScoping` обёртка удалена
- [ ] Helper-методы из `ComponentScoping` переехали в `ComponentSemantics` / `TemplateTopology` / `TemplateElementIndex`
- [ ] `BlockAnalysis` пустой struct удалён
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `05-element-semantics-cluster.md` (Element — последний крупный потребитель `CodegenView`)
