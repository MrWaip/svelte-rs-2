# Architecture Review

Full codebase review with performance analysis. Generated 2026-03-18.

---

### ~~#1 — Codegen re-derives symbol semantics from source text via string-based lookups~~

> **Implemented**: Pre-computed `BindSemanticsData` in analysis (new `BindSemanticsVisitor` in composite walk). Eliminated `Ctx::is_mutable_rune`. Fixed `build_style_concat` to trust pre-transformed expressions (also fixed transform to process `StyleDirective::Concatenation` parts). Refactored `gen_bind_directive`, `build_directive_prop`, `build_bind_this_call`, `gen_window_binding`, `gen_document_binding`, and each_block prop_source to use `is_mutable_rune_target(NodeId)` / `is_prop_source_node(NodeId)`. Deferred: `add_optional_chaining`, `extract_identifiers` mini-parsers (complex bind:this), render_tag per-argument prop_source, `Ctx::is_import`.

**Dimension**: 1. Late knowledge / 7. Wrong phase
**Severity**: critical
**Evidence**:
- `crates/svelte_codegen_client/src/context.rs:233-237` — `is_mutable_rune` takes a name string, resolves to SymbolId via scope lookup, checks `is_rune && is_mutated`
- `crates/svelte_codegen_client/src/template/attributes.rs:21,402,436-465,663` — called ~8 times on raw text extracted from source spans
- `crates/svelte_codegen_client/src/template/attributes.rs:399-413` — `build_style_concat` extracts source text, checks `is_mutable_rune`, then discards pre-transformed expression and rebuilds `$.get()` manually
- `crates/svelte_codegen_client/src/template/component.rs:189-198,269,287` — same for component `bind:this`
- `crates/svelte_codegen_client/src/template/each_block.rs:62-68` — collection expression prop-source check re-resolves by name from source text
- `crates/svelte_codegen_client/src/template/render_tag.rs:20,37-38,92` — extracts callee name from original source text at span offsets
- `crates/svelte_codegen_client/src/template/component.rs:214-259,311` — `is_simple_identifier`, `add_optional_chaining`, `extract_identifiers` are ad-hoc mini-parsers operating on source text

**Problem**: The transform phase already rewrites identifiers based on SymbolId classification (rune, prop, each-var). But codegen independently re-derives the same classification from name strings for bind directives, style directive concatenations, class directive shorthands, each-block collections, render tag callees, and component bind:this. This violates the project rule "String-based membership tests are forbidden for semantic decisions." The style_concat case is especially fragile: it discards the transform phase's output and rebuilds `$.get()` manually, creating an implicit coupling where codegen and transform must independently reach the same conclusion about how rune getters look.

**Fix**: Extend the transform phase to produce per-directive output descriptions (e.g., `BindTarget::Rune(name)` / `BindTarget::Plain(name)` / `BindTarget::PropSource(name)` stored in `TransformData`). For bind:this, transform produces the getter/setter expressions via AST manipulation instead of string surgery. For style concat, trust the pre-transformed expression entirely. Codegen consumes pre-computed tags instead of re-resolving.

**Simplifies**: Eliminates `Ctx::is_mutable_rune`, `Ctx::is_import`, all `source_text(span).trim()` + `find_binding` patterns in codegen, the ad-hoc mini-parsers (`is_simple_identifier`, `add_optional_chaining`, `extract_identifiers`), and the `format!("... = $$value")` + `parse_expression` string round-trip.

---

### #2 — `each_vars` in codegen is a manual scope stack duplicating analysis scoping

