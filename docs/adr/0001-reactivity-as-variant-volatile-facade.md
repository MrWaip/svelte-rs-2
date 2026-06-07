# 1. Реактивность — это вариант, изменчивость — вердикт фасада

Статус: принято (2026-06-07; пересмотрено — порядок пассов reactivity-first)

## Контекст

Вокруг трёх фактов анализа — «реактивен ли символ», «изменчиво ли значение выражения»,
«известно ли значение» — разрослись ~12 синонимичных предикатов, размазанных по слоям и
продублированных: `needs_effect`, `emits_signal_read`, `is_reference_reactive`(`_for_prop_default`),
`is_unified_plain_symbol` (две байт-в-байт копии в `dynamism.rs` и `derive.rs`),
`is_reactive_component_binding`, `is_dynamic_template`, `is_dynamic_element_attr`,
`has_state_component_attr`, `is_dynamic_node/_attr/_component`, `DerivedDeclarationSemantics.{reactive,
value_known}`, варианты `ExprKind` с эмит-флагами.

Симптомы (карта: `docs/designs/dynamism-reactivity-value-map.md`):

- `needs_effect` собирал факт «нужен ли effect» из четырёх источников — нарушение инварианта
  композиции reactivity-semantics §5.
- `needs_effect`/`emits_signal_read` названы по рантайм-конструкции — анти-паттерн «Эмит-форма семантики».
- Изменчивость узла хранилась дважды (`ExprKind.reactive`-флаги и битсеты `DynamismData`).
- Кодген собирал вердикты, матча пары `ExprKind` — анти-паттерн «Анализ в кодгене».
- `has_state_attr` — мёртвая ветка.
- Целевой баг: `{x}` при `x = $derived(c.foo)` с непрозрачным `c` (`load()`) компилируется в статику
  вместо динамики, потому что динамичность derived бралась по реактивности зависимостей, а не по
  известности значения.

### Кросс-зависимость (ключевой факт, определивший порядок пассов)

Известность значения и реактивность **взаимозависимы**:

- Чтобы решить, можно ли оптимизировать `$derived` (свернуть в значение), нужна **известность** его init.
- Чтобы свернуть выражение, нужно знать, **реактивен ли** каждый символ внутри (мутируемый `$state`,
  prop, store, import, contextual → непрозрачен, не сворачивается). Это знание — `BindingSemantics`.

Попытка сделать вычисление значения «reactivity-free и первым» обречена: она вынуждает
re-derive реактивность из синтаксиса (`is_mutated || is_import || prop || store` руками), что
дублирует ReactivitySystem и нарушает инвариант #7 reactivity-semantics («JS-границы и реактивность —
только тут»). Это отвергнуто (см. Альтернативы).

Оригинал решает это **рекурсивным `evaluate`** (`original/compiler/phases/scope.js:803`), который
исполняется ПОСЛЕ классификации биндингов и читает метаданные биндинга (`binding.updated`,
`binding.kind`, `binding.initial`), разворачивая руны синтаксически и рекурсивно сворачивая
init по требованию, с re-entrancy-guard от циклов (`current_evaluations`, `scope.js:804`).

## Решение

Развести три факта по принципу «примитив языка ↔ надстройка-вердикт», убрать композицию из
потребителей, а кросс-зависимость разорвать порядком пассов (reactivity-first).

1. **Реактивность — это вариант `BindingSemantics`, не предикат.** «Реактивный источник?» = к какому
   полюсу относится вариант. Отдельные `emits_signal_read` / `is_reference_reactive` /
   `is_unified_plain_symbol` / `is_reactive_component_binding` удаляются; потребитель читает
   `binding_semantics(SymbolId)`.

2. **Единственную двусмысленность `Derived` чиним классификацией.** `$derived` со статически известным
   значением → новый вариант **`OptimizedDerived`** (как `OptimizedRune` для `$state`). Поля
   `DerivedDeclarationSemantics.{reactive, value_known}` удаляются — факт несёт выбор варианта.

3. **Изменчивость (`volatile`) — вердикт фасада, а не примитив реактивности.** Собирается фасадом
   `ExpressionSemantics` (template-выражения) и `ReactivitySystem` через `declarator_semantics`
   (скрипт-объявления) из двух фактов: `volatile = compose(вариант ссылок, известность значения)`.
   `needs_effect` уезжает из реактивности и растворяется в `volatile`. `reactive` — примитив
   реактивности, `volatile` — надстройка фасада.

