# Context

Rust-компилятор Svelte v5. `.svelte` → client-side JS + scoped CSS.

Доменный язык — секция `## Language` ниже. Инварианты и сопутствующие документы:

- **`ARCHITECTURE.md`** — правила и инварианты по крэйтам (ast → parser → analyze → transform → codegen → compiler).
- **`CODEBASE_MAP.md`** — крэйты, API, типы, pipeline.
- **`CLAUDE.md`** — догмы (smart analyzer / dumb codegen), банлист имён («Never use»), правила взаимодействия.
- **`ROADMAP.md`** — статус портирования по фичам, ссылки на спеки.
- **`debt.md`** — известный технический долг.
- **`specs/`** — спеки портирования по фичам.
- **`reference/compiler/`** — JS-референс. Используется для понимания **что** портировать, не **как**.

ADR не ведём — архитектурные решения фиксируются в `ARCHITECTURE.md` и `debt.md`.

## Language

Под **OXC** в проекте имеется в виду Rust-библиотека парсинга и AST JavaScript/TypeScript: `oxc_parser`, `oxc_ast`, `oxc_semantic`. JS-часть `.svelte` парсится OXC; типы с префиксом `Oxc` или из `oxc_*` крэйтов — OXC, всё прочее — наш домен.

`_Avoid_` каждой статьи дополняет общий банлист имён в `CLAUDE.md` `## Never use` (запрет на употребление в новых идентификаторах кода).

### Pipeline и оригинал

**Оригинал** *(en: Original, original compiler)* — JS-компилятор Svelte 5 в `reference/compiler/`, источник истины для соответствия выходного JS.
_Avoid_: эталон, reference compiler, JS compiler.

**Парсер** *(en: parser, parse, parsing)* — стадия компиляции, превращающая `.svelte` в AST (template + JS + CSS).

**Анализ** *(en: analyze, analyzer)* — стадия компиляции, производящая семантику над AST.
_Avoid_: analysis, semantic phase, семантический анализ.

**Трансформ** *(en: transform)* — стадия компиляции, мутирующая JS AST под рантайм Svelte.
_Avoid_: lowering, lower.

**Кодген** *(en: codegen)* — стадия компиляции, превращающая AST в выходной JS-код.
_Avoid_: code generation, generator.

**Рантайм** *(en: runtime)* — JS-модуль `svelte/internal/client` (и `…/server` для SSR) с примитивами реактивности и DOM-операций (`$.state`/`$.template_effect`/`$.append`/…), импортируемый выходом кодгена под namespace `$`.
_Avoid_: framework, environment.

### Компонент, шаблон и его узлы

**Компонент** *(en: component)* — единица компиляции `.svelte`-файла (скрипт + шаблон + стиль); use-site в шаблоне см. **Элемент** и `Flagged ambiguities`.
_Avoid_: модуль (см. `Flagged ambiguities` про «модуль»).

**Шаблон** *(en: template)* — markup-часть `.svelte`-файла.
_Avoid_: markup, html.

**Фрагмент** *(en: fragment, `Fragment`)* — последовательность шаблонных узлов под общим родителем.
_Avoid_: block.

**Элемент** *(en: element)* — узел шаблона вида `<name>` (HTML-элемент → `RegularElement`; компонент → `Component`; `<svelte:...>` → `Svelte*`).
_Avoid_: тег.

**Блок** *(en: block; кластер `BlockSemantics`)* — шаблонная конструкция `{#name args}…{/name}` с закрывающим маркером; семейство `{#if}` / `{#each}` / `{#await}` / `{#key}` / `{#snippet}`.
_Avoid_: @-тег.

**@-тег** *(en: at-tag; в AST — `*Tag`)* — одиночная шаблонная конструкция `{@name args}` без закрывающего; семейство `{@const}` / `{@html}` / `{@debug}` / `{@render}` / `{@attach}`.
_Avoid_: блок, элемент, голое «тег».

**const-тег** *(en: const tag, `ConstTag`)* — @-тег `{@const name = expr}`, объявляющий локальное значение для потомков фрагмента.
_Avoid_: const, const block.

