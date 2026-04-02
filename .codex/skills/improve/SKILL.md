---
name: improve
description: Fix existing codebase problems such as bugs, workarounds, ad hoc solutions, architectural issues, or missing test coverage. Use when the user asks to fix a hack, refactor, clean up, add tests for an area, or points to a specific code quality issue.
---

# Improve

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

Fix an existing codebase problem: bug, workaround, ad hoc solution, architectural issue, or missing test coverage.

## Step 1: Understand The Problem

If the input is a file path, read the file and find the issue.
If the input is a description, search the codebase for the relevant code.

Classify the problem:

- bug
- workaround or ad hoc
- missing tests
- architecture

Check `specs/*.md` for a spec covering this area. If found, update its use cases or `Current state` after the fix.

## Step 2: Assess Scope

Before writing any code, answer:

1. Which layers are affected
2. What the correct fix is
3. Whether the fix changes JS output
4. How many files change

If more than 5 files need to change, break the work into steps with one logical change per step.

The plan text must include: **"Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries."**

State the layer and approach. Wait for approval if the scope is large, meaning more than 5 files.

## Step 3: Add Tests First

If the problem area lacks unit tests:

- add tests that capture the current behavior, even if wrong
- for parser or analyze, follow `test-pattern`
- for codegen, use existing compiler test cases or add new ones

Rule: extend an existing compiler test if it covers the same feature and `case.svelte` is under 30 lines.

## Step 4: Fix

Apply the fix in the correct layer.

For boundary violations:

1. Add the correct implementation in the right layer
2. Update consumers to use the new path
3. Delete the old implementation
4. Verify JS output is unchanged

For missing tests only:

- add tests
- verify they pass
- stop

## Step 5: Verify

Run:

```bash
just test-all
```

If any test fails that was not failing before, the fix introduced a regression. Investigate and fix it.

Pure refactors must not change JS output. Verify with `just test-compiler`.

## Step 6: Report

Report:

- problem
- fix
- tests added or modified
- whether all tests are passing
- next recommended command, usually `/qa`

## Rules

- fix one problem per invocation
- if the fix requires changes in another layer, do it
- if the fix reveals more problems, report them instead of fixing them in the same run
- if stuck after 3 attempts, follow the blocked workflow from `CLAUDE.md`
