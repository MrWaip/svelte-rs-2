# PRD: Трансформ (корневой, client)

label: transform
topics: transform, client transform, JS AST mutation, dumb transform, transformer passes (runes/state/derived/props/assignments/legacy_reactive), writeback instrumentation

Корневой PRD для слоя клиентского трансформа (`svelte_transform_client`) — backend'а `generate: client`. Серверный аналог — `transform-server.md`.
Догма: **dumb transform** — мутирует JS AST по готовым ответам анализа, новых данных анализа не производит.

## Назначение

Обходит JS-узлы AST и переписывает их. Lowers руны (`$state` / `$derived` / `$props` / `$effect` / …), rune call-sites, реактивные reads/writes, store-сигилы, legacy `$:`-блоки — в runtime-вызовы.

## Inputs / outputs

- Входы: `&AnalysisData`, `JsAst<'a>` (instance + module `oxc::Program`), allocator.
- Выход: мутированные `oxc::Program`-ы, готовые для кодгена.

## Архитектурные инварианты

1. **Мутирует JS AST на месте.** Не производит новых данных анализа.
2. **Один запрос к анализу на один use case достаточен для одного однозначного решения.** Если трансформу нужно комбинировать флаги из нескольких подсистем, чтобы выбрать lowering, — недостающий ответ принадлежит новым полем/вариантом в анализе, не transform-side glue.
3. **Не эмитит диагностик.**
4. **Не пере-walk'ает AST, чтобы переклассифицировать узлы.**

## Reactive reference dispatchers

Все мутации AST, driven by `ReferenceSemantics`, идут через пять централизованных диспетчеров в `crates/svelte_transform_client/src/transformer/rewrites.rs`:

- `dispatch_identifier_read` — identifier reads.
- `dispatch_identifier_assignment` — `=` / `+=` / `&&=` / … на identifier-таргетах.
- `dispatch_identifier_update` — `++` / `--` на identifier-таргетах.
- `dispatch_member_assignment` — assignment на member-таргетах, по `ReferenceSemantics` корня member'а.
- `dispatch_member_update` — update на member-таргетах, по `ReferenceSemantics` корня member'а.

Каждый диспетчер — исчерпывающий `match` по каждому варианту `ReferenceSemantics`. Добавление нового варианта энфорсится компилятором: non-exhaustiveness-ошибки `match` стреляют во всех пяти, заставляя завести read / identifier-write / identifier-update / member-write / member-update нового примитива заранее (или явно пометить no-op). `transform_assignment`, `transform_update`, `template_rewrites::rewrite_template_enter/exit`, runes identifier-traversal зовут только эти диспетчеры — никогда per-kind хелперы напрямую.

## Анти-паттерны

- Пересчёт rune-kind / реактивных фактов инспекцией AST.
- Сшивание смысла из нескольких raw-булеанов через подсистемы.
- Эмит user-facing errors/warnings.

## Связь с другими документами

- `context.md` §«Догмы», §«Анализ в кодгене» (зеркальный анти-паттерн для трансформа).
- `reactivity-semantics.md` — источник `ReferenceSemantics` для диспетчеров.
- `analyze.md` — контракт «один запрос → одно решение».
- `supporting-crates.md` — `svelte_ast_builder` (единственный конструктор oxc-узлов), `svelte_emit_builders` (формы `$.store_*`).
- `compiler.md` — кто зовёт трансформ в пайплайне.
