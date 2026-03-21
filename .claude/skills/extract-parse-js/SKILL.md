# Extract JS Parsing from Analyze into Parser

Move all JavaScript parsing out of `svelte_analyze` into `svelte_parser`. After this refactoring, the parser returns everything parsed (template AST + JS expressions + script program), and analyze receives ready trees — zero parsing.

## Goal

**Before:**
```
svelte_parser::parse(source) → Component
svelte_analyze::analyze(alloc, component) → (AnalysisData, ParsedExprs<'a>, Diagnostics)
                                              ↑ does JS parsing internally
```

**After:**
```
svelte_parser::parse_complete(alloc, source) → (Component, JsParseResult<'a>, Diagnostics)
                                                            ↑ all JS already parsed
svelte_analyze::analyze(component, js_result) → (AnalysisData, Diagnostics)
```

`ParsedExprs` and `ExpressionInfo` tables move from analyze output to parser output.

## What is `JsParseResult`

A new struct that bundles everything the JS parsing phase produces. Lives in `svelte_analyze::data` (or a shared crate) — wherever `ParsedExprs` currently lives.

```rust
pub struct JsParseResult<'a> {
    /// Parsed OXC Expression ASTs, keyed by NodeId
    pub parsed: ParsedExprs<'a>,
    /// Per-expression metadata (references, has_call, side_effects)
    pub expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Per-attribute-expression metadata
    pub attr_expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Script analysis result (declarations, exports, rune info)
    pub script: Option<ScriptInfo>,
    /// OXC semantic scoping from script block (raw, before template scopes)
    pub oxc_scoping: Option<oxc_semantic::Scoping>,
    /// Custom element config (parsed from options)
    pub ce_config: Option<ParsedCeConfig>,
    /// Data collected during parse that analyze needs
    pub each_blocks: EachBlockParseData,
    pub const_tag_names: FxHashMap<NodeId, Vec<String>>,
    pub snippet_params: FxHashMap<NodeId, Vec<String>>,
    pub render_tags: RenderTagParseData,
    pub await_bindings: AwaitBindingParseData,
    pub needs_clsx: FxHashSet<NodeId>,
}
```

The exact fields will be determined during implementation — the principle is: everything that `parse_js` currently writes into `AnalysisData` that is a **fact about syntax** (not a semantic decision) moves into `JsParseResult`.

## Execution plan

Work in 6 phases. Each phase must end with `just test-all` passing. **Do NOT proceed to the next phase until all tests pass.**

### Phase 1: Define `JsParseResult` and new parser entry point

1. Read `crates/svelte_analyze/src/data.rs` to understand `ParsedExprs` and all fields that `parse_js` populates in `AnalysisData`
2. Read `crates/svelte_analyze/src/parse_js.rs` fully — understand every field written
3. Define `JsParseResult<'a>` in `svelte_analyze::data` (temporarily — it may move later)
   - Include `ParsedExprs<'a>` and all metadata tables that `parse_js` currently writes to `AnalysisData`
   - Keep `AnalysisData` fields that are populated by analysis passes (reactivity, content_types, etc.)
4. Add `svelte_types` as a dependency of `svelte_parser` in Cargo.toml
5. Add OXC dependencies to `svelte_parser`: `oxc_allocator`, `oxc_ast`, `oxc_semantic`, `oxc_span`
6. Create `svelte_parser/src/parse_js.rs` — for now just re-export or empty module
7. Add a new public function in `svelte_parser`:
   ```rust
   pub fn parse_complete<'a>(alloc: &'a Allocator, source: &str)
       -> (Component, JsParseResult<'a>, Vec<Diagnostic>)
   ```
   Initially: calls the old `Parser::new(source).parse()`, then calls `parse_js` (still in analyze), returns combined result. This is a shim — tests pass because behavior is identical.
8. **Do NOT remove** the old `Parser::new(source).parse()` API yet — it's used by tests
9. Run `just test-all`

### Phase 2: Move `parse_js` code into `svelte_parser`

1. Copy `svelte_analyze/src/parse_js.rs` functions into `svelte_parser/src/parse_js.rs`:
   - `parse_js()` (renamed to avoid collision)
   - `parse_expr()`, `parse_attr_expr()`, `parse_concat_parts()`
   - `walk_fragment()`, `walk_node()`, `walk_attrs()`
   - All helpers these functions call
2. Adapt signatures: instead of writing to `&mut AnalysisData`, write to `&mut JsParseResult`
3. **Do NOT move** `register_arrow_scopes` — it stays in analyze (needs scoping which is built later)
4. Update `parse_complete()` to call the local `parse_js` instead of analyze's
5. Update `svelte_analyze::analyze_with_options()`:
   - Change signature to accept `JsParseResult<'a>` instead of `alloc`
   - Remove internal `parse_js` call
   - Populate `AnalysisData` fields from `JsParseResult` (move/copy the metadata into side tables)
   - `register_arrow_scopes` reads from `js_result.parsed` (ParsedExprs)
