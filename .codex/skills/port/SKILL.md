---
name: port
description: Port one session-sized slice of a Svelte compiler feature from spec to the Rust implementation. Use when a feature already has a spec and Codex should implement the next bounded slice, not the whole feature, while keeping strict parser/analyze/codegen boundaries.
---

# Port One Slice

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

Reference Svelte compiler is in `reference/compiler/`. Our Rust compiler is in `crates/svelte_*`.

The command argument is either:

- a spec file path
- a feature description that already maps to a spec

If no matching spec exists, stop and recommend `/audit <feature>` first.

If multiple specs plausibly match the feature description, stop and list the candidate spec files. Do not choose arbitrarily.

## Resume From Spec

If the argument is a spec path, or a matching spec was found:

1. Read the spec file
2. Read `Current state` first
3. Find the next incomplete slice
4. Do not widen scope beyond that slice in the current run

If `Current state` is missing or clearly conflicts with `Use cases` or `Tasks`, normalize the spec first and report the drift before selecting a slice.

Derive the slice from the existing spec structure in this order:

1. explicit slice notes in `Current state`
2. unchecked or partial items in `Use cases`
3. concrete file or layer groupings in `Tasks`
4. `Implementation order`, if present

Treat optional headings such as `Execution slices`, `Next slice`, or `Non-goals` as hints when they exist, not as required structure.

## Scope Contract

This skill ports one bounded session-sized slice.

A slice must satisfy all of these rules:

- covers one cohesive behavior cluster
- has explicit owning layer or justified multi-layer flow
- is as large as is still reasonable to implement and verify within the current session without quality dropping
- may include multiple related use cases when together they produce a more useful milestone than a tiny partial port
- must not encourage shortcuts, speculative optimizations, or rushed architecture decisions just to close more scope
- has clear non-goals for the current run

If the spec does not already define slices explicitly, derive one from the existing `Use cases` and `Tasks` before coding.

## Approach

Use the reference compiler to understand expected output, not to copy structure.

Do not port:

- visitor or walker dispatch patterns mechanically
- mutable AST metadata
- JS-specific workarounds
- broad "make the whole feature pass" batches

Do:

- match reference observable behavior exactly for the selected slice
- keep implementation aligned with crate boundaries

## When New Use Cases Are Discovered

Discovery during implementation is expected.

If new behavior is discovered:

- if it is outside the selected slice, add it to the spec as an unchecked use case and leave it for a later slice
- if it is required to complete the selected slice, include it only if it fits within the existing slice limits
- if it would widen the slice so much that implementation quality or verification confidence would likely drop, stop, update the spec, and report the blocker instead of expanding scope silently

Spec updates are allowed during the run. Scope expansion is not.

## PLAN PHASE

These steps are planning-only. Do not write files during this phase.

### Step 1: Load Slice Context

Research four things:

1. Which incomplete use cases in the spec naturally belong to the next slice
2. Which layers own the missing behavior for this slice
3. Which tests already cover part of the slice
4. Which neighboring use cases are explicitly out of scope for this run

When auto-selecting a slice, group use cases by shared owning layer, shared missing data flow, or shared parser/analyze/codegen dependency.

Prefer the largest useful slice that still has a clean ownership story, a clear verification strategy, and a high-confidence path to systematic implementation in one session.

If the next slice is ambiguous, narrow it before proceeding. Do not start coding with a fuzzy slice.

### Step 2: Slice Definition

Produce a slice definition with these sections:

1. Included use cases
2. Excluded use cases
3. Owning layer
4. Expected files to change
5. Verification strategy

Choose the slice size for usefulness, not minimalism. Prefer a meaningful milestone over a tiny fragment when both fit safely within one session.

If implementing the slice would require architecture changes that do not fit existing boundaries, stop and ask for approval. Do not improvise structural changes.

### Step 3: Draft Spec Update

Prepare a proposed update for the same spec file that was selected earlier so the next session can resume cleanly.

