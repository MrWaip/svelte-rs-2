# SnippetBlock

## Current state
- **Complete**: 22/22 use cases — all implemented and covered
- `snippet_destructure_default_mutated_state_ref` landed: `svelte_transform::walk_node` now visits snippet parameter destructure defaults inside the snippet body scope, so identifiers that refer to mutated `$state` bindings are rewritten to `$.get(name)` inside the lazy-fallback thunk. Previously defaults were cloned verbatim by `build_fallback_expr`; the mutated-state variant was missed because the only existing default-state test used a never-mutated binding (demoted to `let` by the reference compiler).
- `snippet_destructure_default_state_ref` landed: snippet param destructuring defaults whose initializer is non-simple (per `is_simple_expression`) are now emitted as `$.fallback(access, () => <default>, true)` matching the reference compiler `build_fallback`. `is_simple_expression` lifted to `svelte_analyze::utils` and consumed from `svelte_codegen_client::template::snippet::build_fallback_expr`.
- `snippet_invalid_export` landed earlier: dual top-level script parsing (`instance_script` + `module_script`) added to AST/parser; validation fires when `<script module>` exports a template snippet name
- Last updated: 2026-04-08

**Next:** feature complete; no further work planned unless new edge cases emerge

## Source
ROADMAP Tier 2b: `{#snippet}` — parameter destructuring

## Use cases

- [x] No parameters: `{#snippet foo()}` → `($$anchor) => { ... }` (test: snippet_basic)
- [x] Simple identifier params: `{#snippet foo(a, b)}` → `($$anchor, a = $.noop, b = $.noop) => { ... }` (test: snippet_basic)
- [x] Hoisted snippet (top-level, no instance refs) → module-level declaration (test: snippet_basic)
- [x] Non-hoisted snippet (references instance vars) → instance-level declaration (test: snippet_ident_conflict_with_script)
- [x] Nested snippet (inside block/element) → local declaration (test: boundary_const_in_snippet)
- [x] Snippet as component prop → passed as named prop (test: component_snippet_prop)
- [x] Dev mode: `$.wrap_snippet(Name, function(...) { $.validate_snippet_args(...arguments); ... })` (test: tag_snippet_dev)
- [x] Object destructuring: `{#snippet foo({ x, y })}` → `$$arg0` param + `let x = () => $$arg0?.().x` (test: snippet_object_destructure)
- [x] Object destructuring with defaults: `{#snippet foo({ x = 5 })}` → `$.derived_safe_equal(() => $.fallback(...))` (test: snippet_object_destructure)
- [x] Snippet destructure default whose initializer is a non-literal expression (e.g. `[counter]`, an array referencing a binding) wraps the default in a lazy thunk and passes `true` as the third `$.fallback` argument: `$.fallback($$arg0?.().values, () => [counter], true)` (test: `snippet_destructure_default_state_ref`)
- [x] Snippet destructure default whose initializer references a **mutated** reactive binding (`$state`, `$derived`, store) must rewrite identifier reads to `$.get(name)` / thunk form inside the lazy-fallback thunk. Default expressions are transformed by `svelte_transform::walk_node::SnippetBlock` in the snippet body scope, matching the reference compiler's `context.visit(path.expression, child_state)`. (test: `snippet_destructure_default_mutated_state_ref`)
- [x] Object rest: `{#snippet foo({ x, ...rest })}` → `$.exclude_from_object($$arg0?.(), ['x'])` (test: snippet_object_destructure)
- [x] Array destructuring: `{#snippet foo([a, b])}` → `$.to_array($$arg0?.(), 2)` + derived intermediary (test: snippet_array_destructure)
- [x] Array destructuring with rest: `{#snippet foo([a, ...rest])}` → `$.get($$array).slice(1)` (test: snippet_array_destructure)
- [x] Mixed params: `{#snippet foo(a, { x }, [b])}` → identifier + object + array in one signature (test: snippet_mixed_params)
- [x] `snippet_parameter_assignment` — error on assignment to snippet param (Tier 5b) (tests: analyzer unit tests)
- [x] Nested object destructuring in snippet params: `{#snippet foo({ a: { b } })}` (test: snippet_nested_destructure)
- [x] Nested array destructuring in snippet params: `{#snippet foo({ a: [x, y] })}` (test: snippet_nested_destructure)
- [x] Computed key destructuring in snippet params: `{#snippet foo({ [key()]: value, ...rest })}` (test: snippet_computed_key_destructure)
- [x] `snippet_invalid_rest_parameter` validation (tests: analyzer unit tests)
- [x] `snippet_shadowing_prop` validation (tests: analyzer unit tests)
- [x] `snippet_conflict` validation (tests: analyzer unit tests)
- [x] `snippet_invalid_export` validation (tests: analyzer unit tests)