**Атрибут** *(en: attribute; кластер `AttributeSemantics`)* — пара «имя=значение» на элементе (`name="..."`, `name={expr}`, `name`, `{shorthand}`, `{...rest}`).
_Avoid_: директива.

**Директива** *(en: directive)* — расширение синтаксиса атрибута с префиксом и двоеточием: `bind:`, `on:`, `use:`, `class:`, `style:`, `transition:`, `animate:`, `in:`, `out:`.
_Avoid_: атрибут.

**`bind:`-директива** *(en: bind directive; AST `BindDirective`)* — директива двусторонней связи: значение биндинга подставляется в атрибут элемента/компонента, изменения внутри элемента пропагируются обратно в тот же биндинг.
_Avoid_: одиночное «биндинг» в этом смысле (см. `Flagged ambiguities`); two-way binding (общее слово, не наше имя).

**Пропс** *(en: prop, props)* — входное значение компонента: объявляется через `$props()` (runes) или `export let X` (legacy) и передаётся потребителем как атрибут на `<Component>`-элементе.
_Avoid_: parameter, аргумент, проперти.

**Сниппет** *(en: snippet)* — переиспользуемый фрагмент шаблона `{#snippet name(args)}…{/snippet}`, вызываемый `{@render name(args)}`; при hoistable-классификации в анализе — эмитится кодгеном в hoisted-bucket.
_Avoid_: partial, фрагмент.

**Рендер** *(en: render)* — функция, в которую кодген компилирует компонент (`function App($$anchor, $$props) {…}`), исполняется рантаймом для построения и обновления DOM.
_Avoid_: render-вызов сниппета (это `{@render}` как @-тег), DOM-операция (это действие рантайма, не сущность).

**Слот** *(en: slot)* — legacy-механизм проброса контента в компонент через `<slot>` / `<div slot="name" />` / `<svelte:fragment slot="…">`.
_Avoid_: сниппет, placeholder.

### Скоупы, биндинги, ссылки

**Скоуп** *(en: scope; `Scope`/`ScopeId`/`ScopeFlags`/`ScopeTable`)* — узел дерева лексических областей видимости JS, держащий свои биндинги и ссылку на родителя.
_Avoid_: пространство имён, namespace, контекст.

**Биндинг** *(en: binding; `Symbol`/`SymbolId`; LHS-узел OXC — `BindingPattern`)* — именованная семантическая идентичность, объявляемая декларатором.
_Avoid_: переменная, variable, слот.

**Декларатор** *(en: declarator; OXC `VariableDeclarator`)* — одна пара «pattern = initializer» внутри декларации.

**Декларация** *(en: declaration)* — JS-стейтмент-контейнер деклараторов: `let/const/var/function/class/import/export`.

**Ссылка** *(en: reference, `Reference`/`ReferenceId`; OXC `IdentifierReference`)* — use-site идентификатора в JS или шаблоне, резолвящееся анализом в биндинг.
_Avoid_: указатель, pointer, link.

**Shadowing** *(en: shadowing)* — стандартная JS-семантика: внутренний скоуп перекрывает биндинг с тем же именем из внешнего.
_Avoid_: перекрытие, override.

**Hoisting** *(en: hoisting; `HoistedBucket`, `HoistedKind`, модуль `codegen/hoisted/`)* — операция кодгена: подъём определённых конструкций (сниппеты, `{@const}`, `{@debug}`, `<svelte:head/window/document/body>`) из их текстовой позиции во фрагменте в отдельный bucket, эмитимый выше.
_Avoid_: JS-hoisting (`var`/function-декларации в scope-семантике — это другое, см. `Flagged ambiguities`); подъём, поднятие.

**Синтетический биндинг / декларация** *(en: synthetic binding, synthetic declaration)* — биндинг или декларация, отсутствующая в исходнике, но представленная в AST как JS-форма для единообразного анализа.
_Avoid_: fake, virtual.

