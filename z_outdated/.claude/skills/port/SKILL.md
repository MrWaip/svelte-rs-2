---
name: port
description: Port the next explicit unchecked use case from a Svelte compiler spec to the Rust implementation. Use when a feature already has a spec and Claude should fully close one use case, or a small group of use cases that naturally close together, while keeping strict parser/analyze/codegen boundaries. If the selected use case is too large to close cleanly, decompose it in the spec and stop.
disable-model-invocation: true
---

# Port Use Cases To Closure

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

Original Svelte compiler: `original/compiler/`. Our Rust compiler: `crates/svelte_*`.

Command arg is one of:

- spec file path
- feature description that maps to spec

No matching spec -> stop, recommend `/audit <feature>` first.

Multiple specs plausibly match -> stop, list candidates. Do not pick arbitrarily.

## Architecture Is Mandatory

Before anything else in `/port`, read `ARCHITECTURE.md` and `CONTEXT.md` (skip only if both already in current session context). Every proposed design, file placement, type name, and layer decision must align with their invariants and glossary (`CONTEXT.md ## Language` + `CLAUDE.md ## Never use`). Solution that violates crate boundaries or smart-analyzer / dumb-codegen dogma is not acceptable — reformulate, do not stretch the architecture to fit.

## Resume From Spec

Spec path or matched spec:

1. Read `ARCHITECTURE.md` and `CONTEXT.md` if not already in session
2. Read spec file
3. Read `Current state` first
4. Find next unchecked use case
5. Close completely, unless multiple unchecked use cases belong to one closure unit

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

Original = understand expected output, not copy structure.

Do not port:

- visitor/walker dispatch patterns mechanically
- mutable AST metadata
- JS-specific workarounds
- broad "make whole feature pass" batches

Do:

- match Original observable behavior exactly for selected use case set
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

### Step 2: Domain Anchor

Mandatory before Step 3. Invoke `domain-anchor` via `Agent` (`subagent_type=domain-anchor`).

Prompt to agent contains exactly two fields, nothing else: **bullet_text** (verbatim quote of the unchecked bullet, no paraphrase) and **divergence** (input `.svelte` + reference `.js` + actual `.js`, or a path to the test case, or — for greenfield — plain-language description without file references).

Forbidden in the prompt: pointers to files/functions/types, candidate clusters or variants, lists of allowed gap classifications, redefinitions of agent's terminology, pre-cooked answers, requests to confirm or be concise. If you catch yourself wanting any of these, send only the two fields anyway — wanting them means you already did the agent's job and don't trust your result.

Wait for tool result. Agent's verdict is binding: layer, domain unit, carrier, gap kind, prescribed action. You do not reclassify, do not propose alternatives, do not "add a variant to the existing local triage enum". Cross-layer order is binding. New-carrier verdict proceeds only if all three greenfield criteria are listed; otherwise stop and escalate to user. If anti-pattern (б) is named, follow the prescribed (i) inline or (ii) debt action verbatim.

You may not introduce in Step 3 or later any new struct / field / method / enum / pass whose role is to triage existing AST or `*Semantics` variants. Wanting to → re-invoke `domain-anchor`.

### Step 3: Closure Definition

Sections:

1. Included use cases
2. Excluded use cases
3. Owning layer
4. Original dispatch
5. Expected files to change
6. Verification strategy
7. Closure condition

`Original dispatch` is mandatory and uses this exact shape:

```
- Original handler: <file:line>  (line number required; function name alone is insufficient)
- Dispatch style: <visitor / match / for-loop / recursive walk>
- Node kinds it covers: <list>
- My change covers: <list>
- Not covered (and why ok): <list, or "nothing excluded">
```

`My change covers` strictly smaller than `Node kinds it covers` without explicit justification in `Not covered` -> not a closure, decompose.

Every `Not covered` item must:

- cite a different `Original handler <file:line>` than the top one — same handler means same closure unit, include it or decompose
- end with `→ spec:<line>` to an existing bullet or to a new `[ ]` added in Step 4

`Closure condition` states what must be true for each included use case to be marked `[x]`.

Chosen use case cannot close without decomposition -> do not proceed. Prepare spec split.

Closure unit requires architecture changes that do not fit existing boundaries -> stop, ask approval. No improvised structural changes.

### Step 4: Draft Spec Update

Prepare proposed update for same spec so next session resumes cleanly.

Do not use `Current state` as planning scratchpad. Terse resume header only.
Spec needs planning update before implementation -> refine `Use cases` or decompose broad item, not planning bullets in `Current state`.