Draft `Current state` updates with:

- current slice name
- why this slice comes next
- non-goals for this run

This pre-implementation spec update is only for planning and resume context. Do not mark use cases as completed in this step.

Do not apply the spec update yet. Present the slice plan and the proposed spec update, then wait for approval.

Do not reshape the spec template just to use this skill. Prefer updating `Current state`, `Use cases`, and `Tasks`.

The plan text must include: **"Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries."**

**Present the slice plan and wait for approval before proceeding. After approval, apply the planned update to the same spec file before writing code.**

## EXECUTE PHASE

Start here only after plan approval. Steps are sequential.

### Step 4: Choose Verification Strategy

Choose the smallest correct verification surface for the selected slice before writing code.

Use e2e compiler tests only when the slice must be checked against reference compiler output.

Use unit tests when the behavior is owned by one layer and does not need end-to-end snapshot comparison.

Default mapping:

- parser syntax and AST shape -> parser unit tests in `test.rs` modules
- analysis metadata, symbol logic, ownership, diagnostics -> analyzer unit tests in `test.rs` modules
- codegen or end-user compiler output that must match the reference compiler -> `tasks/compiler_tests/` e2e coverage

For parser-only or analyze-only slices, prefer layer-local tests and exact AST or analysis expectations unless end-to-end output parity is required.

Do not put diagnostics-only behavior into `tasks/compiler_tests/test_v3.rs` unless the point of the slice is an end-to-end compiler snapshot that must be compared with the reference compiler.

If the slice needs both:

- add unit tests for layer-local behavior
- add the minimum e2e coverage needed to verify observable compiler output

### Step 5: Add Tests For This Slice

Create or extend only the tests selected in Step 4.

For unit tests:

- add them in the owning crate's `test.rs` modules following existing project patterns
- keep them focused on the behavior owned by that layer

For e2e tests:

1. add minimal `tasks/compiler_tests/cases2/<name>/case.svelte`
2. add the matching entry in `tasks/compiler_tests/test_v3.rs`
3. run `just generate` to produce `case-svelte.js`
4. verify generated reference output before implementing

Before implementation, only treat `case-svelte.js` as the reference artifact to review. Do not treat pre-implementation `case-rust.js` output as meaningful.

`case-svelte.js` and `case-rust.js` are generated artifacts. Never edit them manually. They may change only through generation or compiler output.

Rules:

- do not add tests for excluded use cases in this run
- if an existing small test already covers the slice, extend it instead of adding a duplicate

### Step 6: Implement Only The Owning Changes

Implement the slice in the correct layer order:

1. parser and AST only if the slice needs new syntax
2. analyze only if the slice needs new derived data
3. transform or codegen only after required parser or analysis support exists

If the work stops being session-sized or starts pressuring the implementation toward shortcuts, stop, update the spec, and report the blocker instead of widening the slice.

Unit tests are mandatory for every new parser or analyze behavior.

### Step 7: Verify The Slice

If relevant tests already fail before this slice, record that baseline first. Verify that the slice fixes or implements its included use cases without introducing additional regressions. Do not widen scope to fix unrelated baseline failures.

Verify every included test case individually:

```bash
just test-case <test_name>
```

Run this only for e2e tests created for the slice.

Run the relevant unit test command for layer-local coverage, then run:

```bash
just test-compiler
```

Cross-check that:

- every included use case passes
- no new regressions were introduced beyond the recorded baseline
- excluded use cases remain excluded

If a test still fails after 3 attempts, stop and report what was tried. Do not silently expand scope.

### Step 8: Finalize The Slice

Before updating the spec, inspect the diff and confirm that unrelated files were not changed and that generated files changed only through the documented generation or test flow.

Update the spec:

- mark completed use cases
- update `Current state`
- name the next slice
- record any newly discovered unchecked use cases

Mark use cases as completed only here, after implementation and verification succeed.

Move the ROADMAP item only when all spec use cases for the feature are complete.
