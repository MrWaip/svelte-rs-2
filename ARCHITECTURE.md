# Architecture

Purpose: rules and invariants per crate. Not an API reference — see CODEBASE_MAP.md for that.

## Crate layers (bottom-up)

1. ast
2. parser
3. analyze
4. transform
5. codegen
6. compiler (entry)

---

## 1. AST

Crates: `svelte_ast`, `svelte_css`, plus `oxc_ast` for JS expressions/statements.

### Purpose

Owns tree shapes produced by parsing. No semantics, no scopes, no mutation logic.

### What lives here

- `svelte_ast::Component` — root holding `AstStore`, optional `instance_script`/`module_script` (`Script` metadata only), optional `css: RawBlock`, `options: SvelteOptions`, full `source: String`.
- `AstStore` — flat arena. All template nodes and fragments live in vectors keyed by `NodeId(u32)` / `FragmentId(u32)`.
- Template node enum `Node` with variants like `Element`, `IfBlock`, `EachBlock`, `Component`, etc. Every variant carries `pub id: NodeId` and `pub span: Span`.
- `Fragment` — child node list + `FragmentRole` + optional owner `NodeId`.
- `ExprRef` / `StmtRef` (`expr_ref.rs`) — pointer from Svelte AST to a JS node inside an `oxc::Program`. Holds `Span` plus a late-bound `Cell<OxcNodeId>`.
- `svelte_css::ast` — CSS tree. Uses `CompactString` for identifiers and `Option<String>` override fields (e.g. `Declaration.value_override`, `AtRule.prelude_override`) so CSS pruning/rewriting can replace text in place.

### Constraints / invariants

- **No lifetimes on `svelte_ast` types.** AST must be `'static`-friendly. Template nodes must not borrow from source.
- **No strings in template AST.** Identifiers, attribute names, etc. are referenced via `Span` and resolved through `Component::source_text(span)`.
- **Flat storage.** Tree shape is encoded by `NodeId`/`FragmentId` indices in `AstStore`, not by `Box`/`Rc`. Children are `Vec<NodeId>` inside `Fragment`.
- **Every node has a `NodeId` and `Span`.** Enforced by the `Node` enum macro.
- **JS bridge through `OxcNodeId`.** Svelte AST never holds an `oxc::Expression`/`Statement` directly; it holds `ExprRef`/`StmtRef`. The actual `oxc_ast` nodes live inside `oxc::Program` instances managed by analyze.
- **`OxcNodeId` is bound late.** `ExprRef::new` starts as `OxcNodeId::DUMMY`; `ComponentSemanticsBuilder` assigns real IDs during semantic build by walking the corresponding `oxc::Program` and matching by span. `ExprRef::id()` panics if read before binding.
- **Two oxc programs per component.** One for `<script module>`, one for `<script>`. Both built by `svelte_parser::parse_js` into a caller-provided `oxc_allocator::Allocator`. NodeIds across the two programs are kept disjoint via `next_node_id` offset in `ComponentSemanticsBuilder`.
- **CSS AST may hold owned strings.** It is the only AST that gets mutated post-parse (CSS pruning, scoping). Override fields hold the rewritten text; original spans stay intact for diagnostics.

---

## 2. Parser

Crates: `svelte_parser` (template + JS entry), `svelte_css::parser` (CSS).

### Purpose

**The only place** that turns source text into an AST. Downstream layers never re-parse.

### Boundary with analyze

Parser produces tree shape + spans. Analyze produces meaning (scopes, symbols, references, runes mode, reactive graph). If a question needs looking at more than one node together, it belongs in analyze.

---

## 2.1 Template parser

### Purpose

Handwritten scanner over `.svelte` source. Produces `Component` (template tree + script metadata + raw CSS block).

### Inputs / outputs

- Input: `&str` source of `.svelte` plus `&Allocator` (forwarded to 2.2 for embedded JS).
- Output: `svelte_ast::Component` — `AstStore` entries, optional `instance_script` / `module_script` (`Script` metadata only), optional `css: RawBlock`, `options: SvelteOptions`, full `source: String`.

### Constraints

