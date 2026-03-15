# Codebase Map — svelte-rs-2

Rust компилятор Svelte v5. Компилирует `.svelte` → client-side JS.

## Pipeline

```
source: &str
  → svelte_parser::Parser → Component (AST)
  → svelte_analyze::analyze → (AnalysisData, Vec<Diagnostic>)
  → svelte_codegen_client::generate → String (JS)
```

Entry point: `svelte_compiler::compile(source: &str) -> Result<CompileResult, Diagnostic>`

---

## Crates

### `svelte_span`
`crates/svelte_span/src/lib.rs`

```rust
struct Span { start: u32, end: u32 }
trait GetSpan { fn span(&self) -> Span; }
// Span::source_text<'a>(&self, source: &'a str) -> &'a str
// Span::merge(&self, other: &Self) -> Self
```

---

### `svelte_diagnostics`
`crates/svelte_diagnostics/src/lib.rs`

```rust
struct Diagnostic { kind: DiagnosticKind, span: Span, severity: Severity }
// Diagnostic::error(kind, span) / ::invalid_expression(span) / etc.
// Diagnostic::as_err<T>(self) -> Result<T, Diagnostic>
struct LineIndex  // byte offset → (line, col)
```

DiagnosticKind варианты: `UnexpectedEndOfFile | InvalidTagName | UnterminatedStartTag | InvalidAttributeName | UnexpectedToken | UnexpectedKeyword | NoElementToClose | UnclosedNode | InvalidExpression | NoIfBlockToClose | NoIfBlockForElse | OnlyOneTopLevelScript | UnknownDirective | NoEachBlockToClose`

---

### `svelte_ast`
`crates/svelte_ast/src/lib.rs`

```rust
struct NodeId(pub u32)  // уникальный id каждого узла

struct Component {
    fragment: Fragment,
    script: Option<Script>,
    css: Option<RawBlock>,
    source: String,
}
// component.source_text(span: Span) -> &str

struct Fragment { nodes: Vec<Node> }

enum Node {
    Text(Text),               // id, span
    Element(Element),         // id, span, name, self_closing, attributes, fragment, kind
    Comment(Comment),         // id, span
    ExpressionTag(ExprTag),   // id, span, expression_span
    IfBlock(IfBlock),         // id, span, test_span, elseif, consequent, alternate: Option<Fragment>
    EachBlock(EachBlock),     // id, span, expression_span, context_span, index_span?, key_span?, body, fallback?
}

enum Attribute {
    StringAttribute(StringAttribute),             // name, value_span
    ExpressionAttribute(ExpressionAttribute),     // name, expression_span, shorthand
    BooleanAttribute(BooleanAttribute),           // name
    ConcatenationAttribute(ConcatenationAttribute), // name, parts: Vec<ConcatPart>
    ShorthandOrSpread(ShorthandOrSpread),         // expression_span, is_spread
    ClassDirective(ClassDirective),               // name, expression_span?, shorthand
    StyleDirective(StyleDirective),               // name, value: StyleDirectiveValue, important
    BindDirective(BindDirective),                 // name, expression_span?, shorthand
}

enum ConcatPart { Static(String), Dynamic(Span) }
enum StyleDirectiveValue { Shorthand, Expression(Span), String(String), Concatenation(Vec<ConcatPart>) }
enum ElementKind { Unknown, Input }
enum ScriptContext { Default, Module }
enum ScriptLanguage { JavaScript, TypeScript }

struct Script { id, span, content_span, context, language }
struct RawBlock { span, content_span }
struct NodeIdAllocator  // используется только внутри парсера
```

---

### `svelte_js`
`crates/svelte_js/src/lib.rs`
OXC facade — все OXC lifetime'ы замкнуты внутри функций.

```rust
// Публичный API
fn analyze_expression(source: &str, offset: u32) -> Result<ExpressionInfo, Diagnostic>
fn analyze_script_with_scoping(source, offset, typescript) -> Result<(ScriptInfo, oxc_semantic::Scoping), Vec<Diagnostic>>

struct ExpressionInfo {
    kind: ExpressionKind,
    references: Vec<Reference>,
    has_side_effects: bool,
}
enum ExpressionKind {
    Identifier(String), Literal, CallExpression { callee: String },
    MemberExpression, ArrowFunction, Assignment, Other,
}
struct Reference { name: String, span: Span, flags: ReferenceFlags }
enum ReferenceFlags { Read, Write, ReadWrite }

struct ScriptInfo { declarations: Vec<DeclarationInfo>, props_declaration: Option<PropsDeclaration> }
struct DeclarationInfo { name, span, kind: DeclarationKind, init_span?, is_rune: Option<RuneKind> }
enum DeclarationKind { Let, Const, Var, Function }
enum RuneKind { State, Derived, Effect, Props, Bindable, Inspect, Host }
```

