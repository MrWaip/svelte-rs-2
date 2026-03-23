# Architecture Review

Full codebase review with performance analysis. 4 agents, 10 dimensions.

---

### #1 — Offset-based implicit coupling between ParsedExprs and AnalysisData

**Dimension**: 10. Implicit coupling
**Severity**: critical
**Evidence**:
- `crates/svelte_analyze/src/js_analyze.rs:377` — `data.node_expr_offsets.insert(node_id, offset)` stores `span.start` as lookup key
- `crates/svelte_analyze/src/data.rs:544-548` — `node_expr_offsets` and `attr_expr_offsets` side tables populated during expression extraction
- `crates/svelte_codegen_client/src/context.rs:274-280` — codegen calls `ctx.node_expr_offset(node_id)` then looks up `ParsedExprs.exprs.get(&offset)`
- `crates/svelte_codegen_client/src/template/expression.rs:19-26` — `get_expr_at()` performs `ctx.parsed.exprs.remove(&offset)` using offset as key

**Problem**: Both phases assume `Span::start` byte offsets uniquely identify expressions in `ParsedExprs.exprs`. There is no type-level contract enforcing this. If the parser populates `ParsedExprs` with offsets from one version of the source and analyze uses offsets from another, lookups silently fail. The offset-based keying is invisible to the type system.

**Fix**: Replace offset-based indirect lookup with direct NodeId keying in `ParsedExprs`. Change `ParsedExprs.exprs` from `FxHashMap<u32, Expression<'a>>` to `NodeTable<Expression<'a>>`, keyed by NodeId. During `parse_js`, store expressions by their corresponding NodeId. Remove `node_expr_offsets` and `attr_expr_offsets` side tables entirely. Codegen calls `ctx.parsed.exprs.get(node_id)` directly.

**Simplifies**: Eliminates offset tracking in two places (parser and analyze), removes the "must match exactly" assumption, makes the dependency explicit in the type system. Codegen no longer needs intermediate offset lookups.

---

### #2 — Bind:this expressions re-parsed from source text heuristics in codegen

**Dimension**: 1. Late knowledge / 7. Wrong phase
**Severity**: critical
**Evidence**:
- `crates/svelte_codegen_client/src/template/component.rs:221` — `ctx.b.parse_expression(&setter_body)` reconstructs setter as string `"{var_name} = $$value"`
- `crates/svelte_codegen_client/src/template/component.rs:231` — `ctx.b.parse_expression(&var_name)` re-parses getter from text
- `crates/svelte_codegen_client/src/template/component.rs:177` — `is_simple_identifier(&var_name)` string heuristic determines code path
- `crates/svelte_codegen_client/src/builder.rs:920-928` — `parse_expression()` calls `OxcParser::new()` in codegen

**Problem**: Whether bind:this target is a simple identifier or member expression is determined by codegen via string heuristics. If simple, it uses `$.set/$.get` wrappers; if not, it constructs setter/getter text and re-parses it. The parser already knows whether the bind expression is an identifier or member expression — this decision should be made once in analyze. Codegen should never contain `OxcParser::new()`.

**Fix**: In analyze, classify each bind:this directive's expression as `Simple(SymbolId)` or `Complex`. Store a precomputed `BindThisMode` enum in AnalysisData. Codegen matches on mode without string parsing or `OxcParser::new()`.

**Simplifies**: Eliminates setter/getter construction via string formatting and re-parsing, removes `is_simple_identifier()` heuristic from codegen, eliminates `OxcParser::new()` usage in codegen entirely.

---

### #3 — Prop default expressions stored as text, re-parsed in codegen

