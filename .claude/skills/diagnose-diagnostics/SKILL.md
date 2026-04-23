---
name: diagnose-diagnostics
description: Diagnose a diagnostic mismatch or suspected false positive/negative against npm `svelte/compiler` by isolating the first owning failure, capturing it in `tasks/diagnostic_tests`, and recording the follow-up in the owning spec or in `specs/unknown.md`. Use when the problem is diagnostic parity rather than JS/CSS output parity.
---

# Diagnose Diagnostics

`diagnose-diagnostics` = write-skill.

Changes only:

- `tasks/diagnostic_tests/`
- generated diagnostic snapshots produced by normal test workflow
- `specs/*.md`
- `ROADMAP.md`

Must not change compiler implementation code.

## Goal

Turn one broad diagnostic mismatch into durable, reference-backed follow-up.

Every run ends with:

- likely root cause
- first owning layer
- focused persistent diagnostic test added or changed
- spec record created, updated, or cited
- exact `/port specs/...` commands for each touched spec

## Rules

- `cclsp` first for Rust navigation.
- Keep diagnosis separate from implementation. Stop before changing compiler code.
- npm `svelte/compiler` = only oracle for diagnostic parity.
- No hand-edits to generated `case-svelte.json`.
- `case-rust.json` = human comparison only. Never oracle.
- Prefer smallest focused diagnostic repro over broad output fixtures.
- No diagnostic-parity cases under `tasks/compiler_tests/`.
- New diagnostic parity case expected to fail until follow-up -> register as `#[ignore = "diagnose-diagnostics: pending fix"]` so default suite stays green.
- Follow-up `/port` or fix session must remove that `#[ignore]`.
- No arbitrary spec pick when ownership ambiguous.

## Workflow

### Step 1: Reproduce The Diagnostic Mismatch

Reduce report to one minimal source demonstrating one of:

- false positive: Rust emits diagnostic that npm `svelte/compiler` does not
- false negative: npm `svelte/compiler` emits diagnostic that Rust does not
- wrong diagnostic: code or severity mismatch
- wrong location: spans do not overlap

Run repro through both sides. List:

- input shape
- compiler diagnostic from npm `svelte/compiler`
- compiler diagnostic from Rust
- later cascading noise

Classify first owning layer, in this order:

1. parser or AST
2. analysis
3. transform
4. codegen

>1 layer involved -> identify which owns first correct change.

### Step 2: Find The Smallest Persistent Test Surface

Search existing tests before new ones.

Search order:

1. `tasks/diagnostic_tests/`
2. owning crate's `tests.rs` or nearby `test.rs` modules
3. `tasks/compiler_tests/` only if problem is actually JS/CSS output parity, not diagnostics

Prefer:

1. extend existing focused diagnostic case
2. create new focused diagnostic case
3. add unit test only when mismatch already isolated to one owning layer and parity coverage adds no value

Default test mapping:

- observable diagnostic parity against npm `svelte/compiler` -> `tasks/diagnostic_tests/`
- internal analyzer/parser invariant with no parity question -> layer-local unit test
- observable JS/CSS output parity -> `tasks/compiler_tests/`

Need diagnostic parity test:

1. add minimal `tasks/diagnostic_tests/cases/<name>/case.svelte`
2. add matching ignored entry in `tasks/diagnostic_tests/test_diagnostics.rs`
3. run `just generate`
4. review generated `case-svelte.json`
5. run `just test-diagnostic-case <name>`
6. review generated `case-rust.json`

Register new diagnosis-owned cases:

```rust
#[rstest]
#[ignore = "diagnose-diagnostics: pending fix"]
fn <name>() {
    assert_diagnostics("<name>");
}
```

Before implementation follow-up, treat `case-svelte.json` as reference artifact.

### Step 3: Map The Failure To A Planning Owner

Identify feature cluster used by repro.

Search matching feature spec under `specs/*.md`, excluding `specs/unknown.md` until fallback needed.

Exactly one matching spec:

1. read `Current state`
2. read `Use cases`
3. read `Test cases` if present
4. decide if use case already tracked

Multiple plausible specs -> ownership ambiguous.

No matching spec -> fall back to `specs/unknown.md`.

### Step 4: Record The Durable Follow-Up

Every diagnosis leaves one durable repo trace.

Writing to `specs/unknown.md` -> one flat unchecked use case bullet and one flat `Test cases` bullet.

Use case bullet format:

- `[ ] <short diagnostic problem title> — layer: <parser|analysis|transform|codegen>; repro/test: <diagnostic case name>; candidate specs: <a, b> or none; suggested spec: <spec name> or none`

`Test cases` bullet format:

- `[ ] <diagnostic case name>`

Before new unknown item, scan `specs/unknown.md` for existing entry with same diagnostic case name or root-cause cluster.

- entry exists -> update, no duplicates
- else -> add new unchecked use case and matching `Test cases` bullet

Updating `Current state` in `specs/unknown.md`:

- `Working` = current number of unchecked unknown items
- `Tests` = passing test entries over all `Test cases` entries
- `Last updated` = today's date

#### Exactly One Matching Spec

`diagnose-diagnostics` must:

- add or extend narrowest correct diagnostic parity test
- add one unchecked use case if behavior not tracked
- add or update test entry in that spec
- keep `Current state` terse. Prefer only `Working`, `Tests`, `Last updated`
- update `Current state` progress summary if spec already tracks working coverage there
- re-open related `ROADMAP.md` checkbox only when spec maps to one direct roadmap item and it was marked complete while spec is not actually complete

No dated `Completed (...)`, `Confirmed gap (...)`, or history bullets in `Current state`. Durable findings go in `Use cases` or `Test cases`.

Use case already tracked -> cite exact spec item, no duplication. Still add or extend persistent test if parity coverage missing.

#### Multiple Plausible Specs

No arbitrary pick.

`diagnose-diagnostics` must:

- report candidate specs
- add one unchecked use case to `specs/unknown.md` describing mismatch, likely owning layer, candidate specs, diagnostic case name
- add or update matching `Test cases` entry in `specs/unknown.md`
- update `specs/unknown.md` `Current state`
- keep focused diagnostic case if isolated safely
- return `/port specs/unknown.md` as next command

#### No Matching Spec

`diagnose-diagnostics` must:

- add one unchecked use case to `specs/unknown.md` describing mismatch, likely owning layer, suggested future spec name, diagnostic case name
- add or update matching `Test cases` entry in `specs/unknown.md`
- update `specs/unknown.md` `Current state`
- keep focused diagnostic case if isolated safely
- return `/port specs/unknown.md` as next command

### Step 5: Clean Up Temporary Artifacts

Remove temp `_diagnose_tmp` cases and temp test registration after durable diagnostic case or spec record exists.

## Final Report

Always report:

- root cause
- first owning layer
- diagnostic tests added or changed
- specs updated
- roadmap changes, if any
- next commands

Format next commands as flat bullets with exact spec paths:

- `/port specs/foo.md`
- `/port specs/bar.md`
