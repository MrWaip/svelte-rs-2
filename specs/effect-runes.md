# $effect / $effect.pre

## Current state
- **Working**: 9/10 use cases
- **Missing**: 1 (validation for `$effect.root`/`.tracking`/`.pending`)
- **Completed (2026-04-02)**:
  - Added `EffectPre` and `EffectRoot` to `RuneKind`; wired into `detect_rune_from_call()`
  - Ported `$effect` / `$effect.pre` placement validation (`EffectInvalidPlacement`)
  - Ported arg-count validation for `$effect` / `$effect.pre` / `$effect.root` (exactly 1) and `$effect.tracking` (zero args)
  - Deleted ad-hoc `is_effect_call()` helper; replaced with canonical `detect_rune_from_call()` path
  - Unignored 4 analyzer tests: all pass
- Last updated: 2026-04-02

## Source

- ROADMAP item: `$effect` / `$effect.pre`
- Audit request: `$effect / $effect.pre`

## Syntax variants

- `$effect(() => { ... })`
- `$effect(() => { ...; return () => { ... }; })`
- `$effect.pre(() => { ... })`
- `$effect.pre(() => { ...; return () => { ... }; })`
- `$effect.root(() => { ... })` and `const cleanup = $effect.root(() => { ... })`
- `$effect.tracking()`
- `$effect.pending()`

## Use cases

- [x] Top-level `$effect(() => ...)` in component script lowers to `$.user_effect(...)`
  Tests: `effect_runes`, `effect_cleanup_return`
- [x] Top-level `$effect.pre(() => ...)` in component script lowers to `$.user_pre_effect(...)`
  Tests: `effect_runes`
- [x] `$effect` cleanup return is preserved in client output
  Tests: `effect_cleanup_return`
- [x] `$effect.root(() => ...)` lowers to `$.effect_root(...)` and preserves cleanup return values
  Tests: `effect_root_basic`, `effect_root_cleanup`
- [x] `$effect.tracking()` lowers to `$.effect_tracking()` and can flow into template output
  Tests: `effect_tracking`
- [~] `$effect.pending()` lowers in template expressions, but coverage is limited to template/branch positions rather than script-side usage
  Tests: `effect_pending`
- [x] `$effect()` placement validation: only allowed as an expression statement
  Tests: `validate_effect_invalid_placement_fn_arg`
- [x] `$effect.pre()` placement validation: only allowed as an expression statement
  Tests: `validate_effect_pre_invalid_placement_assignment`
- [x] `$effect()` argument validation: exactly one argument required
  Tests: `validate_effect_wrong_arg_count`
- [x] `$effect.pre()` argument validation: exactly one argument required
  Tests: `validate_effect_pre_wrong_arg_count`

- [ ] Full validation coverage for `$effect.root`, `$effect.tracking`, `$effect.pending` argument-count and placement rules

## Reference

- Reference docs:
  - `reference/docs/02-runes/04-$effect.md`
- Reference compiler:
  - `reference/compiler/phases/2-analyze/visitors/CallExpression.js`
  - `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`
  - `reference/compiler/phases/3-transform/server/visitors/CallExpression.js`
  - `reference/compiler/phases/3-transform/server/visitors/ExpressionStatement.js`
  - `reference/compiler/errors.js`
- Rust implementation:
  - `crates/svelte_analyze/src/utils/script_info.rs`
  - `crates/svelte_analyze/src/validate/runes.rs`
  - `crates/svelte_analyze/src/passes/js_analyze/script_body.rs`
  - `crates/svelte_analyze/src/types/script.rs`
  - `crates/svelte_codegen_client/src/script/traverse/runes.rs`
  - `tasks/compiler_tests/cases2/effect_runes/case.svelte`
  - `tasks/compiler_tests/cases2/effect_cleanup_return/case.svelte`
  - `tasks/compiler_tests/cases2/effect_root_basic/case.svelte`
  - `tasks/compiler_tests/cases2/effect_root_cleanup/case.svelte`
  - `tasks/compiler_tests/cases2/effect_tracking/case.svelte`
  - `tasks/compiler_tests/cases2/effect_pending/case.svelte`

## Tasks

1. `analyze`: extend rune classification so `detect_rune_from_call()` and `RuneKind` include `$effect.pre` and `$effect.root`
   Files: `crates/svelte_analyze/src/utils/script_info.rs`, `crates/svelte_analyze/src/types/script.rs`
2. `analyze`: port reference validation for `$effect` / `$effect.pre`
   Requirements:
   - placement must be expression-statement only
   - arg count must be exactly one
   File: `crates/svelte_analyze/src/validate/runes.rs`
3. `analyze`: fold related `$effect.*` siblings into the same validator path instead of scattered ad hoc detection
   Scope:
   - `$effect.root` exactly one argument
   - `$effect.tracking` zero arguments
   - `$effect.pending` zero arguments
4. `tests`: keep compiler snapshots that already prove client lowering, and add analyzer validation tests for missing diagnostics
   File: `crates/svelte_analyze/src/tests.rs`

## Implementation order

1. Add missing `RuneKind` variants and rune detection.
2. Port analyzer validation for `$effect` / `$effect.pre`.
3. Extend the same validator to `$effect.root` / `.tracking` / `.pending`.
4. Re-run analyzer and compiler tests for the effect cases.

## Discovered bugs

- OPEN: `crates/svelte_analyze/src/validate/runes.rs` validates state/derived placement and some rune arities, but it does not validate `$effect()` placement or arity at all.
- OPEN: `crates/svelte_analyze/src/utils/script_info.rs` does not classify `$effect.pre` or `$effect.root` as runes, which prevents analyzer-side validation from matching the reference compiler.
- OPEN: `crates/svelte_analyze/src/passes/js_analyze/script_body.rs` has an ad hoc `is_effect_call()` helper for `$effect.pre`, which keeps `needs_context` working but bypasses the main rune-classification path.

## Test cases

- Existing compiler coverage:
  - `effect_runes`
  - `effect_cleanup_return`
  - `effect_root_basic`
  - `effect_root_cleanup`
  - `effect_tracking`
  - `effect_pending`
- Added during this audit:
  - analyzer: `validate_effect_invalid_placement_fn_arg`
  - analyzer: `validate_effect_pre_invalid_placement_assignment`
  - analyzer: `validate_effect_wrong_arg_count`
  - analyzer: `validate_effect_pre_wrong_arg_count`
