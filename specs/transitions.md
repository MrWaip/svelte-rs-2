# Transitions

## Current state
- **Working**: 11/16 use cases
- **Missing**: 5/16 use cases
- **Next**: port analyzer validation for `transition_duplicate`, `transition_conflict`, and `illegal_await_expression`
- Last updated: 2026-04-07

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
- [ ] Reject duplicate `transition:` directives on one element via `transition_duplicate`. Diagnostic kind exists in `svelte_diagnostics`, but `template_validation` does not emit it.
- [ ] Reject duplicate `in:` directives on one element via `transition_duplicate`. Diagnostic kind exists in `svelte_diagnostics`, but `template_validation` does not emit it.
- [ ] Reject duplicate `out:` directives on one element via `transition_duplicate`. Diagnostic kind exists in `svelte_diagnostics`, but `template_validation` does not emit it.
- [ ] Reject `transition:` together with `in:` on one element via `transition_conflict`. Diagnostic kind exists in `svelte_diagnostics`, but `template_validation` does not emit it.
- [ ] Reject `transition:` together with `out:` on one element via `transition_conflict`. Diagnostic kind exists in `svelte_diagnostics`, but `template_validation` does not emit it.
- [ ] Reject `await` expressions inside transition directive values via `illegal_await_expression`. Reference analyze handles this in `TransitionDirective.js`; our analyzer currently does not.

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

## Tasks

- Parser:
  keep `TransitionDirective` direction/modifier parsing in `crates/svelte_parser/src/scanner/mod.rs` and `crates/svelte_parser/src/attr_convert.rs` as the single source for `transition:` / `in:` / `out:`.
- Analyzer validation:
  add duplicate/conflict detection for transition directives in `crates/svelte_analyze/src/passes/template_validation.rs`, matching reference `validate_element`.
- Analyzer async validation:
  add `illegal_await_expression` coverage for transition directive values in `crates/svelte_analyze/src/passes/template_validation.rs`.
- Codegen:
  confirm the `{:else if}` transition path continues to compile through the `$.if(..., true)` branch and keep `gen_transition_directive` aligned with reference flag ordering.
- Tests:
  keep successful codegen coverage in `tasks/compiler_tests/cases2/transition_*`; keep negative validation coverage in `crates/svelte_analyze/src/tests.rs` until compiler error-fixture support exists.

## Implementation order

1. Verify the new positive compiler case for the `{:else if}` path.
2. Port analyzer validation for duplicate/conflict transition directives.
3. Port analyzer validation for `illegal_await_expression` in transition directive values.
4. Re-run transition compiler cases plus analyzer validation tests.

## Discovered bugs

- OPEN: `transition_duplicate` is declared in `svelte_diagnostics` but not emitted from `crates/svelte_analyze/src/passes/template_validation.rs`.
- OPEN: `transition_conflict` is declared in `svelte_diagnostics` but not emitted from `crates/svelte_analyze/src/passes/template_validation.rs`.
- OPEN: `illegal_await_expression` is declared in `svelte_diagnostics`, but transition directive expressions are not checked for it.
- OPEN: compiler test harness only supports successful snapshot cases, so transition validation failures need analyzer-test coverage for now.

## Test cases

- Existing compiler cases: `transition_basic`, `transition_params`, `transition_in`, `transition_out`, `transition_in_out_separate`, `transition_local`, `transition_global`, `transition_dotted_name`, `transition_in_if`, `transition_reactive_params`, `transition_blockers`
- Added compiler case: `transition_elseif_local`
- Added analyzer cases: duplicate `transition:`, duplicate `in:`, duplicate `out:`, conflicting `transition:` + `in:`, conflicting `transition:` + `out:`, `await` inside transition directive value
