---
name: run-analyze-plan
description: Execute the next concrete checkpoint from ANALYZE_MATURITY_PLAN.md, or a user-specified slice, without jumping ahead, inventing generic frameworks, or leaving half-integrated analyzer refactors. Use when the user asks to continue the analyze maturity plan, take the next slice, implement part of a slice, or drive ANALYZE_MATURITY_PLAN.md across sessions.
---

# Run Analyze Plan

Execute `ANALYZE_MATURITY_PLAN.md` as a sequence of stable checkpoints, not as one giant refactor.

## Read First

1. `README.md`
2. `CLAUDE.md`
3. `ANALYZE_MATURITY_PLAN.md`
4. `CODEBASE_MAP.md` only if the current checkpoint touches shared APIs or types

Use `phase-boundaries`, `test-pattern`, and `oxc-analyze-api` when the current checkpoint needs them.

## Choose The Target

If the user names a slice, stay inside that slice.

If the user says "next", "continue", or "гони план":

1. Read `ANALYZE_MATURITY_PLAN.md` from the top.
2. Pick the earliest slice whose required deliverables are not obviously complete.
3. Do a bounded code check for that slice only.
4. Do not pre-audit all later slices before starting work.

If the current slice is ambiguous after a bounded inspection, ask the user which slice to target instead of guessing across multiple slices.

## Work Unit

Do not treat one slice as one session.

The unit of work is one shippable checkpoint inside the current slice. Good checkpoint shapes:

- one new analyzer-owned fact plus storage, accessor, and tests
- one shared helper plus at least one real migrated consumer
- one validator cleanup step plus matching diagnostic parity coverage
- one ownership contract update that matches the touched code in the same diff

Bad checkpoint shapes:

- helper added but unused
- enum or accessor added but no real caller uses it
- half-migrated validator with two competing paths
- file split with no new semantic fact behind it
- starting the next slice before the current checkpoint is stable

## Workflow

### 1. Lock The Checkpoint

Before editing, state the exact checkpoint in one sentence:

- `Slice 1: add ExprRole storage + accessor + first analyzer tests`
- `Slice 8: add fragment traversal helper + migrate template_side_tables`
- `Slice 9: extract invalid each assignment query from template_validation`

Keep the checkpoint narrower than the whole slice unless the slice is genuinely small enough to finish cleanly.

### 2. Gather Bounded Context

Read only the files needed for the current checkpoint:

- the current slice section in `ANALYZE_MATURITY_PLAN.md`
- the owning pass and data files
- the first consumer to migrate
- the first tests to update

When navigating Rust code, use `cclsp` first. Fall back to `rg` only for non-code text search or after a failed retry.

### 3. Implement The Smallest Complete Cut

Finish the checkpoint end to end:

1. add the fact/helper/type in the owning layer
2. wire one real consumer to it
3. remove the touched duplicate logic
4. add or update tests
5. verify

Do not land scaffolding without a consumer.

## Verification Matrix

Pick the narrowest correct validation for the checkpoint:

- semantic fact, accessor, traversal helper, invariants: `just test-analyzer`
- validator cleanup or diagnostic behavior: add or update `tasks/diagnostic_tests` and run `just test-diagnostic-case <name>` or `just test-diagnostics`
- output-affecting change outside diagnostics: `just test-compiler` or the smallest owning test case

When a validator checkpoint changes user-visible diagnostics, `tasks/diagnostic_tests` are required. Analyzer tests do not replace them.

## Slice-Specific Notes

- Slices 1-3: prefer analyzer tests that assert facts directly, not only final JS.
- Slice 4: validator cleanup must read canonical facts and must be protected by diagnostic parity tests.
- Slice 5: invariants must stay analyzer-local; do not turn them into client policy.
- Slice 6: remove semantic meaning hidden in string rediscovery only where a typed fact or `SymbolId` path exists.
- Slice 7: document pass ownership and traversal family only for touched passes.
- Slice 8: shared fragment traversal is for non-owner fragment recursion first. Do not rewrite `build_component_semantics` or main `lower` recursion just for symmetry.
- Slice 9: shared JS query helpers must answer concrete questions, not become a generic visitor framework.
- Slice 10: debug dump is for analyzer facts only.
- Slice 11: recovery contracts must define which queries are safe on broken source and which may return `None`.

## Stop Rules

Stop at a stable boundary if:

- the next step would require client-only policy to answer an analyzer question
- the new type starts turning into hidden lowering IR
- the current cleanup needs a prerequisite fact from an earlier unfinished slice
- the session would otherwise spill into the next slice

If blocked after repeated attempts:

1. keep the stable part
2. do not jump ahead to another slice
3. report the blocker and the missing prerequisite

## Final Report Format

End each run with four things:

1. current slice and checkpoint completed
2. files changed
3. tests run
4. remaining work inside the same slice

Use `done / remaining` language. If the slice is still open, say so explicitly and do not claim the next slice has started.

## Guardrails

- Do not invent a generic query framework ahead of the concrete slices.
- Do not move client runtime or emission policy into `svelte_analyze`.
- Do not force all passes onto one traversal style.
- Do not rewrite special-owner traversals onto shared helpers for uniformity.
- Do not add new hand-written fragment recursion in non-owner passes once the shared helper exists.
- Do not add new one-off OXC micro-visitors for a question already owned by a shared JS helper.
