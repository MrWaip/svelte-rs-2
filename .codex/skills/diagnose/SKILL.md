---
name: diagnose
description: Diagnose a Svelte component, playground repro, or component file by isolating the first owning failure, turning it into focused persistent tests, and recording the follow-up in the owning spec or in `specs/unknown.md`. Use when a broad repro is failing and the next step is not yet reduced to one named failing test or one approved spec slice.
---

# Diagnose

`diagnose` is a write-skill.

It may change only:

- test source files
- test registration files
- generated test snapshots produced by the normal test workflow
- `specs/*.md`
- `ROADMAP.md`

It must not change compiler implementation code.

## Goal

Turn one broad repro into a durable, spec-owned follow-up.

Every run must end with:

- likely root cause
- first owning layer
- focused persistent tests added or changed
- spec record created, updated, or cited
- exact `$port specs/...` commands for each touched spec

## Rules

- Use `cclsp` first when navigating Rust code.
- Keep diagnosis separate from implementation. Stop before changing compiler code.
- Test-owned artifacts may change when required by the normal test workflow.
- Do not leave the result only in `_diagnose_tmp`.
- Do not hand-edit generated `case-svelte.js` or `case-rust.js`.
- Do not use this skill for diagnostic parity against npm `svelte/compiler`; use `diagnose-diagnostics` instead.
- Prefer the smallest focused persistent test over keeping one giant repro.
- Do not choose a spec arbitrarily when ownership is ambiguous.

## Workflow

### Step 1: Reproduce The Failure

Create a temporary reproduction only as needed to inspect the mismatch.

Run the repro through the current pipeline and list:

- the input shape
- the first observable failure
- any later cascading noise

Classify the first owning layer in this order:

1. parser or AST
2. analysis
3. transform
4. codegen

If more than one layer looks involved, identify which layer should own the first correct change.

### Step 2: Find The Smallest Persistent Test Surface

Search existing tests before creating new ones.

Search in this order:

1. the existing e2e compiler test registry and cases under `tasks/compiler_tests/`
2. the owning crate's `tests.rs` or nearby `test.rs` modules
3. other layer-local test modules that already cover the same syntax form, directive, rune, or node family

Use `cclsp` first for Rust test symbols and `rg` for test names, fixture names, and non-code text search.

Prefer this order:

1. extend an existing focused test
2. create a new focused test
3. keep a broad repro only when the issue cannot yet be isolated further

Default test mapping:

- parser syntax or AST shape -> parser unit tests
- analysis metadata or diagnostics -> analyzer unit tests
- observable diagnostic parity against npm `svelte/compiler` -> `tasks/diagnostic_tests/` via `diagnose-diagnostics`
- observable compiler output parity -> `tasks/compiler_tests/`

Decision rule:

- if the failure is already isolated to one owning layer, prefer that layer's unit test
- if the main value is observable compiler-output parity against the reference compiler, keep or add e2e coverage
- if both are useful, add the narrow unit test first and keep the minimum e2e repro needed for durable parity tracking
- if a focused test cannot yet be isolated safely, keep one minimal broad repro and record that repro name in the durable spec entry

If an e2e compiler test is needed:

1. add minimal `tasks/compiler_tests/cases2/<name>/case.svelte`
2. add the matching entry in `tasks/compiler_tests/test_v3.rs`
3. run `just generate`
4. review generated `case-svelte.js`

Before implementation follow-up, treat `case-svelte.js` as the reference artifact. Do not rely on pre-fix `case-rust.js` as anything more than evidence of failure.

### Step 3: Map The Failure To A Planning Owner

Identify the feature cluster used by the repro.

Search for the matching feature spec under `specs/*.md`, excluding `specs/unknown.md` until fallback is needed.

If exactly one matching spec exists:

1. read `Current state`
2. read `Use cases`
3. read `Test cases` if present
4. decide whether the use case is already tracked

If multiple plausible specs exist, treat ownership as ambiguous.

If no matching spec exists, fall back to `specs/unknown.md`.

### Step 4: Record The Durable Follow-Up

Every diagnosis must leave one durable repo trace.

When writing to `specs/unknown.md`, use one flat unchecked use case bullet and one flat `Test cases` bullet.

Use this format for the use case bullet:

- `[ ] <short problem title> — layer: <parser|analysis|transform|codegen>; repro/test: <focused test name or broad repro name>; candidate specs: <a, b> or none; suggested spec: <spec name> or none`

Use this format for the `Test cases` bullet:

- `[ ] <focused test name or broad repro name>`

Before adding a new unknown item, scan `specs/unknown.md` for an existing entry with the same repro or test name, or the same root-cause cluster.

- if such an entry exists, update it instead of creating a duplicate
- otherwise add a new unchecked use case and matching `Test cases` bullet

When updating `Current state` in `specs/unknown.md`:

- set `Working` to the current number of unchecked unknown items
- set `Next` to the first unchecked unknown item in file order
- update `Last updated` to today's date

#### When Exactly One Matching Spec Exists

`diagnose` must:

- add or extend the narrowest correct persistent test
- add one unchecked use case if the behavior is not already tracked
- add or update the test entry in that spec
- update `Current state` with a short dated note when that helps the next session resume
- update the `Current state` progress summary if the spec already tracks working coverage there
- re-open the related `ROADMAP.md` checkbox only when that spec maps to one direct roadmap item and it was marked complete even though the spec is not actually complete

If the use case is already tracked, cite the exact spec item instead of duplicating it. Still add or extend the persistent test if coverage is missing.

#### When Multiple Plausible Specs Exist

Do not choose arbitrarily.

`diagnose` must:

- report the candidate specs
- add one unchecked use case to `specs/unknown.md` describing the failure, likely owning layer, candidate specs, and the focused test name or broad repro name
- add or update the matching `Test cases` entry in `specs/unknown.md` using the focused test name or broad repro name
- update `specs/unknown.md` `Current state` to reflect the recorded unknown item
- keep the focused failing test if it was isolated safely
- return `$port specs/unknown.md` as the next command

#### When No Matching Spec Exists

`diagnose` must:

- add one unchecked use case to `specs/unknown.md` describing the failure, likely owning layer, suggested future spec name, and the focused test name or broad repro name
- add or update the matching `Test cases` entry in `specs/unknown.md` using the focused test name or broad repro name
- update `specs/unknown.md` `Current state` to reflect the recorded unknown item
- keep the focused failing test if it was isolated safely
- return `$port specs/unknown.md` as the next command

### Step 5: Clean Up Temporary Artifacts

Remove temporary `_diagnose_tmp` cases and temporary test registration after the durable test or spec record exists.

## Final Report

Always report:

- root cause
- first owning layer
- tests added or changed
- specs updated
- roadmap changes, if any
- next commands

Format next commands as flat bullets with exact spec paths. Use `specs/unknown.md` when ownership is ambiguous or no owning feature spec exists yet:

- `$port specs/foo.md`
- `$port specs/bar.md`
