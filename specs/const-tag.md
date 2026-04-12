# Const Tag

## Current state
- **Working**: 10/10 use cases
- **Tests**: 22/22 green
- Last updated: 2026-04-04

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
- [x] Multiple independent `{@const}` tags in one fragment
- [x] TypeScript annotations on `{@const}` declarations are stripped before client codegen
- [x] `{@const}` inside `if` / `else if` branches
- [x] `{@const}` inside `{#key}` blocks
- [x] `<svelte:boundary>` snippets can read boundary-local `{@const}` bindings in the currently covered success path
- [x] Allowed-parent coverage confirmed with focused cases: `{#await}` (`const_tag_await`) and `<Component>` (`const_tag_component`)
- [x] Invalid placement should report `const_tag_invalid_placement`
- [x] Invalid declaration shapes should report `const_tag_invalid_expression`

## Out of scope

- `const_tag_invalid_reference` — only fires in `experimental.async` mode (gated at `Identifier.js:162` on `binding.metadata.is_template_declaration && experimental.async`); tracked as use case 37 in `specs/experimental-async.md`.
- Legacy Svelte 4 parent-placement variants are owned by `specs/legacy-slots.md`, not this runes-mode spec.

## Reference

- Reference docs:
- `reference/docs/03-template-syntax/10-@const.md`
- `reference/docs/07-misc/07-v5-migration-guide.md`
- Reference compiler:
- `reference/compiler/phases/1-parse/state/tag.js`
- `reference/compiler/phases/2-analyze/visitors/ConstTag.js`
- `reference/compiler/phases/3-transform/client/visitors/ConstTag.js`
- `reference/compiler/phases/3-transform/utils.js`
- `reference/compiler/phases/3-transform/client/visitors/SvelteBoundary.js`
- `reference/compiler/errors.js`
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
