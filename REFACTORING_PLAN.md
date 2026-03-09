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

### Stage 3: Client codegen и интеграция ⏳

**Нужно создать:**
- `crates/svelte_codegen_client/` — client codegen с OXC внутри `generate()`:
  - `lib.rs` — `generate()`, allocator живёт здесь
  - `context.rs` — `ClientContext<'alloc>`: OXC builder, template accumulator
  - `template.rs` — fragment → `$.template()` + DOM traversal
  - `element.rs` — Element → attributes, class directives
  - `expression.rs` — ExpressionTag → `$.set_text()` / template literal
  - `if_block.rs` — IfBlock → `$.if()`
  - `each_block.rs` — EachBlock → `$.each()`
  - `script.rs` — Script → rune declarations, `$.get`/`$.set` wrapping
  - `bindings.rs` — `bind:value`, `bind:checked`, `bind:group`
- `crates/svelte_compiler/` — orchestrator: `compile(source, options) -> CompileResult`

**Файлы-источники:**
- `crates/transform_hir/src/` — текущий codegen (основной)
- `crates/transform_hir/src/script/` — rune transformation
- `crates/ast_builder/src/lib.rs` — OXC builder helpers
- `crates/compiler/src/lib.rs` — текущий orchestrator

**Верификация:**
- Сравнение output нового vs старого pipeline на `tasks/compiler_tests/cases/`
- `cargo test`

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
