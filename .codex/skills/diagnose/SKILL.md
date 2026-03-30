---
name: diagnose
description: Diagnose a Svelte component or component file by compiling it through the current pipeline, identifying mismatches by layer, and turning broad breakage into focused tests. Use when the user provides component source or a file path and asks what is broken. Do not trigger when one named failing test already isolates the issue.
---

# Diagnose Component

## 1) Create a temporary reproduction

Create a temporary compiler case such as `_diagnose_tmp` from the provided component source or file path.

Generate expected output and register a temporary test.

## 2) Run and compare

Run the temporary case verbosely, then compare:

- input `case.svelte`
- expected `case-svelte.js`
- actual `case-rust.js`

## 3) Classify mismatches by layer

Use:

1. parser or AST
2. analyze
3. transform
4. codegen

## 4) Build a fix plan

Report:

- features used by the component
- each mismatch
- likely root cause
- fix complexity
- dependency-ordered fix order

## 5) Convert findings into focused tests

Turn the most important issues into narrow parser, analyzer, or compiler tests. Avoid a single giant reproduction test when smaller targeted tests would isolate behavior better.

## 6) Clean up

Remove the temporary `_diagnose_tmp` case and temporary test registration after harvesting the information you need.

Rules:
- never hand-edit generated `case-svelte.js` / `case-rust.js`
- keep diagnosis and implementation separated unless user explicitly asks to fix now
