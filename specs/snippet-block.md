# SnippetBlock

## Current state
- **Complete**: 19/19 use cases — all implemented and covered
- `snippet_invalid_export` landed: dual top-level script parsing (`instance_script` + `module_script`) added to AST/parser; validation fires when `<script module>` exports a template snippet name
- Last updated: 2026-04-04

## Source
ROADMAP Tier 2b: `{#snippet}` — parameter destructuring

## Use cases

1. [x] No parameters: `{#snippet foo()}` → `($$anchor) => { ... }` (test: snippet_basic)
2. [x] Simple identifier params: `{#snippet foo(a, b)}` → `($$anchor, a = $.noop, b = $.noop) => { ... }` (test: snippet_basic)
3. [x] Hoisted snippet (top-level, no instance refs) → module-level declaration (test: snippet_basic)
4. [x] Non-hoisted snippet (references instance vars) → instance-level declaration (test: snippet_ident_conflict_with_script)
5. [x] Nested snippet (inside block/element) → local declaration (test: boundary_const_in_snippet)
6. [x] Snippet as component prop → passed as named prop (test: component_snippet_prop)
7. [x] Dev mode: `$.wrap_snippet(Name, function(...) { $.validate_snippet_args(...arguments); ... })` (test: tag_snippet_dev)
8. [x] Object destructuring: `{#snippet foo({ x, y })}` → `$$arg0` param + `let x = () => $$arg0?.().x` (test: snippet_object_destructure)
9. [x] Object destructuring with defaults: `{#snippet foo({ x = 5 })}` → `$.derived_safe_equal(() => $.fallback(...))` (test: snippet_object_destructure)
10. [x] Object rest: `{#snippet foo({ x, ...rest })}` → `$.exclude_from_object($$arg0?.(), ['x'])` (test: snippet_object_destructure)
11. [x] Array destructuring: `{#snippet foo([a, b])}` → `$.to_array($$arg0?.(), 2)` + derived intermediary (test: snippet_array_destructure)
12. [x] Array destructuring with rest: `{#snippet foo([a, ...rest])}` → `$.get($$array).slice(1)` (test: snippet_array_destructure)
13. [x] Mixed params: `{#snippet foo(a, { x }, [b])}` → identifier + object + array in one signature (test: snippet_mixed_params)
14. [x] `snippet_parameter_assignment` — error on assignment to snippet param (Tier 5b) (tests: analyzer unit tests)
15. [x] Nested object destructuring in snippet params: `{#snippet foo({ a: { b } })}` (test: `snippet_nested_destructure`)
16. [x] Nested array destructuring in snippet params: `{#snippet foo({ a: [x, y] })}` (test: `snippet_nested_destructure`)
17. [x] Computed key destructuring in snippet params: `{#snippet foo({ [key()]: value, ...rest })}` (test: `snippet_computed_key_destructure`)
18. [x] `snippet_invalid_rest_parameter` validation (tests: analyzer unit tests)
19. [x] `snippet_shadowing_prop` validation (tests: analyzer unit tests)
20. [x] `snippet_conflict` validation (tests: analyzer unit tests)
21. [x] `snippet_invalid_export` validation (tests: analyzer unit tests)

## Reference

### Svelte (reference compiler)
- `reference/compiler/phases/3-transform/client/visitors/SnippetBlock.js` — parameter dispatch, `extract_paths` usage, dev wrapping
- `reference/compiler/utils/ast.js` lines 243–415 — `extract_paths` / `_extract_paths`: recursive destructuring → inserts (array intermediaries) + paths (leaf bindings)
- `reference/compiler/utils/ast.js` lines 585–597 — `build_fallback`: default value wrapping with `$.fallback()`
- `reference/compiler/phases/scope.js` lines 1331–1346 — snippet param declared as `kind: 'snippet'`
- `reference/compiler/phases/2-analyze/visitors/SnippetBlock.js` — hoistability, validation

### Our code
- `crates/svelte_codegen_client/src/template/snippet.rs` — parsed-param-driven destructuring codegen, including nested object/array patterns and computed keys
- `crates/svelte_analyze/src/passes/template_side_tables.rs` — `SnippetParamMarker` marks snippet-param symbols for downstream validation
- `crates/svelte_analyze/src/passes/template_validation.rs` — snippet param assignment/rest/shadowing/conflict validation
- `crates/svelte_analyze/src/validate/mod.rs` — `validate_snippet_exports` fires `snippet_invalid_export` when module script exports a snippet name
- `crates/svelte_analyze/src/tests.rs` — analyzer-level coverage for snippet diagnostics; `tasks/compiler_tests/test_v3.rs` remains snapshot-only
- `crates/svelte_analyze/src/scope.rs` — `is_snippet_param` / `is_snippet_name` symbol classification
- `crates/svelte_parser/src/lib.rs` — dual `<script>` + `<script module>` now accepted; each stored in `Component.instance_script` / `Component.module_script`
- `crates/svelte_ast/src/lib.rs` — `Component.instance_script` + `Component.module_script` (replaces single `script` field)
- `crates/svelte_parser/src/types.rs` — `ParserResult.module_program` + `module_script_content_span`

## Tasks

### Analysis
- [x] Mark snippet parameter symbols explicitly in scoping so validation can reject writes by `SymbolId`, not by name
- [x] Validate snippet parameter assignment in template JS, including nested assignment targets
- [x] Validate snippet rest parameters, component-prop shadowing, and `children` snippet conflicts
- [x] Add `snippet_invalid_export` — parser now supports dual scripts; validation fires from `validate_snippet_exports`

### Codegen
- [x] Walk parsed `FormalParameters` directly from `parsed.stmts`
- [x] Generate nested object/array destructuring declarations, defaults, rest bindings, and computed-key accessors
- [x] Preserve dev-mode eager reads for destructured snippet bindings

## Implementation order
1. Land parsed-param codegen support for nested/computed destructuring
2. Land analyzer validation for snippet writes/rest/shadowing/conflicts
3. Unblock dual-script parsing, then finish `snippet_invalid_export`
