# Legacy component tags

## Current state
- **Working**: 0/9 legacy-tag use cases
- **Missing**: parser lowering, analysis/validation, and client codegen for `<svelte:component>`, `<svelte:self>`, and `<svelte:fragment>`
- **Next**: port parser node kinds and validation first, then hook the existing component/snippet machinery up to the legacy tags in analyze and client codegen
- Last updated: 2026-04-01

## Source

- ROADMAP item: `<svelte:component>` / `<svelte:self>` / `<svelte:fragment>`
- User request: `/audit component`

## Syntax variants

- `<svelte:component this={Expr} ... />`
- `<svelte:component this={Expr}>...</svelte:component>`
- `<svelte:self ... />`
- `<svelte:self>...</svelte:self>`
- `<svelte:fragment slot="name">...</svelte:fragment>`
- `<svelte:fragment slot="name" let:x>...</svelte:fragment>`
- Invalid `<svelte:component>` forms: missing `this`, non-expression `this`
- Invalid `<svelte:self>` placement: outside `{#if}`, `{#each}`, `{#snippet}`, or component slot content
- Invalid `<svelte:fragment>` forms: wrong parent, extra attributes/directives

## Use cases

- [ ] `<svelte:component this={Expr}>` lowers to dynamic component mounting via `$.component(...)` in legacy mode
- [ ] `<svelte:component>` forwards props, spreads, `bind:this`, events, and children through the same component code path as normal component tags
- [ ] Falsy `<svelte:component this={Expr}>` renders nothing instead of instantiating a DOM element
- [ ] Runes-mode analysis emits `svelte_component_deprecated`
- [ ] `<svelte:self>` lowers to the current component constructor in allowed recursive positions
- [ ] `<svelte:self>` placement validation rejects top-level usage with `svelte_self_invalid_placement`
- [ ] Runes-mode analysis emits `svelte_self_deprecated`
- [ ] `<svelte:fragment slot="name">` contributes named-slot content without adding a wrapper element
- [ ] `<svelte:fragment>` validation rejects non-component placement and non-`slot`/`let:` attributes

### Deferred

- SSR parity for the same tags
- Migration-specific warnings or tooling outside compile/analyze parity

## Reference

- Reference docs:
  - `reference/docs/99-legacy/30-legacy-svelte-component.md`
  - `reference/docs/99-legacy/31-legacy-svelte-self.md`
  - `reference/docs/99-legacy/22-legacy-svelte-fragment.md`
- Reference parser/analyze:
  - `reference/compiler/phases/1-parse/state/element.js`
  - `reference/compiler/phases/2-analyze/visitors/Component.js`
  - `reference/compiler/phases/2-analyze/visitors/SvelteComponent.js`
  - `reference/compiler/phases/2-analyze/visitors/SvelteSelf.js`
  - `reference/compiler/phases/2-analyze/visitors/SvelteFragment.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/component.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/attribute.js`
- Reference client transform:
  - `reference/compiler/phases/3-transform/client/visitors/Component.js`
  - `reference/compiler/phases/3-transform/client/visitors/SvelteComponent.js`
  - `reference/compiler/phases/3-transform/client/visitors/SvelteSelf.js`
  - `reference/compiler/phases/3-transform/client/visitors/SvelteFragment.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- Rust implementation:
  - `crates/svelte_parser/src/lib.rs`
  - `crates/svelte_parser/src/handlers.rs`
  - `crates/svelte_parser/src/svelte_elements.rs`
  - `crates/svelte_ast/src/lib.rs`
  - `crates/svelte_analyze/src/passes/template_scoping.rs`
  - `crates/svelte_analyze/src/passes/element_flags.rs`
  - `crates/svelte_analyze/src/passes/lower.rs`
  - `crates/svelte_codegen_client/src/template/traverse.rs`
  - `crates/svelte_codegen_client/src/template/component.rs`
  - `crates/svelte_diagnostics/src/lib.rs`
  - `tasks/compiler_tests/cases2/component_*`

## Tasks

- [ ] Parser/AST: add dedicated legacy node kinds or equivalent parser lowering for `<svelte:component>`, `<svelte:self>`, and `<svelte:fragment>` instead of treating them as generic elements
- [ ] Parser/analyze: validate `<svelte:component this={...}>` and emit `svelte_component_missing_this` / `svelte_component_invalid_this`
- [ ] Analyze: mark `<svelte:component>` dynamic in legacy mode, emit runes deprecation warnings, and preserve existing component attribute/event/binding analysis behavior
- [ ] Analyze: validate `<svelte:self>` placement and emit runes deprecation warnings
- [ ] Analyze: validate `<svelte:fragment>` placement and attribute restrictions while preserving slot-scope semantics
- [ ] Client codegen: route `<svelte:component>` through the dynamic `$.component(...)` path rather than the static `ComponentName(...)` call
- [ ] Client codegen: route `<svelte:self>` through the current component name path
- [ ] Client codegen: flatten `<svelte:fragment>` into slot content without wrapper output
- [ ] Tests: keep the audit bounded to one snapshot case per tag, then add diagnostic-focused parser/analyzer tests when implementation begins

## Implementation order

1. Parser/AST ownership first, because the current AST cannot represent these tags distinctly.
2. Port analyze-time validation and deprecation warnings so the legacy tags are classified correctly before codegen.
3. Port client codegen for `<svelte:component>` and `<svelte:self>` using the existing component builder as the shared backend.
4. Port `<svelte:fragment>` slot flattening last, because it depends on correct parent-component/slot analysis.

## Discovered bugs

- OPEN: the parser currently recognizes only uppercase or dotted names as `ComponentNode`, so all three legacy tags fall through as generic elements.
- OPEN: diagnostics for `svelte_component_missing_this`, `svelte_component_invalid_this`, `svelte_fragment_invalid_attribute`, `svelte_fragment_invalid_placement`, `svelte_self_invalid_placement`, `svelte_component_deprecated`, and `svelte_self_deprecated` exist but are not emitted by the current Rust pipeline.
- OPEN: client codegen only has the static `ComponentName($$anchor, props)` path, not the reference compiler's `$.component(...)` path for legacy dynamic components.

## Test cases

- Existing related coverage:
  - `component_basic`
  - `component_non_self_closing`
  - `component_children`
  - `component_snippet_only`
  - `component_bind_this`
- Added during this audit:
  - `svelte_component_basic`
  - `svelte_self_if`
  - `svelte_fragment_named_slot`
- Recommended next command:
  - `port specs/legacy-component-tags.md`
