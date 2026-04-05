# 5a — Diagnostics Infrastructure Setup

## Current state
- **Working**: 18/22 use cases — infrastructure + first batch of warning emission
- **Done this session**: early bail on parser errors; `ScriptContextDeprecated`; `SlotElementDeprecated`; `AttributeAvoidIs`; `AttributeIllegalColon`; `AttributeInvalidPropertyName`
- **Missing**: A11y checks (~26 remaining variants), CSS unused selector (Tier 3 dependency), remaining non-A11y warnings (see Use cases below)
- **Next**: Emit `NonReactiveUpdate`, `ComponentNameLowercase`, `AttributeGlobalEventReference`, `AttributeQuoted`, and `SvelteComponentDeprecated` warnings in the walker validate pass.
- Last updated: 2026-04-04

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
- [ ] Remaining non-A11y warnings: `NonReactiveUpdate`, `ComponentNameLowercase`, `AttributeGlobalEventReference`, `AttributeQuoted`, `NodeInvalidPlacementSsr`, `SvelteComponentDeprecated`, `SvelteSelfDeprecated`, `SlotElementDeprecated` (legacy), options warnings, perf class warnings
- [ ] A11y checks (5f) — ~26 missing variants (ARIA role/attribute validation)
- [ ] CSS unused selector warning (depends on Tier 3)

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