**Dimension**: 8. Scattered ownership
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/context.rs:163` — `pub each_vars: Vec<String>` field on Ctx
- `crates/svelte_codegen_client/src/template/each_block.rs:84-95` — push/pop of each-block context and index variable names around `gen_fragment`
- `crates/svelte_codegen_client/src/template/component.rs:305-307` — `extract_identifiers(expr_text).filter(|id| ctx.each_vars.contains(id))` — string-based membership test
- `crates/svelte_analyze/src/scope.rs:309-321` — `build_scoping` already creates child scopes with each-block bindings

**Problem**: Analysis already builds a complete scope tree with each-block context and index variables (`ComponentScoping`). Codegen rebuilds a parallel shadow as a `Vec<String>` stack. If push/pop gets out of sync with `build_scoping`, `bind:this` on components inside each blocks will silently generate wrong output. No compile-time contract ensures sync.

**Fix**: Add `is_each_block_var(sym_id) -> bool` to `ComponentScoping`. Codegen resolves identifiers via the scoping tree and queries this flag.

**Simplifies**: `Ctx.each_vars` field, push/pop ceremony in `gen_each_block`, and `extract_identifiers` + `filter` pattern in `component.rs` all deleted.

---

### #3 — `NodeIndex` builds 15 separate HashMaps by walking the full AST at codegen startup

**Dimension**: 8. Scattered ownership / Performance
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/context.rs:13-29` — 15 separate `FxHashMap<NodeId, &T>` fields
- `crates/svelte_codegen_client/src/context.rs:32-129` — `walk` populates them all in a single traversal
- `crates/svelte_codegen_client/src/context.rs:200-215` — 12 near-identical accessor methods that panic on missing IDs

**Problem**: Every codegen invocation walks the entire AST to build 15 type-specific HashMaps. Since `NodeId` is a sequential `u32`, all 15 maps could be replaced by a single `Vec` indexed directly, giving true O(1) with no hashing. Adding a new node type requires updating a HashMap, the walk function, and an accessor — three places in lockstep with no type system guidance.

**Fix**: Replace 15 `FxHashMap<NodeId, &T>` with a single `Vec<NodeRef<'a>>` enum indexed by `node.id().0 as usize`. Long-term: store AST nodes in a flat arena indexed by `NodeId`.

**Simplifies**: 15 HashMap fields, the walk function's 15 insert arms, and 12 accessor methods collapse to one each. Eliminates hashing overhead on every node lookup.

---

### #4 — `SingleBlockKind` mirrors `FragmentItem` variants 1:1

**Dimension**: 5. Incidental logic
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/data.rs:402-426` — `SingleBlockKind` enum with 10 variants, each wrapping `NodeId`
- `crates/svelte_analyze/src/data.rs:284-337` — `FragmentItem` has the same block variants
- `crates/svelte_analyze/src/content_types.rs:54-67` — `classify_items` converts FragmentItem to SingleBlockKind, one arm per variant

**Problem**: `SingleBlockKind` exists solely to embed a NodeId-typed discriminant into `ContentStrategy::SingleBlock`. Every block variant in `FragmentItem` has a mirror, and conversion is a mechanical 1:1 mapping. When a new block type is added, both enums must grow in lockstep.

**Fix**: `ContentStrategy::SingleBlock` stores a `FragmentItem` directly. Eliminate `SingleBlockKind` entirely.

**Simplifies**: Removes 10-variant enum, `node_id()` method, all conversion code. Codegen matches on `FragmentItem` directly.

---

### #5 — `ContentStrategy::Dynamic` booleans are raw facts that codegen interprets

**Dimension**: 3. Raw handoff
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/data.rs:440-441` — `ContentStrategy::Dynamic { has_elements: bool, has_blocks: bool, has_text: bool }`
- `crates/svelte_codegen_client/src/template/mod.rs:137` — `Dynamic { has_elements: false, has_blocks: false, .. }` selects `gen_root_dynamic_text`
- `crates/svelte_codegen_client/src/template/element.rs:77,92` — same boolean pattern check

**Problem**: Analysis produces three booleans that codegen must pattern-match in specific combinations. The combination `(false, false, true)` means "dynamic text only" — this interpretation is implicit knowledge shared between phases.

**Fix**: Split `ContentStrategy::Dynamic` into `DynamicText` and `Mixed { has_elements, has_blocks, has_text }`. Analysis produces the named variant.

**Simplifies**: All `Dynamic { has_elements: false, has_blocks: false, .. }` guards become simple `DynamicText` arms.

---

### #6 — `gen_root_fragment` and `gen_fragment` have ~200 lines of near-duplicate code

