# Codebase Map — svelte-rs-2

Rust компилятор Svelte v5. Компилирует `.svelte` → client-side JS + scoped CSS.

## Pipeline

```
source: &str
  → svelte_parser::parse_with_js → (Component, ParserResult, Vec<Diagnostic>)
  → svelte_parser::parse_css_block → Option<(StyleSheet, Vec<Diagnostic>)>
  → svelte_analyze::analyze_with_options → (AnalysisData, ParserResult, Vec<Diagnostic>)
  → svelte_analyze::analyze_css_pass → mutates AnalysisData (used selectors, hash, keyframes)
  → svelte_transform_css::transform_css_with_usage → String (scoped CSS)
  → svelte_transform::transform_component → (mutates ParserResult in-place, returns TransformData)
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

Ключевые типы: `NodeId`, `FragmentId`, `Component` (содержит `AstStore`), `Fragment`, `FragmentRole`, `Script`, `RawBlock`, `AstStore` (flat arena для всех template nodes).

**`Node` variants:** Text, Element, SlotElementLegacy, ComponentNode, Comment, ExpressionTag, IfBlock, EachBlock, SnippetBlock, RenderTag, HtmlTag, ConstTag, DebugTag, KeyBlock, SvelteHead, SvelteFragmentLegacy, SvelteElement, SvelteWindow, SvelteDocument, SvelteBody, SvelteBoundary, AwaitBlock, Error.

**`Attribute` variants:** StringAttribute, ExpressionAttribute, BooleanAttribute, ConcatenationAttribute, Shorthand, SpreadAttribute, ClassDirective, StyleDirective, BindDirective, LetDirectiveLegacy, UseDirective, OnDirectiveLegacy, TransitionDirective, AnimateDirective, AttachTag.

**Enums:** `ConcatPart` (Static/Dynamic), `StyleDirectiveValue`, `TransitionDirection`, `ScriptContext`, `ScriptLanguage`, `Namespace` (Html/Svg/Mathml).

`SVELTE_COMPONENT`, `SVELTE_SELF` — константы тегов.

---

### `svelte_component_semantics`
`crates/svelte_component_semantics/src/` — единый semantic graph для `.svelte` компонента (module script + instance script + template).

Заменяет `oxc_semantic::Scoping` / `SemanticBuilder` — OXC предоставляет только AST + Visit trait. Использует те же ID-типы (`ScopeId`, `SymbolId`, `ReferenceId`) из `oxc_syntax`.

Ключевые типы: `ComponentSemantics` (source of truth для scopes/symbols/references), `ComponentSemanticsBuilder` (builder, traverses JS via OXC Visit + template via `TemplateWalker` trait), `JsSemanticVisitor` (OXC Visit impl), `TemplateBuildContext`.

Подмодули: `builder/` (builder + JS visitor), `scope.rs`, `symbol.rs`, `storage.rs`, `reference.rs`.

---

### `svelte_ast_builder`
`crates/svelte_ast_builder/src/` — высокоуровневая обёртка над `oxc_ast::AstBuilder` для построения JS AST в codegen/transform.

Ключевые типы: `Builder<'a>` (враппер над OXC AstBuilder), `Arg<'a, 'short>` (Str/StrRef/Num/Ident/IdentRef/Expr/Arrow/Bool/Spread), `AssignLeft`, `TemplatePart`, `ObjProp` (KeyValue/Method/Shorthand/Spread/Getter/GetterBody/Setter/Computed).

Подмодули: `base.rs`, `calls.rs`, `classes.rs`, `functions.rs`, `members.rs`, `modules.rs`, `objects.rs`, `statements.rs`, `svelte_patterns.rs`, `templates.rs`.

---

### `svelte_parser`
`crates/svelte_parser/src/lib.rs` — парсер + JS pre-parsing.

Public API: `parse_with_js` (Svelte source → Component + ParserResult), `parse_module` (`.svelte.js`/`.svelte.ts`), `parse_css_block` (топ-уровневый `<style>` → `svelte_css::StyleSheet`).

