# PRD: Поддерживающие крэйты (корневой)

label: supporting-crates
topics: svelte_ast_builder, svelte_emit_builders, svelte_transform_css, css scoping, IdentGen, emit builders (store/binding/runes/props/legacy_wrap/server_refs)

Корневой PRD для поддерживающих крэйтов: `svelte_ast_builder`, `svelte_emit_builders`, `svelte_transform_css`.

## `svelte_ast_builder`

Держит `Builder<'a>` плюс `AssignLeft`, `TemplatePart`, `ObjProp`, `Arg`.

- **Назначение:** единая эргономичная поверхность для конструирования `oxc_ast`-узлов из трансформа/кодгена. Кодифицирует частые JS-формы (call, member, template literal, object literal, assignment, declarator, function), чтобы emit-код оставался декларативным.
- **Инварианты:** единственное разрешённое место конструирования `oxc_ast`-узлов вне `oxc_parser`. Трансформ / кодген ОБЯЗАНЫ идти через `Builder`. `Builder` заимствует `Allocator` из compiler entry.
- **Анти-паттерны:** hand-rolling `oxc_allocator::Box::new_in(...)` / `Vec::new_in(...)` / `oxc_ast::ast::*`-конструкторов из трансформа/кодгена; one-off хелпер внутри трансформа/кодгена вместо расширения `Builder`.

## `svelte_emit_builders`

Держит общие builder'ы Svelte runtime-вызовов, используемые трансформом и кодгеном: `build_store_base_read`, `make_store_set`, `make_store_mutate`, `make_store_update`. Модуль `server_refs` — тот же единый источник истины для серверных (SSR) форм чтения ссылок: `$$props`-ссылка → `$$sanitized_props`, `$store`-чтение → `$.store_get($$store_subs ??= {}, "$name", base)`; форма выбирается по вердикту `ReferenceSemantics`. Это per-node форм-билдеры без собственного обхода дерева — обход держит потребитель (серверный трансформ, `transform-server.md`). Серверный base-read плоский там, где клиентский несёт `$.get` — legacy-state на сервере компилируется в обычный `let`.

- **Назначение:** единый источник истины для формы `$.store_*` runtime-вызовов и выражения «прочитать base-биндинг store-автоподписки». Один match на `BindingSemantics(base_symbol)` живёт тут; трансформ и кодген только зовут внутрь.
- **Инварианты:** зависит от `svelte_ast_builder` + `svelte_analyze` (+ `svelte_component_semantics`); обратной зависимости нет. Функции чистые: `(&Builder<'a>, &AnalysisData, …) -> Expression<'a>`. Новые общие runtime-call builder'ы (rune get/set, thunk call, dollar-member) приходят сюда, как только получают второго потребителя.
- **Анти-паттерны:** реимплементация форм `$.get` / `$.store_*` инлайн в трансформе/кодгене; хелперы, зависящие от `ComponentTransformer` / codegen `Ctx`.

## `svelte_transform_css`

Public entry: `transform_css(...)`, `transform_css_with_usage(...)`, `compact_css_for_injection(css)`.

- **Назначение:** применяет CSS-scoping (`svelte-<hash>`-класс), прунит неиспользуемые селекторы, пишет переписанный текст в `*_override`-поля на `svelte_css::ast`. Читает `CssAnalysis` из анализа.
- **Инварианты:** единственный writer `*_override`-полей; оригинальные span'ы + строки идентификаторов остаются нетронуты. Новой классификации селекторов нет — ей владеет анализ.
- **Анти-паттерны:** пере-парсинг CSS / переклассификация селекторов тут; трогание CSS AST-полей кроме `*_override`.

## Связь с другими документами

- `context.md` §«Кросс-каттинг».
- `ast.md` — `oxc_ast` (что строит `Builder`) и `svelte_css::ast` (`*_override`-поля).
- `transform.md`, `codegen.md` — потребители `Builder` и `emit_builders`.
- `reactivity-semantics.md` — `BindingSemantics` для `$.store_*`-форм.
- `compiler.md` — CSS-шаг пайплайна.
