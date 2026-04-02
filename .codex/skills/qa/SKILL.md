---
name: qa
description: Review recent code changes against project guidelines. Use after fix-test, improve, port, or any implementation work when Codex needs to review recent changes against `CLAUDE.md` and report violations or a pass verdict.
---

# Check Quality

Review recent code changes against project guidelines from `CLAUDE.md`.

This command is read-only. It finds violations and produces a fix plan, but does not apply fixes.

## Step 1: Collect Changes

Determine the review scope from the input:

- commit hash: review `git diff <hash>..HEAD`
- range such as `HEAD~N`: review `git diff <range>..HEAD`
- directory or file path: review all code in that path, not just a diff
- no input: auto-detect in order
  1. uncommitted changes
  2. last commit

Read every changed file in full to understand the context.

## Step 2: Review

Check all project rules against the collected scope, including architecture boundaries, visitor usage, naming, Rust idioms, edge cases, test hygiene, and generated files.

## Step 3: Verdict

Output in this exact format:

```text
STATUS: PASS | FAIL

VIOLATIONS:
1. [file:line] — [rule number + name] — [what is wrong and why]
2. ...

FIX PLAN:
1. [file:line] — [exact change to make]
2. ...
```

Rules:

- if any violation exists, status is `FAIL`
- if zero violations exist, status is `PASS` and skip `VIOLATIONS` and `FIX PLAN`
- no warnings-only mode

If status is `PASS`, stop here.

If status is `FAIL`, proceed to Step 4.

## Step 4: Fix

Treat the fix plan as a strict contract. For each item:

1. Apply the fix
2. Mark the item as done

After all items are done, do not re-review in the same run. A fresh `qa` run is the confirmation step.

Only fix reported violations. Do not add unrelated improvements or refactors.

## Step 5: Test Coverage Recommendation

After fixing violations, check whether any fix changed logic rather than style or naming. If so, output a recommendation block listing the affected changes and recommending additional tests.

Skip this step if all fixes were stylistic.

## Rules

- check every rule against every changed file
- read actual code, not just diff hunks
- if unsure whether something is a violation, report it
- do not declare `PASS` yourself after fixing; only a fresh `qa` run can do that
