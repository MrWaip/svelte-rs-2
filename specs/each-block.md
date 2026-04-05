# Each Block

## Current state
- **Working**: 15/15 core client-side `{#each}` use cases implemented and passing.
- **Open gap**: When an each block's inner scope has a declaration that shadows an outer scope binding (e.g. `{@const a = ...}` shadowing a snippet named `a`), the reference compiler sets `collection_id` and emits `$$index, $$array` as extra render-callback params. The Rust codegen lacks this `collection_id` concept entirely — new use case added below.
- **Next**: implement `collection_id` inner-scope shadowing logic in `crates/svelte_codegen_client/src/template/each_block.rs` (reference: `EachBlock.js` lines 112–123 and 316–318)
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

- [x] Basic item iteration: `{#each items as item}`.
- [x] Item iteration with index: `{#each items as item, i}`.
- [x] Keyed each blocks, including key expressions that reference the index.
- [x] Key-is-item optimization in runes mode.
- [x] Destructured object and array patterns.
- [x] Destructured defaults inside each context.
- [x] Item-less each blocks: `{#each items}`.
- [x] Item-less each blocks with index: `{#each { length: 8 }, rank}`.
- [x] `{:else}` fallback blocks for empty collections.
- [x] Bind/group and bind:this interactions with parent each scopes.
- [x] `animate:` codegen flags for keyed each blocks that already satisfy placement constraints.
- [x] Diagnostic: keyed each without `as` should raise `each_key_without_as`.
- [x] Diagnostic: `animate:` outside a keyed each or on a non-sole child should raise `animation_invalid_placement`.
- [x] Diagnostic: `animate:` inside an unkeyed each should raise `animation_missing_key`.
- [x] Diagnostic: runes-mode reassignment or binding to an each item should raise `each_item_invalid_assignment`.
- [ ] Inner-scope shadowing: when an each block's inner scope declares a binding that shadows an outer scope name, emit `$$index, $$array` as extra render-callback params (reference: `collection_id` logic in `EachBlock.js` lines 112–123 and 316–318).
- [ ] Parser: support `{#each expression, index}` without `as` (currently requires `as` to finalize the each header).
- [ ] Diagnostic: `{#each expression, index}` without `as` — currently unimplemented in `crates/svelte_analyze/src/validate/mod.rs`.

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

## Test cases

- [x] `each_block`
- [x] `each_keyed_index`
- [x] `each_key_uses_index`
- [x] `each_key_is_item`
- [x] `each_destructured_obj`
- [x] `each_destructured_array`
- [x] `each_destructured_default`
- [x] `each_keyed_destructure`
- [x] `each_block_no_item`
- [x] `each_block_no_item_multi`
- [x] `each_fallback`
- [x] `async_each_basic`
- [x] `animate_basic`
- [x] `animate_params`
- [x] `animate_dotted_name`
- [x] `animate_reactive_params`
- [x] `animate_blockers`
- [x] `animate_with_spread`
- [ ] `each_block_no_item_with_index`
- [ ] `validate_each_key_without_as`
- [ ] `validate_each_animation_missing_key`
- [ ] `validate_each_animation_invalid_placement`
- [ ] `validate_each_item_invalid_assignment`
