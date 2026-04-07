# 5a — Diagnostics Infrastructure Setup

## Current state
- **Working**: 30/34 use cases — infrastructure + warning emission slices
- **Current slice**: A11y role/attribute interaction checks (`A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, `A11yNoNoninteractiveTabindex`)
- **Why this slice came next**: it was the next analyzer-only cluster after role/property support validation; it reused the implicit-role work and added one local interactivity/handler classification concept without pulling in value-type parsing or broader event-policy checks
- **Done this session**: early bail on parser errors; `ScriptContextDeprecated`; `SlotElementDeprecated`; `AttributeAvoidIs`; `AttributeIllegalColon`; `AttributeInvalidPropertyName`; `AttributeGlobalEventReference`; `ComponentNameLowercase`; verified `AttributeQuoted` coverage already matched the intended analyzer behavior; implemented `NonReactiveUpdate` for top-level mutated normal bindings referenced directly from template, with function-boundary suppression and `bind:this` dynamic-block parity; implemented `OptionsDeprecatedAccessors`, `OptionsDeprecatedImmutable`, and `OptionsMissingCustomElement` from preserved `<svelte:options>` attributes; implemented `PerfAvoidInlineClass` and `PerfAvoidNestedClass` from script validation with instance/module depth parity; implemented `SvelteComponentDeprecated` and `SvelteSelfDeprecated` in template validation, including filename/component-name message plumbing for the self-import hint; implemented the basic A11y cluster (`A11yAccesskey`, `A11yAutofocus`, `A11yPositiveTabindex`, `A11yMissingAttribute`, `A11yDistractingElements`); implemented `A11yAriaAttributes`, `A11yUnknownAriaAttribute`, and `A11yHidden`; implemented `A11yMisplacedRole`, `A11yUnknownRole`, and `A11yNoAbstractRole`; implemented `A11yNoRedundantRoles` and `A11yRoleHasRequiredAriaProps`; implemented `A11yRoleSupportsAriaProps` and `A11yRoleSupportsAriaPropsImplicit`; implemented `A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, and `A11yNoNoninteractiveTabindex`
- **Missing**: 4 use cases — 3 A11y slices and `NodeInvalidPlacementSsr`
- **Next**: implement the ARIA value-type validation slice (`A11yIncorrectAriaAttributeType*`) before moving on to broader interaction/content checks
- **Non-goals for this run**: no SSR placement warnings, no ARIA value-type validation, no click/static/mouse interaction warnings, no parser or codegen changes
- Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.
- Last updated: 2026-04-07

## Source

ROADMAP Tier 5, item 5a

## Use cases

- [x] Warning constructor `Diagnostic::warning(kind, span)` (test: unit)
- [x] All 81 warning enum variants with `code()`, `message()`, `svelte_doc_url()` (test: unit)
- [x] All ~165 semantic error enum variants with `code()`, `message()` (test: compile)
- [x] `DiagnosticKind::all_warning_codes()` registry for svelte-ignore validation (test: unit)
- [x] Legacy code migration map — 9 mappings (test: unit)
- [x] Runes mode: comma-separated, strict validation (test: unit)
- [x] Legacy mode: space-separated, lenient (test: unit)
- [x] Legacy code auto-migration in svelte-ignore comments (test: unit)
- [x] Unknown code fuzzy-match suggestion (test: unit)
- [x] `LegacyCode` / `UnknownCode` warning emission from svelte-ignore parser (test: unit)
- [x] Ignore stack push/pop in walker — preceding comment scan (test: integration)
- [x] Per-node ignore snapshot in `IgnoreData` side table (test: unit)
- [x] `is_ignored(node_id, code)` check (test: unit)
- [x] `AnalyzeOptions` struct replacing `custom_element: bool` (test: compile)
- [x] `warning_filter` applied after analysis (test: unit)
- [x] `ctx.warn(node_id, kind, span)` API for visitors (test: integration)
- [x] Early bail on parser errors — skip analyze/codegen when parser produces errors
- [x] `ScriptContextDeprecated` — warn when `context="module"` used in runes mode
- [x] `SlotElementDeprecated` — warn when `<slot>` used in runes mode (non-custom-element)
- [x] `AttributeAvoidIs` — warn when element has `is` attribute
- [x] `AttributeIllegalColon` — warn when attribute name contains `:` (excluding xml/xlink/xmlns)
- [x] `AttributeInvalidPropertyName` — warn for `className`/`htmlFor` React-style props
- [x] Options warnings: `OptionsDeprecatedAccessors`, `OptionsDeprecatedImmutable`, `OptionsMissingCustomElement`
- [x] Perf warnings: `PerfAvoidInlineClass`, `PerfAvoidNestedClass`
- [x] Basic A11y attribute checks: `A11yAccesskey`, `A11yAutofocus`, `A11yPositiveTabindex`, `A11yMissingAttribute`, `A11yDistractingElements`
- [x] A11y ARIA attribute-name checks: `A11yAriaAttributes`, `A11yUnknownAriaAttribute`, `A11yHidden`
- [x] A11y role name validation: `A11yMisplacedRole`, `A11yUnknownRole`, `A11yNoAbstractRole`
- [x] A11y role semantics: `A11yNoRedundantRoles`, `A11yRoleHasRequiredAriaProps`
- [x] A11y ARIA role/property support validation: `A11yRoleSupportsAriaProps`, `A11yRoleSupportsAriaPropsImplicit`
- [x] A11y ARIA role/attribute interaction checks: `A11yAriaActivedescendantHasTabindex`, `A11yInteractiveSupportsFocus`, `A11yNoNoninteractiveTabindex`
- [ ] A11y ARIA value-type validation: `A11yIncorrectAriaAttributeType`, `A11yIncorrectAriaAttributeTypeBoolean`, `A11yIncorrectAriaAttributeTypeId`, `A11yIncorrectAriaAttributeTypeIdlist`, `A11yIncorrectAriaAttributeTypeInteger`, `A11yIncorrectAriaAttributeTypeToken`, `A11yIncorrectAriaAttributeTypeTokenlist`, `A11yIncorrectAriaAttributeTypeTristate`
- [ ] A11y interaction/event checks: `A11yClickEventsHaveKeyEvents`, `A11yNoNoninteractiveElementInteractions`, `A11yNoStaticElementInteractions`, `A11yMouseEventsHaveKeyEvents`
- [ ] A11y element-content checks: `A11yConsiderExplicitLabel`, `A11yInvalidAttribute`, `A11yAutocompleteValid`, `A11yImgRedundantAlt`, `A11yLabelHasAssociatedControl`, `A11yMissingContent`, `A11yMediaHasCaption`, `A11yFigcaptionParent`, `A11yFigcaptionIndex`, `A11yMisplacedScope`
- [ ] Remaining non-A11y warnings: `NodeInvalidPlacementSsr`

## Reference

- `reference/compiler/warnings.js` — 81 warning codes, parameterized messages, `w()` function
- `reference/compiler/errors.js` — ~165 semantic error codes, parameterized messages
- `reference/compiler/state.js` — ignore_stack, ignore_map, push/pop/is_ignored, warning_filter
- `reference/compiler/utils/extract_svelte_ignore.js` — comment parsing (runes vs legacy mode)
- `reference/compiler/phases/2-analyze/index.js` — ignore integration in `_` visitor
- `crates/svelte_diagnostics/src/lib.rs` — DiagnosticKind (~274 variants), Severity, Diagnostic
- `crates/svelte_diagnostics/src/codes.rs` — legacy map, fuzzymatch, is_valid
- `crates/svelte_diagnostics/src/extract_svelte_ignore.rs` — svelte-ignore comment parsing
- `crates/svelte_analyze/src/validate.rs` — empty stub (ready for 5b–5g)
- `crates/svelte_analyze/src/walker.rs` — VisitContext with ignore stack, ctx.warn()
- `crates/svelte_analyze/src/types/data.rs` — IgnoreData side table in AnalysisData
- `crates/svelte_analyze/src/lib.rs` — AnalyzeOptions, warning_filter
- `crates/svelte_compiler/src/lib.rs` — compile entry point, AnalyzeOptions construction

## Test cases

- [x] unit: `Diagnostic::warning` constructor
- [x] unit: all 81 warning enum variants (`code()`, `message()`, `svelte_doc_url()`)
- [x] unit: `DiagnosticKind::all_warning_codes()` registry
- [x] unit: legacy code migration map (9 mappings)
- [x] unit: runes mode svelte-ignore parsing
- [x] unit: legacy mode svelte-ignore parsing
- [x] unit: legacy code auto-migration
- [x] unit: unknown code fuzzy-match suggestion
- [x] unit: `LegacyCode` / `UnknownCode` warning emission
- [x] unit: `IgnoreData` side table (`is_ignored`, `intern_snapshot`, `set_snapshot`)
- [x] unit: `warning_filter` applied after analysis
- [x] integration: ignore stack push/pop in walker
- [x] integration: `ctx.warn()` API
- [x] compile: all ~165 semantic error enum variants
- [x] compile: `AnalyzeOptions` struct
- [x] unit: `SvelteComponentDeprecated` / `SvelteSelfDeprecated`
- [x] unit: basic A11y attribute checks
- [x] unit: `A11yAriaAttributes` / `A11yUnknownAriaAttribute` / `A11yHidden`
- [x] unit: `A11yMisplacedRole` / `A11yUnknownRole` / `A11yNoAbstractRole`
- [x] unit: `A11yNoRedundantRoles` / `A11yRoleHasRequiredAriaProps`
- [x] unit: `A11yRoleSupportsAriaProps*`
- [x] unit: `A11yAriaActivedescendantHasTabindex` / `A11yInteractiveSupportsFocus` / `A11yNoNoninteractiveTabindex`
- [ ] unit: `A11yIncorrectAriaAttributeType*`
- [ ] unit: `A11yClickEventsHaveKeyEvents` / `A11yNoNoninteractiveElementInteractions` / `A11yNoStaticElementInteractions` / `A11yMouseEventsHaveKeyEvents`
- [ ] unit: `A11yConsiderExplicitLabel` / `A11yInvalidAttribute` / `A11yAutocompleteValid` / `A11yImgRedundantAlt` / `A11yLabelHasAssociatedControl` / `A11yMissingContent` / `A11yMediaHasCaption` / `A11yFigcaption*` / `A11yMisplacedScope`
