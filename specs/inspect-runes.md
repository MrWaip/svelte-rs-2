# `$inspect` / `$inspect.trace`

## Current state
- **Working**: 10/10 use cases
- **Tests**: 26/26 green
- Last updated: 2026-04-11

## Source

- `ROADMAP.md` item: `$inspect` / `$inspect.trace`
- User request: `/audit $inspect / $inspect.trace`

## Syntax variants

- `$inspect(a, b, ...)`
- `$inspect(a, b, ...).with(callback)`
- `$inspect.trace()`
- `$inspect.trace(label)`

## Use cases

- [x] `$inspect(...)` in dev rewrites to `$.inspect(...)` with `console.log`
- [x] `$inspect(...).with(callback)` in dev rewrites to `$.inspect(...)` with the provided callback
- [x] `$inspect(...)` is stripped in prod builds
- [x] `$inspect.trace(label?)` rewrites the surrounding function body to `$.trace(...)`
- [x] `$inspect.trace(label?)` works in async functions and template event handlers
- [x] `$inspect(...)` reports `rune_invalid_arguments_length` when called with zero arguments
- [x] `$inspect(...).with(callback)` reports `rune_invalid_arguments_length` unless exactly one callback argument is provided
- [x] `$inspect.trace(...)` reports `rune_invalid_arguments_length` when called with more than one argument
- [x] `$inspect.trace(...)` reports `inspect_trace_invalid_placement` unless it is the first statement of a function body, and reports `inspect_trace_generator` inside generator functions
- [x] `$inspect(...)` and `$inspect(...).with(callback)` do not emit `state_referenced_locally` for inspected rune values

## Reference

- Reference compiler:
  - `reference/docs/02-runes/07-$inspect.md`
  - `reference/docs/07-misc/01-best-practices.md`
  - `reference/compiler/phases/2-analyze/visitors/CallExpression.js`
  - `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`
  - `reference/compiler/phases/3-transform/client/visitors/ExpressionStatement.js`
  - `reference/compiler/phases/3-transform/utils.js`
- Rust implementation:
  - `crates/svelte_analyze/src/utils/script_info.rs`
  - `crates/svelte_analyze/src/validate/runes.rs`
  - `crates/svelte_codegen_client/src/script/traverse/inspect.rs`
  - `crates/svelte_codegen_client/src/script/traverse/statement_passes.rs`
  - `crates/svelte_codegen_client/src/template/events/handlers.rs`
  - `tasks/compiler_tests/cases2/inspect_basic/`
  - `tasks/compiler_tests/cases2/inspect_with_callback/`
  - `tasks/compiler_tests/cases2/inspect_trace_basic/`
  - `tasks/compiler_tests/cases2/inspect_trace_contexts/`

## Test cases

- [x] `inspect_basic`
- [x] `inspect_with_callback`
- [x] `inspect_prod_strip`
- [x] `inspect_trace_basic`
- [x] `inspect_trace_contexts`
- [x] `inspect_trace_prod_strip`
- [x] `inspect_trace_reactive_contexts`
- [x] analyzer unit tests for inspect validation
- [x] `validate_inspect_requires_arguments`
- [x] `validate_inspect_zero_args`
- [x] `validate_inspect_one_or_more_args_ok`
- [x] `validate_inspect_with_requires_callback`
- [x] `validate_inspect_with_wrong_arg_count_zero`
- [x] `validate_inspect_with_wrong_arg_count_two`
- [x] `validate_inspect_with_one_arg_ok`
- [x] `validate_inspect_derived_no_state_referenced_locally_warning`
- [x] `validate_inspect_trace_wrong_arg_count`
- [x] `validate_inspect_trace_too_many_args`
- [x] `validate_inspect_trace_invalid_placement`
- [x] `validate_inspect_trace_invalid_placement_top_level`
- [x] `validate_inspect_trace_invalid_placement_not_first_stmt`
- [x] `validate_inspect_trace_zero_args_ok`
- [x] `validate_inspect_trace_one_arg_ok`
- [x] `validate_inspect_trace_valid_in_arrow`
- [x] `validate_inspect_trace_generator_invalid`
- [x] `validate_inspect_trace_generator_rejected`
