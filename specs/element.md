# Element

## Current state
- **Working**: 7/13 use cases are covered or already tracked by passing compiler cases
- **Missing**: template validation for regular elements, textarea/option/select special handling, and several regular-element runtime special cases
- **Next**: implement regular-element template validation first, then port form-element special handling in analyze/codegen order
- Last updated: 2026-04-01

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
- `[ ]` Template validation for regular elements and element attributes
  Missing today: `node_invalid_placement`, `slot_attribute_invalid_placement`, `textarea_invalid_content`, `component_name_lowercase`, `element_invalid_self_closing_tag`
- `[ ]` `<textarea>` child-content lowering to a synthetic `value` attribute
  Missing behavior: reference compiler rewrites dynamic child content and clears fragment children
- `[ ]` `<option>{expr}</option>` synthetic value handling
  Missing behavior: reference compiler preserves non-string values via synthetic value metadata / codegen
- `[ ]` Customizable select subtree handling
  Missing behavior: `select` / `option` / `optgroup` rich-content paths and `<selectedcontent>` handling
- `[ ]` `autofocus` helper path on regular elements
  Missing behavior: reference uses `$.autofocus(...)` instead of a generic attribute setter
- `[ ]` Full namespace parity for edge cases like ancestor-derived `<a>` / `<title>` switching
  Current coverage proves common cases only

### Deferred

- `[ ]` Legacy `<slot>` semantics and slot elements
  Tracked in `specs/legacy-component-tags.md`
- `[ ]` A11y warnings for regular elements
  Tracked under diagnostics roadmap work, not this spec
- `[ ]` CSS-scoped element metadata and pruning
  Tracked in `specs/css-pipeline.md`

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

1. `[ ]` Add template-validation pass ownership in `svelte_analyze`
   Files: new validate module(s) plus `crates/svelte_analyze/src/lib.rs`
   Scope: regular-element placement, slot attribute placement, textarea conflict, self-closing warning, lowercase-component warning
   Effort: needs infrastructure
2. `[ ]` Port `<textarea>` dynamic child-content lowering into analyze-side metadata
   Files: likely new element-side-table metadata in `svelte_analyze`, consumed by `crates/svelte_codegen_client/src/template/element.rs` / `attributes.rs`
   Effort: moderate
3. `[ ]` Port `<option>{expr}</option>` synthetic value handling
   Files: analyze metadata + regular element codegen
   Effort: moderate
4. `[ ]` Port customizable select / selectedcontent behavior
   Files: analyze metadata + regular element codegen + maybe lowered fragment handling
   Effort: needs infrastructure
5. `[ ]` Port regular-element runtime special cases such as `autofocus`
   Files: `crates/svelte_codegen_client/src/template/attributes.rs`
   Effort: quick fix
6. `[ ]` Expand namespace edge-case coverage after the semantic gaps above land
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
