# PRD: Compiler entry (корневой)

label: compiler
topics: compiler entry, compile/compile_module, pipeline orchestration, CompileOptions, diagnostics aggregation, standalone .svelte.js, hmr

Корневой PRD для crate `svelte_compiler` — единственного оркестратора пайплайна.
Public entry: `compile(source, &CompileOptions) -> CompileResult`. Module entry: `compile_module(source, &ModuleCompileOptions)`.

## Назначение

Единый оркестратор, связывающий парсер → анализ → трансформ → кодген → CSS-трансформ. Владеет JS-`Allocator` и протягивает `JsAst<'a>` через фазы. Всегда возвращает `CompileResult { js, css, diagnostics }` — никогда не паникует. На провале `js` — `None`, но диагностики текут.

## Пайплайн (component path)

1. `oxc_allocator::Allocator::default()` — владеется тут, живёт до конца кодгена.
2. `svelte_parser::parse_with_js(&alloc, source)` → `(Component, JsAst, Vec<Diagnostic>)`.
3. Применить `CompileOptions` к компоненту (namespace fallback; runes / accessors / immutable / preserve_whitespace против `<svelte:options>`).
4. `svelte_parser::parse_css_block(&component)` → CSS AST.
5. `svelte_analyze::analyze_with_options` → `AnalysisData`, может добавить диагностики. Анализ target-агностичен: один прогон валиден для обоих target'ов, диагностики совпадают.
6. Если нет `Severity::Error` — ветвление по `CompileOptions::generate`:
   - `client` (и `false`): `svelte_transform_client` мутирует `JsAst`; `svelte_codegen_client` эмитит client JS.
   - `server`: `svelte_transform_server` мутирует `JsAst`; `svelte_codegen_server` эмитит server JS под `svelte/internal/server`.
7. `svelte_transform_css::transform_css_with_usage` переписывает CSS по `CssAnalysis`.

**Точка ветвления client | server — строго после анализа.** Два backend'а (`transform.md` + `codegen.md` против `transform-server.md` + `codegen-server.md`) потребляют одну и ту же `AnalysisData`; target-специфичных веток и полей в анализе нет (см. `context.md` §«Codegen-агностичность анализа»).

## Архитектурные инварианты

1. **Compiler — единственный владелец `Allocator`.** Фаза-функции его заимствуют; второй `Allocator` посреди пайплайна не аллоцируется.
2. **Сам compiler не производит диагностик** — агрегирует из парсера + анализа и возвращает единый `Vec<Diagnostic>`.
3. **Standalone module path** (`compile_module`) роутится через `analyze_module` и пропускает template/css-шаги. После анализа ветвится по `generate` так же, как component path: `server` → `svelte_transform_server::transform_module` + `svelte_codegen_server::generate_module` (import `svelte/internal/server` + стёртое тело, без render-функции), иначе клиентский `generate_module`.
4. **`hmr` — только backend-опция.** Протягивается в `CodegenOptions` (оба backend'а), но не в `AnalyzeOptions`/`TransformOptions`: анализ и трансформ hmr не наблюдают. Проверка: `grep -ri hmr crates/svelte_analyze crates/svelte_transform_client crates/svelte_transform_server` пуст.

## Анти-паттерны

- Аллокация второго `Allocator` посреди пайплайна.
- Инлайн логики анализа / трансформа / кодгена в entry-crate.

## Релизная сборка

Публикуемый napi-аддон собирается с PGO: `just build-napi-pgo` (инструментированная сборка →
тренировка на корпусе репозитория → `-Cprofile-use`). Даёт −12,5% времени компиляции на корпусе
реального приложения. Инструментировать и оптимизировать нужно одним и тем же вызовом cargo —
иначе профиль не матчится по mangled-именам; причина, замеры и границы — `adr/0007-pgo-release-build.md`.

## Связь с другими документами

- `context.md` §«Слои крэйтов», §«Кросс-каттинг» (standalone-модули, диагностики).
- `parser.md`, `analyze.md`, `transform.md`, `codegen.md` — фазы, которые оркестрирует.
- `supporting-crates.md` — `svelte_transform_css` (CSS-шаг пайплайна).
- `adr/0007-pgo-release-build.md` — почему релизный аддон собирается с PGO.