**Dimension**: 4. Cognitive complexity
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/template/mod.rs:63-163` — `gen_root_fragment` with all content strategies
- `crates/svelte_codegen_client/src/template/mod.rs:347-533` — `gen_fragment` with nearly identical handling

**Problem**: Both functions have the same `ContentStrategy` match with nearly identical code for all variants. Root adds `$.next()` and uses direct hoisting; nested uses `ctx.module_hoisted`. Fixes in one are easily missed in the other.

**Fix**: Extract a shared `gen_fragment_body(ctx, key, is_root) -> FragmentOutput` that handles all content strategies, with `is_root` controlling behavioral differences.

**Simplifies**: Eliminates ~200 lines of near-duplicate code. Bug fixes apply once instead of twice.

---

### #7 — `register_snippet_params` is a hand-written tree walk missing node types

**Dimension**: 8. Scattered ownership / 5. Incidental logic
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/lib.rs:147-190` — manual recursion through Element, ComponentNode, IfBlock, EachBlock, SvelteElement, SvelteBoundary
- `crates/svelte_analyze/src/walker.rs:55-170` — `walk_template` already does the same recursion

**Problem**: This function duplicates the walker's structural recursion but misses `KeyBlock`, `SvelteHead`, `SvelteWindow`, `SvelteDocument`, `SvelteBody`, and `AwaitBlock`. Snippets nested inside those containers would not have their params registered.

**Fix**: Make `register_snippet_params` a `TemplateVisitor` or fold it into `build_scoping`'s walk.

**Simplifies**: Removes 40 lines of hand-written recursion. Automatically handles all container node types.

---

### #8 — Duplicated `item_needs_var` and `item_is_dynamic` across crates

**Dimension**: 6. Dead weight
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/needs_var.rs:59-68` — `item_needs_var` in analysis
- `crates/svelte_codegen_client/src/template/element.rs:160-168` — `item_needs_var` duplicated in codegen
- `crates/svelte_analyze/src/content_types.rs:26-46` — `item_is_dynamic` in analysis
- `crates/svelte_codegen_client/src/template/expression.rs:215-222` — `item_is_dynamic` duplicated in codegen

**Problem**: Two copies of the same match over `FragmentItem` variants, one in analysis and one in codegen, answering the same question using the same data. If a new variant is added, both must be updated.

**Fix**: Move to methods on `FragmentItem` or expose from `svelte_analyze` as public utilities.

**Simplifies**: Single source of truth. One fewer place to update per new variant.

---

### #9 — `ConcatPart` name collision between `svelte_ast` and `svelte_analyze`

**Dimension**: 9. Naming that misleads
**Severity**: warning
**Evidence**:
- `crates/svelte_ast/src/lib.rs:504-509` — `pub enum ConcatPart { Static(String), Dynamic(Span) }`
- `crates/svelte_analyze/src/data.rs:366-371` — `pub enum ConcatPart { Text(String), Expr(NodeId) }`
- `crates/svelte_codegen_client/src/template/expression.rs:5-6` — `use svelte_analyze::ConcatPart; use svelte_ast::ConcatPart as AstConcatPart;`

**Problem**: Two types with the same name in different crates. Codegen must alias one. Readers can't tell which type they're looking at without checking imports.

**Fix**: Rename the analysis-level type to `LoweredConcatPart`.

**Simplifies**: Eliminates the `AstConcatPart` alias and the CODEBASE_MAP warning.

---

### #10 — `ContentStrategy` cloned on every query (copies String for Static variants)

**Dimension**: Performance
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/data.rs:133-134` — `content_type()` returns owned `ContentStrategy` via `.cloned()`
- `crates/svelte_codegen_client/src/context.rs:248` — delegates, called multiple times per fragment

**Problem**: Every content type query clones the `ContentStrategy`. The `Static(String)` variant heap-allocates on every clone. Called multiple times per fragment across codegen.

**Fix**: Return `&ContentStrategy` instead of owned. Use a sentinel `const EMPTY` for missing keys.

**Simplifies**: Eliminates string clones for every static-text fragment query.

---

### #11 — LoweredFragment items cloned before traversal due to borrow conflicts

**Dimension**: Performance
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/template/mod.rs:186,299,375,483` — `.items.clone()` or `.items[0].clone()`
- `crates/svelte_codegen_client/src/template/element.rs:123` — `.items.clone()`

**Problem**: Codegen clones `Vec<FragmentItem>` because `ctx` is borrowed immutably via `lowered_fragment()` but then mutably for code generation. `FragmentItem::TextConcat` contains `Vec<ConcatPart>` with `String`, so each clone allocates.

**Fix**: Separate read-only analysis data from mutable codegen state in `Ctx`, allowing simultaneous borrows.

**Simplifies**: Eliminates one clone per Dynamic/DynamicText fragment.

---

### #12 — Per-expression arena string duplication in parse_js

**Dimension**: Performance
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/parse_js.rs:52,72,97` — `alloc.alloc_str(source)` copies each expression into the OXC arena

