# `ExpressionSemantics` cluster (5-й кластер)

## Parent

`specs/analyzer-target-design.md`

## What to build

Новый кластер `ExpressionSemantics` — пятый кластер per Svelte kind. Идентичность — `NodeId` expression-узла. Используется и для standalone `{expr}` в фрагменте, и для expression внутри атрибута, и для `{#each expr as ...}` collection, и для аргумента `{@render fn(arg)}`.

Все факты для одного expression-узла лежат в одном payload:

- `legacy_wrap` — composition `runes() + needs_legacy_coarse_wrap + uses_legacy_sanitized_props` уезжает в analyzer. Codegen матчится по `LegacyWrap` enum один раз.
- `async_kind`
- `memoization` (включая `needs_clsx`)
- `role`
- `references`
- `is_pickled_await: bool` — заменяет `PickledAwaitOffsets` (offset → NodeId-лукап)
- `dynamism: bool` — bit, заменяющий `is_dynamic_attr(id)` композицию
- `expr_deps`, `expr_has_await`, `expr_has_blockers`, `is_each_index_sym`, `is_expression_shorthand` — поля payload

Codegen expression-pipeline (`wrap_expression`, `expr_*` helper-ы из `Ctx`) переключается на `data.expressions.get(id)` — один матч по варианту enum.

Transform `is_pickled_await(span.start)` переписывается на NodeId-лукап в `template_rewrites.rs`.

`ExpressionInfo` (per-expression bag-of-facts) растворяется в payload. `PickledAwaitOffsets` удаляется. `expr_*`-helper-ы исчезают из `Ctx`/`AnalysisData`.

Decision basis: §8 + §10 + §18 + §24 + §52.

## Acceptance criteria

- [ ] `ExpressionSemanticsBuilder` существует и регистрируется в Phase 4
- [ ] `ExpressionSemanticsStore` — top-level поле `AnalysisData::expressions`
- [ ] Все поля payload (legacy_wrap, async_kind, memoization, role, references, is_pickled_await, dynamism) пред-вычислены
- [ ] Codegen expression-pipeline читает `data.expressions.get(id)` единым матчем
- [ ] 0 чтений `ExpressionInfo` (как типа) из codegen и transform
- [ ] `ExpressionInfo` удалён
- [ ] `PickledAwaitOffsets` удалён
- [ ] Transform лукапит `is_pickled_await` через NodeId, не через span.start
- [ ] 0 helper-методов с префиксом `expr_` на `Ctx`/`AnalysisData`
- [ ] Юнит-тесты на `ExpressionSemanticsBuilder` per поле payload
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `02-block-semantics-debug-html-tag.md`
