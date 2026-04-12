# Analyze Maturity Progress

Tracks checkpoint-level progress for `ANALYZE_MATURITY_PLAN.md`.

Last updated: 2026-04-12

## Current Status

- Current slice: `Slice 2: BindTargetSemantics`
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

### 4. `BindTargetSemantics` foundation across element, special, and component hosts

Done:
- Added analyzer-owned bind semantics types:
  - `BindHostKind`
  - `BindPropertyKind`
  - `BindTargetSemantics`
- Covered host families:
  - `Element`
  - `Component`
  - `Window`
  - `Document`
  - `Body`
- Covered initial property families:
  - `Value`
  - `Checked`
  - `Group`
  - `Files`
  - `Media`
  - `Dimension`
  - `This`
  - `ContentEditable`
  - `ComponentProp`
  - `Other`
- Stored semantics in analyzer-owned bind data and exposed `AnalysisData::bind_target_semantics(NodeId)`
- Computed semantics in `bind_semantics` from `ParentKind`, instead of caller-side host/property rediscovery
- Migrated first real consumers:
  - `template_validation` now reads bind:this/contenteditable/mutable-target semantics through the accessor
  - `validate/non_reactive_update` now detects bind:this via analyzer semantics instead of `name == "this"`
  - `element_flags` now classifies component `bind:this` via the same bind semantics type instead of a raw string branch
- Added analyzer tests for:
  - regular element `bind:value`
  - input `bind:checked`
  - element `bind:this`
  - window/document binds
  - component prop bind
  - component `bind:this`

Notes:
- `BindHostKind::Component` covers all `ParentKind::ComponentNode` cases, including normal components, `<svelte:self>`, and `<svelte:component>`
- `ComponentBindMode` still owns component emission policy; it was not collapsed into bind target semantics

## Slice 2 Remaining

Open questions:
- Which remaining analyze/validation bind decisions still rediscover host/property meaning from raw names and should move to `bind_target_semantics` next?
- Does component bind validation need an explicit host-specific contract here, or is the current “component props stay open-ended” behavior the intended rule?

Do not do next:
- Do not move client runtime helper names or bind runtime behavior into `BindTargetSemantics`
- Do not collapse `ComponentBindMode` into bind target semantics just for uniformity
- Do not rewrite codegen bind lowering unless the next checkpoint removes a real duplicated semantic decision

Recommended next checkpoint:
- Do one bounded audit of remaining bind string-rediscovery sites in analyze/validation
- Migrate the next real bind consumer only if it can read host/property/mutability directly from `BindTargetSemantics`

## Validation Last Run

- `just test-analyzer`
  - result: `205 passed`
- `cargo test -p svelte_codegen_client --no-run`
  - result: success
- `git diff --check -- crates/svelte_analyze/src crates/svelte_codegen_client/src`
  - result: clean

## Not Started

- `Slice 3: concrete fragment accessors`
- `Slice 4: validator cleanup`
- `Slice 5: invariants and semantic tests`
- `Slice 6: string rediscovery reduction`
- `Slice 7: pass ownership contracts`
- `Slice 8: shared fragment traversal layer`
- `Slice 9: shared JS query helpers`
- `Slice 10: analyzer debug dump`
- `Slice 11: recovery contracts`