4. **`ExprKind` растворяется в плоские поля `ExpressionData`:** `evaluation` (известность; вариант
   `KnownLiteral` удалён как дубль), `volatile`, `heavy` (содержит динамический вызов), `asynchronous`
   (содержит `await`). `SimpleRead`/`Computed` сливаются. Форма выноса (sync-ячейка vs async-слот) —
   derived-правило эмиссии, в анализе не хранится.

5. **Порядок пассов — reactivity-first; вычисление значения ПОТРЕБЛЯЕТ реактивность, не re-derive.**

   ```
   1) ReactivitySystem (main)        классифицирует BindingSemantics символов
   2) ValueEvaluation                зависит от (1): рекурсивный evaluate
        - идентификатор: symbol_for_reference(ref) -> SymbolId; читает по SymbolId
          binding_semantics(sym) (prop/store/import/contextual -> непрозрачно),
          is_mutated(sym), init декларатора (разворачивает руну, рекурсия в init)
        - cycle-guard на циклы derived (как scope.js), forward-ref корректен
        - НЕ ветвится по Derived/OptimizedDerived (его ещё нет и он не нужен)
        - produces: Evaluation по SymbolId и по выражению
   3) ReactivitySystem DerivedOptimizer (sub-pass)
        - relabel Derived -> OptimizedDerived там, где Evaluation(sym) == Known
        - чистый relabel вниз по потоку, без обратной связи в (2)
   ```

   Вычисление значения обращается к реактивности **по биндингу (`SymbolId`)** — резолвит use-site
   `ReferenceId` в `SymbolId` и читает `binding_semantics(sym)`; `reference_semantics(ReferenceId)`
   не используется (статическое значение — identity-by-symbol). Это **чтение** единого источника
   истины, а не переизобретение классификации.

6. **Удаляются** проход `DynamismVisitor` и хранилище `DynamismData` (изменчивость несёт фасад);
   мёртвая цепочка `has_state_attr`; `needs_effect`, `emits_signal_read`; дублирующие предикаты.
   Живые `is_dynamic_node/_attr/_component` переезжают на чтение `volatile`. Кодген читает готовые
   `volatile`/`heavy`/`asynchronous`/`evaluation`.

## Последствия

- Один факт — одно место: `reactive` (вариант) в реактивности, `volatile`/`heavy`/`asynchronous`/
  `evaluation` в `ExpressionData`. §5 соблюдён, трансформ тупой.
- Кросс-зависимость известность↔реактивность разорвана порядком: реактивность main → вычисление
  значения (читает её) → derived-оптимизатор (читает вычисление). Цикла нет, обратной связи нет.
- `ValueEvaluation` — потребитель реактивности по `SymbolId`, а не «reactivity-free примитив». Он не
  дублирует классификацию.
- Минус: `BindingSemantics` получает вариант `OptimizedDerived`; реактивность разбивается на main-часть
  и derived-optimizer sub-pass вокруг вычисления значения.
- Это **не** набор переименований: символы удаляются/сливаются/меняют слой — LSP-rename неприменим.

## Альтернативы

- **`value_evaluation` reactivity-free и первым** (вычисление до реактивности, непрозрачность из
  синтаксиса) — **отвергнуто**: непрозрачность включает prop/store/contextual/let-promotion, это
  реактивность; «первым» вынуждает re-derive её из синтаксиса и дублировать ReactivitySystem
  (нарушение инварианта #7). Правильно — reactivity-first, потребление по `SymbolId`.
- **Топологический проход по derived** (отсортировать граф зависимостей, считать в топопорядке) —
  **отвергнуто**: `$derived` — ленивый thunk, легально forward-reference на derived ниже по исходнику;
  source-order ≠ топопорядок. Оригинал не сортирует — рекурсивный `evaluate` с cycle-guard, порядок
  итерации безразличен.
- **Отдельная подсистема-фасад над скриптом** (`DeclarationSemantics`) — отвергнута: роль играет
  `declarator_semantics` у `ReactivitySystem`.
- **Ленивый `value_evaluation`-сервис** — отвергнут: дискретный билдер проще.
- **`Memoization { None, Sync, Async }`** (async как режим heavy) — отвергнут: `heavy` (вызов) и
  `asynchronous` (`await`) — разные ортогональные причины; лоссово для `{await foo()}`.
- **Трейт-абстракция вокруг непрозрачности (вид `*Oracle`)** — отвергнута: при reactivity-first
  остаётся одна реализация (чтение `binding_semantics`), абстракция-трейт лишняя.