**Dimension**: 1. Late knowledge / 7. Wrong phase
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/script/mod.rs:68` — `b.parse_expression(text)` re-parses prop default values
- `crates/svelte_codegen_client/src/script/mod.rs:156` — same re-parsing in another code path
- `crates/svelte_codegen_client/src/lib.rs:147` — custom element prop defaults re-parsed
- `crates/svelte_analyze/src/data.rs:784-785` — `PropAnalysis` stores `default_text: Option<String>` as raw text

**Problem**: Prop default value expressions are stored as source text in `PropAnalysis` and re-parsed in codegen. The parser already pre-parses expressions; prop defaults should follow the same pattern. Re-parsing in codegen has no access to validation errors and cannot leverage the allocator's lifetime.

**Fix**: In `svelte_parser::parse_with_js`, after extracting `$props()` destructuring, pre-parse default expressions and store them in `ParsedExprs`. Move `default_text` preprocessing to analyze, not codegen.

**Simplifies**: Removes two re-parsing call sites, eliminates need to pass `Builder` into prop default handling, unifies expression handling across all expression types.

---

### #4 — Magic offset arithmetic for ConstTag duplicated across phases

**Dimension**: 9. Naming that misleads / 10. Implicit coupling
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/js_analyze.rs:233-234` — `let ref_offset = tag.declaration_span.start.wrapping_sub(6);`
- `crates/svelte_transform/src/lib.rs:129-130` — same arithmetic duplicated: `let ref_offset = tag.declaration_span.start.wrapping_sub(6);`

