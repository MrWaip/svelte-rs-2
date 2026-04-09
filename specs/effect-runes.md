# $effect / $effect.pre

## Current state
- **Working**: 10/10 use cases
- **Missing**: none
- **Done this session**: added script-side compiler coverage for top-level and nested-expression `$effect.pending()` usage (`effect_pending_script_init`, `effect_pending_script_derived`).
- **Next**: no open implementation work in this spec
- Last updated: 2026-04-09

**Next:** no remaining implementation slices

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
- [x] `$effect.pending()` lowers in template expressions and in script-side expressions covered by compiler snapshots
  Tests: `effect_pending`, `effect_pending_script_init`, `effect_pending_script_derived`
- [x] `$effect()` placement validation: only allowed as an expression statement
  Tests: `validate_effect_invalid_placement_fn_arg`
- [x] `$effect.pre()` placement validation: only allowed as an expression statement
  Tests: `validate_effect_pre_invalid_placement_assignment`
- [x] `$effect()` argument validation: exactly one argument required
  Tests: `validate_effect_wrong_arg_count`
- [x] `$effect.pre()` argument validation: exactly one argument required
  Tests: `validate_effect_pre_wrong_arg_count`
- [x] Full validation coverage for `$effect.root`, `$effect.tracking`, `$effect.pending` argument-count and placement rules
  Tests: `validate_effect_root_wrong_arg_count`, `validate_effect_tracking_with_argument`

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

## Test cases

- [x] `effect_runes`
- [x] `effect_cleanup_return`
- [x] `effect_root_basic`
- [x] `effect_root_cleanup`
- [x] `effect_tracking`
- [x] `effect_pending`
- [x] `effect_pending_script_init`
- [x] `effect_pending_script_derived`
- [x] `validate_effect_invalid_placement_fn_arg`
- [x] `validate_effect_pre_invalid_placement_assignment`
- [x] `validate_effect_wrong_arg_count`
- [x] `validate_effect_pre_wrong_arg_count`
- [x] `validate_effect_root_wrong_arg_count`
- [x] `validate_effect_tracking_with_argument`
