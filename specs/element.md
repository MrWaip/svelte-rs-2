# Element

## Current state
- **Working**: 9/13 use cases covered (added textarea child-content lowering and option synthetic __value)
- **Partial**: template validation — `element_invalid_self_closing_tag` warning and `textarea_invalid_content` error now emitted
- **Missing**: customizable select subtree, autofocus already done; node_invalid_placement, slot_attribute_invalid_placement, component_name_lowercase diagnostics still absent
- **Next**: port remaining validation diagnostics (node_invalid_placement, slot_attribute_invalid_placement), then customizable select
- Last updated: 2026-04-02

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
- `[~]` Template validation for regular elements and element attributes
  Working: `element_invalid_self_closing_tag` warning, `textarea_invalid_content` error now emitted from `ElementFlagsVisitor`.
  Missing: `node_invalid_placement`, `slot_attribute_invalid_placement`, `component_name_lowercase`
- `[x]` `<textarea>` child-content lowering to a synthetic `value` attribute
  Implemented: `needs_textarea_value_lowering` flag in ElementFlags; codegen emits `$.remove_textarea_child` + `$.set_value` with raw expression (no constant folding). Test: `textarea_child_value_dynamic`.
- `[x]` `<option>{expr}</option>` synthetic value handling
  Implemented: `option_synthetic_value_expr` side table in ElementFlags; codegen emits `option.__value = expr` via `get_node_expr` after textContent. Test: `option_expr_child_value`.
- `[ ]` Customizable select subtree handling
  Missing behavior: `select` / `option` / `optgroup` rich-content paths and `<selectedcontent>` handling
- `[x]` `autofocus` helper path on regular elements
  Implemented: `$.autofocus(el, expr)` emitted from `attributes.rs`. Test: `element_autofocus`.
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
