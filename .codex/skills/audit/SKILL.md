---
name: audit
description: Feature completeness audit workflow for comparing this compiler against the reference Svelte compiler. Use when the user asks what is missing for a feature, wants a gap analysis, or needs a spec plus failing tests before implementation. Do not trigger when the task is already a known focused fix.
---

# Audit Feature

## 1) Resume existing work first

If a matching spec already exists in `specs/`, read it and continue from `Current state` instead of restarting the audit.

## 2) Research both sides

Compare:

- reference compiler behavior and tests in `reference/compiler/`
- current Rust implementation and existing compiler tests

## 3) Build a use-case matrix

Classify each use case as:

- Covered
- Partial
- Missing
- Unknown

## 4) Write or update a spec

Use `spec-template` for structure. Capture:

- use cases
- reference files
- our files
- implementation tasks
- recommended order

If this audit creates a new spec for a roadmap feature, add a link to that spec in `ROADMAP.md` right away. Keep the checklist status unchanged; only append or refresh the `(specs/<name>.md)` reference for the matching item.

## 5) Add focused missing tests

For Missing or Unknown cases, add focused compiler tests where useful. Generate expected output with the reference compiler and never hand-edit generated snapshots.

Cap the run to a bounded set of new tests rather than exploding coverage in one go.

## 6) Report recommended fix order

Point to the next best command:

- `fix-test <name>` for narrow gaps
- `port specs/<name>.md` for infrastructure or multi-layer work

Keep audit primarily diagnostic. Do not mix in broad implementation.
