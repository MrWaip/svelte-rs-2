# Animate directive

## Current state
- **Working**: 10/10 use cases
- **Tests**: 14/14 green
- Last updated: 2026-04-09

## Source

- ROADMAP Directives: `animate:`
- Audit request: `/audit animate`

## Syntax variants

`animate:name`
`animate:name={expression}`
`animate:dotted.name`

## Use cases

- [x] Parse `animate:name` and `animate:name={expr}` directives, including dotted names such as `animate:animations.flip`.
- [x] Generate client output for `animate:` on a regular element that is the sole child of a keyed `{#each}` block.
- [x] Pass directive parameter thunks through to `$.animation`, including reactive params and blocker-aware scheduling.
- [x] Mark keyed `{#each}` blocks as animated when their direct child element carries `animate:`.
- [x] Reject `animate:` on keyed `{#each}` blocks without a key via `animation_missing_key`.
- [x] Reject `animate:` when the animated element is not the sole non-trivial child of a keyed `{#each}` block via `animation_invalid_placement`.
- [x] Generate client output for `animate:` on `<svelte:element>` as the sole child of a keyed `{#each}` block.
- [x] Allow comments, whitespace, and `{@const}` alongside an animated keyed-each child without tripping placement validation. Comment/whitespace is analyzer-covered, and `animate_with_const_tag` now matches the reference output.
- [x] Reject duplicate `animate:` directives on the same element via `animation_duplicate`.
- [x] Reject await expressions in `animate:` directive values via `illegal_await_expression`.

## Out of scope

- SSR/runtime behavior of animations
- Runtime semantics of specific animation functions from `svelte/animate`
- Legacy Svelte 4 transition behavior

## Reference

- Reference docs: `reference/docs/03-template-syntax/16-animate.md`
- Reference parse classification: `reference/compiler/phases/1-parse/state/element.js`
- Reference validation: `reference/compiler/phases/2-analyze/visitors/shared/element.js`
- Reference client transform: `reference/compiler/phases/3-transform/client/visitors/EachBlock.js`
- Reference diagnostics: `reference/compiler/errors.js`
- Rust AST: `crates/svelte_ast/src/lib.rs`
- Rust parser scanner: `crates/svelte_parser/src/scanner/mod.rs`
- Rust validation: `crates/svelte_analyze/src/passes/template_validation.rs`
- Rust animate side tables: `crates/svelte_analyze/src/passes/template_side_tables.rs`
- Rust fragment facts: `crates/svelte_analyze/src/types/data/fragment_facts.rs`
- Rust each animate flag: `crates/svelte_analyze/src/types/data/each_context_index.rs`
- Rust client each-block codegen: `crates/svelte_codegen_client/src/template/each_block.rs`
- Rust animate directive emission: `crates/svelte_codegen_client/src/template/events/actions.rs`
- Existing analyzer tests: `crates/svelte_analyze/src/tests.rs`
- Existing compiler cases: `tasks/compiler_tests/cases2/animate_*`

## Test cases

- [x] `animate_basic`
- [x] `animate_params`
- [x] `animate_dotted_name`
- [x] `animate_reactive_params`
- [x] `animate_with_spread`
- [x] `animate_blockers`
- [x] `validate_each_animation_missing_key`
- [x] `validate_each_animation_invalid_placement`
- [x] `validate_each_animation_duplicate`
- [x] `validate_animate_directive_illegal_await_expression`
- [x] `fragment_facts_track_each_body_child_shape_and_animate`
- [x] `fragment_facts_track_svelte_element_animate_in_each_body`
- [x] `animate_svelte_element`
- [x] `animate_with_const_tag`
