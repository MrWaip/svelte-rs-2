---
name: fix-test
description: Fix a single failing compiler test case. Use when the user asks to fix one test, make one test pass, or provides a failing compiler test name.
---

# Fix Failing Test

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

Fix a single compiler test case. The test name is provided as input.

## Approach

When reading Svelte reference visitors to understand the fix, focus on what output they produce, not how they are structured. Do not port visitor patterns, mutable metadata, or JS workarounds. Use our existing architecture: direct recursion, `AnalysisData` side tables, and normal Rust control flow.

## PLAN PHASE

These steps are read-only. Complete them before writing any code.

### Step 1: Understand The Failure

If `/explain-test` was already run for this test in the current session, use its findings and skip Step 1 and Step 2.

Run the test to see the diff:

```bash
just test-case-verbose <test-name>
```

Read the three files in `tasks/compiler_tests/cases2/<test-name>/`:

- `case.svelte`
- `case-svelte.js`
- `case-rust.js`

Compare `case-rust.js` with `case-svelte.js`. List every mismatch.

### Step 2: Diagnose The Root Cause

Determine which layer has the issue, in this order:

1. parser or AST
2. analysis
3. codegen

Use `CLAUDE.md` to navigate to the right files. Read `CODEBASE_MAP.md` for type signatures or module structure when needed.

### Step 3: Research

Research two things in parallel:

1. How the reference compiler handles this specific case. Focus on the code path that produces the observed diff.
2. Where the corresponding code lives in our compiler: which function handles this node type, what analysis data is available, and what is missing.

After research completes, identify the gap: what the reference does that we do not.

### Step 4: Plan The Fix

Produce a plan with all of these sections:

1. Layer
2. Root cause
3. Changes
4. Unit tests

The plan text must include: **"Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries."**

**Present the plan and wait for approval before proceeding.**

## EXECUTE PHASE

Start here only after plan approval.

### Step 5: Fix

Implement the planned fix. Do not fix multiple test cases at once.

Never edit `case-svelte.js` or `case-rust.js`.

### Step 6: Unit Tests

If the fix touches parser or analyze logic, add a unit test covering the specific behavior following project test patterns.

### Step 7: Verify

Run the single test:

```bash
just test-case <test-name>
```

Then run all compiler tests:

```bash
just test-compiler
```

Do not consider the task complete until `just test-compiler` has been run and its result has been reported to the user explicitly.

If the fix breaks other tests, stop and report the regression count or failing cases you observed. Do not fix other tests in the same run.

If the test still fails after 3 fix attempts, stop and report what you tried.

### Step 8: Update Spec

If this test is tracked in a spec file under `specs/*.md`, mark it as fixed and update the `Current state` section.

Recommended next command:

- `/qa`
