# CSS

## Current state
- **Working**: CSS pipeline parity is complete for the scope owned by this spec: top-level `<style>` extraction, scoped CSS transform, selector scoping, class injection, emitted CSS, unused-selector pruning, keyframes, `:global(...)`, global blocks, nested rules, pseudo selectors, snippet/component boundary traversal, CSS custom properties, and the remaining invalid-CSS diagnostic cases tracked here.
- **Synced**: spec test inventory now reflects the registered `css_prune` diagnostic parity cases in `tasks/diagnostic_tests/test_diagnostics.rs`, including `:is(...)`, `:where(...)`, implicit nesting, `:root:has(...)`, and escaped-selector matching repros.
- **Next**: complete
- **Done this session**: the remaining ignored CSS diagnostic cases were stale. The full `mod css` diagnostic block already matched `svelte/compiler`, so this slice removed the stale `ignore` markers and closed the last open item in this spec without changing analyze/transform behavior.
- Last updated: 2026-04-12

## Source
Project CSS pipeline parity work and follow-up diagnostic audit.

## Use cases

- [x] Top-level component CSS is extracted from `<style>`, analyzed, transformed, and returned through `CompileResult.css` in external mode (tests: `css_scoped_basic`, `explicit_external_css_mode_returns_compile_result_css`)
- [x] Injected CSS mode works through compile options and inline `<svelte:options css="injected">` precedence (tests: `css_injected`, `css_injected_via_compile_options`, `inline_css_injected_overrides_external_compile_option`)
- [x] Scoped selector marking and scope-class injection work for ordinary elements, snippets, `<svelte:element>`, class-object attrs, and spread attrs (tests: `css_scoped_class_selector`, `css_scope_class_in_snippet`, `css_scope_svelte_element_class`, `css_scope_class_object`, `css_scope_spread_attribute`)
- [x] Selector matching covers type, class, id, attribute presence, static attribute matcher/value selectors, and bounded dynamic attribute expansion with reference-conservative behavior where required (tests: `css_scoped_id_selector`, `css_scoped_attr_presence`, `css_scoped_attr_value_selector`, `css_scoped_attr_matcher_operators`, `css_scoped_attr_name_casefolding`, `css_dynamic_attr_selector_match`, `concat_attribute_selector_no_match`)
- [x] `:global(...)`, bare `:global`, `:global { ... }`, and `:global(...)` inside `:is(...)`, `:where(...)`, `:not(...)`, and `:has(...)` match the reference transform/analyze behavior for valid CSS (tests: `css_global_basic`, `css_global_compound`, `css_global_block`, `css_global_in_pseudo`)
- [x] Scoped keyframes and `-global-` escapes are rewritten correctly (test: `css_keyframes_scoped`)
- [x] Unused selector warnings and emitted-CSS pruning work for descendant/child/sibling combinators, pseudo selectors, nesting selectors, escaped class/id selectors, and snippet/component boundaries (tests: `css_unused_external`, `css_unused_injected`, `css_pseudo_compound_unused_but_scoped`, `css_pseudo_has`, `css_nesting_selector_scoped`, `css_root_has_scoped`, `css_escaped_selector_scoped`, `css_snippet_descendant_scope_boundary`, `css_snippet_sibling_boundary`, `css_component_snippet_descendant_boundary`)
- [x] Diagnostic parity for CSS pruning covers `:is(...)`, `:where(...)`, implicit nesting, `:root:has(...)`, escaped selectors, attribute concatenation, and conservative no-match cases where the reference compiler reports `css_unused_selector` (tests: `is_selector_match`, `is_selector_no_match`, `is_selector_compound_no_match`, `where_selector_match`, `where_selector_complex_branch_conservative`, `implicit_nesting_match`, `root_has_match`, `escaped_selector_match`, `concat_attribute_selector_no_match`, `type_selector_no_match`, `class_selector_no_match`, `id_selector_no_match`, `descendant_combinator_no_match`, `child_combinator_indirect_no_match`, `adjacent_sibling_combinator_no_match`, `general_sibling_combinator_no_match`, `multiple_selectors_mixed`, `media_query_unused_selector`, `no_elements_all_unused`)
- [x] CSS serializer specificity matches the reference for nested selectors and pseudo selector lists, including implicit nesting and `:root:has(...)` handling (tests: `css_pseudo_has`, `css_nesting_selector_scoped`, `css_root_has_scoped`)
- [x] CSS comments are preserved in emitted output (test: `css_comments_preserved`)
- [x] Nested `<style>` elements inside markup or blocks remain ordinary DOM `<style>` elements; only the root `<style>` is extracted into component CSS (test: `css_nested_style`)
- [x] Component custom properties lower through the wrapper-based `$.css_props(...)` path in HTML and SVG namespaces (tests: `css_custom_prop_component`, `css_custom_prop_component_svg`)
- [x] Invalid CSS diagnostics for the tracked `:global(...)`/global-block placement and nesting-placement cases now match `svelte/compiler`; the previous ignored diagnostic cases were stale and have been unignored (tests: `css_global_block_invalid_placement`, `css_global_invalid_placement`, `css_global_invalid_placement_multiple_non_global_after`, `css_global_invalid_selector_list`, `css_type_selector_invalid_placement`, `css_global_invalid_selector`, `css_global_block_invalid_modifier_start`, `css_global_block_invalid_combinator`, `css_global_block_invalid_list`, `css_global_block_invalid_modifier`, `css_nesting_selector_invalid_placement`, `css_selector_invalid`, `css_global_block_descendant_ok`, `css_global_nesting_modifier_start_in_global_block`, `css_global_block_invalid_list_mixed`, `css_nesting_in_compound_global_block_ok`)

