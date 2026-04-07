# Element

## Current state
- **Working**: 15/17 use cases
- **Partial**: template validation — `slot_attribute_invalid_placement` added; `node_invalid_placement` and `component_name_lowercase` skipped (require HTML content model table and symbol ref-count access respectively). A11y ownership lives in `specs/a11y-warnings.md`.
- **Missing**: 2 — namespace edge cases, legacy slots
- **Current slice**: `Literal Concat Folding` completed. Shared concat codegen now folds literal dynamic parts into adjacent static text and emits a plain string literal when no runtime expressions remain, covering regular element attrs and component prop concatenations through the shared `build_attr_concat` path. Changes were systematic, without workarounds or temporary solutions, respecting crate and module boundaries.
- **Completed non-goals**: this slice intentionally did not include identifier-based constant propagation, namespace parity, legacy slots, A11y expansion, or parser/analyzer redesign.
- **Next**: Implement full namespace parity for ancestor-derived `<a>` / `<title>` edge cases.
- Last updated: 2026-04-07

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

- `[x]` Basic regular elements parse and compile as DOM nodes (tests: `single_element`, `nested_elements`, `elements_childs`, `mixed_html_elements`)
- `[x]` Static and simple dynamic attributes compile on regular elements (tests: `element_attributes`, `spread_attribute`)
- `[x]` `ConcatenationAttribute` for `class` (e.g. `class="static {expr}"`) compiles via `$.set_class`, not `$.set_attribute` — `class_attr_id` in `element_flags.rs` now registers the concat attr, and codegen routes it through the shared class pipeline (test: `class_concat`)
- `[x]` Constant folding for all-literal `ConcatenationAttribute` parts (e.g. `class="1231 {1231}"` → `"1231 1231"`, not a template literal) — shared codegen now folds literal dynamic parts for regular-element attrs and component prop concatenations (tests: `class_concat_literal_fold`, `attribute_concat_literal_fold`, `component_prop_concat_literal_fold`)
- `[x]` Root namespace options and basic inline SVG/MathML paths compile (tests: `namespace_svg`, `namespace_mathml`, `svg_inner_template_from_svg`, `html_tag_svg`)
- `[x]` Non-void self-closing tags lower to explicit open/close HTML (tests: `non_void_self_closing`, `mixed_html_elements`)
- `[x]` `<noscript>` content is stripped from the static template payload (tests: `smoke`, `smoke_all`)
- `[x]` Child fragment lowering respects SVG whitespace rules (tests: `svg_inner_whitespace_trimming`, `svg_text_preserves_whitespace`)
- `[ ]` Regular-element directives and advanced attribute paths mostly work, but coverage/spec ownership still lives elsewhere — see `specs/bind-directives.md`, `specs/css-pipeline.md`, `specs/experimental-async.md`
- `[x]` Template validation for regular elements and element attributes — working: `element_invalid_self_closing_tag`, `textarea_invalid_content`, `slot_attribute_invalid_placement`; skipped (out of scope): `node_invalid_placement` (requires HTML content model table), `component_name_lowercase` (requires symbol ref-count access)
- `[x]` `<textarea>` child-content lowering to a synthetic `value` attribute — `needs_textarea_value_lowering` flag in ElementFlags; codegen emits `$.remove_textarea_child` + `$.set_value` with raw expression (test: `textarea_child_value_dynamic`)
- `[x]` `<option>{expr}</option>` synthetic value handling — `option_synthetic_value_expr` side table in ElementFlags; codegen emits `option.__value = expr` via `get_node_expr` after textContent (test: `option_expr_child_value`)
- `[x]` Customizable select subtree handling — `is_customizable_select` flag in ElementFlags; codegen emits `$.customizable_select(el, callback)` with separate hoisted template; `<selectedcontent>` emits `$.selectedcontent(el, setter)` (tests: `customizable_select_option_el`, `customizable_select_select_div`, `selectedcontent_basic`)
- `[x]` `autofocus` helper path on regular elements — `$.autofocus(el, expr)` emitted from `attributes.rs` (test: `element_autofocus`)
- `[ ]` Full namespace parity for edge cases like ancestor-derived `<a>` / `<title>` switching — current coverage proves common cases only

## Out of scope
- Legacy `<slot>` semantics and slot elements
- CSS-scoped element metadata and pruning live in `specs/css-pipeline.md`

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

## Test cases

- `[x]` `single_element`
- `[x]` `nested_elements`
- `[x]` `elements_childs`
- `[x]` `element_attributes`
- `[x]` `spread_attribute`
- `[x]` `namespace_svg`
- `[x]` `namespace_mathml`
- `[x]` `svg_inner_template_from_svg`
- `[x]` `html_tag_svg`
- `[x]` `non_void_self_closing`
- `[x]` `mixed_html_elements`
- `[x]` `smoke`
- `[x]` `smoke_all`
- `[x]` `textarea_child_value_dynamic`
- `[x]` `option_expr_child_value`
- `[x]` `customizable_select_option_el`
- `[x]` `customizable_select_select_div`
- `[x]` `selectedcontent_basic`
- `[x]` `element_autofocus`
- `[x]` `class_concat`
- `[x]` `class_concat_literal_fold`
- `[x]` `attribute_concat_literal_fold`
- `[x]` `component_prop_concat_literal_fold`
- `[x]` `svg_inner_whitespace_trimming`
- `[x]` `svg_text_preserves_whitespace`
