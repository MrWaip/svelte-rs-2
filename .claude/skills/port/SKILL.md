---
name: port
description: Port the next explicit unchecked use case from a Svelte compiler spec to the Rust implementation. Use when a feature already has a spec and Claude should fully close one use case, or a small group of use cases that naturally close together, while keeping strict parser/analyze/codegen boundaries. If the selected use case is too large to close cleanly, decompose it in the spec and stop.
---

# Port Use Cases To Closure

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

Reference Svelte compiler: `reference/compiler/`. Our Rust compiler: `crates/svelte_*`.

Command arg is one of:

- spec file path
- feature description that maps to spec

No matching spec -> stop, recommend `/audit <feature>` first.

Multiple specs plausibly match -> stop, list candidates. Do not pick arbitrarily.

## Resume From Spec

Spec path or matched spec:

1. Read spec file
2. Read `Current state` first
3. Find next unchecked use case
4. Close completely, unless multiple unchecked use cases belong to one closure unit

`Current state` missing or conflicts with `Use cases`/`Tasks` -> normalize spec first, report drift before picking closure target.
`Current state` turned into dated changelog -> collapse back to terse resume header before proceeding.

Derive closure target from existing spec structure, in this order:

1. first unchecked or partial item in `Use cases`
2. concrete file/layer groupings in `Tasks`
3. `Implementation order`, if present

Optional headings like `Execution slices`, `Next slice`, `Non-goals` = hints, not required structure.

## Scope Contract

Skill closes explicit use cases.

Default unit = one unchecked use case. Multiple only when they close together naturally.

Selected unit must satisfy all:

- closes at least one use case completely
- one cohesive behavior cluster
- explicit owning layer, or justified multi-layer flow
- may bundle related use cases sharing missing data flow, tests, or ownership path
- no shortcuts, speculative optimizations, rushed architecture decisions
- clear non-goals for current run

Never do partial pass and leave same checkbox open without changing spec structure.

Selected use case too broad or entangled:

- stop implementation
- update spec: decompose into smaller unchecked use cases
- tell user it was split and why

Spec lacks usable closure units -> derive from existing `Use cases`/`Tasks` before coding.

## Approach

Reference compiler = understand expected output, not copy structure.

Do not port:

- visitor/walker dispatch patterns mechanically
- mutable AST metadata
- JS-specific workarounds
- broad "make whole feature pass" batches

Do:

- match reference observable behavior exactly for selected use case set
- keep implementation aligned with crate boundaries

Do not respond to repeated `/port` runs by explaining open checkboxes are expected. Close a use case or split one that is too large.

## When New Use Cases Are Discovered

Discovery is expected.

New behavior:

- outside selected set -> add as unchecked use case, leave for later
- required to complete selected set -> include only when still cleanly closes unit
- reveals selected use case is broader than spec implied -> stop, decompose in spec, report split. Do not silently widen scope.

Spec updates allowed. Scope expansion not.

## PLAN PHASE

Planning only. No file writes in this phase.

### Step 1: Load Closure Context

Research four things:

1. Which unchecked use case closes next
2. Which other unchecked use cases close together with it
3. Which layers own missing behavior for this closure unit
4. Which tests already cover part of it

Group by shared owning layer, shared missing data flow, or shared parser/analyze/codegen dependency.

Do not guess what fits in session. Pick next explicit use case, or small set obviously belonging together, close fully.

Next unit ambiguous -> narrow before proceeding. Do not start coding with fuzzy target.

### Step 2: Closure Definition

Sections:

1. Included use cases
2. Excluded use cases
3. Owning layer
4. Expected files to change
5. Verification strategy
6. Closure condition

`Closure condition` states what must be true for each included use case to be marked `[x]`.

Chosen use case cannot close without decomposition -> do not proceed. Prepare spec split.

Closure unit requires architecture changes that do not fit existing boundaries -> stop, ask approval. No improvised structural changes.

### Step 3: Draft Spec Update

Prepare proposed update for same spec so next session resumes cleanly.

Do not use `Current state` as planning scratchpad. Terse resume header only.
Spec needs planning update before implementation -> refine `Use cases` or decompose broad item, not planning bullets in `Current state`.

Selected use case too broad -> draft decomposition instead of normal closure plan.

