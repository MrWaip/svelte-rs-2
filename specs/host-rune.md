# $host

## Current state
- **Working**: 4/4 use cases
- **Missing**: 0
- **Next**: feature complete for current scope
- Last updated: 2026-04-04

## Source

- ROADMAP item: `Runes & Script -> $host`
- Audit request: `/audit $host`

## Syntax variants

- `const host = $host();`
- `let host = $host();`
- `$host()` inside a custom-element instance script
- Invalid forms to validate:
  - `$host(1)`
  - `$host(...args)`
  - `$host()` outside custom-element instance scripts
  - `$host()` inside `<script module>`

## Use cases

- [x] Basic client transform for `$host()` inside a custom element instance script.
  Evidence: `tasks/compiler_tests/cases2/host_basic`
- [x] Reject `$host()` arguments with `rune_invalid_arguments`.
- [x] Reject `$host()` outside custom element instance scripts with `host_invalid_placement`.
- [x] `$host()` coexists with `$props()` in custom elements — rest props exclude `$$host`.
  Evidence: `tasks/compiler_tests/cases2/host_props_rest`

## Reference

- Reference compiler
  - `reference/compiler/phases/2-analyze/visitors/CallExpression.js`
  - `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`
  - `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`
  - `reference/compiler/phases/3-transform/client/transform-client.js`
  - `reference/compiler/errors.js`
- Rust implementation
  - `crates/svelte_analyze/src/utils/script_info.rs`
  - `crates/svelte_codegen_client/src/script/traverse/runes.rs`
  - `crates/svelte_codegen_client/src/script/props.rs`
  - `crates/svelte_diagnostics/src/lib.rs`
  - `tasks/compiler_tests/cases2/host_basic`

## Tasks

- [x] Analyze: add `$host()` validation in the rune call validation path.
  Files: `crates/svelte_analyze/src/validate/runes.rs`, `crates/svelte_analyze/src/tests.rs`
- [x] Codegen: include `$$host` in the excluded names passed to `$.rest_props(...)` for custom elements.
  Files: `crates/svelte_codegen_client/src/script/props.rs`, `crates/svelte_codegen_client/src/script/pipeline.rs`, `crates/svelte_codegen_client/src/script/model.rs`
- [x] Tests: unignore added parity tests.
  Files: `crates/svelte_analyze/src/tests.rs`, `tasks/compiler_tests/test_v3.rs`

## Implementation order

1. Add analyzer diagnostics for invalid arguments and placement.
2. Fix custom-element rest-props exclusion for `$$host`.
3. Unignore `$host` parity tests and run `just test-analyzer` plus `just test-case host_props_rest`.

## Discovered bugs

- FIXED: `crates/svelte_codegen_client/src/script/props.rs` did not exclude `$$host` from `$props()` rest props in custom elements.
- FIXED: analyzer had no `$host()` validation despite `HostInvalidPlacement` diagnostic existing.

## Test cases

- Existing
  - `host_basic`
- Added during audit
  - `host_props_rest` (ignored compiler parity test)
  - `validate_host_invalid_placement_without_custom_element` (ignored analyzer parity test)
  - `validate_host_invalid_arguments` (ignored analyzer parity test)