## Reference

### Svelte (reference compiler)
- `reference/compiler/phases/3-transform/client/visitors/SnippetBlock.js` — parameter dispatch, `extract_paths` usage, dev wrapping
- `reference/compiler/utils/ast.js` lines 243–415 — `extract_paths` / `_extract_paths`: recursive destructuring → inserts (array intermediaries) + paths (leaf bindings)
- `reference/compiler/utils/ast.js` lines 585–597 — `build_fallback`: default value wrapping with `$.fallback()`
- `reference/compiler/phases/scope.js` lines 1331–1346 — snippet param declared as `kind: 'snippet'`
- `reference/compiler/phases/2-analyze/visitors/SnippetBlock.js` — hoistability, validation

### Our code
- `crates/svelte_codegen_client/src/template/snippet.rs` — parsed-param-driven destructuring codegen, including nested object/array patterns, computed keys, and lazy `$.fallback` for non-simple defaults
- `crates/svelte_transform/src/lib.rs` — `walk_node::SnippetBlock` transforms parameter destructure defaults via `transform_snippet_param_defaults`, reusing `ExprTransformer` in the snippet body scope so state/store reads inside defaults get `$.get(...)` / thunk rewrites
- `crates/svelte_analyze/src/utils/simple_expression.rs` — `is_simple_expression` syntactic check (mirrors reference `is_simple_expression`); consumed by snippet codegen to choose lazy vs eager `$.fallback` form
- `crates/svelte_analyze/src/passes/template_side_tables.rs` — `SnippetParamMarker` marks snippet-param symbols for downstream validation
- `crates/svelte_analyze/src/passes/template_validation.rs` — snippet param assignment/rest/shadowing/conflict validation
- `crates/svelte_analyze/src/validate/mod.rs` — `validate_snippet_exports` fires `snippet_invalid_export` when module script exports a snippet name
- `crates/svelte_analyze/src/tests.rs` — analyzer-level coverage for snippet diagnostics; `tasks/compiler_tests/test_v3.rs` remains snapshot-only
- `crates/svelte_analyze/src/scope.rs` — `is_snippet_param` / `is_snippet_name` symbol classification
- `crates/svelte_parser/src/lib.rs` — dual `<script>` + `<script module>` now accepted; each stored in `Component.instance_script` / `Component.module_script`
- `crates/svelte_ast/src/lib.rs` — `Component.instance_script` + `Component.module_script` (replaces single `script` field)
- `crates/svelte_parser/src/types.rs` — `ParserResult.module_program` + `module_script_content_span`

## Test cases

- [x] `snippet_basic`
- [x] `snippet_ident_conflict_with_script`
- [x] `boundary_const_in_snippet`
- [x] `component_snippet_prop`
- [x] `tag_snippet_dev`
- [x] `snippet_object_destructure`
- [x] `snippet_array_destructure`
- [x] `snippet_mixed_params`
- [x] `snippet_nested_destructure`
- [x] `snippet_computed_key_destructure`
- [x] `snippet_parameter_assignment` (analyzer)
- [x] `validate_snippet_parameter_assignment`
- [ ] `validate_snippet_parameter_assignment_in_nested_target`
- [x] `snippet_invalid_rest_parameter` (analyzer)
- [x] `validate_snippet_invalid_rest_parameter`
- [x] `snippet_shadowing_prop` (analyzer)
- [x] `validate_snippet_shadowing_prop`
- [x] `snippet_conflict` (analyzer)
- [x] `validate_snippet_conflict`
- [x] `snippet_invalid_export` (analyzer)
- [ ] `validate_snippet_invalid_export`
- [x] `validate_snippet_invalid_export_no_false_positive`
- [x] `validate_snippet_invalid_export_module_bound_no_fire`
- [x] `fragment_facts_track_non_trivial_child_counts`
- [x] `validate_snippet_children_without_other_content_has_no_conflict`
- [x] `snippet_destructure_default_state_ref`
- [x] `snippet_destructure_default_mutated_state_ref`
