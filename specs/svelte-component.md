# Svelte Component

## Current state
- **Working**: 3/5 closure items
- **Next**: port parser-owned `this` validation (`svelte_component_missing_this`, `svelte_component_invalid_this`) and remove the ignored diagnostic parity cases
- **Done this session**: added passing `svelte_component_children` snapshot coverage and durable diagnostic parity repros for missing/invalid `this`
- **Verification**: `just generate`; `just test-case svelte_component_children`; `just test-diagnostic-case svelte_component_missing_this`; `just test-diagnostic-case svelte_component_invalid_this_string`
- **Known gap**: both failing diagnostic cases currently panic in `crates/svelte_codegen_client/src/template/component.rs` with `"<svelte:component> missing \`this\` attribute"` and produce `internal_error` instead of the reference parser diagnostics
- Last updated: 2026-04-12

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

- [x] Legacy-mode `<svelte:component this={expr} .../>` lowers through the dynamic-component runtime path and excludes `this` from serialized props. (test: `svelte_component_basic`)
- [x] Runes mode emits `svelte_component_deprecated`, while legacy mode does not warn for the same template form. (tests: `svelte_component_deprecated_warns_in_runes_mode`, `svelte_component_deprecated_no_warn_in_legacy_mode`)
- [ ] Missing `this` is rejected with `svelte_component_missing_this` before codegen fallback/panic paths. (test: `svelte_component_missing_this`, #[ignore], moderate)
- [ ] Non-expression `this` is rejected with `svelte_component_invalid_this` instead of being treated as a normal prop and reaching the codegen panic path. (test: `svelte_component_invalid_this_string`, #[ignore], moderate)
- [x] Non-self-closing `<svelte:component>` preserves shared child-content lowering after the `this` attribute is stripped. (test: `svelte_component_children`)

## Out of scope

- Generic shared component prop, binding, event, snippet, attach-tag, CSS-prop, and slot semantics already tracked in `specs/component-node.md`, `specs/legacy-slots.md`, `specs/attach-tag.md`, and `specs/css-pipeline.md`
- SSR-specific `<svelte:component>` behavior

## Reference
### Svelte
- `reference/docs/99-legacy/30-legacy-svelte-component.md`
- `reference/docs/07-misc/07-v5-migration-guide.md`
- `reference/compiler/phases/1-parse/state/element.js`
- `reference/compiler/phases/2-analyze/visitors/SvelteComponent.js`
- `reference/compiler/phases/2-analyze/visitors/shared/component.js`
- `reference/compiler/phases/3-transform/client/visitors/SvelteComponent.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- `reference/compiler/errors.js`
- `reference/compiler/warnings.js`

### Our code
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/attr_convert.rs`
- `crates/svelte_parser/src/svelte_elements.rs`
- `crates/svelte_analyze/src/passes/element_flags.rs`
- `crates/svelte_analyze/src/passes/template_validation.rs`
- `crates/svelte_codegen_client/src/template/component.rs`
- `tasks/compiler_tests/cases2/svelte_component_basic/`
- `tasks/diagnostic_tests/cases/components/svelte_component_deprecated_warns_in_runes_mode/`
- `tasks/diagnostic_tests/cases/components/svelte_component_deprecated_no_warn_in_legacy_mode/`

## Test cases

- [x] `svelte_component_basic`
- [x] `svelte_component_deprecated_warns_in_runes_mode`
- [x] `svelte_component_deprecated_no_warn_in_legacy_mode`
- [x] `svelte_component_children`
- [ ] `svelte_component_missing_this`
- [ ] `svelte_component_invalid_this_string`
