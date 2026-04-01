# Key Block

## Current state
- **Working**: 3/6 use cases
- **Missing**: 2 use cases
- **Partial**: 1 use case
- **Next**: add template validation for `block_empty` and runes-mode opening-tag diagnostics, then decide whether `KeyBlock` should be marked dynamic in analysis for stricter parity with the reference compiler
- Last updated: 2026-04-01

## Source

- ROADMAP template item: `{#key}`
- Audit request: `/audit {#key}`

## Syntax variants

- `{#key expression}...{/key}`
- `{#key member.expression}...{/key}`
- `{#key await expression}...{/key}` with experimental async enabled
- `{#key expression}` nested inside an element fragment

## Use cases

- `[x]` Parse `{#key expression}` into `KeyBlock { expression_span, fragment }` and recover unclosed/extra closing tags.
- `[x]` Generate client code for a basic reactive key expression with `$.key(...)`.
- `[x]` Generate async client code for awaited key expressions via `$.async(...)` and `$.get($$key)`.
- `[x]` Handle `{#key}` nested inside element children without breaking parent fragment traversal or DOM anchors.
- `[ ]` Emit `block_empty` when the key block body contains only whitespace.
- `[ ]` In runes mode, emit `block_unexpected_character` when the opening tag is malformed (reference `validate_opening_tag` parity).
- `[~]` Reference analyzer marks key-block subtrees dynamic; this repo lowers and codegens `{#key}` correctly for audited cases, but does not explicitly insert `KeyBlock` ids into `dynamic_nodes`.

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
- `crates/svelte_analyze/src/tests.rs`
- `crates/svelte_transform/src/lib.rs`
- `crates/svelte_codegen_client/src/template/key_block.rs`
- `tasks/compiler_tests/cases2/key_block/case.svelte`
- `tasks/compiler_tests/cases2/async_key_basic/case.svelte`
- `tasks/compiler_tests/cases2/key_block_nested/case.svelte`

## Tasks

- `[ ]` quick fix: add template validation coverage for whitespace-only `{#key}` bodies (`block_empty`).
- `[ ]` quick fix: add runes-mode opening-tag validation coverage for `{#key}`.
- `[ ]` moderate: decide whether `ReactivityVisitor` should mark `KeyBlock` as dynamic to match reference analyzer behavior, then add the smallest test that proves the need.
- `[ ]` quick fix: keep expanding `{#key}` coverage only with narrowly scoped cases; avoid broad refactors.

## Implementation order

1. Add analyzer tests for `block_empty` and opening-tag diagnostics.
2. Implement the missing template validation path in the analyzer.
3. Re-check whether any remaining parity gap around `dynamic_nodes` is externally observable; only then change reactivity/content analysis.

## Discovered bugs

- OPEN: template validation currently does not emit `block_empty` for whitespace-only `{#key}` bodies.
- OPEN: runes-mode opening-tag validation for `{#key}` is not wired through the analyzer.
- OPEN: `KeyBlock` ids are not explicitly added to `dynamic_nodes`, unlike the reference analyzer.

## Test cases

- Existing:
- `key_block`
- `async_key_basic`
- Parser: `key_block_basic`, `key_block_complex_expr`
- Added during this audit:
- `key_block_nested`
- Analyzer: `validate_key_block_empty_warns`
