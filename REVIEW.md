# Architecture Review

Full codebase review. 3 agents: Data Flow, Simplicity, Boundaries.

---

### #1 — `ShorthandOrSpread` conflates two semantically distinct attribute kinds

**Dimension**: 2. Types that lie
**Severity**: warning
**Evidence**:
- `crates/svelte_ast/src/lib.rs:510-516` — `ShorthandOrSpread { expression_span, is_spread: bool }`
- `crates/svelte_codegen_client/src/template/attributes.rs:137` — `Attribute::ShorthandOrSpread(a) if !a.is_spread`
- `crates/svelte_codegen_client/src/template/attributes.rs:757` — `Attribute::ShorthandOrSpread(a) if a.is_spread`
- `crates/svelte_codegen_client/src/template/component.rs:64-65` — two guard branches on `a.is_spread`

**Problem**: `{name}` (shorthand attribute) and `{...obj}` (spread) are packed into a single variant distinguished by a boolean. Every consumer must use guard patterns (`if a.is_spread` / `if !a.is_spread`), splitting what should be two distinct match arms. The type system allows a "shorthand spread" state that is semantically nonsensical.

**Fix**: Split into `Attribute::Shorthand(ShorthandAttribute)` and `Attribute::Spread(SpreadAttribute)` with different fields. A shorthand has a name; a spread has only an expression.

**Simplifies**: Every match on `Attribute` becomes exhaustive without guard clauses. Code that only handles spreads or only handles shorthands becomes a single clean match arm.

---

### #2 — Event detection via `strip_prefix("on")` repeated in 4 codegen sites

