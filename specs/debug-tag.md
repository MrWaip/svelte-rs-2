# `{@debug}`

## Current state
- **Working**: 6/6 client use cases — feature complete
- **Last updated**: 2026-04-04
- **Next**: No remaining work — all use cases complete

## Source

- ROADMAP Template item: `{@debug}`
- Audit request: `$audit {@debug}`

## Syntax variants

- `{@debug}`
- `{@debug name}`
- `{@debug name1, name2, name3}`
- Invalid forms rejected by the parser: member expressions, calls, and other non-identifier expressions

## Use cases

- [x] Parse empty `{@debug}` tags and preserve the empty identifier list
- [x] Parse one or more comma-separated identifiers
- [x] Reject non-identifier arguments with `debug_tag_invalid_arguments`
- [x] Emit client debug effects in top-level and nested fragments, including each-block context values
- [x] Match reference non-runes client output by wrapping each `$.snapshot(...)` in `$.untrack(() => ...)`
- [x] Match reference runes-mode analyzer validation for `{@debug}` opening-tag syntax

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

## Test cases

- [x] `debug_basic`
- [x] `debug_in_blocks`
- [x] `debug_non_dev`
- [x] `debug_non_runes_untrack`
