# PRD: ExpressionSemantics (корневой)

label: expression-semantics
topics: template expression, volatility, suspension, value evaluation, Evaluation, memoization, heavy, asynchronous/await, value folding, references, evaluated reads, closure

Корневой PRD для модуля `svelte_analyze::expression_semantics` (3.A.3).
Дочерний по слою: `analyze.md`. Зависит от `ComponentSemantics`, `ReactivitySemantics` (фаза 1), `ComponentScoping`.

## Назначение

Per-expression факты для каждого template/attribute-выражения. На один expression-`NodeId` потребитель получает один `ExpressionSemantics`-вариант.

## Public API

- `ExpressionSemanticsStore::get(NodeId) -> &ExpressionSemantics` — тотальный. Out-of-range / non-expression id'ы коллапсят в `&ExpressionSemantics::NonSpecial`. Без `Option`.
- `ExpressionSemanticsStore::is_context_required() -> bool` — один component-wide булеан.

## Что несёт вариант

- expression kind (sync / async / non-special);
- `suspension: Suspension` — **приостановка выражения**: `None` / `Outermost` (выражение есть ровно один `await`, его операнд не ждёт снова) / `Interleaved` (ждёт и в других точках). Уточняет `Volatility::Asynchronous`;
- `Evaluation` — `Known(KnownValue)` / `Defined { class }` / `MaybeNullish { has_unknown }`; выражение несёт две оценки одного типа под разные модели исполнения: `evaluation` — значение при реактивном чтении (`ReadContext::Runtime`, реактивный источник непрозрачен), потребляет клиентский backend; `declared_evaluation` — значение при сколлапсированной реактивности (`ReadContext::Declaration`, `$state`/`$derived` свёрнуты к init, **const-теги** фрагмента свёрнуты к своему init), потребляет серверный backend, где рендер однократен. Обе target-агностичны — считаются всегда, backend выбирает свою; выбор формы (свернуть в статический текст vs `$.escape` в рантайме) живёт в кодгене. const-теги видит только `declared_evaluation`: их init-биндинги подмешиваются в declaration-эвалуатор, но не в runtime-эвалуатор и не в общий `ValueEvaluation` (тот питает `OptimizedDerived`), поэтому клиентская классификация не затрагивается;
- `LegacyWrap`-выбор для legacy reactive-контекстов;
- `references: SmallVec<SymbolId>` — биндинги, которые выражение упоминает где угодно, включая тела вложенных функций;
- `evaluated_reads: SmallVec<SymbolId>` — биндинги, которые выражение читает **при вычислении**: только чтения вне тел вложенных функций. Подмножество `references`;
- blocker-индексы.

Плюс единый агрегат `is_context_required()` — единственный component-wide вердикт «нужен ли компоненту runtime-контекст», по любому выражению шаблона **и** скрипта (напр. `$$props.x`, store-мутация, `$effect`). Потребитель читает его одним запросом; пере-выводить наблюдение контекста собственным обходом запрещено.

## Архитектурные инварианты

1. **Ключ — template-`NodeId` выражения**, не `OxcNodeId`.
2. **Один источник истины для `Evaluation` и `LegacyWrap`.** Трансформ / кодген никогда не пере-walk'ают выражение, чтобы их пересчитать.
3. **Read-only после build.**
4. **Тотальность.** Публичный `get` всегда возвращает вариант (`NonSpecial` валидный), не `Option`.
5. **Идентичность ссылки не подменяется.** `references` несёт `SymbolId` именно того биндинга, который выражение реально читает (store-чтение `$x` реально читает синтетический store-sub — он и лежит). Подписка биндинга — производный факт `ReactivitySemantics` (`store_symbol` в `LegacyStateSubscribed*` / `ImportSubscribedRead`, `store_shadow_of_internal`); потребитель, которому нужна подписка, разворачивает её сам — коллектор символ не перепрыгивает.

`references` отвечает на «какие биндинги выражение упоминает где угодно» (включая замыкания) — он для потребителей, которым важны захваченные чтения. Это **не** вход реактивности: реактивность (`Volatility`) — другой вопрос, «наблюдает ли вычисление выражения реактивный state», и замыкания на него не влияют. Решать форму чтения (getter / мемоизация) по `references` — значит смешать два разных вопроса.

Вход реактивности — `evaluated_reads`: тело вложенной функции при вычислении выражения не исполняется, поэтому прочитанное внутри неё (собственный параметр, локальная переменная, захваченный реактивный источник, `$store`, `$$props`) на вердикт не влияет. На `evaluated_reads` считаются все входы `Volatility` — реактивность идентификаторов, `Heavy` (динамический вход вызова), `needs_context` — и `LegacyWrap`. Перепутать списки — вернуть замыкания в реактивность: `{[(x) => x]}` станет изменчивым, хотя его значение постоянно.

`references` остаётся ответом про упоминания и питает **только** ось blocker'ов (`blockers` здесь, `push_expression_data` в кодгене): захваченный биндинг может блокировать выражение, даже если не читается при вычислении.

6. **Свёртка биндинга считается один раз на эвалуатор.** `ValueEvaluator` мемоизирует `Evaluation` биндинга по `SymbolId` (`binding_memo`). Без мемо свёртка экспоненциальна по глубине цепочки биндингов: `const aN = aN-1 + aN-1` заставляет каждый уровень пересчитывать предыдущий дважды, и цепочка из 20 деклараций компилируется четверть секунды, из 25 — секунды. Мемо-запись кладётся только если поддерево посчиталось без обрыва по cycle-guard (`cycle_truncated`): результат, усечённый циклом, зависит от того, какие init'ы были в обходе, и переиспользовать его нельзя. Оба эвалуатора (`Runtime` / `Declaration`) держат свой кеш — вердикт зависит от `ReadContext`; подмешивание const-тегов (`ingest_const_tag_bindings`) кеш сбрасывает.

7. **`suspension` собирается запросом к `AwaitSemantics`, не вторым обходом.** `BuildExpressionSemantics` объявляет `requires: AwaitSemantics` в `PASS_DESCRIPTORS`; «операнд ждёт снова» — это «в выражении есть другой `await` с вердиктом не `Detached`». Вопрос уровня выражения живёт здесь: у `AwaitSemantics` ключ — узел-`await`, и про выражение целиком он не отвечает.

## Связь с другими документами

- `context.md` §«Семантика и анализ» (мемоизация — `Memoization`-кластер живёт здесь), §«Реактивность».
- `analyze.md` — место в build order (между фазами `ReactivitySemantics`).
- `reactivity-semantics.md` — источник per-reference реактивных фактов.
- `await-semantics.md` — вердикт на отдельном `await` внутри выражения; другая гранулярность ключа.
- `block-semantics.md` — читает per-expression факты вместо параллельной классификации.
- `attribute-semantics.md` — со-потребитель тех же выражений.
