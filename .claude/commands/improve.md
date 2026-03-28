---
description: Fix existing codebase problems — bugs, workarounds, ad hoc solutions, architectural issues, missing unit tests. Use when the user asks to "fix this hack", "refactor", "clean up", "add tests for", or points to a specific code quality issue.
argument-hint: "[description-or-file-path]"
---

# Fix debt: $ARGUMENTS

Fix an existing codebase problem: bug, workaround, ad hoc solution, architectural issue, or missing test coverage.

## Step 1: Understand the problem

If `$ARGUMENTS` is a file path — read the file, find the issue.
If `$ARGUMENTS` is a description — search the codebase for the relevant code.

Classify the problem:
- **Bug** — produces wrong output
- **Workaround / ad hoc** — works but violates architecture (wrong layer, string hacks, manual matching)
- **Missing tests** — code exists but has no unit tests
- **Architecture** — structural issue (wrong abstraction, scattered ownership, implicit coupling)

Check `specs/*.md` for a spec covering this area. If found, update its Use cases / Current state after the fix.

## Step 2: Assess scope

Before writing any code, answer:
1. **Which layers are affected?** (parser / analyze / transform / codegen)
2. **What's the correct fix?** Not the quick fix — the architecturally right one.
3. **Does the fix change JS output?** If yes, existing tests will catch regressions. If no, this is a pure refactor.
4. **How many files change?** If > 5 files — break into steps, one commit per logical change.

State the layer and approach. Wait for approval if the scope is large (> 5 files).

## Step 3: Add tests first (if missing)

If the problem area lacks unit tests:
- Add tests that capture the **current** behavior (even if wrong)
- For parser/analyze: follow `/test-pattern`
- For codegen: use existing compiler test cases or add new ones
- Rule: extend existing test if same feature AND `case.svelte` < 30 lines

This ensures the fix doesn't break unrelated behavior.

## Step 4: Fix

Apply the fix in the correct layer.

For boundary violations (code in wrong layer):
1. Add the correct implementation in the right layer
2. Update consumers to use the new path
3. Delete the old implementation
4. Verify JS output unchanged

For missing tests only (no code fix needed):
- Add tests, verify they pass, done.

## Step 5: Verify

```
just test-all
```

If any test fails that wasn't failing before — the fix introduced a regression. Investigate and fix.

## Step 6: Report

```
## Fixed: [brief description]

### Problem
[what was wrong and where]

### Fix
[what was changed and why]

### Tests
- Added: [new tests]
- Modified: [changed tests]
- All passing: yes/no

### Next
→ `/qa` to verify no new boundary violations
```

## Rules

- Fix ONE problem per invocation. Don't scope-creep into adjacent issues.
- If the fix requires changes in another layer — do it. Don't leave TODOs.
- If fixing reveals more problems — report them, don't fix them in the same run.
- Pure refactors MUST NOT change JS output. Verify with `just test-compiler`.
- If stuck after 3 attempts, follow the "When blocked" process from CLAUDE.md.