**Resolve identifier name** *(en: resolve identifier name)* — лукап имени идентификатора по `SymbolId`/`ReferenceId` через `ComponentSemantics`; единственный валидный способ.
_Avoid_: parallel name stack, ad-hoc name tracking, имя в visitor-state.

### Семантика и анализ

**Семантика** *(en: semantics; кластеры `*Semantics` per Svelte unit)* — доменная интерпретация конкретной единицы Svelte, готовая для одного match потребителя.
_Avoid_: метаданные, analysis, семантический анализ.

**Сырые факты** *(en: raw facts)* — атомарные признаки на узле (`has_call`, `has_rune_call`, `is_legacy_wrap`, …), требующие сборки `&&`-цепочкой.
_Avoid_: сырые данные, сырой анализ, raw data.

**`ComponentSemantics`** — центральное хранилище семантики компонента: scope-tree, `SymbolTable`, `ReferenceTable`, side-tables.

### Реактивность

**Реактивность** *(en: reactivity; кластер `ReactivitySemantics`)* — механизм автоматического обновления значений и UI при изменении источников.
_Avoid_: реактивное обновление; signal-based reactivity (узко).

**Руна** *(en: rune; `RuneKind`)* — спецзывов в runes-mode из набора `$state`/`$derived`/`$effect`/`$props`/`$bindable`/`$inspect`/`$host`.
_Avoid_: magic identifier, `$rune`.

**Сигнал** *(en: signal; `SignalReferenceKind`)* — рантайм-примитив реактивности (`$.state` / `$.derived` / `$.user_effect` / `$.template_effect`), в который компилируется руна.
_Avoid_: руна, observable, reactive value.

**Стор** *(en: store)* — observable-стиль реактивный примитив (`subscribe(fn)` + `set(value)` у writable) из `svelte/store`, с сахаром автоподписки `$store`.
_Avoid_: subject, observable (Rx-термин), сигнал.

**Потенциально реактивный биндинг** *(en: maybe-reactive binding; `BindingSemantics::MaybeReactive`)* — категория **биндинга**, истинная реактивность которого недоступна анализу из-за модульной границы: импортированное значение может быть руной/сигналом из `.svelte.js`-модуля или другого компонента, а может быть plain JS-значением. Преимущественный источник — `import`-символы (анализ помечает их `record_maybe_reactive_symbol` консервативно), множество открыто. Потребители (`expression_semantics`, `block_semantics`, dynamism, кодген) трактуют **ссылку** на такой биндинг как реактивную: выражение мемоизируется через `$.derived`, узел шаблона маркируется динамическим, в кодгене импорт читается напрямую идентификатором.
_Avoid_: реактивный импорт, неизвестная реактивность, опционально реактивный, одиночное «MaybeReactive» в проектной речи.

**Мемоизация** *(en: memoization, memoize; кластер `Memoization` в `ExpressionSemantics`; `Memoizer`/`TemplateMemoState` в кодгене)* — автоматическая подмена «дорогого» template-выражения (вызов функции, `await`, опционально чтение **сигнала**) единой реактивной ячейкой, которая вычисляется один раз за обновление рантайма и читается во всех точках шаблона, где это выражение встречалось. Sync-форма — `$.derived` (`$.derived_safe_equal` в SoftLegacy/HardLegacy), читается через `$.get($N)`; async-форма (выражение содержит `await`) — слот в массиве `async_values` `$.template_effect`/`$.deferred_template_effect`.
_Avoid_: кеширование, cache, expression hoisting.

### Режимы и legacy

**Mode** *(en: mode)* — режим компиляции компонента: `runes` / `legacy` / `auto` (см. `specs/auto-mode-detection.md`).
_Avoid_: «классический режим», «новый режим».

**Legacy** *(en: legacy; маркер `LEGACY(svelte4)`)* — совокупность синтаксиса и поведения Svelte 4: deprecated в Svelte 5, подлежит удалению в Svelte 6.
_Avoid_: deprecated (статус, а не категория), Svelte 4-стиль.

