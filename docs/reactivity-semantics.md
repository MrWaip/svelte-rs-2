# PRD: ReactivitySemantics (корневой)

label: reactivity-semantics
topics: reactivity, rune, signal, $state, $derived, $props, $bindable, store/$store, legacy state, $: reactive statement, maybe-reactive, contextual (each-item/index/await), BindingSemantics, ReferenceSemantics, OptimizedDerived, proxy, writeback reactivity

Корневой PRD для кластера `crates/svelte_analyze/src/reactivity_semantics/`.
Описывает архитектурную рамку и перечень реактивных фич. Алгоритмы классификации каждой фичи — в дочерних PRD 

## Назначение

Единый ответ компилятора на вопрос «что реактивно в этом компоненте и как читается/пишется каждая ссылка».

Определяет режим работы реактивности: runes, soft, legacy hard

## Public API

Четыре точки расширения наружу + узкие итераторы. `BindingFacts`/`ReferenceFacts` — приватные, наружу не видны. Builder — `pub(crate)`.

- `binding_semantics(SymbolId) -> BindingSemantics`
- `declarator_semantics(OxcNodeId) -> DeclaratorSemantics`
- `reference_semantics(ReferenceId) -> ReferenceSemantics`
- `class_field_semantics(OxcNodeId) -> ClassFieldSemantics` — семантика доступа к полю класса (`this.field` / `this.#field`) по узлу доступа. Тотальный (`None`-дефолт, `is_field()`); варианты `State { kind, proxy }`, `Derived { kind }`. Композиция: `field_access_target` (резолв в декларацию) + `declarator_semantics` (вид поля) + per-write `proxy`. Трансформ — один запрос, без строк и резолва на своей стороне.

**Per-write proxy.** Флаг для `$.set(source, value, should_proxy)` живёт в самом варианте: `ReferenceSemantics::SignalWrite/SignalUpdate { proxy }` (identifier-записи) и `ClassFieldSemantics::State { proxy }` (приватное поле). Init-proxy `$state`-инициализатора — на семантике декларации (`StateDeclarationSemantics.proxied` / `ClassFieldStateSemantics.proxied`), не в сайд-таблице. Считает `finalize_proxy`: `proxy ⟺ kind == State ∧ оператор не-coercive ∧ правая часть проксируема`. Проксируемость — синтаксический `should_proxy` по типу AST-узла (порт Оригинала, не `ValueEvaluation`); для идентификатора — рекурсия по инициализатору через карту `init_proxyable`. Деструктур-листья переиспользуют тот же `ReferenceId`. Трансформ читает готовый флаг.

Факты уровня компонента — одной пачкой, в доменном виде:

- `summary() -> ReactivitySummary { props: PropsSummary, has_store_bindings, legacy: LegacySummary }` — единый агрегат компонент-уровневых фактов реактивности. `props` — runes-props (`has_props` / `has_bindable` / `has_custom_element`); `legacy` — `has_bindable_prop` (export let), `reads_props_object` (`$$props`), `reads_rest_props_object` (`$$restProps`), `has_member_mutated`. Потребители: `build_runtime_info`, `CodegenView` (sanitized legacy props). `has_bindable` — отдельный флаг, а не скан `bindings`: оптимизация в `finish()` стирает bindable-метку у read-only source-props.

Узкие запросы prop-фактов (поверх тех же `BindingFacts::Prop`, без композиции на стороне потребителя):

- `is_rest_prop(SymbolId) -> bool` (crate) — биндинг объявлен `...rest` в `$props()`-паттерне.
- `iter_runes_prop_symbols() -> impl Iterator<Item = SymbolId>` — runes-props в порядке объявления, без `Rest` и identifier-формы (`let props = $props()`); основа перечисления accessor-пропов (CE-метаданные, getter/setter `$$exports`). Имена потребитель резолвит сам: ключ — `binding_origin_key`, локальное имя — `symbol_name`.
- `prop_default_span(SymbolId) -> Option<Span>` — положение default-выражения пропа в исходнике; кодген перепарсит слайс для setter-дефолта (`set x($$value = <default>)`).
- `legacy_bindable_prop_symbols() / has_legacy_bindable_prop()` — `export let`-props (LEGACY(svelte4)).
- `legacy_bindable_prop_alias(SymbolId) -> Option<&str>` — exported-алиас `export { foo as bar }` для bindable-prop (LEGACY(svelte4)). Известное отступление от «identity by id» — хранит строку; кандидат на перенос источника в `ComponentSemantics`.

