---
name: diagnose-diagnostics
description: Diagnose a diagnostic mismatch or suspected false positive/negative against npm `svelte/compiler` by isolating the first owning failure, capturing it in `tasks/diagnostic_tests`, and recording the follow-up in the owning spec or in `specs/unknown.md`. Use when the problem is diagnostic parity rather than JS/CSS output parity.
---

# Diagnose Diagnostics

`diagnose-diagnostics` is a write-skill.

It may change only:

- `tasks/diagnostic_tests/`
- generated diagnostic snapshots produced by the normal test workflow
- `specs/*.md`
- `ROADMAP.md`

It must not change compiler implementation code.

## Goal

Turn one broad diagnostic mismatch into a durable, reference-backed follow-up.

Every run must end with:

- likely root cause
- first owning layer
- focused persistent diagnostic test added or changed
- spec record created, updated, or cited
- exact `$port specs/...` commands for each touched spec

## Rules

- Use `cclsp` first when navigating Rust code.
- Keep diagnosis separate from implementation. Stop before changing compiler code.
- Use npm `svelte/compiler` as the only oracle for diagnostic parity.
- Do not hand-edit generated `case-svelte.json`.
- `case-rust.json` is for human comparison only; never treat it as the oracle.
- Prefer the smallest focused diagnostic repro over broad output fixtures.
- Do not add diagnostic-parity cases under `tasks/compiler_tests/`.
- Do not choose a spec arbitrarily when ownership is ambiguous.

## Workflow

### Step 1: Reproduce The Diagnostic Mismatch

Reduce the report to one minimal source that demonstrates one of:

- false positive: Rust emits a diagnostic that npm `svelte/compiler` does not
- false negative: npm `svelte/compiler` emits a diagnostic that Rust does not
- wrong diagnostic: code or severity mismatch
- wrong location: spans do not overlap

Run the repro through both sides and list:

- input shape
- compiler diagnostic from npm `svelte/compiler`
- compiler diagnostic from Rust
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

1. `tasks/diagnostic_tests/`
2. the owning crate's `tests.rs` or nearby `test.rs` modules
3. `tasks/compiler_tests/` only if the problem is actually JS/CSS output parity rather than diagnostics

Prefer this order:

1. extend an existing focused diagnostic case
2. create a new focused diagnostic case
3. add a unit test only when the mismatch is already isolated to one owning layer and parity coverage would add no value

Default test mapping:

- observable diagnostic parity against npm `svelte/compiler` -> `tasks/diagnostic_tests/`
- internal analyzer or parser invariant with no parity question -> layer-local unit test
- observable JS/CSS output parity -> `tasks/compiler_tests/`

If a diagnostic parity test is needed:

1. add minimal `tasks/diagnostic_tests/cases/<name>/case.svelte`
2. add the matching entry in `tasks/diagnostic_tests/test_diagnostics.rs`
3. run `just generate`
4. review generated `case-svelte.json`
5. run `just test-diagnostic-case <name>`
6. review generated `case-rust.json`

Before implementation follow-up, treat `case-svelte.json` as the reference artifact.

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

- `[ ] <short diagnostic problem title> — layer: <parser|analysis|transform|codegen>; repro/test: <diagnostic case name>; candidate specs: <a, b> or none; suggested spec: <spec name> or none`

Use this format for the `Test cases` bullet:

- `[ ] <diagnostic case name>`

Before adding a new unknown item, scan `specs/unknown.md` for an existing entry with the same diagnostic case name or root-cause cluster.

- if such an entry exists, update it instead of creating a duplicate
- otherwise add a new unchecked use case and matching `Test cases` bullet

When updating `Current state` in `specs/unknown.md`:

- set `Working` to the current number of unchecked unknown items
- set `Next` to the first unchecked unknown item in file order
- update `Last updated` to today's date

#### When Exactly One Matching Spec Exists

`diagnose-diagnostics` must:

- add or extend the narrowest correct diagnostic parity test
- add one unchecked use case if the behavior is not already tracked
- add or update the test entry in that spec
- update `Current state` with a short dated note when that helps the next session resume
- update the `Current state` progress summary if the spec already tracks working coverage there
- re-open the related `ROADMAP.md` checkbox only when that spec maps to one direct roadmap item and it was marked complete even though the spec is not actually complete

If the use case is already tracked, cite the exact spec item instead of duplicating it. Still add or extend the persistent test if parity coverage is missing.

#### When Multiple Plausible Specs Exist

Do not choose arbitrarily.

`diagnose-diagnostics` must:

- report the candidate specs
- add one unchecked use case to `specs/unknown.md` describing the mismatch, likely owning layer, candidate specs, and the diagnostic case name
- add or update the matching `Test cases` entry in `specs/unknown.md`
- update `specs/unknown.md` `Current state`
- keep the focused diagnostic case if it was isolated safely
- return `$port specs/unknown.md` as the next command

#### When No Matching Spec Exists

`diagnose-diagnostics` must:

- add one unchecked use case to `specs/unknown.md` describing the mismatch, likely owning layer, suggested future spec name, and the diagnostic case name
- add or update the matching `Test cases` entry in `specs/unknown.md`
- update `specs/unknown.md` `Current state`
- keep the focused diagnostic case if it was isolated safely
- return `$port specs/unknown.md` as the next command

### Step 5: Clean Up Temporary Artifacts

Remove temporary `_diagnose_tmp` cases and temporary test registration after the durable diagnostic case or spec record exists.

## Final Report

Always report:

- root cause
- first owning layer
- diagnostic tests added or changed
- specs updated
- roadmap changes, if any
- next commands

Format next commands as flat bullets with exact spec paths:

- `$port specs/foo.md`
- `$port specs/bar.md`