**Модуль** *(en: module)* — двоякий термин: `<script module>`-блок или standalone `.svelte.js`/`.svelte.ts`-файл (см. `Flagged ambiguities`).
_Avoid_: одиночное «модуль» / «module» без квалификатора.

## Relationships

- **`.svelte`-файл** = **скрипт** + **шаблон** + **стиль**; pipeline: **парсер** → **анализ** → **трансформ** → **кодген**.
- **Шаблон** содержит **фрагменты**; фрагмент — последовательность **элементов**, **блоков**, **@-тегов**, текста и выражений.
- **Блок** содержит один или несколько **фрагментов** (например, `{#if}`-ветки `then`/`else`).
- **Элемент** содержит **атрибуты** и/или **директивы**.
- **Сниппет** заменяет **слот** в Svelte 5; сниппет — это блок-объявление, `{@render}` — @-тег-вызов.
- **Декларация** содержит один или несколько **деклараторов**; каждый декларатор через свой `BindingPattern` заводит один или несколько **биндингов**.
- **Ссылка** резолвится анализом в **биндинг** (`SymbolId`); цепочка: ссылка → биндинг → декларатор → декларация.
- **Скоуп** держит **биндинги** и ссылается на родителя; **shadowing** — следствие резолвинга по этой цепочке.
- Дерево скоупов едино для **скрипта** и **шаблона**: scope-узлы шаблона — потомки `instance_scope_id`, тот — `module_scope_id`.
- **Анализ** заполняет **`ComponentSemantics`** и `*Semantics`-кластеры; **трансформ** и **кодген** их только читают.
- **Семантика** и **сырые факты** — две формы хранения данных анализа; целевая форма — только семантика (см. `specs/analyzer-target-design.md`).
- **Реактивность** имеет два рода: **сигнальная** (через **руны** → **сигналы** рантайма) и **legacy** (let-promotion + `$:` + `$store` в non-runes mode); **стор** — третий вид (observable-style), параллельный обоим.
- **Руна** = синтаксис в исходнике; **сигнал** = объект в рантайме, в который руна компилируется.
- **Mode** влияет на всё: легальный синтаксис, классификацию **реактивности**, выбор runtime-функций кодгеном.
- **Потенциально реактивный биндинг** — отдельная ось от **сигнальной** и **legacy**-реактивности: маркер на биндинге «истинная реактивность скрыта модульной границей», ставится анализом для `import`-символов и пробрасывает их в реактивные пути потребителей (динамизм, мемоизация, кодген).
- **Мемоизация** = синтетический **сигнал** `$.derived`, эмитимый кодгеном для template-выражения без явной **руны**; по форме неотличимо от руки написанного `$derived`.

## Flagged ambiguities

