# If Block

## Current state
- **Working**: 10/10 use cases
- **Tests**: 10/10 snapshot tests + 9 block-semantics builder unit tests green
- Migrated to `BlockSemantics::If` (see SEMANTIC_LAYER_ARCHITECTURE.md). Legacy consumer (`template/if_block.rs`), `alt_is_elseif` populator and bitset, `first_if_block_id` accessor, and `Ctx::is_elseif_alt` removed.
- Last updated: 2026-04-20

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

- [x] Plain condition expression in `{#if ...}` with no alternate.
- [x] `{:else if ...}` chains with a final `{:else}` in the sync path.
- [x] `{@const}` inside `if` / `else if` branches.
- [x] Root async condition `{#if await expr}` with `experimental.async`.
- [x] Nested blocks under `{#if}` such as `{#await}`, `<svelte:boundary>`, `<svelte:element>`, `use:` and transitions.
- [x] Condition expressions containing calls and tracked symbols memoize exactly like the reference (`$.derived(() => expr)`).
- [x] `{:else if await expr}` under `experimental.async` remains a nested transparent else-if instead of being flattened into the parent branch chain.
- [x] `{:else if expr}` that introduces blockers not present in the parent branch follows the same non-flattened path as the reference compiler.
- [x] Analyzer validation: `validate_block_not_empty` for consequent/alternate
- [x] Analyzer validation: `validate_opening_tag` for runes mode

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
- `crates/svelte_analyze/src/block_semantics/data.rs` (`IfBlockSemantics`, `IfBranch`, `IfConditionKind`, `IfAsyncKind`, `IfAlternate`)
- `crates/svelte_analyze/src/block_semantics/builder/if_.rs`
- `crates/svelte_analyze/src/block_semantics/builder/common.rs` (`expression_if_facts`)
- `crates/svelte_codegen_client/src/template/if_block_semantics.rs`
- `crates/svelte_codegen_client/src/template/async_plan.rs`

## Test cases

- [x] `single_if_block`
- [x] `single_if_else_block`
- [x] `if_else_chain_with_const`
- [x] `async_if_basic`
- [x] `await_in_if`
- [x] `if_call_condition`
- [x] `async_if_else_if_condition`
- [x] `if_elseif_new_blockers`
- [x] `if_block_empty_consequent`
- [x] `if_block_empty_alternate`
