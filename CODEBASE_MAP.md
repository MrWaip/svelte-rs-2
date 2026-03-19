# Codebase Map — svelte-rs-2

Rust компилятор Svelte v5. Компилирует `.svelte` → client-side JS.

## Pipeline

```
source: &str
  → svelte_parser::Parser → Component (AST)
  → svelte_analyze::analyze → (AnalysisData, ParsedExprs<'a>, Vec<Diagnostic>)
  → svelte_transform::transform_component → (mutates ParsedExprs in-place)
  → svelte_codegen_client::generate → String (JS)
```

Entry point: `svelte_compiler::compile(source: &str, options: &CompileOptions) -> CompileResult`

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
    Text(Text),                   // id, span
    Element(Element),             // id, span, name, self_closing, attributes, fragment, kind
    ComponentNode(ComponentNode), // id, span, name, attributes, fragment
    Comment(Comment),             // id, span
    ExpressionTag(ExpressionTag), // id, span, expression_span
    IfBlock(IfBlock),             // id, span, test_span, elseif, consequent, alternate: Option<Fragment>
    EachBlock(EachBlock),         // id, span, expression_span, context_span, index_span?, key_span?, body, fallback?
    SnippetBlock(SnippetBlock),   // id, span, params_span?, body
    RenderTag(RenderTag),         // id, span, expression_span
    HtmlTag(HtmlTag),             // id, span, expression_span
    ConstTag(ConstTag),           // id, span, declaration_span
    DebugTag(DebugTag),           // id, span, expression_spans
    KeyBlock(KeyBlock),           // id, span, expression_span, fragment
    SvelteHead(SvelteHead),       // id, span, fragment
    SvelteElement(SvelteElement), // id, span, tag (Tag|Expression), attributes, fragment
    SvelteWindow(SvelteWindow),   // id, span, attributes
    SvelteDocument(SvelteDocument), // id, span, attributes
    SvelteBody(SvelteBody),       // id, span, attributes
    SvelteBoundary(SvelteBoundary), // id, span, attributes, fragment
    AwaitBlock(AwaitBlock),       // id, span, expression_span, pending?, then_block?, catch_block?
    Error(ErrorNode),             // id, span
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
    UseDirective(UseDirective),                   // name, expression_span?
    OnDirectiveLegacy(OnDirectiveLegacy),         // name, expression_span?, modifiers (Svelte 4)
    TransitionDirective(TransitionDirective),     // name, expression_span?, modifiers, direction: TransitionDirection
    AnimateDirective(AnimateDirective),           // name, expression_span?
    AttachTag(AttachTag),                         // expression_span (Svelte 5.29+)
}

