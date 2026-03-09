# Codebase Map — svelte-rs-2

Rust компилятор Svelte v5. Компилирует `.svelte` → client-side JS.

> **Два поколения кода.** Крейты без префикса `svelte_` — **v2 (legacy/reference)**. Крейты с префиксом `svelte_` — **v3 (новая архитектура)**, описана ниже в разделе «V3». V2 не удалён — используется как reference implementation.

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
    BindDirective(BindDirective),                 // name, kind, expression_span?, shorthand
}

enum ConcatPart { Static(String), Dynamic(Span) }
enum ElementKind { Unknown, Input }
enum BindDirectiveKind { Unknown, Value, Group, Checked }
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
fn analyze_script(source: &str, offset: u32, typescript: bool) -> Result<ScriptInfo, Vec<Diagnostic>>

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

struct ScriptInfo { declarations: Vec<DeclarationInfo> }
struct DeclarationInfo { name, span, kind: DeclarationKind, init_span?, is_rune: Option<RuneKind> }
enum DeclarationKind { Let, Const, Var, Function }
enum RuneKind { State, Derived, Effect, Props, Bindable, Inspect, Host }
```

---

### `svelte_parser`
`crates/svelte_parser/src/lib.rs`

```rust
Parser::new(source: &str).parse() -> Result<Component, Diagnostic>
```

Внутри: `scanner/mod.rs` + `scanner/token.rs`.

---

### `svelte_analyze`
`crates/svelte_analyze/src/`

```rust
// Публичный API
fn analyze(component: &Component) -> (AnalysisData, Vec<Diagnostic>)

// Re-exports из data.rs:
pub use data::{AnalysisData, ConcatPart, ContentType, FragmentItem, FragmentKey, LoweredFragment, SymbolId, SymbolInfo};
```

**7 passes** (порядок важен):
1. `parse_js` — парсит JS-выражения → `expressions`, `attr_expressions`, `script`
2. `symbols` — из `script.declarations` → `symbols`, `symbol_by_name`
3. `runes` — проверяет `DeclarationInfo.is_rune` → `runes`
4. `lower` — trim whitespace, группирует Text+ExprTag → `lowered_fragments`
5. `reactivity` — ссылается на rune-символы → `dynamic_nodes`, `dynamic_attrs`, `node_needs_ref`
6. `content_types` — классификация по lowered items → `content_types`
7. `validate` — placeholder

**Ключевые типы** (`data.rs`):
```rust
struct SymbolId(pub u32)  // typed index в symbols Vec

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
    symbols: Vec<SymbolInfo>,
    symbol_by_name: HashMap<String, SymbolId>,
    runes: HashMap<SymbolId, RuneKind>,
    dynamic_attrs: HashSet<(NodeId, usize)>,
    dynamic_nodes: HashSet<NodeId>,
    node_needs_ref: HashSet<NodeId>,
    content_types: HashMap<FragmentKey, ContentType>,
}
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
- `mutated_runes: HashSet<String>` — rune-символы, которые присваиваются в скрипте
- `gen_ident(prefix)` — генерирует уникальные имена (`text`, `text_1`, …)
- `is_dynamic_node(id) -> bool`
- `FragmentCtx<'a>` — template/init/update/after_update буферы
- `DomCursor<'a>` — отслеживает DOM-позицию при обходе

`builder.rs` — `Builder<'a>`:
Враппер над OXC `AstBuilder`. Методы: `bid/rid/rid_expr`, `bool_expr/num_expr/str_expr`, `call/call_expr/call_stmt`, `var_stmt/let_stmt_init/const_stmt`, `block_stmt/if_stmt/assign_stmt`, `params/no_params/arrow/arrow_expr/function_decl`, `static_member_expr`, `template_str_expr/template_parts_expr`, `import_all/export_default/program`.
Аргументы: `enum Arg<'a> { Str(String) | Num(f64) | Ident(&str) | IdentRef(…) | Expr(…) | Arrow(…) | Bool(bool) }`

`script.rs` — `gen_script(ctx) -> (imports, body)`:
Трансформации rune:
- mutated rune `$state(val)` → `$.state(val)`, read → `$.get(name)`, `name = x` → `$.set(name, x)`, `name++` → `$.update(name)`, `++name` → `$.update_pre(name)`
- unmutated rune → inline value (`void 0` если нет аргументов)

