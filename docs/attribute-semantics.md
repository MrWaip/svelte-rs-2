# PRD: AttributeSemantics (корневой)

label: attribute-semantics

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
- `BoundaryProp(BoundaryPropSemantics)` — prop на `<svelte:boundary>`.
- `HtmlConcat(HtmlConcatSemantics)` — concat-атрибут на обычном элементе.
- `NonSpecial` — default.

Каждый вариант несёт выбранную emit-форму (`EventEmit`, `ComponentSpreadEmit`, `ComponentAttachEmit`, `ConcatPartEmit`, `ComponentPropMemo`, `HtmlConcatPart`, `TemplateEffect`, `ElementBindPropertyKind`, …). Кодген читает, не комбинирует.

## Архитектурные инварианты

1. **Одна запись на атрибут / директиву `NodeId`.**
2. **Пред-вычисленная emit-форма живёт на варианте.** Кодген ветвится на варианте и печатает.
3. **Read-only после build.**

## Связь с другими документами

- `context.md` §«Компонент, шаблон и его узлы» (атрибут vs директива), §«Эмит-форма семантики» (граница: emit-форма допустима только тут как выбранная runtime-форма на варианте, не как поле «как эмитить» без доменного смысла).
- `analyze.md` — место в build order.
- `expression-semantics.md` — источник фактов для значений атрибутов.
- `codegen.md` — единственный потребитель.
