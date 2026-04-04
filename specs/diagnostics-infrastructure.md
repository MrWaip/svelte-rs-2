# 5a — Diagnostics Infrastructure Setup

## Current state
- **Working**: 18/22 use cases — infrastructure + first batch of warning emission
- **Done this session**: early bail on parser errors; `ScriptContextDeprecated`; `SlotElementDeprecated`; `AttributeAvoidIs`; `AttributeIllegalColon`; `AttributeInvalidPropertyName`
- **Missing**: A11y checks (~26 remaining variants), CSS unused selector (Tier 3 dependency), remaining non-A11y warnings (see Use cases below)
Last updated: 2026-04-04

## Source
ROADMAP Tier 5, item 5a

## Reference
- Svelte reference:
  - `reference/compiler/warnings.js` — 81 warning codes, parameterized messages, `w()` function
  - `reference/compiler/errors.js` — ~165 semantic error codes, parameterized messages
  - `reference/compiler/state.js` — ignore_stack, ignore_map, push/pop/is_ignored, warning_filter
  - `reference/compiler/utils/extract_svelte_ignore.js` — comment parsing (runes vs legacy mode)
  - `reference/compiler/phases/2-analyze/index.js` — ignore integration in `_` visitor
- Our code:
  - `crates/svelte_diagnostics/src/lib.rs` — DiagnosticKind (~274 variants), Severity, Diagnostic
  - `crates/svelte_diagnostics/src/codes.rs` — legacy map, fuzzymatch, is_valid
  - `crates/svelte_diagnostics/src/extract_svelte_ignore.rs` — svelte-ignore comment parsing
  - `crates/svelte_analyze/src/validate.rs` — empty stub (ready for 5b–5g)
  - `crates/svelte_analyze/src/walker.rs` — VisitContext with ignore stack, ctx.warn()
  - `crates/svelte_analyze/src/types/data.rs` — IgnoreData side table in AnalysisData
  - `crates/svelte_analyze/src/lib.rs` — AnalyzeOptions, warning_filter
  - `crates/svelte_compiler/src/lib.rs` — compile entry point, AnalyzeOptions construction

## Use cases

1. [x] Warning constructor `Diagnostic::warning(kind, span)` (test: unit)
2. [x] All 81 warning enum variants with `code()`, `message()`, `svelte_doc_url()` (test: unit)
3. [x] All ~165 semantic error enum variants with `code()`, `message()` (test: compile)
4. [x] `DiagnosticKind::all_warning_codes()` registry for svelte-ignore validation (test: unit)
5. [x] Legacy code migration map — 9 mappings (test: unit)
6. [x] Runes mode: comma-separated, strict validation (test: unit)
7. [x] Legacy mode: space-separated, lenient (test: unit)
8. [x] Legacy code auto-migration in svelte-ignore comments (test: unit)
9. [x] Unknown code fuzzy-match suggestion (test: unit)
10. [x] `LegacyCode` / `UnknownCode` warning emission from svelte-ignore parser (test: unit)
11. [x] Ignore stack push/pop in walker — preceding comment scan (test: integration)
12. [x] Per-node ignore snapshot in `IgnoreData` side table (test: unit)
13. [x] `is_ignored(node_id, code)` check (test: unit)
14. [x] `AnalyzeOptions` struct replacing `custom_element: bool` (test: compile)
15. [x] `warning_filter` applied after analysis (test: unit)
16. [x] `ctx.warn(node_id, kind, span)` API for visitors (test: integration)
- [x] Early bail on parser errors — skip analyze/codegen when parser produces errors
- [x] `ScriptContextDeprecated` — warn when `context="module"` used in runes mode
- [x] `SlotElementDeprecated` — warn when `<slot>` used in runes mode (non-custom-element)
- [x] `AttributeAvoidIs` — warn when element has `is` attribute
- [x] `AttributeIllegalColon` — warn when attribute name contains `:` (excluding xml/xlink/xmlns)
- [x] `AttributeInvalidPropertyName` — warn for `className`/`htmlFor` React-style props
- [ ] Remaining non-A11y warnings: `NonReactiveUpdate`, `ComponentNameLowercase`, `AttributeGlobalEventReference`, `AttributeQuoted`, `NodeInvalidPlacementSsr`, `SvelteComponentDeprecated`, `SvelteSelfDeprecated`, `SlotElementDeprecated` (legacy), options warnings, perf class warnings
- [ ] A11y checks (5f) — ~26 missing variants (ARIA role/attribute validation)
- [ ] CSS unused selector warning (depends on Tier 3)

## Tasks

### DiagnosticKind — all variants
- [x] Add `Diagnostic::warning(kind, span)` constructor
- [x] Add all 81 warning enum variants with parameterized fields
- [x] Add all ~165 semantic error enum variants with parameterized fields
- [x] Implement `code()` for all variants (snake_case matching reference)
- [x] Implement `message()` for all variants (exact messages from reference)
- [x] Implement `svelte_doc_url()` for all variants
- [x] Add `DiagnosticKind::all_warning_codes() -> &'static [&'static str]`
- [x] Add `DiagnosticKind::severity(&self) -> Severity` method
- [x] Unit tests

### Warning code registry & legacy map
- [x] `legacy_replacement(code: &str) -> Option<&'static str>` — 9 mappings
- [x] `fuzzymatch(input: &str, candidates: &[&str]) -> Option<&'static str>` — Levenshtein-based
- [x] `is_valid_warning_code(code: &str) -> bool`
- [x] Unit tests

### svelte-ignore comment parsing
- [x] `extract_svelte_ignore(offset, text, runes) -> ExtractResult`
- [x] Runes: comma-separated, strict validation, emits LegacyCode/UnknownCode
- [x] Legacy: space-separated, lenient, adds both old and new codes
- [x] Unit tests

### IgnoreData side table
- [x] `IgnoreData` struct with interned snapshots
- [x] `is_ignored()`, `intern_snapshot()`, `set_snapshot()`
- [x] Added to `AnalysisData`

### Walker ignore stack integration
- [x] Added to `VisitContext`: ignore_current, ignore_stack, runes, source, warnings
- [x] `push_ignore()`, `pop_ignore()`, `record_ignore_for_node()`
- [x] `warn()`, `take_warnings()`
- [x] Preceding Comment scan in `walk_template()`
- [x] Updated all VisitContext call sites (~6) with source + runes

### AnalyzeOptions & warning filter
- [x] `AnalyzeOptions { custom_element, runes, dev, warning_filter }`
- [x] Changed `analyze_with_options` signature
- [x] Filter applied after validate
- [x] Updated compiler + test call sites

### Validate stub
- [x] Signature unchanged — ready for 5b–5g validators
