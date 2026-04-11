# CSS

## Current state
- **Working**: scoped CSS pipeline complete — hash, selector scoping, element marking, class injection, `CompileResult.css`. Both `css:"external"` (default) and `css:"injected"` modes work. Tests: `css_scoped_basic`, `css_injected`, `css_injected_via_compile_options`.
- **Architecture**: `svelte_transform_css` crate owns CSS AST → CSS string transform (scoping, serialization, injection compaction). `svelte_analyze::analyze_css_pass` is read-only classifier (hash, scoped elements, inject flag) and CSS validator (`:global` diagnostics). `svelte_compiler` orchestrates and owns mode-specific post-processing.
- **Working**: `:global(.foo)` functional form — AST-level stripping of pseudo-class wrapper, mixed selectors (`p :global(.bar)`) scope outer LocalName correctly. Test: `css_global_basic`.
- **Done this session**: bare `:global` compound-form parity. Selectors like `.a :global .b .c`, `section :global strong`, and leading `:global .page .title` now match the reference compiler across both CSS scoping analysis and CSS transform output. `css_prune` truncates matching at bare `:global` so only the local prefix receives the scope class, while `svelte_transform_css` leaves the selector tail unscoped and removes middle empty selectors without disturbing the leading-empty `:global ...` form. Coverage: analyzer unit `bare_global_tail_only_scopes_local_prefix`, transform unit tests `bare_global_*`, compiler case `css_global_compound`.
- **Done this session**: unused selector CSS pruning in emitted output. `svelte_transform_css` now consumes `AnalysisData.output.css.used_selectors`, drops unused selectors/rules from both external and injected CSS output, and `tasks/generate_test_cases` strips reference-only `/* (unused)` comment wrappers from committed `case-svelte.css` snapshots so compiler tests compare the intended CSS payload instead of Svelte's comment-only bookkeeping. Coverage: transform unit tests `external_mode_drops_unused_rules_and_selectors` / `injected_mode_removes_unused_rules_and_selectors_before_compaction`, compiler cases `css_unused_external` / `css_unused_injected`.
- **Working**: `:global { ... }` block form — lone `:global` blocks hoisted at transform time (inner rules promoted unscoped to parent level). Works at top level, inside `@media`/`@supports`, and nested inside style rules. Analyze pass skips type selector collection for global blocks. Test: `css_global_block`.
- **Done**: `:global()` validation diagnostics — all 12 CSS validation error diagnostics ported from reference `css-analyze.js`. `CssValidator` visitor in `svelte_analyze::passes::css_analyze` tracks parent rule context via stack. 20 unit tests covering all diagnostic kinds plus valid cases.
- **Done**: Scoped `@keyframes` + `-global-` escape — keyframe names prefixed with hash, `-global-` prefix stripped, `animation`/`animation-name` values rewritten.
- **Done**: `:global()` inside `:not()`/`:is()`/`:where()`/`:has()` — visitor recurses into pseudo-class args, unwraps `:global()` and scopes non-global selectors. Also fixed scope class insertion position to go before trailing pseudo-classes (matching reference compiler). Test: `css_global_in_pseudo`.
- **Done**: CSS prune pass — basic backward selector matching (type/class/ID selectors, descendant/child combinators). Emits `css_unused_selector` warnings for selectors that don't match any template element. New `css_prune` module in `svelte_analyze::passes`. 24 unit tests.
- **Partial**: nested `<style>` elements likely compile as plain DOM elements, but no focused compiler case proves "unscoped, inserted as-is" parity.
- **Remaining gaps**: nested `<style>` parity, CSS comments preserved in output, broader selector-matching expansion for `css_unused_selector`, and explicit `css: "external"` option handling.
- **Done**: Component custom-property wrapper lowering — `--*` attrs on a component are pre-classified into a `component_css_props` side-table during analyze, and codegen routes the component through a wrapper element (`<svelte-css-wrapper style="display: contents">` for HTML, `<g>` for SVG) plus `$.css_props(node, () => ({...}))`, with the inner component anchored on `node.lastChild`. Tests: `css_custom_prop_component`, `css_custom_prop_component_svg`. Side change: `CompileOptions.namespace` is now merged into `component.options.namespace` as a fallback inside `compile()` (matches reference behavior); the parser-side `regex_illegal_attribute_character` check skips component tags so `--name` is accepted.
- **Spec drift (2026-04-11)**: the older “Open bugs” note is stale. `css_scoped_class_selector`, `css_scope_class_in_snippet`, `css_scope_svelte_element_class`, `css_scope_class_object`, and `css_scope_spread_attribute` all pass in `just test-case ...`, so the next slice should come from the remaining gaps below rather than that note.
- **Next**: nested `<style>` parity. We likely already leave nested `<style>` tags as plain DOM elements, but the feature still lacks one focused compiler case proving that they stay unscoped and are inserted as-is.
- **Non-goals for the next run**: general CSS comment preservation, broader selector-matching expansion beyond the existing prune logic, and explicit `css: "external"` option cleanup.
- **Known debt**: `has_global_component` is duplicated between `svelte_analyze` and `svelte_transform_css` — to be resolved when `:global()` work makes the function non-trivial. The earlier blocker noted above (executor.rs / render_tags.rs / template_validation.rs compile errors) is no longer present — removed from tracking.
- **Historical note**: the 2026-04-08 diagnose cluster for snippet / `<svelte:element>` / class-object / spread-only scope-class injection is no longer active; those regression cases pass in `just test-compiler`. Keep the spec drift note above until the broader class/id/attribute selector item is re-audited cleanly.
- Last updated: 2026-04-11

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
- [x] Scope class injection on an element inside a `{#snippet}` body — the scope class is appended to the static `class="..."` literal and to the template HTML root (test: `css_scope_class_in_snippet`)
- [x] Scope class injection on `<svelte:element this={...}>` with a static `class="..."` — scope class is appended both at the template HTML root and in the dynamic class argument passed to `$.element` (test: `css_scope_svelte_element_class`)
- [x] Scope class argument for class-object attributes — when `class={{ active, big }}` is compiled via `$.set_class`, the third argument is the scope-hash string so the runtime can merge it with the object classes (test: `css_scope_class_object`)
- [x] Scope class pass-through for spread attributes — `$.attribute_effect` emits the full scoped form so spread-only elements retain the scope-hash string (test: `css_scope_spread_attribute`)
- [x] Compile result CSS plumbing — `CompileResult.css` field, `analyze_css_pass()` integrated into `compile()`
- [ ] `css: "external"` output — mode flag not explicitly enforced; external is current default behavior with no special handling
- [x] `css: "injected"` output — `const $$css = { hash, code }` hoisted module-level const + `$.append_styles($$anchor, $$css)` as first statement in component body (tests: `css_injected`, `css_injected_via_compile_options`)
- [x] Unused selectors/rules are omitted from emitted CSS in both external and injected modes; reference-only `/* (unused)` wrappers are stripped from committed CSS snapshots instead of being treated as observable parity (tests: `css_unused_external`, `css_unused_injected`)
- [x] `:global(.foo)` functional form — strip wrapper, scope outer LocalName (test: `css_global_basic`)
- [x] Bare `:global` compound form keeps the local prefix scoped while leaving the selector tail unscoped, including leading `:global .foo` selectors that remain fully global (test: `css_global_compound`)
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
- [x] `css_global_basic`
- [x] `css_global_block`
- [x] `css_global_compound`
- [x] `css_keyframes_scoped`
- [x] `css_global_in_pseudo`
- [x] `style_directive` extended with `style:--columns`
- [x] `css_custom_prop_component`
- [x] `css_custom_prop_component_svg`
- [x] `css_unused_external`
- [x] `css_unused_injected`
- [x] `css_scope_class_in_snippet`
- [x] `css_scope_svelte_element_class`
- [x] `css_scope_class_object`
- [x] `css_scope_spread_attribute`
