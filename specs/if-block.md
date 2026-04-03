# If Block

## Current state
- **Working**: 10/10 use cases — Complete
- Last updated: 2026-04-03

## Source

- ROADMAP template item: `{#if}` / `{:else}`
- Audit request: `{#if expression}`

## Syntax variants

```svelte
{#if expression}...{/if}
{#if expression}...{:else if expression}...{/if}
{#if expression}...{:else}...{/if}
{#if await expression}...{/if}
{#if expression}...{:else if await expression}...{/if}
```

## Use cases

- `[x]` Plain condition expression in `{#if ...}` with no alternate.
- `[x]` `{:else if ...}` chains with a final `{:else}` in the sync path.
- `[x]` `{@const}` inside `if` / `else if` branches.
- `[x]` Root async condition `{#if await expr}` with `experimental.async`.
- `[x]` Nested blocks under `{#if}` such as `{#await}`, `<svelte:boundary>`, `<svelte:element>`, `use:` and transitions.
- `[x]` Condition expressions containing calls and tracked symbols memoize exactly like the reference (`$.derived(() => expr)`).
- `[x]` `{:else if await expr}` under `experimental.async` remains a nested transparent else-if instead of being flattened into the parent branch chain.
- `[x]` `{:else if expr}` that introduces blockers not present in the parent branch follows the same non-flattened path as the reference compiler.

- `[x]` Analyzer validation: `validate_block_not_empty` for consequent/alternate
- `[x]` Analyzer validation: `validate_opening_tag` for runes mode

## Reference

- Reference docs:
- `reference/docs/03-template-syntax/02-if.md`
- Reference compiler:
- `reference/compiler/phases/1-parse/state/tag.js`
- `reference/compiler/phases/2-analyze/visitors/IfBlock.js`
- `reference/compiler/phases/3-transform/client/visitors/IfBlock.js`
- Our parser/analyze/codegen:
- `crates/svelte_parser/src/scanner/mod.rs`
- `crates/svelte_parser/src/handlers.rs`
- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/types/data/analysis.rs`
- `crates/svelte_codegen_client/src/template/if_block.rs`
- `crates/svelte_codegen_client/src/template/async_plan.rs`
- Existing compiler tests:
- `tasks/compiler_tests/cases2/single_if_block/case.svelte`
- `tasks/compiler_tests/cases2/single_if_else_block/case.svelte`
- `tasks/compiler_tests/cases2/if_else_chain_with_const/case.svelte`
- `tasks/compiler_tests/cases2/async_if_basic/case.svelte`
- `tasks/compiler_tests/cases2/if_call_condition/case.svelte`
- `tasks/compiler_tests/cases2/async_if_else_if_condition/case.svelte`
- `tasks/compiler_tests/cases2/if_elseif_new_blockers/case.svelte`

## Discovered bugs

- FIXED: `thunk()` strips no-arg calls (`is_even()` → `is_even`) but `$.derived` requires the call preserved inside an arrow.
- FIXED: Statement ordering: all consequent arrows emitted first, then all deriveds; reference interleaves per-branch.
- FIXED: `alt_is_elseif` flattened unconditionally — no check for `has_await` or new blockers on the nested else-if.
- FIXED: `$.async` callback always used hardcoded `"node"` parameter, causing name collisions when nested.
- FIXED: Root expression consumed when `needs_async` even for blocker-only async where it's not needed for the thunk.

## Tasks

- `[x]` Fix condition call memoization: always wrap in arrow for `$.derived()`, don't use `thunk()` which strips no-arg calls.
- `[x]` Fix statement ordering: interleave consequent arrows with derived declarations per branch (matching reference order).
- `[x]` Move else-if flattening guard to codegen: check `has_await` and `has_more_blockers_than` before flattening.
- `[x]` Add `elseif` flag (`true` third arg) on `$.if()` for non-flattened else-if blocks.
- `[x]` Fix expression consumption for blocker-only async: only consume at root for `has_await`, not `needs_async`.
- `[x]` Add unique `node` parameter naming in `$.async` callbacks to support nesting.
- `[x]` Add `if_elseif_new_blockers` test case for blocker-changing else-if.
- `[x]` Add `validate_block_not_empty` in `TemplateValidationVisitor::visit_if_block` (consequent + alternate).
- `[x]` Add `validate_opening_tag` in `visit_if_block` (runes mode, `#` for `{#if}`, `:` for `{:else if}`).

## Test cases

- Existing:
- `single_if_block`
- `single_if_else_block`
- `if_else_chain_with_const`
- `async_if_basic`
- `await_in_if`
- Fixed during this port:
- `if_call_condition` — condition call memoization
- `async_if_else_if_condition` — async else-if flattening
- Added during this port:
- `if_elseif_new_blockers` — blocker-changing else-if
- `if_block_empty_consequent` — BlockEmpty warning on whitespace-only consequent
- `if_block_empty_alternate` — BlockEmpty warning on whitespace-only alternate
