# CSS

## Current state
- **Working**: scoped CSS pipeline complete — hash, selector scoping, element marking, class injection, `CompileResult.css`. Both `css:"external"` (default) and `css:"injected"` modes work. Tests: `css_scoped_basic`, `css_injected`, `css_injected_via_compile_options`.
- **Architecture**: `svelte_transform_css` crate owns CSS AST → CSS string transform (scoping, serialization, injection compaction). `svelte_analyze::analyze_css_pass` is read-only classifier (hash, scoped elements, inject flag) and CSS validator (`:global` diagnostics). `svelte_compiler` orchestrates and owns mode-specific post-processing.
- **Working**: `:global(.foo)` functional form — AST-level stripping of pseudo-class wrapper, mixed selectors (`p :global(.bar)`) scope outer LocalName correctly. Test: `css_global_basic`.
- **Working**: `:global { ... }` block form — lone `:global` blocks hoisted at transform time (inner rules promoted unscoped to parent level). Works at top level, inside `@media`/`@supports`, and nested inside style rules. Analyze pass skips type selector collection for global blocks. Test: `css_global_block`.
- **Done**: `:global()` validation diagnostics — all 12 CSS validation error diagnostics ported from reference `css-analyze.js`. `CssValidator` visitor in `svelte_analyze::passes::css_analyze` tracks parent rule context via stack. 20 unit tests covering all diagnostic kinds plus valid cases.
- **Done**: Scoped `@keyframes` + `-global-` escape — keyframe names prefixed with hash, `-global-` prefix stripped, `animation`/`animation-name` values rewritten.
- **Done**: `:global()` inside `:not()`/`:is()`/`:where()`/`:has()` — visitor recurses into pseudo-class args, unwraps `:global()` and scopes non-global selectors. Also fixed scope class insertion position to go before trailing pseudo-classes (matching reference compiler). Test: `css_global_in_pseudo`.
- **Done**: CSS prune pass — basic backward selector matching (type/class/ID selectors, descendant/child combinators). Emits `css_unused_selector` warnings for selectors that don't match any template element. New `css_prune` module in `svelte_analyze::passes`. 24 unit tests.
- **Partial**: nested `<style>` elements likely compile as plain DOM elements, but no focused compiler case proves "unscoped, inserted as-is" parity.
- **Missing**: `:global .foo { ... }` compound form (non-lone), unused selector CSS output wrapping.
- **Done**: Component custom-property wrapper lowering — `--*` attrs on a component are pre-classified into a `component_css_props` side-table during analyze, and codegen routes the component through a wrapper element (`<svelte-css-wrapper style="display: contents">` for HTML, `<g>` for SVG) plus `$.css_props(node, () => ({...}))`, with the inner component anchored on `node.lastChild`. Tests: `css_custom_prop_component`, `css_custom_prop_component_svg`. Side change: `CompileOptions.namespace` is now merged into `component.options.namespace` as a fallback inside `compile()` (matches reference behavior); the parser-side `regex_illegal_attribute_character` check skips component tags so `--name` is accepted.
- **Next**: remaining CSS pipeline gaps — `:global .foo` compound form, unused selector CSS wrapping, nested `<style>` element parity. None scoped to current slice.
- **Known debt**: `has_global_component` is duplicated between `svelte_analyze` and `svelte_transform_css` — to be resolved when `:global()` work makes the function non-trivial. The earlier blocker noted above (executor.rs / render_tags.rs / template_validation.rs compile errors) is no longer present — removed from tracking.
- **Open bugs (2026-04-08 diagnose)**: scope-class injection misses 4 variants — element inside `{#snippet}` body, `<svelte:element>` with static class, class-object attribute (third arg to `$.set_class` is `null`), and spread-only element (`$.attribute_effect` emitted with 2 args instead of 6). Isolated by `css_scope_class_in_snippet`, `css_scope_svelte_element_class`, `css_scope_class_object`, `css_scope_spread_attribute` (all `#[ignore]`). Likely unified in the codegen attribute-lowering path (`crates/svelte_codegen_client/src/template/attributes.rs`) + analyze `scoped_elements` propagation.
- Last updated: 2026-04-08

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
- Element custom properties via `style:--prop={expr}` or `style:--prop="text"`
- Component custom properties via `--prop="..."`
- Component custom properties via `--prop={expr}`
- Component custom properties in SVG namespace via `<g style="..."><Component /></g>`
- Compiler CSS modes: `css: "external"` and `css: "injected"`

## Use cases

