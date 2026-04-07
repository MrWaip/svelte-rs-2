# A11y warnings

## Current state
- **Working**: 7/8 use cases
- **Current slice**: A11y interaction/event checks (`A11yClickEventsHaveKeyEvents`, `A11yNoNoninteractiveElementInteractions`, `A11yNoStaticElementInteractions`, `A11yMouseEventsHaveKeyEvents`)
- **Why this slice came next**: it was the next explicit analyzer-only cluster after ARIA value-type validation; it reused the existing handler/interactivity/role helpers and added only one local concept, the recommended interactive-handler subset plus static-element gating
- **Done so far**: implemented the basic A11y cluster (`A11yAccesskey`, `A11yAutofocus`, `A11yPositiveTabindex`, `A11yMissingAttribute`, `A11yDistractingElements`); implemented `A11yAriaAttributes`, `A11yUnknownAriaAttribute`, and `A11yHidden`; implemented `A11yMisplacedRole`, `A11yUnknownRole`, and `A11yNoAbstractRole`; implemented `A11yNoRedundantRoles` and `A11yRoleHasRequiredAriaProps`; implemented `A11yRoleSupportsAriaProps` and `A11yRoleSupportsAriaPropsImplicit`; implemented `A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, and `A11yNoNoninteractiveTabindex`; implemented ARIA value-type validation for known static `aria-*` attributes, matching the reference helper behavior for generic, boolean, idlist, integer, token, tokenlist, and tristate warnings; implemented the interaction/event warning cluster for click-keyboard pairing, noninteractive/static element handlers, and mouse/focus-blur pairing
- **Missing**: 1 use case — the element-content A11y cluster
- **Next**: implement the element-content A11y cluster
- **Non-goals for this run**: no non-A11y diagnostics, no parser or codegen changes
- Last updated: 2026-04-07

## Source

- ROADMAP section: `## A11y Warnings`
- Extracted from `specs/diagnostics-infrastructure.md`

## Syntax variants

- Static HTML attributes that participate in A11y checks (`alt`, `title`, `tabindex`, `aria-*`, `role`, `autocomplete`, `scope`)
- Regular elements and special elements with event handlers (`onclick`, `onkeydown`, `onfocus`, `onblur`, legacy `on:`)
- Elements whose semantics depend on implicit roles or required content (`img`, `label`, `button`, headings, media, `figure` / `figcaption`)
- Static and partially-static attribute values where the reference compiler emits warnings conservatively

## Use cases

- [x] Basic A11y attribute checks: `A11yAccesskey`, `A11yAutofocus`, `A11yPositiveTabindex`, `A11yMissingAttribute`, `A11yDistractingElements`
- [x] A11y ARIA attribute-name checks: `A11yAriaAttributes`, `A11yUnknownAriaAttribute`, `A11yHidden`
- [x] A11y role name validation: `A11yMisplacedRole`, `A11yUnknownRole`, `A11yNoAbstractRole`
- [x] A11y role semantics: `A11yNoRedundantRoles`, `A11yRoleHasRequiredAriaProps`
- [x] A11y ARIA role/property support validation: `A11yRoleSupportsAriaProps`, `A11yRoleSupportsAriaPropsImplicit`
- [x] A11y ARIA role/attribute interaction checks: `A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, `A11yNoNoninteractiveTabindex`
- [x] A11y ARIA value-type validation: `A11yIncorrectAriaAttributeType`, `A11yIncorrectAriaAttributeTypeBoolean`, `A11yIncorrectAriaAttributeTypeIdlist`, `A11yIncorrectAriaAttributeTypeInteger`, `A11yIncorrectAriaAttributeTypeToken`, `A11yIncorrectAriaAttributeTypeTokenlist`, `A11yIncorrectAriaAttributeTypeTristate` (reference helper maps schema type `id` through the generic `A11yIncorrectAriaAttributeType` path rather than the dedicated `...TypeId` warning)
- [x] A11y interaction/event checks: `A11yClickEventsHaveKeyEvents`, `A11yNoNoninteractiveElementInteractions`, `A11yNoStaticElementInteractions`, `A11yMouseEventsHaveKeyEvents`
- [ ] A11y element-content checks: `A11yConsiderExplicitLabel`, `A11yInvalidAttribute`, `A11yAutocompleteValid`, `A11yImgRedundantAlt`, `A11yLabelHasAssociatedControl`, `A11yMissingContent`, `A11yMediaHasCaption`, `A11yFigcaptionParent`, `A11yFigcaptionIndex`, `A11yMisplacedScope`

## Out of scope

- Non-A11y warning infrastructure
- Non-A11y diagnostics such as `NodeInvalidPlacementSsr`
- Parser or codegen changes unrelated to analyzer warning parity

## Reference

- `reference/compiler/warnings.js` — A11y warning codes and parameterized messages
- `reference/compiler/phases/2-analyze/visitors/shared/a11y.js` — shared A11y helpers and validation logic
- `reference/compiler/phases/2-analyze/visitors/RegularElement.js` — element-driven A11y warning entrypoints
- `reference/compiler/phases/2-analyze/visitors/shared/attribute.js` — attribute-level validation hooks used by A11y checks
- `crates/svelte_diagnostics/src/lib.rs` — `DiagnosticKind` warning variants and messages
- `crates/svelte_analyze/src/validate.rs` — analyzer validation entrypoint
- `crates/svelte_analyze/src/walker.rs` — `ctx.warn()` integration during analysis

## Tasks

- Keep A11y warning ownership in a dedicated analyzer spec instead of mixing it back into generic diagnostics infrastructure
- Implement the remaining element-content warning cluster in `svelte_analyze`
- Add or update unit coverage for the remaining warning variants
- Sync `ROADMAP.md` and related specs when the remaining open use case closes

## Implementation order

- Land the remaining element-content checks first
- Re-run or extend warning coverage
- Mark the `ROADMAP.md` item done once all A11y use cases are `[x]`

## Test cases

- [x] unit: basic A11y attribute checks
- [x] unit: `A11yAriaAttributes` / `A11yUnknownAriaAttribute` / `A11yHidden`
- [x] unit: `A11yMisplacedRole` / `A11yUnknownRole` / `A11yNoAbstractRole`
- [x] unit: `A11yNoRedundantRoles` / `A11yRoleHasRequiredAriaProps`
- [x] unit: `A11yRoleSupportsAriaProps*`
- [x] unit: `A11yAriaActivedescendantHasTabindex` / `A11yInteractiveSupportsFocus` / `A11yNoNoninteractiveTabindex`
- [x] unit: `A11yIncorrectAriaAttributeType*` (with reference-aligned generic handling for schema type `id`)
- [x] unit: `A11yClickEventsHaveKeyEvents` / `A11yNoNoninteractiveElementInteractions` / `A11yNoStaticElementInteractions` / `A11yMouseEventsHaveKeyEvents`
- [ ] unit: `A11yConsiderExplicitLabel` / `A11yInvalidAttribute` / `A11yAutocompleteValid` / `A11yImgRedundantAlt` / `A11yLabelHasAssociatedControl` / `A11yMissingContent` / `A11yMediaHasCaption` / `A11yFigcaption*` / `A11yMisplacedScope`
