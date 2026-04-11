# A11y warnings

## Current state
- **Working**: 16/16 use cases
- **Current slice completed**: `A11yAutocompleteValid` + `A11yImgRedundantAlt` + `A11yMisplacedScope`. Invalid static `autocomplete` values now warn, redundant image wording in static `alt` warns, and `scope` on non-`<th>` elements now matches the reference warning span.
- **Previous slice completed**: `A11yMissingContent` + `A11yMediaHasCaption` + `A11yFigcaption*`. Empty headings now warn for missing content, `<video src>` without a caption track warns, `<figcaption>` outside `<figure>` warns, and middle-position figcaptions in `<figure>` now match the reference.
- **Drift normalized (2026-04-11)**: `a11y_no_noninteractive_element_to_interactive_role_warns_for_div_role_button` was a stale mismatch marker. Both npm `svelte/compiler` and Rust report no diagnostics for `<div role="button"></div>`, so the case is now only a no-warning regression guard and not evidence that `A11yNoNoninteractiveElementToInteractiveRole` is complete.
- **Count normalized (2026-04-11)**: the old single “element-content cluster” checkbox hid multiple independent warning groups. It is now split into focused use cases so progress can resume cleanly.
- **Why this slice came next**: it was the last remaining analyzer-only cluster, and all three warnings are driven by static attribute inspection in the same validation module rather than by new fragment traversal or cross-pass data flow.
- **Done so far**: implemented the basic A11y cluster (`A11yAccesskey`, `A11yAutofocus`, `A11yPositiveTabindex`, `A11yMissingAttribute`, `A11yDistractingElements`); implemented `A11yAriaAttributes`, `A11yUnknownAriaAttribute`, and `A11yHidden`; implemented `A11yMisplacedRole`, `A11yUnknownRole`, and `A11yNoAbstractRole`; implemented `A11yNoRedundantRoles` and `A11yRoleHasRequiredAriaProps`; implemented `A11yNoInteractiveElementToNoninteractiveRole` and `A11yNoNoninteractiveElementToInteractiveRole`; implemented `A11yRoleSupportsAriaProps` and `A11yRoleSupportsAriaPropsImplicit`; implemented `A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, and `A11yNoNoninteractiveTabindex`; implemented ARIA value-type validation for known static `aria-*` attributes, matching the reference helper behavior for generic, boolean, idlist, integer, token, tokenlist, and tristate warnings; implemented the interaction/event warning cluster for click-keyboard pairing, noninteractive/static element handlers, and mouse/focus-blur pairing
- **Audit revision (2026-04-07)**: reference parity review found two still-missing areas that the initial extracted spec had undercounted: role-transition warnings (`A11yNoInteractiveElementToNoninteractiveRole`, `A11yNoNoninteractiveElementToInteractiveRole`) and the broader element-content cluster
- **Missing**: none for this spec
- **Next**: feature complete for the current spec scope
- **Verification**: `cargo test -p svelte_analyze invalid_input_autocomplete_warns`, `cargo test -p svelte_analyze valid_input_autocomplete_does_not_warn`, `cargo test -p svelte_analyze redundant_img_alt_warns`, `cargo test -p svelte_analyze th_scope_does_not_warn_for_misplaced_scope`, `just test-diagnostic-case a11y_autocomplete_valid_warns_for_invalid_input_token`, `just test-diagnostic-case a11y_img_redundant_alt_warns_for_redundant_image_wording`, `just test-diagnostic-case a11y_misplaced_scope_warns_on_td`, `just test-analyzer`, and `just test-diagnostics`
- **Non-goals for the completed slice**: non-A11y diagnostics and any parser/codegen changes
- Last updated: 2026-04-11

## Source

- ROADMAP section: `## A11y Warnings`
- Extracted from `specs/diagnostics-infrastructure.md`

## Syntax variants

