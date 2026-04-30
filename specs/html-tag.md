# `{@html}`

## Current state
- **Working**: 10/10 use cases
- **Tests**: 12/12 green
- Last updated: 2026-04-30

## Source

- ROADMAP template item: `{@html}`
- Audit request: `/audit {@html}`

## Syntax variants

- `{@html expr}`
- `{@html await expr}` with `experimental.async`
- Standalone root fragment usage
- Sole child of an element (`is_controlled` optimization in the reference compiler)
- Non-controlled usage inside `svg` / `mathml` namespace subtrees

## Use cases

- [x] Parse `{@html expr}` and preserve the JS expression span
- [x] Emit standalone HTML insertion in the default HTML namespace
- [x] Emit controlled parent-`innerHTML` update when `{@html}` is the only child of an element
- [x] Emit async `{@html await expr}` through the async wrapper path
- [x] Emit top-level `svg` namespace `{@html}` via `<svelte:options namespace="svg" />`
- [x] Emit top-level `mathml` namespace `{@html}` via `<svelte:options namespace="mathml" />`
- [x] Namespace propagation for non-controlled nested `svg` / `mathml`
- [x] Preserve reference behavior for `svelte-ignore hydration_html_changed` — codegen passes 6th arg `true` to `$.html(...)` when annotated (test: `html_tag_hydration_ignore`)
- [x] Parser tolerates whitespace between `{` and `@` for `{@html}` (parser unit test: `html_tag_with_leading_whitespace`)
- [x] Analyzer emits `block_unexpected_character` (`@`) for `{ @html ...}` in runes mode (diagnostic test: `html_tag_invalid_opening_tag_runes`)

## Reference

- `reference/compiler/phases/1-parse/state/tag.js`
- `reference/compiler/phases/2-analyze/visitors/HtmlTag.js`
- `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/fragment.js`
- `reference/compiler/types/template.d.ts`
- `reference/docs/03-template-syntax/08-@html.md`
- `reference/docs/07-misc/07-v5-migration-guide.md`
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/tests.rs`
- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/passes/content_types.rs`
- `crates/svelte_analyze/src/passes/template_scoping.rs`
- `crates/svelte_codegen_client/src/template/html_tag.rs`
- `crates/svelte_codegen_client/src/template/element.rs`
- `crates/svelte_codegen_client/src/template/html.rs`
- `tasks/compiler_tests/cases2/html_tag/`
- `tasks/compiler_tests/cases2/html_tag_controlled/`
- `tasks/compiler_tests/cases2/html_tag_svg/`
- `tasks/compiler_tests/cases2/async_html_basic/`

## Test cases

- [x] `html_tag`
- [x] `html_tag_controlled`
- [x] `html_tag_svg`
- [x] `async_html_basic`
- [x] `html_tag_mathml`
- [x] `html_tag_nested_svg`
- [x] `html_tag_basic` (parser unit test)
- [x] `html_tag_complex_expression` (parser unit test)
- [x] `html_tag_nested_mathml`
- [x] `html_tag_with_leading_whitespace` (parser unit test)
- [x] `html_tag_invalid_opening_tag_runes` (diagnostic parity test)
- [x] `html_tag_hydration_ignore`
