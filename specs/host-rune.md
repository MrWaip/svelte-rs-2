# $host

## Current state
- **Working**: 4/5 use cases
- **Tests**: 1/4 green
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

- [x] Basic client transform for `$host()` inside a custom element instance script. (test: `host_basic`)
- [x] Reject `$host()` arguments with `rune_invalid_arguments`. (test: `validate_host_invalid_arguments`)
- [x] Reject `$host()` outside custom element instance scripts with `host_invalid_placement`. (test: `validate_host_invalid_placement_without_custom_element`)
- [ ] Reject `$host()` inside `<script module>` even in custom elements (`host_invalid_placement`) — reference: `ast_type === 'module'` check in `CallExpression.js`, distinct from the non-custom-element case
- [x] `$host()` coexists with `$props()` in custom elements — rest props exclude `$$host`. (test: `host_props_rest`)

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

## Test cases

- [x] `host_basic`
- [ ] `host_props_rest`
- [ ] `validate_host_invalid_placement_without_custom_element`
- [ ] `validate_host_invalid_arguments`