- `<div accesskey="a">content</div>`
- `<input autofocus />`
- `<div tabindex="2">content</div>`
- `<div aria-hidden="true">Title</div>`
- `<div aria-label="name"></div>`
- `<div role="button"></div>`
- `<button role="presentation"></button>`
- `<div role="button" onclick={handle}></div>`
- `<div onclick={handle} onkeydown={handle}></div>`
- `<button><svg /></button>`
- `<a href="#">link</a>`
- `<input type="image" src="submit.png" />`
- `<input type="text" autocomplete="totally-wrong" />`
- `<img alt="image of a cat" />`
- `<label>Username</label>`
- `<video src="movie.mp4"></video>`
- `<figcaption>Caption</figcaption>`
- `<figure><img /><p>middle</p><figcaption>Caption</figcaption></figure>`
- `<td scope="col">value</td>`
- `<h1></h1>`

## Use cases

- [x] Basic A11y attribute checks: `A11yAccesskey`, `A11yAutofocus`, `A11yPositiveTabindex`, `A11yMissingAttribute`, `A11yDistractingElements`
- [x] `A11yMissingAttribute` parity: do not warn for `<a {href}>` when `href` is a dynamic shorthand attribute sourced from `$props()` (diagnostic parity case: `props/validate_props_typed_children_snippet_no_diagnostic`)
- [x] A11y ARIA attribute-name checks: `A11yAriaAttributes`, `A11yUnknownAriaAttribute`, `A11yHidden`
- [x] A11y role name validation: `A11yMisplacedRole`, `A11yUnknownRole`, `A11yNoAbstractRole`
- [x] A11y role semantics: `A11yNoRedundantRoles`, `A11yRoleHasRequiredAriaProps`
- [x] `A11yNoInteractiveElementToNoninteractiveRole`: interactive elements must warn when a static `role` token forces them into a non-interactive or presentation role (diagnostic test: `a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation_with_text`; the original unlabeled button repro remains ignored until `A11yConsiderExplicitLabel` lands)
- [x] `A11yNoNoninteractiveElementToInteractiveRole`: non-interactive semantic elements warn when given an interactive role, excluding reference exceptions (diagnostic test: `a11y_no_noninteractive_element_to_interactive_role_warns_for_footer_role_button`; existing `a11y_no_noninteractive_element_to_interactive_role_warns_for_div_role_button` remains a no-warning regression guard)
- [x] A11y ARIA role/property support validation: `A11yRoleSupportsAriaProps`, `A11yRoleSupportsAriaPropsImplicit`
- [x] A11y ARIA role/attribute interaction checks: `A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, `A11yNoNoninteractiveTabindex`
- [x] A11y ARIA value-type validation: `A11yIncorrectAriaAttributeType`, `A11yIncorrectAriaAttributeTypeBoolean`, `A11yIncorrectAriaAttributeTypeIdlist`, `A11yIncorrectAriaAttributeTypeInteger`, `A11yIncorrectAriaAttributeTypeToken`, `A11yIncorrectAriaAttributeTypeTokenlist`, `A11yIncorrectAriaAttributeTypeTristate` (reference helper maps schema type `id` through the generic `A11yIncorrectAriaAttributeType` path rather than the dedicated `...TypeId` warning)
- [x] A11y interaction/event checks: `A11yClickEventsHaveKeyEvents`, `A11yNoNoninteractiveElementInteractions`, `A11yNoStaticElementInteractions`, `A11yMouseEventsHaveKeyEvents`
- [x] `svelte-ignore a11y_no_static_element_interactions` suppresses `A11yNoStaticElementInteractions` for a static element with `mouseenter`/`mouseleave` handlers (diagnostic case: `a11y_no_static_element_interactions_ignored_on_mouseenter_mouseleave`)
- [x] A11y element-content checks for the shared `a/button` branch: `A11yConsiderExplicitLabel`, `A11yInvalidAttribute`
- [x] A11y label-control association check: `A11yLabelHasAssociatedControl`
- [x] A11y child-content/structure checks: `A11yMissingContent`, `A11yMediaHasCaption`, `A11yFigcaptionParent`, `A11yFigcaptionIndex`
- [x] Remaining A11y static-attribute/content-token checks: `A11yAutocompleteValid`, `A11yImgRedundantAlt`, `A11yMisplacedScope`

## Out of scope

- Non-A11y warning infrastructure
- Non-A11y diagnostics such as `NodeInvalidPlacementSsr`
- Parser or codegen changes unrelated to analyzer warning parity

## Reference

- `reference/compiler/warnings.js` — A11y warning codes and parameterized messages
- `reference/compiler/phases/2-analyze/visitors/shared/a11y.js` — shared A11y helpers and validation logic
- `reference/compiler/phases/2-analyze/visitors/shared/a11y/constants.js` — canonical required attrs/content, implicit roles, and handler lists
- `reference/compiler/phases/2-analyze/visitors/RegularElement.js` — element-driven A11y warning entrypoints
- `reference/compiler/phases/2-analyze/visitors/shared/attribute.js` — attribute-level validation hooks used by A11y checks
- `crates/svelte_diagnostics/src/lib.rs` — `DiagnosticKind` warning variants and messages
- `crates/svelte_analyze/src/validate.rs` — analyzer validation entrypoint
- `crates/svelte_analyze/src/walker.rs` — `ctx.warn()` integration during analysis
- `crates/svelte_analyze/src/passes/template_validation/a11y.rs` — current Rust implementation surface
- `crates/svelte_analyze/src/tests/a11y.rs` — analyzer warning coverage

## Test cases

- [x] unit: basic A11y attribute checks
- [x] diagnostic parity: `props/validate_props_typed_children_snippet_no_diagnostic`
- [x] unit: `A11yAriaAttributes` / `A11yUnknownAriaAttribute` / `A11yHidden`
- [x] unit: `A11yMisplacedRole` / `A11yUnknownRole` / `A11yNoAbstractRole`
- [x] unit: `A11yNoRedundantRoles` / `A11yRoleHasRequiredAriaProps`
- [x] unit: `A11yRoleSupportsAriaProps*`
- [x] unit: `A11yAriaActivedescendantHasTabindex` / `A11yInteractiveSupportsFocus` / `A11yNoNoninteractiveTabindex`
- [x] unit: `A11yIncorrectAriaAttributeType*` (with reference-aligned generic handling for schema type `id`)
- [x] unit: `A11yClickEventsHaveKeyEvents` / `A11yNoNoninteractiveElementInteractions` / `A11yNoStaticElementInteractions` / `A11yMouseEventsHaveKeyEvents`
- [x] diagnostic parity: `a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation_with_text`
- [x] diagnostic parity: `a11y_no_static_element_interactions_ignored_on_mouseenter_mouseleave`
- [x] diagnostic parity: `a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation`
- [x] diagnostic parity no-warning regression: `a11y_no_noninteractive_element_to_interactive_role_warns_for_div_role_button` (reference snapshot is empty)
- [x] diagnostic parity: `a11y_no_noninteractive_element_to_interactive_role_warns_for_footer_role_button`
- [x] diagnostic parity: `a11y_consider_explicit_label_warns_for_icon_button`
- [x] diagnostic parity: `a11y_invalid_attribute_warns_for_anchor_hash_href`
- [x] diagnostic parity: `a11y_label_has_associated_control_warns_without_for_or_control`
- [x] diagnostic parity: `a11y_missing_content_warns_for_empty_h1`
- [x] diagnostic parity: `a11y_media_has_caption_warns_for_video_without_caption_track`
- [x] diagnostic parity: `a11y_figcaption_parent_warns_outside_figure`
- [x] diagnostic parity: `a11y_figcaption_index_warns_for_middle_figcaption`
- [x] diagnostic parity: `a11y_autocomplete_valid_warns_for_invalid_input_token`
- [x] diagnostic parity: `a11y_img_redundant_alt_warns_for_redundant_image_wording`
- [x] diagnostic parity: `a11y_misplaced_scope_warns_on_td`
- [x] unit: `A11yConsiderExplicitLabel` / `A11yInvalidAttribute`
- [x] unit: `A11yLabelHasAssociatedControl`
- [x] unit: `A11yMissingContent` / `A11yMediaHasCaption` / `A11yFigcaption*`
- [x] unit: `A11yAutocompleteValid` / `A11yImgRedundantAlt` / `A11yMisplacedScope`
