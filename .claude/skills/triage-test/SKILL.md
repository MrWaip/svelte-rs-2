---
name: triage-test
description: Triage one failing compiler test to decide whether it should be fixed locally, escalated into a slice for port2, or sent back to audit/spec work. Use when a failing test may hide a larger architectural or feature-completeness gap and Claude should classify the work before implementing.
---

# Triage Failing Test

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

Triage a single failing compiler test case. The test name is provided as input.

## Goal

Use one failing test as an entry point to classify the real task size before coding.

This skill does not assume the test is a small fix.

The output must classify the failure as one of:

- `local-fix` — one bounded bug in an existing implementation path
- `slice-gap` — missing behavior that belongs in a bounded `port2` slice
- `spec-gap` — the feature or use-case map is incomplete and needs `audit` or spec revision first

## Approach

When reading the reference compiler, focus on the output and required semantics, not JS visitor structure.

Do not:

- assume one failing test means one small fix
- expand from one test into multiple implementation targets without reclassification
- edit `case-svelte.js` or `case-rust.js`

## PLAN PHASE

These steps are read-only. Complete them before writing any code.

### Step 1: Understand The Failure

Run:

```bash
just test-case-verbose <test-name>
```

Read the three files in `tasks/compiler_tests/cases2/<test-name>/`:

- `case.svelte`
- `case-svelte.js`
- `case-rust.js`

List the observable mismatches between `case-rust.js` and `case-svelte.js`.

### Step 2: Diagnose The Owning Layer

Determine the most likely owning layer in this order:

1. parser or AST
2. analysis
3. codegen

If the failure spans more than one layer, note which layer should own the first correct change.

### Step 3: Research

Research two things:

1. how the reference compiler handles this specific case
2. where the corresponding behavior lives in our compiler and what data is missing

If the test is tracked in a spec, read that spec's `Current state` and `Use cases`.

### Step 4: Classify The Task

Classify the failing test as exactly one of these:

#### `local-fix`

Use this only when all of these are true:

- one existing implementation path is clearly wrong or incomplete
- no new feature slice is required
- the change stays bounded to one small bug or one tightly related behavior

#### `slice-gap`

Use this when any of these are true:

- the test reveals a missing use-case cluster, not just one bug
- the correct fix requires a bounded new analysis or codegen step
- the test is small, but the required implementation is a real feature slice

#### `spec-gap`

Use this when any of these are true:

- the test reveals behavior not represented in the current spec
- the owning layer or implementation order is still unclear after research
- the failing case suggests the feature was under-audited

### Step 5: Produce The Recommendation

Produce a recommendation with these sections:

1. Classification
2. Root cause
3. Owning layer
4. Why this is not just "one test"
5. Next command

`Next command` must be exactly one of:

- `/fix-test <name>` for `local-fix`
- `/port2 <spec-or-feature>` for `slice-gap`
- `/audit <feature>` or spec update for `spec-gap`

If the classification is `slice-gap` or `spec-gap`, stop after reporting. Do not start implementation in this skill.

## EXECUTE PHASE

Start here only if the classification is `local-fix` and the user explicitly wants this skill to continue through the fix.

### Step 6: Fix

Implement only the classified local fix.

If the work grows into `slice-gap` or `spec-gap`, stop, update the recommendation, and do not keep coding under the `local-fix` assumption.

### Step 7: Add Tests

Choose the smallest correct test surface:

- parser or analyze behavior -> unit tests in `test.rs` modules
- end-to-end compiler output differences -> keep the existing compiler test and add unit tests only if they improve layer-local coverage

Never edit `case-svelte.js` or `case-rust.js`.

### Step 8: Verify

Run:

```bash
just test-case <test-name>
```

Then run the relevant unit test command if unit tests were added, followed by:

```bash
just test-compiler
```

If the fix breaks unrelated tests, stop and report. Do not broaden scope.

### Step 9: Update Spec

If the test is tracked in a spec:

- mark it fixed for `local-fix`
- or add it as an unchecked use case if triage discovered a larger missing slice
- update `Current state` with the triage outcome when useful

## Summary

Report:

- classification
- root cause
- owning layer
- recommended next command

Then recommend:

- `/qa` after a completed `local-fix`
- `/port2` for bounded feature work
- `/audit` when the spec is incomplete
