# `{@debug}`

## Current state
- **Working**: 4/6 client use cases
- **Missing**: 2/6 client use cases
- **Next**: wire `config.json.runes` through the compiler test harness, then fix non-runes client emission to match the reference compiler's `$.untrack(() => $.snapshot(...))` wrapping
- Last updated: 2026-04-01

## Source

- ROADMAP Template item: `{@debug}`
- Audit request: `$audit {@debug}`

## Syntax variants

- `{@debug}`
- `{@debug name}`
- `{@debug name1, name2, name3}`
- Invalid forms rejected by the parser: member expressions, calls, and other non-identifier expressions

## Use cases

- [x] Parse empty `{@debug}` tags and preserve the empty identifier list.
- [x] Parse one or more comma-separated identifiers.
- [x] Reject non-identifier arguments with `debug_tag_invalid_arguments`.
- [x] Emit client debug effects in top-level and nested fragments, including each-block context values.
- [ ] Match reference non-runes client output by wrapping each `$.snapshot(...)` in `$.untrack(() => ...)`.
- [ ] Match reference runes-mode analyzer validation for `{@debug}` opening-tag syntax.

## Reference

- Reference docs: `reference/docs/03-template-syntax/11-@debug.md`
- Reference parse: `reference/compiler/phases/1-parse/state/tag.js`
- Reference analyze: `reference/compiler/phases/2-analyze/visitors/DebugTag.js`
- Reference client transform: `reference/compiler/phases/3-transform/client/visitors/DebugTag.js`
- Reference server transform: `reference/compiler/phases/3-transform/server/visitors/DebugTag.js`
- Rust AST node: `crates/svelte_ast/src/lib.rs`
- Rust parser: `crates/svelte_parser/src/lib.rs`
- Rust parser tests: `crates/svelte_parser/src/tests.rs`
- Rust analyze lowering: `crates/svelte_analyze/src/passes/lower.rs`
- Rust analysis data: `crates/svelte_analyze/src/types/data/template_data.rs`
- Rust client codegen: `crates/svelte_codegen_client/src/template/debug_tag.rs`
- Compiler tests: `tasks/compiler_tests/test_v3.rs`

## Tasks

- [ ] Add compiler-test support for `config.json.runes` so non-runes snapshots can be executed by `just test-case`.
- [ ] Add a focused compiler case for non-runes `{@debug}` output and keep it failing until the client emitter matches the reference.
- [ ] Update client `{@debug}` emission to branch on runes mode and generate `$.untrack(() => $.snapshot(expr))` in non-runes mode.
- [ ] Add analyzer validation for debug tags if the project wants parity with the reference `validate_opening_tag` checks in runes mode.

## Implementation order

1. Test harness support for `config.json.runes`
2. Focused non-runes compiler case
3. Client codegen parity fix
4. Optional analyzer validation parity

## Discovered bugs

- OPEN: `tasks/compiler_tests/test_v3.rs` ignores `config.json.runes`, so non-runes compiler cases compare a runes-true Rust compile against a runes-configured reference snapshot.
- OPEN: `crates/svelte_codegen_client/src/template/debug_tag.rs` always emits `$.snapshot(...)` directly and never generates the reference compiler's non-runes `$.untrack(() => $.snapshot(...))` wrapper.

## Test cases

- Existing: `debug_basic`, `debug_in_blocks`, `debug_non_dev`
- Added by this audit: `debug_non_runes_untrack`