**Problem**: Every template expression gets its source text copied into the OXC allocator. The source is already available as a `&str` slice. If the entire source were allocated into the arena once, all expressions could reference subslices.

**Fix**: Allocate the entire component source into the arena once at parse_js start. All expressions reference subslices.

**Simplifies**: Eliminates O(E) allocator copies where E = expression count.

---

### #13 — `ShorthandOrSpread` is a boolean-encoded enum

**Dimension**: 2. Types that lie
**Severity**: suggestion
**Evidence**:
- `crates/svelte_ast/src/lib.rs:511-517` — `ShorthandOrSpread { is_spread: bool }`
- `crates/svelte_codegen_client/src/template/attributes.rs:135-142,745-753` — codegen branches on `is_spread`
- `crates/svelte_analyze/src/element_flags.rs:33-34` — analysis branches on `is_spread`

**Problem**: `ShorthandOrSpread` is always either shorthand or spread after parsing, but the type permits both states. Every consumer branches on a boolean.

**Fix**: Split into `Attribute::Shorthand(ShorthandAttribute)` and `Attribute::SpreadAttribute(SpreadAttribute)`.

**Simplifies**: Removes all `if a.is_spread` guards.

---

### #14 — `ClassDirective.shorthand` and `expression_span` encode redundant state

**Dimension**: 2. Types that lie
**Severity**: suggestion
**Evidence**:
- `crates/svelte_ast/src/lib.rs:519-526` — `ClassDirective { expression_span: Option<Span>, shorthand: bool }`
- `crates/svelte_codegen_client/src/template/attributes.rs:225-233` — codegen checks both

**Problem**: `shorthand: bool` is always `true` when `expression_span` is `None`. Two fields that always agree, permitting impossible states.

**Fix**: Replace with `ClassDirectiveValue::Shorthand | Expression(Span)`. Same for `BindDirective`.

**Simplifies**: Removes double-checks on shorthand + expression_span pairs.

---

### #15 — Repeated `clone_without_fragment` calls to work around borrow conflicts

**Dimension**: 4. Cognitive complexity / Performance
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/template/element.rs:50,66,70` — `el.clone_without_fragment()` called 3 times
- `crates/svelte_ast/src/lib.rs:174-184` — method definition

**Problem**: `process_element` clones element attribute data multiple times because `ctx` must be mutably borrowed by sub-functions. Each clone allocates attribute vectors.

**Fix**: Extract attributes once into an owned struct at the start of `process_element`, then pass to sub-functions.

**Simplifies**: Removes `clone_without_fragment`, eliminates 3 redundant clones in the hot path.

---

### #16 — `rune_transform.rs` in codegen is unused dead code

**Dimension**: 6. Dead weight
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/rune_transform.rs:1-48` — defines `transform_rune_get`, `transform_rune_set`, `transform_rune_update`
- `crates/svelte_codegen_client/src/lib.rs:3` — `mod rune_transform;` declared

**Problem**: These functions duplicate `svelte_transform/src/rune_refs.rs`. The transform crate handles all rune rewrites. In codegen, rune transforms are done inline. This module was likely created before the transform crate existed.

**Fix**: Remove `rune_transform.rs` and its `mod` declaration.

**Simplifies**: Removes 48 lines of dead code.

---

### #17 — `dynamic_nodes` is a flat set that codegen re-interprets per item