- Lives in `svelte_parser::scanner` + `svelte_parser::handlers`. No external grammar generator.
- Pure syntax. No semantic classification (scope, references, runes, store sigils).
- Best-effort recovery: malformed regions become `ErrorNode` with a span; parsing continues.
- MAY emit synthetic JS statements when a template tag is itself a JS construct. `{@const name = expr}` is parsed via `parse_js::parse_const_declaration_with_alloc` into a real `oxc` `VariableDeclaration` so analyze sees a normal binding. This is the only allowed form of synthesis — wrapping template-level JS source in valid JS so `oxc_parser` can consume it.

### Anti-patterns

- Calling template parser from analyze/transform/codegen to "re-parse" a fragment.

---

## 2.2 JS parser (`parse_js`)

### Purpose

Single bridge between `.svelte` source and `oxc_parser`. Produces `oxc::Program` / `Expression` / `VariableDeclaration` for `<script module>`, `<script>`, template expressions, each-block context patterns, `{@const}` declarations, etc.

### Inputs / outputs

- Input: `&'a Allocator`, source `&str`, source type (JS/TS, module/script), and offset for span correction when parsing wrapped sub-strings.
- Output: `oxc::Program<'a>` / `Expression<'a>` / `VariableDeclaration<'a>` in the caller-provided allocator.

### Constraints

- `Allocator` is always passed in by the caller. `oxc::Program` outlives the parse call — analyze owns it.
- Does NOT bind `OxcNodeId`s on `ExprRef` / `StmtRef`. Binding happens in `ComponentSemanticsBuilder`.
- Does NOT classify runes, store sigils, or prop bindings — that is analyze's job.
- Wrapped sub-parses (expressions, each contexts, const decls) reconstruct accurate spans via offset arithmetic.

### Anti-patterns

- Hand-writing a JS parser instead of routing through `oxc_parser`.
- Owning an `Allocator` inside the parser.
- Calling `parse_js` from analyze/transform/codegen to "re-parse" a fragment.

---

## 2.3 CSS parser

### Purpose

Handwritten scanner over the `<style>` raw block content. Produces `svelte_css::ast::StyleSheet`.

### Inputs / outputs

- Input: `&str` of style content + base offset (so spans line up with the original `.svelte` source).
- Output: `StyleSheet` in `svelte_css::ast`.

### Constraints

- Fills original fields only (`name`, `prelude`, identifiers as `CompactString`). `*_override: Option<String>` fields stay `None`; they are reserved for `svelte_transform_css` to rewrite text in place while keeping original spans for diagnostics.
- Pure syntax. No selector resolution, no scope classification, no used/unused tracking.

### Anti-patterns

- Resolving selectors, scoping, or pruning at parse time.
- Calling CSS parser from analyze/transform/codegen to "re-parse" a fragment.

---

## 3. Analyze

### Purpose (smart analyzer dogma)

Pre-compute every decision so transform + codegen stay linear and dumb.

Transform/codegen MUST NOT:

- assemble meaning from raw boolean flags scattered across multiple side-tables
- re-walk AST to gather facts
- combine raw indices to derive a question

Analyze owes them ready-made answers: enums and structs that name the decision directly. If transform/codegen needs `if a && !b && c.kind == X` to know what to emit, that compound condition belongs as a single named field on an analyze struct.

### Target shape

Analyze is composed of **semantic subsystems** (`ComponentSemantics`, `ReactivitySemantics`, `BlockSemantics`, …). Each subsystem:

- absorbs multiple raw indices / side-tables internally
- exposes a query API answering one concrete question per call:
  - "how do I read this identifier?" → `ReactivitySemantics::reference_semantics(ref_id)`
  - "what does this block lower into?" → `BlockSemanticsStore::get(node_id)`
- never leaks raw flags to consumers

Current code is partway there. New facts go into a subsystem (or motivate a new one), not loose `AnalysisData` fields.

### Validation

Analyze also owns the checker/validate phase. User-facing diagnostics (errors + warnings) are produced here from already-built semantics. Transform and codegen run on validated input and emit no diagnostics.

### BindingPattern handling (cross-cutting)

For walking destructuring patterns (`let { a, b: { c } } = …`, function params, each-block items, snippet params) **do not flatten the pattern into analyze data**. Mapping `BindingPattern` shape into bespoke side-table structures duplicates the AST and silently drops details (defaults, rest elements, nested rest, computed keys).

Rules:

