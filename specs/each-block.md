# Each Block

## Current state
- **Working**: 15/15 core client-side `{#each}` use cases implemented and passing.
- **Open gap**: When an each block's inner scope has a declaration that shadows an outer scope binding (e.g. `{@const a = ...}` shadowing a snippet named `a`), the reference compiler sets `collection_id` and emits `$$index, $$array` as extra render-callback params. The Rust codegen lacks this `collection_id` concept entirely — new use case added below.
- Last updated: 2026-04-04

## Source

- ROADMAP template item: `{#each}`
- Audit request: `/audit {#each}`

## Syntax variants

- `{#each expression as item}...{/each}`
- `{#each expression as item, index}...{/each}`
- `{#each expression as item (key)}...{/each}`
- `{#each expression as item, index (key)}...{/each}`
- `{#each expression as { id, ...rest }}...{/each}`
- `{#each expression as [id, ...rest]}...{/each}`
- `{#each expression}...{/each}`
- `{#each expression, index}...{/each}`
- `{#each expression as item}...{:else}...{/each}`
- `{#each await expression as item}...{/each}` under experimental async

## Use cases

- `[x]` Basic item iteration: `{#each items as item}`.
- `[x]` Item iteration with index: `{#each items as item, i}`.
- `[x]` Keyed each blocks, including key expressions that reference the index.
- `[x]` Key-is-item optimization in runes mode.
- `[x]` Destructured object and array patterns.
- `[x]` Destructured defaults inside each context.
- `[x]` Item-less each blocks: `{#each items}`.
- `[x]` Item-less each blocks with index: `{#each { length: 8 }, rank}`.
- `[x]` `{:else}` fallback blocks for empty collections.
- `[x]` Bind/group and bind:this interactions with parent each scopes.
- `[x]` `animate:` codegen flags for keyed each blocks that already satisfy placement constraints.
- `[x]` Diagnostic: keyed each without `as` should raise `each_key_without_as`.
- `[x]` Diagnostic: `animate:` outside a keyed each or on a non-sole child should raise `animation_invalid_placement`.
- `[x]` Diagnostic: `animate:` inside an unkeyed each should raise `animation_missing_key`.
- `[x]` Diagnostic: runes-mode reassignment or binding to an each item should raise `each_item_invalid_assignment`.
- `[ ]` Inner-scope shadowing: when an each block's inner scope declares a binding that shadows an outer scope name, emit `$$index, $$array` as extra render-callback params (reference: `collection_id` logic in `EachBlock.js` lines 112–123 and 316–318).

## Reference

- Reference docs:
  - `reference/docs/03-template-syntax/03-each.md`
- Reference compiler:
  - `reference/compiler/phases/1-parse/state/tag.js`
  - `reference/compiler/phases/2-analyze/visitors/EachBlock.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/utils.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/element.js`
  - `reference/compiler/phases/3-transform/client/visitors/EachBlock.js`
- Rust implementation:
  - `crates/svelte_parser/src/scanner/mod.rs`
  - `crates/svelte_parser/src/handlers.rs`
  - `crates/svelte_parser/src/walk_js.rs`
  - `crates/svelte_analyze/src/passes/template_scoping.rs`
  - `crates/svelte_analyze/src/passes/template_semantic.rs`
  - `crates/svelte_analyze/src/passes/template_side_tables.rs`
  - `crates/svelte_analyze/src/passes/collect_symbols.rs`
  - `crates/svelte_analyze/src/passes/bind_semantics.rs`
  - `crates/svelte_analyze/src/validate/mod.rs`
  - `crates/svelte_codegen_client/src/template/each_block.rs`
- Existing coverage:
  - `tasks/compiler_tests/cases2/each_block`
  - `tasks/compiler_tests/cases2/each_keyed_index`
  - `tasks/compiler_tests/cases2/each_key_uses_index`
  - `tasks/compiler_tests/cases2/each_key_is_item`
  - `tasks/compiler_tests/cases2/each_destructured_obj`
  - `tasks/compiler_tests/cases2/each_destructured_array`
  - `tasks/compiler_tests/cases2/each_destructured_default`
  - `tasks/compiler_tests/cases2/each_keyed_destructure`
  - `tasks/compiler_tests/cases2/each_block_no_item`
  - `tasks/compiler_tests/cases2/each_block_no_item_multi`
  - `tasks/compiler_tests/cases2/each_fallback`
  - `tasks/compiler_tests/cases2/async_each_basic`
  - `tasks/compiler_tests/cases2/animate_basic`
  - `tasks/compiler_tests/cases2/animate_params`
  - `tasks/compiler_tests/cases2/animate_dotted_name`
  - `tasks/compiler_tests/cases2/animate_reactive_params`
  - `tasks/compiler_tests/cases2/animate_blockers`
  - `tasks/compiler_tests/cases2/animate_with_spread`

## Tasks

- [ ] Parser: port support for `{#each expression, index}` without `as` from the reference parser into `crates/svelte_parser/src/scanner/mod.rs` and the surrounding handler/tests.
- [ ] Analyze: add template validation pass ownership for `{#each}` diagnostics instead of leaving `validate/` script-only.
- [ ] Analyze: emit `each_key_without_as` when a keyed each block has no context binding.
- [ ] Analyze: emit `animation_missing_key` and `animation_invalid_placement` using actual each ancestry and child-shape checks.
- [ ] Analyze: emit `each_item_invalid_assignment` for assignments and bindings that target each-item bindings in runes mode.
- [ ] Tests: unignore the audit tests after implementation and add compiler snapshots only where behavior is best expressed through generated JS.

## Implementation order

1. Parser support for item-less indexed each blocks.
2. Template validation infrastructure for each/animate diagnostics.
3. Each-item assignment validation in runes mode.
4. Optional follow-up compiler snapshots for any behavior that is not well covered by unit tests.

## Discovered bugs

- OPEN: `crates/svelte_parser/src/scanner/mod.rs` currently finalizes an `{#each}` header only after finding `as`, which leaves the documented `{#each expression, index}` form unsupported.
- OPEN: `crates/svelte_analyze/src/validate/mod.rs` only runs script-rune validation; `{#each}` template diagnostics from the reference analyzer are currently unimplemented.

## Test cases

- Existing passing coverage:
  - `each_block`
  - `each_keyed_index`
  - `each_key_uses_index`
  - `each_key_is_item`
  - `each_destructured_obj`
  - `each_destructured_array`
  - `each_destructured_default`
  - `each_keyed_destructure`
  - `each_block_no_item`
  - `each_block_no_item_multi`
  - `each_fallback`
  - `async_each_basic`
- Added during this audit as ignored gap markers:
  - parser: `each_block_no_item_with_index`
  - analyzer: `validate_each_key_without_as`
  - analyzer: `validate_each_animation_missing_key`
  - analyzer: `validate_each_animation_invalid_placement`
  - analyzer: `validate_each_item_invalid_assignment`
