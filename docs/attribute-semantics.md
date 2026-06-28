# PRD: AttributeSemantics (корневой)

label: attribute-semantics
topics: attribute, directive, bind/BindDirective, on/use/class/style/transition/animate, spread/rest, ComponentBind, prop passing

Корневой PRD для модуля `svelte_analyze::attribute_semantics` (3.A.4).
Дочерний по слою: `analyze.md`. Зависит от `ComponentSemantics`, `ReactivitySemantics` (фаза 1), `ComponentScoping`, `ExpressionSemantics`.

## Назначение

Единый ответ кодгену на один атрибут / директиву: какую runtime-форму он эмитит. На один attribute-`NodeId` потребитель получает один `AttributeSemantics`-вариант.

## Public API

- `AttributeSemanticsStore::get(NodeId) -> &AttributeSemantics` — тотальный. Out-of-range / non-attribute id'ы коллапсят в `&AttributeSemantics::NonSpecial`. Без `Option`.

## Варианты

- `ElementBind(ElementBindSemantics)` — `bind:` на обычном элементе.
- `WindowBind` / `DocumentBind` — `bind:` на `<svelte:window>` / `<svelte:document>`.
- `ComponentBind(ComponentBindSemantics)` — `bind:` на компоненте.
- `Event(EventSemantics)` — `on:` / event-атрибут.
- `ComponentProp(ComponentPropSemantics)` — prop в дочерний компонент.
- `ComponentSpread(ComponentSpreadSemantics)` — `{...spread}` на компоненте.
- `ComponentAttach(ComponentAttachSemantics)` — `{@attach …}`.
- `BoundaryProp(BoundaryPropSemantics)` — prop на `<svelte:boundary>`; несёт `Volatility` значения, не emit-форму.
- `HtmlConcat(HtmlConcatSemantics)` — concat-атрибут на обычном элементе.
- `StyleDirectives(StyleDirectivesSemantics)` — агрегат style-директив элемента
- `NonSpecial` — default.

Вариант несёт доменный вердикт; кодген ветвится на варианте и выбирает форму, не комбинирует. Ряд существующих вариантов пока несёт выбранную emit-форму (`EventEmit`, `ComponentSpreadEmit`, `ComponentAttachEmit`, `ConcatPartEmit`, `ComponentPropMemo`, …) — это долг по §«Codegen-агностичность анализа», не норма.

## Архитектурные инварианты

1. **Одна запись на атрибут / директиву `NodeId`.**
2. **Вариант несёт доменный вердикт, не форму печати.** Кодген ветвится на варианте и выбирает форму. Имя варианта/поля не называет форму печати или runtime-вызов — это связало бы анализ с кодгеном (см. §«Codegen-агностичность анализа»). Где форму определяет доменный вердикт (например `Volatility` — реактивность значения), на варианте лежит он.
3. **Read-only после build.**

## Связь с другими документами

- `context.md` §«Компонент, шаблон и его узлы» (атрибут vs директива), §«Codegen-агностичность анализа» (вариант несёт доменный вердикт, форму выбирает кодген — без исключений) и §«Эмит-форма семантики» (нарушение — анти-паттерн).
- `analyze.md` — место в build order.
- `expression-semantics.md` — источник фактов для значений атрибутов.
- `codegen.md` — единственный потребитель.
