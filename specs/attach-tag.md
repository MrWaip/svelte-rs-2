# `{@attach}`

## Current state
- **Working**: 10/10 use cases
- **Tests**: 11/11 green
- Last updated: 2026-04-08

## Source

- ROADMAP directive item: `{@attach}`
- Audit request: `/audit {@attach}`

## Syntax variants

- `<div {@attach attachment}>`
- `<div {@attach factory(arg)}>`
- `<div {@attach (node) => { ... }}>`
- `<div {@attach condition && attachment}>`
- `<div {@attach first} {@attach second}>`
- `<Component {@attach attachment} />`
- `<Component {@attach factory(arg)} />`
- `<svelte:document {@attach attachment} />`

## Use cases

- [x] Parse `{@attach expr}` as an attribute-position tag and preserve the JS expression span
- [x] Walk/analyze attach expressions like other attribute expressions, including symbol/reference collection
- [x] Reject `await` expressions inside attachments with the shared illegal-await diagnostic
- [x] Emit `$.attach(node, thunk(expr))` for regular elements
- [x] Wrap element attachment setup in `$.run_after_blockers(...)` when async blockers are present
- [x] Preserve multiple attachments on the same element in source order
- [x] Allow inline arrow attachments and conditional/falsy attachment expressions on elements
- [x] Emit component attachment props with `[$.attachment()]` keys for static expressions
- [x] Emit dynamic component attachment wrappers that re-read reactive attachment expressions
- [x] Emit `{@attach}` on `<svelte:document>` using the same runtime attach path as other attachment-bearing targets (test: `attach_on_document`)

## Out of scope

- Runtime semantics of cleanup functions and re-run timing
- `createAttachmentKey` / `fromAction` helper APIs from `svelte/attachments`
- SSR behavior beyond noting the reference compiler's blocker accounting

## Reference

- `reference/docs/03-template-syntax/09-@attach.md`
- `reference/docs/05-special-elements/03-svelte-document.md`
- `reference/compiler/phases/1-parse/state/element.js`
- `reference/compiler/phases/2-analyze/visitors/AttachTag.js`
- `reference/compiler/phases/2-analyze/visitors/shared/component.js`
- `reference/compiler/phases/3-transform/client/visitors/AttachTag.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- `reference/compiler/phases/3-transform/server/visitors/shared/component.js`
- `reference/compiler/types/template.d.ts`
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/scanner/mod.rs`
- `crates/svelte_parser/src/attr_convert.rs`
- `crates/svelte_analyze/src/passes/build_component_semantics.rs`
- `crates/svelte_analyze/src/passes/element_flags.rs`
- `crates/svelte_analyze/src/passes/template_validation.rs`
- `crates/svelte_analyze/src/walker/traverse.rs`
- `crates/svelte_codegen_client/src/template/events/actions.rs`
- `crates/svelte_codegen_client/src/template/attributes.rs`
- `crates/svelte_codegen_client/src/template/element.rs`
- `crates/svelte_codegen_client/src/template/component.rs`
- `crates/svelte_codegen_client/src/template/svelte_document.rs`
- `tasks/compiler_tests/cases2/attach_basic/`
- `tasks/compiler_tests/cases2/attach_inline_arrow/`
- `tasks/compiler_tests/cases2/attach_conditional/`
- `tasks/compiler_tests/cases2/attach_multiple/`
- `tasks/compiler_tests/cases2/attach_with_directives/`
- `tasks/compiler_tests/cases2/attach_in_if/`
- `tasks/compiler_tests/cases2/attach_in_each/`
- `tasks/compiler_tests/cases2/attach_blockers/`
- `tasks/compiler_tests/cases2/attach_on_component/`
- `tasks/compiler_tests/cases2/attach_on_component_dynamic/`
- `tasks/compiler_tests/cases2/attach_on_document/`

## Test cases

- [x] `attach_basic`
- [x] `attach_inline_arrow`
- [x] `attach_conditional`
- [x] `attach_multiple`
- [x] `attach_with_directives`
- [x] `attach_in_if`
- [x] `attach_in_each`
- [x] `attach_blockers`
- [x] `attach_on_component`
- [x] `attach_on_component_dynamic`
- [x] `attach_on_document`
