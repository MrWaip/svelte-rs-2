# Key Block

## Current state
- **Working**: 6/6 use cases
- **Tests**: 7/7 snapshot tests + 4 block-semantics builder unit tests green
- Migrated to `BlockSemantics::Key` (see SEMANTIC_LAYER_ARCHITECTURE.md). Legacy consumer (`template/key_block.rs`) and `Ctx::key_block` wrapper removed. `AsyncEmissionPlan::for_node` no longer called from KeyBlock codegen.
- Last updated: 2026-04-20

## Source

- ROADMAP template item: `{#key}`
- Audit request: `/audit {#key}`

## Syntax variants

- `{#key expression}...{/key}`
- `{#key member.expression}...{/key}`
- `{#key await expression}...{/key}` with experimental async enabled
- `{#key expression}` nested inside an element fragment

## Use cases

- [x] Parse `{#key expression}` into `KeyBlock { expression_span, fragment }` and recover unclosed/extra closing tags.
- [x] Generate client code for a basic reactive key expression with `$.key(...)`.
- [x] Generate async client code for awaited key expressions via `$.async(...)` and `$.get($$key)`.
- [x] Handle `{#key}` nested inside element children without breaking parent fragment traversal or DOM anchors.
- [x] Emit `block_empty` when the key block body contains only whitespace.
- [x] Reference analyzer marks key-block subtrees dynamic via `mark_subtree_dynamic()`; investigated and intentionally not ported — `has_dynamic_children` is only consulted for `ContentStrategy::DynamicText`, which never co-exists with `{#key}` (presence of a block shifts strategy to `SingleBlock`/`Mixed`). Not observable. Verified by `key_block_nested` output match.

## Out of scope

- Legacy-style malformed opening-tag permissiveness such as `{ #key ...}`; our parser rejects it earlier, so the analyzer's `block_unexpected_character` path is intentionally unreachable here

## Reference

- Reference compiler:
- `reference/compiler/phases/1-parse/state/tag.js`
- `reference/compiler/phases/2-analyze/visitors/KeyBlock.js`
- `reference/compiler/phases/2-analyze/visitors/shared/utils.js`
- `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js`
- Our implementation:
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/handlers.rs`
- `crates/svelte_parser/src/tests.rs`
- `crates/svelte_analyze/src/passes/template_scoping.rs`
- `crates/svelte_analyze/src/passes/reactivity.rs`
- `crates/svelte_analyze/src/passes/content_types.rs`
- `crates/svelte_analyze/src/block_semantics/data.rs` (`KeyBlockSemantics`, `KeyAsyncKind`)
- `crates/svelte_analyze/src/block_semantics/builder/key.rs`
- `crates/svelte_analyze/src/tests.rs`
- `crates/svelte_transform/src/lib.rs`
- `crates/svelte_codegen_client/src/template/key_block_semantics.rs`
- `tasks/compiler_tests/cases2/key_block/case.svelte`
- `tasks/compiler_tests/cases2/async_key_basic/case.svelte`
- `tasks/compiler_tests/cases2/key_block_nested/case.svelte`

## Test cases

- [x] `key_block`
- [x] `async_key_basic`
- [x] `key_block_nested`
- [x] `key_block_basic` (parser)
- [x] `key_block_complex_expr` (parser)
- [x] `validate_key_block_empty_warns` (analyzer)
- [x] `fragment_facts_single_child_supports_block_empty`
