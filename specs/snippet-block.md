# SnippetBlock

## Current state
- **Working**: 13/14 use cases — all basic snippets plus full parameter destructuring (object, array, defaults, rest, mixed)
- **Missing**: use case 14 — `snippet_parameter_assignment` validation (Tier 5b, deferred)
- **Next**: Tier 5 diagnostics or mark feature complete
- Last updated: 2026-04-02

## Source
ROADMAP Tier 2b: `{#snippet}` — parameter destructuring

## Use cases

### Basic structure
1. [x] No parameters: `{#snippet foo()}` → `($$anchor) => { ... }` (test: snippet_basic)
2. [x] Simple identifier params: `{#snippet foo(a, b)}` → `($$anchor, a = $.noop, b = $.noop) => { ... }` (test: snippet_basic)
3. [x] Hoisted snippet (top-level, no instance refs) → module-level declaration (test: snippet_basic)
4. [x] Non-hoisted snippet (references instance vars) → instance-level declaration (test: snippet_ident_conflict_with_script)
5. [x] Nested snippet (inside block/element) → local declaration (test: boundary_const_in_snippet)
6. [x] Snippet as component prop → passed as named prop (test: component_snippet_prop)
7. [x] Dev mode: `$.wrap_snippet(Name, function(...) { $.validate_snippet_args(...arguments); ... })` (test: tag_snippet_dev)

### Parameter destructuring
8. [x] Object destructuring: `{#snippet foo({ x, y })}` → `$$arg0` param + `let x = () => $$arg0?.().x` (test: snippet_object_destructure)
9. [x] Object destructuring with defaults: `{#snippet foo({ x = 5 })}` → `$.derived_safe_equal(() => $.fallback(...))` (test: snippet_object_destructure)
10. [x] Object rest: `{#snippet foo({ x, ...rest })}` → `$.exclude_from_object($$arg0?.(), ['x'])` (test: snippet_object_destructure)
11. [x] Array destructuring: `{#snippet foo([a, b])}` → `$.to_array($$arg0?.(), 2)` + derived intermediary (test: snippet_array_destructure)
12. [x] Array destructuring with rest: `{#snippet foo([a, ...rest])}` → `$.get($$array).slice(1)` (test: snippet_array_destructure)
13. [x] Mixed params: `{#snippet foo(a, { x }, [b])}` → identifier + object + array in one signature (test: snippet_mixed_params)

### Validation (Tier 5)
14. [ ] `snippet_parameter_assignment` — error on assignment to snippet param (deferred to Tier 5b)

### Deferred
- [ ] Nested object destructuring in snippet params: `{#snippet foo({ a: { b } })}` (silently skipped, binding lost — codegen produces wrong output)
- [ ] Nested array destructuring in snippet params: `{#snippet foo({ a: [x, y] })}` (same)
- SSR snippet codegen
- `snippet_invalid_rest_parameter` validation (rest params in snippet are an error in reference)
- `snippet_shadowing_prop` / `snippet_conflict` validation (Tier 5)
- `snippet_invalid_export` validation (Tier 5)

## Reference

### Svelte (reference compiler)
- `reference/compiler/phases/3-transform/client/visitors/SnippetBlock.js` — parameter dispatch, `extract_paths` usage, dev wrapping
- `reference/compiler/utils/ast.js` lines 243–415 — `extract_paths` / `_extract_paths`: recursive destructuring → inserts (array intermediaries) + paths (leaf bindings)
- `reference/compiler/utils/ast.js` lines 585–597 — `build_fallback`: default value wrapping with `$.fallback()`
- `reference/compiler/phases/scope.js` lines 1331–1346 — snippet param declared as `kind: 'snippet'`
- `reference/compiler/phases/2-analyze/visitors/SnippetBlock.js` — hoistability, validation

### Our code
- `crates/svelte_codegen_client/src/template/snippet.rs` — `gen_snippet_block`, `build_snippet_params` (flat params only)
- `crates/svelte_analyze/src/passes/template_side_tables.rs:148` — `SnippetParamMarker`, `SnippetParamNameCollector`
- `crates/svelte_analyze/src/types/data/template_data.rs` — `SnippetData` (stores flat `Vec<String>` of leaf names)
- `crates/svelte_parser/src/parse_js.rs:184` — `parse_snippet_decl_with_alloc` (OXC parses patterns correctly)

## Tasks

### Analysis
- [ ] Preserve original parameter patterns in `SnippetData` — store `Vec<SnippetParam>` enum (Identifier / ObjectPattern / ArrayPattern) instead of flat `Vec<String>`. OXC already parses these; analyze just needs to preserve the shape info.
- [ ] Alternative: skip shape info in analyze, use `parsed.stmts` directly in codegen (the arrow params are already parsed by OXC). Codegen can walk the `FormalParameters` from the parsed arrow to reconstruct declarations.

### Codegen
- [ ] Implement `extract_paths`-equivalent in `snippet.rs`: walk OXC `BindingPattern` recursively, producing `inserts` (array intermediaries → `$.derived(() => $.to_array(...))`) and `paths` (leaf bindings → `let name = ...` or `$.derived_safe_equal(...)`)
- [ ] For `ObjectPattern` properties: generate `let name = () => $$argN?.().prop`
- [ ] For `ObjectPattern` rest: generate `let rest = () => $.exclude_from_object($$argN?.(), [excluded_keys])`
- [ ] For `ArrayPattern`: generate `var $$array = $.derived(() => $.to_array($$argN?.(), len))` + `let name = () => $.get($$array)[i]`
- [ ] For `AssignmentPattern` (defaults): generate `$.derived_safe_equal(() => $.fallback(expr, default))` or `$.fallback(expr, default)` for simple defaults
- [ ] For array rest: `let rest = () => $.get($$array).slice(i)`
- [ ] Dev mode: after each destructured `let` declaration, emit eager evaluation statement
- [ ] Update `build_snippet_params` to emit `$$argN` (plain identifier) for destructured params instead of flat `name = $.noop`

## Implementation order
1. Codegen first — walk parsed arrow params from `parsed.stmts`, generate declarations inline
2. No analyze changes needed if codegen reads params directly from parsed arrow AST
3. Tests verify output matches reference