6. Update `svelte_compiler::compile()` to use `parse_complete()` → `analyze()`
7. Run `just test-all`

### Phase 3: Update all test call sites

1. Grep for all `analyze(` and `analyze_with_options(` calls across test files
2. Update each to use the new two-step flow:
   - `let (component, js_result, parse_diags) = parse_complete(alloc, source);`
   - `let (analysis, analyze_diags) = analyze(&component, js_result);`
3. Or: provide a convenience function `analyze_source(source) → (Component, AnalysisData)` for tests that combines both steps
4. Run `just test-all`

### Phase 4: Clean up analyze

1. Remove `parse_js` module from `svelte_analyze` (the file, the `mod` declaration)
2. Remove `parse_js::parse_js` function and all its helpers
3. Keep `register_arrow_scopes` — move it to its own module or into `scope.rs`
4. Remove `alloc: &'a Allocator` from analyze's public API if no longer needed
5. Clean up `AnalysisData`: remove fields that now live in `JsParseResult` and are only forwarded
6. Verify `svelte_analyze` no longer depends on `oxc_parser` (it may still need `oxc_ast` for types)
7. Run `just test-all`

### Phase 5: Update transform and codegen

1. `svelte_transform::transform_component` — currently takes `&mut ParsedExprs<'a>`:
   - Now receives it from `JsParseResult` (the caller passes `&mut js_result.parsed`)
   - No signature change needed if caller destructures
2. `svelte_codegen_client::generate` — same: receives `&mut ParsedExprs<'a>` from caller
3. Update `svelte_compiler::compile()` to thread data correctly:
   ```rust
   let (component, mut js_result, mut diags) = svelte_parser::parse_complete(&alloc, source);
   let (analysis, analyze_diags) = svelte_analyze::analyze(&component, &js_result);
   let transform_data = svelte_transform::transform_component(&alloc, &component, &analysis, &mut js_result.parsed, &mut ident_gen);
   let js = svelte_codegen_client::generate(&alloc, &component, &analysis, &mut js_result.parsed, ...);
   ```
4. Run `just test-all`

### Phase 6: Final verification

1. Run `just test-all` — must pass
2. Run `just test-compiler` — JS output must be byte-identical
3. Verify the old `Parser::new(source).parse()` still works (used in parser tests)
4. Run `just compare-benchmark` — no performance regression expected

## Critical invariants

- **JS output must not change.** This is a pure refactoring. All compiler tests must produce identical output.
- **`register_arrow_scopes` stays in analyze.** It needs `ComponentScoping` which is built from OXC scoping + template tree. This is analysis, not parsing.
- **`build_scoping` stays in analyze.** It combines OXC scoping with template-introduced scopes (each, snippet, const, await). This is semantic analysis.
- **`ParsedExprs` ownership:** parser creates it, analyze borrows it (read-only for scoping), transform mutates it, codegen consumes it.
- **`ExpressionInfo` tables:** parser populates them, analyze reads them for reactivity/reference resolution. They may live in `JsParseResult` or be moved into `AnalysisData` during analyze — implementation decides.

## Dependency changes

**svelte_parser gains:**
- `svelte_types` (shared types + OXC utils)
- `oxc_allocator`, `oxc_ast`, `oxc_semantic`, `oxc_span`
- `svelte_analyze` (for `ParsedExprs`, `JsParseResult` types) — OR these types move to a shared location

**Circular dependency risk:** `svelte_parser` → `svelte_analyze` (for types) and `svelte_analyze` → `svelte_parser` (for tests). This is a problem. Solutions:
1. **Preferred:** Move `ParsedExprs`, `JsParseResult`, `ExpressionInfo` types to `svelte_ast` or a new `svelte_types` crate
2. **Alternative:** Move them to `svelte_types` (which both parser and analyze already depend on)
3. **Simplest:** Move them to `svelte_types` since it already defines `ExpressionInfo` and `ScriptInfo`

Evaluate this during Phase 1 and pick the option that minimizes crate changes.

## What NOT to do

- Do not change any analysis pass logic (reactivity, content_types, elseif, etc.)
- Do not change transform logic
- Do not change codegen logic
- Do not rename functions for "cleanliness" — only move code and adjust signatures
- Do not refactor `parse_js` internals (splitting `walk_node` into smaller functions, etc.) — that's a separate task
- Do not change the old `Parser::new(source).parse()` API — keep it for parser-only tests
- If stuck after 3 attempts on the same compilation error, stop and report

## Verification checklist

After each phase:
- [ ] `just test-all` passes
- [ ] `just test-compiler` passes (JS output unchanged)
- [ ] No new warnings from `cargo check`

After all phases:
- [ ] `svelte_analyze` does not contain `parse_js` module
- [ ] `svelte_analyze::analyze()` does not accept `Allocator`
- [ ] `svelte_parser::parse_complete()` returns parsed JS expressions
- [ ] `svelte_compiler::compile()` uses two-step flow: parse → analyze
- [ ] Old `Parser::new(source).parse()` still works for parser-only tests