- Walk patterns with `walk_bindings`. If it doesn't fit, write a focused manual recursion against `oxc_ast` — still no flattening into analyze.
- Analyze publishes per-leaf facts keyed by `OxcNodeId` / `ReferenceId` / `SymbolId` of the leaf identifier (or pattern node) — never a reshaped pattern tree.
- Codegen / transform walks the original `BindingPattern` and queries analyze per leaf to pick lowering.

Anti-pattern: an `enum AnalyzedPattern { Identifier(...), Object { props: Vec<...> }, ... }` mirroring `BindingPattern` shape.

### Constraints

- read-only over AST
- single source of truth for metadata
- analysis vs transformation split

---

## 3.1 ComponentSemantics (generic JS scope graph)

Crate: `svelte_component_semantics`.

### Purpose

Modeled after `oxc` `Scoping` module, adapted to a Svelte component (multi-program: `<script module>`, `<script>`, template). Single pass over AST to build the JS-level scope/symbol/reference graph and bind `OxcNodeId`s.

### What it does

- Walks AST and collects:
  - **Scopes** (`ScopeId`)
  - **Bindings** (`SymbolId`)
  - **References** (`ReferenceId`)
- Assigns `OxcNodeId` to every relevant `oxc_ast` node, contiguous across `<script module>` → `<script>` → template (offset bookkeeping in `ComponentSemanticsBuilder::next_node_id`).
- Binds `OxcNodeId` into `ExprRef` / `StmtRef` carried on Svelte AST nodes (resolves the `Cell<OxcNodeId>::DUMMY` slots set by parser).
- Tracks per-binding usage facts: read, write, mutate (member mutation, update expression).

### What it does NOT do

- No Svelte-specific classification (runes, store sigils, prop kinds, each-block kinds). That lives in `ComponentScoping` / `ReactivitySemantics` / `BlockSemantics`.

### Constraints

- Generic: knows nothing about `$state`, `$props`, stores, snippets, each-blocks.
- One source of `OxcNodeId`s. Downstream subsystems index by them; nobody else hands out fresh ones.
- Read-only on AST after build.
- **Identity by id, never by string.** Symbol / binding / reference resolution goes through `OxcNodeId` / `ReferenceId` / `SymbolId`. No `find_binding_by_name("foo")` for real lookups. Name comparison is allowed only for syntactic predicates (e.g. detecting `$state` rune callee, `$$props`).

### Anti-patterns

- Adding Svelte-specific flags (`is_state`, `is_prop`, `is_store`) directly onto `ComponentSemantics`.
- Re-running the walker from another pass to re-derive scopes/references.
- Resolving identifiers by string lookup instead of `OxcNodeId` → `ReferenceId` → `SymbolId`.

---

## 3.2 ReactivitySemantics

Module: `svelte_analyze::reactivity_semantics`.

### Depends on

`ComponentSemantics` (scopes, symbols, references, `OxcNodeId` bindings).

### Purpose

Build the **reactivity graph** of the component. Owns everything reactive: runes, stores, legacy props, legacy `$:` reactivity. Single answer surface for:

- "is this binding reactive?"
- "what kind of binding is this?" (state / derived / prop / store / contextual / plain local)
- "what is this identifier read?" (signal read, prop read, store subscription, plain read, member-mutation root, …)

### What it owns

- Per-declaration facts indexed by `OxcNodeId` (rune kind, mutation status, lowering plan).
- Per-reference facts indexed by `ReferenceId` (read kind, target family, member-mutation flag).
- Side indices for cheap lookup of subsets: store declarations, legacy bindable props, contextual owners, etc.
- Rune-mode flag (`uses_runes`) and legacy `$$props` / `$$restProps` usage flags.

### Query surface

Consumers use mainly two calls:

- `declaration_semantics(oxc_node_id) -> DeclarationSemantics`
- `reference_semantics(reference_id) -> ReferenceSemantics`

Exceptions are coarse-grained iterators over a known subset: `iter_store_declarations`, `legacy_bindable_prop_symbols`, etc.

### Constraints

- Read-only for consumers. Mutation is private to the builder.
- One source of truth for reactivity classification. No shadow flags on `ComponentScoping` or `ScriptAnalysis`.
- Builder runs once, after `ComponentSemantics` is finished.

### Anti-patterns

- Re-deriving rune kind from AST in transform/codegen.
- Adding a getter that exposes raw bool flags to a consumer — encode the decision as an enum variant on `DeclarationSemantics` / `ReferenceSemantics`.
- Duplicating a fact onto another subsystem just because the import is shorter.

