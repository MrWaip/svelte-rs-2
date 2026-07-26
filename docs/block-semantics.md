# PRD: BlockSemantics (корневой)

label: block-semantics
topics: block, if-block, each-block/keyed each, await-block, key-block, snippet, each-item/index, key expression, fragment topology, then/else branches, const tag, declaration tag, async-kind

Корневой PRD для модуля `svelte_analyze::block_semantics` (3.A.5).
Дочерний по слою: `analyze.md`. Зависит от `ComponentSemantics`, `ReactivitySemantics`, `ExpressionSemantics`, плюс AST.

## Назначение

Единый исчерпывающий ответ кодгену: «во что превращается этот template-блок?». На один block-`NodeId` кодген получает один `BlockSemantics`-вариант, несущий каждое решение для эмита runtime-кода — без доп-лукапов, без сборки булеанов.

Читает per-expression факты из `ExpressionSemantics` для collection / callee / promise-выражений (legacy-wrap, async-kind, blockers, references) вместо дублирования классификации. Each / Render / Await / If / Key выводят `async_kind` и legacy-wrap прямо из `ExpressionData`.

## Public API

- `BlockSemanticsStore::get(node_id) -> &BlockSemantics` — тотальный. Out-of-range / non-block id'ы коллапсят в `&BlockSemantics::NonSpecial`. Без `Option`.
- Side-индекс: `block_for_each_index_sym(sym)` / `is_each_index_sym(sym)`.
- `fragment_declaration_group(fragment_id) -> &[NodeId]` — тотальный (пустой срез, если группы нет). Упорядоченный по исходнику состав **группы объявлений фрагмента**: const-теги и declaration-теги одного фрагмента, попавшие в общий `$.run` / `$$renderer.run`. Ключ — фрагмент, потому что группа и есть свойство фрагмента, а не отдельного тега.
- `fragment_declaration_group_order() -> &[FragmentId]` — фрагменты с группой в порядке ОТКРЫТИЯ группы во время обхода (то есть в исходном порядке первого члена). Даёт backend'у порядок именования массива promise'ов, не зависящий от того, когда backend доберётся до фрагмента.

## Варианты

- `Each(EachBlockSemantics)` — `{#each ...}`.
- `If(IfBlockSemantics)` — `{#if ...}` / `:else if` / `:else`.
- `Await(AwaitBlockSemantics)` — `{#await ...}` / `:then` / `:catch`.
- `Key(KeyBlockSemantics)` — `{#key ...}`.
- `Snippet(SnippetBlockSemantics)` — `{#snippet name(...)}`.
- `Render(RenderTagBlockSemantics)` — `{@render name(...)}`.
- `ConstTag(ConstTagBlockSemantics)` — `{@const ... = ...}`.
- `DeclarationTag(DeclarationTagBlockSemantics)` — **declaration-тег** `{const}`/`{let}`: несёт `async_kind` инициализатора (`FragmentDeclarationAsyncKind`, общий с `ConstTag`).
- `NonSpecial` — default.

## Архитектурные инварианты

1. **Тотальный API.** Никакого `Option<&BlockSemantics>` на публичной поверхности.
2. **Каждый вариант несёт пред-вычисленную lowering-форму** (flavor, async-kind, item/index/key-стратегия, render-call-форма, await-wrapper-layout). Кодген читает, не комбинирует.
3. **Read-only после build.**
4. **Группа объявлений фрагмента — один вердикт на фрагмент, а не сборка в кодгене.** Состав и порядок группы (`fragment_declaration_group`) рождаются в ТОМ ЖЕ обходе, что и вердикты самих тегов: кадр группы живёт на стеке walker'а, открывается при входе во фрагмент и закрывается при выходе, первое async-объявление открывает группу, каждое последующее объявление того же фрагмента в неё входит. Отдельного прохода по AST под это нет и быть не должно — пасс обходит дерево только внутри себя. Const-теги и declaration-теги делят ОДНУ группу — backend не собирает её из двух корзин и не сортирует заново; он печатает `$.run` по готовому списку.
5. **Блокер на объявление именуется узлом, не слотом.** `FragmentDeclarationAsyncKind::{Awaited,Deferred}` несёт `blockers` (индексы скриптового `$$promises`) и `declaration_blockers` — `NodeId` объявлений ИЗ ДРУГИХ фрагментов, чьё значение читает инициализатор. Анализ отвечает «на кого ждать», backend — «где этот кто-то приземлился» (имя массива + номер thunk'а он раздаёт сам при эмите). Объявления своего фрагмента в `declaration_blockers` не попадают: внутри одной группы порядок обеспечивает сам `run`.

## Связь с другими документами

- `context.md` §«Компонент, шаблон и его узлы» (блок vs @-тег).
- `analyze.md` — место в build order (после фазы 2 `ReactivitySemantics`).
- `expression-semantics.md` — источник per-expression фактов.
- `reactivity-semantics.md` — реактивность ссылок в выражениях блока.
- `codegen.md` — единственный потребитель.
