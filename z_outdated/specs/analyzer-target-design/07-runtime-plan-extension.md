# `RuntimePlan` extension: `ParamsShape` + `EpilogueKind` + bitflags + enriched `StoresSetup`

## Parent

`specs/analyzer-target-design.md`

## What to build

`RuntimePlanBuilder` поглощает inline-сборки в `lib.rs::generate` и производит **полное** emit-описание component-функции. Codegen матчится по полям/вариантам без ИЛИ-цепочек и без AST-walks за component-level фактами.

Финальная форма `RuntimePlan`:

- `fn_params: ParamsShape { AnchorOnly, AnchorAndProps }` — сводный enum для (`needs_props_param` ∨ `has_bubble_events` ∨ `has_legacy_slots` ∨ `needs_sanitized_legacy_slots`). `has_bubble_events` / `has_legacy_slots` остаются приватными intermediates `RuntimePlanBuilder`, в публичный API не попадают.
- `epilogue: EpilogueKind { Nothing, Cleanup, PopExpression, PopAndCleanup, PopWithReturn, PopWithReturnAndCleanup }` — единый enum для матрицы (`needs_push`, `needs_pop_with_return`, `has_stores`). Невалидные комбинации делает невозможными по типу.
- `bare_imports: BareImportsFlags { ASYNC, LEGACY, TRACING, DEV_FILENAME }` — bitflags для prelude-импортов.
- `exports_flags: ExportsFlags { HAS_EXPORTS, HAS_CE_PROPS, HAS_LEGACY_ACCESSOR_PROPS }` — bitflags. Сводный `has_explicit_exports = !flags.is_empty()` — derive-метод.
- `stores_setup: Option<StoresSetup { bindings: Vec<EnrichedStoreBinding> }>`. `EnrichedStoreBinding { base_symbol, store_symbol, base_via_legacy_state: bool }` — per-store enriched факт.
- `needs_ownership_validator: bool` — orthogonal feature, plain bool. Решение принимается analyzer-ом (а не транзит-ИЛИ из transform-output + analyzer-fact как сейчас).

Codegen `lib.rs::generate` переходит:

- от ИЛИ-цепочек `runtime.needs_props_param || has_bubble_events || ...` → к матчу по `ParamsShape`
- от 4-way if/else-цепочки (`needs_push`, `needs_pop_with_return`, `has_stores`) → к матчу по `EpilogueKind`
- от 4 независимых bool-проверок prelude-импортов → к bit-тестам `bare_imports`
- от 3 независимых bool-проверок exports → к bit-тестам `exports_flags`
- от per-store повторного `binding_semantics(base_symbol)`-лукапа в loop-е → к чтению enriched `EnrichedStoreBinding.base_via_legacy_state`

AST-walks в codegen за `has_bubble_events` / `has_legacy_slots` уходят — они приватные intermediates `RuntimePlanBuilder`.

Атомарным шагом (Risk 6: массивный refactor одного файла, ломающий несвязанные тесты на промежуточных состояниях). Один builder-расширение + один codegen-переход, обязательный green `compiler_tests/cases2` перед merge.

Decision basis: §47 + §50.

## Acceptance criteria

- [ ] `RuntimePlan` имеет поля `fn_params`, `epilogue`, `bare_imports`, `exports_flags`, `stores_setup`, `needs_ownership_validator`
- [ ] `ParamsShape` enum реализован (2 варианта)
- [ ] `EpilogueKind` enum реализован (6 вариантов)
- [ ] `BareImportsFlags` bitflags реализован
- [ ] `ExportsFlags` bitflags реализован
- [ ] `StoresSetup` / `EnrichedStoreBinding` реализованы; `iter_store_bindings()` возвращает enriched
- [ ] `has_bubble_events` / `has_legacy_slots` — приватные intermediates `RuntimePlanBuilder`, не в публичном API
- [ ] 0 ИЛИ-цепочек в `svelte_codegen_client/src/lib.rs::generate` для решений уровня "shape компонент-функции"
- [ ] 0 AST-walks в codegen за component-level фактами (`has_bubble_events`, `has_legacy_slots`)
- [ ] Codegen per-store loop НЕ делает повторный `binding_semantics(base_symbol)`-лукап
- [ ] `just test-compiler` зелёный — JS-output не меняется
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `06-fragment-semantics-store.md`