---

## 3.3 BlockSemantics

Module: `svelte_analyze::block_semantics`.

### Depends on

`ComponentSemantics`, `ReactivitySemantics`, plus AST.

### Purpose

Single, exhaustive answer to codegen: "what does this template block become?". For one block `NodeId` codegen receives one `BlockSemantics` variant carrying every decision needed to emit runtime code — no extra lookups, no boolean assembly.

### Block variants

- `Each(EachBlockSemantics)` — `{#each ...}`
- `If(IfBlockSemantics)` — `{#if ...}` / `:else if` / `:else`
- `Await(AwaitBlockSemantics)` — `{#await ...}` / `:then` / `:catch`
- `Key(KeyBlockSemantics)` — `{#key ...}`
- `Snippet(SnippetBlockSemantics)` — `{#snippet name(...)}`
- `Render(RenderTagBlockSemantics)` — `{@render name(...)}`
- `ConstTag(ConstTagBlockSemantics)` — `{@const ... = ...}`
- `NonSpecial` — default for any non-block `NodeId`

### Query surface

- `BlockSemanticsStore::get(node_id) -> &BlockSemantics` — total. Out-of-range / non-block ids collapse to `&BlockSemantics::NonSpecial`. No `Option`.
- Side index: `block_for_each_index_sym(sym)` / `is_each_index_sym(sym)`.

### Constraints

- Total API. No `Option<&BlockSemantics>` on the public surface.
- Each variant carries pre-computed lowering shape (flavor, async kind, item/index/key strategy, render call shape, await wrapper layout, etc.). Codegen reads, never combines.
- Read-only after build.

### Anti-patterns

- Codegen branching on `if has_index && !is_keyed && collection_kind == X` to decide what to emit — that compound belongs as a single named field on the variant.
- Returning `Option<&BlockSemantics>` from the store.
- Adding a parallel side-table that re-encodes a fact already on a variant.

---

## 3.4 Validation

Module: `svelte_analyze::validate`.

### Purpose

Walk the AST and emit user-facing diagnostics — warnings + errors. Runs after all semantic subsystems are built; reads them, never mutates them.

### Constraints

- Read-only on AST and on every analyze subsystem.
- Only producer of `Diagnostic` for user-facing parity. Transform/codegen are diagnostic-free.
- Do not re-derive facts here that a subsystem already owns — query and report.

### Anti-patterns

- Mutating semantic subsystems from a validation pass.
- Re-walking AST to recompute classification before reporting.
- Emitting diagnostics from transform or codegen.

---

## 4. Transform

Crate: `svelte_transform`.

### Purpose

Walk JS AST nodes and rewrite them. Lowers runes (`$state` / `$derived` / `$props` / `$effect` / …), rune call sites, reactive reads/writes, store sigils, legacy `$:` blocks — into runtime calls.

### Inputs / outputs

- Inputs: `&AnalysisData`, `JsAst<'a>` (instance + module `oxc::Program`), allocator.
- Output: mutated `oxc::Program`s ready for codegen.

### Constraints

- Mutates JS AST in place. Does not produce new analyze data.
- One analyze query per use case should be enough for a single unambiguous decision. If transform needs to combine flags from several subsystems to pick a lowering, the missing answer belongs as a new field/variant in analyze, not as transform-side glue.
- Does not emit diagnostics.
- Does not re-walk AST to reclassify nodes.

### Anti-patterns

- Recomputing rune kind / reactivity facts by inspecting AST.
- Stitching meaning together from multiple raw boolean flags across subsystems.
- Emitting user-facing errors/warnings.

### Reactive reference dispatchers

All AST mutations driven by `ReferenceSemantics` go through five centralized dispatchers in `crates/svelte_transform/src/transformer/rewrites.rs`:

- `dispatch_identifier_read` — identifier reads.
- `dispatch_identifier_assignment` — `=` / `+=` / `&&=` / … on identifier targets.
- `dispatch_identifier_update` — `++` / `--` on identifier targets.
- `dispatch_member_assignment` — assignment on member targets, keyed off the member root's reference semantics.
- `dispatch_member_update` — update on member targets, keyed off the member root's reference semantics.

