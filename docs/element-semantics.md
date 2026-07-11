# PRD: ElementSemantics (корневой)

label: element-semantics
topics: element, regular element, svelte:element, svelte:boundary, boundary, failed branch, pending branch, ElementSemantics, element async, async_kind

Кластер `svelte_analyze::element_semantics` — семантика уровня **элемента**: доменная интерпретация узла-`<name>`, которую backend читает одним match'ем. Element-уровневый двойник `BlockSemantics` (тот — про `{#...}`-блоки). Слой: `analyze.md`.

## Назначение

Что этот узел-**элемент** значит на element-уровне — для любого вида: обычного (`<div>` → `RegularElement`), компонента (`Component`), `<svelte:*>`-специального (`element`, `boundary`, `head`, `window`, …), custom element. Несёт его доменную роль и element-уровневые доменные факты, агрегированные из детей узла (приостановка узла из-за async/blocking-зависимостей его атрибутов, …). Один вердикт на узел; *во что* он превратится — `$$renderer.boundary`, `$$renderer.async`, инлайн — решает backend, не анализ. Каждый вид — свой вариант `ElementSemantics`; варианты заводятся по мере надобности, но архитектурно все виды проходят через один вердикт.

## Инварианты

1. **Элемент vs блок — по синтаксису, не по поведению.** Элемент — `<name>`-узел, несёт атрибуты; блок — `{#…}`/`{@…}`-конструкция без атрибутов (`context.md` §«Компонент, шаблон и его узлы»). `<svelte:boundary>` и `<svelte:element>` ведут себя как управляющие конструкции, но по синтаксису (`<>` + атрибуты) это элементы — поэтому их вердикт здесь, а не в `BlockSemantics`.

2. **Вердикт — домен, не форма эмита.** Вариант несёт доменный ответ (какая ветвь boundary — `failed`/`pending` — откуда; какие blocker'ы приостанавливают узел), а не форму runtime-вызова. Выбор `$$renderer.boundary` / `$$renderer.async` и прочих форм — дело backend'а (`context.md` §«Codegen-агностичность анализа»).

## Anti-patterns

- Backend восстанавливает вид или async сам — матчит имя узла (`el.name == "svelte:boundary"`) или пере-обходит атрибуты. Домен рождается здесь, backend только матчит вариант — «Анализ в кодгене».
- Async вынесен во второй стор рядом (`element_async`, `is_async` по id) вместо поля варианта — «Сырые факты».
- Вариант хранит готовый `$$renderer.async(...)` — «Эмит-форма семантики».

## Связь с другими документами

- `context.md` §«Компонент, шаблон и его узлы», §«Принципы», §«Codegen-агностичность анализа».
- `block-semantics.md` — семантика блоков; `BlockSemantics::Snippet` потребляется при эмите boundary-ветвей.
- `codegen-server.md`, `analyze.md`.