**Dimension**: 1. Late knowledge / 3. Raw handoff
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/template/attributes.rs:65` — `a.name.strip_prefix("on")` to detect event attributes
- `crates/svelte_codegen_client/src/template/svelte_window.rs:33` — same pattern
- `crates/svelte_codegen_client/src/template/svelte_body.rs:30` — same pattern
- `crates/svelte_codegen_client/src/template/svelte_document.rs:33` — same pattern

**Problem**: Whether an `ExpressionAttribute` is an event handler (`onclick`, `onfocus`) is determined by codegen via `name.strip_prefix("on")`. This classification should happen during analysis or at the AST level. Instead, codegen re-derives this knowledge from the raw name string at every call site.

**Fix**: Either split `ExpressionAttribute` into `EventAttribute` and `ExpressionAttribute` at the AST level, or add an `is_event: bool` flag computed by the parser. Alternatively, mark event attributes in `ElementFlags` during analysis.

**Simplifies**: Four separate `strip_prefix("on")` sites in codegen collapse into a type-directed match. Event-specific logic (delegation, capture, passive) moves to a dedicated handler without prefix-stripping boilerplate.

---

### ~~#3 — Rune transformation logic duplicated across transform and codegen~~ ✅ FIXED

**Dimension**: 8. Scattered ownership
**Severity**: warning
**Evidence**:
- `crates/svelte_transform/src/rune_refs.rs:9-76` — `make_rune_get`, `make_rune_set`, `make_rune_update` using raw `AstBuilder`
- `crates/svelte_codegen_client/src/rune_transform.rs:8-48` — `transform_rune_get`, `transform_rune_set`, `transform_rune_update` using `Builder`
- `crates/svelte_codegen_client/src/script.rs:1017` — codegen calls `crate::rune_transform::*`
- `crates/svelte_transform/src/lib.rs:287-298` — transform calls `rune_refs::make_rune_get`

**Problem**: Two independent implementations produce identical `$.get(name)` / `$.set(name, value)` / `$.update(name)` AST nodes. One in `svelte_transform` for template expressions, one in `svelte_codegen_client` for script expressions. To change the calling convention of `$.set`, both must be updated.

**Fix**: Move the canonical rune-transform AST builders to a single location. Either share through a common utility crate or have `svelte_transform` expose the builders and have codegen import them.

**Simplifies**: Eliminates the second implementation (~49 lines). Single place to update when rune calling convention changes.

---

### #4 — `const_aliases` stack replicates scope semantics already in `ComponentScoping`

**Dimension**: 5. Incidental logic
**Severity**: warning
**Evidence**:
- `crates/svelte_transform/src/lib.rs:60` — `const_aliases: Vec<FxHashMap<String, (String, String)>>`
- `crates/svelte_transform/src/lib.rs:101-108` — `with_alias_scope` pushed for IfBlock, EachBlock, SnippetBlock, KeyBlock, SvelteHead, SvelteBoundary, AwaitBlock
- `crates/svelte_transform/src/lib.rs:143-154` — ConstTag registers aliases into the top of the stack

**Problem**: The `const_aliases` stack is a parallel scope system. It mirrors the nesting structure of the template just like `ComponentScoping` already does. The alias information could be stored as symbol metadata in `ComponentScoping` during analysis — each destructured const binding already gets a symbol via `scoping.add_binding()`. Instead, a second stack-based scope system is maintained during transform, requiring careful push/pop pairing for every scope-introducing node type.

**Fix**: During the analysis `build_scoping` pass, when destructured const bindings are created, also store the alias info (tmp_var, prop_name) on the SymbolId. Then `transform_expr` can look it up via `find_binding(scope, name)` + metadata query. No parallel scope stack needed.

**Simplifies**: Removes the `const_aliases` stack, the `with_alias_scope` helper, and 7 push/pop call sites. Transform becomes purely scope-lookup-based.

---

### #5 — Render tag codegen re-derives prop-source status from post-transform AST ✅ FIXED

**Dimension**: 7. Wrong phase, wrong abstraction level
**Severity**: warning
**Status**: Fixed — per-arg prop-source names pre-computed in analysis (`render_tag_prop_sources`), codegen reads flags directly.

**Simplifies**: The complex `prop_getter_name` detection block (15 lines of pattern matching + scoping resolution) becomes a simple flag check.

---

### #6 — `is_store_ref` implemented independently in 3 locations

**Dimension**: 8. Scattered ownership
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/reactivity.rs:165-172` — free function `is_store_ref(name, data)` checks `data.scoping`
- `crates/svelte_codegen_client/src/script.rs:386-394` — method `ScriptTransformer::is_store_ref(&self, name)` checks `self.component_scoping`
- `crates/svelte_transform/src/lib.rs:295-301` — inline `$`-prefix check + `find_binding` + `is_store(s)`

**Problem**: Three places independently implement "is this `$name` a store subscription?" using the same pattern: strip `$` prefix, look up base name in root scope, check `is_store`. If the store detection logic changes, three sites need updating.

**Fix**: Add `is_store_ref(name: &str) -> bool` method directly on `ComponentScoping`. All three call sites import and call the same method.

**Simplifies**: Single point of change for store reference classification. Removes 3 independent re-implementations.

---

### ~~#7 — Bind directive variable names re-extracted from source text in codegen~~ ✅

**Dimension**: 1. Late knowledge
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/template/attributes.rs:471-477` — `gen_bind_directive` reads `source_text(span).trim()` to get the bind target variable name
- `crates/svelte_codegen_client/src/template/svelte_window.rs:54-60` — same pattern in `gen_window_binding`
- `crates/svelte_codegen_client/src/template/svelte_document.rs:54-60` — same pattern in `gen_document_binding`

**Problem**: The bind directive's target variable name is reconstructed from raw source text at codegen time. The parser already parsed this expression and knows the identifier name, but only stores it as a `Span`. Analysis also resolves this identifier via `BindSemanticsData`, but doesn't store the resolved name.

**Fix**: Store the resolved variable name in `BindDirective` at parse time, or in `BindSemanticsData` during analysis.

**Simplifies**: Every `gen_bind_directive` / `gen_window_binding` / `gen_document_binding` call site can stop re-extracting `source_text(span).trim()`.

---

### ~~#8 — Each-block context/index names re-extracted from source text~~ ✅

**Dimension**: 1. Late knowledge
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/template/each_block.rs:33` — `ctx.component.source_text(context_span).to_string()`
- `crates/svelte_codegen_client/src/template/each_block.rs:60` — `ctx.component.source_text(span).to_string()` for index name
- `crates/svelte_analyze/src/scope.rs:323-330` — `build_scoping` already extracts these same names from source text

