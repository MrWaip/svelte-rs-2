# Text / ExpressionTag

## Current state
- **Working**: 7/7 use cases
- **Missing**: 0 use cases
- **Next**: feature complete for current client-side scope; keep parity checks in `/qa` and follow-up audits
- Last updated: 2026-04-02

## Source

- `ROADMAP.md` Template item: `Text / ExpressionTag`
- Audit request: `Template -> Text / ExpressionTag`

## Syntax variants

- Plain text nodes in fragments and element children
- Standalone expression tags: `{expression}`
- Mixed text and expression sequences: `Hello {name}!`
- Title text sequences in `<svelte:head><title>...</title></svelte:head>`
- Async expression tags inside text content
- Text containing HTML character references such as `&amp;` and `&lt;`

## Use cases

- [x] Standalone static text nodes compile to static DOM text
- [x] Standalone expression tags compile at root and inside elements
- [x] Mixed text and expression sequences compile for root, regular elements, and `<title>`
- [x] SVG whitespace handling works for ignorable inter-element whitespace and `<text>` content
- [x] Text entities decode correctly for mixed text/expression concatenation in root fragments, regular elements, and `<title>`
- [x] Template validation rejects invalid text / expression placement with `node_invalid_placement`
- [x] Bidirectional control character warnings in text nodes are implemented, including `svelte-ignore` handling

## Reference

- Reference docs:
- `reference/docs/03-template-syntax/01-basic-markup.md`
- Reference compiler:
- `reference/compiler/phases/1-parse/state/text.js`
- `reference/compiler/phases/2-analyze/visitors/Text.js`
- `reference/compiler/phases/2-analyze/visitors/ExpressionTag.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/fragment.js`
- `reference/compiler/phases/3-transform/client/visitors/RegularElement.js`
- Rust implementation:
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/scanner/mod.rs`
- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/passes/content_types.rs`
- `crates/svelte_codegen_client/src/template/expression.rs`
- `crates/svelte_codegen_client/src/template/element.rs`
- `crates/svelte_codegen_client/src/template/title_element.rs`
- `tasks/compiler_tests/test_v3.rs`

## Tasks

- [x] Parse and store decoded text-node values so runtime text concatenation matches reference semantics for HTML entities
- [x] Thread decoded text through lowering/content classification without breaking whitespace-trimming rules
- [x] Add template validation for invalid text / expression placement using analyzer-side parent-context checks
- [x] Add bidirectional control character warnings for text nodes, including `svelte-ignore` handling
- [x] Expand compiler coverage for decoded text and diagnostics once behavior is implemented

## Implementation order

- 1. Fix decoded text semantics and make `text_entity_decoding` pass
- 2. Port analyzer diagnostics for text / expression placement
- 3. Port bidirectional control character warnings and add focused analyzer coverage

## Discovered bugs

- FIXED: text participating in runtime concatenation now uses parser-decoded text payloads, so `&amp;` and similar entities emit decoded characters in runtime strings

## Test cases

- Existing:
- `single_text_node`
- `single_interpolation`
- `static_interpolation`
- `inline_await_text_concat`
- `title_variants`
- `svg_inner_whitespace_trimming`
- `svg_text_preserves_whitespace`
- `ts_strip_expression_tag`
- Added in this audit:
- `text_entity_decoding`
- `text_entity_decoding_root`
- `title_entity_decoding`
- Analyzer unit coverage:
- `validate_text_invalid_placement`
- `validate_expression_tag_invalid_placement`
- `validate_text_bidirectional_control_warning`
- `validate_text_bidirectional_control_warning_ignored`
