# CSS

## Current state
- **Working**: scoped CSS pipeline complete — hash, selector scoping, element marking, class injection, `CompileResult.css`. Both `css:"external"` (default) and `css:"injected"` modes work. Tests: `css_scoped_basic`, `css_injected`, `css_injected_via_compile_options`.
- **Architecture**: `svelte_transform_css` crate owns CSS AST → CSS string transform (scoping, serialization, injection compaction). `svelte_analyze::analyze_css_pass` is read-only classifier (hash, scoped elements, inject flag). `svelte_compiler` orchestrates and owns mode-specific post-processing.
- **Partial**: nested `<style>` elements likely compile as plain DOM elements, but no focused compiler case proves "unscoped, inserted as-is" parity.
- **Missing**: `:global()` validation and transform, `@keyframes` scoping, unused-selector warnings, CSS custom properties.
- **Next slice**: `:global()` selector validation and transform (Task 4)
- **Known debt**: `has_global_component` is duplicated between `svelte_analyze` and `svelte_transform_css` — to be resolved when Task 4 makes the function non-trivial.
- Last updated: 2026-04-05

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
- `[x]` Scoped CSS pipeline for top-level `<style>`
  Implemented: hash, selector scoping, element marking, class injection, CSS output. Test: `css_scoped_basic`
- `[x]` Compile result CSS plumbing
  Implemented: `CompileResult.css` field, `analyze_css_pass()` integrated into `compile()`
- `[ ]` `css: "external"` output
  Missing: explicit mode enforcement (currently works by default, mode flag not checked)
- `[x]` `css: "injected"` output
  Implemented: `const $$css = { hash, code }` hoisted module-level const + `$.append_styles($$anchor, $$css)` as first statement in component body. Test: `css_injected`
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
- `crates/svelte_transform_css/src/lib.rs` — CSS AST transform: scoping, serialization, injection compaction
- `crates/svelte_compiler/src/options.rs` — `CssMode`
- `crates/svelte_compiler/src/lib.rs` — compile() orchestration, CSS mode dispatch

## Tasks

1. `[x]` Introduce a dedicated CSS subsystem and choose the CSS AST/parser strategy
   Uses lightningcss for parsing/scoping/rendering
2. `[x]` Add compile-time CSS result plumbing
   `CompileResult.css`, `analyze_css_pass()`, `inject_styles` threading
3. `[x]` Implement the minimal scoped CSS happy path
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

- FIXED: `CompileOptions.css` is now consumed by `compile()` — Injected mode activates `$.append_styles()` path
- FIXED: there is now a Rust CSS analysis pass and codegen path for scoped + injected CSS
- OPEN: no focused compiler coverage exists for nested `<style>` elements or CSS custom properties
- OPEN: `css: "external"` mode flag is not explicitly enforced — external is the default behavior, mode check exists but external has no special handling vs no-css

## Test cases

- Existing parser coverage:
  `style_tag`, `style_tag_with_selectors`, `style_tag_with_script`, `duplicate_style_tag_returns_diagnostic`
- Existing compiler coverage related to styling syntax but not the CSS subsystem:
  `style_attr_dynamic`, `style_attr_object`, `style_directive`, `style_directive_concat`, `style_directive_important`, `style_directive_string`, `svelte_element_style_directive`
- Added in this audit:
  `css_scoped_basic` compiler case to pin missing top-level scoped CSS behavior
- Added in css:injected slice:
  `css_injected` — `<svelte:options css="injected">` path
  `css_injected_via_compile_options` — `CompileOptions.css == Injected` path (unit test in `svelte_compiler/src/tests.rs`)
