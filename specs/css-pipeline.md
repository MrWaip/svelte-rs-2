# CSS

## Current state
- **Working**: scoped CSS pipeline complete — hash, selector scoping, element marking, class injection, `CompileResult.css`. Both `css:"external"` (default) and `css:"injected"` modes work. Tests: `css_scoped_basic`, `css_injected`, `css_injected_via_compile_options`.
- **Architecture**: `svelte_transform_css` crate owns CSS AST → CSS string transform (scoping, serialization, injection compaction). `svelte_analyze::analyze_css_pass` is read-only classifier (hash, scoped elements, inject flag). `svelte_compiler` orchestrates and owns mode-specific post-processing.
- **Working**: `:global(.foo)` functional form — AST-level stripping of pseudo-class wrapper, mixed selectors (`p :global(.bar)`) scope outer LocalName correctly. Test: `css_global_basic`.
- **Working**: `:global { ... }` block form — lone `:global` blocks hoisted at transform time (inner rules promoted unscoped to parent level). Works at top level, inside `@media`/`@supports`, and nested inside style rules. Analyze pass skips type selector collection for global blocks. Test: `css_global_block`.
- **Partial**: nested `<style>` elements likely compile as plain DOM elements, but no focused compiler case proves "unscoped, inserted as-is" parity.
- **Missing**: `:global .foo { ... }` compound form (non-lone), `:global()` inside `:not()`/`:is()`/`:where()`, `:global()` validation diagnostics, `@keyframes` scoping, unused-selector warnings, CSS custom properties.
- **Done**: Scoped `@keyframes` + `-global-` escape — keyframe names prefixed with hash, `-global-` prefix stripped, `animation`/`animation-name` values rewritten.
- **Next**: Port `:global .foo { ... }` compound form or `:global()` inside `:not()`/`:is()`/`:where()`.
- **Known debt**: `has_global_component` is duplicated between `svelte_analyze` and `svelte_transform_css` — to be resolved when `:global()` work makes the function non-trivial.
- Last updated: 2026-04-06

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

- [x] Parse and retain one top-level `<style>` block as raw source (tests: `style_tag`, `style_tag_with_selectors`, `style_tag_with_script`)
- [x] Diagnose duplicate top-level `<style>` blocks (test: `duplicate_style_tag_returns_diagnostic`)
- [x] Parse `<svelte:options css="injected">`
- [x] Scoped CSS pipeline for top-level `<style>` — hash, selector scoping, element marking, class injection, CSS output (test: `css_scoped_basic`)
- [x] Compile result CSS plumbing — `CompileResult.css` field, `analyze_css_pass()` integrated into `compile()`
- [ ] `css: "external"` output — mode flag not explicitly enforced; external is current default behavior with no special handling
- [x] `css: "injected"` output — `const $$css = { hash, code }` hoisted module-level const + `$.append_styles($$anchor, $$css)` as first statement in component body (tests: `css_injected`, `css_injected_via_compile_options`)
- [x] `:global(.foo)` functional form — strip wrapper, scope outer LocalName (test: `css_global_basic`)
- [x] `:global { ... }` block form transform (test: `css_global_block`)
- [ ] `:global()` inside `:not()`, `:is()`, `:where()` — currently unvisited (visitor declares SELECTORS only, not PSEUDO_CLASSES; nested selectors inside functional pseudo-classes silently pass through)
- [ ] `:global()` validation diagnostics
- [x] Scoped `@keyframes` plus `-global-*` escape (test: `css_keyframes_scoped`)
- [ ] CSS comments preserved in output — lightningcss drops comments during AST parsing; reference compiler preserves them via MagicString text manipulation
- [ ] Unused selector warning (`css_unused_selector`)
- [ ] CSS custom properties on components — `<svelte-css-wrapper>` / `<g>` wrapper lowering for `--prop=...`
- [~] Nested `<style>` elements inside markup — likely compile as plain elements today, no focused compiler case for "unscoped, inserted as-is" parity

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

## Test cases

- [x] `style_tag` (parser)
- [x] `style_tag_with_selectors` (parser)
- [x] `style_tag_with_script` (parser)
- [x] `duplicate_style_tag_returns_diagnostic` (parser)
- [x] `style_attr_dynamic`
- [x] `style_attr_object`
- [x] `style_directive`
- [x] `style_directive_concat`
- [x] `style_directive_important`
- [x] `style_directive_string`
- [x] `svelte_element_style_directive`
- [x] `css_scoped_basic`
- [x] `css_injected`
- [x] `css_injected_via_compile_options`
- [x] `css_global_block`
- [x] `css_keyframes_scoped`
