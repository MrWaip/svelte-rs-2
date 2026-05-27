# Svelte Component

## Current state
- **Working**: 11/11 use cases
- **Tests**: 12/12 green
- Last updated: 2026-05-16

## Source
- ROADMAP: `Legacy Svelte 4` -> `<svelte:component>`
- User request: `/audit <svelte:component>`

## Syntax variants
```svelte
<svelte:component this={Expr} />
<svelte:component this={Expr}></svelte:component>
<svelte:component this={Expr} answer={42} />
<svelte:component this={Expr} {...props} />
<svelte:component this={Expr} bind:this={ref} />
<svelte:component this={Expr} on:done={handler} />
<svelte:component this={Expr}>child content</svelte:component>
<svelte:component />
<svelte:component this="Foo" />
```

## Use cases

- [x] `<svelte:component>` is represented as a dedicated AST node (`SvelteComponentLegacy`) built inline at parse time. Codegen panic path for missing `this` removed; analyze validation owns `this` rejection.
- [x] Missing `this` is rejected with `svelte_component_missing_this` from analyze before codegen. (test: `svelte_component_missing_this`)
- [x] Non-expression `this` is rejected with `svelte_component_invalid_this` from analyze. (test: `svelte_component_invalid_this_string`)
- [x] Legacy-mode `<svelte:component this={expr} .../>` lowers through the dynamic-component runtime path and excludes `this` from serialized props. (test: `svelte_component_basic`)
- [x] Non-self-closing `<svelte:component>` preserves shared child-content lowering after the `this` attribute is stripped. (test: `svelte_component_children`)
- [x] Runes mode emits `svelte_component_deprecated`, while legacy mode does not warn for the same template form. (tests: `svelte_component_deprecated_warns_in_runes_mode`, `svelte_component_deprecated_no_warn_in_legacy_mode`)
- [x] Legacy `<svelte:component>` with multiple expression attributes wraps hoisted `derived_safe_equal` locals in an outer block scope before `$.component(...)`. (test: `svelte_component_legacy_derived_hoist_block`)
- [x] Named-slot child of `<svelte:component>` (`<X slot="name">…</X>`) compiles end-to-end: parser collects pending expressions inside the slot fragment, and analyze accepts `SvelteComponentLegacy`/`SvelteSelf` as a valid owner for the `slot=` attribute (no `slot_attribute_invalid_placement`). (test: `diagnose_svelte_component_named_slot_expression_binds`)
- [x] Block children of `<svelte:component>` (`{#if}`, `{#each}`, `{#await}`, `{#key}`, `{@render}`, `{@const}`, `{@html}`) have their `BlockSemantics` populated so codegen dispatch finds the expected variant instead of panicking with `UnexpectedBlockSemantics`. (tests: `svelte_component_if_child`, `svelte_component_each_child`)
- [x] Named-slot fragments inside `<svelte:component>` get the same legacy reactive expression rewriting as regular component slots: dynamic text expressions in slot children lower to `$.template_effect` + `$.set_text`, and `{#if cond}` conditions wrap store reads in `$.untrack` with the dependency comma-sequence — `ExpressionSemantics` walker traverses `legacy_slots` for `SvelteComponentLegacy` symmetrically with `ComponentNode`/`SvelteSelf`. (tests: `svelte_component_slot_legacy_store_reactivity`, `svelte_component_slot_legacy_if_store_untrack`)

## Out of scope

- Generic shared component prop, binding, event, snippet, attach-tag, CSS-prop, and slot semantics already tracked in `specs/component-node.md`, `specs/legacy-slots.md`, `specs/attach-tag.md`, and `specs/css-pipeline.md`
- SSR-specific `<svelte:component>` behavior

## Reference
### Svelte
- `original/docs/99-legacy/30-legacy-svelte-component.md`
- `original/docs/07-misc/07-v5-migration-guide.md`
- `original/compiler/phases/1-parse/state/element.js`
- `original/compiler/phases/2-analyze/visitors/SvelteComponent.js`
- `original/compiler/phases/2-analyze/visitors/shared/component.js`
- `original/compiler/phases/3-transform/client/visitors/SvelteComponent.js`
- `original/compiler/phases/3-transform/client/visitors/shared/component.js`
- `original/compiler/errors.js`
- `original/compiler/warnings.js`

### Our code
- `crates/svelte_ast/src/lib.rs` — `SvelteComponentLegacy` node + `Node::as_component_like` view
- `crates/svelte_parser/src/lib.rs`, `handlers.rs` — inline `SvelteComponentLegacy` build at `StartTag`/`EndTag`/auto-close
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/attr_convert.rs`
- `crates/svelte_parser/src/svelte_elements.rs`
- `crates/svelte_analyze/src/passes/element_flags.rs` — `process_component_like`
- `crates/svelte_analyze/src/passes/template_validation.rs` — `validate_svelte_component_legacy_this`
- `crates/svelte_analyze/src/walker/traverse.rs` — `walk_component_like`
- `crates/svelte_codegen_client/src/template/component.rs`
- `tasks/compiler_tests/cases2/svelte_component_basic/`
- `tasks/diagnostic_tests/cases/components/svelte_component_deprecated_warns_in_runes_mode/`
- `tasks/diagnostic_tests/cases/components/svelte_component_deprecated_no_warn_in_legacy_mode/`

## Test cases

- [x] `svelte_component_basic`
- [x] `svelte_component_deprecated_warns_in_runes_mode`
- [x] `svelte_component_deprecated_no_warn_in_legacy_mode`
- [x] `svelte_component_children`
- [x] `svelte_component_missing_this`
- [x] `svelte_component_invalid_this_string`
- [x] `svelte_component_legacy_derived_hoist_block`
- [x] `diagnose_svelte_component_named_slot_expression_binds`
- [x] `svelte_component_if_child`
- [x] `svelte_component_each_child`
- [x] `svelte_component_slot_legacy_store_reactivity`
- [x] `svelte_component_slot_legacy_if_store_untrack`
