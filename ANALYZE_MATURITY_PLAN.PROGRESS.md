# Analyze Maturity Progress

Tracks checkpoint-level progress for `ANALYZE_MATURITY_PLAN.md`.

Last updated: 2026-04-12

## Current Status

- Current slice: `Slice 1: ExprRole`
- Status: `in progress`
- Blockers: none

## Completed Checkpoints

### 1. `ExprRole` foundation

Done:
- Added analyzer-owned `ExprRole` with the initial variant set:
  - `Static`
  - `DynamicPure`
  - `DynamicWithContext`
  - `Async`
  - `RenderTag`
- Stored role next to `ExpressionInfo`
- Added `AnalysisData::expr_role(NodeId) -> Option<ExprRole>`
- Computed role inside existing analyze passes
- Added analyzer tests for all initial role variants

Notes:
- `RenderTag` stays assigned at the render-tag call site
- `ExprRole` is computed in analyze, not in codegen

### 2. `ExpressionInfo` encapsulation + semantic helper methods

Done:
- Made `ExpressionInfo` fields private
- Switched writes to specialized mutators instead of direct field mutation
- Added narrow helper methods for common semantic questions, including:
  - identifier shape checks
  - context-sensitive shape checks
  - memoization/coarse-wrap checks
- Migrated first real analyze/codegen consumers off raw field access

Notes:
- `kind` was kept as a low-level shape fact
- exact flags were not collapsed into one coarse enum

### 3. First real `ExprRole` readers

Done:
- Added role-backed readers on `ExpressionInfo` for:
  - async role
  - dynamic-with-context role
- Added `AnalysisData::expr_is_async(NodeId) -> bool`
- Added `CodegenView::expr_is_async(NodeId) -> bool`
- Switched `Ctx::expr_has_await(...)` to the role-backed node-level async query
- Switched `output.needs_context` aggregation after `ClassifyNeedsContext` to read `ExprRole::DynamicWithContext`
- Added analyzer tests covering:
  - `DynamicWithContext -> output.needs_context == true`
  - `DynamicPure -> output.needs_context == false`
  - `expr_is_async(...)`

Notes:
- exact `ExprDeps` async logic was intentionally left unchanged
- attr-level async/memo decisions still use lower-level facts where needed

## Slice 1 Remaining

Open questions:
- Are there any other real coarse expression questions that should read `ExprRole` directly?
- Or is `Slice 1` now sufficiently closed to move to `Slice 2: BindTargetSemantics`?

Do not do next:
- Do not replace exact `has_call` / `has_await` / `ref_symbols` consumers with `ExprRole` where the coarse role loses needed precision
- Do not start `BindTargetSemantics` until `Slice 1` is explicitly considered complete for the current plan standard

Recommended next checkpoint:
- Do one bounded audit of remaining coarse expression readers
- If no natural `ExprRole` consumers remain, mark `Slice 1` complete and start `Slice 2`

## Validation Last Run

- `just test-analyzer`
  - result: `203 passed`
- `cargo test -p svelte_codegen_client --no-run`
  - result: success
- `git diff --check -- crates/svelte_analyze/src crates/svelte_codegen_client/src`
  - result: clean

## Not Started

- `Slice 2: BindTargetSemantics`
- `Slice 3: concrete fragment accessors`
- `Slice 4: validator cleanup`
- `Slice 5: invariants and semantic tests`
- `Slice 6: string rediscovery reduction`
- `Slice 7: pass ownership contracts`
- `Slice 8: shared fragment traversal layer`
- `Slice 9: shared JS query helpers`
- `Slice 10: analyzer debug dump`
- `Slice 11: recovery contracts`
