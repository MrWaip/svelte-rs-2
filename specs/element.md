# Element

## Current state
- **Working**: 13/16 use cases
- **Partial**: template validation — `slot_attribute_invalid_placement` added; `node_invalid_placement` and `component_name_lowercase` skipped (require HTML content model table and symbol ref-count access respectively). A11y: 5 checks implemented (`a11y_distracting_elements`, `a11y_accesskey`, `a11y_positive_tabindex`, `a11y_autofocus`, `a11y_missing_attribute` for img/area/iframe/object/a); remaining (ARIA roles, event handler A11y, html[lang], input type=image) deferred.
- **Missing**: 3 — namespace edge cases, legacy slots, CSS-scoped metadata
- **Next**: ARIA role/attribute checks or remaining A11y (html[lang], missing_content, event handler pair checks)
- Last updated: 2026-04-04

## Source

- ROADMAP Template item: `Element`
- Related roadmap items with overlap:
  - `Attributes & Spreads -> Form element special handling`
  - `Validation & Diagnostics -> Element & directive validation`
- Request: `/audit Template -> Element`

## Syntax variants

- Plain HTML elements: `<div />`, `<div></div>`, nested elements, void elements
- Special regular-element parsing branches:
  - `<title>` inside `<svelte:head>`
  - `<slot>` outside shadowroot templates (legacy, tracked separately)
  - lower-case names that could shadow imports
- Namespace-sensitive elements:
  - root namespace via `<svelte:options namespace="svg|mathml">`
  - inline `<svg>` / MathML subtrees in HTML components
  - namespace-sensitive descendants like `<a>` / `<title>`
- Form-element special cases:
  - `<textarea>{expr}</textarea>`
  - `<option>{expr}</option>`
  - customizable `<select>` / `<option>` / `<optgroup>` / `<selectedcontent>`
- Regular-element-only diagnostics:
  - invalid DOM placement
  - `slot="..."` placement
  - non-void self-closing warning
  - textarea value/content conflict

## Use cases

- `[x]` Basic regular elements parse and compile as DOM nodes
  Existing tests: `single_element`, `nested_elements`, `elements_childs`, `mixed_html_elements`
- `[x]` Static and simple dynamic attributes compile on regular elements
  Existing tests: `element_attributes`, `spread_attribute`
- `[x]` Root namespace options and basic inline SVG/MathML paths compile
  Existing tests: `namespace_svg`, `namespace_mathml`, `svg_inner_template_from_svg`, `html_tag_svg`
- `[x]` Non-void self-closing tags lower to explicit open/close HTML
  Existing tests: `non_void_self_closing`, `mixed_html_elements`
- `[x]` `<noscript>` content is stripped from the static template payload
  Existing tests: `smoke`, `smoke_all`
- `[x]` Child fragment lowering respects SVG whitespace rules
  Existing tests: `svg_inner_whitespace_trimming`, `svg_text_preserves_whitespace`
- `[~]` Regular-element directives and advanced attribute paths work, but coverage/spec ownership lives elsewhere
  See: `specs/bind-directives.md`, `specs/css-pipeline.md`, `specs/experimental-async.md`
- `[x]` Template validation for regular elements and element attributes
  Working: `element_invalid_self_closing_tag`, `textarea_invalid_content`, `slot_attribute_invalid_placement`.
  Skipped (out of scope): `node_invalid_placement` (requires HTML content model table), `component_name_lowercase` (requires symbol ref-count access).
- `[x]` `<textarea>` child-content lowering to a synthetic `value` attribute
  Implemented: `needs_textarea_value_lowering` flag in ElementFlags; codegen emits `$.remove_textarea_child` + `$.set_value` with raw expression (no constant folding). Test: `textarea_child_value_dynamic`.
- `[x]` `<option>{expr}</option>` synthetic value handling
  Implemented: `option_synthetic_value_expr` side table in ElementFlags; codegen emits `option.__value = expr` via `get_node_expr` after textContent. Test: `option_expr_child_value`.
- `[x]` Customizable select subtree handling
  Implemented: `is_customizable_select` flag in ElementFlags; `element_needs_var` updated; codegen emits `$.customizable_select(el, callback)` with separate hoisted template. `<selectedcontent>` emits `$.selectedcontent(el, setter)`.
  Tests: `customizable_select_option_el`, `customizable_select_select_div`, `selectedcontent_basic`.
- `[x]` `autofocus` helper path on regular elements
  Implemented: `$.autofocus(el, expr)` emitted from `attributes.rs`. Test: `element_autofocus`.
- `[ ]` Full namespace parity for edge cases like ancestor-derived `<a>` / `<title>` switching
  Current coverage proves common cases only

