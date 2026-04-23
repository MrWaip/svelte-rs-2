---
name: diagnose
description: Diagnose a Svelte component, playground repro, or component file by isolating the first owning failure, turning it into focused persistent tests, and recording the follow-up in the owning spec or in `specs/unknown.md`. Use when a broad repro is failing and the next step is not yet reduced to one named failing test or one approved spec slice.
---

# Diagnose

`diagnose` = write-skill.

Changes only:

- test source files
- test registration files
- generated test snapshots produced by normal test workflow
- `specs/*.md`
- `ROADMAP.md`

Must not change compiler implementation code.

## Goal

Turn one broad repro into durable, spec-owned follow-up.

Every run ends with:

- likely root cause
- first owning layer
- focused persistent tests added or changed
- spec record created, updated, or cited
- exact `/port specs/...` commands for each touched spec

## Rules

- `cclsp` first for Rust navigation.
- Keep diagnosis separate from implementation. Stop before changing compiler code.
- Test-owned artifacts change only through normal test workflow.
- Do not leave result only in `_diagnose_tmp`.
- No hand-edits to generated `case-svelte.js` or `case-rust.js`.
- No diagnostic parity against npm `svelte/compiler` here. Use `diagnose-diagnostics`.
- Prefer smallest focused persistent test over one giant repro.
- New compiler test case expected to fail until follow-up implementation -> register as `#[ignore = "diagnose: pending fix"]` so default suite stays green.
- Follow-up `/port` or fix session must remove that `#[ignore]`.
- No arbitrary spec pick when ownership ambiguous.

## Workflow

### Step 1: Reproduce The Failure

Create temp reproduction only as needed to inspect mismatch.

Run repro through current pipeline. List:

- input shape
- first observable failure
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

1. e2e compiler test registry and cases under `tasks/compiler_tests/`
2. owning crate's `tests.rs` or nearby `test.rs` modules
3. other layer-local test modules covering same syntax form, directive, rune, node family

`cclsp` first for Rust test symbols. `rg` for test names, fixture names, non-code text.

Prefer:

1. extend existing focused test
2. create new focused test
3. keep broad repro only when issue cannot yet be isolated

Default test mapping:

- parser syntax or AST shape -> parser unit tests
- analysis metadata or diagnostics -> analyzer unit tests
- observable diagnostic parity against npm `svelte/compiler` -> `tasks/diagnostic_tests/` via `diagnose-diagnostics`
- observable compiler output parity -> `tasks/compiler_tests/`

Decision rule:

- failure already isolated to one owning layer -> prefer layer unit test
- main value = observable compiler-output parity vs reference -> keep or add e2e coverage
- both useful -> add narrow unit test first, keep minimum e2e repro for durable parity tracking
- focused test cannot yet isolate safely -> keep one minimal broad repro, record repro name in durable spec entry

Need e2e compiler test:

1. add minimal `tasks/compiler_tests/cases2/<name>/case.svelte`
2. add matching ignored entry in `tasks/compiler_tests/test_v3.rs`
3. run `just generate`
4. review generated `case-svelte.js`

Register new diagnosis-owned cases:

```rust
#[rstest]
#[ignore = "diagnose: pending fix"]
fn <name>() {
    assert_compiler("<name>");
}
```

Before implementation follow-up, treat `case-svelte.js` as reference artifact. Pre-fix `case-rust.js` = evidence of failure only.

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

- `[ ] <short problem title> — layer: <parser|analysis|transform|codegen>; repro/test: <focused test name or broad repro name>; candidate specs: <a, b> or none; suggested spec: <spec name> or none`

`Test cases` bullet format:

- `[ ] <focused test name or broad repro name>`

Before new unknown item, scan `specs/unknown.md` for existing entry with same repro/test name, or same root-cause cluster.

- entry exists -> update, no duplicates
- else -> add new unchecked use case and matching `Test cases` bullet

Updating `Current state` in `specs/unknown.md`:

- `Working` = current number of unchecked unknown items
- `Tests` = passing test entries over all `Test cases` entries
- `Last updated` = today's date

#### Exactly One Matching Spec

`diagnose` must:

- add or extend narrowest correct persistent test
- add one unchecked use case if behavior not tracked
- add or update test entry in that spec
- keep `Current state` terse. Prefer only `Working`, `Tests`, `Last updated`
- update `Current state` progress summary if spec already tracks working coverage there
- re-open related `ROADMAP.md` checkbox only when spec maps to one direct roadmap item and it was marked complete while spec is not actually complete

No dated `Completed (...)`, `Confirmed gap (...)`, or history bullets in `Current state`. Durable findings go in `Use cases` or `Test cases`.

Use case already tracked -> cite exact spec item, no duplication. Still add or extend persistent test if coverage missing.

#### Multiple Plausible Specs

No arbitrary pick.

`diagnose` must:

- report candidate specs
- add one unchecked use case to `specs/unknown.md` describing failure, likely owning layer, candidate specs, focused test name or broad repro name
- add or update matching `Test cases` entry in `specs/unknown.md`
- update `specs/unknown.md` `Current state`
- keep focused failing test if isolated safely
- return `/port specs/unknown.md` as next command

#### No Matching Spec

`diagnose` must:

- add one unchecked use case to `specs/unknown.md` describing failure, likely owning layer, suggested future spec name, focused test name or broad repro name
- add or update matching `Test cases` entry in `specs/unknown.md`
- update `specs/unknown.md` `Current state`
- keep focused failing test if isolated safely
- return `/port specs/unknown.md` as next command

### Step 5: Clean Up Temporary Artifacts

Remove temp `_diagnose_tmp` cases and temp test registration after durable test or spec record exists.

## Final Report

Always report:

- root cause
- first owning layer
- tests added or changed
- specs updated
- roadmap changes, if any
- next commands

Format next commands as flat bullets with exact spec paths. Use `specs/unknown.md` when ownership ambiguous or no owning feature spec exists:

- `/port specs/foo.md`
- `/port specs/bar.md`