**Problem**: The context variable name and index name are extracted from source text twice: once in `build_scoping` to create scope bindings, and again in codegen. The parser stores these as `Span` rather than resolved names, even though they are simple identifiers.

**Fix**: Store the context and index names as `String` fields on `EachBlock` during parsing, or as analysis side-table data during `build_scoping`.

**Simplifies**: Eliminates `source_text()` calls in `each_block.rs` codegen and makes the data flow explicit.

---

### ~~#9 — `bind:this` setter/getter uses string manipulation then re-parses as JS~~ ✅

**Dimension**: 1. Late knowledge
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/template/component.rs:302-303` — `format!("{expr_text} = $$value")` then `ctx.b.parse_expression(&setter_body)`
- `crates/svelte_codegen_client/src/template/component.rs:314-315` — `add_optional_chaining(expr_text)` then `ctx.b.parse_expression(&getter_text)`

**Problem**: For `bind:this={obj.ref}` on components, codegen takes raw expression text, performs string manipulation (appending `= $$value`, inserting `?.`), and re-parses the result as JS. The `add_optional_chaining` function is a hand-rolled parser for member expressions that doesn't handle edge cases. The transform phase already has the parsed AST for this expression.

**Fix**: Generate the getter and setter OXC ASTs in the transform phase from the already-parsed bind expression AST, at the AST level rather than string level.

**Simplifies**: Eliminates two `parse_expression` calls in codegen, removes the fragile `add_optional_chaining` string function.

---

### ~~#10 — `extract_binding_names` does hand-rolled parsing of destructuring patterns~~ ✅

**Dimension**: 10. Implicit coupling
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/scope.rs:284-306` — `extract_binding_names` does naive string splitting (`split(',')`, `split('=')`)
- `crates/svelte_js/src/lib.rs:151-175` — `parse_snippet_params` does the same job using OXC parser (correct approach)

**Problem**: `extract_binding_names` implements a hand-rolled parser for destructuring patterns using string splitting. This is implicitly coupled with the parser's output format: it assumes simple comma separation. JS destructuring can have nested patterns, computed keys, and rest elements that this approach mishandles. Meanwhile, `parse_snippet_params` in `svelte_js` handles this correctly through OXC.

**Fix**: Use `svelte_js::parse_snippet_params` (or a shared OXC-based function) for await block bindings too.

**Simplifies**: Removes the hand-rolled parser (22 lines). Eliminates correctness risk for non-trivial destructuring patterns.

---

### #11 — `PropsGenInfo` construction duplicated in two codepaths ✅ FIXED

**Dimension**: 8. Scattered ownership
**Severity**: warning
**Status**: Fixed — extracted `PropsGenInfo::from_analysis()` constructor, both call sites use it.

**Simplifies**: Removes ~20 duplicated lines. Single place to modify prop flag computation.

---

### #12 — Five independent tree walkers with near-identical structure