enum ConcatPart { Static(String), Dynamic(Span) }
enum StyleDirectiveValue { Shorthand, Expression(Span), String(String), Concatenation(Vec<ConcatPart>) }
enum ElementKind { Unknown, Input }
enum TransitionDirection { Both, In, Out }
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
struct Reference { name: String, span: Span, flags: ReferenceFlags, symbol_id: Option<SymbolId> }
enum ReferenceFlags { Read, Write, ReadWrite }
// symbol_id populated by resolve_references pass in svelte_analyze

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
fn analyze<'a>(alloc: &'a Allocator, component: &Component) -> (AnalysisData, ParsedExprs<'a>, Vec<Diagnostic>)

// Re-exports из data.rs:
pub use data::{
    AnalysisData, LoweredTextPart, ConstTagData, ContentStrategy, DebugTagData, ElementFlags, FragmentData,
    FragmentItem, FragmentKey, LoweredFragment, ParsedExprs, PropAnalysis, PropsAnalysis, SnippetData,
};
pub use ident_gen::IdentGen;
pub use scope::ComponentScoping;
```

**11 passes** (порядок важен, composite walk is 5 visitors):
1. `parse_js` — парсит JS-выражения → `ParsedExprs`, `expressions`, `attr_expressions`, `script`
2. `build_scoping` — строит единое дерево скоупов (script + template) → `ComponentScoping`
3. `resolve_references` — резолвит template-ссылки к SymbolId, регистрирует мутации
4. `store_subscriptions` — определяет `$store` подписки → `store_subscriptions`
5. `known_values` — const-декларации с литеральным init → `known_values`
6. `props` — анализ `$props()` деструктуризации → `props`
7. `lower` — trim whitespace, группирует Text+ExprTag → `fragments.lowered`
8. **composite walk** — `reactivity` + `elseif` + `element_flags` + `hoistable_snippets` (5 visitor'ов за один обход)
9. `classify_and_mark_dynamic` — классификация фрагментов → `fragments.content_types`, `fragments.has_dynamic_children`
10. `needs_var` — элементы, которым нужна DOM-переменная → `element_flags.needs_var`
11. `validate` — семантические проверки

**Scope system** (`scope.rs`):
```rust
struct ComponentScoping { /* oxc-based, lifetime-free */ }
// empty() / root_scope_id() -> ScopeId
// add_child_scope(parent) -> ScopeId
// add_binding(scope, name) -> SymbolId
// find_binding(scope, name) -> Option<SymbolId>  — walks parent chain
// is_rune(sym_id) / rune_kind(sym_id) / rune_info_by_name(name) -> Option<(RuneKind, mutated)>
// is_mutated(sym_id) -> bool
// mark_each_block_var(sym_id) / is_each_block_var(sym_id) -> bool
// node_scope(NodeId) -> Option<ScopeId>
```

**`IdentGen`** (`ident_gen.rs`):
```rust
struct IdentGen { /* counters: HashMap<String, u32> */ }
// gen(prefix: &str) -> String  — "root" → "root", "root_1", "root_2", ...
// Shared between svelte_transform and svelte_codegen_client via &mut IdentGen
```

**Ключевые типы** (`data.rs`):
```rust
// Parsed OXC Expression ASTs — separate from AnalysisData to avoid lifetime propagation
struct ParsedExprs<'a> {
    exprs: FxHashMap<NodeId, Expression<'a>>,                        // template expressions
    attr_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,          // attribute expressions
    concat_part_exprs: FxHashMap<(NodeId, usize, usize), Expression<'a>>, // concat parts
    script_program: Option<Program<'a>>,                             // consumed by codegen
}

enum FragmentKey {
    Root,
    Element(NodeId), ComponentNode(NodeId),
    IfConsequent(NodeId), IfAlternate(NodeId),
    EachBody(NodeId), EachFallback(NodeId),
    SnippetBody(NodeId), KeyBlockBody(NodeId),
    SvelteHeadBody(NodeId),
}

enum FragmentItem {
    Element(NodeId), ComponentNode(NodeId),
    IfBlock(NodeId), EachBlock(NodeId),
    RenderTag(NodeId), HtmlTag(NodeId), KeyBlock(NodeId),
    TextConcat { parts: Vec<LoweredTextPart>, has_expr: bool },
}
// item.is_standalone_expr() -> bool
// item.node_id() -> NodeId  (panic on TextConcat)

enum LoweredTextPart { Text(String), Expr(NodeId) }  // NB: другой тип чем ConcatPart в svelte_ast!

enum ContentStrategy {
    Empty, Static(String), SingleElement(NodeId), SingleBlock(FragmentItem),
    DynamicText, Mixed { has_elements, has_blocks, has_text },
}

// Sub-structs (поля pub(crate), доступ через методы):
struct ElementFlags {
    // has_spread, has_class_directives, static_class, has_style_directives, static_style,
    // needs_input_defaults, needs_var, needs_ref, dynamic_attrs
}
// ElementFlags методы: has_spread(id), has_class_directives(id), has_style_directives(id),
// needs_input_defaults(id), needs_var(id), needs_ref(id), is_dynamic_attr(id, idx),
// static_class(id) -> Option<Span>, static_style(id) -> Option<Span>

