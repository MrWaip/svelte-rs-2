# 5a — Diagnostics Infrastructure Setup

## Current state
- **Working**: 24/25 use cases — infrastructure + non-A11y warning emission slices
- **Done this session**: added diagnostic parity harness under `tasks/diagnostic_tests/`; `just generate` now writes `case-svelte.json` reference diagnostics and tests write `case-rust.json` for visual comparison. Earlier completed items remain: early bail on parser errors; `ScriptContextDeprecated`; `AttributeAvoidIs`; `AttributeIllegalColon`; `AttributeInvalidPropertyName`; `AttributeGlobalEventReference`; `ComponentNameLowercase`; verified `AttributeQuoted` coverage already matched the intended analyzer behavior; implemented `NonReactiveUpdate` for top-level mutated normal bindings referenced directly from template, with function-boundary suppression and `bind:this` dynamic-block parity; implemented `OptionsDeprecatedAccessors`, `OptionsDeprecatedImmutable`, and `OptionsMissingCustomElement` from preserved `<svelte:options>` attributes; implemented `PerfAvoidInlineClass` and `PerfAvoidNestedClass` from script validation with instance/module depth parity
- **Missing**: 1 use case — `NodeInvalidPlacementSsr`
- **Next**: implement the standalone SSR placement warning
- **Non-goals for this run**: no A11y warnings in this spec, no parser or codegen changes
- Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.
- Last updated: 2026-04-11

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
- [x] `AttributeAvoidIs` — warn when element has `is` attribute
- [x] `AttributeIllegalColon` — warn when attribute name contains `:` (excluding xml/xlink/xmlns)
- [x] `AttributeInvalidPropertyName` — warn for `className`/`htmlFor` React-style props
- [x] Options warnings: `OptionsDeprecatedAccessors`, `OptionsDeprecatedImmutable`, `OptionsMissingCustomElement`
- [x] Perf warnings: `PerfAvoidInlineClass`, `PerfAvoidNestedClass`
- [x] Diagnostic parity harness with generated `case-svelte.json` / `case-rust.json` snapshots (test: integration)
- [ ] Remaining non-A11y warnings: `NodeInvalidPlacementSsr`

## Reference

- `reference/compiler/warnings.js` — 81 warning codes, parameterized messages, `w()` function
- `reference/compiler/errors.js` — ~165 semantic error codes, parameterized messages
- `reference/compiler/state.js` — ignore_stack, ignore_map, push/pop/is_ignored, warning_filter
- `reference/compiler/utils/extract_svelte_ignore.js` — comment parsing (runes vs legacy mode)
- `reference/compiler/phases/2-analyze/index.js` — ignore integration in `_` visitor
- `specs/a11y-warnings.md` — dedicated ownership for all A11y warning parity work
- `crates/svelte_diagnostics/src/lib.rs` — DiagnosticKind (~274 variants), Severity, Diagnostic
- `crates/svelte_diagnostics/src/codes.rs` — legacy map, fuzzymatch, is_valid
- `crates/svelte_diagnostics/src/extract_svelte_ignore.rs` — svelte-ignore comment parsing
- `crates/svelte_analyze/src/validate.rs` — empty stub (ready for 5b–5g)
- `crates/svelte_analyze/src/walker.rs` — VisitContext with ignore stack, ctx.warn()
- `crates/svelte_analyze/src/types/data.rs` — IgnoreData side table in AnalysisData
- `crates/svelte_analyze/src/lib.rs` — AnalyzeOptions, warning_filter
- `crates/svelte_compiler/src/lib.rs` — compile entry point, AnalyzeOptions construction
- `tasks/diagnostic_tests/test_diagnostics.rs` — diagnostic parity harness
- `tasks/generate_test_cases/generate.mjs` — reference diagnostic snapshot generation

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
- [x] integration: diagnostic parity snapshot harness (`case-svelte.json` / `case-rust.json`)
- [x] compile: all ~165 semantic error enum variants
- [x] compile: `AnalyzeOptions` struct
- [x] unit: `SvelteComponentDeprecated` / `SvelteSelfDeprecated`
- [ ] `attribute_global_event_reference_missing_binding`
- [ ] `attribute_global_event_reference_local_binding`
- [x] `attribute_quoted_on_component`
- [x] `attribute_quoted_custom_element`
- [x] `attribute_quoted_regular_element_no_warn`
- [x] `component_attribute_illegal_colon_warns`
- [x] `component_name_lowercase_unused_import`
- [x] `component_name_lowercase_plain_html_element`
- [ ] `options_deprecated_accessors_runes`
- [x] `options_deprecated_accessors_legacy`
- [ ] `options_deprecated_immutable_runes`
- [x] `options_deprecated_immutable_legacy`
- [ ] `validate_options_custom_element_warns_without_compiler_flag`
- [x] `validate_options_custom_element_no_warn_with_compiler_flag`
- [x] `validate_perf_avoid_nested_class_no_warning_at_instance_top_level`
- [x] `validate_perf_avoid_nested_class_warns_in_instance_nested_function`
- [x] `validate_perf_avoid_nested_class_no_warning_at_module_top_level`
- [x] `validate_perf_avoid_nested_class_warns_in_module_nested_function`
- [x] `validate_perf_avoid_inline_class_warns_at_instance_top_level`
- [x] `validate_perf_avoid_inline_class_no_warning_at_module_top_level`
- [x] `validate_perf_avoid_inline_class_warns_in_nested_function`
- [ ] `validate_non_reactive_update_for_direct_template_read`
- [ ] `validate_non_reactive_update_no_warning_across_function_boundary`
- [x] `validate_non_reactive_update_bind_this_no_warning_without_dynamic_block`
- [ ] `validate_non_reactive_update_bind_this_warns_inside_if_block`