---

### `svelte_parser`
`crates/svelte_parser/src/lib.rs`

```rust
Parser::new(source: &str).parse() -> (Component, Vec<Diagnostic>)
```

Внутри: `scanner/mod.rs` + `scanner/token.rs`.

---

### `svelte_analyze`
`crates/svelte_analyze/src/`

```rust
// Публичный API
fn analyze(component: &Component) -> (AnalysisData, Vec<Diagnostic>)

// Re-exports из data.rs:
pub use data::{AnalysisData, ConcatPart, ContentType, FragmentItem, FragmentKey, LoweredFragment, PropAnalysis, PropsAnalysis};
```

**9 passes** (порядок важен):
1. `parse_js` — парсит JS-выражения → `expressions`, `attr_expressions`, `script`; builds `ComponentScoping` via single OXC parse
2. `build_scoping` — marks runes in scoping, walks template to add each-block child scopes
3. `known_values` — const-декларации с литеральным init → `known_values`
4. `mutations` — template assignments + bind directives → `mutated_runes`, `bind_mutated_runes`
5. `lower` — trim whitespace, группирует Text+ExprTag → `lowered_fragments`
6. `reactivity` — scope-aware binding resolution → `dynamic_nodes`, `dynamic_attrs`, `node_needs_ref`
7. `content_types` — классификация по lowered items → `content_types`
8. `elseif` — определяет alternate-фрагменты с единственным elseif → `alt_is_elseif`
9. `validate` — семантические проверки

**Scope system** (`scope.rs`):
```rust
struct ComponentScoping {
    scoping: oxc_semantic::Scoping,       // unified scope tree (script + template)
    runes: HashMap<SymbolId, RuneKind>,   // which symbols are runes
    bind_mutated: HashSet<SymbolId>,      // symbols mutated via bind:
    node_scopes: HashMap<NodeId, ScopeId>, // each-block NodeId → child ScopeId
}
// from_scoping(Scoping) / empty()
// add_child_scope(parent) -> ScopeId
// add_binding(scope, name) -> SymbolId
// find_binding(scope, name) -> Option<SymbolId>  — walks parent chain
// mark_rune(id, kind) / is_rune(id) / rune_kind(id)
// mark_symbol_mutated(id) / symbol_is_mutated(id)
// is_dynamic_ref(scope, name) -> bool  — true if rune or non-root binding
// node_scope(NodeId) -> Option<ScopeId>
// rune_names() / mutated_rune_names() / bind_mutated_rune_names()
```

**Ключевые типы** (`data.rs`):
```rust
enum FragmentKey { Root, Element(NodeId), IfConsequent(NodeId), IfAlternate(NodeId), EachBody(NodeId), EachFallback(NodeId) }

enum FragmentItem {
    Element(NodeId),
    IfBlock(NodeId),
    EachBlock(NodeId),
    TextConcat { parts: Vec<ConcatPart> },
}
enum ConcatPart { Text(String), Expr(NodeId) }  // NB: другой ConcatPart чем в svelte_ast!

enum ContentType { Empty, StaticText, DynamicText, SingleElement, SingleBlock, Mixed }

struct AnalysisData {
    lowered_fragments: HashMap<FragmentKey, LoweredFragment>,
    expressions: HashMap<NodeId, ExpressionInfo>,
    attr_expressions: HashMap<(NodeId, usize), ExpressionInfo>,  // (element_id, attr_index)
    script: Option<ScriptInfo>,
    scoping: ComponentScoping,              // unified scope tree (oxc-based)
    dynamic_attrs: HashSet<(NodeId, usize)>,
    dynamic_nodes: HashSet<NodeId>,
    node_needs_ref: HashSet<NodeId>,
    content_types: HashMap<FragmentKey, ContentType>,
    known_values: HashMap<String, String>,  // compile-time known const values
    alt_is_elseif: HashSet<NodeId>,         // IfBlock'и, чей alternate — единственный elseif
    props: Option<PropsAnalysis>,
    // Cached sets for codegen (populated by cache_rune_sets()):
    rune_names: HashSet<String>,
    mutated_runes: HashSet<String>,
    bind_mutated_runes: HashSet<String>,
}

// Методы:
// data.is_rune(name) -> bool
// data.is_mutable_rune(name) -> bool
// data.rune_kind(name) -> Option<RuneKind>
// data.cache_rune_sets()  — call after build_scoping + detect_mutations
```

---

### `svelte_codegen_client`
`crates/svelte_codegen_client/src/`

```rust
// Публичный API (lib.rs)
fn generate(component: &Component, analysis: &AnalysisData) -> String
// Внутри: OXC Allocator + AstBuilder → Codegen::default().build(&program).code
```

**Модули:**