- [x] Parse and retain one top-level `<style>` block as raw source (tests: `style_tag`, `style_tag_with_selectors`, `style_tag_with_script`)
- [x] Diagnose duplicate top-level `<style>` blocks (test: `duplicate_style_tag_returns_diagnostic`)
- [x] Parse `<svelte:options css="injected">`
- [x] Scoped CSS pipeline for top-level `<style>` — hash, selector scoping, element marking, class injection, CSS output (test: `css_scoped_basic`)
- [ ] Element marking via class / id / attribute selectors — `mark_scoped_elements` (`crates/svelte_analyze/src/passes/css_analyze.rs`) only walks `SimpleSelector::Type` via `TypeSelectorCollector`, so an element matched only by `.foo`, `#bar`, or `[attr]` never gets the scope-hash class injected. The full matching logic in `css_prune.rs::PruneVisitor` already knows which elements match each selector but never propagates that to `CssAnalysis::scoped_elements` (test: `css_scoped_class_selector`, `#[ignore]`, M)
- [ ] Scope class injection on an element inside a `{#snippet}` body — the scope class should be appended to the static `class="..."` literal and to the template HTML root, but neither happens. Same root cause family as class-selector element marking above, but also reproducible when the element already carries a type-selector match via its tag name if it sits inside snippet scope (test: `css_scope_class_in_snippet`, `#[ignore]`, S)
- [ ] Scope class injection on `<svelte:element this={...}>` with a static `class="..."` — scope class is not appended either at the template HTML root or in the dynamic class argument passed to `$.element` (test: `css_scope_svelte_element_class`, `#[ignore]`, S)
- [ ] Scope class argument for class-object attributes — when `class={{ active, big }}` is compiled via `$.set_class`, the third argument must be the scope-hash string (`'svelte-xxxxxx'`) instead of `null` so the runtime can merge it with the object classes (test: `css_scope_class_object`, `#[ignore]`, S)
- [ ] Scope class pass-through for spread attributes — `$.attribute_effect` is emitted with only 2 arguments for `{...rest}` elements, but the reference emits the full 6-argument form including the trailing scope-hash string, so scope styling is lost on spread-only elements (test: `css_scope_spread_attribute`, `#[ignore]`, S)
- [x] Compile result CSS plumbing — `CompileResult.css` field, `analyze_css_pass()` integrated into `compile()`
- [ ] `css: "external"` output — mode flag not explicitly enforced; external is current default behavior with no special handling
- [x] `css: "injected"` output — `const $$css = { hash, code }` hoisted module-level const + `$.append_styles($$anchor, $$css)` as first statement in component body (tests: `css_injected`, `css_injected_via_compile_options`)
- [x] `:global(.foo)` functional form — strip wrapper, scope outer LocalName (test: `css_global_basic`)
- [x] `:global { ... }` block form transform (test: `css_global_block`)
- [x] `:global()` inside `:not()`, `:is()`, `:where()`, `:has()` — visitor recurses into pseudo-class args (test: `css_global_in_pseudo`)
- [x] `:global()` validation diagnostics (20 unit tests in `css_analyze::tests`)
- [x] Scoped `@keyframes` plus `-global-*` escape (test: `css_keyframes_scoped`)
- [ ] CSS comments preserved in output — lightningcss drops comments during AST parsing; reference compiler preserves them via MagicString text manipulation
- [ ] Unused selector warning (`css_unused_selector`) — basic type/class/ID matching with descendant/child combinators works; missing: sibling combinators, `:is/:where/:not/:has` special matching, nesting selector, attribute value matching, component/snippet boundary matching
- [x] Element custom properties via `style:--prop` reuse the generic style-directive path and emit through `$.set_style(...)`
- [x] Component custom properties in HTML namespace wrap the component in `<svelte-css-wrapper style="display: contents">` and apply values through `$.css_props(...)` instead of passing `--*` as component props (test: `css_custom_prop_component`)
- [x] Component custom properties in SVG namespace wrap the component in `<g>` and apply values through `$.css_props(...)` (test: `css_custom_prop_component_svg`)
- [ ] Nested `<style>` elements inside markup — likely compile as plain elements today, but there is still no focused compiler case proving "unscoped, inserted as-is" parity

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
- `reference/docs/07-misc/01-best-practices.md` — preferred `style:--prop` and parent-to-child custom-property patterns
- `crates/svelte_analyze/src/passes/css_prune.rs` — CSS selector pruning: backward matching against template elements
- `crates/svelte_analyze/src/passes/element_flags.rs` — component attributes currently classified as plain `ComponentPropKind::*`
- `crates/svelte_parser/src/lib.rs` — top-level style extraction into `RawBlock`
- `crates/svelte_parser/src/tests.rs` — current style parser coverage
- `crates/svelte_parser/src/svelte_elements.rs` — `<svelte:options css="injected">`
- `crates/svelte_transform_css/src/lib.rs` — CSS AST transform: scoping, serialization, injection compaction
- `crates/svelte_compiler/src/options.rs` — `CssMode`
- `crates/svelte_compiler/src/lib.rs` — compile() orchestration, CSS mode dispatch
- `crates/svelte_codegen_client/src/template/attributes.rs` — generic `style:` directive lowering for elements
- `crates/svelte_codegen_client/src/template/component.rs` — missing custom-property wrapper lowering for components

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
- [x] `css_global_in_pseudo`
- [x] `style_directive` extended with `style:--columns`
- [x] `css_custom_prop_component`
- [x] `css_custom_prop_component_svg`
- [ ] `css_scope_class_in_snippet` (`#[ignore]`)
- [ ] `css_scope_svelte_element_class` (`#[ignore]`)
- [ ] `css_scope_class_object` (`#[ignore]`)
- [ ] `css_scope_spread_attribute` (`#[ignore]`)