## Out of scope

- CSS source maps; tracked in `specs/source-maps.md`
- Custom-element default CSS injection behavior outside the main component CSS pipeline
- SSR-specific CSS behavior

## Reference
### Svelte
- `reference/compiler/phases/1-parse/read/style.js`
- `reference/compiler/phases/2-analyze/css/css-analyze.js`
- `reference/compiler/phases/2-analyze/css/css-prune.js`
- `reference/compiler/phases/2-analyze/css/css-warn.js`
- `reference/compiler/phases/3-transform/css/index.js`
- `reference/compiler/phases/3-transform/client/transform-client.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/element.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/component.js`

### Our code
- `crates/svelte_parser/src/lib.rs`
- `crates/svelte_parser/src/svelte_elements.rs`
- `crates/svelte_analyze/src/passes/css_analyze.rs`
- `crates/svelte_analyze/src/passes/css_prune.rs`
- `crates/svelte_analyze/src/passes/css_prune_index.rs`
- `crates/svelte_transform_css/src/lib.rs`
- `crates/svelte_compiler/src/lib.rs`
- `crates/svelte_compiler/src/options.rs`
- `crates/svelte_codegen_client/src/template/attributes.rs`
- `crates/svelte_codegen_client/src/template/component.rs`
- `tasks/diagnostic_tests/test_diagnostics.rs` — authoritative CSS diagnostic parity inventory for `mod css` and `mod css_prune`

## Test cases

- [x] `css_scoped_basic`
- [x] `explicit_external_css_mode_returns_compile_result_css`
- [x] `css_injected`
- [x] `css_injected_via_compile_options`
- [x] `inline_css_injected_overrides_external_compile_option`
- [x] `css_scoped_class_selector`
- [x] `css_scope_class_in_snippet`
- [x] `css_scope_svelte_element_class`
- [x] `css_scope_class_object`
- [x] `css_scope_spread_attribute`
- [x] `css_scoped_id_selector`
- [x] `css_scoped_attr_presence`
- [x] `css_scoped_attr_value_selector`
- [x] `css_scoped_attr_matcher_operators`
- [x] `css_scoped_attr_name_casefolding`
- [x] `css_dynamic_attr_selector_match`
- [x] `concat_attribute_selector_no_match`
- [x] `css_global_basic`
- [x] `css_global_compound`
- [x] `css_global_block`
- [x] `css_global_in_pseudo`
- [x] `css_keyframes_scoped`
- [x] `css_unused_external`
- [x] `css_unused_injected`
- [x] `css_pseudo_compound_unused_but_scoped`
- [x] `css_pseudo_has`
- [x] `css_nesting_selector_scoped`
- [x] `css_root_has_scoped`
- [x] `css_escaped_selector_scoped`
- [x] `css_snippet_descendant_scope_boundary`
- [x] `css_snippet_sibling_boundary`
- [x] `css_component_snippet_descendant_boundary`
- [x] `css_comments_preserved`
- [x] `css_nested_style`
- [x] `css_custom_prop_component`
- [x] `css_custom_prop_component_svg`
- [x] `css_global_block_invalid_placement`
- [x] `css_global_invalid_placement`
- [x] `css_global_invalid_placement_multiple_non_global_after`
- [x] `css_global_invalid_placement_end_ok`
- [x] `css_global_invalid_placement_start_ok`
- [x] `css_global_invalid_selector_list`
- [x] `css_type_selector_invalid_placement`
- [x] `css_global_invalid_selector`
- [x] `css_global_block_invalid_modifier_start`
- [x] `css_global_block_invalid_combinator`
- [x] `css_global_block_invalid_list`
- [x] `css_global_block_invalid_declaration`
- [x] `css_global_block_invalid_modifier`
- [x] `css_nesting_selector_invalid_placement`
- [x] `css_nesting_selector_valid_in_global`
- [x] `css_selector_invalid`
- [x] `css_global_block_with_nested_rules_ok`
- [x] `css_global_block_descendant_ok`
- [x] `css_global_nesting_modifier_start_in_global_block`
- [x] `css_global_block_invalid_list_mixed`
- [x] `css_nesting_in_compound_global_block_ok`
- [x] `valid_scoped_css_no_diagnostics`
- [x] `type_selector_matches_element`
- [x] `type_selector_case_insensitive_css`
- [x] `universal_selector_always_matches`
- [x] `class_selector_static_match`
- [x] `class_directive_match`
- [x] `id_selector_match`
- [x] `global_functional_always_used`
- [x] `global_block_always_used`
- [x] `descendant_combinator_match`
- [x] `descendant_combinator_deep_match`
- [x] `child_combinator_direct_match`
- [x] `adjacent_sibling_combinator_match`
- [x] `general_sibling_combinator_match`
- [x] `keyframes_not_warned`
- [x] `media_query_used_selector`
- [x] `spread_attribute_conservative_class_match`
- [x] `nested_element_match`
- [x] `deeply_nested_descendant`
- [x] `type_selector_no_match`
- [x] `class_selector_no_match`
- [x] `id_selector_no_match`
- [x] `descendant_combinator_no_match`
- [x] `child_combinator_indirect_no_match`
- [x] `adjacent_sibling_combinator_no_match`
- [x] `general_sibling_combinator_no_match`
- [x] `multiple_selectors_mixed`
- [x] `media_query_unused_selector`
- [x] `no_elements_all_unused`
- [x] `is_selector_match`
- [x] `is_selector_no_match`
- [x] `is_selector_compound_no_match`
- [x] `where_selector_match`
- [x] `where_selector_complex_branch_conservative`
- [x] `implicit_nesting_match`
- [x] `root_has_match`
- [x] `escaped_selector_match`