Do not apply spec update yet. Present closure plan and proposed spec update, wait for approval.

Do not reshape spec template just to use this skill. Prefer updating `Current state`, `Use cases`, `Tasks`.

Plan text must include: **"Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries."**

**Present closure plan and wait for approval. After approval, apply planned update to spec before writing code.**

## EXECUTE PHASE

Start only after plan approval. Sequential.

### Step 4: Choose Verification Strategy

Pick smallest correct verification surface before writing code.

e2e compiler tests only when closure unit must check against reference compiler output.

Unit tests when behavior owned by one layer and needs no end-to-end snapshot comparison.

Default mapping:

- parser syntax and AST shape -> parser unit tests in `test.rs` modules
- analysis metadata, symbol logic, ownership, diagnostics -> analyzer unit tests in `test.rs` modules
- observable diagnostic parity against npm `svelte/compiler` -> `tasks/diagnostic_tests/`
- codegen or compiler output that must match reference -> `tasks/compiler_tests/` e2e coverage

Parser-only or analyze-only closure units -> prefer layer-local tests and exact AST/analysis expectations unless e2e parity required.

Do not put diagnostics-only behavior into `tasks/compiler_tests/test_v3.rs` unless point of closure unit is e2e compiler snapshot vs reference compiler.

Closure unit needs both:

- unit tests for layer-local behavior
- minimum e2e coverage to verify observable compiler output

### Step 5: Add Tests For This Closure Unit

Create or extend only tests selected in Step 4.

Unit tests:

- add in owning crate's `test.rs` modules following existing project patterns
- focus on behavior owned by that layer

e2e tests:

1. add minimal `tasks/compiler_tests/cases2/<name>/case.svelte`
2. add matching entry in `tasks/compiler_tests/test_v3.rs`
3. run `just generate` to produce `case-svelte.js`
4. verify generated reference output before implementing

Before implementation, treat only `case-svelte.js` as reference artifact to review. Do not treat pre-implementation `case-rust.js` as meaningful.

`case-svelte.js` and `case-rust.js` = generated artifacts. Never edit manually. Change only through generation or compiler output.

Diagnostic parity tests:

1. add minimal `tasks/diagnostic_tests/cases/<name>/case.svelte`
2. add matching entry in `tasks/diagnostic_tests/test_diagnostics.rs`
3. run `just generate` to produce `case-svelte.json`
4. verify generated reference diagnostics before implementing

Before implementation, treat only `case-svelte.json` as reference artifact. Do not treat pre-implementation `case-rust.json` as meaningful.

`case-svelte.json` and `case-rust.json` = generated artifacts. Never edit manually. Change only through generation or compiler output.

Rules:

- no tests for excluded use cases in this run
- existing small test already covers closure unit -> extend instead of duplicating

### Step 6: Implement Only The Owning Changes

Layer order:

1. parser and AST only if closure unit needs new syntax
2. analyze only if closure unit needs new derived data
3. transform or codegen only after required parser/analysis support exists

Second infrastructural concept becomes necessary mid-run -> stop, decompose in spec, report split. No half-finished implementation.

Unit tests mandatory for every new parser or analyze behavior.

### Step 7: Verify The Closure Unit

Relevant tests already fail before closure unit -> record baseline first. Verify closure unit fixes included use cases without new regressions. Do not widen scope to fix unrelated baseline failures.

Verify every included test case individually:

```bash
just test-case <test_name>
```

Run only for e2e tests created for closure unit.

Diagnostic parity cases:

```bash
just test-diagnostic-case <test_name>
```

Run relevant unit test command for layer-local coverage, then:

```bash
just test-compiler
```

Cross-check:

- every included use case passes
- no new regressions beyond recorded baseline
- excluded use cases remain excluded

Test fails after 3 attempts -> stop, report what was tried. Do not silently expand scope.

### Step 8: Finalize The Closure Unit

Before updating spec, inspect diff. Confirm unrelated files not changed and generated files changed only through documented generation or test flow.

Update spec:

- mark completed use cases
- update `Current state` counts and date
- record newly discovered unchecked use cases

Mark use cases completed only here, after implementation and verification succeed.

Decomposed instead of closing:

- replace original broad use case with smaller explicit unchecked use cases
- tell user exactly which new use cases were created

Move ROADMAP item only when all spec use cases for feature are complete.