struct FragmentData {
    // lowered, content_types, has_dynamic_children
}
// FragmentData методы: content_type(key) -> ContentStrategy, has_dynamic_children(key) -> bool,
// lowered(key) -> Option<&LoweredFragment>

struct SnippetData {
    // params, hoistable
}
// SnippetData методы: params(id) -> Option<&Vec<String>>, is_hoistable(id) -> bool

struct ConstTagData {
    // names: per-node declared names, by_fragment: const tags per fragment,
    // tmp_names: generated tmp vars for destructuring (filled by transform, consumed by codegen)
}
// ConstTagData методы: names(id) -> Option<&Vec<String>>, by_fragment(key) -> Option<&Vec<NodeId>>,
// tmp_name(id) -> Option<&String>, insert_tmp_name(id, name)

struct DebugTagData {
    // by_fragment: debug tags per fragment
}
// DebugTagData методы: by_fragment(key) -> Option<&Vec<NodeId>>

struct BindSemanticsData {
    // mutable_rune_targets, prop_source_nodes, bind_each_context
}
// BindSemanticsData методы: is_mutable_rune_target(id) -> bool,
// is_prop_source(id) -> bool, each_context(id) -> Option<&Vec<String>>

struct AnalysisData {
    // Flat fields:
    expressions: FxHashMap<NodeId, ExpressionInfo>,
    attr_expressions: FxHashMap<(NodeId, usize), ExpressionInfo>,
    script: Option<ScriptInfo>,
    scoping: ComponentScoping,
    dynamic_nodes: FxHashSet<NodeId>,
    known_values: FxHashMap<String, String>,
    alt_is_elseif: FxHashSet<NodeId>,
    props: Option<PropsAnalysis>,
    exports: Vec<ExportInfo>,
    needs_context: bool,
    store_subscriptions: FxHashSet<String>,
    // Sub-structs:
    element_flags: ElementFlags,
    fragments: FragmentData,
    snippets: SnippetData,
    const_tags: ConstTagData,
    debug_tags: DebugTagData,
    bind_semantics: BindSemanticsData,
}

// AnalysisData методы:
// data.is_dynamic(id) -> bool
// data.is_elseif_alt(id) -> bool
// data.expression(id) -> Option<&ExpressionInfo>
// data.attr_expression(id, idx) -> Option<&ExpressionInfo>
// data.known_value(name) -> Option<&String>
// data.is_rune(name) -> bool
// data.is_mutable_rune(name) -> bool
// data.rune_kind(name) -> Option<RuneKind>
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
- Shortcuts для sub-structs: `content_type(key)`, `has_dynamic_children(key)`, `has_spread(id)`,
  `has_class_directives(id)`, `has_style_directives(id)`, `needs_var(id)`, `needs_input_defaults(id)`,
  `is_dynamic_attr(id, idx)`, `static_class(id)`, `static_style(id)`, `is_snippet_hoistable(id)`,
  `const_tag_names(id)`, `const_tags_for_fragment(key)`,
  `is_mutable_rune_target(id)`, `is_prop_source_node(id)`, `bind_each_context(id)`

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

`template/const_tag.rs` — генерация `{@const ...}`:
- Simple: `const name = $.derived(() => expr)`
- Destructured: генерирует tmp var (`$$const_0`), сохраняет в `const_tags.tmp_names` для transform

`template/mod.rs` — `gen_root_fragment(ctx) -> (hoisted, body)`:
Стратегии по `ContentStrategy`:
- `Empty` → ничего
- `Static(text)` → `$.next(); var text = $.text("…"); $.append($$anchor, text)`
- `DynamicText` → `$.next(); var text = $.text(); $.template_effect(() => $.set_text(text, expr)); $.append`
- `SingleElement(id)` → `var root = $.template(\`<div>…</div>\`); var div = root(); …; $.append`
- `SingleBlock(item)` → `var fragment = $.comment(); var node = $.first_child(fragment); $.if/$.each; $.append`
- `Mixed { .. }` → `var root = $.template(\`…\`, 1); var fragment = root(); traverse_items; $.append`

