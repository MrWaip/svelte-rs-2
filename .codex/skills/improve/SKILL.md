---
name: improve
description: Fix an existing codebase problem such as a bug, workaround, hack, missing tests, or architectural issue. Use when the user asks to clean up a path, remove an ad hoc solution, add missing tests around existing behavior, or improve a specific file or subsystem without starting from a named failing compiler case.
---

# Improve Existing Code

## 1) Understand the problem

If the input is a file path, read the file and locate the issue. If it is a description, search the repo for the relevant code.

Classify the problem:

- bug
- workaround or ad hoc solution
- missing tests
- architecture issue

Check `specs/*.md` for an existing spec that covers the same area and update it if the work belongs there.

## 2) Assess scope before editing

Answer:

1. which layers are affected
2. what the correct architectural fix is
3. whether JS output should change
4. how many files must change

If the fix is large, break it into bounded steps instead of mixing multiple problems together.

## 3) Add tests first when coverage is missing

- parser and analyze: follow `test-pattern`
- codegen: use existing compiler tests or add a focused new one with `add-test`

Capture current behavior before changing it when that helps prevent accidental regressions.

## 4) Fix the problem in the correct layer

For boundary violations:

1. add the correct implementation upstream
2. update consumers
3. delete the workaround
4. verify behavior or output

For test-only work, stop after the new tests pass.

## 5) Verify

Run the narrowest meaningful validation first, then widen as needed. Use `just test-compiler` or `just test-all` when the change crosses multiple layers.

If a pure refactor changes generated JS, treat that as a bug and investigate.

## 6) Report

Include:

- what was wrong
- what changed and why
- tests added or updated
- whether follow-up `qa` is recommended
