# Transitions

## Current state
- **Working**: 16/16 use cases
- **Tests**: 18/19 green
- Last updated: 2026-04-09

## Source

- ROADMAP Directives: `transition:` / `in:` / `out:`
- Audit request: `/audit transition: / in: / out:`

## Syntax variants

`transition:name`
`transition:name={expression}`
`transition:name|local`
`transition:name|global`
`transition:dotted.name`
`in:name`
`in:name={expression}`
`in:name|local`
`in:name|global`
`in:dotted.name`
`out:name`
`out:name={expression}`
`out:name|local`
`out:name|global`
`out:dotted.name`

## Use cases

- [x] Parse `transition:`, `in:`, and `out:` directives into one AST node with preserved direction flags.
- [x] Parse optional directive values and dotted directive names such as `transition:custom.fn`.
- [x] Preserve `local` and `global` modifiers for transition directives.
- [x] Emit `$.transition` for bidirectional `transition:` directives on regular elements.
- [x] Emit `$.transition` flags correctly for intro-only `in:` and outro-only `out:`.
- [x] Emit separate `$.transition` calls when an element has both `in:` and `out:` directives.
- [x] Pass directive value thunks through to `$.transition`, including reactive parameter objects.
- [x] Delay transition setup behind async blockers when the directive value depends on blocker-producing expressions.
- [x] Emit local/global flags in client codegen for `transition:fade|local` and `transition:fade|global`.
- [x] Preserve the special `{:else if}` local-transition path so flattened else-if branches compile through `$.if(..., true)` when they contain transitions.
- [x] Reject duplicate `transition:` directives on one element via `transition_duplicate`.
- [x] Reject duplicate `in:` directives on one element via `transition_duplicate`.
- [x] Reject duplicate `out:` directives on one element via `transition_duplicate`.
- [x] Reject `transition:` together with `in:` on one element via `transition_conflict`.
- [x] Reject `transition:` together with `out:` on one element via `transition_conflict`.
- [x] Reject `await` expressions inside transition directive values via `illegal_await_expression`.

## Out of scope

- SSR transition output
- Runtime behavior of specific transition functions from `svelte/transition`
- Transition events (`onintrostart`, `onintroend`, `onoutrostart`, `onoutroend`) beyond existing event-attribute support

## Reference

- Reference docs: `reference/docs/03-template-syntax/14-transition.md`
- Reference docs: `reference/docs/03-template-syntax/15-in-and-out.md`
- Reference parse classification: `reference/compiler/phases/1-parse/state/element.js`
- Reference transition analyze visitor: `reference/compiler/phases/2-analyze/visitors/TransitionDirective.js`
- Reference element validation: `reference/compiler/phases/2-analyze/visitors/shared/element.js`
- Reference client transform: `reference/compiler/phases/3-transform/client/visitors/TransitionDirective.js`
- Reference if-block transition note: `reference/compiler/phases/3-transform/client/visitors/IfBlock.js`
- Reference diagnostics: `reference/compiler/errors.js`
- Rust AST: `crates/svelte_ast/src/lib.rs`
- Rust parser scanner: `crates/svelte_parser/src/scanner/mod.rs`
- Rust attr conversion: `crates/svelte_parser/src/attr_convert.rs`
- Rust analyzer semantic walk: `crates/svelte_analyze/src/passes/build_component_semantics.rs`
- Rust analyzer validation: `crates/svelte_analyze/src/passes/template_validation.rs`
- Rust analyzer tests: `crates/svelte_analyze/src/tests.rs`
- Rust client element handling: `crates/svelte_codegen_client/src/template/element.rs`
- Rust client transition emission: `crates/svelte_codegen_client/src/template/events/actions.rs`
- Existing compiler cases: `tasks/compiler_tests/cases2/transition_*`

## Test cases

- [x] `transition_basic`
- [x] `transition_params`
- [x] `transition_in`
- [x] `transition_out`
- [x] `transition_in_out_separate`
- [x] `transition_local`
- [x] `transition_global`
- [x] `transition_dotted_name`
- [x] `transition_in_if`
- [x] `transition_reactive_params`
- [x] `transition_blockers`
- [x] `transition_elseif_local`
- [x] Analyzer coverage for duplicate `transition:`, duplicate `in:`, duplicate `out:`, conflicting `transition:` + `in:`, conflicting `transition:` + `out:`, and `await` inside transition directive values
- [x] `validate_transition_duplicate_transition`
- [x] `validate_transition_duplicate_in`
- [x] `validate_transition_duplicate_out`
- [x] `validate_transition_conflict_in`
- [x] `validate_transition_conflict_out`
- [ ] `validate_transition_illegal_await_expression`