Shared types в `types.rs`: `ParserResult` (instance/module/standalone OXC Programs + template expressions/statements keyed by span offset), `ExprHandle`, `StmtHandle`, `ParsedCeConfig`, `CePropConfig`, `CeShadowMode`.

Подмодули: `scanner/`, `parse_js.rs`, `walk_js.rs` (обход template для сбора JS-фрагментов), `html.rs` (HTML character reference decoding), `html_entities.rs`, `attr_convert.rs`, `handlers.rs`, `svelte_elements.rs`.

---

### `svelte_css`
`crates/svelte_css/src/` — Svelte-specific CSS parser, AST, printer.

Public API: `parse(&str) → (StyleSheet, Vec<Diagnostic>)`, `Printer::print` / `Printer::print_with_usage`, `Visit` / `VisitMut` traits.

Ключевые типы: `StyleSheet`, `StyleSheetChild`, `Rule`, `Block`, `BlockChild`, `Declaration`, `AtRule`, `SelectorList`, `ComplexSelector`, `RelativeSelector`, `SimpleSelector`, `CssNodeId` (стабильный id для side-tables в analyze).

Подмодули: `ast.rs`, `parser.rs`, `printer.rs`, `scanner.rs`, `visit.rs`.

---

### `svelte_transform_css`
`crates/svelte_transform_css/src/lib.rs` — мутация CSS AST: scope-class injection, keyframes rewriting, unused-selector pruning, `:global(...)` обработка, specificity bumps.

Public API: `transform_css(hash_class, keyframes, stylesheet, source) → String`, `transform_css_with_usage(... used_selectors, remove_unused, ...) → String`, `compact_css_for_injection(&str) → String` (минификация для injected mode).

Внутренний `ScopeSelectors` — `VisitMut` impl, обходит stylesheet.

---

### `svelte_analyze`
`crates/svelte_analyze/src/` — multi-pass analysis pipeline.

**Pass orchestration** (`passes/mod.rs`, `passes/executor.rs`):

Каждый pass — `PassKey` enum-вариант с `PassDescriptor` (requires/produces `DataToken`-ы). `executor::execute_pass` вызывает соответствующий pass module. Зависимости проверяются через токены, порядок задаётся стадиями.

Стадии (выполняются строго по порядку, см. `lib.rs::analyze_with_options`):

1. **`PRE_TEMPLATE_SCRIPT_STAGE`** — `AnalyzeScript`, `BuildComponentSemantics`, `FinalizeComponentName`, `ScanIgnoreComments`, `ExtractCeConfig`
2. **`INDEX_BUILD_STAGE`** — `TemplateSideTables`, `CollectSymbols`
3. **`POST_TEMPLATE_ANALYSIS_STAGE`** — `JsAnalyzePostTemplate`, `ClassifyNeedsContext`, `PostResolve`, `CollectConstTagFragments`, `BuildReactivitySemantics`
4. **`TEMPLATE_EXECUTION_STAGE`** — `BuildFragmentTopology`, `CollectHtmlTagNsFlags`, `ReactivityWalk`, `TemplateClassificationWalk`, `BuildBlockSemantics`
5. **`VALIDATION_STAGE`** — `ValidateTemplate`, `Validate`

