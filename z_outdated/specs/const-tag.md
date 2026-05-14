# Const Tag

## Current state
- **Working**: 16/16 use cases
- **Tests**: 28/28 green
- Last updated: 2026-05-25

## Source

- ROADMAP template item: `{@const}`
- Audit request: `{@const}`

## Syntax variants

```svelte
{#each items as item}
	{@const doubled = item * 2}
{/each}

{#if visible}
	{@const { x, y } = point}
{/if}

{#snippet row(item)}
	{@const label = item.name}
{/snippet}
```

## Use cases

- [x] Simple identifier binding inside an allowed block parent such as `{#each}` or `{#if}`
- [x] Destructured binding patterns (`{ x, y }`) with derived reads through the generated temp binding
- [x] Single-key object destructure `{@const { x } = expr}` rewrites reads as `$.get(computed_const).x` (test: `const_tag_destructured_single_key`)
- [x] Destructured pattern preserves defaults (`{ a, b = 'x' } = expr`), alias renames, and nested patterns by reusing the original `BindingPattern` from the parsed declarator (test: `const_tag_destructured_default`)
- [x] Multiple independent `{@const}` tags in one fragment
- [x] TypeScript annotations on `{@const}` declarations are stripped before client codegen
- [x] `{@const}` inside `if` / `else if` branches
- [x] `{@const}` inside `{#key}` blocks
- [x] `<svelte:boundary>` snippets can read boundary-local `{@const}` bindings in the currently covered success path
- [x] Allowed-parent coverage confirmed with focused cases: `{#await}` (`const_tag_await`) and `<Component>` (`const_tag_component`)
- [x] Invalid placement should report `const_tag_invalid_placement`
- [x] Invalid declaration shapes should report `const_tag_invalid_expression`
- [x] Legacy-mode `template_effect` dependency expressions for destructured `{@const}` bindings must rewrite the identifier through `computed_const` (`$.deep_read_state($.get(computed_const).button)`), not emit `$.safe_get(button)` (test: `diagnose_const_tag_legacy_dependency_destructure`)
- [x] Legacy-mode `{@const}` whose body is a top-level spread expression over other reactive `{@const}` derives must wrap as `$.derived_safe_equal(() => ($.deep_read_state($.get(<dep>)), …, $.untrack(() => [...$.get(<dep>), …])))`. Today our codegen emits the bare spread `() => [...$.get(xs), ...$.get(ys)]` and loses the explicit dependency reads + untrack barrier, so dependency tracking diverges from reference. Layer: transform — the `derived_safe_equal` body builder for `{@const}` must lift each spread argument that resolves to a reactive `$.get(…)` read into the comma-prefix `deep_read_state` chain and rebuild the array spread inside the trailing `$.untrack(() => …)` callback. Test: `diagnose_const_tag_legacy_dep_read_spread_of_derived`.
- [x] Legacy-mode destructured `{@const}` bindings consumed inside a child component invocation (shorthand prop and default-slot text) must rewrite to reactive reads through `computed_const`. Reference emits `get status() { return $.get(computed_const).status; }` for the component prop and wraps the text update in `$.template_effect(() => $.set_text(text, $.get(computed_const).title))`. Layer: analyze 3.A.2 `ReactivitySemantics` — `ConstBindingSemantics::ConstTag.reactive` must be forced `true` for destructured const-tags in legacy/soft-legacy mode, because transform unconditionally rewrites their reads through `$.get(computed_const).<key>`. Test: `diagnose_const_tag_legacy_destructure_into_component`.
- [x] Non-destructured `{@const}` binding with a function-literal initializer consumed as a shorthand prop on a child component must emit a flat `name: $.get(name)` property. Reference picks the flat (`b.init`) form because the binding's `is_function()` is true (`Identifier.js:95`–`101` + `scope.js:181`–`195`): the const-tag derive is reactive, so the read must go through `$.get(...)`, but a function-typed reference does not contribute to `has_state` and `build_attribute_value` falls to `b.init(value)` instead of `b.get(...)`. Our codegen currently emits a bare `{ name }` shorthand (no `$.get`, no getter) — minimal repro: `{#each items as item}{@const callback = () => item.id}<Child {callback} />{/each}` lowers to `Child($$anchor, { callback })` while reference emits `Child($$anchor, { callback: $.get(callback) })`. Triggers in default mode (auto-detected legacy) and explicit `runes: false`; the bug is independent of any `{#if}` wrapping and reproduces with multiple shorthand props in the same call. Layer: analyze 3.A.2 `ReactivitySemantics` — `ConstBindingSemantics::ConstTag` must mark the binding so that attribute-value lowering on a component recognises the identifier as a reactive read; `initial_is_function` then selects the flat `name: $.get(name)` form over a getter. Test: `diagnose_each_const_tag_shorthand_prop_to_component`.

