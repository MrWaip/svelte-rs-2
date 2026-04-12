# `<svelte:head>` / `<title>`

## Current state
- **Working**: 5/10 use cases
- **Tests**: 12/15 green
- Last updated: 2026-04-07

## Source

- ROADMAP Special Elements: `<svelte:head>` / `<title>`
- Audit request: `$audit <svelte:head> / <title>`

## Syntax variants

- `<svelte:head></svelte:head>`
- `<svelte:head><meta ... /></svelte:head>`
- `<svelte:head><link ... /></svelte:head>`
- `<svelte:head><title>static text</title></svelte:head>`
- `<svelte:head><title>{expr}</title></svelte:head>`
- `<svelte:head><title>text {expr} text</title></svelte:head>`
- `<svelte:head><title>&amp; {expr} &lt;</title></svelte:head>`
- `<svelte:head ...>...</svelte:head>`
- `<svelte:head>` only at component top level
- one `<svelte:head>` per component
- `<title ...>...</title>` inside `<svelte:head>`
- `<title>{await expr}</title>` inside `<svelte:head>`

## Use cases

- [x] Parse a top-level `<svelte:head>` fragment and preserve its child fragment as a dedicated `SvelteHead` node.
- [x] Treat `<title>` inside `<svelte:head>` as a special title element for lowering/codegen while leaving other head children as normal elements.
- [x] Generate `$.head(hash(filename), ($$anchor) => { ... })` for `<svelte:head>` content.
- [x] Lower `<title>` content into `$.document.title = ...` with static, reactive, mixed-text, entity-decoded, and async expression variants.
- [x] Allow other regular head children like `<meta>` and `<link>` alongside `<title>`.
- [ ] Reject duplicate `<svelte:head>` tags with `svelte_meta_duplicate`.
- [ ] Reject `<svelte:head>` outside the component top level with `svelte_meta_invalid_placement`.
- [ ] Reject attributes or directives on `<svelte:head>` with `svelte_head_illegal_attribute`.
- [ ] Reject attributes or directives on `<title>` inside `<svelte:head>` with `title_illegal_attribute`.
- [ ] Reject non-text / non-expression children inside `<title>` with `title_invalid_content`.

## Out of scope

- SSR `head` string emission and server-transform parity
- Browser runtime verification beyond generated client output parity
- `<title>` outside `<svelte:head>` semantics beyond existing regular-element parsing

## Reference

- Reference docs: `reference/docs/05-special-elements/05-svelte-head.md`
- Reference parser: `reference/compiler/phases/1-parse/state/element.js`
- Reference analyze: `reference/compiler/phases/2-analyze/visitors/SvelteHead.js`
- Reference analyze: `reference/compiler/phases/2-analyze/visitors/TitleElement.js`
- Reference client transform: `reference/compiler/phases/3-transform/client/visitors/SvelteHead.js`
- Reference client transform: `reference/compiler/phases/3-transform/client/visitors/TitleElement.js`
- Reference diagnostics: `reference/compiler/errors.js`
- Rust parser conversion: `crates/svelte_parser/src/svelte_elements.rs`
- Rust parser entry: `crates/svelte_parser/src/lib.rs`
- Rust analyze lowering: `crates/svelte_analyze/src/passes/lower.rs`
- Rust analyze fragment traversal: `crates/svelte_analyze/src/walker/traverse.rs`
- Rust client codegen: `crates/svelte_codegen_client/src/template/svelte_head.rs`
- Rust client codegen: `crates/svelte_codegen_client/src/template/title_element.rs`
- Existing compiler cases: `tasks/compiler_tests/cases2/svelte_head_basic`, `tasks/compiler_tests/cases2/svelte_head_reactive`, `tasks/compiler_tests/cases2/svelte_head_with_content`, `tasks/compiler_tests/cases2/title_variants`, `tasks/compiler_tests/cases2/async_title_basic`, `tasks/compiler_tests/cases2/svelte_head_title_meta`, `tasks/compiler_tests/cases2/title_entity_decoding`, `tasks/compiler_tests/cases2/head_with_special_elements`, `tasks/compiler_tests/cases2/head_with_snippets`, `tasks/compiler_tests/cases2/head_position_with_body`

## Test cases

- [x] `svelte_head_basic`
- [x] `svelte_head_reactive`
- [x] `svelte_head_with_content`
- [x] `title_variants`
- [x] `async_title_basic`
- [x] `svelte_head_title_meta`
- [x] `title_entity_decoding`
- [x] `head_with_special_elements`
- [x] `head_with_snippets`
- [x] `head_position_with_body`
- [x] Parser coverage for duplicate `<svelte:head>` and invalid `<svelte:head>` placement
- [x] Analyzer coverage for illegal `<svelte:head>` attributes, illegal `<title>` attributes, and invalid `<title>` content
- [ ] `svelte_head_illegal_attribute`
- [ ] `title_illegal_attribute`
- [ ] `title_invalid_content`
