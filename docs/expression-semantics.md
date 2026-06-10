# PRD: ExpressionSemantics (корневой)

label: expression-semantics

Корневой PRD для модуля `svelte_analyze::expression_semantics` (3.A.3).
Дочерний по слою: `analyze.md`. Зависит от `ComponentSemantics`, `ReactivitySemantics` (фаза 1), `ComponentScoping`.

## Назначение

Per-expression факты для каждого template/attribute-выражения. На один expression-`NodeId` потребитель получает один `ExpressionSemantics`-вариант.

## Public API

- `ExpressionSemanticsStore::get(NodeId) -> &ExpressionSemantics` — тотальный. Out-of-range / non-expression id'ы коллапсят в `&ExpressionSemantics::NonSpecial`. Без `Option`.
- `ExpressionSemanticsStore::is_context_required() -> bool` — один component-wide булеан.

## Что несёт вариант

- expression kind (sync / async / non-special);
- `Evaluation` — `Known(KnownValue)` / `Defined { class }` / `MaybeNullish { has_unknown }`;
- `LegacyWrap`-выбор для legacy reactive-контекстов;
- `references: SmallVec<SymbolId>`, реально прочитанные выражением;
- blocker-индексы.

Плюс единый агрегат `is_context_required()` — true, если любое выражение компонента трогает rest-prop members, store-мутацию или import-or-prop members и потому требует runtime-контекст.

## Архитектурные инварианты

1. **Ключ — template-`NodeId` выражения**, не `OxcNodeId`.
2. **Один источник истины для `Evaluation` и `LegacyWrap`.** Трансформ / кодген никогда не пере-walk'ают выражение, чтобы их пересчитать.
3. **Read-only после build.**
4. **Тотальность.** Публичный `get` всегда возвращает вариант (`NonSpecial` валидный), не `Option`.
5. **Идентичность ссылки не подменяется.** `references` несёт `SymbolId` именно того биндинга, который выражение реально читает (store-чтение `$x` реально читает синтетический store-sub — он и лежит). Подписка биндинга — производный факт `ReactivitySemantics` (`store_symbol` в `LegacyStateSubscribed*` / `ImportSubscribedRead`, `store_shadow_of_internal`); потребитель, которому нужна подписка, разворачивает её сам — коллектор символ не перепрыгивает.

## Связь с другими документами

- `context.md` §«Семантика и анализ» (мемоизация — `Memoization`-кластер живёт здесь), §«Реактивность».
- `analyze.md` — место в build order (между фазами `ReactivitySemantics`).
- `reactivity-semantics.md` — источник per-reference реактивных фактов.
- `block-semantics.md` — читает per-expression факты вместо параллельной классификации.
- `attribute-semantics.md` — со-потребитель тех же выражений.
