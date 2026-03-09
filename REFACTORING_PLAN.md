# Рефакторинг Svelte-RS: План и прогресс

## Прогресс выполнения

### Stage 1: Фундамент — owned AST, парсер, OXC-фасад ✅ ГОТОВ

**Коммит**: `bdf55b0` на ветке `claude/refactor-svelte-v5-1fL93`

**Что сделано:**
- `crates/svelte_ast/` (390 строк) — owned AST типы: Component, Fragment, Node (Text, Element, Comment, ExpressionTag, IfBlock, EachBlock), все виды атрибутов (String, Expression, Boolean, Concatenation, Shorthand/Spread), ClassDirective, BindDirective, Script, RawBlock, NodeIdAllocator. **Без OXC зависимости.**
- `crates/svelte_parser/` (629 + scanner строк) — парсер строит owned AST. Scanner адаптирован из старого. **22 теста, все зелёные.** Без OXC.
- `crates/svelte_js/` (405 строк) — OXC фасад: `analyze_expression()`, `analyze_script()`, rune detection, reference collection. **6 тестов.** OXC lifetime не выходит наружу.
- `crates/svelte_diagnostics/` (141 строк) — Diagnostic, DiagnosticKind, Severity, LineIndex (offset → line/col).

**Исправленный баг:** `if_elseif_else_block` — парсер падал из-за недостающего уровня `children_stack` для alternate в `{:else if}`.

### Stage 2: Анализ с диагностиками ⏳ СЛЕДУЮЩИЙ

**Нужно создать:**
- `crates/svelte_analyze/` — 7 пассов (все read-only по AST, пишут в side tables):
  1. `lower.rs` — whitespace trim + concat → `LoweredFragment`
  2. `parse_js.rs` — `svelte_js::analyze_*()` → `expressions`, `script`
  3. `symbols.rs` — сбор символов, scope resolution
  4. `runes.rs` — детекция `$state`/`$derived`/`$effect`/`$props`
  5. `reactivity.rs` — пометка dynamic nodes/attrs
  6. `content_types.rs` — тип контента (StaticText, DynamicText, SingleElement, Mixed)
  7. `validate.rs` — семантические диагностики

**Файлы-источники для адаптации:**
- `crates/analyze_hir/src/` — текущие анализы
- `crates/analyzer/src/` — текущий AST-анализ
- `crates/hir/src/` — ContentType, flags

**Верификация:**
- parse → analyze для test cases из `tasks/compiler_tests/cases/`
- Rune detection на `rune_handling` тестах
- `lowered_fragments` корректно группирует Text+ExprTag
- `cargo test`

### Stage 3a: Порт логики кодгена ⏳

**Каркас уже есть** в `svelte_codegen_client/` (~2000 строк) и `svelte_compiler/`.

**Задачи:**
1. Починить `svelte_compiler` (`is_error` → `as_err`)
2. Найти пробелы: сравнить текущий `svelte_codegen_client` с `transform_hir` по покрытию фич
3. Дописать недостающее: script rune transforms, bind directives, edge cases
4. `cargo build -p svelte_codegen_client -p svelte_compiler` — без ошибок

**Файлы-источники:**
- `crates/transform_hir/src/` — текущий codegen (основной)
- `crates/transform_hir/src/script/` — rune transformation
- `crates/ast_builder/src/lib.rs` — OXC builder helpers
- `crates/compiler/src/lib.rs` — текущий orchestrator

### Stage 3b: End-to-end ⏳

**Задачи:**
1. Запустить `svelte_compiler::compile()` на нескольких `.svelte` файлах из `tasks/compiler_tests/cases2/`
2. Сравнить вывод с `case-svelte.js` (expected — output официального Svelte)
3. Исправить расхождения до приемлемого совпадения
4. Проверить что OXC lifetime и `&mut` не расползаются за пределы `svelte_js` и `generate()` в `svelte_codegen_client` — публичные API (`analyze_expression`, `analyze_script`, `generate`) возвращают только owned типы без lifetime-параметров

### Stage 3c: Тесты в стиле cases2 ⏳

**Структура**: `case.svelte` (вход) + `case-svelte.js` (expected output от Svelte).
`case-rust.js` — фиксируется как output Rust-компилятора после прохождения тестов.

**Задачи:**
1. Тест-раннер в `svelte_compiler`: читает папки из `tasks/compiler_tests/cases2/`, для каждой `compile(case.svelte)` → сравнивает с `case-svelte.js`
2. Покрыть кейсы: empty, single_text_node, single_element, single_interpolation, element_attributes, state_runes, single_if_block, single_if_else_block, each_block, bind_directives, smoke
3. `cargo test` — все зелёные