- **`reference/compiler/`** — путь оставлен; в проектной речи компилятор Svelte 5 — **Оригинал**, не «reference».
- **`original`** — имя **Оригинал** (JS-компилятор) приоритетнее общего смысла «исходный X до изменений». Для второго смысла предпочесть `source` / `pre-rewrite` / `prior` / `earlier draft`. Исключение — frozen-строки диагностик, скопированные дословно с Оригинала ради парити.
- **`scope`** — лексический **скоуп** (JS-семантика) vs CSS-scoping (хеширование классов в `svelte_transform_css`); без квалификатора подразумевается лексический.
- **`template`** — Svelte-**шаблон** (markup `.svelte`) vs JS template literal («шаблонная строка», `TemplateLiteral`); для JS-варианта всегда полное словосочетание.
- **`Template`-struct в `svelte_codegen_client`** — собранная HTML-строка для эмита, не сам **шаблон**.
- **`module`** — **module-script** (`<script module>`/`<script context="module">`-блок внутри `.svelte`, код один раз на импорт) vs standalone **`.svelte.js`/`.svelte.ts`-модуль** (отдельный файл с поддержкой рун, компилируется `compile_module`/`generate_module`).
- **`tag`** — **@-тег** (`{@...}`, в AST `*Tag`) vs HTML-маркер `<...>` (это **элемент**, не тег); в проектной речи всегда конкретно.
- **`shadow`** — **shadowing** в скоупах vs Shadow DOM в custom-elements (`CeShadowMode`).
- **«синтетический биндинг»** — два источника: парсер (`{@const ...}` представлен как JS-декларация) и анализ (store-sub `$count` для store `count` в legacy-режиме); оба легитимны, контекст уточняется по фазе.
- **«фрагмент»** — наш **Фрагмент** (`Fragment`-узел) vs `<svelte:fragment slot="…">` в legacy-слотах (это **элемент** под именем `SvelteFragment`).
- **«компонент»** — единица компиляции `.svelte`-файла (определение) vs use-site в шаблоне `<MyButton />` или `<svelte:component this={…} />` (AST-узел `Component`, категория **элемента**); в проектной речи при риске смешения уточнять «компонент-определение» / «компонент-вызов».
- **«пропс»** — сторона объявления (`$props()` или legacy `export let X` в самом компоненте) vs сторона передачи (любой **атрибут** на `<Component>`-элементе в шаблоне потребителя; в `AttributeSemantics`/кодгене «prop» употребляется именно в этом смысле).
- **«binding» / «биндинг»** — наш **биндинг** (`Symbol`/`SymbolId`, scope-binding) vs семантика **`bind:`-директивы** (data-binding, двусторонняя связь UI ↔ state); в проектной речи всегда конкретно.
- **«render» / «рендер»** — **render**-функция компонента (то, что эмитит кодген) vs `{@render snippet()}` (это **@-тег** вызова сниппета) vs DOM-вставка/обновление в рантайме (это операция, не сущность); в проектной речи всегда конкретно.
- **«hoisting»** — наш **hoisting** (codegen-подъём в `HoistedBucket`) vs JS-семантика hoisting `var`/function-декларации в scope (тесты `var_hoists_to_function_scope`, `function_declaration_hoisting`); это разные механизмы на разных фазах.
- **«мемоизация»** — наша **Мемоизация** (single-slot вынос в `$N`, dedup в пределах одного rerun) vs JS-сообществовая memoization (argument-keyed кэш в духе `lodash.memoize`); ключей и LRU у нас нет.
- **«AST»** — сосуществуют два дерева: **Svelte AST** (template-узлы и метаданные, `crates/svelte_ast`) и **OXC AST** (JS/TS-узлы, `oxc_ast::ast::*`); идентификация узлов — наш `NodeId` vs `OxcNodeId` соответственно. В проектной речи без квалификатора «AST» обычно подразумевается комбинированное дерево; при пересечении доменов писать конкретно.

## Example dialogue

> **Дев:** В кодгене я хочу понять, реактивна ли ссылка на `count` в выражении шаблона. Откуда брать?
> **Анализатор-эксперт:** Через **семантику** — `ReactivitySemantics` несёт `*Semantics`-вариант на каждой **ссылке**. Один match, никаких **сырых фактов** вроде «есть ли вызов руны рядом» — это уже сделано в **анализе**.
>
> **Дев:** А если `count` — **стор**?
> **Анализатор-эксперт:** Тогда `$count` парсится как **ссылка** на (возможно **синтетический**) **биндинг**, и `*Semantics` отметит её как store-read. Кодген эмитит `$.store_get(count, '$count', $$stores)`.
>
> **Дев:** В **сниппете** объявил `count`. Та же ли это **ссылка**, что наружу?
> **Анализатор-эксперт:** Нет — внутри сниппета свой **скоуп**, и там **shadowing** перекрывает внешний биндинг. **Resolve identifier name** через `ComponentSemantics`, не своими «стеками строк», и получишь правильный `SymbolId`.
>
> **Дев:** В **legacy-mode** на `$:`-блок что эмитим?
> **Анализатор-эксперт:** Это **legacy-реактивность**. Анализ promot'ит участвующие top-level `let` в `legacy state`, кодген оборачивает их в `$.mutable_source(...)`. Никаких **рун** и **сигналов** — другая ось **реактивности**.
