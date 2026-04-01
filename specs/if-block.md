# If Block

## Current state
- **Working**: 5/8 use cases
- **Missing**: 3/8 use cases
- **Next**: use `/port specs/if-block.md` to fix the reproduced call-memoization mismatch and the async `else if` flattening mismatch, then add one blocker-changing `{:else if}` snapshot to close the remaining audit gap.
- **Risk**: client codegen currently flattens structural `{:else if}` chains unconditionally, while the reference only flattens when the nested branch does not introduce `await` or new blockers.
- **Confirmed mismatches**:
- `if_call_condition`: Rust emits `$.derived(is_even)` while the reference emits `$.derived(() => is_even())`.
- `async_if_else_if_condition`: Rust flattens `{:else if await second()}` into the parent `$.if`, while the reference keeps it nested in a transparent else-if branch.
- Last updated: 2026-04-01

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
- `[ ]` Condition expressions containing calls and tracked symbols should memoize exactly like the reference. Reproduced mismatch: `if_call_condition`.
- `[ ]` `{:else if await expr}` under `experimental.async` should remain a nested transparent else-if instead of being flattened into the parent branch chain.
- `[ ]` `{:else if expr}` that introduces blockers not present in the parent branch should follow the same non-flattened path as the reference compiler.

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
- Existing and new compiler tests:
- `tasks/compiler_tests/cases2/single_if_block/case.svelte`
- `tasks/compiler_tests/cases2/single_if_else_block/case.svelte`
- `tasks/compiler_tests/cases2/if_else_chain_with_const/case.svelte`
- `tasks/compiler_tests/cases2/async_if_basic/case.svelte`
- `tasks/compiler_tests/cases2/if_call_condition/case.svelte`
- `tasks/compiler_tests/cases2/async_if_else_if_condition/case.svelte`

## Tasks

- `[ ]` Compare generated Rust snapshot for `async_if_else_if_condition` against the reference snapshot and capture the exact structural mismatch.
- `[ ]` Fix condition call memoization in `crates/svelte_codegen_client/src/template/if_block.rs` so memoized call conditions always wrap the expression in a thunk, matching the reference `$.derived(() => expr)` shape.
- `[ ]` Move else-if flattening ownership from structural lowering to analysis/codegen data that can respect `has_await` and blocker comparisons.
- `[ ]` Preserve the current sync flattening fast path for plain `{:else if}` chains.
- `[ ]` Keep `elseif` transition locality behavior aligned with the reference when the alternate remains nested.

## Implementation order

- 1. Reproduce on `async_if_else_if_condition`.
- 2. Fix flattening eligibility for else-if branches.
- 3. Re-run the focused compiler tests and existing `if` snapshots.

## Discovered bugs

- OPEN: `crates/svelte_codegen_client/src/template/if_block.rs` follows `alt_is_elseif` structurally and therefore cannot distinguish sync-flattenable `{:else if}` from async or blocker-changing branches that must stay nested.
- OPEN: `crates/svelte_codegen_client/src/template/if_block.rs` emits `$.derived(expr)` for memoized `if` call conditions instead of the reference `$.derived(() => expr)` form.

## Test cases

- Existing:
- `single_if_block`
- `single_if_else_block`
- `if_else_chain_with_const`
- `async_if_basic`
- `await_in_if`
- Added during this audit:
- `if_call_condition`
- `async_if_else_if_condition`
