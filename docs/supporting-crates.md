# PRD: Поддерживающие крэйты (корневой)

label: supporting-crates
topics: svelte_ast_builder, svelte_emit_builders, svelte_transform_css, svelte_preprocess, css scoping, injected styles, IdentGen, emit builders (store/binding/runes/props/legacy_wrap/server_refs), preprocess, scss, sass, grass, lightningcss, load_paths, css_targets

Корневой PRD для поддерживающих крэйтов: `svelte_ast_builder`, `svelte_emit_builders`, `svelte_transform_css`, `svelte_preprocess`.

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
- **Injected CSS сверяется семантически, не побайтово.** Строка `$$css.code` (`css: injected` / custom element, встраивается в JS) — единственное место, где выходной JS намеренно не байт-парити с Оригиналом: мы минифицируем её в обоих режимах, Оригинал в dev сохраняет исходное форматирование. Парити держится через `test_support::canonicalize_injected_css_in_js` — переминификацию `$$css.code` обеих сторон через lightningcss во всех точках сверки (харнес, quick-check, sweep). Sourcemap injected CSS парити не держит. Не выравнивать под Оригинал — расхождение намеренное; причина и механизм — `adr/0003-injected-css-semantic-compare.md`.

## `svelte_preprocess`

Public entry: компиляция `<style lang="scss|sass">` до разбора CSS. Включается опцией `transform_style` (см. `compiler.md` §«Встроенный препроцессинг»); scss через `grass`, browser targets через `lightningcss`, сплайс региона обратно в исходник через `svelte_sourcemap`.

- **Назначение:** снять препроцессинг стилей с JS-обвязки, оставив вход и выход текстовыми — препроцессор получает строку блока, а не файл.
- **Спецификаторы с явным расширением резолвятся до `grass`.** Для `@use "…/theme.scss"` `grass` load paths игнорирует, поэтому такие спецификаторы переписываются в абсолютные пути заранее — включая резолв через `exports` из `package.json` пакета.
- **Каталог самого компонента — первый load path.** Вход для `grass` — строка, а не файл, поэтому без этого относительный `@import '../styles'` внутри `<style>` не резолвится.
- **Анти-паттерны:** отдавать `grass` спецификатор с расширением, не резолвя его; собирать load paths без каталога компонента; полагаться на кэш стилей в watch-режиме (`cache_styles` — только one-shot, `compiler.md` §«Архитектурные инварианты»).

## Связь с другими документами

- `context.md` §«Кросс-каттинг».
- `ast.md` — `oxc_ast` (что строит `Builder`) и `svelte_css::ast` (`*_override`-поля).
- `transform.md`, `codegen.md` — потребители `Builder` и `emit_builders`.
- `reactivity-semantics.md` — `BindingSemantics` для `$.store_*`-форм.
- `compiler.md` — CSS-шаг пайплайна, §«Встроенный препроцессинг» (опции `transform_style` / `cache_styles`).
- `adr/0003-injected-css-semantic-compare.md` — почему injected CSS сверяется семантически.
