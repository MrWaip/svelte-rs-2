# PRD: Compiler entry (корневой)

label: compiler

Корневой PRD для crate `svelte_compiler` — единственного оркестратора пайплайна.
Public entry: `compile(source, &CompileOptions) -> CompileResult`. Module entry: `compile_module(source, &ModuleCompileOptions)`.

## Назначение

Единый оркестратор, связывающий парсер → анализ → трансформ → кодген → CSS-трансформ. Владеет JS-`Allocator` и протягивает `JsAst<'a>` через фазы. Всегда возвращает `CompileResult { js, css, diagnostics }` — никогда не паникует. На провале `js` — `None`, но диагностики текут.

## Пайплайн (component path)

1. `oxc_allocator::Allocator::default()` — владеется тут, живёт до конца кодгена.
2. `svelte_parser::parse_with_js(&alloc, source)` → `(Component, JsAst, Vec<Diagnostic>)`.
3. Применить `CompileOptions` к компоненту (namespace fallback; runes / accessors / immutable / preserve_whitespace против `<svelte:options>`).
4. `svelte_parser::parse_css_block(&component)` → CSS AST.
5. `svelte_analyze::analyze_with_options` → `AnalysisData`, может добавить диагностики.
6. Если нет `Severity::Error` — `svelte_transform` мутирует `JsAst`.
7. `svelte_codegen_client` эмитит client JS из `AnalysisData` + трансформированного `JsAst`.
8. `svelte_transform_css::transform_css_with_usage` переписывает CSS по `CssAnalysis`.

## Архитектурные инварианты

1. **Compiler — единственный владелец `Allocator`.** Фаза-функции его заимствуют; второй `Allocator` посреди пайплайна не аллоцируется.
2. **Сам compiler не производит диагностик** — агрегирует из парсера + анализа и возвращает единый `Vec<Diagnostic>`.
3. **Standalone module path** (`compile_module`) роутится через `analyze_module` и пропускает template/css-шаги.

## Анти-паттерны

- Аллокация второго `Allocator` посреди пайплайна.
- Инлайн логики анализа / трансформа / кодгена в entry-crate.

## Связь с другими документами

- `context.md` §«Слои крэйтов», §«Кросс-каттинг» (standalone-модули, диагностики).
- `parser.md`, `analyze.md`, `transform.md`, `codegen.md` — фазы, которые оркестрирует.
- `supporting-crates.md` — `svelte_transform_css` (CSS-шаг пайплайна).
