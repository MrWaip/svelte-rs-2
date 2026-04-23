---
name: fix-test
description: Triage one failing compiler test to decide whether it should be fixed locally, escalated into a slice for port, or sent back to audit/spec work. Use when a failing test may hide a larger architectural or feature-completeness gap and Claude should classify the work before implementing.
---

# Triage Failing Test

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

Triage one failing compiler test case. Test name = input.

## Goal

Use one failing test as entry point to classify real task size before coding.

Skill does not assume test is small fix.

Output must classify failure as one of:

- `local-fix` — one bounded bug in existing implementation path
- `slice-gap` — missing behavior belongs in bounded `port` slice
- `spec-gap` — feature or use-case map incomplete, needs `audit` or spec revision first

## Approach

Reading reference compiler -> focus on output and required semantics, not JS visitor structure.

Do not:

- assume one failing test = one small fix
- expand from one test into multiple implementation targets without reclassification
- edit `case-svelte.js` or `case-rust.js`

## PLAN PHASE

Read-only. Complete before writing any code.

### Step 1: Understand The Failure

Run:

```bash
just test-case-verbose <test-name>
```

Read three files in `tasks/compiler_tests/cases2/<test-name>/`:

- `case.svelte`
- `case-svelte.js`
- `case-rust.js`

List observable mismatches between `case-rust.js` and `case-svelte.js`.

### Step 2: Diagnose The Owning Layer

Determine most likely owning layer, in this order:

1. parser or AST
2. analysis
3. codegen

Failure spans >1 layer -> note which layer owns first correct change.

### Step 3: Research

Research two things:

1. how reference compiler handles this specific case
2. where corresponding behavior lives in our compiler, what data is missing

Test tracked in spec -> read `Current state`, `Use cases`, `Tasks`.

### Step 4: Classify The Task

Exactly one of:

#### `local-fix`

Use only when all true:

- one existing implementation path clearly wrong or incomplete
- no new feature slice required
- change stays bounded to one small bug or tightly related behavior

#### `slice-gap`

Use when any true:

- test reveals missing use-case cluster, not just one bug
- correct fix requires bounded new analysis or codegen step
- test small, but required implementation = real feature slice

#### `spec-gap`

Use when any true:

- test reveals behavior not represented in current spec
- owning layer or implementation order still unclear after research
- failing case suggests feature under-audited

### Step 5: Produce The Recommendation

Sections:

1. Classification
2. Root cause
3. Owning layer
4. Why this is not just "one test"
5. Next command

`Next command` must be exactly one of:

- `/fix-test <name>` for `local-fix`
- `/port <spec-or-feature>` for `slice-gap`
- `/audit <feature>` or spec update for `spec-gap`

Classification `slice-gap` or `spec-gap` -> stop after reporting. No implementation in this skill.

## EXECUTE PHASE

Start only if classification = `local-fix` and user explicitly wants skill to continue through fix.

### Step 6: Fix

Implement only classified local fix.

Work grows into `slice-gap` or `spec-gap` -> stop, update recommendation. No coding under `local-fix` assumption.

### Step 7: Add Tests

Pick smallest correct test surface:

- parser or analyze behavior -> unit tests in `test.rs` modules
- e2e compiler output differences -> keep existing compiler test, add unit tests only if they improve layer-local coverage

Never edit `case-svelte.js` or `case-rust.js`.

### Step 8: Verify

Run:

```bash
just test-case <test-name>
```

Then relevant unit test command if unit tests added, then:

```bash
just test-compiler
```

Fix breaks unrelated tests -> stop, report. No scope broadening.

### Step 9: Update Spec

Test tracked in spec:

- mark fixed for `local-fix`
- or add as unchecked use case if triage discovered larger missing slice
- update `Current state` only as terse resume header when useful. Prefer `Working`, `Tests`, `Last updated`

No dated outcome bullets in `Current state`. Durable triage outcomes go in `Use cases`.

## Summary

Report:

- classification
- root cause
- owning layer
- recommended next command

Then recommend:

- `/qa` after completed `local-fix`
- `/port` for bounded feature work
- `/audit` when spec incomplete
