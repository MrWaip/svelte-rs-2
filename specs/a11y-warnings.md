# A11y warnings

## Current state
- **Working**: 8/11 use cases
- **Current slice**: A11y interaction/event checks (`A11yClickEventsHaveKeyEvents`, `A11yNoNoninteractiveElementInteractions`, `A11yNoStaticElementInteractions`, `A11yMouseEventsHaveKeyEvents`)
- **New parity gap (2026-04-11)**: analyzer emits a false-positive `a11y_missing_attribute` warning for `<a {href}>` shorthand when `href` comes from typed `$props()` destructuring; npm `svelte/compiler` reports no diagnostics (`props/validate_props_typed_children_snippet_no_diagnostic`).
- **Why this slice came next**: it was the next explicit analyzer-only cluster after ARIA value-type validation; it reused the existing handler/interactivity/role helpers and added only one local concept, the recommended interactive-handler subset plus static-element gating
- **Done so far**: implemented the basic A11y cluster (`A11yAccesskey`, `A11yAutofocus`, `A11yPositiveTabindex`, `A11yMissingAttribute`, `A11yDistractingElements`); implemented `A11yAriaAttributes`, `A11yUnknownAriaAttribute`, and `A11yHidden`; implemented `A11yMisplacedRole`, `A11yUnknownRole`, and `A11yNoAbstractRole`; implemented `A11yNoRedundantRoles` and `A11yRoleHasRequiredAriaProps`; implemented `A11yRoleSupportsAriaProps` and `A11yRoleSupportsAriaPropsImplicit`; implemented `A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, and `A11yNoNoninteractiveTabindex`; implemented ARIA value-type validation for known static `aria-*` attributes, matching the reference helper behavior for generic, boolean, idlist, integer, token, tokenlist, and tristate warnings; implemented the interaction/event warning cluster for click-keyboard pairing, noninteractive/static element handlers, and mouse/focus-blur pairing
- **Audit revision (2026-04-07)**: reference parity review found two still-missing areas that the initial extracted spec had undercounted: role-transition warnings (`A11yNoInteractiveElementToNoninteractiveRole`, `A11yNoNoninteractiveElementToInteractiveRole`) and the broader element-content cluster
- **Missing**: 4 use cases — role-transition warnings, the element-content A11y cluster, and false-positive `a11y_missing_attribute` on shorthand dynamic `href` for anchors
- **Next**: fix `A11yMissingAttribute` anchor-`href` presence detection for shorthand/dynamic attributes, then continue role-transition warnings and the remaining element-content cluster
- **Verification**: `just test-analyzer` passed on 2026-04-07 with the current A11y slice covered and the remaining gaps still represented by ignored analyzer tests
- **Non-goals for this run**: no non-A11y diagnostics, no parser or codegen changes
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
- [ ] `A11yMissingAttribute` parity: do not warn for `<a {href}>` when `href` is a dynamic shorthand attribute sourced from `$props()` (diagnostic parity case: `props/validate_props_typed_children_snippet_no_diagnostic`)
- [x] A11y ARIA attribute-name checks: `A11yAriaAttributes`, `A11yUnknownAriaAttribute`, `A11yHidden`
- [x] A11y role name validation: `A11yMisplacedRole`, `A11yUnknownRole`, `A11yNoAbstractRole`
- [x] A11y role semantics: `A11yNoRedundantRoles`, `A11yRoleHasRequiredAriaProps`
- [ ] A11y role-transition warnings: `A11yNoInteractiveElementToNoninteractiveRole`, `A11yNoNoninteractiveElementToInteractiveRole`
- [x] A11y ARIA role/property support validation: `A11yRoleSupportsAriaProps`, `A11yRoleSupportsAriaPropsImplicit`
- [x] A11y ARIA role/attribute interaction checks: `A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, `A11yNoNoninteractiveTabindex`
- [x] A11y ARIA value-type validation: `A11yIncorrectAriaAttributeType`, `A11yIncorrectAriaAttributeTypeBoolean`, `A11yIncorrectAriaAttributeTypeIdlist`, `A11yIncorrectAriaAttributeTypeInteger`, `A11yIncorrectAriaAttributeTypeToken`, `A11yIncorrectAriaAttributeTypeTokenlist`, `A11yIncorrectAriaAttributeTypeTristate` (reference helper maps schema type `id` through the generic `A11yIncorrectAriaAttributeType` path rather than the dedicated `...TypeId` warning)
- [x] A11y interaction/event checks: `A11yClickEventsHaveKeyEvents`, `A11yNoNoninteractiveElementInteractions`, `A11yNoStaticElementInteractions`, `A11yMouseEventsHaveKeyEvents`
- [ ] `svelte-ignore a11y_no_static_element_interactions` suppresses `A11yNoStaticElementInteractions` for a static element with `mouseenter`/`mouseleave` handlers (diagnostic case: `a11y_no_static_element_interactions_ignored_on_mouseenter_mouseleave`)
- [ ] A11y element-content checks: `A11yConsiderExplicitLabel`, `A11yInvalidAttribute`, `A11yAutocompleteValid`, `A11yImgRedundantAlt`, `A11yLabelHasAssociatedControl`, `A11yMissingContent`, `A11yMediaHasCaption`, `A11yFigcaptionParent`, `A11yFigcaptionIndex`, `A11yMisplacedScope`

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
- [ ] ignored diagnostic parity: `props/validate_props_typed_children_snippet_no_diagnostic`
- [x] unit: `A11yAriaAttributes` / `A11yUnknownAriaAttribute` / `A11yHidden`
- [x] unit: `A11yMisplacedRole` / `A11yUnknownRole` / `A11yNoAbstractRole`
- [x] unit: `A11yNoRedundantRoles` / `A11yRoleHasRequiredAriaProps`
- [x] unit: `A11yRoleSupportsAriaProps*`
- [x] unit: `A11yAriaActivedescendantHasTabindex` / `A11yInteractiveSupportsFocus` / `A11yNoNoninteractiveTabindex`
- [x] unit: `A11yIncorrectAriaAttributeType*` (with reference-aligned generic handling for schema type `id`)
- [x] unit: `A11yClickEventsHaveKeyEvents` / `A11yNoNoninteractiveElementInteractions` / `A11yNoStaticElementInteractions` / `A11yMouseEventsHaveKeyEvents`
- [ ] diagnostic parity: `a11y_no_static_element_interactions_ignored_on_mouseenter_mouseleave` (`tasks/diagnostic_tests`, ignored: `diagnose-diagnostics: pending fix`)
- [ ] ignored unit: `a11y_no_interactive_element_to_noninteractive_role_warns_for_button_role_presentation`
- [ ] ignored unit: `a11y_no_noninteractive_element_to_interactive_role_warns_for_div_role_button`
- [ ] ignored unit: `a11y_consider_explicit_label_warns_for_icon_button`
- [ ] ignored unit: `a11y_invalid_attribute_warns_for_anchor_hash_href`
- [ ] ignored unit: `a11y_label_has_associated_control_warns_without_for_or_control`
- [ ] unit: `A11yConsiderExplicitLabel` / `A11yInvalidAttribute` / `A11yAutocompleteValid` / `A11yImgRedundantAlt` / `A11yLabelHasAssociatedControl` / `A11yMissingContent` / `A11yMediaHasCaption` / `A11yFigcaption*` / `A11yMisplacedScope`
