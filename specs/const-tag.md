# Const Tag

## Current state
- **Working**: 10/10 in-scope use cases — feature complete
- **Partial**: 0
- **Missing**: 0
- **Next slice**: none — all use cases complete
- **Reclassified**: `const_tag_invalid_reference` moved to `Out of scope` — this diagnostic is gated on `experimental.async` in the reference compiler (`Identifier.js:162`) and requires `is_template_declaration` tracking in analyze; tracked as use case 37 in `experimental-async.md`.
- **Completed this session**:
  - `const_tag_await`: `{@const}` inside `{#await}` then branch — passes.
  - `const_tag_component`: `{@const}` inside `<Component>` default children — passes after fixing `template_scoping.rs` to register `FragmentKey::ComponentNode` so `mark_const_tag_bindings` can find the scope and mark bindings as `RuneKind::Derived`.
  - Bug fix: `crates/svelte_analyze/src/passes/template_scoping.rs` — `ComponentNode` now registers `current_scope` as its fragment scope; without this, all `{@const}` bindings inside component children were silently skipped by `mark_const_tag_bindings`, leaving identifiers unwrapped in the transform pass.
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

- `[x]` Simple identifier binding inside an allowed block parent such as `{#each}` or `{#if}`.
- `[x]` Destructured binding patterns (`{ x, y }`) with derived reads through the generated temp binding.
- `[x]` Multiple independent `{@const}` tags in one fragment.
- `[x]` TypeScript annotations on `{@const}` declarations are stripped before client codegen.
- `[x]` `{@const}` inside `if` / `else if` branches.
- `[x]` `{@const}` inside `{#key}` blocks.
- `[x]` `<svelte:boundary>` snippets can read boundary-local `{@const}` bindings in the currently covered success path.
- `[x]` Allowed-parent coverage confirmed with focused cases: `{#await}` (`const_tag_await`) and `<Component>` (`const_tag_component`).
- `[x]` Invalid placement should report `const_tag_invalid_placement`.
- `[x]` Invalid declaration shapes should report `const_tag_invalid_expression`.

## Out of scope

- `const_tag_invalid_reference` — only fires in `experimental.async` mode (gated at `Identifier.js:162` on `binding.metadata.is_template_declaration && experimental.async`); tracked as use case 37 in `specs/experimental-async.md`.
- Slotted fragments (`<element slot="name">`, `<svelte:fragment slot="name">`, `<slot />`) — Svelte 4 legacy; not in scope for runes-mode work.

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

## Tasks

- `[x]` Add a template validation pass for `ConstTag` placement and declaration-shape diagnostics, using the reference compiler's allowed-parent matrix.
- `[x]` Add focused success cases for the remaining allowed parents: `{#await}` and `<Component>` (slotted fragments are legacy Svelte 4, out of scope).
- N/A `const_tag_invalid_reference` — moved to `specs/experimental-async.md` (use case 37).

## Implementation order

1. Add focused success cases for remaining allowed parents.

## Discovered bugs

- FIXED: `crates/svelte_analyze/src/validate/mod.rs` only ran rune validation; no template validation path reached the existing `const_tag_*` diagnostics. Fixed by adding `visit_const_tag` to `TemplateValidationVisitor` in `template_validation.rs`.

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