**Problem**: The parser stores `ConstTag.declaration_span` starting at the `const` keyword. To find the RHS expression offset, both analyze and transform **subtract 6** (assuming "const " is 6 bytes). This magic number is implicit, fragile (assumes single-space formatting), scattered across two phases, and unnamed (`ref_offset` doesn't explain the derivation).

**Fix**: In the parser, compute and store `const_rhs_expression_offset: u32` alongside the declaration span. Remove `wrapping_sub(6)` from analyze and transform entirely.

**Simplifies**: No magic numbers; parser owns ConstTag structure extraction; single source of truth for the offset.

---

### #5 — Element and attribute cloning in spread codegen path

**Dimension**: 6. Dead weight / Performance
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/template/element.rs:54-71` — spread path clones `el`, `el.name`, and `el.attributes`
- `crates/svelte_codegen_client/src/template/element.rs:68` — clones entire `Attribute` in filter operation
- `crates/svelte_codegen_client/src/template/element.rs:193` — clones lowered fragment items: `ctx.lowered_fragment(&child_key).items.clone()`
- `crates/svelte_codegen_client/src/template/element.rs:53-103` — spread vs non-spread paths duplicate directive dispatch logic

**Problem**: The spread attribute path clones the entire element and attributes Vec when only a few directives need special handling. The same 5 directive types are handled twice: once in spread (filter_map + manual match), once in non-spread (via `process_attr`). This is both a performance issue (O(n*m) cloning where n=elements, m=attributes) and a maintenance issue (new directive types need updating in two places).

**Fix**: Extract a unified directive-processing helper that takes borrowed references. Separate directive pass from attribute pass without cloning:
```rust
let el = ctx.element(el_id);
process_directives(&el.attributes, ...);  // borrows only
process_spread_attrs(&el.attributes, ...); // borrows only
```

**Simplifies**: One canonical directive dispatch instead of two. No Vec clone per spread element. Adding a new directive type updates one place.

---

### #6 — Scattered ownership of attribute output decisions

**Dimension**: 8. Scattered ownership
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/element_flags.rs:102-109` — `ElementFlagsVisitor` computes `needs_memo` via `data.component_attr_needs_memo(a.id)`
- `crates/svelte_analyze/src/element_flags.rs:127` — `is_dynamic` checked and stored separately
- `crates/svelte_codegen_client/src/template/component.rs:46-69` — codegen uses `needs_memo` to decide between thunk+derived, getter, or direct
- `crates/svelte_codegen_client/src/context.rs:244` — `event_handler_mode(attr_id)` shortcut exists for events, but no `attr_output_mode` for all attributes

**Problem**: The decision "does this attribute need memoization?" is split across a helper method in analyze, `ComponentPropInfo` storage, and codegen matching. To add a new output variant, you must touch 3 files in lockstep with no type guiding the coupling.

**Fix**: Create `enum AttrOutputMode { Static, Dynamic, DynamicWithMemo, ... }` in AnalysisData. Precompute the full output mode in `ComponentPropInfo.mode`. Codegen matches directly on the enum.

**Simplifies**: Codegen becomes a flat match (CLAUDE.md's green flag); adding a new variant is guided by type errors; analysis owns all classification in one place.

---

### #7 — Analysis passes run in implicit sequence with hidden data dependencies

**Dimension**: 4. Cognitive complexity
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/lib.rs:40-138` — 12 sequential analysis passes with implicit ordering
- `crates/svelte_analyze/src/lib.rs:177` — `resolve_render_tag_prop_sources` drains `render_tag_arg_idents` (populated by earlier pass)
- `crates/svelte_analyze/src/lib.rs:193` — collects render tag IDs into Vec just to iterate

**Problem**: The analysis pipeline is ordered implicitly by execution sequence. There's no data structure documenting which passes depend on which outputs. If a pass is moved or skipped, downstream passes silently fail or panic. Drained intermediate data (`render_tag_arg_idents`, `render_tag_is_chain`) signals hidden sequencing dependencies not enforced by the type system.

**Fix**: Encode dependencies in documentation or types. At minimum, add comments declaring inputs/outputs per pass. Ideally, each pass returns a typed result that the next pass requires, making reordering a compile error.

**Simplifies**: Pass dependencies become explicit. Reordering produces errors. New contributors understand which passes are safe to modify independently.

---

### #8 — Mutable nested state in Ctx accumulated non-locally

**Dimension**: 4. Cognitive complexity
**Severity**: warning
**Evidence**:
- `crates/svelte_codegen_client/src/context.rs:77,86,98` — `module_hoisted`, `group_index_names`, `delegated_events` accumulate state during codegen
- `crates/svelte_codegen_client/src/lib.rs:42-43,184-189,244` — consumed via `.drain(..)` during final assembly

**Problem**: Non-local data flow. A reader tracing through `gen_root_fragment` sees statements pushed into `ctx.module_hoisted`, then later in `lib.rs` sees `ctx.module_hoisted.drain(..)` without understanding the connection. Mutable shared state across many functions makes reasoning about when data is populated/drained difficult.

**Fix**: Return accumulated data from `gen_root_fragment` instead of mutating Ctx:
```rust
let CodegenResult { hoisted, body, module_hoisted, delegated_events } = gen_root_fragment(&mut ctx);
```
Data flow becomes visible in function signatures.

**Simplifies**: Codegen functions become closer to pure functions. Data flow visible in signatures. Testing doesn't require mocking Ctx state.

---

### #9 — ElementFlags stores 11+ BitSets as feature-specific caches

**Dimension**: 4. Cognitive complexity / 5. Incidental logic
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/data.rs:162-243` — ElementFlags contains 11 BitSets and 5 NodeTables
- Comments at lines 167, 184, 186 — acknowledge this avoids "span→string conversion in codegen" and "re-traversal"

**Problem**: ElementFlags has many separate boolean/bitset fields for related concepts (has_class_attribute, has_class_directives, has_dynamic_class_directives, class_needs_state). This is feature-specific caching rather than a unified classification strategy. Each new feature adds more fields rather than extending an enum.

**Fix**: Consolidate related flags into typed enums. Instead of separate class-related booleans, create `ClassStrategy { None, Static(String), Dynamic, DirectivesOnly, Both }`. Similarly for style. Reduces field count and makes design intent explicit.

**Simplifies**: Fewer fields to remember. Codegen matches on enums rather than checking boolean combinations. Adding new class/style strategies guided by exhaustive match.

---

### #10 — Fragment scope mapping resolved on-demand in transform

**Dimension**: 1. Late knowledge
**Severity**: warning
**Evidence**:
- `crates/svelte_transform/src/lib.rs:99` — `ctx.analysis.scoping.fragment_scope(&FragmentKey::...)` called during transform
- `crates/svelte_analyze/src/walker.rs:93,96,136,140,145,159,165,169` — repeated `fragment_scope()` HashMap lookups during composite walk

**Problem**: Fragment-to-scope mapping is computed on-demand via HashMap lookup. During the composite walk, every fragment boundary triggers a lookup. This should be precomputed in analyze when scoping is built, making scope resolution O(1) table access.

**Fix**: During `build_scoping()`, pre-populate `fragment_scopes: NodeTable<ScopeId>` in AnalysisData. Transform and walker read precomputed table.

**Simplifies**: Transform doesn't call scoping methods; reads a table. Walker scope lookups become O(1). Eliminates repeated HashMap access on hot path.

---

### #11 — ContentStrategy doesn't fully encode the decision for codegen

**Dimension**: 3. Raw handoff
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/template/element.rs:119` — `let has_state = ctx.has_dynamic_children(&child_key);` re-checks dynamics
- `crates/svelte_codegen_client/src/template/element.rs:145` — `let is_text = items.first().is_some_and(|item| item.is_standalone_expr());` re-classifies

**Problem**: `ContentStrategy::DynamicText` is supposed to capture all information codegen needs, but codegen re-checks `has_dynamic_children()` and `is_standalone_expr()`. The strategy enum doesn't fully encode the decision.

**Fix**: Extend `DynamicText` with fields: `DynamicText { has_updates: bool, is_pure_expr: bool }`. Codegen matches without re-inspecting.

**Simplifies**: `ContentStrategy` becomes a true fat enum. No re-inspection in codegen.

---

### #12 — Static text classification allocates intermediate strings

**Dimension**: Performance
**Severity**: warning
**Evidence**:
- `crates/svelte_analyze/src/content_types.rs:174-178` — collects string slices via `.map()`, then joins with `collect::<String>()`

**Problem**: For fragments with many concatenated text parts, this creates O(m) temporary iterator state and intermediate allocations. Called during content classification for every lowered fragment.

**Fix**: Use a single `String` buffer with `.push_str()` loop instead of collecting.

**Simplifies**: Fewer allocations on hot path. Direct string building.

---

### #13 — ParsedExprs consumed destructively, prevents re-use

**Dimension**: 2. Types that lie
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/template/expression.rs:19-21` — `ctx.parsed.exprs.remove(&offset)` takes ownership
- `crates/svelte_codegen_client/src/template/component.rs:158` — similar `.remove()` for bind:this

**Problem**: Destructive removal prevents defensive re-reads or error recovery. If an expression is accidentally consumed twice, only the second call gets a fallback `str_expr("")`. The API permits silent double-consumption.

**Fix**: Use `.get()` for reads and only remove at the final consumption site, or use a checked-remove that panics on double-consume in debug builds.

**Simplifies**: Better error messages, safer refactoring, clearer ownership semantics.

---

### #14 — LoweredTextPart and ConcatPart are parallel types for the same concept

**Dimension**: 2. Types that lie
**Severity**: suggestion
**Evidence**:
- `crates/svelte_ast/src/lib.rs:104` — `enum ConcatPart { Static(String), Dynamic(Span) }` in AST
- `crates/svelte_analyze/src/data.rs:747-754` — `enum LoweredTextPart { TextSpan(Span), TextOwned(String), Expr(NodeId) }` in analyze
- `crates/svelte_codegen_client/src/template/expression.rs:136-149` — codegen converts between them

**Problem**: Two separate types for the same domain concept (text-or-expression parts). The migration from `ConcatPart` to `LoweredTextPart` happens in the lowering pass. Readers must understand both types and their relationship.

**Fix**: Unify on `LoweredTextPart`. Have parser produce `Dynamic(Span)` early and convert during lowering.

**Simplifies**: Single type across phases, clearer intent.

---

### #15 — Tuple composite visitor requires 43-line macro with 5 instantiations

**Dimension**: 4. Cognitive complexity
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/walker.rs:187-250+` — macro delegates all TemplateVisitor methods to tuple elements
- Instantiated for 2-tuple through 6-tuple with repetitive code

**Problem**: If a new visitor method is added to the trait, the macro must be updated. The repetition suggests the pattern strains at its abstraction boundaries. Most Rust codebases use a simpler approach.

**Fix**: Accept a slice of trait objects `&mut [&mut dyn TemplateVisitor]`, or use a builder that composes visitors without macros.

**Simplifies**: No macro boilerplate. Adding a new visitor method updates trait definition only. Composite visitor size is unbounded.

---

### #16 — FragmentKey enum flattens a hierarchical structure into 15 variants

**Dimension**: 6. Dead weight
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/data.rs:68-85` — FragmentKey has 15 variants, many parallel (IfConsequent/IfAlternate, EachBody/EachFallback)

**Problem**: The enum conflates leaf nodes (Element, ComponentNode) with block sub-fragments (IfConsequent). The reader doesn't immediately see that EachBody and EachFallback are children of EachBlock.

**Fix**: Use a nested enum: `Block { node_id, variant: BlockVariant }` vs `Leaf { node_id, kind: LeafKind }`. Impossible combinations become unrepresentable.

**Simplifies**: Structure mirrors AST hierarchy. Pattern matching is more natural.

---

### #17 — Ctx::get_node uses function pointer indirection for type extraction

**Dimension**: 4. Cognitive complexity
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/context.rs:151-170` — all node accessors call `get_node(id, label, extract_fn)` where extract_fn is a function pointer like `Node::as_element`

**Problem**: Function pointer indirection for a simple AST type hierarchy. The reader must understand `Node::as_element` is passed as a function pointer and invoked via the type system. Overly generic for what is a single-level enum extraction.

**Fix**: Use direct match or a macro per node type. Simpler, no function pointer indirection.

**Simplifies**: No function pointer indirection. Type-safe by construction.

---

### #18 — Vec allocations for single-iteration collections in analyze

**Dimension**: Performance
**Severity**: suggestion
**Evidence**:
- `crates/svelte_analyze/src/content_types.rs:40-43` — collects fragment keys into Vec, iterates once
- `crates/svelte_analyze/src/lib.rs:193` — `let all_ids: Vec<NodeId> = data.render_tag_arg_has_call.keys().collect();` collected just to iterate

**Problem**: Temporary Vec allocations for data that is iterated exactly once. Could iterate directly without materializing.

**Fix**: Replace `.collect::<Vec<_>>()` with direct iterator consumption.

**Simplifies**: Eliminates unnecessary allocations. Simpler code.

---

### #19 — format!() in const declaration wrapping allocates before alloc copy

**Dimension**: Performance
**Severity**: suggestion
**Evidence**:
- `crates/svelte_parser/src/parse_js.rs:98` — `let wrapped_owned = format!("const {};", source);` then `alloc.alloc_str(&wrapped_owned)`

**Problem**: Creates temporary owned String via `format!()`, then copies to allocator. Double allocation.

**Fix**: Use `String::with_capacity(source.len() + 8)` with manual `push_str`.

**Simplifies**: One fewer allocation per const tag.

---

### #20 — Store subscription name cloned unnecessarily

**Dimension**: Performance
**Severity**: suggestion
**Evidence**:
- `crates/svelte_codegen_client/src/lib.rs:91-99` — `dollar_name.clone()` when passed to `Arg::Str()`

**Problem**: `dollar_name` is cloned when it could be moved or allocated once.

**Fix**: Use `.into()` or allocate via builder.

**Simplifies**: One fewer clone per store subscription.

---

## Summary

| Severity | Count |
|----------|-------|
| Critical | 2 |
| Warning | 8 |
| Suggestion | 10 |
| **Total** | **20** |

| Agent | Findings |
|-------|----------|
| Data Flow | 7 (#1, #2, #3, #10, #11, #13, #14) |
| Simplicity | 6 (#5, #7, #8, #9, #15, #16, #17) |
| Boundaries | 3 (#4, #6, merged into #1, #2, #3) |
| Performance | 5 (#5, #12, #18, #19, #20) |

**Top 3 for `/review-fix`**:
1. **#1** — Offset-based implicit coupling between ParsedExprs and AnalysisData
2. **#2** — Bind:this expressions re-parsed from source text heuristics in codegen
3. **#3** — Prop default expressions stored as text, re-parsed in codegen