Each dispatcher uses an exhaustive `match` over every `ReferenceSemantics` variant. Adding a new variant is enforced by the compiler: `match` non-exhaustiveness errors fire in all five dispatchers, forcing the developer to wire the new primitive's read / identifier-write / identifier-update / member-write / member-update behavior up front (or explicitly mark it as no-op for that operation).

`transform_assignment`, `transform_update`, `template_rewrites::rewrite_template_enter`, `rewrite_template_exit`, and the runes identifier traversal all call only these dispatchers — never the per-kind helpers directly. Adding a new reactive primitive should never need a new "if X try this" chain in any traversal.

---

## 5. Codegen (dumb codegen dogma)

Crate: `svelte_codegen_client`.

### Purpose

Walk the (analyzed + transformed) AST and emit Svelte runtime JS. One query to analyze per use case → one unambiguous emit decision. No re-walks to gather meaning, no compound flag stitching.

### Inputs / outputs

- Inputs: `&Component` (template AST), `&AnalysisData`, transformed `oxc::Program`s.
- Output: `oxc::Program` of Svelte runtime client JS, ready for printing.

### Constraints

- No new analysis. No re-walks of AST to reclassify nodes.
- No diagnostics.
- Codegen still owns layout-only computations that depend on the surrounding emit context (sibling layout, anchors, ident generation). These are not "analyze of meaning" — they are local printing decisions.

### Local emit-time analyses

Codegen does carry one analyze-shaped phase that lives in codegen by design — it depends on the emit context (preserve_whitespace, surrounding tags) and only matters for printing.

**Fragment prepare** — `crates/svelte_codegen_client/src/codegen/fragment/prepare.rs`. In one pass over a fragment's children:

- Hoist out structural nodes (snippets, const-tags, debug-tags, `<svelte:head>` / `<svelte:window>` / `<svelte:document>` / `<svelte:body>`, head-titles) into the bucket.
- Trim/normalize whitespace per Svelte rules (preserve_whitespace, `<pre>`, `<textarea>`, expression-tag adjacency).
- Coalesce adjacent `Text` + `{expression}` runs into a single `Concat`.
- Classify the resulting children into a `ContentStrategy` (`Empty`, `SingleStatic`, `SingleExpr`, `SingleConcat`, `SingleElement`, `SingleBlock`, `Multi { … }`) so the parent emitter takes one branch with no extra inspection.

### Anchors

Codegen tracks a current **fragment anchor** — the DOM reference children get appended/inserted relative to. `FragmentAnchor` variants (in `data_structures.rs`):

- `Root` — top-level fragment, anchor is `$$anchor` parameter.
- `CallbackParam { name, append_inside }` — block body where the anchor is a callback parameter (`#each`, `#if`, …).
- `Child { parent_var }` — anchor is a child node of a known parent ident.
- `SiblingVar { var }` — anchor is an existing sibling ident.

Anchor handling lives in `codegen/anchor.rs` (`reserve_comment_anchor_pre`, `commit_comment_anchor`, `comment_anchor_node_name`, `direct_anchor_expr`). Block emitters reserve anchor idents up-front (`pending_anchor_idents`) and commit them when the comment anchor is materialized.

### Anti-patterns

- Re-walking AST to derive a fact analyze already owns.
- Compound branching on raw bools / indices to pick an emit shape — push the decision into analyze as a named variant.
- Emitting diagnostics from codegen.
- Re-running fragment-prepare-style passes outside of codegen (it is emit-context-specific).

---

## 6. Compiler entry

Crate: `svelte_compiler`. Public entry: `compile(source, &CompileOptions) -> CompileResult`. Module entry: `compile_module(source, &ModuleCompileOptions)`.

### Purpose

Single orchestrator wiring parser → analyze → transform → codegen → CSS transform. Owns the JS `Allocator` and threads `JsAst<'a>` through phases. Always returns `CompileResult { js, css, diagnostics }` — never panics. On failure `js` is `None` but diagnostics flow.

### Pipeline (component path)

1. `oxc_allocator::Allocator::default()` — owned here, lives until codegen finishes.
2. `svelte_parser::parse_with_js(&alloc, source)` → `(Component, JsAst, Vec<Diagnostic>)`.
3. Apply `CompileOptions` to component (namespace fallback; runes / accessors / immutable / preserve_whitespace resolved against `<svelte:options>`).
4. `svelte_parser::parse_css_block(&component)` → CSS AST.
5. `svelte_analyze::analyze_with_options` → `AnalysisData`, may add diagnostics.
6. If no `Severity::Error` so far → `svelte_transform` mutates `JsAst`.
7. `svelte_codegen_client` emits client JS from `AnalysisData` + transformed `JsAst`.
8. `svelte_transform_css::transform_css_with_usage` rewrites CSS using `CssAnalysis`.