Запуск конкретного теста: `cargo test -p compiler_tests <test_name>`

### Stage 3d-1: Text whitespace trimming ⏳
Тесты: `single_text_node`, `single_if_block`, `single_if_else_block`
Проблема: лишние пробелы в строках `$.text(" some long text line ")`

### Stage 3d-2: Dynamic text API ⏳
Тесты: `single_interpolation`, `single_concatenation`
Проблема: генерируем `$.set_text(text, expr)`, ожидается `text.nodeValue = expr`

### Stage 3d-3: else-if codegen ⏳
Тесты: `single_if_else_block`, `generic_root_sequence`
Проблема: alternate должен быть `($$anchor, $$elseif)`, вложенный `$.if` получает `$$anchor`, лишний `, true` в конце внешнего `$.if`, лишнее создание fragment+first_child в else-if ветке

### Stage 3d-4: Whitespace between siblings in templates ⏳
Тесты: `utf8`, `nested_resets`, `bind_directives`
Проблема: пробелы между соседними элементами в template строках убираются, должны сохраняться

### Stage 3d-5: Boolean attribute format + shorthand attr as dynamic ⏳
Тесты: `element_attributes`
Проблема: `visible` → должно быть `visible=""` в шаблоне; `{description}` shorthand не попадает в `$.set_attribute`

### Stage 3d-6: `$.proxy` + TypeScript stripping ⏳
Тесты: `state_runes`
Проблема: объектные `$state({})` / `$state(varRef)` → `$.proxy(...)`, TypeScript type declarations попадают в output

### Stage 3d-7: bind directive codegen ⏳
Тесты: `bind_directives`
Проблема: `$.bind_value`, `$.remove_input_defaults`, `$.state` для rune-переменных

### Stage 3d-8: Element children traversal ⏳
Тесты: `nested_resets`, `elements_childs`
Проблема: `textContent =` для single-expr children, `$.reset`, правильный обход child/sibling

### Stage 3d-9: each block with Mixed content ⏳
Тесты: `each_block`
Проблема: Mixed content type внутри each item body

### Stage 3d-10: smoke (интеграция) ⏳
Тесты: `smoke`, `elements_childs`
После исправления всего выше

---

### Stage 3e: Миграция cases/ → cases2/

Для каждой группы: скопировать `case.svelte` в `cases2/<name>/`, сгенерировать свежий `case-svelte.js` официальным Svelte, добавить тест в `test_v3.rs`, починить codegen до зелёного теста.

**Stage 3e-1: Script & imports** ⏳
Кейсы: `only_script`, `hoist_imports`

**Stage 3e-2: Static content** ⏳
Кейсы: `static_attributes`, `static_interpolation`, `only_compressible_nodes`, `element_without_new_line`

**Stage 3e-3: Rune interactions** ⏳
Кейсы: `access_to_muted_state_rune`, `interpolation_rune_handling`, `attibute_rune_handling`, `rune_update`, `assign_in_template`

**Stage 3e-4: Directives** ⏳
Кейсы: `class_directive`

**Stage 3e-5: Misc / complex** ⏳
Кейсы: `element_with_interpolation`, `big`

**Stage 3e-6: Evaluate potential dupes** ⏳
Кейсы из cases/: `single_text`, `single_element_node`, `if_block`, `if_else` — сравнить с имеющимися cases2/, мигрировать если есть отличия

---

### Stage 4: Очистка и SSR scaffold ⏳

- Удалить старые crates: `ast`, `parser`, `analyzer`, `transformer`, `hir`, `ast_to_hir`, `analyze_hir`, `transform_hir`, `ast_builder`, `compiler`
- Обновить workspace `Cargo.toml`
- Создать `svelte_codegen_ssr` scaffold (SSR = string concat, без OXC)
- Обновить `svelte_wasm`

---

## Архитектура

### Критические выводы

1. **String-based codegen невозможен** — codegen ТРАНСФОРМИРУЕТ пользовательский JS (рекурсивная замена `count` → `$.get(count)` внутри вложенных выражений). Нужен OXC AST visitor.

2. **OXC facade "analyze only" недостаточен** — codegen-у нужно: парсить → мутировать → встраивать → эмитить выражения. Это AST-операция с одним allocator lifetime.

3. **Решение: OXC lifetime содержится в codegen** — parse и analyze фазы lifetime-free, codegen создаёт allocator внутри `generate()`, делает всю OXC-работу, и возвращает `String`.

### Где живут OXC lifetime

