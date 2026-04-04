# Codebase Map — svelte-rs-2

Rust компилятор Svelte v5. Компилирует `.svelte` → client-side JS.

## Pipeline

```
source: &str
  → svelte_parser::parse_with_js → (Component, ParserResult, Vec<Diagnostic>)
  → svelte_analyze::analyze → (AnalysisData, ParserResult, Vec<Diagnostic>)
  → svelte_transform::transform_component → (mutates ParserResult in-place)
  → svelte_codegen_client::generate → String (JS)
```

Entry point: `svelte_compiler::compile` / `svelte_compiler::compile_module`

---

## Reference materials

### `reference/compiler/`
Исходники оригинального Svelte 5 JS-компилятора.
- `phases/1-parse/` — парсер (AST, template syntax)
- `phases/2-analyze/` — анализ (visitors, scoping, validation)
- `phases/3-transform/` — codegen (client/server JS output)
- `errors.js`, `warnings.js` — диагностики

### `reference/docs/`
Официальная документация Svelte: `02-runes/`, `03-template-syntax/`, `04-styling/`, `05-special-elements/`, `06-runtime/`, `99-legacy/`.

---

## Crates

### `svelte_span`
`crates/svelte_span/src/lib.rs` — `Span { start: u32, end: u32 }`, `GetSpan` trait.

---

### `svelte_diagnostics`
`crates/svelte_diagnostics/src/lib.rs` — `Diagnostic`, `DiagnosticKind` (~274 variants), `Severity`, `LineIndex`.

Подмодули: `codes.rs` (legacy replacement, fuzzymatch), `extract_svelte_ignore.rs`.

---

### `svelte_ast`
`crates/svelte_ast/src/lib.rs` — Svelte AST types. Immutable после парсинга.

Ключевые типы: `NodeId`, `Component`, `Fragment`, `Script`, `RawBlock`.

**`Node` variants:** Text, Element, ComponentNode, Comment, ExpressionTag, IfBlock, EachBlock, SnippetBlock, RenderTag, HtmlTag, ConstTag, DebugTag, KeyBlock, SvelteHead, SvelteElement, SvelteWindow, SvelteDocument, SvelteBody, SvelteBoundary, AwaitBlock, Error.

**`Attribute` variants:** StringAttribute, ExpressionAttribute, BooleanAttribute, ConcatenationAttribute, Shorthand, SpreadAttribute, ClassDirective, StyleDirective, BindDirective, UseDirective, OnDirectiveLegacy, TransitionDirective, AnimateDirective, AttachTag.

**Enums:** `ConcatPart` (Static/Dynamic), `StyleDirectiveValue`, `TransitionDirection`, `ScriptContext`, `ScriptLanguage`.

---

### `svelte_component_semantics`
`crates/svelte_component_semantics/src/` — единый semantic graph для `.svelte` компонента (module script + instance script + template).

Заменяет `oxc_semantic::Scoping` / `SemanticBuilder` — OXC предоставляет только AST + Visit trait. Использует те же ID-типы (`ScopeId`, `SymbolId`, `ReferenceId`) из `oxc_syntax`.

Ключевые типы: `ComponentSemantics` (source of truth для scopes/symbols/references), `ComponentSemanticsBuilder` (builder, traverses JS via OXC Visit + template via `TemplateWalker` trait), `JsSemanticVisitor` (OXC Visit impl), `TemplateBuildContext`.

Подмодули: `builder/` (builder + JS visitor), `scope.rs`, `symbol.rs`, `storage.rs`, `reference.rs`.

---

### `svelte_parser`
`crates/svelte_parser/src/lib.rs` — парсер + JS pre-parsing.

Shared domain types в `types.rs`: `ScriptInfo`, `DeclarationInfo`, `DeclarationKind`, `RuneKind`, `ParserResult` (instance/module OXC Programs + template expressions/statements keyed by span offset).

Подмодули: `scanner/`, `parse_js.rs`, `html.rs` (HTML character reference decoding), `html_entities.rs`.

---

### `svelte_analyze`
`crates/svelte_analyze/src/` — multi-pass analysis pipeline.

**Analysis pipeline** (порядок важен, см. `lib.rs`):

1. `classify_render_tags` — unwrap ChainExpression в render tags
2. `extract_script_info` + `js_analyze::analyze_script` — script metadata, OXC SemanticBuilder → Scoping, `NeedsContextVisitor`
3. `mark_runes` — rune classification
4. `template_scoping` — scopes для template constructs (each, snippet, if, await, key, head, boundary, svelte:element)
5. **Walk: `template_semantic` + `template_side_tables`** — `SemanticCollector` (mini-SemanticBuilder для template JS, resolves all IdentifierReference) + side table collection
6. `collect_symbols` — `ref_symbols` из OXC references, store detection
7. `classify_expression_needs_context` — per-expression needs_context
8. `post_resolve` — props analysis, known values, store aggregation
9. `classify_expression_dynamicity` — dynamicity classification
10. `lower` — whitespace trim, Text+ExprTag merge → `LoweredFragment`
11. **Walk 1: `reactivity`** — dynamic_nodes, dynamic_attrs, needs_ref
12. **Walk 2: `element_flags` + `hoistable` + `bind_semantics` + `content_types`** — 4 visitors за один обход
13. `classify_non_element_fragments` — Root, IfConsequent, EachBody classification
14. `validate` — семантические проверки

