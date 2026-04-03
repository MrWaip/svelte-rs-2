---
name: diagnose2
description: Diagnose a Svelte component or component file by compiling it through the current pipeline, mapping the mismatch to an existing spec when possible, and recommending the correct next workflow. Use when a playground repro or component source shows compiler output differences and the issue is not yet isolated to one named failing test.
---

# Diagnose Component Through Spec

**Diagnosis must stay separate from implementation unless the user explicitly asks for a fix.**

Use this skill when a playground repro, component snippet, or component file shows a mismatch between the reference compiler output and the Rust compiler output.

## Goal

Turn one broad reproduction into a spec-owned next action.

The output must classify the reproduction as one of:

- `local-bug` — existing implementation is wrong in one bounded path
- `known-spec-gap` — the mismatch belongs to an existing spec and should become the next slice there
- `new-use-case` — the mismatch belongs to an existing spec but is not yet tracked in its use cases
- `needs-audit` — no suitable spec exists or the feature map is too incomplete

## Approach

Do not:

- fix the compiler during diagnosis unless the user explicitly asks for that follow-up
- leave findings unattached to a spec when a relevant spec exists
- hand-edit generated `case-svelte.js` or `case-rust.js`
- turn every broad repro into permanent tests before deciding where the work belongs

## PLAN PHASE

These steps are read-only except for creating temporary reproduction artifacts.

### Step 1: Create A Temporary Reproduction

Create a temporary compiler case such as `_diagnose_tmp` from the provided component source or file path.

Generate expected output and register a temporary test only as needed to inspect the mismatch.

### Step 2: Run And Compare

Run the temporary case verbosely, then compare:

- input `case.svelte`
- expected `case-svelte.js`
- actual `case-rust.js`

List the observable mismatches. Keep the list concrete and output-oriented.

### Step 3: Classify By Layer

Determine the most likely owning layer in this order:

1. parser or AST
2. analysis
3. transform
4. codegen

If more than one layer is involved, identify which layer should own the first correct change.

### Step 4: Map To Spec

Identify the feature or feature cluster used by the reproduction.

Search for a matching spec under `specs/*.md`.

If a matching spec exists:

1. read `Current state`
2. read `Use cases`
3. read `Tasks`
4. decide whether the mismatch is already represented there

If no matching spec exists, classify as `needs-audit` unless the failure is clearly a bounded local bug in an already implemented path.

### Step 5: Produce A Diagnosis

Produce a diagnosis with these sections:

1. Features used by the reproduction
2. Observable mismatches
3. Likely owning layer
4. Classification
5. Spec mapping
6. Recommended next command

Classification rules:

#### `local-bug`

Use this only when:

- the behavior already belongs to an implemented path
- the mismatch looks like one bounded bug, not a missing slice

Recommended next command:

- `/triage-test <name>` if a stable failing compiler test should be kept
- `/fix-test <name>` only if the case is already clearly isolated as a local fix

#### `known-spec-gap`

Use this when:

- the mismatch belongs to an existing spec
- the use case is already represented there, or clearly belongs to the next bounded slice

Recommended next command:

- `/port2 <spec-path-or-feature>`

#### `new-use-case`

Use this when:

- the mismatch belongs to an existing spec
- but the specific use case is not yet tracked there

Recommended next action:

- update the existing spec with the new unchecked use case
- then recommend `/port2 <spec-path-or-feature>`

#### `needs-audit`

Use this when:

- no matching spec exists
- or the reproduction reveals a feature area that is still too under-specified

Recommended next command:

- `/audit <feature>`

### Step 6: Convert To Focused Follow-Up

Only after classification, decide whether the broad reproduction should produce persistent tests.

Rules:

- for `local-bug`, keep or create the smallest stable failing test that isolates the bug
- for `known-spec-gap` or `new-use-case`, add permanent tests only in the follow-up `port2` or spec-owned workflow
- for `needs-audit`, prefer updating the spec and audit plan before creating multiple permanent tests

Do not turn the broad reproduction into a grab-bag of unrelated permanent tests.

### Step 7: Clean Up

Remove the temporary `_diagnose_tmp` case and temporary test registration after harvesting the information you need.

## Summary

Report:

- classification
- likely owning layer
- matching spec or missing spec
- recommended next command

Then recommend exactly one of:

- `/triage-test`
- `/fix-test`
- `/port2`
- `/audit`