- `[ ]` Legacy `<slot>` semantics and slot elements
- `[~]` A11y warnings for regular elements
  Implemented: `a11y_distracting_elements` (`<marquee>`/`<blink>`), `a11y_accesskey`, `a11y_positive_tabindex`, `a11y_autofocus` (suppressed inside `<dialog>`), `a11y_missing_attribute` (img[alt], area[alt|aria-label|aria-labelledby], iframe[title], object[title|aria-label|aria-labelledby], a[href] with id/name/aria-disabled exceptions).
  Tests: `a11y_distracting_elements_marquee`, `a11y_distracting_elements_blink`, `a11y_accesskey_warns`, `a11y_positive_tabindex_warns`, `a11y_tabindex_zero_no_warning`, `a11y_tabindex_negative_no_warning`, `a11y_tabindex_dynamic_no_warning`, `a11y_autofocus_warns`, `a11y_autofocus_on_dialog_no_warning`, `a11y_autofocus_inside_dialog_no_warning`, `a11y_missing_attribute_img_no_alt`, `a11y_missing_attribute_img_with_alt_no_warning`, `a11y_missing_attribute_img_spread_no_warning`, `a11y_missing_attribute_area_no_alt`, `a11y_missing_attribute_area_with_aria_label_no_warning`, `a11y_missing_attribute_iframe_no_title`, `a11y_missing_attribute_iframe_with_title_no_warning`, `a11y_missing_attribute_anchor_no_href`, `a11y_missing_attribute_anchor_with_href_no_warning`, `a11y_missing_attribute_anchor_with_id_no_warning`, `a11y_missing_attribute_anchor_with_name_no_warning`, `a11y_missing_attribute_anchor_aria_disabled_no_warning`, `a11y_missing_attribute_anchor_spread_no_warning`.
  Remaining: `html[lang]`, `input[type=image]` alt, `a11y_missing_content`, ARIA role/attribute checks, event handler A11y checks.
  [ ] `html[lang]` — root element, needs separate treatment
  [ ] `input[type=image]` — needs static type attribute value inspection
  [ ] ARIA role checks (`a11y_unknown_role`, `a11y_no_abstract_role`, `a11y_role_has_required_aria_props`, etc.)
  [ ] Event handler A11y (`a11y_click_events_have_key_events`, `a11y_mouse_events_have_key_events`)
- `[ ]` CSS-scoped element metadata and pruning

## Reference

- Reference compiler:
  - `reference/compiler/phases/1-parse/state/element.js`
  - `reference/compiler/phases/2-analyze/visitors/RegularElement.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/attribute.js`
  - `reference/compiler/phases/3-transform/client/visitors/RegularElement.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/element.js`
  - `reference/compiler/errors.js`
  - `reference/compiler/warnings.js`
- Our parser/analyze/codegen:
  - `crates/svelte_parser/src/lib.rs`
  - `crates/svelte_parser/src/scanner/mod.rs`
  - `crates/svelte_analyze/src/passes/element_flags.rs`
  - `crates/svelte_analyze/src/passes/bind_semantics.rs`
  - `crates/svelte_analyze/src/passes/lower.rs`
  - `crates/svelte_analyze/src/validate/mod.rs`
  - `crates/svelte_codegen_client/src/template/mod.rs`
  - `crates/svelte_codegen_client/src/template/element.rs`
  - `crates/svelte_codegen_client/src/template/attributes.rs`
  - `crates/svelte_codegen_client/src/template/html.rs`

## Tasks

1. `[x]` Add `element_invalid_self_closing_tag` warning + `textarea_invalid_content` error in `ElementFlagsVisitor::visit_element`
   Files: `crates/svelte_analyze/src/passes/element_flags.rs`
2. `[x]` Port `<textarea>` dynamic child-content lowering into analyze-side metadata + codegen
   Files: `crates/svelte_analyze/src/types/data/elements.rs`, `element_flags.rs`, `codegen_view.rs`, `context.rs`, `crates/svelte_codegen_client/src/template/element.rs`
3. `[x]` Port `<option>{expr}</option>` synthetic value handling
   Files: same as above
4. `[x]` Port regular-element runtime special case `autofocus`
   Files: `crates/svelte_codegen_client/src/template/attributes.rs`
5. `[ ]` Port remaining template validation: `node_invalid_placement`, `slot_attribute_invalid_placement`, `component_name_lowercase`
   Effort: moderate (needs component ancestor walk for slot, HTML tree validity table for placement)
6. `[ ]` Port customizable select / selectedcontent behavior
   Files: analyze metadata + regular element codegen + maybe lowered fragment handling
   Effort: needs infrastructure
7. `[ ]` Expand namespace edge-case coverage
   Effort: moderate

## Implementation order

1. Add validation plumbing in `svelte_analyze` so missing element diagnostics stop being silently accepted.
2. Add analyze-owned metadata for textarea/option/select special cases rather than re-deriving in codegen.
3. Port regular-element codegen consumers for that metadata.
4. Finish with namespace edge cases and additional focused tests.

## Discovered bugs

- OPEN: `crates/svelte_analyze/src/validate/mod.rs` currently validates only rune usage; regular template validation is absent.
- OPEN: regular-element analyze/codegen path has no equivalent for reference `textarea` child lowering, synthetic `option` value metadata, or customizable select handling.
- OPEN: regular-element codegen has no dedicated `autofocus` helper path.

## Test cases

- Existing compiler coverage:
  `single_element`, `nested_elements`, `elements_childs`, `element_attributes`, `spread_attribute`, `namespace_svg`, `namespace_mathml`, `svg_inner_template_from_svg`, `html_tag_svg`, `non_void_self_closing`, `mixed_html_elements`, `smoke`, `smoke_all`
- Added during this audit:
  - `textarea_child_value_dynamic`
  - `option_expr_child_value`
