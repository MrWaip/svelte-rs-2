---
name: diagnose
description: Diagnose a Svelte component, playground repro, or component file by isolating the first owning failure, turning it into focused persistent tests, and recording the follow-up in the owning spec or in `specs/unknown.md`. Use when a broad repro is failing and the next step is not yet reduced to one named failing test or one approved spec slice.
disable-model-invocation: true
---

# Diagnose

Write-skill. Allowed changes: tests, test registration, generated snapshots (via normal flow), `specs/*.md`, `ROADMAP.md`, `debt.md`. Never edits compiler code.

Goal: turn one broad repro into a durable, spec-owned follow-up.

## Prerequisites

Read `ARCHITECTURE.md` and `CONTEXT.md` if not already in session. Layer choice, test placement, and spec wording must match their invariants and `CONTEXT.md ## Language` (banlist in `CLAUDE.md ## Never use`).

## Rules

- narsil-mcp first for Rust navigation. `rg`/`find` only for raw text, fixtures, non-Rust.
- Stop before changing compiler code.
- No hand-edits to generated `case-*.js` / `case-*.json`.
- Diagnostic parity vs npm `svelte/compiler` -> `diagnose-diagnostics`, not here.
- New failing case registered as `#[ignore = "diagnose: pending fix"]`. Removed by follow-up `/port` or fix.
- Ambiguous ownership -> `specs/unknown.md`, never an arbitrary pick.
- Empty-hands forbidden: a run without an added/changed test and a spec entry = failure. If repro does not reproduce, write a `specs/unknown.md` entry recording what was tried (input, mode, generate, flags).
- Repro sourced from real user code MUST be rewritten as a synthetic minimal case before landing. Original snippet never committed.
- `quick-check` is a probe inside Step 1, not a substitute. Pass `--mode` / `--generate` / `--dev` / `--filename` through when the input names them.

## Step 1 — Reproduce

Run the repro through the current pipeline. Note input shape, first observable failure, cascading noise.

Owning layer (per `ARCHITECTURE.md`):

1. parser (template / `parse_js` / CSS)
2. analyze — one of:
   - 3.A.1 `ComponentSemantics`
   - 3.A.2 `ReactivitySemantics`
   - 3.A.3 `ExpressionSemantics`
   - 3.A.4 `AttributeSemantics`
   - 3.A.5 `BlockSemantics`
   - 3.B side-tables (`ScriptAnalysis` / `ElementAnalysis` / `TemplateAnalysis` / `BlockAnalysis` / `OutputPlanData` / `DynamismData` / `PickledAwaits`) — new facts MUST NOT land here
   - 3.C `validate`
3. transform
4. codegen
5. css transform (`svelte_transform_css`)

Multiple layers involved -> name the one that owns the first correct change.

## Step 2 — Find the smallest test surface

Search existing tests first:

1. e2e cases under `tasks/compiler_tests/`
2. owning crate's `test.rs` modules
3. layer-local tests for the same syntax / directive / rune / node family

Default mapping:

- parser syntax / AST shape -> parser unit test
- analyze metadata -> analyzer unit test
- compiler-output parity -> `tasks/compiler_tests/`

Prefer (in order): extend an existing focused test, add a new focused test, keep a minimal broad repro only when no isolation is safe yet.

New e2e case -> register via `add-test` skill. Diagnostic parity case -> `add-diagnostic-test` skill. Add the ignore attribute on the generated entry:

```rust
#[rstest]
#[ignore = "diagnose: pending fix"]
fn <name>() {
    assert_compiler("<name>");
}
```

Pre-fix `case-rust.*` is evidence of failure; `case-svelte.*` is the reference artifact.

## Step 3 — Find the spec owner

Consult `ROADMAP.md` (spec index) and `CONTEXT.md` (domain map) before searching `specs/*.md`. Skip `specs/unknown.md` until fallback.

Outcomes:

- exactly one matching spec -> read its `Current state`, `Use cases`, `Test cases`; decide if behavior is already tracked
- multiple plausible specs -> ownership ambiguous, fallback to `specs/unknown.md`
- no matching spec -> fallback to `specs/unknown.md`

## Step 4 — Record the follow-up

Use case bullet format:

`[ ] <short title> — layer: <parser|3.A.<n>|3.B|3.C|transform|codegen|css>; repro/test: <test name>; candidate specs: <a, b> or none; suggested spec: <name> or none`

If owner is 3.B, append `debt: migrate to 3.A.<n>` and add a section to `debt.md`. Title and test names come from `CONTEXT.md ## Language` and avoid the banlist.

Test cases bullet format: `[ ] <test name>`.

### Exactly one matching spec

- add/extend the narrowest correct test
- add one unchecked use case if behavior not tracked (cite the existing item if it is)
- add or update the test entry
- keep `Current state` terse — `Working`, `Tests`, `Last updated` only; no dated history bullets
- re-open a `ROADMAP.md` checkbox only if the spec maps to one direct roadmap item and was marked complete prematurely

### Ambiguous or missing owner -> `specs/unknown.md`

Before writing, scan for an existing entry with the same repro/test name or root-cause cluster. Update if found; otherwise add a new use case + matching `Test cases` bullet. Update `Current state`: `Working` = unchecked count, `Tests` = passing/total, `Last updated` = today.

Next command in the report: `/port specs/unknown.md`.

## Step 5 — Cleanup

Remove `_diagnose_tmp` cases and any temporary test registration once a durable test or spec entry exists.

Scrub real-code repros: rewrite `case.svelte` to minimal synthetic form. No domain names, business logic, comments, or identifiers from the source. Same for spec entries — describe behavior, don't quote the original.

## Final report

- root cause
- owning layer
- tests added/changed
- specs updated
- roadmap changes, if any
- next commands as flat bullets, e.g. `/port specs/foo.md`
