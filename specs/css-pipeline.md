# CSS

## Current state
- **Working**: parser extracts a single top-level `<style>` block into `Component.css`, preserves its raw source span, parses `css="injected"` from `<svelte:options>`, and reports duplicate top-level `<style>` tags
- **Partial**: nested `<style>` elements likely work as ordinary DOM elements, but there is no focused compiler coverage proving parity with the reference "insert as-is, unscoped" behavior
- **Missing**: there is no Rust CSS subsystem matching Svelte's parse/analyze/prune/transform pipeline; no scoped class injection, no selector analysis, no keyframe scoping, no unused-selector warnings, no CSS output, and `CompileOptions.css` is currently unused by `compile()`
- **Next**: establish the first end-to-end scoped CSS path before deeper selector parity, starting with stylesheet parsing/representation, deterministic hash generation, scoped element marking, and CSS output plumbing
- Last updated: 2026-04-01

## Source

User request: `$audit css`

ROADMAP.md — CSS

## Syntax variants

- Single top-level `<style>...</style>` block with component-scoped CSS
- Scoped selectors on regular elements
- `@keyframes` inside component CSS, including `-global-*` escape hatch
- `:global(...)` and `:global { ... }`
- Nested selectors using `&`
- Nested `<style>` elements inside regular markup or blocks, emitted unprocessed into the DOM
- CSS custom properties passed to elements and components via `--prop="..."`
- Compiler CSS modes: `css: "external"` and `css: "injected"`

## Use cases

- `[x]` Parse and retain one top-level `<style>` block as raw source
  Existing tests: parser `style_tag`, `style_tag_with_selectors`, `style_tag_with_script`
- `[x]` Diagnose duplicate top-level `<style>` blocks
  Existing test: parser `duplicate_style_tag_returns_diagnostic`
- `[x]` Parse `<svelte:options css="injected">`
  Existing parser test already covers `options.css`
- `[ ]` Scoped CSS pipeline for top-level `<style>`
  Missing: CSS AST, selector analysis, pruning, scoped class injection, keyframe scoping, stylesheet rendering
- `[ ]` Compile result CSS plumbing
  Missing: `CompileResult` has no CSS field, and `CompileOptions.css` is not consumed by `compile()`
- `[ ]` `css: "external"` output
  Missing: extracted stylesheet result and filename/hash handling
- `[ ]` `css: "injected"` output
  Missing: runtime style injection path and generated JS hooks
- `[ ]` `:global(...)` and `:global { ... }` validation and transform
  Missing: parser/analyzer/transform parity for global selector forms and their diagnostics
- `[ ]` Scoped `@keyframes` plus `-global-*` escape
  Missing: keyframe collection, renaming, and declaration rewriting
- `[ ]` Unused selector warning
  Missing: `css_unused_selector`
- `[ ]` CSS custom properties on components
  Missing: `<svelte-css-wrapper>` / `<g>` wrapper lowering for `--prop=...`
- `[~]` Nested `<style>` elements inside markup
  Likely compile as plain elements today because top-level style handling is separate, but there is no focused compiler case for the required "unscoped, inserted as-is" behavior

### Deferred

- SSR CSS behavior is out of scope for this spec
- `style:` directives, `class:` directives, and class object/array syntax belong to the `Attributes & Spreads` roadmap area, not this CSS pipeline spec

## Reference

- `reference/docs/04-styling/01-scoped-styles.md`
- `reference/docs/04-styling/02-global-styles.md`
- `reference/docs/04-styling/03-custom-properties.md`
- `reference/docs/04-styling/04-nested-style-elements.md`
- `reference/compiler/phases/1-parse/read/style.js` — reference CSS parser
- `reference/compiler/phases/2-analyze/css/css-analyze.js` — selector/global/keyframe analysis
- `reference/compiler/phases/2-analyze/css/css-prune.js` — template-aware selector pruning
- `reference/compiler/phases/2-analyze/css/css-warn.js` — unused selector warnings
- `reference/compiler/phases/3-transform/css/index.js` — stylesheet rendering/scoping/minification
- `reference/compiler/phases/3-transform/client/transform-client.js` — injected CSS path
- `reference/compiler/phases/3-transform/client/visitors/shared/element.js` — scoped class injection on elements
- `reference/compiler/phases/3-transform/client/visitors/shared/component.js` — custom-property wrapper lowering
- `crates/svelte_parser/src/lib.rs` — top-level style extraction into `RawBlock`
- `crates/svelte_parser/src/tests.rs` — current style parser coverage
- `crates/svelte_parser/src/svelte_elements.rs` — `<svelte:options css="injected">`
- `crates/svelte_compiler/src/options.rs` — `CssMode`
- `crates/svelte_compiler/src/lib.rs` — current compile output shape, currently JS-only

## Tasks

1. `[ ]` Introduce a dedicated CSS subsystem and choose the CSS AST/parser strategy
   This is the roadmap's first blocker before any selector-level parity work
2. `[ ]` Add compile-time CSS result plumbing
   Extend `CompileResult` and thread `CompileOptions.css` through compile/analyze/transform
3. `[ ]` Implement the minimal scoped CSS happy path
   Top-level stylesheet parse, deterministic hash, scoped element marking, stylesheet emission, JS injection/external output
4. `[ ]` Port CSS analysis/validation
   `:global(...)`, `:global {}`, nesting, keyframes, invalid selector diagnostics
5. `[ ]` Port selector pruning and `css_unused_selector`
6. `[ ]` Implement CSS custom property lowering for components and SVG wrappers
7. `[ ]` Add focused compiler cases for nested `<style>` elements and custom properties once infrastructure exists

## Implementation order

1. CSS AST / subsystem choice
2. CompileResult + CSS mode plumbing
3. Minimal scoped-style end-to-end path
4. Global selectors and keyframes
5. Pruning and warnings
6. Custom properties and nested-style parity backfill

## Discovered bugs

- OPEN: `CompileOptions.css` exists, but `compile()` currently returns only JS and diagnostics, so CSS mode is effectively a dead option
- OPEN: there is no Rust equivalent of reference `phases/2-analyze/css/*` or `phases/3-transform/css/index.js`
- OPEN: no focused compiler coverage exists for top-level scoped CSS, nested `<style>` elements, or CSS custom properties

## Test cases

- Existing parser coverage:
  `style_tag`, `style_tag_with_selectors`, `style_tag_with_script`, `duplicate_style_tag_returns_diagnostic`
- Existing compiler coverage related to styling syntax but not the CSS subsystem:
  `style_attr_dynamic`, `style_attr_object`, `style_directive`, `style_directive_concat`, `style_directive_important`, `style_directive_string`, `svelte_element_style_directive`
- Added in this audit:
  `css_scoped_basic` compiler case to pin missing top-level scoped CSS behavior
