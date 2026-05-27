# SnippetBlock

## Current state
- **Working**: 32/33 use cases
- **Tests**: 35/36 green
- Last updated: 2026-05-14

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
- [x] Dev-mode `{@render snippet(...)}` calls are wrapped in `$.add_svelte_meta(() => snippet(...), "render", App, line, col)` rather than emitted bare. (test: `tag_render_dev`)
- [x] Dev-mode snippet parameter destructuring emits an eager-evaluation read after each leaf binding declaration (`name()` for thunk-form, `$.get(name)` for `has_default` derived form) so "Cannot access X before initialization" errors fire eagerly. Mirrors reference `SnippetBlock.js:63-67`. (test: `snippet_destructure_dev`)
- [x] Snippet body always emits a block-form arrow body `($$anchor) => { ... }`, never collapsed to expression body, even when the body compiles to a single `ExpressionStatement` (e.g. one inline component call). Reference compiler never collapses snippet bodies. Owning layer: codegen — `svelte_ast_builder::builder::functions::arrow` collapses any single-`ExpressionStatement` arrow to expression body, and `emit_snippet_block` / `build_snippet_const_inner` use that builder. Should switch snippet body emission to `arrow_block` (or equivalent non-collapsing variant). (test: `snippet_body_single_component`)
- [x] Top-level snippet that only references instance-script **imports** must hoist to module scope, matching reference `can_hoist_snippet` rule `function_depth === 0`. Owning layer: 3.A.5 `BlockSemantics`. `finalize_hoistable` in `crates/svelte_analyze/src/block_semantics/builder/walker.rs` exempts references whose symbol carries `BindingSemantics::MaybeReactive` (the marker `record_maybe_reactive_imports` places on every import) from the instance-scope taint. Reactive instance-script bindings (`State`/`Derived`/`Store`/`LegacyState`/`Prop`/…) keep tainting. (test: `diagnose_snippet_hoistable_with_script_import`)
- [x] Snippet name is scanned as a JS identifier, so `_` and `$` are accepted (`{#snippet extra_element()}`, `{#snippet $foo()}`). `Scanner::start_snippet_tag` reads the name through `Scanner::js_identifier_segment` (which mirrors JS identifier rules) instead of `Scanner::identifier` (HTML tag-name shape, terminates at `_`/`$`). (test: `diagnose_snippet_name_with_underscore`)
- [x] Module-scope `var root_<n> = $.from_html(...)` declarations emit in fragment-visit (source) order, and the `root_<n>` counter ticks for every hoisted snippet body — even when the body does not materialize a `from_html` template (e.g. snippet whose only content is `{@render mf()}`). `emit_fragment` in `crates/svelte_codegen_client/src/codegen/fragment/mod.rs` merges `bucket.snippets`/`svelte_head`/`svelte_window`/`svelte_document`/`svelte_body` into a single list sorted by `NodeId.0` before dispatching, so hoisted items consume `gen_ident("root")` slots in the order they appeared in the `.svelte` source. (test: `diagnose_hoisted_snippet_module_order_with_sibling_template`)
- [x] Snippet body uses its own per-function `fragment`/`node` ident counter, starting at `fragment` (no suffix). When the snippet is referenced from a JS expression (e.g. passed as a component prop value via a conditional like `icon={cond ? body : undefined}`), the inner counter must not inherit/share state with the surrounding function's template idents. Layer: codegen — fragment-id generation scope leaks across snippet body when the snippet is consumed via an `ExpressionTag`/derived-wrapped prop instead of a direct `{@render}` or named-slot binding. (test: `diagnose_fragment_id_in_snippet_used_as_expression`)
- [x] Top-level snippet whose parameter list carries only TypeScript type annotations (e.g. `{#snippet defaultWrapWith(mf: Snippet)}`) must still hoist to module scope. Type-only identifier references resolve to `SymbolFlags::TypeImport` bindings; `finalize_hoistable` in `crates/svelte_analyze/src/block_semantics/builder/walker.rs` exempts those from the instance-scope taint, mirroring the existing `BindingSemantics::MaybeReactive` continue. (test: `diagnose_snippet_hoistable_param_type_annotation`)
- [ ] Top-level snippet whose body auto-subscribes to a `$store` (e.g. `{$page.url}`) must NOT hoist to module scope, because store auto-subscription expands to `$page()` → `$.store_get(page, "$page", $$stores)` which depends on the instance-scoped `$$stores` array. Owning layer: 3.A.5 `BlockSemantics`. `finalize_hoistable` in `crates/svelte_analyze/src/block_semantics/builder/walker.rs` currently exempts every `BindingSemantics::MaybeReactive` reference (the marker on every instance import) from the instance-scope taint; the exemption is too permissive — store auto-sub identifiers (`$ident` resolved to a store import) must taint regardless. (test: `diagnose_snippet_store_autosub_not_hoistable`)
- [x] Snippet declared inside an `{#if}` (or any non-fragment template block) and lexically enclosed by an element child of that block emits the `const subtitle = ...` declaration inside the consequent/alternate arrow's body block, wrapped in its own `{ ... }` lexical scope. Implementation: `emit_local_snippet_block` in `crates/svelte_codegen_client/src/codegen/hoisted/snippet.rs` builds the snippet const via `build_snippet_const` and pushes it as a `BlockStatement { const X = ... }` directly into the enclosing fragment's `state.init`. `emit_fragment` non-root-anchor branch in `crates/svelte_codegen_client/src/codegen/fragment/mod.rs` calls this instead of `emit_hoisted_snippet`, so non-hoistable snippets in nested fragments stay local instead of being lifted into `instance_snippets`/`hoistable_snippets`. (test: `diagnose_snippet_inside_if_consequent`)

## Reference

### Svelte (reference compiler)
- `original/compiler/phases/3-transform/client/visitors/SnippetBlock.js` — parameter dispatch, `extract_paths` usage, dev wrapping
- `original/compiler/utils/ast.js` lines 243–415 — `extract_paths` / `_extract_paths`: recursive destructuring → inserts (array intermediaries) + paths (leaf bindings)
- `original/compiler/utils/ast.js` lines 585–597 — `build_fallback`: default value wrapping with `$.fallback()`
- `original/compiler/phases/scope.js` lines 1331–1346 — snippet param declared as `kind: 'snippet'`
- `original/compiler/phases/2-analyze/visitors/SnippetBlock.js` — hoistability, validation

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
- [x] `tag_render_dev`
- [x] `snippet_destructure_dev`
- [x] `snippet_object_destructure`
- [x] `snippet_array_destructure`
- [x] `snippet_mixed_params`
- [x] `snippet_nested_destructure`
- [x] `snippet_computed_key_destructure`
- [x] `snippet_parameter_assignment` (analyzer)
- [x] `validate_snippet_parameter_assignment`
- [x] `validate_snippet_parameter_assignment_in_nested_target`
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
- [x] `snippet_body_single_component`
- [x] `diagnose_snippet_inside_if_consequent`
- [x] `diagnose_snippet_hoistable_with_script_import`
- [x] `diagnose_snippet_name_with_underscore`
- [x] `diagnose_hoisted_snippet_module_order_with_sibling_template`
- [x] `diagnose_fragment_id_in_snippet_used_as_expression`
- [ ] `diagnose_snippet_store_autosub_not_hoistable`
- [x] `diagnose_snippet_hoistable_param_type_annotation`
