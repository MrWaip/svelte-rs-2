# Animate directive

## Current state
- **Working**: 8/10 use cases
- **Missing**: 2/10 use cases
- **Done this session**: `animate_with_const_tag` now matches the reference output. The root cause was shared codegen thunk optimization in `Builder::thunk`: zero-arg method calls such as `item.name.toUpperCase()` were being collapsed into bare member references, which dropped the call when `{@const}` emitted `$.derived(...)`. The optimization is now limited to plain identifier calls, preserving method-call thunks across codegen.
- **Done this session**: `animate_svelte_element` now matches the reference output. Analyze `fragment_facts` now treats `<svelte:element animate:...>` as a direct animate child, so keyed `{#each}` bodies get `EACH_IS_ANIMATED`; codegen reuses the shared `gen_animate_directive` helper inside `gen_svelte_element`, so `$.animation($$element, ...)` is emitted in the element callback just like the regular-element path.
- **Next**: port missing diagnostics for duplicate `animate:` and illegal await expressions
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
- [ ] Reject duplicate `animate:` directives on the same element via `animation_duplicate`. Diagnostic exists in `svelte_diagnostics`, but no analyzer implementation currently emits it.
- [ ] Reject await expressions in `animate:` directive values via `illegal_await_expression`. Diagnostic exists in `svelte_diagnostics`, but no analyzer implementation currently emits it.

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

## Tasks

- Validation:
  add analyzer coverage for `animation_duplicate`, placement outside `{#each}`, and `illegal_await_expression` for animate directive expressions in `crates/svelte_analyze/src/passes/template_validation.rs`.
- Analyzer data:
  keep `fragment_facts`/`each_context_index` as the single source for keyed-each animate classification; extend tests if placement logic changes.
- Codegen:
  confirm `gen_animate_directive` and keyed-each `EACH_IS_ANIMATED` handling remain correct for `RegularElement` and `SvelteElement`.
- Tests:
  keep compiler snapshots for positive output paths in `tasks/compiler_tests/cases2/`; keep negative validation coverage in `crates/svelte_analyze/src/tests.rs` until compiler error-fixture support exists.

## Implementation order

1. Fill analyzer diagnostic gaps (`animation_duplicate`, outside-keyed-each coverage, `illegal_await_expression`).
2. Add/keep positive compiler snapshots for animate-specific codegen shapes.
3. Re-run animate compiler cases and analyzer validation tests.

## Discovered bugs

- OPEN: `animation_duplicate` diagnostic is declared in `svelte_diagnostics` but not emitted from `template_validation`.
- OPEN: `illegal_await_expression` diagnostic is declared in `svelte_diagnostics` but not emitted for animate directive expressions.
- OPEN: compiler test harness only supports successful JS/CSS snapshot cases, so animate error cases remain analyzer-test coverage for now.

## Test cases

- Existing compiler cases: `animate_basic`, `animate_params`, `animate_dotted_name`, `animate_reactive_params`, `animate_with_spread`, `animate_blockers`
- Existing analyzer tests: `validate_each_animation_missing_key`, `validate_each_animation_invalid_placement`, `fragment_facts_track_each_body_child_shape_and_animate`, `fragment_facts_track_svelte_element_animate_in_each_body`
- Added audit cases: `animate_svelte_element`, `animate_with_const_tag`
