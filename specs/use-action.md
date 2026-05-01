# `use:action`

## Current state
- **Working**: 11/11 use cases
- **Tests**: 13/13 green
- Last updated: 2026-05-01

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
- [x] Parse dotted directive names whose later segments are not valid identifiers, such as `use:actions.tooltip-extra`. Scanner now reads `-` in segments; `walk_js.rs` converts to bracket notation before OXC parsing (test: `use_action_dotted_hyphen`).
- [x] Walk action argument expressions through semantics/analyze so symbol references, dynamicity, and async blockers are recorded like other attribute expressions.
- [x] Reject `await` expressions inside action arguments via `illegal_await_expression`. Analyzer validation now reads `ExpressionInfo.has_await` for `UseDirective` argument expressions and emits the diagnostic in `template_validation.rs`.
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
- `experimental_async` diagnostic for `await` inside `use:` argument when `experimental.async` is disabled — owned by `specs/experimental-async.md`

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

## Test cases

- [x] `use_action_basic`
- [x] `use_action_expression`
- [x] `use_action_reactive`
- [x] `use_action_dotted`
- [x] `use_action_dotted_hyphen`
- [x] `use_action_multiple`
- [x] `use_action_in_if`
- [x] `use_action_in_each`
- [x] `svelte_body_action`
- [x] `svelte_body_combined`
- [x] `bind_use_deferral`
- [x] `action_blockers`
- [x] analyzer validation: `await` inside `use:` directive value