`template/element.rs` — генерация элементов (process_element)
`template/attributes.rs` — атрибуты: static (string/boolean) в HTML, dynamic → `$.set_attribute`, bind directives
`template/if_block.rs` — `$.if(anchor, ($$render) => { … })`
`template/each_block.rs` — `$.each(anchor, 16, () => collection, $.index, ($$anchor, item) => { … })`
`template/expression.rs` — генерация JS-выражений из span'ов
`template/html.rs` — построение HTML template strings
`template/traverse.rs` — обход DOM-дерева (`.first_child`, `.sibling`)
`template/svelte_head.rs` — `<svelte:head>` codegen (`$.head()`)
`template/svelte_element.rs` — `<svelte:element>` codegen (`$.element()`)
`template/svelte_window.rs` — `<svelte:window>` codegen (events, bindings)
`template/svelte_document.rs` — `<svelte:document>` codegen (events, bindings)
`template/svelte_body.rs` — `<svelte:body>` codegen (events, actions)
`template/svelte_boundary.rs` — `<svelte:boundary>` codegen (`$.boundary()`)
`template/title_element.rs` — `<title>` in `<svelte:head>` codegen
`template/await_block.rs` — `{#await}` codegen
`template/debug_tag.rs` — `{@debug}` codegen
`template/key_block.rs` — `{#key}` codegen
`template/html_tag.rs` — `{@html}` codegen
`template/render_tag.rs` — `{@render}` codegen
`template/snippet.rs` — `{#snippet}` codegen
`template/component.rs` — `<Component>` codegen

---

### `svelte_transform`
`crates/svelte_transform/src/`

```rust
// Публичный API
fn transform_component<'a>(
    alloc: &'a Allocator,
    component: &Component,
    analysis: &mut AnalysisData,
    parsed: &mut ParsedExprs<'a>,
    ident_gen: &mut IdentGen,
)
```

Трансформирует expression AST'ы in-place. Вызывается ПОСЛЕ `analyze`, ДО `generate`.

Перезаписывает:
- Rune references → `$.get(name)` / `$.set(name, val)` / `$.update(name)`
- Prop sources → `name()` (thunk call)
- Prop non-sources → `$$props.name`
- Each-block context variables → `$.get(name)`
- Snippet parameters → `name()` (thunk call)
- Destructured const aliases → `$.get(tmp).prop`

---

### `svelte_compiler`
`crates/svelte_compiler/src/lib.rs`

```rust
struct CompileResult { pub js: Option<String>, pub diagnostics: Vec<Diagnostic> }
struct CompileOptions { dev, filename, name, custom_element, namespace, css, runes, ... }
struct ModuleCompileOptions { dev, filename }
fn compile(source: &str, options: &CompileOptions) -> CompileResult
// = parse → analyze → (fatal diag check) → transform → generate
fn compile_module(source: &str, options: &ModuleCompileOptions) -> CompileResult
// = OXC parse → analyze_module → rune transforms → JS output (no template/CSS)
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
         svelte_transform
                 ↑
         svelte_codegen_client
                 ↑
          svelte_compiler
                 ↑
          wasm_compiler
```

## Ключевые инварианты

- OXC Expression AST'ы живут в `ParsedExprs<'a>` (аллокатор принадлежит caller'у). В публичный API `svelte_compiler` они не выходят
- `oxc_semantic::Scoping` — owned, lifetime-free. Живёт в `ComponentScoping` внутри `AnalysisData`
- Все поля `AnalysisData` — owned, без lifetime параметров
- AST хранит `Span` для JS-выражений; `parse_js` парсит их один раз в `ParsedExprs`; `transform` мутирует; `codegen` использует
- `u32` везде где возможно вместо `usize` (NodeId, Span)
- `ConcatPart` (svelte_ast) и `LoweredTextPart` (svelte_analyze) — **разные типы** для аналогичных целей в разных фазах
- Sub-struct поля `ElementFlags`, `FragmentData`, `SnippetData`, `ConstTagData` — `pub(crate)`, снаружи только через методы
