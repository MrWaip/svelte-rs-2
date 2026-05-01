# Text / ExpressionTag

## Current state
- **Working**: 8/8 use cases
- **Tests**: 17/17 green
- Last updated: 2026-05-01

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
- [x] Concat-context expression interpolations whose value is statically known to be non-nullish skip the `?? ""` fallback. `is_node_expr_definitely_defined` in `crates/svelte_codegen_client/src/codegen/mod.rs` returns `true` for `BinaryExpression`, so `${a() + b()}` inside a `set_text` template literal emits without the fallback (test: `text_expression_binary_no_nullish_fallback`).

## Out of scope

- Dev-only equality rewrites via `$.strict_equals(...)` / `$.equals(...)`; if this parity work is revived, track it in a separate dev-codegen spec instead of the roadmap-closed text/expression feature

## Reference

- Reference docs:
- `reference/docs/03-template-syntax/01-basic-markup.md`
- Reference compiler:
- `reference/compiler/phases/1-parse/state/text.js`
- `reference/compiler/phases/2-analyze/visitors/Text.js`
- `reference/compiler/phases/2-analyze/visitors/ExpressionTag.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/fragment.js`
- `reference/compiler/phases/3-transform/client/visitors/RegularElement.js`
- `reference/compiler/phases/3-transform/client/visitors/BinaryExpression.js`
- Rust implementation:
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/scanner/mod.rs`
- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/passes/content_types.rs`
- `crates/svelte_codegen_client/src/lib.rs`
- `crates/svelte_codegen_client/src/template/expression.rs`
- `crates/svelte_codegen_client/src/template/element.rs`
- `crates/svelte_codegen_client/src/template/title_element.rs`
- `tasks/compiler_tests/test_v3.rs`

## Test cases

- [x] `single_text_node`
- [x] `single_interpolation`
- [x] `static_interpolation`
- [x] `inline_await_text_concat`
- [x] `title_variants`
- [x] `svg_inner_whitespace_trimming`
- [x] `svg_text_preserves_whitespace`
- [x] `ts_strip_expression_tag`
- [x] `text_entity_decoding`
- [x] `text_entity_decoding_root`
- [x] `title_entity_decoding`
- [x] `invalid_text_parent_uses_topology_ancestor_lookup`
- [x] `validate_text_invalid_placement` (analyzer)
- [x] `validate_expression_tag_invalid_placement` (analyzer)
- [x] `validate_text_bidirectional_control_warning` (analyzer)
- [x] `validate_text_bidirectional_control_warning_ignored` (analyzer)
- [x] `text_expression_binary_no_nullish_fallback`
