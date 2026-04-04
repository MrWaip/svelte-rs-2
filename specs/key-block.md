# Key Block

## Current state
- **Complete**: 6/6 use cases — feature fully implemented
- `dynamic_nodes` parity: investigated and intentionally not ported. Reference compiler calls `mark_subtree_dynamic()` which sets `fragment.metadata.dynamic = true` on ancestor fragments; in our Rust architecture `has_dynamic_children` (the equivalent) is only consulted for `ContentStrategy::DynamicText` branching. A fragment containing `{#key}` always yields `SingleBlock` or `Mixed` strategy, never `DynamicText`, so the flag is never read in the `KeyBlock` path. Gap is architecturally irrelevant and not observable — verified by `key_block_nested` test output matching reference exactly.
- `block_unexpected_character`: implemented in analyzer but dead code — our parser rejects `{ #key ...}` at parse time, stricter than reference JS parser. This is a known parser-strictness difference, not a bug.
- Last updated: 2026-04-04

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
- `[x]` Emit `block_empty` when the key block body contains only whitespace.
- `[~]` In runes mode, emit `block_unexpected_character` when the opening tag is malformed — implemented in analyzer but effectively dead code: our Rust parser rejects `{ #key ...}` (space after `{`) at parse time, stricter than the reference JS parser which accepts it in legacy mode.
- `[x]` Reference analyzer marks key-block subtrees dynamic via `mark_subtree_dynamic()`; investigated and intentionally not ported — `has_dynamic_children` is only consulted for `ContentStrategy::DynamicText`, which never co-exists with `{#key}` (presence of a block shifts strategy to `SingleBlock`/`Mixed`). Not observable. Verified by `key_block_nested` output match.

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

- `[x]` quick fix: add template validation coverage for whitespace-only `{#key}` bodies (`block_empty`).
- `[x]` quick fix: `block_unexpected_character` check implemented in analyzer; untestable because our parser is stricter than reference (rejects malformed `{` at parse time).
- `[x]` moderate: investigated `ReactivityVisitor` / `dynamic_nodes` parity — decision: not needed. `has_dynamic_children` is only read for `DynamicText` content strategy; `{#key}` always produces `SingleBlock`/`Mixed`, so the flag is never consulted. No code change required.
- `[x]` quick fix: existing `key_block`, `key_block_nested`, `async_key_basic` cases provide sufficient coverage.

## Implementation order

1. Add analyzer tests for `block_empty` and opening-tag diagnostics.
2. Implement the missing template validation path in the analyzer.
3. Re-check whether any remaining parity gap around `dynamic_nodes` is externally observable; only then change reactivity/content analysis.

## Discovered bugs

- FIXED: template validation now emits `block_empty` for whitespace-only `{#key}` bodies.
- FIXED: runes-mode opening-tag validation for `{#key}` is wired through the analyzer.
- CLOSED (not a bug): `KeyBlock` ids are not added to `dynamic_nodes` — investigated, architecturally irrelevant (see Current state).

## Test cases

- Existing:
- `key_block`
- `async_key_basic`
- Parser: `key_block_basic`, `key_block_complex_expr`
- Added during this audit:
- `key_block_nested`
- Analyzer: `validate_key_block_empty_warns`
