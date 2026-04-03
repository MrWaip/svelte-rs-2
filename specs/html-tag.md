# `{@html}`

## Current state
- **Working**: 6/9 use cases
- **Missing**: 3 use cases
- **Unknown**: 0 use cases
- **Next**: fix non-controlled namespace propagation for nested `svg`/`mathml`, then decide whether hydration-ignore and diagnostics work belong here or stays under diagnostics/hydration tracks
- Last updated: 2026-04-01

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
- [~] Namespace propagation for non-controlled nested `svg` / `mathml`
  Current code only checks component options in client codegen, so nested namespace switches are not represented on the `$.html(...)` call.
- [ ] Preserve reference behavior for non-repaired hydration mismatches / `svelte-ignore hydration_html_changed`

- [ ] Runes-mode invalid-opening-tag diagnostics

## Reference

### Reference compiler

- `reference/compiler/phases/1-parse/state/tag.js`
- `reference/compiler/phases/2-analyze/visitors/HtmlTag.js`
- `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/fragment.js`
- `reference/compiler/types/template.d.ts`
- `reference/docs/03-template-syntax/08-@html.md`
- `reference/docs/07-misc/07-v5-migration-guide.md`

### Rust implementation

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

## Tasks

- [ ] Analyze: expose the effective runtime namespace for each `HtmlTag` in analysis data, or otherwise make current namespace available to client codegen without rediscovery.
- [ ] Client codegen: switch `gen_html_tag` namespace flags from component-level options to the effective namespace of the tag site.
- [ ] Client codegen: support the extra hydration-ignore argument used by the reference compiler when `hydration_html_changed` is ignored.
- [ ] Tests: keep the existing standalone / controlled / async / root-svg coverage and add nested namespace coverage.
- [ ] Diagnostics follow-up: decide whether `validate_opening_tag` parity belongs in this spec or in the broader diagnostics roadmap item.

## Implementation order

1. Lock coverage with focused compiler cases for top-level mathml and nested SVG.
2. Fix namespace ownership in analyze/codegen for nested namespaces.
3. Add hydration-ignore support if the runtime/test harness can observe it cleanly.
4. Revisit diagnostics ownership after the template diagnostics roadmap advances.

## Discovered bugs

- OPEN: `crates/svelte_codegen_client/src/template/html_tag.rs` derives non-controlled `svg` / `mathml` flags from component options, not from the current template namespace. This should break `{@html}` inside nested namespace transitions such as `<svg>...{@html ...}</svg>` in an otherwise HTML component.

## Test cases

### Existing

- `html_tag`
- `html_tag_controlled`
- `html_tag_svg`
- `async_html_basic`
- parser unit tests: `html_tag_basic`, `html_tag_complex_expression`

### Added during audit

- `html_tag_mathml`
- `html_tag_nested_svg`

### Likely next

- `html_tag_nested_mathml`
- hydration mismatch / ignore coverage once the harness can assert it
