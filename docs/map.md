# Map — карта проекта

Где что лежит. Крэйты по слоям пайплайна, точки входа, файлы с основными структурами. PRD каждого слоя — в одноимённом `*.md` рядом (оглавление в `context.md`).

Пайплайн: **парсер → анализ → трансформ → кодген**, вход через `svelte_compiler`; после анализа ветвление по `generate: client | server` на два зеркальных backend'а. JS/TS внутри `.svelte` — через OXC.

## Точки входа

| Что | Функция | Файл |
| --- | --- | --- |
| Компиляция компонента | `compile` | `svelte_compiler/src/lib.rs:108` |
| Компиляция модуля (`.svelte.js`) | `compile_module` | `svelte_compiler/src/lib.rs:312` |
| Анализ | `analyze` / `analyze_module` | `svelte_analyze/src/lib.rs:126` |
| Трансформ (client) | `transform_component` | `svelte_transform_client/src/lib.rs:52` |
| Трансформ (server) | `transform_component` | `svelte_transform_server/src/lib.rs:1` |
| Кодген (client) | `generate` / `generate_module` | `svelte_codegen_client/src/lib.rs:108` |
| Кодген (server) | `generate` | `svelte_codegen_server/src/lib.rs:7` |
| Node-биндинг | — | `napi_compiler/src/lib.rs` |
| WASM-биндинг | — | `wasm_compiler/src/lib.rs` |

## Крэйты по слоям

**База**
- `svelte_span` — спаны.
- `svelte_ast` — Svelte (template) AST. Корневые типы: `Component`, `Fragment`, `Node` — `svelte_ast/src/lib.rs:236`.
- `svelte_ast_builder` — билдер template-AST (`src/builder/`).
- `svelte_types` — общие типы.

**Парсер** (`parser.md`)
- `svelte_parser` — `.svelte` → template + JS + CSS AST. Вход `src/lib.rs`; handlers `src/handlers.rs`, JS-walk `src/walk_js.rs`, JS-постобработка `src/js_postprocess.rs`, спец-элементы `src/svelte_elements.rs`, сканер `src/scanner/`.
- `svelte_css` — CSS-парсер/AST/принтер: `parser.rs`, `scanner.rs`, `ast.rs`, `printer.rs`, `visit.rs`.

**Анализ** (`analyze.md` + кластерные PRD)
- `svelte_component_semantics` — хранилище семантики. Главное: `ComponentSemantics` (`src/storage.rs:125`), `BindingPattern` (`src/pattern.rs`), `Symbol` (`src/symbol.rs`), `Scope` (`src/scope.rs`), `Reference` (`src/reference.rs`); билдер `src/builder/` (`js_visitor.rs`, `template.rs`).
- `svelte_analyze` — анализ (read-only над AST). Вход `src/lib.rs`. Пассы — `src/passes/` (`build_component_semantics.rs`, `collect_symbols.rs`, `dynamism.rs`, `fragment_topology.rs`, `css_*`, `js_analyze/`, `template_validation/`). Кластеры `*Semantics`: `src/expression_semantics/`, `block_semantics/`, `attribute_semantics/`, `await_semantics/`, `reactivity_semantics/` (+ `legacy_reactive.rs`, `mode_resolution.rs`). Скоупы `src/scope.rs`, валидация `src/validate/`.
  - PRD: `component-semantics.md`, `expression-semantics.md`, `block-semantics.md`, `attribute-semantics.md`, `await-semantics.md`, `reactivity-semantics.md` (дочерний `state-rune.md`). Идентификаторы — `bindings-and-references.md`.

**Трансформ** (client — `transform.md`, server — `transform-server.md`)
- `svelte_transform_client` — мутация JS AST под `svelte/internal/client`. Вход `src/lib.rs`; пассы — `src/transformer/` (`runes.rs`, `state.rs`, `derived.rs`, `props.rs`, `legacy_reactive.rs`, `props_legacy.rs`, `assignments.rs`, `inspect.rs`, …).
- `svelte_transform_server` — мутация JS AST под `svelte/internal/server`: один `VisitMut`-обход, `ServerTransform` в `src/model.rs`, обработчики по конструкциям (`src/state.rs`, `src/effect.rs`). Вход `src/lib.rs`.
- `svelte_transform_css` — трансформ CSS (scoping) — `src/lib.rs`.
- `svelte_emit_builders` — переиспользуемые JS-эмиттеры (`store.rs`, `binding.rs`, `runes.rs`, `legacy_wrap.rs`, `props.rs`).

**Кодген** (client — `codegen.md`, server — `codegen-server.md`)
- `svelte_codegen_client` — печать выходного JS (client). Вход `src/lib.rs`; контекст `src/context.rs`; визиторы — `src/codegen/` (`fragment/`, `blocks/`, `attributes/`, `expr.rs`, `effect.rs`, `hoisted/`, `component_props/`, `async_emit.rs`, `containers/`, `data_structures/`). Custom elements — `src/custom_element.rs`.
- `svelte_codegen_server` — печать выходного JS (server): рендер-функция под `$$renderer`. Вход `src/lib.rs`; `ServerCodegen` `src/model.rs`, аккумулятор и flush в `$$renderer.push` `src/renderer.rs`, сборка программы `src/program.rs`, визит фрагмента `src/fragment.rs`, элементы `src/element.rs`, атрибуты `src/attribute.rs`, текст/интерполяции `src/text.rs`, HTML-эскейп `src/escape.rs`.

**Entry / supporting** (`compiler.md`, `supporting-crates.md`)
- `svelte_compiler` — оркестрация + опции (`src/options.rs`), агрегация диагностик.
- `svelte_diagnostics` — тип `Diagnostic`, коды (`src/codes.rs`), `svelte-ignore` (`src/extract_svelte_ignore.rs`).
- `svelte_sourcemap` — sourcemaps (WIP).

## Тесты и инструменты (`tasks/`)

- `compiler_tests` — e2e-кейсы `cases2/<name>/` (`case.svelte`, `case-rust.js`, `case-svelte.js`, `config.json`); бинарь `ssr_flip_scan` (`just ssr-flip-scan`) — пары, чей серверный выход уже совпадает с референсами.
- `diagnostic_tests` — кейсы `cases/<name>/`.
- `quick_check` — разовая проверка парити (`just quick-check`).
- `benchmark`, `generate_test_cases`, `generate_benchmark`, `test_support`.

Оригинал (JS-референс) — `original/compiler/`.
