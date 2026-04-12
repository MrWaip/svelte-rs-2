# <svelte:self>

## Current state
- **Working**: 5/8 closure items
- **Next**: implement `<svelte:self slot="name" />` as named-slot lowering instead of default `children`, then add analyzer placement validation plus a valid-placement diagnostic parity case for the runes deprecation hint
- Last updated: 2026-04-12

## Source
- ROADMAP.md Legacy Svelte 4: `<svelte:self>`
- User request: `/audit <svelte:self>`

## Syntax variants
```svelte
<svelte:self />
<svelte:self></svelte:self>
{#if condition}<svelte:self {...props} />{/if}
{#each items as item}<svelte:self value={item} />{/each}
{#snippet recurse()}<svelte:self />{/snippet}
<Outer><svelte:self slot="footer" /></Outer>
<svelte:self bind:this={ref} />
<svelte:self on:done={handler} />
<svelte:self let:item />
```

## Use cases
- [x] Parser accepts `<svelte:self>` as a component-like special tag and codegen emits a recursive self-call in `{#if}` blocks (test: `svelte_self_if`)
- [x] Recursive self-call works inside `{#each}` blocks with prop forwarding (test: `svelte_self_each`)
- [x] Recursive self-call works inside `{#snippet}` blocks (test: `svelte_self_snippet`)
- [x] Component-style props lower on `<svelte:self>` the same way as ordinary components (test: `svelte_self_props`)
- [x] `bind:this` lowers on `<svelte:self>` the same way as ordinary components (test: `svelte_self_bind_this`)
- [ ] `<svelte:self slot="name" />` passed to another component should lower into `$$slots.<name>` instead of the default `children` slot path; current Rust output keeps `slot="footer"` on the recursive call but still builds `children`/`$$slots.default` on the parent (test: `svelte_self_slot`, `#[ignore]`, moderate)
- [ ] Top-level `<svelte:self>` should emit `svelte_self_invalid_placement` in both legacy and runes modes; analyzer currently emits no placement diagnostic in legacy mode and a warning-only deprecation diagnostic in runes mode (tests: `svelte_self_deprecated_warns_with_default_self_import_hint`, `svelte_self_deprecated_warns_with_configured_self_import_hint`, `svelte_self_deprecated_uses_deconflicted_component_name`, `svelte_self_deprecated_uses_reserved_word_deconflicted_component_name`, `svelte_self_deprecated_no_warn_in_legacy_mode`, needs infrastructure)
- [ ] In runes mode, valid-placement `<svelte:self>` should still emit the deprecation warning with the correct self-import hint, including deconflicted component names and basename selection; current diagnostic coverage only exercises invalid top-level placement, so the deprecation path is still unproven in a valid context (test: none yet, needs infrastructure)

## Out of scope
- SSR behavior

## Reference
### Svelte
- `reference/docs/99-legacy/31-legacy-svelte-self.md`
- `reference/compiler/phases/1-parse/state/element.js`
- `reference/compiler/phases/2-analyze/visitors/SvelteSelf.js`
- `reference/compiler/phases/2-analyze/visitors/shared/component.js`
- `reference/compiler/phases/3-transform/client/visitors/SvelteSelf.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- `reference/compiler/errors.js`
- `reference/compiler/warnings.js`

### Our code
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_analyze/src/passes/element_flags.rs`
- `crates/svelte_analyze/src/passes/template_validation.rs`
- `crates/svelte_analyze/src/types/data/elements.rs`
- `crates/svelte_codegen_client/src/template/mod.rs`
- `crates/svelte_codegen_client/src/template/component.rs`
- `crates/svelte_diagnostics/src/lib.rs`
- `tasks/compiler_tests/test_v3.rs`
- `tasks/compiler_tests/cases2/svelte_self_*`
- `tasks/diagnostic_tests/test_diagnostics.rs`
- `tasks/diagnostic_tests/cases/components/svelte_self_*`

## Test cases
- [x] `svelte_self_if`
- [x] `svelte_self_each`
- [x] `svelte_self_snippet`
- [x] `svelte_self_props`
- [x] `svelte_self_bind_this`
- [ ] `svelte_self_slot`
- [ ] `svelte_self_deprecated_warns_with_default_self_import_hint`
- [ ] `svelte_self_deprecated_warns_with_configured_self_import_hint`
- [ ] `svelte_self_deprecated_uses_deconflicted_component_name`
- [ ] `svelte_self_deprecated_uses_reserved_word_deconflicted_component_name`
- [ ] `svelte_self_deprecated_no_warn_in_legacy_mode`