**Dimension**: 3. Raw handoff
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/data.rs:211` — `dynamic_nodes: FxHashSet<NodeId>`
- `crates/svelte_codegen_client/src/template/expression.rs:215-232` — `item_is_dynamic` pattern-matches FragmentItem to extract NodeId, checks the set; `parts_are_dynamic` iterates concat parts

**Problem**: Analysis produces `dynamic_nodes` as raw facts. Codegen re-derives "is this item dynamic?" by extracting NodeIds from FragmentItems and checking the set. The conclusion could be pre-computed.

**Fix**: Add `is_dynamic: bool` to `FragmentItem` or a parallel `Vec<bool>` in `LoweredFragment`.

**Simplifies**: `item_is_dynamic` and `parts_are_dynamic` become field reads.

---

### #18 — `ExpressionInfo` stored in full but only partially consumed after analysis

**Dimension**: 3. Raw handoff
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/data.rs:203-205` — `expressions: FxHashMap<NodeId, ExpressionInfo>` stored in full
- `crates/svelte_codegen_client/src/template/component.rs:50-51` — only reads `has_call` and `kind.is_simple()`

**Problem**: After analysis uses `references` for dynamism classification, codegen only reads `has_call` and `kind`. The `references` and `has_side_effects` fields are dead data from codegen's perspective.

**Fix**: Strip `ExpressionInfo` to codegen-relevant fields after reactivity pass, or promote `has_call` to a `FxHashSet<NodeId>`.

**Simplifies**: Reduces memory carried to codegen. Makes phase data needs explicit.

---

### #19 — `TransformData` is a struct with a single field

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_transform/src/data.rs:1-18` — one field: `const_tag_tmp_names: FxHashMap<NodeId, String>`
- Threaded through `compile()`, `generate()`, `Ctx::new()` as a separate parameter

**Problem**: A public type exists to carry a single HashMap between phases. The data is semantically "const tag metadata" which already has a home in `ConstTagData`.

**Fix**: Add `tmp_names` to `ConstTagData`. Remove `TransformData`.

**Simplifies**: Removes a type, its module, and one parameter from 4 function signatures.

---

### #20 — `needs_clsx` computed during `parse_js` by inspecting OXC AST types

**Dimension**: 7. Wrong phase
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/parse_js.rs:275-286` — checks `!matches!(expr, StringLiteral | TemplateLiteral | BinaryExpression)` inside JS parsing pass
- `crates/svelte_analyze/src/data.rs:75` — stored in `ElementFlags`

**Problem**: The `parse_js` pass inspects OXC Expression types for a codegen concern. OXC types should be confined behind `svelte_js`.

**Fix**: Move to `ElementFlagsVisitor` using `ExpressionKind` from `ExpressionInfo`.

**Simplifies**: `parse_js` becomes purely about parsing. OXC types stop leaking.

---

### #21 — `known_values` resolves same name to SymbolId twice

**Dimension**: 10. Implicit coupling
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/known_values.rs:22-23` — first `find_binding`
- `crates/svelte_analyze/src/known_values.rs:49` — second `find_binding` for same name

**Problem**: Redundant double resolution. Comment acknowledges root-scope assumption.

**Fix**: Reuse `sym_id` from first call.

**Simplifies**: One fewer `find_binding` call. Assumption enforced by code structure.

---

### #22 — `gen_bind_directive` has 30+ match arms with repetitive pattern

**Dimension**: 4. Cognitive complexity
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/template/attributes.rs:467-689` — 30+ match arms

**Problem**: Most arms follow the same pattern: build getter + setter, call `$.bind_<name>`. The differences are minor (which runtime fn, needs_getter, extra args). Repetition obscures the few unique cases.

**Fix**: Create a lookup table for the ~25 common cases. Keep explicit arms for `group`, `this`, `indeterminate`/`open`.

**Simplifies**: Reduces function from ~220 lines to ~80. New bind directives become table entries.

---

### #23 — `ComponentScoping::empty()` parses empty JS on every analysis

**Dimension**: Performance
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/scope.rs:55-58` — `empty()` calls `svelte_js::analyze_script_with_scoping("", 0, false)`
- `crates/svelte_analyze/src/data.rs:243` — called in `AnalysisData::new()` for every compilation

**Problem**: Every compilation spins up OXC parser + semantic builder for an empty string. When a script block is present (common case), this is immediately discarded.

**Fix**: Make `scoping` an `Option<ComponentScoping>` or cache the empty scoping via `LazyLock`.

**Simplifies**: Eliminates fixed overhead for the common case.

---

### #24 — 6 separate AST traversals in analysis

**Dimension**: Performance
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/lib.rs:56-111` — parse_js, register_snippet_params, build_scoping, resolve_references, lower, composite walk, needs_var

