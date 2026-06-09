# PRD: ReactivitySemantics (корневой)

label: reactivity-semantics

Корневой PRD для кластера `crates/svelte_analyze/src/reactivity_semantics/`.
Описывает архитектурную рамку и перечень реактивных фич. Алгоритмы классификации каждой фичи — в дочерних PRD 

## Назначение

Единый ответ компилятора на вопрос «что реактивно в этом компоненте и как читается/пишется каждая ссылка».

Определяет режим работы реактивности: runes, soft, legacy hard

## Public API

Только три точки расширения наружу + узкие итераторы. `BindingFacts`/`ReferenceFacts` — приватные, наружу не видны. Builder — `pub(crate)`.

- `binding_semantics(SymbolId) -> BindingSemantics`
- `declarator_semantics(OxcNodeId) -> DeclaratorSemantics`
- `reference_semantics(ReferenceId) -> ReferenceSemantics`

## Архитектурные инварианты

1. **Identity by id.** Ключи — `SymbolId` / `OxcNodeId` / `ReferenceId`. Никаких имён в payload, никаких `find_binding_by_name`.
2. **Read-only после build.** Любая мутация — только через `pub(crate)` API в фазах 1/2.
3. **Один источник истины.** Никаких shadow-флагов реактивности на `ComponentSemantics`, `ScriptAnalysis`, `ElementAnalysis`.
4. **Totality.** `binding_semantics`/`reference_semantics` всегда возвращают вариант (`NonReactive`/`Unresolved` валидные), а не `Option`.
5. **Композиция факта.** Если потребитель собирает ответ из 2+ полей (`a && !b && c.kind==X`) — это разрыв контракта; недостающий вариант добавляется в `BindingSemantics`/`ReferenceSemantics`.
6. **Mode-агностичность API.** Один и тот же вариант (`SignalRead`, `StoreRead`, …) обслуживает Runes/SoftLegacy/HardLegacy — режим не разводит API, а влияет на классификатор внутри builder'а.
7. **JS-границы только тут.** `MaybeReactive` — единственный путь для импортов; другие подсистемы не угадывают «может это реактивно».

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