**Dimension**: 4. Cognitive complexity
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/walker.rs:55` — `walk_template` (visitor-based, analysis)
- `crates/svelte_analyze/src/scope.rs:308` — `walk_template_scopes` (scope building)
- `crates/svelte_analyze/src/lib.rs:159` — `register_snippet_params` (snippet registration)
- `crates/svelte_analyze/src/parse_js.rs:117` — `walk_fragment` (expression parsing)
- `crates/svelte_transform/src/lib.rs:68` — `walk_fragment` (expression transform)
- `crates/svelte_codegen_client/src/context.rs:54` — `NodeIndex::walk` (index building)

**Problem**: Six hand-written recursive walkers each implement the same structural recursion over the `Node` enum. When a new node type is added, every walker must be updated independently. The `register_snippet_params` walker is particularly fragile — it enumerates all node variants manually with a 26-arm match just to find `SnippetBlock` nodes.

**Fix**: `register_snippet_params` and `walk_template_scopes` could both be implemented as `TemplateVisitor` implementations combined into the existing composite walk, eliminating two of the six independent walkers.

**Simplifies**: Removes two separate full-tree traversals and their independent Node variant matching.

---

### ~~#13 — `is_simple_identifier` duplicated between analysis and codegen~~ ✅ FIXED

**Dimension**: 8. Scattered ownership
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/bind_semantics.rs:38-42` — `BindSemanticsVisitor::is_simple_identifier`
- `crates/svelte_codegen_client/src/template/component.rs:215-219` — standalone `is_simple_identifier`

**Problem**: The exact same function appears in two crates with identical logic. Both are used to decide whether `bind:this` needs member-access handling. If one is updated, the other silently diverges.

**Fix**: Move `is_simple_identifier` to `svelte_js` and import it in both locations.

**Simplifies**: Removes the duplicated function. Ensures consistent identifier classification.

---

### #14 — `NodeIndex` builds 14 separate `FxHashMap`s at codegen start

**Dimension**: 4. Cognitive complexity
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/context.rs:13-29` — 14 separate `FxHashMap` fields
- `crates/svelte_codegen_client/src/context.rs:54-131` — full tree walk populating all 14 maps
- `crates/svelte_codegen_client/src/context.rs:202-213` — 12 lookup methods that are identical except for which map they query

**Problem**: `NodeIndex` walks the entire AST once to populate 14 typed HashMaps. Each accessor is O(1) but the cost is paid upfront for all node types, even unused ones. The 12 nearly-identical accessor methods are boilerplate.

**Fix**: Use a single `FxHashMap<NodeId, &'a Node>` with typed accessors that use the existing `Node::as_*()` methods. 14 maps become 1.

**Simplifies**: The walk inserts one entry per node instead of dispatching. The 12 accessors become thin wrappers around `Node::as_*()`.

---

### #15 — `PropsAnalysis.is_prop_source` duplicates `ComponentScoping.prop_source_syms`

**Dimension**: 2. Types that lie
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/data.rs:437` — `PropAnalysis.is_prop_source: bool`
- `crates/svelte_analyze/src/scope.rs:179-181` — `ComponentScoping.prop_source_syms: FxHashSet<SymbolId>` + `mark_prop_source()`

**Problem**: The "is this prop a source?" fact is stored in two places: on `PropAnalysis` and on `ComponentScoping`. Both are set during the `props` pass. They must stay in sync, but nothing enforces this.

**Fix**: Remove `PropAnalysis.is_prop_source` and always query through `ComponentScoping.is_prop_source()`, which is the canonical SymbolId-keyed source of truth per project conventions.

**Simplifies**: Eliminates a redundant truth source and removes the risk of inconsistency.

---

### #16 — `classify_bind` uses source text for semantic decisions despite SymbolId principle

**Dimension**: 9. Naming that misleads
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/bind_semantics.rs:87-99` — `classify_bind` extracts variable name from `self.source[span.start..span.end].trim()` then calls `is_mutable_rune(name, data)` which does `find_binding(root, name)`

**Problem**: `classify_bind` is documented as pre-computing semantics to avoid string-based lookups in codegen. But internally it performs its own string-based lookup: it reads source text from a span, extracts a name string, then does `find_binding(root, name)`. The architectural principle says "String-based membership tests are forbidden for semantic decisions."

**Fix**: Use the pre-parsed `ExpressionInfo` (which has `references` with `symbol_id`) to determine if the target is a mutable rune, rather than re-extracting text from source.

**Simplifies**: Eliminates source-text-based name resolution in analysis. Makes `classify_bind` truly work at the SymbolId level.

---