`context.rs` — `Ctx<'a>` (центральный контекст):
- `b: Builder<'a>` — обёртка над OXC AstBuilder
- `component: &'a Component`
- `analysis: &'a AnalysisData`
- `module_hoisted: Vec<Statement<'a>>` — template-объявления из вложенных фрагментов
- `needs_binding_group: bool` — флаг для генерации `binding_group`
- `gen_ident(prefix)` — генерирует уникальные имена (`text`, `text_1`, …)
- `element(id) / if_block(id) / each_block(id) / expr_span(id)` — O(1) lookup по NodeId

`builder.rs` — `Builder<'a>`:
Враппер над OXC `AstBuilder`. Методы: `bid/rid/rid_expr`, `bool_expr/num_expr/str_expr`, `call/call_expr/call_stmt`, `var_stmt/let_stmt_init/const_stmt`, `block_stmt/if_stmt/assign_stmt`, `params/no_params/arrow/arrow_expr/function_decl`, `static_member_expr`, `template_str_expr/template_parts_expr`, `import_all/export_default/program`, `empty_array_expr`, `alloc`.
Аргументы: `enum Arg<'a> { Str(String) | Num(f64) | Ident(&str) | IdentRef(…) | Expr(…) | Arrow(…) | Bool(bool) }`

`rune_transform.rs` — хелперы для трансформации rune-выражений:
- `transform_rune_get(b, name) -> Expression` — `$.get(name)`
- `transform_rune_set(b, name, right, proxy) -> Expression` — `$.set(name, value)`
- `transform_rune_update(b, name, …) -> Expression` — `$.update(name)` / `$.update_pre(name)`

`script.rs` — `gen_script(ctx) -> (imports, body)`:
Трансформации rune через OXC `SemanticBuilder` + `ScriptTransformer`:
- mutated rune `$state(val)` → `$.state(val)`, read → `$.get(name)`, `name = x` → `$.set(name, x)`, `name++` → `$.update(name)`, `++name` → `$.update_pre(name)`
- unmutated rune → inline value (`void 0` если нет аргументов)

`template/mod.rs` — `gen_root_fragment(ctx) -> (hoisted, body)`:
Стратегии по `ContentType`:
- `Empty` → ничего
- `StaticText` → `$.next(); var text = $.text("…"); $.append($$anchor, text)`
- `DynamicText` → `$.next(); var text = $.text(); $.template_effect(() => $.set_text(text, expr)); $.append`
- `SingleElement` → `var root = $.template(\`<div>…</div>\`); var div = root(); …; $.append`
- `SingleBlock` → `var fragment = $.comment(); var node = $.first_child(fragment); $.if/$.each; $.append`
- `Mixed` → `var root = $.template(\`…\`, 1); var fragment = root(); traverse_items; $.append`

`template/element.rs` — генерация элементов (process_element)
`template/attributes.rs` — атрибуты: static (string/boolean) в HTML, dynamic → `$.set_attribute`, bind directives
`template/if_block.rs` — `$.if(anchor, ($$render) => { … })`
`template/each_block.rs` — `$.each(anchor, 16, () => collection, $.index, ($$anchor, item) => { … })`
`template/expression.rs` — генерация JS-выражений из span'ов
`template/html.rs` — построение HTML template strings
`template/traverse.rs` — обход DOM-дерева (`.first_child`, `.sibling`)

---

### `svelte_compiler`
`crates/svelte_compiler/src/lib.rs`

```rust
struct CompileResult { pub js: String }
fn compile(source: &str) -> Result<CompileResult, Diagnostic>
// = parse → analyze → (fatal diag check) → generate
```

---

### `wasm_compiler`
`crates/wasm_compiler/`

WASM-обёртка над `svelte_compiler::compile` для использования из JS.

---

## Dependency graph

```
svelte_span
  ↑
svelte_diagnostics
  ↑
svelte_ast ← svelte_js
  ↑              ↑
svelte_parser  svelte_analyze
  ↑              ↑
         svelte_codegen_client
                 ↑
          svelte_compiler
                 ↑
          wasm_compiler
```

## Ключевые инварианты

- OXC lifetime'ы **никогда** не выходят из `svelte_js` или `svelte_codegen_client`
- `oxc_semantic::Scoping` — owned, lifetime-free (внутри self_cell). Живёт в `ComponentScoping` внутри `AnalysisData`
- Все side tables в `AnalysisData` — owned, без lifetime параметров
- AST хранит `Span` для JS-выражений; codegen re-парсит из source
- `u32` везде где возможно вместо `usize` (NodeId, Span)
- `ConcatPart` в `svelte_ast` и `svelte_analyze` — **разные типы** с одинаковым именем
- Script block парсится **один раз** через `analyze_script_with_scoping()` → `(ScriptInfo, Scoping)`
