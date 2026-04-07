# `use:action`

## Current state
- **Working**: 9/11 use cases
- **Missing**: 2/11 use cases
- **Next**: port parser/codegen support for non-identifier dotted directive segments like `use:a.b-c`, then add analyzer validation for `illegal_await_expression` in action arguments
- Last updated: 2026-04-07

## Source

- ROADMAP directive item: `use:action`
- Audit request: `/audit use:action`

## Syntax variants

- `<div use:myaction>...</div>`
- `<div use:myaction={data}>...</div>`
- `<div use:actions.tooltip>...</div>`
- `<div use:actions.tooltip-extra>...</div>`
- `<div use:focus use:tooltip={config}>...</div>`
- `<svelte:body use:someAction />`

## Use cases

- [x] Parse `use:name` and `use:name={expr}` into `Attribute::UseDirective` with preserved directive-name and optional argument spans.
- [x] Parse dotted directive names whose member segments are valid identifiers, such as `use:actions.tooltip`.
- [ ] Parse dotted directive names whose later segments are not valid identifiers, such as `use:actions.tooltip-extra`. Reference client transform lowers these via computed member access; our scanner stops at `-`.
- [x] Walk action argument expressions through semantics/analyze so symbol references, dynamicity, and async blockers are recorded like other attribute expressions.
- [ ] Reject `await` expressions inside action arguments via `illegal_await_expression`. The diagnostic kind exists, but no `use:`-specific analyzer validation emits it today.
- [x] Emit `$.action(node, handler)` for plain actions on regular elements.
- [x] Pass argument thunks through to `$.action(node, handler, thunk)` for valued actions.
- [x] Preserve source order for multiple actions on the same element.
- [x] Defer non-`bind:this` bindings to init-time `$.effect(...)` when a sibling `use:` directive is present, matching reference ordering with bindings/events.
- [x] Wrap action setup in `$.run_after_blockers(...)` when the action argument depends on async blockers.
- [x] Emit actions inside control-flow blocks and on `<svelte:body>`.

## Out of scope

- SSR behavior for actions
- Legacy action `update`/`destroy` runtime semantics beyond preserving reference client output shape
- Migration guidance toward `{@attach ...}`

## Reference

- `reference/docs/03-template-syntax/13-use.md`
- `reference/docs/05-special-elements/04-svelte-body.md`
- `reference/docs/07-misc/01-best-practices.md`
- `reference/compiler/phases/1-parse/state/element.js`
- `reference/compiler/phases/2-analyze/visitors/UseDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/UseDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/BindDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/utils.js`
- `reference/compiler/types/template.d.ts`
- `reference/compiler/errors.js`
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/scanner/mod.rs`
- `crates/svelte_parser/src/attr_convert.rs`
- `crates/svelte_parser/src/walk_js.rs`
- `crates/svelte_transform/src/lib.rs`
- `crates/svelte_analyze/src/passes/build_component_semantics.rs`
- `crates/svelte_analyze/src/passes/element_flags.rs`
- `crates/svelte_analyze/src/passes/template_side_tables.rs`
- `crates/svelte_analyze/src/passes/template_validation.rs`
- `crates/svelte_analyze/src/validate/non_reactive_update.rs`
- `crates/svelte_analyze/src/walker/traverse.rs`
- `crates/svelte_codegen_client/src/template/attributes.rs`
- `crates/svelte_codegen_client/src/template/bind.rs`
- `crates/svelte_codegen_client/src/template/element.rs`
- `crates/svelte_codegen_client/src/template/events/actions.rs`
- `crates/svelte_codegen_client/src/template/svelte_body.rs`
- `tasks/compiler_tests/cases2/use_action_basic/`
- `tasks/compiler_tests/cases2/use_action_expression/`
- `tasks/compiler_tests/cases2/use_action_reactive/`
- `tasks/compiler_tests/cases2/use_action_dotted/`
- `tasks/compiler_tests/cases2/use_action_dotted_hyphen/`
- `tasks/compiler_tests/cases2/use_action_multiple/`
- `tasks/compiler_tests/cases2/use_action_in_if/`
- `tasks/compiler_tests/cases2/use_action_in_each/`
- `tasks/compiler_tests/cases2/svelte_body_action/`
- `tasks/compiler_tests/cases2/svelte_body_combined/`
- `tasks/compiler_tests/cases2/bind_use_deferral/`
- `tasks/compiler_tests/cases2/action_blockers/`
- `tasks/compiler_tests/test_v3.rs`
- `crates/svelte_analyze/src/tests.rs`

## Tasks

- Parser: extend `crates/svelte_parser/src/scanner/mod.rs` so `use:` dotted names consume the same legacy segment shapes the reference `parse_directive_name` expects, not just `[A-Za-z0-9_]`.
- Analyze: add `illegal_await_expression` validation for `UseDirective` argument expressions in `crates/svelte_analyze/src/passes/template_validation.rs`, matching the reference `UseDirective` visitor.
- Codegen: keep lowering through `gen_use_directive`, but ensure parsed directive-name expressions for non-identifier segments become computed member access in the emitted optional call path.
- Tests: keep passing compiler coverage for plain/valued/dotted/body/blocker cases, keep the new hyphenated dotted-name compiler case ignored until parser/codegen parity lands, and keep the analyzer `await` validation test ignored until diagnostics support is added.

## Implementation order

1. Port dotted directive-name parsing for `use:a.b-c` and re-run the action compiler cases.
2. Port `illegal_await_expression` validation for action arguments and re-run the targeted analyzer test.
3. Unignore the two regression tests and rerun the full `use:` action slice.

## Discovered bugs

- OPEN: `crates/svelte_parser/src/scanner/mod.rs` only consumes `[A-Za-z0-9_]` after `.` in `use:` directive names, so reference-valid names like `use:actions.tooltip-extra` do not round-trip.
- OPEN: `crates/svelte_analyze/src/passes/template_validation.rs` does not emit `illegal_await_expression` for `use:` directive argument expressions.
- OPEN: `tasks/compiler_tests/test_v3.rs` only supports successful snapshot cases, so action validation gaps need analyzer-test coverage until compiler error-fixture support exists.

## Test cases

- [x] `use_action_basic`
- [x] `use_action_expression`
- [x] `use_action_reactive`
- [x] `use_action_dotted`
- [ ] `use_action_dotted_hyphen`
- [x] `use_action_multiple`
- [x] `use_action_in_if`
- [x] `use_action_in_each`
- [x] `svelte_body_action`
- [x] `svelte_body_combined`
- [x] `bind_use_deferral`
- [x] `action_blockers`
- [ ] analyzer validation: `await` inside `use:` directive value