### #17 — `AssignRight` is a single-variant enum

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/builder.rs:37-39` — `enum AssignRight<'a> { Expr(Expression<'a>) }`
- `crates/svelte_codegen_client/src/builder.rs:400-402` — only usage: `let right = match right { AssignRight::Expr(e) => e, };`
- 14 call sites all pass `AssignRight::Expr(...)`

**Problem**: `AssignRight` has exactly one variant. Every caller wraps an `Expression` in `AssignRight::Expr(...)`, and the receiver immediately unwraps it. Pure ceremony.

**Fix**: Replace `AssignRight<'a>` with plain `Expression<'a>` in `assign_stmt` and `assign_expr` signatures.

**Simplifies**: Removes the wrapper type, the match, and `AssignRight::Expr(...)` at all 14+ call sites.

---

### #18 — `Component::next_node_id` field is set but never read

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_ast/src/lib.rs:27` — `next_node_id: u32` field
- `crates/svelte_ast/src/lib.rs:43-45` — `pub fn set_next_node_id(&mut self, id: u32)`

**Problem**: `next_node_id` is stored on `Component` and has a setter, but no getter and no reader. After parsing sets it, nothing uses it.

**Fix**: Remove `next_node_id` and `set_next_node_id` from `Component`.

**Simplifies**: Removes a dead field and its setter method.

---

### #19 — `ReferencesResolved` marker type is unused beyond construction

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/markers.rs:7` — `pub(crate) struct ReferencesResolved(());`
- `crates/svelte_analyze/src/lib.rs:73` — `let _refs_resolved = resolve_references(...)` — bound to `_` and discarded

**Problem**: `ReferencesResolved` is a zero-sized witness type meant to enforce pass ordering. It is returned by `resolve_references()` but immediately discarded. No downstream function requires it as a parameter. It enforces nothing.

**Fix**: Either remove it or wire it into a pass that logically depends on resolved references.

**Simplifies**: Removes a phantom type that provides false security.

---

### #20 — `analyze_expression` (allocator-owning variant) is test-only dead code

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_js/src/lib.rs:270-281` — `pub fn analyze_expression(source, offset)` creates its own `Allocator`
- No production caller exists; only used in tests within `svelte_js`

**Problem**: The production path uses `analyze_expression_with_alloc`. The self-allocating variant is a test convenience function that still carries a `pub` API surface.

**Fix**: Mark it `#[cfg(test)]` or move it to the test module.

**Simplifies**: Clarifies the public API — one entry point for expression analysis, not two.

---

### #21 — `transform_expr` shadow parameter threaded through entire tree unnecessarily

**Dimension**: 4. Cognitive complexity
**Severity**: suggestion
**Evidence**:
- `crates/svelte_transform/src/lib.rs:224` — `transform_node_expr` creates `&mut Vec::new()` for shadow
- `crates/svelte_transform/src/lib.rs:269-273` — `transform_expr` receives `shadow: &mut Vec<FxHashSet<String>>`
- `crates/svelte_transform/src/lib.rs:386-397` — only `ArrowFunctionExpression` actually pushes/pops the shadow stack

**Problem**: The `shadow` parameter (a stack of name sets for arrow function parameter shadowing) is allocated fresh at every `transform_node_expr` and `transform_attrs` call, then threaded through the entire expression tree. But it only has content inside arrow functions. For non-arrow expressions, it's an empty Vec being passed through 10+ recursive calls.

**Fix**: Move `shadow` into `TransformCtx` as a field instead of a parameter. Push/pop in the `ArrowFunctionExpression` arm.

**Simplifies**: Removes one parameter from `transform_expr`, `walk_expr_children`, and `transform_stmt`. Eliminates per-node `Vec::new()` allocations.

---

### #22 — `static_class`/`static_style` stored as String instead of Span

