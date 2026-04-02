---
name: port
description: Port a Svelte compiler feature from the reference JS compiler to our Rust implementation. Use when the user asks to port, implement, or add support for a Svelte feature, or when continuing work from a related `specs/*.md` file.
---

# Port Svelte Feature

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

Reference Svelte compiler is in `reference/compiler/`. Our Rust compiler is in `crates/svelte_*`.

The command argument is either:

- A spec file path (contains `/` or ends with `.md`) — go straight to "Resume from spec" below
- A feature description (for example `$derived`, `{@html}`, `style:prop`) — read `ROADMAP.md`, find the matching item, then check for existing spec

## Resume From Spec

If the argument is a spec file path, or a matching spec was found by search:

1. Read the spec file
2. Check the `Current state` section — what is done, what is next
3. Skip to the appropriate step in EXECUTE PHASE
4. Do not re-run the PLAN PHASE unless the spec file says the plan needs revision

## Session Continuation

If the argument is a feature description rather than a path, search for an existing spec:

Run `Glob("specs/*.md")` and scan the results for a file matching this feature. Names may differ — for example argument `$state` may map to file `state-rune.md`. If found, go to "Resume from spec" above. If no matching spec exists, proceed to PLAN PHASE.

## Approach

Use Svelte reference to understand the expected output, not to copy the implementation.

Do not port:

- visitor or walker dispatch patterns
- mutable AST metadata
- JS-specific workarounds
- intermediate abstractions that only exist for compatibility in the reference compiler

Do:

- match the JS output exactly
- simplify control flow when Rust makes it natural
- keep functions short and focused

## PLAN PHASE

These steps are read-only. Complete them before writing any code.

### Step 1: Parallel Research

Research three things:

1. Trace the feature through all relevant phases of the reference compiler. Focus on runtime calls, arguments, conditions, and edge cases.
2. Find what is already implemented in our Rust compiler for this feature: AST types, analysis passes, codegen, and tests.
3. Search `reference/compiler/tests/` for snapshot inputs and outputs, plus other references to the feature.

After all three are complete, synthesize findings. Read only the files identified as critical for planning details.

Output: what the feature requires end-to-end, what is already done, and what is missing.

### Step 2: Use-Case Checklist

Produce a structured list grouped by category. Number every case. Mark which are already handled.

If more than 10 use cases exist, present them to the user in batches and get explicit selection before proceeding. After selection, record:

- selected for porting
- deferred in spec

Add each deferred case as its own unchecked checkbox in the spec `Use cases` deferred subsection. If there is no corresponding spec, report that explicitly instead of recording it elsewhere.

### Step 3: Implementation Plan

Produce a concrete plan:

- files to create for test cases
- files to modify in AST, parser, analyze, transform if needed, and codegen
- specific changes per layer
- execution order

If the feature requires changes that do not fit the existing architecture, flag this explicitly and wait for approval. Do not improvise structural changes.

Write the plan to `specs/<feature-name>.md` following the `spec-template` skill. The `Current state` section goes first.

The plan text must include: **"Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries."**

**Present the plan and wait for approval before proceeding.**

## EXECUTE PHASE

Start here only after plan approval. Steps are sequential.

### Step 4: Branch

Check current branch:

```bash
git branch --show-current
```

- If on a `claude/*` branch: stay on it
- If on any other non-`master` branch: switch to `master` first, then create a new branch
- If on `master`: create a feature branch

Verify with `git branch --show-current`. If still on `master`, stop and fix before proceeding. Never commit directly to `master`.

### Step 5: Test Cases

Create one test case per selected use case from Step 2:

1. `tasks/compiler_tests/cases2/<feature>_<variant>/case.svelte` — minimal component for that use case
2. Add test in `tasks/compiler_tests/test_v3.rs`: `#[rstest] fn <test_name>() { assert_compiler("<test_name>"); }`

After creating all case files, run `just generate` once to generate all `case-svelte.js` files. If it fails, stop and report. Do not attempt to fix the generator.

After generation, read each `case-svelte.js` and verify the output matches expectations from planning. If the output looks wrong, fix `case.svelte` before implementing anything.

After creating and running tests, update `specs/<feature>.md` `Current state` with progress.

Rules:

- never edit `case-svelte.js` or `case-rust.js`
- one thing per case, minimal component
- use snake_case names

### Step 6: Parser And AST

If new syntax is needed:

1. Add types to `crates/svelte_ast/src/lib.rs`
2. Add parsing to `crates/svelte_parser/src/lib.rs`
3. Add parser unit tests following project test patterns

### Step 7: Analysis And Codegen

If new metadata is needed:

1. Add or extend a pass in `crates/svelte_analyze/src/`
2. Add analyze unit tests following project test patterns

Unit tests are mandatory for every new analysis pass or parser change.

Implement codegen in the corresponding `svelte_codegen_client` module.

Key differences from Svelte:

- direct recursive functions, not AST walker dispatch
- `AnalysisData` side tables, not mutated AST metadata
- use repo architecture rules rather than JS compiler structure

Update `specs/<feature>.md` `Current state` with progress so far.

### Step 8: Verify And Finalize

Verify each test case individually:

```bash
just test-case <test_name>
```

Cross-check against the Step 2 checklist and confirm every selected use case has a passing test.

Run full suite:

```bash
just test-compiler
```

If a test fails after 3 attempts, stop and report what was tried. Do not fix unrelated tests in the same run.

Update tracking:

- update `specs/<feature>.md`
- move completed feature to `Done` in `ROADMAP.md` when appropriate
- add newly discovered deferred items to the spec as unchecked checkboxes

Benchmark only when the feature adds new syntax or other benchmark-relevant constructs.

## Summary

Report:

- changes
- decisions
- tests
- next steps

Then recommend:

- `/qa`
- `/sync-docs`