`template.rs` — `gen_root_fragment(ctx) -> (hoisted, body)`:
Стратегии по `ContentType`:
- `Empty` → ничего
- `StaticText` → `$.next(); var text = $.text("…"); $.append($$anchor, text)`
- `DynamicText` → `$.next(); var text = $.text(); $.template_effect(() => $.set_text(text, expr)); $.append`
- `SingleElement` → `var root = $.template(\`<div>…</div>\`); var div = root(); …; $.append`
- `SingleBlock` → `var fragment = $.comment(); var node = $.first_child(fragment); $.if/$.each; $.append`
- `Mixed` → `var root = $.template(\`…\`, 1); var fragment = root(); traverse_items; $.append`

Атрибуты: static (string/boolean) — только в HTML. Dynamic (`ExpressionAttribute`) → `$.set_attribute(el, name, val)` в update. BindDirective — TODO.

IfBlock → `$.if(anchor, ($$render) => { if (test) $$render(consequent); else $$render(alternate, false); })`
EachBlock → `$.each(anchor, 16, () => collection, $.index, ($$anchor, item) => { … })`

---

### `svelte_compiler`
`crates/svelte_compiler/src/lib.rs`

```rust
struct CompileResult { pub js: String }
fn compile(source: &str) -> Result<CompileResult, Diagnostic>
// = parse → analyze → (fatal diag check) → generate
```

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
```

## Ключевые инварианты (V3)

- OXC lifetime'ы **никогда** не выходят из `svelte_js` или `svelte_codegen_client`
- Все side tables в `AnalysisData` — owned, без lifetime параметров
- AST хранит `Span` для JS-выражений; codegen re-парсит из source
- `u32` везде где возможно вместо `usize` (NodeId, Span, SymbolId)
- `ConcatPart` в `svelte_ast` и `svelte_analyze` — **разные типы** с одинаковым именем

---

---

# V2 (Legacy / Reference)

Старая архитектура. **Читай как reference**, не переиспользуй напрямую.
OXC lifetime'ы проникают во все слои. Используется `RcCell<T>` для совместного владения.

## V2 Pipeline

```
source: &str
  → parser::Parser → ast::Ast<'a>
  → analyzer::Analyzer → AnalyzeResult          # v1 pipeline: compiler.compile()
  → transformer::transform_client → String

  — или —

  → ast_to_hir::AstToHir → AstToHirRet { store: HirStore<'hir> }
  → analyze_hir::AnalyzeHir → HirAnalyses        # v2 pipeline: compiler.compile2()
  → transform_hir::transform_hir → Program<'hir>
  → oxc_codegen::Codegen → String
```

Entry: `compiler::Compiler::compile(source, allocator)` / `compile2(source, allocator)`

---

## V2 Crates

### `ast` (v2 AST)
`crates/ast/src/lib.rs`

Хранит OXC `Expression<'a>` напрямую. Узлы завёрнуты в `RcCell<T>`.

```rust
struct Ast<'a> { template: RcCell<Template<'a>>, script: Option<ScriptTag<'a>> }
struct Template<'a> { nodes: Fragment<'a> }
struct Fragment<'a> { nodes: Vec<Node<'a>>, metadata: Option<FragmentMetadata>, node_id: Option<NodeId> }

enum Node<'a> {
    Element(RcCell<Element<'a>>),
    Comment(RcCell<Comment<'a>>),
    Text(RcCell<Text<'a>>),
    Interpolation(RcCell<Interpolation<'a>>),      // { expr } — хранит Expression<'a> напрямую
    IfBlock(RcCell<IfBlock<'a>>),
    VirtualConcatenation(RcCell<VirtualConcatenation<'a>>),  // Text+Interpolation слиты вместе
    ScriptTag(RcCell<ScriptTag<'a>>),
    EachBlock(RcCell<EachBlock<'a>>),
}

struct Element<'a> { name, span, self_closing, nodes: Vec<Node<'a>>, attributes: Vec<Attribute<'a>>, metadata?, node_id?, kind: ElementKind }
struct Interpolation<'a> { expression: Expression<'a>, span, metadata? }
struct VirtualConcatenation<'a> { parts: Vec<ConcatenationPart<'a>>, span, metadata: InterpolationMetadata }
struct IfBlock<'a> { span, test: Expression<'a>, is_elseif, consequent: Fragment<'a>, alternate: Option<Fragment<'a>> }
struct EachBlock<'a> { span, collection: Expression<'a>, item: Expression<'a>, index?, key?, nodes: Fragment<'a> }
struct Text<'a> { value: &'a str, span }
struct ScriptTag<'a> { program: Program<'a>, span, language: Language }

enum Attribute<'a> {
    StringAttribute / ExpressionAttribute / BooleanAttribute / ConcatenationAttribute
    SpreadAttribute / ClassDirective / BindDirective
}
// ast::ConcatenationPart<'a>: String(&'a str) | Expression(Expression<'a>)
```

---

### `hir` (v2 HIR)
`crates/hir/src/`

Arena-based. Все узлы хранятся в `HirStore` через `IndexVec`. OXC expressions — в `IndexVec<ExpressionId, RefCell<Expression<'hir>>>`.

```rust
// IDs
struct NodeId(usize)       // impl Idx
struct OwnerId(usize)      // impl Idx
struct ExpressionId(usize) // impl Idx

struct HirStore<'hir> {
    owners: IndexVec<OwnerId, OwnerNode<'hir>>,
    expressions: IndexVec<ExpressionId, RefCell<Expression<'hir>>>,
    node_to_owner: HashMap<NodeId, OwnerId>,
    nodes: IndexVec<NodeId, Node<'hir>>,
    program: Program<'hir>,
}
// HirStore::TEMPLATE_OWNER_ID = OwnerId(0)
// HirStore::TEMPLATE_NODE_ID  = NodeId(0)

// store.get_owner(id) / get_node(id) / get_expression(id) / get_expression_mut(id)
// store.first_of(owner_id) / node_to_owner(node_id) / owner_to_node(owner_id)

enum Node<'hir> {
    Text(&'hir Text<'hir>),
    Interpolation(&'hir Interpolation),      // expression_id: ExpressionId
    Element(&'hir Element<'hir>),
    Comment(&'hir Comment<'hir>),
    IfBlock(&'hir IfBlock),                  // test: ExpressionId
    EachBlock(&'hir EachBlock),              // collection/item: ExpressionId
    Script,
    Concatenation(&'hir Concatenation<'hir>), // parts: Vec<ConcatenationPart<'hir>>
    Phantom,
}
// node.contains_expression() / is_text_like() / is_interpolation_like() / owner_id()

enum OwnerNode<'hir> {
    Template(&'hir Template),   // node_ids: Vec<NodeId>
    Element(&'hir Element),     // node_ids: Vec<NodeId>, attributes: AttributeStore
    IfBlock(&'hir IfBlock),     // consequent: Vec<NodeId>, alternate: Option<Vec<NodeId>>
    EachBlock(&'hir EachBlock), // node_ids: Vec<NodeId>
    Phantom,
}
// owner.first() / iter_nodes_rev() / is_require_next() / scope_id()

struct Template { node_ids: Vec<NodeId>, node_id: NodeId, scope_id: Cell<Option<ScopeId>> }
struct Element<'hir> { node_id, owner_id, name: &'hir str, node_ids, self_closing, kind: ElementKind, attributes: AttributeStore<'hir>, scope_id }
struct IfBlock { node_id, owner_id, is_elseif, test: ExpressionId, consequent: Vec<NodeId>, alternate: Option<Vec<NodeId>>, scope_id }
struct EachBlock { node_id, owner_id, node_ids, collection: ExpressionId, item: ExpressionId, index?, key?, scope_id }
struct Interpolation { owner_id, node_id, expression_id: ExpressionId }
struct Concatenation<'hir> { owner_id, node_id, parts: Vec<ConcatenationPart<'hir>> }
enum ConcatenationPart<'hir> { Text(&'hir str), Expression(ExpressionId) }
struct Program<'hir> { language: Language, program: RefCell<oxc_ast::ast::Program<'hir>> }
```

**Атрибуты HIR** (`hir/src/attributes.rs`):
```rust
enum Attribute<'hir> { StringAttribute | ExpressionAttribute | SpreadAttribute | BooleanAttribute | ConcatenationAttribute }
// Directives (только в AnyAttribute): Use | Animation | Bind | On | Transition | Class | Style | Let
struct ExpressionAttribute<'hir> { shorthand, name: &'hir str, expression_id: ExpressionId }
struct BindDirective<'hir> { shorthand, name: &'hir str, expression_id: ExpressionId }
struct ClassDirective<'hir> { shorthand, name: &'hir str, expression_id: ExpressionId }
```

---

### `ast_to_hir`
`crates/ast_to_hir/src/lib.rs`

```rust
struct AstToHir<'hir> { allocator, builder: ast_builder::Builder<'hir> }
struct AstToHirRet<'hir> { store: HirStore<'hir> }

// AstToHir::new(allocator).traverse(ast: Ast<'hir>) -> AstToHirRet<'hir>
```

Внутри: trim_nodes, compress_nodes (Text+Interpolation → Concatenation), clean_comments.

---

### `analyze_hir`
`crates/analyze_hir/src/`

```rust
// Публичный API
struct AnalyzeHir<'hir> { _allocator }
// AnalyzeHir::new(allocator).analyze(&HirStore) -> HirAnalyses

struct HirAnalyses {
    scope: RefCell<ScopeTree>,
    symbols: RefCell<SymbolTable>,
    content_types: HashMap<OwnerId, OwnerContentType>,
    dynamic_nodes: HashSet<NodeId>,          // nodes that are or contain reactive expressions
    runes: HashMap<SymbolId, SvelteRune>,
    expression_flags: HashMap<ExpressionId, ExpressionFlags>,
    identifier_generators: RefCell<…>,
}
// analyses.is_dynamic(&NodeId) / get_content_type(&OwnerId) / get_rune(SymbolId)
// analyses.get_expression_flags(ExpressionId) / generate_ident(prefix)
// analyses.take_scoping() -> (SymbolTable, ScopeTree)
```

**Passes** (в `analyze()`):
1. `oxc_semantic_pass` — OXC SemanticBuilder → SymbolTable + ScopeTree
2. `content_type_pass` → `content_types`
3. `dynamic_markers_pass` → `dynamic_nodes`
4. `script_pass` → `runes` (через OXC Visit)
5. `scope_adding_pass` — добавляет scope_id к owners
6. `rune_reference_pass` → `expression_flags`

**Типы:**
```rust
// bitflags OwnerContentTypeFlags: Text|Interpolation|Concatenation|Element|IfBlock|Comment|EachBlock
enum OwnerContentType {
    Common(OwnerContentTypeFlags),
    IfBlock(OwnerContentTypeFlags, OwnerContentTypeFlags),  // (consequent, alternate)
}
// flags.only_element() / only_text() / any_text_like() / any_interpolation_like() / only_synthetic_node()

// bitflags ExpressionFlags: RuneReference | FunctionCall
// flags.has_rune_reference()

struct SvelteRune { kind: SvelteRuneKind, mutated: bool }
enum SvelteRuneKind { State|StateRaw|StateSnapshot|Props|PropsId|Bindable|Derived|DerivedBy|Effect|EffectPre|EffectTracking|EffectRoot|Inspect|InspectWith|InspectTrace|Host }
```

---

### `transform_hir`
`crates/transform_hir/src/`

```rust
fn transform_hir<'hir>(analyses: &'hir HirAnalyses, store: &'hir mut HirStore<'hir>, b: &'hir Builder<'hir>) -> Program<'hir>
// Внутри: transform_script + transform_template → собирает Program
// script: rune declarations + assignments transform (аналог svelte_codegen_client/script.rs)
// template: nodes/element/fragment/if_block/each_block/interpolation/attributes
```

---

### `transformer` (v1 transformer — использует старый ast без hir)
`crates/transformer/src/`

```rust
fn transform_client<'a>(ast: Ast<'a>, b: &'a Builder<'a>, analyze: AnalyzeResult) -> String
// Принимает v2 ast::Ast и старый AnalyzeResult (из crates/analyzer)
// Используется в compiler.compile() (v1 path)
```

---

### `compiler` (v2 entry point)
`crates/compiler/src/lib.rs`

```rust
struct Compiler {}
// Compiler::new().compile(source, allocator)  → v1 path (ast → transformer)
// Compiler::new().compile2(source, allocator) → v2 path (ast → hir → analyze_hir → transform_hir)
```

---

## V2 Dependency graph

```
span / diagnostics
  ↑
ast ← parser
  ↑          ↑
analyzer   ast_to_hir ← hir
  ↑              ↑
transformer   analyze_hir
  ↑              ↑
           transform_hir
                 ↑
            compiler (обе ветки)
```

## V2 vs V3 — ключевые отличия

| | V2 | V3 |
|---|---|---|
| AST nodes | `RcCell<T>`, lifetime'ы повсюду | owned, span-based |
| JS expressions | `Expression<'a>` в каждом узле | `Span` → re-parse |
| HIR | arena + IndexVec (OwnerId/NodeId/ExpressionId) | нет отдельного HIR |
| Analysis | `HirAnalyses` (symbols+scopes+runes+dynamic) | `AnalysisData` (7 passes) |
| Content type | `OwnerContentTypeFlags` (bitflags) | `ContentType` enum |
| Rune detection | через OXC SymbolId (semantic) | по имени callee |