```
ФАЗА           │ OXC?  │ Lifetime │ Что делает
────────────────┼───────┼──────────┼──────────────────────────
svelte_parser   │ НЕТ   │ нет      │ Строит owned AST из HTML/Svelte
svelte_js       │ ДА    │ внутри   │ analyze_expression() → owned ExpressionInfo
svelte_analyze  │ через │ нет      │ Заполняет side tables, вызывает svelte_js
                │svelte_│          │
                │  js   │          │
svelte_codegen  │ ДА    │ внутри   │ Allocator в generate(), парсит выражения из
_client         │       │generate()│ source spans, трансформирует ($.get/$.set),
                │       │          │ строит Program, эмитит → String
```

### Pipeline

```
Source (.svelte)
    │
    ▼
┌──────────────┐
│ svelte_parser │  → Component (owned AST, no lifetime, no OXC)
└──────────────┘
    │
    ▼
┌──────────────┐
│svelte_analyze│  → AnalysisResult (owned side tables, no lifetime)
│  7 passes    │     uses svelte_js internally (OXC contained)
│  (all r/o)   │
└──────────────┘
    │
    ├────────────────────┐
    ▼                    ▼
┌────────────────┐  ┌────────────────┐
│codegen_client  │  │codegen_ssr     │  (будущее)
│ OXC LIVES HERE │  │ string concat  │  (SSR не нужен OXC — просто строки)
│(&Component,    │  │(&Component,    │
│ &AnalysisData) │  │ &AnalysisData) │
│→ String        │  │→ String        │
└────────────────┘  └────────────────┘
```

### Ключевые типы данных

#### AST (owned, immutable после парсинга)

```rust
pub struct Component {
    pub fragment: Fragment,
    pub script: Option<Script>,
    pub css: Option<RawBlock>,
    pub source: String,
}

pub enum Node {
    Text(Text), Element(Element), Comment(Comment),
    ExpressionTag(ExpressionTag), IfBlock(IfBlock), EachBlock(EachBlock),
}
// AST хранит Span для JS-выражений → codegen берёт &source[span] и парсит через OXC
```

#### AnalysisData (side tables, owned)

```rust
pub struct AnalysisData {
    pub lowered_fragments: HashMap<NodeId, LoweredFragment>,  // whitespace trim + concat groups
    pub expressions: HashMap<NodeId, ExpressionInfo>,          // parsed JS metadata
    pub script: Option<ScriptInfo>,                            // script declarations + runes
    pub symbols: Vec<SymbolInfo>,                              // all symbols
    pub symbol_by_name: HashMap<String, usize>,
    pub runes: HashMap<usize, RuneKind>,                       // which symbols are runes
    pub dynamic_attrs: HashSet<(NodeId, usize)>,               // attrs referencing runes
    pub dynamic_nodes: HashSet<NodeId>,                        // nodes referencing runes
    pub node_needs_ref: HashSet<NodeId>,                       // elements needing JS variable
    pub content_types: HashMap<NodeId, ContentType>,           // StaticText/DynamicText/SingleElement/Mixed
}

pub enum FragmentItem {
    Single(NodeId),
    TextConcat { parts: Vec<ConcatPart> },
    TrimmedText { node_id: NodeId, trimmed: String },
}
```

#### Codegen (OXC lifetime contained in generate())

```rust
pub fn generate(component: &Component, analysis: &AnalysisData) -> GeneratedCode {
    let allocator = oxc_allocator::Allocator::default();
    let builder = oxc_ast::AstBuilder::new(&allocator);
    let mut ctx = ClientContext::new(&allocator, &builder, component, analysis);
    let program = ctx.build_program();
    let code = oxc_codegen::Codegen::default().build(&program).code;
    GeneratedCode { js: code, source_map: None }
}
// ClientContext<'alloc> — единственная структура с lifetime, ограничен телом generate()
```

### Принципы реализации

1. **AST и AnalysisData — owned, без lifetime, без OXC типов**
2. **OXC lifetime содержится**: в `svelte_js` функциях (анализ) и в `generate()` (codegen)
3. **Span вместо String** для JS-выражений — codegen парсит из source по span
4. **NodeId на каждом узле** — ключ для side tables
5. **Lowering в side tables** — `LoweredFragment` описывает как рендерить children
6. **Диагностики — accumulator** — `Vec<Diagnostic>` через все фазы
7. **Один pipeline** — parse → analyze → codegen
8. **CSS не трогаем** — `RawBlock`
9. **SSR без OXC** — string concatenation, не нужны AST-трансформации выражений
