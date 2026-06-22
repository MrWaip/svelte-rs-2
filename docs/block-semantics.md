# PRD: BlockSemantics (корневой)

label: block-semantics
topics: block, if-block, each-block/keyed each, await-block, key-block, snippet, each-item/index, key expression, fragment topology, then/else branches

Корневой PRD для модуля `svelte_analyze::block_semantics` (3.A.5).
Дочерний по слою: `analyze.md`. Зависит от `ComponentSemantics`, `ReactivitySemantics`, `ExpressionSemantics`, плюс AST.

## Назначение

Единый исчерпывающий ответ кодгену: «во что превращается этот template-блок?». На один block-`NodeId` кодген получает один `BlockSemantics`-вариант, несущий каждое решение для эмита runtime-кода — без доп-лукапов, без сборки булеанов.

Читает per-expression факты из `ExpressionSemantics` для collection / callee / promise-выражений (legacy-wrap, async-kind, blockers, references) вместо дублирования классификации. Each / Render / Await / If / Key выводят `async_kind` и legacy-wrap прямо из `ExpressionData`.

## Public API

- `BlockSemanticsStore::get(node_id) -> &BlockSemantics` — тотальный. Out-of-range / non-block id'ы коллапсят в `&BlockSemantics::NonSpecial`. Без `Option`.
- Side-индекс: `block_for_each_index_sym(sym)` / `is_each_index_sym(sym)`.

## Варианты

- `Each(EachBlockSemantics)` — `{#each ...}`.
- `If(IfBlockSemantics)` — `{#if ...}` / `:else if` / `:else`.
- `Await(AwaitBlockSemantics)` — `{#await ...}` / `:then` / `:catch`.
- `Key(KeyBlockSemantics)` — `{#key ...}`.
- `Snippet(SnippetBlockSemantics)` — `{#snippet name(...)}`.
- `Render(RenderTagBlockSemantics)` — `{@render name(...)}`.
- `ConstTag(ConstTagBlockSemantics)` — `{@const ... = ...}`.
- `NonSpecial` — default.

## Архитектурные инварианты

1. **Тотальный API.** Никакого `Option<&BlockSemantics>` на публичной поверхности.
2. **Каждый вариант несёт пред-вычисленную lowering-форму** (flavor, async-kind, item/index/key-стратегия, render-call-форма, await-wrapper-layout). Кодген читает, не комбинирует.
3. **Read-only после build.**

## Связь с другими документами

- `context.md` §«Компонент, шаблон и его узлы» (блок vs @-тег).
- `analyze.md` — место в build order (после фазы 2 `ReactivitySemantics`).
- `expression-semantics.md` — источник per-expression фактов.
- `reactivity-semantics.md` — реактивность ссылок в выражениях блока.
- `codegen.md` — единственный потребитель.
