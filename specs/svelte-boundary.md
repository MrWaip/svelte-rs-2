# `<svelte:boundary>`

## Current state
- **Working**: 19/19 use cases
- **Tests**: 25/25 green
- Last updated: 2026-05-01

## Source

- `ROADMAP.md` `<svelte:boundary>`
- Audit request: `$audit <svelte:boundary>`

## Syntax variants

`<svelte:boundary>...</svelte:boundary>`
`<svelte:boundary onerror={handler}>...</svelte:boundary>`
`<svelte:boundary {onerror}>...</svelte:boundary>`
`<svelte:boundary failed={failed}>...</svelte:boundary>`
`<svelte:boundary {failed}>...</svelte:boundary>`
`<svelte:boundary pending={pending}>...</svelte:boundary>`
`<svelte:boundary {pending}>...</svelte:boundary>`
`<svelte:boundary>...{#snippet failed(error, reset)}...{/snippet}...</svelte:boundary>`
`<svelte:boundary>...{#snippet pending()}...{/snippet}...</svelte:boundary>`
`<svelte:boundary>...{#snippet helper()}...{/snippet}...</svelte:boundary>`
`<svelte:boundary>...{@const x = expr}...{/svelte:boundary}`
`<svelte:boundary ...>` nested inside control-flow blocks or another `<svelte:boundary>`

## Use cases

- [x] Parse `<svelte:boundary>` anywhere in template content and lower it into a dedicated AST node
- [x] Generate a plain client boundary with no boundary props
- [x] Generate `onerror` from expression attributes, including reactive handlers and imported handlers
- [x] Generate `failed` from an inline `failed` snippet
- [x] Generate `failed` from an explicit `failed={expr}` attribute
- [x] Generate `pending` from an inline `pending` snippet
- [x] Generate `pending` from an explicit `pending={expr}` attribute; covered by `boundary_pending_attribute` and `boundary_pending_imported`
- [x] Combine `onerror`, `failed`, and `pending` in one boundary
- [x] Keep non-special snippets inside the boundary body while hoisting snippet declarations
- [x] Allow `{@const}` as a direct child of `<svelte:boundary>`
- [x] Duplicate boundary `{@const}` declarations into hoisted snippets that reference them
- [x] Preserve boundary behavior when nested inside `if` blocks or inside another boundary
- [x] Prefer inline `failed` snippet over `failed={expr}` when both are present; covered by `boundary_failed_attribute_override`
- [x] Prefer inline `pending` snippet over `pending={expr}` when both are present; covered by `boundary_pending_attribute_override`
- [x] Reject invalid boundary attributes and directives with `svelte_boundary_invalid_attribute` diagnostics in analyze
- [x] Reject bare, string, or multi-chunk attribute values with `svelte_boundary_invalid_attribute_value` diagnostics in analyze
- [x] Treat boundary children as their own fragment scope in semantics/analyze
- [x] Permit boundary-local snippets such as `failed`/`pending` to coexist with other snippet declarations
- [x] Support async-mode boundary const-tag duplication for snippet references

## Out of scope

- SSR `transformError` behavior and server renderer boundary semantics
- Hydration-time error transformation details
- Runtime behavior of caught errors and pending promises in `svelte/internal`

## Reference

- Reference docs: `reference/docs/05-special-elements/01-svelte-boundary.md`
- Reference parser tag registration: `reference/compiler/phases/1-parse/state/element.js`
- Reference analyze validation: `reference/compiler/phases/2-analyze/visitors/SvelteBoundary.js`
- Reference identifier handling for async boundary snippets: `reference/compiler/phases/2-analyze/visitors/Identifier.js`
- Reference client transform: `reference/compiler/phases/3-transform/client/visitors/SvelteBoundary.js`
- Reference server transform: `reference/compiler/phases/3-transform/server/visitors/SvelteBoundary.js`
- Local parser conversion: `crates/svelte_parser/src/svelte_elements.rs`
- Local semantics scope entry: `crates/svelte_analyze/src/passes/build_component_semantics.rs`
- Local template scope traversal: `crates/svelte_analyze/src/walker/traverse.rs`
- Local boundary side tables: `crates/svelte_analyze/src/passes/template_side_tables.rs`
- Local boundary codegen: `crates/svelte_codegen_client/src/template/svelte_boundary.rs`
- Local diagnostics enum/messages: `crates/svelte_diagnostics/src/lib.rs`
- Local compiler tests: `tasks/compiler_tests/cases2/boundary_*`

## Test cases

- [x] `boundary_basic`
- [x] `boundary_failed_snippet`
- [x] `boundary_onerror`
- [x] `boundary_pending_snippet`
- [x] `boundary_failed_onerror`
- [x] `boundary_failed_attribute`
- [x] `boundary_all_three`
- [x] `boundary_reactive_onerror`
- [x] `boundary_nested`
- [x] `boundary_const_tag`
- [x] `boundary_in_if`
- [x] `boundary_other_snippets`
- [x] `boundary_const_in_snippet`
- [x] `boundary_imported_handler`
- [x] `async_boundary_const`
- [x] `boundary_pending_attribute`
- [x] `boundary_pending_imported`
- [x] `boundary_failed_attribute_override`
- [x] `boundary_pending_attribute_override`
- [x] `svelte_boundary_invalid_attribute_directive`
- [x] `svelte_boundary_invalid_attribute_unknown`
- [x] `svelte_boundary_invalid_attribute_spread`
- [x] `svelte_boundary_invalid_attribute_value_boolean`
- [x] `svelte_boundary_invalid_attribute_value_string`
- [x] `svelte_boundary_invalid_attribute_value_concat`