## Out of scope

- `const_tag_invalid_reference` — only fires in `experimental.async` mode (gated at `Identifier.js:162` on `binding.metadata.is_template_declaration && experimental.async`); tracked as use case 37 in `specs/experimental-async.md`.
- Legacy Svelte 4 parent-placement variants are owned by `specs/legacy-slots.md`, not this runes-mode spec.

## Reference

- Reference docs:
- `original/docs/03-template-syntax/10-@const.md`
- `original/docs/07-misc/07-v5-migration-guide.md`
- Reference compiler:
- `original/compiler/phases/1-parse/state/tag.js`
- `original/compiler/phases/2-analyze/visitors/ConstTag.js`
- `original/compiler/phases/3-transform/client/visitors/ConstTag.js`
- `original/compiler/phases/3-transform/utils.js`
- `original/compiler/phases/3-transform/client/visitors/SvelteBoundary.js`
- `original/compiler/errors.js`
- Our parser/analyze/transform/codegen:
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/parse_js.rs`
- `crates/svelte_analyze/src/passes/template_side_tables.rs`
- `crates/svelte_analyze/src/passes/collect_symbols.rs`
- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/lib.rs`
- `crates/svelte_transform/src/lib.rs`
- `crates/svelte_codegen_client/src/template/const_tag.rs`
- `crates/svelte_codegen_client/src/template/svelte_boundary.rs`
- Diagnostics:
- `crates/svelte_diagnostics/src/lib.rs`
- Tests:
- `tasks/compiler_tests/cases2/const_tag_await/case.svelte`
- `tasks/compiler_tests/cases2/const_tag_component/case.svelte`
- `tasks/compiler_tests/cases2/const_tag/case.svelte`
- `tasks/compiler_tests/cases2/const_tag_destructured/case.svelte`
- `tasks/compiler_tests/cases2/const_tag_destructured_multi/case.svelte`
- `tasks/compiler_tests/cases2/const_tag_destructured_if/case.svelte`
- `tasks/compiler_tests/cases2/const_tag_dev/case.svelte`
- `tasks/compiler_tests/cases2/ts_strip_const_tag/case.svelte`
- `tasks/compiler_tests/cases2/const_tag_key_block/case.svelte`
- `tasks/compiler_tests/cases2/boundary_const_tag/case.svelte`
- `tasks/compiler_tests/cases2/boundary_const_in_snippet/case.svelte`
- `tasks/compiler_tests/cases2/if_else_chain_with_const/case.svelte`
- `crates/svelte_compiler/src/tests.rs`
- `crates/svelte_analyze/src/tests.rs`

## Test cases

- [x] `const_tag`
- [x] `const_tag_destructured`
- [x] `const_tag_destructured_multi`
- [x] `const_tag_destructured_if`
- [x] `const_tag_destructured_single_key`
- [x] `const_tag_destructured_default`
- [x] `const_tag_dev`
- [x] `ts_strip_const_tag`
- [x] `const_tag_key_block`
- [x] `boundary_const_tag`
- [x] `boundary_const_in_snippet`
- [x] `if_else_chain_with_const`
- [x] `const_tag_await`
- [x] `const_tag_component`
- [x] `validate_const_tag_invalid_placement_root`
- [x] `validate_const_tag_invalid_placement_inside_element`
- [x] `validate_const_tag_invalid_expression`
- [x] `validate_const_tag_valid_placement_each`
- [x] `validate_const_tag_valid_placement_if`
- [x] `validate_const_tag_valid_placement_key`
- [x] `validate_const_tag_parenthesized_sequence_ok`
- [x] `async_const_tag` (covered by `experimental-async`)
- [x] `async_const_derived_chain` (covered by `experimental-async`)
- [x] `async_boundary_const` (covered by `experimental-async`)
- [x] `diagnose_const_tag_legacy_dependency_destructure`
- [x] `diagnose_const_tag_legacy_dep_read_spread_of_derived`
- [x] `diagnose_const_tag_legacy_destructure_into_component`
- [x] `diagnose_each_const_tag_shorthand_prop_to_component`
