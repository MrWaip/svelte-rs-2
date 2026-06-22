# PRD: Парсер (корневой)

label: parser
topics: parser/parse/parsing, scanner, svelte-to-ast, handlers, walk_js, js_postprocess, special elements, css parse

Корневой PRD для слоя парсинга: `svelte_parser` (template + JS entry), `svelte_css::parser` (CSS).
Единственное место, превращающее исходный текст в AST. Даунстрим-слои не пере-парсят.

## Назначение

Парсер производит форму дерева + span'ы. Анализ производит смысл (скоупы, символы, ссылки, runes-mode, реактивный граф). Если вопрос требует смотреть на >1 узел вместе — он принадлежит анализу.

## Подсистемы

### Template-парсер

Рукописный сканер по исходнику `.svelte`. Производит `Component` (template-дерево + метаданные скрипта + raw CSS-блок).

- Вход: `&str` исходника `.svelte` + `&Allocator` (форвардится в JS-парсер для встроенного JS).
- Выход: `svelte_ast::Component`.
- Живёт в `svelte_parser::scanner` + `svelte_parser::handlers`. Без внешнего генератора грамматики.

### JS-парсер (`parse_js`)

Единственный мост между исходником `.svelte` и `oxc_parser`. Производит `oxc::Program` / `Expression` / `VariableDeclaration` для `<script module>`, `<script>`, template-выражений, each-block context-паттернов, `{@const}`-деклараций.

- Вход: `&'a Allocator`, исходник `&str`, source type (JS/TS, module/script), offset для коррекции span при парсинге обёрнутых под-строк.
- Выход: `oxc::Program<'a>` / `Expression<'a>` / `VariableDeclaration<'a>` в переданном allocator'е.

### CSS-парсер

Рукописный сканер по содержимому raw-блока `<style>`. Производит `svelte_css::ast::StyleSheet`.

- Вход: `&str` содержимого стиля + базовый offset (чтобы span'ы лежали на исходнике `.svelte`).
- Выход: `StyleSheet` в `svelte_css::ast`.

## Архитектурные инварианты

1. **Чистый синтаксис.** Никакой семантической классификации (scope, ссылки, runes, store-сигилы, prop-kinds, selector-resolution).
2. **`Allocator` всегда передаётся снаружи.** `oxc::Program` переживает вызов парсинга — им владеет анализ. Парсер не владеет `Allocator`.
3. **`OxcNodeId` не биндится в парсере.** Биндинг — в `ComponentSemanticsBuilder`.
4. **Best-effort recovery.** Битые регионы становятся `ErrorNode` со span'ом; парсинг продолжается.
5. **Единственная разрешённая синтез-форма** — `{@const name = expr}` парсится через `parse_js::parse_const_declaration_with_alloc` в реальную `oxc` `VariableDeclaration`, чтобы анализ видел нормальный биндинг. Оборачивание template-level JS-исходника в валидный JS, чтобы `oxc_parser` его съел.
6. **Wrapped sub-parses** (выражения, each-контексты, const-decls) реконструируют точные span'ы через offset-арифметику.
7. **CSS-парсер заполняет только оригинальные поля** (`name`, `prelude`, идентификаторы как `CompactString`); `*_override: Option<String>` остаются `None` — зарезервированы для `supporting-crates.md` §`svelte_transform_css`.

## Анти-паттерны

- Вызов template-парсера / `parse_js` / CSS-парсера из анализа/трансформа/кодгена, чтобы «пере-парсить» фрагмент.
- Hand-writing JS-парсера вместо роутинга через `oxc_parser`.
- Владение `Allocator` внутри парсера.
- Резолвинг селекторов, scoping, pruning на parse-time.

## Связь с другими документами

- `context.md` §«Слои крэйтов».
- `ast.md` — формы дерева, которые парсер производит.
- `analyze.md` — граница парсер ↔ анализ (форма vs смысл).
- `compiler.md` — кто владеет `Allocator` и зовёт парсер.