Узкие предикаты — методы на самих енамах семантик, каждый с exhaustive match по всем вариантам (новый вариант — ошибка компиляции, никаких `_`-проваливаний). У потребителей `matches!` по этим енамам запрещён: либо метод енама, либо локальный exhaustive `match` для сайт-специфичного набора.

`BindingSemantics`:

- `is_reactive()` — биндинг есть **Реактивный источник** по глоссарию: чтение идёт через реактивную ячейку рантайма либо трактуется реактивно (state/derived/prop/legacy-state/store/contextual/maybe-reactive). False — статичные чтения: `OptimizedDerived`/`OptimizedRune` (демотированы в плоские значения), `RuntimeRune` (статичные runtime-значения), `Const`, `NonReactive`, `LegacyApiExport`, `Unresolved`.
- `is_store()` — биндинг стор-подписки (`Store(_)` пишется только на `$`-символ).
- `is_props()` — проп любого вида: `$props()` или `export let` (LEGACY(svelte4)); `is_runes_prop()` / `is_legacy_prop()` — точные половины; `is_rest_props()` — `...rest` в `$props()`.
- `is_derived()`, `is_optimized_rune()`, `is_maybe_reactive()`, `is_non_reactive()`, `is_legacy_state()`, `is_reactive_const_tag()` — точечные вопросы.
- Аксессоры payload: `state() -> Option<StateDeclarationSemantics>`, `legacy_state() -> Option<LegacyStateSemantics>` — для вопросов с гардом (`kind == StateEager`, `var_declared`, `is_signal_source`).

`ReferenceSemantics`:

- `is_store_subscription()` — ссылка читает/пишет через стор-подписку.
- `is_legacy_props_object_read()` — чтение магического `$$props`/`$$restProps` (LEGACY(svelte4)).
- `is_prop_mutation()`, `is_bindable_prop_access()`, `is_rest_prop_member_rewrite()`, `is_legacy_state_member_mutation_root()`, `is_legacy_reactive_import_member_mutation_root()` — точечные вопросы.

`DeclaratorSemantics`:

- `group() -> DeclaratorGroup` (`Rune | Legacy | Contextual | Plain`) — групповой срез, когда важна только категория.
- `is_rune_props()`, `is_rune_derived()`, `is_legacy_props()`, `is_bindable_call()`; аксессор `class_field_state() -> Option<ClassFieldStateSemantics>`.

## Архитектурные инварианты