**Problem**: Analysis performs 6+ tree walks. The composite walker already demonstrates fusing visitors. `register_snippet_params` could be folded into `build_scoping`.

**Fix**: Merge walks where dependencies allow. Start with `register_snippet_params` → `build_scoping`.

**Simplifies**: Reduces traversal overhead proportional to AST size.

---

### #25 — Recursive String allocation in HTML template building

**Dimension**: Performance
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/template/html.rs:41-112` — `element_html()` creates a new `String` per level, concatenated upward
- `crates/svelte_codegen_client/src/template/html.rs:32` — `html.push_str(&element_html(ctx, el))`

**Problem**: Each nesting level creates an intermediate `String` that is immediately concatenated into the parent.

**Fix**: Change to take `&mut String` and write directly into it.

**Simplifies**: Eliminates all intermediate string allocations during HTML building.

---

### #26 — `AssignRight` is a single-variant enum

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/builder.rs:37-39` — `enum AssignRight<'a> { Expr(Expression<'a>) }`
- `crates/svelte_codegen_client/src/builder.rs:400-402` — immediately unwraps

**Problem**: One variant, zero type-safety benefit. 15 characters of noise per call site.

**Fix**: Replace with `Expression<'a>` directly.

**Simplifies**: Removes wrapping/unwrapping at ~15 call sites.

---

### #27 — `svelte_doc_url` matches ~20 variants explicitly for the same return value

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_diagnostics/src/lib.rs:146-176` — positive list of ~20 variants returning `Some(url)`; only 3 exceptions return `None`

**Problem**: The positive list grows with every new variant and is error-prone.

**Fix**: Invert: match only the 3 exceptions returning `None`, default returns `Some(...)`.

**Simplifies**: Match goes from 20+ arms to 3.

---

### #28 — Diagnostic constructor methods are one-to-one wrappers

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_diagnostics/src/lib.rs:204-298` — 18 constructors that are one-liners calling `Self::error(kind, span)`

**Problem**: No logic, no validation. Callers could use `Diagnostic::error(kind, span)` directly.

**Fix**: Remove constructors except `svelte_options_deprecated_tag` (Warning severity) and `internal_error` (zero span).

**Simplifies**: Removes ~90 lines. One fewer place to update per new DiagnosticKind.

---

### #29 — Marker types (`ScopingBuilt`, `ReferencesResolved`) are immediately discarded

**Dimension**: 5. Incidental logic
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/markers.rs:1-15` — two zero-sized types
- `crates/svelte_analyze/src/lib.rs:61-62` — used on consecutive lines, `_refs_resolved` immediately dropped

**Problem**: Markers enforce ordering that is already enforced by sequential call order and `debug_assert!` guards. No possibility of reordering exists.

**Fix**: Remove markers. Sequential calls + existing asserts provide equivalent safety.

**Simplifies**: Removes two types, two `::new()` calls, two bindings.

---

### #30 — String allocations in `IdentGen::gen()`

**Dimension**: Performance
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/ident_gen.rs:16-29` — `gen()` returns `String`, allocates via `format!()` or `to_string()` on every call

**Problem**: Each call allocates a heap String. HashMap keys also allocate. Most identifiers are short ("root_2", "text_1").

**Fix**: Use `CompactString` for keys and return values. Most fit in the 24-byte inline buffer.

**Simplifies**: Eliminates heap allocation per generated identifier.

---

### #31 — No benchmarks exist in any crate

**Dimension**: Performance
**Severity**: warning
**Evidence**:
- No `crates/*/benches/` directories exist
- No criterion benchmarks in the project

**Problem**: Without benchmarks, algorithmic changes cannot be validated. All performance findings are theoretical.

**Fix**: Add criterion benchmarks for parse, analyze, transform, codegen, and end-to-end compile using existing benchmark `.svelte` files.

**Simplifies**: Enables validation of all performance work.

---

## Summary

| Severity | Count |
|----------|-------|
| Critical | 1 |
| Warning | 16 |
| Suggestion | 14 |
| **Total** | **31** |

| Agent | Findings |
|-------|----------|
| Data Flow (1-3) | 8 |
| Simplicity (4-6) | 10 |
| Boundaries (7-10) | 7 |
| Performance | 6 |
