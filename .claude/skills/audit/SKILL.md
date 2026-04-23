---
name: audit
description: Gap analysis for an existing feature vs reference Svelte compiler. Use when the user asks what is missing in our implementation of a feature, asks to audit a feature, or wants to check feature completeness against the reference compiler.
---

# Audit Feature

Gap analysis: compare our implementation vs reference Svelte compiler, produce spec file listing what is missing.

## Session Continuation

Run `Glob("specs/*.md")`, scan for file matching feature. Names may differ from arg — e.g. `$state` maps to `state-rune.md`. If match exists:

1. Read spec file
2. Check `Current state` — what is done, current test status
3. Skip to likely Step 4 or Step 5
4. Do not re-run Steps 1–3 unless spec says audit needs revision

## Step 1: Research

Research three things in parallel:

1. Trace feature through all three phases of reference compiler. Exhaustive enumeration: every code path = one use case.
2. Find everything related in our compiler. Run each existing test with `just test-case <name>`, determine which pass/fail.
3. Extract all syntax variants from two sources:
   - `reference/docs/`
   - reference compiler parser under `reference/compiler/phases/1-parse/`

Syntax variants output = flat list of Svelte template forms, one per line.

After research, synthesize. Read only key files identified as critical.

## Step 2: Gap Analysis

For each use case from reference compiler, classify:

- Covered
- Partial
- Missing
- Unknown

## Step 3: Write Spec File

Write spec per `spec-template` skill.

Order `Use cases` by implementation sequence, not docs order or reference discovery order. Checklist order = intended closure order for follow-up `/port` work.

Phase order:

1. `ast`
2. `scanner`
3. `parse`
4. `analyze + data structure`
5. `transform + codegen`
6. `validate / warnings`

Place each use case under earliest phase that must own first real implementation work. Multi-phase use case -> sort by first owning phase, describe downstream follow-up inline.

Step creates new spec for item already in `ROADMAP.md` -> immediately update roadmap entry with link to `specs/<name>.md`. Do not mark feature complete during audit. Only sync spec reference.

## Step 4: Add Missing Test Cases

For each `Missing` or `Unknown` use case, create test case:

- `tasks/compiler_tests/cases2/<feature>_<variant>/case.svelte`
- run `just generate` once for all new cases
- add `#[rstest]` functions in `test_v3.rs`
- run tests, report pass/fail

For each failing test:

- add `#[ignore = "missing: <description> (<layer>)"]`
- classify effort: quick fix, moderate, needs infrastructure

Rule: existing test case covers same feature and `case.svelte` under 30 lines -> extend, do not create new.

Rules:

- no compiler fixes during audit
- no edits to `case-svelte.js` or `case-rust.js`
- max 5 new test cases per run
- stuck after 3 attempts -> stop, report

## Step 5: Report

Report:

- coverage count and percentage
- passing tests
- failing tests
- recommended fix order, matching `Use cases` sequence from spec
- test results by effort
- spec file path

Recommended next commands:

- `/port specs/<name>.md` for bounded follow-up implementation
- `/fix-test <name>` for quick-fix and moderate tests