1. **Identity by id.** Ключи — `SymbolId` / `OxcNodeId` / `ReferenceId`. Никаких имён в payload, никаких `find_binding_by_name`. Поверхность `declarator_semantics(OxcNodeId)` — классификация **JS-узла** по id, не только буквального декларатора: builder пишет факты и на узлы присваиваний (class-поля в конструкторе), и на узлы вызовов runtime-рун (`RuntimeRuneCall { kind: RuntimeRuneKind }` — effect/inspect-семейства, `$host`, `$props.id`, `$state.snapshot`; в legacy факт не записывается). Сигнальные руны деклараций несут `RuneState`/`RuneDerived`/`RuneProps`/`ClassField*` — на узле вызова они не дублируются.
2. **Read-only после build.** Любая мутация — только через `pub(crate)` API в фазах 1/2.
3. **Один источник истины.** Никаких shadow-флагов реактивности на `ComponentSemantics`, `ScriptAnalysis`, `ElementAnalysis`.
4. **Totality.** `binding_semantics`/`reference_semantics` всегда возвращают вариант (`NonReactive`/`Unresolved` валидные), а не `Option`.
5. **Композиция факта.** Если потребитель собирает ответ из 2+ полей (`a && !b && c.kind==X`) — это разрыв контракта; недостающий вариант добавляется в `BindingSemantics`/`ReferenceSemantics`.
6. **Mode-агностичность API.** Один и тот же вариант (`SignalRead`, `StoreRead`, …) обслуживает Runes/SoftLegacy/HardLegacy — режим не разводит API, а влияет на классификатор внутри builder'а.
7. **JS-границы только тут.** `MaybeReactive` — единственный путь для импортов; другие подсистемы не угадывают «может это реактивно».
8. **Ортогональные оси — раздельно** (уточнение границы #5). #5 требует завести вариант, когда потребитель собирает **один** доменный ответ из сырых полей на **одном** сайте. Но когда фактов **две независимые оси**, спрашиваемые в **разных** местах, их декартово произведение — **не** доменная категория:
   - (а) **Не плоди N×M вариантов.** Признак нарушения — имя варианта склеено из двух ортогональных прилагательных (`Destructured`+`Default`+`Legacy`). Каждая ось — свой запрос по id (`binding_semantics` / `reference_semantics`), потребитель комбинирует на месте.
   - (б) **Выводимое не храни.** Если ось выводится из другой оси + структуры AST (`walk_bindings`, форма bind-выражения identifier-vs-member), её не держат ни вариантом енама, ни side-table/методом по `SymbolId` — потребитель выводит на месте. Side-table законен только для **невыводимого** факта (как `each_rest_symbols`), не для «срезать угол».

## Карта реактивных фич (scope корневого PRD)

Корневой документ фиксирует **только перечень и роль**. Правила классификации — в дочерних PRD.

### Mode
- **Mode resolution** — определяет `RunesMode::{Runes, SoftLegacy, HardLegacy}` и `uses_runes`; влияет на классификатор, не на API.

### Runes (сигнальная реактивность)
- **State-семья** — `$state` / `$state.raw`, proxied/raw, оптимизированные формы. PRD: `state-rune`.
- **Derived** — `$derived` / `$derived.by`, sync/async, destructured-варианты.
- **Props** — `$props()` identifier/object/rest, `$bindable`, custom-element mode.
- **Runtime-руны без сигнала** — `$effect.tracking/pending`, `$host`, `$inspect.trace`, `$props.id`.
- **Class-field руны** — `state`/`derived` на полях класса.

### Legacy (Svelte 4)
- **`export let` props** — legacy bindable props.
- **Классификация экспортов** — единственный владелец развилки «каждый экспортированный let/var/class — bindable prop, всё прочее — API-экспорт компонента»; specifier со строковым exported-именем в legacy пропускается целиком. В runes-режиме та же точка классифицирует тривиально (каждый export — API-экспорт). Внутри кластера остаются только реактивные следствия (bindable-метки, `LegacyApiExport`-вариант `binding_semantics`); классифицированный список API-экспортов и флаг «есть export-стейтмент ⇒ нужен `$$props`» — выводы об компоненте в целом, лежат в `OutputData` (`api_exports`, `legacy_has_export_declaration`), поверхность кластера не расширяют. Кодген и трансформ список/метки только читают, своей классификации не держат.
- **Let-promotion в state** — top-level `let`, мутируемый в `$:` или handler'е.
- **`$:`-блоки** — legacy reactive statements/declarations.

### Stores (observable)
- **Декларации stores** — store-биндинги и их shadow-символы.
- **`$store`-автоподписка** — синтетические биндинги и read/write/update.
- **Legacy + stores** — автоподписка поверх legacy state и импортов.
- **Import-subscribed** — `$`-подписка на импортированный store.

### Cross-cutting
- **MaybeReactive** — кросс-модульные импорты неизвестной реактивности.
- **Contextual bindings** — each-item/index, await value/error, let-directive, snippet-params.
- **Read-form «параметр» (`raw_param`)** — each-контекстная ячейка читается сыро, потому что объемлющее замыкание получает её параметром: ключ `{#each … (key)}` и get/set `bind:this`. Это read-form-факт (доменно — reactivity), он лежит на самом варианте ссылки (`ContextualReadKind::EachItem/EachIndex`, `LegacyEachItemMemberMutationRoot`, `EachItemMemberMutationStoreInvalidate`), а не отдельной таблицей у потребителя — потребитель читает один вариант. Сканирование позиций идёт по общему AST на фазе build (как у each-key, так и у `bind:this`); промежуточный `raw_param_reads` — узаконенный builder-time staging: сворачивается в вариант при классификации и за пределы анализа (в transform/codegen) не выходит. Аналогично proxy-таргет `bind:this` доводится штатным `set_signal_write_proxy`, а не shadow-таблицей.
- **Const-tag реактивность** — `{@const}` как реактивный alias.
- **Let-carrier desugaring** — destructure-инициализаторы через carrier-символ.
- **Each-item member-mutation** — мутации полей item-а в `{#each}`.
- **IllegalWrite** — запись по read-only ссылке (парность диагностик).

## SSR

Заглушка — ожидает `/audit`. Раскрывает, как `ReactivitySemantics` потребляется server-pipeline'ом и что меняется в правилах эмита по сравнению с клиентом.

## Связь с другими документами

- `analyze.md` — родительский слой; место в build order (фаза 1 script-only → фаза 2 template-walk).
- `context.md` §«Реактивность», §«Скоупы, биндинги, ссылки» — терминология.
- `component-semantics.md` — scope-граф и `OxcNodeId`-биндинги, на которых строится классификация.
- `bindings-and-references.md` — система идентификаторов под нашим API: биндинг/ссылка, `SymbolId`/`ReferenceId`, `BindingPattern`/`walk_bindings`, плоская семантика (контракт `binding_semantics`/`reference_semantics`).
- Дочерние PRD: `state-rune.md`.