**Scope system** (`scope.rs`): `ComponentScoping` wraps `ComponentSemantics` (via `Deref`) и добавляет Svelte-specific classification (runes, props, each-block vars, rest props).

**Ключевые типы** (`data.rs`):
- `AnalysisData` — центральная side table, keyed by `NodeId`. Содержит sub-structs: `ElementFlags`, `FragmentData`, `SnippetData`, `ConstTagData`, `DebugTagData`, `EachBlockData`, `AwaitBindingData`, `BindSemanticsData`, `IgnoreData`
- `ExpressionInfo` / `ExpressionKind` — per-expression analysis
- `ParsedExprs<'a>` — OXC Expression ASTs (template exprs, attr exprs, concat parts, keys, etc.)
- `FragmentKey` variants: Root, Element, ComponentNode, IfConsequent, IfAlternate, EachBody, EachFallback, SnippetBody, KeyBlockBody, SvelteHeadBody
- `FragmentItem` variants: Element, ComponentNode, IfBlock, EachBlock, RenderTag, HtmlTag, KeyBlock, TextConcat
- `ContentStrategy` variants: Empty, Static, SingleElement, SingleBlock, DynamicText, Mixed
- `LoweredTextPart` variants: TextSpan, TextOwned, Expr

`IdentGen` (`ident_gen.rs`) — генерация уникальных имен. Shared между transform и codegen.

---

### `svelte_codegen_client`
`crates/svelte_codegen_client/src/` — generates client-side JS from AST + AnalysisData.

**Модули:**
- `context.rs` — `Ctx<'a>` (центральный контекст: builder, component, analysis, module_hoisted)
- `builder.rs` — `Builder<'a>` (враппер над OXC AstBuilder, `Arg` enum)
- `rune_transform.rs` — `transform_rune_get/set/update`
- `script.rs` — script rune transforms via OXC SemanticBuilder + ScriptTransformer
- `template/mod.rs` — root fragment generation по `ContentStrategy`
- `template/element.rs` — element codegen
- `template/attributes.rs` — static/dynamic attributes, bind directives
- `template/if_block.rs` — `$.if()`
- `template/each_block.rs` — `$.each()`
- `template/expression.rs` — JS expression generation from spans
- `template/html.rs` — HTML template string construction
- `template/traverse.rs` — DOM tree traversal (`.first_child`, `.sibling`)
- `template/const_tag.rs`, `debug_tag.rs`, `key_block.rs`, `html_tag.rs` — tag codegen
- `template/render_tag.rs`, `snippet.rs`, `component.rs` — snippet/component codegen
- `template/svelte_head.rs`, `svelte_element.rs`, `svelte_window.rs`, `svelte_document.rs`, `svelte_body.rs`, `svelte_boundary.rs`, `title_element.rs` — special elements
- `template/await_block.rs` — `{#await}` codegen

---

### `svelte_transform`
`crates/svelte_transform/src/` — mutates OXC expression ASTs in-place (после analyze, до generate).

Перезаписывает: rune references → `$.get/set/update`, prop sources → thunk calls, each-block context → `$.get`, snippet params → thunk calls, destructured const aliases → `$.get(tmp).prop`.

---

### `svelte_compiler`
`crates/svelte_compiler/src/lib.rs` — `compile()` (parse → analyze → transform → generate) и `compile_module()`.

---

### `wasm_compiler`
`crates/wasm_compiler/` — WASM-обёртка для JS.

---

## Dependency graph

```
svelte_span → svelte_diagnostics → svelte_ast → svelte_parser
  → svelte_component_semantics → svelte_analyze → svelte_transform
  → svelte_codegen_client → svelte_compiler → wasm_compiler
```

## Ключевые инварианты

- OXC Expression ASTs живут в `ParsedExprs<'a>` (аллокатор принадлежит caller'у), не выходят в публичный API
- `ComponentScoping` — owned, lifetime-free (oxc_semantic::Scoping внутри)
- Все поля `AnalysisData` — owned, без lifetime параметров
- AST хранит `Span`; parser парсит JS один раз в `ParsedExprs`; transform мутирует; codegen использует
- `u32` везде где возможно (NodeId, Span)
- `ConcatPart` (svelte_ast) и `LoweredTextPart` (svelte_analyze) — **разные типы** для разных фаз
- Sub-struct поля — `pub(crate)`, снаружи через методы