**Dimension**: 7. Wrong phase, wrong abstraction level
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/element_flags.rs:46-49` — `self.source_text(sa.value_span).to_string()` stored in `static_class`/`static_style`
- `crates/svelte_analyze/src/data.rs:77-80` — `static_class: FxHashMap<NodeId, String>`, `static_style: FxHashMap<NodeId, String>`

**Problem**: Analysis converts `Span` to owned `String` for static class/style values. The rest of the architecture stores `Span` and lets codegen read source text on demand. These two fields break the span-based pattern by materializing strings during analysis.

**Fix**: Store `Span` instead of `String`. Codegen already has `ctx.component.source_text(span)` available.

**Simplifies**: Eliminates two `to_string()` allocations per element during analysis. Consistent with span-based architecture.

---

### #23 — `ContentStrategy::Static` pre-joins text, embedding output format in analysis

**Dimension**: 7. Wrong phase, wrong abstraction level
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/content_types.rs:97-106` — concatenates text parts into a single `String` stored in `ContentStrategy::Static(text)`

**Problem**: Analysis produces a pre-joined string for static text content. This means analysis is making an output-format decision: "static text fragments should be concatenated into one string." The lowered fragment already has the individual parts — codegen could join them itself.

**Fix**: Change `ContentStrategy::Static` to not carry data (or carry a reference to the lowered fragment). Let codegen concatenate text parts when rendering.

**Simplifies**: Analysis no longer produces output-format-specific data. `ContentStrategy` becomes a pure classification enum.

---

### #24 — `Ctx` shortcuts layer over-proxies simple two-level chains

**Dimension**: 4. Cognitive complexity
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/context.rs:233-282` — 20+ methods like `has_spread`, `has_class_directives`, `needs_var`, etc.
- Each is a one-line delegation: `pub fn has_spread(&self, id: NodeId) -> bool { self.analysis.element_flags.has_spread(id) }`

**Problem**: `Ctx` has 20+ shortcut methods that each delegate to one `AnalysisData` sub-struct method. These aren't simplifying a complex access path — they're adding a second name for the same thing. A caller reading `ctx.has_spread(id)` must still look up what it means, but the definition is in a different file from where the logic lives.

**Fix**: Keep shortcuts only where the chain is genuinely painful (3+ levels) or adds semantic value. For simple two-level chains like `ctx.analysis.element_flags.has_spread(id)`, direct access is equally readable.

**Simplifies**: Reduces the surface of `Ctx` from 30+ methods to ~10 meaningful ones.

---

### #25 — `ElementKind` removed from code but still in CODEBASE_MAP.md

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `CODEBASE_MAP.md:105` — lists `enum ElementKind { Unknown, Input }`
- Zero hits for `ElementKind` in actual source code

**Problem**: `ElementKind` was removed from the AST but the codebase map still references it, creating confusion.

**Fix**: Remove `ElementKind` from `CODEBASE_MAP.md`.

**Simplifies**: Accurate documentation.

---

### #26 — `ConstTagData` docs mismatch: `tmp_names` documented but lives in `TransformData`

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/data.rs:165-180` — `ConstTagData` has no `tmp_names` field
- `CODEBASE_MAP.md:266` — documents `tmp_names` as a field of `ConstTagData`

**Problem**: CODEBASE_MAP says `ConstTagData` has `tmp_names`, but it actually lives on `TransformData`. Documentation is wrong.

**Fix**: Update CODEBASE_MAP to accurately reflect that `tmp_names` lives in `TransformData`.

**Simplifies**: Documentation accuracy.

---

### #27 — `ReactivityVisitor::new()` constructs a unit struct

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/reactivity.rs:10-11` — `pub(crate) struct ReactivityVisitor;`
- `crates/svelte_analyze/src/reactivity.rs:13-15` — `pub(crate) fn new() -> Self { Self }`

**Problem**: `ReactivityVisitor` is a unit struct. The `new()` constructor is `Self`. A constructor that does nothing for a type that holds nothing.

**Fix**: Use `ReactivityVisitor` directly at the call site instead of `ReactivityVisitor::new()`.

**Simplifies**: Removes a trivial constructor.
