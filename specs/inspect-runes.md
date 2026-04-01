# `$inspect` / `$inspect.trace`

## Current state
- **Working**: 5/9 use cases
- **Missing**: 4/9 use cases, all in analyzer validation
- **Next**: extend rune detection and validation so `$inspect().with` and `$inspect.trace` match reference argument-count and placement rules
- Last updated: 2026-04-01

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
- [ ] `$inspect(...)` reports `rune_invalid_arguments_length` when called with zero arguments
- [ ] `$inspect(...).with(callback)` reports `rune_invalid_arguments_length` unless exactly one callback argument is provided
- [ ] `$inspect.trace(...)` reports `rune_invalid_arguments_length` when called with more than one argument
- [ ] `$inspect.trace(...)` reports `inspect_trace_invalid_placement` unless it is the first statement of a function body, and reports `inspect_trace_generator` inside generator functions

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

## Tasks

1. `svelte_analyze`: teach rune detection to distinguish `$inspect`, `$inspect().with`, and `$inspect.trace` from `CallExpression` shape instead of only bare identifier/static-member cases.
2. `svelte_analyze`: port reference validation for inspect runes:
   - `$inspect` requires one or more arguments
   - `$inspect().with` requires exactly one argument
   - `$inspect.trace` allows zero or one arguments
   - `$inspect.trace` must be the first statement of a function body
   - `$inspect.trace` is invalid inside generator functions
3. Tests:
   - keep existing compiler snapshot coverage for dev/prod transforms
   - add focused analyzer tests for the missing diagnostics
   - keep one focused compiler test for docs-level tracing inside `$effect` / `$derived.by`

## Implementation order

1. Add the missing analyzer tests first.
2. Extend inspect rune detection in analyze.
3. Port inspect validation rules.
4. Re-run targeted analyzer and compiler tests.

## Discovered bugs

- OPEN: `svelte_analyze` currently recognizes bare `$inspect(...)` but does not validate `$inspect().with(...)` or `$inspect.trace(...)`, so reference diagnostics are silently missing.

## Test cases

- Existing:
  - `inspect_basic`
  - `inspect_with_callback`
  - `inspect_prod_strip`
  - `inspect_trace_basic`
  - `inspect_trace_contexts`
  - `inspect_trace_prod_strip`
- Added during this audit:
  - analyzer unit tests for missing inspect validation
  - `inspect_trace_reactive_contexts`
