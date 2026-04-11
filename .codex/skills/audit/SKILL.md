---
name: audit
description: Gap analysis for an existing feature vs reference Svelte compiler. Use when the user asks what is missing in our implementation of a feature, asks to audit a feature, or wants to check feature completeness against the reference compiler.
---

# Audit Feature

Gap analysis for an existing feature: compare our implementation against the reference Svelte compiler and produce a spec file with what is missing.

## Session Continuation

Run `Glob("specs/*.md")` and scan the results for a file matching this feature. Names may differ from the argument — for example `$state` may map to `state-rune.md`. If a matching spec exists:

1. Read the spec file
2. Check the `Current state` section — what is done, what is next
3. Skip to the appropriate step, likely Step 4 or Step 5
4. Do not re-run Steps 1–3 unless the spec says the audit needs revision

## Step 1: Research

Research three things in parallel:

1. Trace the feature through all three phases of the reference compiler. Focus on exhaustive enumeration: every code path equals one use case.
2. Find everything related to the feature in our compiler. Run each existing test with `just test-case <name>` and determine which pass and which fail.
3. Extract all syntax variants of the feature from two sources:
   - `reference/docs/`
   - reference compiler parser under `reference/compiler/phases/1-parse/`

The syntax variants output should be a flat list of Svelte template forms, one per line.

After research completes, synthesize findings. Read only the key files identified as critical.

## Step 2: Gap Analysis

For each use case from the reference compiler, classify it as:

- Covered
- Partial
- Missing
- Unknown

## Step 3: Write Spec File

Write the spec following the `spec-template` skill.

If this step creates a new spec for an item that already exists in `ROADMAP.md`, immediately update that roadmap entry to include a link to `specs/<name>.md`. Do not mark the feature complete during audit; only sync the spec reference.

## Step 4: Add Missing Test Cases

For each `Missing` or `Unknown` use case, create a test case:

- `tasks/compiler_tests/cases2/<feature>_<variant>/case.svelte`
- run `just generate` once for all new cases
- add `#[rstest]` functions in `test_v3.rs`
- run tests and report which pass and which fail

For each test that fails:

- add `#[ignore = "missing: <description> (<layer>)"]`
- classify effort as quick fix, moderate, or needs infrastructure

Rule: if an existing test case covers the same feature and `case.svelte` is under 30 lines, extend it instead of creating a new one.

Rules:

- do not fix the compiler during audit
- do not edit `case-svelte.js` or `case-rust.js`
- max 5 new test cases per run
- if stuck after 3 attempts, stop and report

## Step 5: Report

Report:

- coverage count and percentage
- passing tests
- failing tests
- recommended fix order
- test results by effort
- spec file path

Recommended next commands:

- `/port specs/<name>.md` for bounded follow-up implementation
- `/fix-test <name>` for quick-fix and moderate tests