CSS analysis вызывается отдельно из `svelte_compiler` через `analyze_css_pass` (после основного pipeline'а, до transform).

`analyze_module` — упрощённый pipeline для `.svelte.js`/`.svelte.ts`: только parse JS + scoping + rune detection + standalone validation + `reactivity_semantics::build_v2`.

**Ключевые модули:**
- `lib.rs` — entry points, `AnalyzeOptions`, `RuntimePlan` builder
- `passes/` — все analysis passes (по одному модулю на pass)
- `block_semantics/` — типы для control-flow блоков (`AwaitBlockSemantics`, `EachBlockSemantics`, `IfBlockSemantics`, `KeyBlockSemantics`, `SnippetBlockSemantics`, `RenderTagBlockSemantics`, `ConstTagBlockSemantics`)
- `reactivity_semantics/` — reactive declarations и signals
- `types/` — `data.rs` (`AnalysisData`), `script.rs`
- `scope.rs` — `ComponentScoping` (wraps `ComponentSemantics`)
- `validate/`, `passes/template_validation/` — семантические и template-level проверки (включая a11y warnings)
- `walker/` — общая инфраструктура обхода template
- `css.rs`, `passes/css_analyze.rs`, `passes/css_prune.rs`, `passes/css_prune_index.rs` — CSS pipeline
- `utils/` — `IdentGen`, `script_info`, helpers (`is_capture_event`, `is_delegatable_event`, `is_passive_event`, `is_regular_dom_property`, `normalize_regular_attribute_name`, etc.)

**Ключевые типы данных** (`types/data.rs`):
- `AnalysisData<'a>` — центральная side table, keyed by `NodeId`. Содержит ScriptAnalysis, TemplateAnalysis, ReactivitySemantics, BlockAnalysis, ElementAnalysis, FragmentFacts, RichContentFacts, CssAnalysis, CodegenView, RuntimePlan, и десятки специализированных side-tables
- `ElementFacts` владеет нормализованными per-element attribute facts (`AttrIndex` — внутренний by-name primitive). Consumer-pass’ы используют `AnalysisData` accessors (`has_attribute`, `attribute`, `string_attribute`, `bind_directive`, `expression_attribute`, etc.), не `attr_index(...)` напрямую
- `ExpressionInfo` / `ExpressionKind` / `ExprHandle` / `ExprDeps` / `ExprRole` / `ExprSite` — per-expression analysis
- `ParsedExprs<'a>` (через `ParserResult<'a>`) — OXC Expression ASTs
- `RuntimePlan` — финальный план для codegen (`needs_push`, `has_component_exports`, `has_bindable`, `has_stores`, etc.)
- `RuntimeRuneKind`, `OptimizedRuneSemantics` — rune-related semantics
- `CssAnalysis` — hash, keyframes, used_selectors, inject_styles

---

### `svelte_codegen_client`
`crates/svelte_codegen_client/src/` — generates client-side JS from AST + AnalysisData.

**Top-level:**
- `lib.rs` — `generate()` entry point
- `context.rs` — `Ctx<'a>` (центральный контекст: ast_builder, component, analysis, transform_data, ident_gen, module_hoisted)
- `custom_element.rs` — custom element wrapper

**`script/`** — script-side codegen
- `mod.rs` — script entry
- `pipeline.rs` — script transform pipeline (rune setup, props extraction, store subscriptions, etc.)

**`codegen/`** — template-side codegen
- `mod.rs` — root fragment dispatch
- `anchor.rs`, `async_plan.rs`, `dev.rs`, `effect.rs`, `expr.rs`, `namespace.rs`, `let_directive_legacy.rs` — shared helpers
- `attributes/` — `regular`, `class_directive`, `style_directive`, `bind/`, `concat_attr`, `expression_attr`, `spread_attr`, `events_common`, `on_directive_legacy`, `transition_directive`, `animate_directive`, `use_directive`, `attach_tag`, `dispatch`
- `blocks/` — `if_block`, `each_block`, `await_block`, `key_block`, `render_tag`, `snippet_block`, `html_tag`, `async_wrap`, `dispatch`
- `containers/` — `element`, `component`, `legacy_slot`, `special_target`, `svelte_element`, `svelte_boundary`
- `fragment/` — `prepare`, `process_children`, `legacy_slot_fragment`, `types`
- `hoisted/` — `const_tag`, `debug_tag`, `snippet`, `svelte_head`, `title`, `special_target`
- `component_props/` — codegen для props/spread/bind/events/snippet_children/slots при инстанцировании компонентов (`attach_prop`, `bind_prop`, `bind_this`, `dispatch`, `dynamic_ref`, `events`, `expression_prop`, `slots`, `snippet_children`, `spread_prop`)
- `data_structures/` — `async_emission_plan`, `codegen_error`, `codegen_result`, `concat`, `emit_state`, `fragment_anchor`, `fragment_ctx`, `memo`, `pre_anchor`, `template`

---

### `svelte_transform`
`crates/svelte_transform/src/` — mutates OXC expression ASTs in-place (после analyze, до generate).

Перезаписывает: rune references → `$.get/set/update`, prop sources → thunk calls, prop non-sources → `$$props.name`, each-block context → `$.get`, snippet params → thunk calls, destructured const aliases → `$.get(tmp).prop`. Также: TS-cleanup, derived calls, inspect runes, location injection (dev mode).

Public API: `transform_component(...) → TransformData`, `transform_script(...)`, `compute_line_col`, `sanitize_location`, `IgnoreQuery`, `TransformScriptOutput`.

**Структура:**
- `lib.rs` — entry, `transform_component`
- `data.rs` — `TransformData` (output)
- `rune_refs.rs` — rune reference rewrites
- `transformer/` — `mod.rs`, `entry.rs`, `template_entry.rs`, `model.rs`, `assignments.rs`, `derived.rs`, `inspect.rs`, `location.rs`, `props.rs`, `rewrites.rs`, `runes.rs`, `state.rs`, `statement_passes.rs`, `template_rewrites.rs`, `ts_cleanup.rs`

Template entry собирает `ExprHandle`/`StmtHandle` через структурный обход Svelte-AST, затем `oxc_traverse` гоняет `ComponentTransformer` по reusable synthetic `Program`.

---

### `svelte_compiler`
`crates/svelte_compiler/src/` — public compile API.

- `lib.rs` — `compile(source, &CompileOptions) → CompileResult` (полный pipeline: parse + CSS parse → analyze → analyze_css_pass → transform_css → transform_component → codegen), `compile_module(source, &ModuleCompileOptions) → CompileResult` (для `.svelte.js`/`.svelte.ts`)
- `options.rs` — `CompileOptions`, `ModuleCompileOptions`, `CssMode`, `GenerateMode`, `Namespace`, `ExperimentalOptions`
- `tests.rs` — unit tests

Codegen завёрнут в `catch_unwind` для надёжности; ошибки превращаются в diagnostics, не панику.

---

### `napi_compiler`
`crates/napi_compiler/src/lib.rs` — Node.js native addon (NAPI). Экспортирует `NativeCompileResult`, `NativeDiagnostic`, `NativeCompileOptions`, обёртку над `svelte_compiler::compile` / `compile_module`.

Публикуется как пакет `svelte-rs2` (см. `packages/svelte-rs2/`) с per-platform binaries (`packages/svelte-rs2-*/`).

---

### `wasm_compiler`
`crates/wasm_compiler/` — WASM-обёртка для browser/JS. Используется в playground.

---

## Dependency graph

```
svelte_span → svelte_diagnostics → svelte_ast → svelte_css
  → svelte_component_semantics → svelte_parser → svelte_ast_builder
  → svelte_analyze → svelte_transform → svelte_transform_css
  → svelte_codegen_client → svelte_compiler → { wasm_compiler, napi_compiler }
```

## Ключевые инварианты

- OXC Expression ASTs живут в `ParsedExprs<'a>` (аллокатор принадлежит caller'у), не выходят в публичный API
- `ComponentScoping` — owned, lifetime-free (`ComponentSemantics` внутри)
- Все поля `AnalysisData` — owned, без lifetime параметров (кроме `ParsedExprs<'a>`)
- AST хранит `Span`; parser парсит JS один раз в `ParsedExprs`; transform мутирует; codegen использует
- `u32` везде где возможно (`NodeId`, `FragmentId`, `Span`, `CssNodeId`)
- `ConcatPart` (svelte_ast) и аналоги в svelte_analyze — **разные типы** для разных фаз
- Sub-struct поля — `pub(crate)`, снаружи через методы
- Каждый analyze pass — изолированный модуль; зависимости описаны через `DataToken`, выполнение в фиксированном порядке стадий
- CSS pipeline отдельный: `svelte_css` (parser/AST/printer) и `svelte_transform_css` (мутации) — analyze CSS интегрирован в `svelte_analyze::passes::css_*`, но запускается из `svelte_compiler`
- Codegen — «dumb»: всю логику решает analyze, codegen только эмитит. Transform только мутирует JS AST, не делает классификацию.
