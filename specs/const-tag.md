# Const Tag

## Current state
- **Working**: 7/11 use cases
- **Partial**: 1/11 use cases
- **Missing**: 3/11 use cases
- **Next**: use `/port specs/const-tag.md` to add template-side validation (`const_tag_invalid_placement`, `const_tag_invalid_expression`, `const_tag_invalid_reference`) and then expand allowed-parent coverage beyond the now-audited `{#key}` success path.
- **Confirmed gaps**:
- The analyze pipeline does not currently emit any `const_tag_*` diagnostics even though the diagnostic kinds already exist.
- Last updated: 2026-04-01

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

- `[x]` Simple identifier binding inside an allowed block parent such as `{#each}` or `{#if}`.
- `[x]` Destructured binding patterns (`{ x, y }`) with derived reads through the generated temp binding.
- `[x]` Multiple independent `{@const}` tags in one fragment.
- `[x]` TypeScript annotations on `{@const}` declarations are stripped before client codegen.
- `[x]` `{@const}` inside `if` / `else if` branches.
- `[x]` `{@const}` inside `{#key}` blocks.
- `[x]` `<svelte:boundary>` snippets can read boundary-local `{@const}` bindings in the currently covered success path.
- `[~]` Allowed-parent coverage is broader in the AST/analyze/codegen paths than in tests, but `{#await}` branches, `<Component>`, and slotted fragments have not been audited with focused cases yet.
- `[ ]` Invalid placement should report `const_tag_invalid_placement`.
- `[ ]` Invalid declaration shapes should report `const_tag_invalid_expression`.
- `[ ]` Snippets that reference an out-of-scope `{@const}` binding should report `const_tag_invalid_reference`.

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
- Existing and added tests:
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

## Tasks

- `[ ]` Add a template validation pass for `ConstTag` placement and declaration-shape diagnostics, using the reference compiler's allowed-parent matrix.
- `[ ]` Track snippet visibility errors for `{@const}` bindings and emit `const_tag_invalid_reference` when a snippet reads a binding outside its valid scope.
- `[ ]` Add focused success cases for the remaining allowed parents once the baseline validation mismatches are fixed.

## Implementation order

1. Add missing analyzer/compiler diagnostics for placement and invalid declaration shape.
2. Add snippet visibility validation.
3. Expand the allowed-parent test matrix after the core validation behavior matches the reference.

## Discovered bugs

- OPEN: `crates/svelte_analyze/src/validate/mod.rs` only runs rune validation today; no template validation path reaches the existing `const_tag_*` diagnostics.

## Test cases

- Existing:
- `const_tag`
- `const_tag_destructured`
- `const_tag_destructured_multi`
- `const_tag_destructured_if`
- `const_tag_dev`
- `ts_strip_const_tag`
- `const_tag_key_block`
- `boundary_const_tag`
- `boundary_const_in_snippet`
- `if_else_chain_with_const`
- Covered by `experimental-async` spec:
- `async_const_tag`
- `async_const_derived_chain`
- `async_boundary_const`
- Added during this audit:
- `const_tag_key_block`
- `validate_const_tag_invalid_placement_root` (`#[ignore]`, missing template validation)
- `compile_const_tag_invalid_expression` (`#[ignore]`, missing const-tag declaration validation)