Selected use case too broad -> draft decomposition instead of normal closure plan.

`Not covered` items from Step 3 without an existing spec anchor must be added here as new unchecked bullets in this exact shape:

```
- [ ] <use case> — deferred: <one-line why separate from current closure>
```

Do not apply spec update yet. Present closure plan and proposed spec update, wait for approval.

Do not reshape spec template just to use this skill. Prefer updating `Current state`, `Use cases`, `Tasks`.

Plan text must include: **"Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries."**

**Present closure plan and wait for approval. After approval, apply planned update to spec before writing code.**

## EXECUTE PHASE

Start only after plan approval. Sequential.

### Step 5: Choose Verification Strategy

Pick smallest correct verification surface before writing code.

e2e compiler tests only when closure unit must check against Original output.

Unit tests when behavior owned by one layer and needs no end-to-end snapshot comparison.

Default mapping:

- parser syntax and AST shape -> parser unit tests in `test.rs` modules
- semantics, symbols, scopes, diagnostics -> analyze unit tests in `test.rs` modules
- observable diagnostic parity against Original -> `tasks/diagnostic_tests/`
- codegen or compiler output that must match Original -> `tasks/compiler_tests/` e2e coverage

Parser-only or analyze-only closure units -> prefer layer-local tests and exact AST/analysis expectations unless e2e parity required.

Do not put diagnostics-only behavior into `tasks/compiler_tests/test_v3.rs` unless point of closure unit is e2e compiler snapshot vs Original.

Closure unit needs both:

- unit tests for layer-local behavior
- minimum e2e coverage to verify observable compiler output

### Step 6: Add Tests For This Closure Unit

Create or extend only tests selected in Step 4.

Unit tests:

- add in owning crate's `test.rs` modules following existing project patterns
- focus on behavior owned by that layer

e2e tests:

1. add minimal `tasks/compiler_tests/cases2/<name>/case.svelte`
2. add matching entry in `tasks/compiler_tests/test_v3.rs`
3. run `just generate` to produce `case-svelte.js`
4. verify generated Original output before implementing

Before implementation, treat only `case-svelte.js` as Original artifact to review. Do not treat pre-implementation `case-rust.js` as meaningful.

`case-svelte.js` and `case-rust.js` = generated artifacts. Never edit manually. Change only through generation or compiler output.

Diagnostic parity tests:

1. add minimal `tasks/diagnostic_tests/cases/<name>/case.svelte`
2. add matching entry in `tasks/diagnostic_tests/test_diagnostics.rs`
3. run `just generate` to produce `case-svelte.json`
4. verify generated Original diagnostics before implementing

Before implementation, treat only `case-svelte.json` as Original artifact. Do not treat pre-implementation `case-rust.json` as meaningful.

`case-svelte.json` and `case-rust.json` = generated artifacts. Never edit manually. Change only through generation or compiler output.

Rules:

- no tests for excluded use cases in this run
- existing small test already covers closure unit -> extend instead of duplicating

### Step 7: Implement Only The Owning Changes

Layer order:

1. parser and Svelte AST only if closure unit needs new syntax
2. analyze only if closure unit needs new semantics
3. transform or codegen only after required parser/analysis support exists

Second infrastructural concept becomes necessary mid-run -> stop, decompose in spec, report split. No half-finished implementation.

Unit tests mandatory for every new parser or analyze behavior.

### Step 8: Verify The Closure Unit

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

### Step 9: Finalize The Closure Unit

Before updating spec, inspect diff. Confirm unrelated files not changed and generated files changed only through documented generation or test flow.

Update spec — minimal edits only:

- flip `[ ]` -> `[x]` on completed use cases
- bump `Working: N/M` and `Tests: X/Y` counters
- update `Last updated` date
- append newly discovered unchecked use cases as plain `[ ]` lines

Forbidden when closing a use case:

- appending implementation summary, file list, owner, Original parallels, or sibling cluster notes to the checkbox line
- adding dated entries (`<date>: <what was closed>`) anywhere in `Current state`
- rewriting `Current state` into a prose paragraph or changelog — it stays three lines: `Working`, `Tests`, `Last updated`

Mark use cases completed only here, after implementation and verification succeed.

Decomposed instead of closing:

- replace original broad use case with smaller explicit unchecked use cases
- tell user exactly which new use cases were created

Move ROADMAP item only when all spec use cases for feature are complete.