### Constraints

- Compiler is the only owner of `Allocator`. Phase functions borrow it.
- Compiler itself produces no diagnostics; it aggregates from parser + analyze.
- Standalone module path (`compile_module`) routes through `analyze_module` and skips template/css steps.

### Anti-patterns

- Allocating a second `Allocator` mid-pipeline.
- Inlining analyze / transform / codegen logic into the entry crate.

---

## 7. Supporting crates

### 7.1 `svelte_ast_builder`

Crate: `svelte_ast_builder`. Holds `Builder<'a>` plus `AssignLeft`, `TemplatePart`, `ObjProp`, `Arg`, etc.

#### Purpose

Single, ergonomic surface for constructing `oxc_ast` nodes from transform/codegen. Codifies common JS shapes (call, member, template literal, object literal, assignment, declarator, function) so emit code stays declarative.

#### Constraints

- The only allowed construction site for `oxc_ast` nodes outside `oxc_parser`. Transform / codegen MUST go through `Builder`.
- `Builder` borrows the `Allocator` from compiler entry.

#### Anti-patterns

- Hand-rolling `oxc_allocator::Box::new_in(...)` / `Vec::new_in(...)` / `oxc_ast::ast::*` constructors from transform/codegen.
- Adding a one-off helper inside transform/codegen instead of extending `Builder`.

### 7.2 `svelte_transform_css`

Crate: `svelte_transform_css`. Public entry: `transform_css(...)`, `transform_css_with_usage(...)`, `compact_css_for_injection(css)`.

#### Purpose

Apply CSS scoping (`svelte-<hash>` class), prune unused selectors, write rewritten text into `*_override` fields on `svelte_css::ast`. Reads `CssAnalysis` from analyze.

#### Constraints

- Only writer of `*_override` fields. Original spans + identifier strings stay intact.
- No new selector classification — analyze owns it.

#### Anti-patterns

- Re-parsing CSS or re-classifying selectors here.
- Touching CSS AST fields other than `*_override`.

---

## 8. Cross-cutting

### Diagnostics

Crate: `svelte_diagnostics`. Single `Diagnostic` type with `Severity` (Error / Warning / Info).

- Producers: parser (syntactic), analyze (semantic + validation). Nobody else.
- Transform / codegen / transform-css / compiler entry never produce diagnostics.
- `AnalyzeOptions::warning_filter` is the only place that suppresses warnings post-collection.
- Compiler entry aggregates and returns the unified `Vec<Diagnostic>`.

### Standalone modules (`.svelte.js` / `.svelte.ts`)

- Entry: `svelte_compiler::compile_module` → `svelte_analyze::analyze_module`.
- Builds a dummy `Component` (no template, empty `AstStore`, source kept) just to satisfy APIs that take `&Component`.
- Pipeline skips template walking, CSS, fragment prepare. Only JS scoping + rune transforms run.
- Component path code MUST NOT be reused as-is for modules — they have different invariants (no template fragment, no `<script>` distinction).

### IdentGen — unique identifiers

`svelte_analyze::utils::IdentGen` (+ `IdentGenSnapshot`).

- Single source of fresh JS identifiers across analyze / transform / codegen.
- `gen("prefix")` returns a name that does not collide with any binding seen by `ComponentSemantics` or any previously generated ident.
- `snapshot` / `restore` for backtracking emit branches.
- Anti-pattern: `format!("__name_{}", counter)` ad-hoc in any consumer.

### Test harness

- Compiler tests: `tasks/compiler_tests/cases2/<name>` — input `.svelte` + generated reference output (`case-*.json`, `case-*.js`).
- Diagnostic tests: `tasks/diagnostic_tests/cases/<name>`.
- Generated files (`case-*.json`, `case-*.js`) are produced by `just generate`. **Never hand-edit them.**
- Required gates after a task: `just test-compiler`, `just test-diagnostics`, `just clippy-strict` — all green.
- New skill-driven flows (`add-test`, `port`, `diagnose`, `audit`, `quick-check`) are the canonical way to register cases.
