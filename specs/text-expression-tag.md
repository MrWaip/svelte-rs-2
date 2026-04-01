# Text / ExpressionTag

## Current state
- **Working**: 4/7 use cases
- **Missing**: 2 use cases
- **Next**: fix decoded text semantics for mixed text/expression output, then port template text diagnostics
- Last updated: 2026-04-01

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
- [~] Text entities are only correct when the browser decodes a static template; mixed text/expression concatenation currently uses raw source text instead of decoded text
- [ ] Template validation for invalid text / expression placement is not implemented (`node_invalid_placement`)
- [ ] Bidirectional control character warnings in text nodes are not implemented

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

- [ ] Parse or store decoded text-node values so runtime text concatenation matches reference semantics for HTML entities
- [ ] Thread decoded text through lowering/content classification without breaking whitespace-trimming rules
- [ ] Add template validation for invalid text / expression placement using analyzer-side parent-context checks
- [ ] Add bidirectional control character warnings for text nodes, including `svelte-ignore` handling
- [ ] Expand compiler coverage for decoded text and diagnostics once behavior is implemented

## Implementation order

- 1. Fix decoded text semantics and make `text_entity_decoding` pass
- 2. Port analyzer diagnostics for text / expression placement
- 3. Port bidirectional control character warnings and add focused analyzer coverage

## Discovered bugs

- OPEN: text participating in runtime concatenation is emitted from raw source slices, so `&amp;` and similar entities remain escaped in output strings instead of decoding to their character values

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
