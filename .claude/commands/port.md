---
description: Port a Svelte compiler feature from the reference JS compiler to our Rust implementation. Use when the user asks to "port", "implement", or "add support for" a Svelte feature.
argument-hint: "[feature-description or path/to/spec.md]"
---

# Port Svelte feature: $ARGUMENTS

Reference Svelte compiler is in `reference/compiler/`. Our Rust compiler is in `crates/svelte_*`.

The command argument is either:
- A **spec file path** (contains `/` or ends with `.md`) — go straight to "Resume from spec" below
- A **feature description** (e.g. `$derived`, `{@html}`, `style:prop`) — read `ROADMAP.md`, find the matching item, then check for existing spec

## Resume from spec

If the argument is a spec file path, or a matching spec was found by search:
1. Read the spec file
2. Check the **Current state** section — what's done, what's next
3. Skip to the appropriate step in EXECUTE PHASE
4. Do NOT re-run the PLAN PHASE unless the spec file says the plan needs revision

## Session continuation (feature description argument)

If the argument is a feature description (not a path), search for an existing spec:
Run `Glob("specs/*.md")` and scan the results for a file matching this feature (names may differ — e.g. argument `$state` → file `state-rune.md`). If found — go to "Resume from spec" above.
If no matching spec exists — proceed to PLAN PHASE.

## Approach

Use Svelte reference to understand the **expected output**, not to copy the implementation.

Do NOT port:
- Visitor/walker dispatch patterns (we use direct recursive functions)
- Mutable AST metadata (we use AnalysisData side tables)
- JS-specific workarounds (nullish checks on arrays, var hoisting patterns)
- Intermediate abstractions that only exist for zimmerframe compatibility

DO:
- Match the JS output exactly (same function calls, same argument order)
- Simplify control flow when Rust makes it natural (match, iterators, Option)
- Keep functions short and focused — if a Svelte visitor does 5 things, split into clear helpers

---

## PLAN PHASE (steps 1–3)

These steps are read-only. Complete them in plan mode before writing any code.

### Step 1: Parallel research

Launch 3 agents simultaneously:

1. **@reference-tracer** — trace the feature through all 3 phases of the reference compiler. Focus: runtime calls, arguments, conditions, edge cases.
2. **@codebase-analyzer** — find what's already implemented in our Rust compiler for this feature: AST types, analysis passes, codegen, tests.
3. **Explore agent** — search `reference/compiler/tests/` for snapshot inputs/outputs, and cross-cutting references to this feature.

After all agents complete, synthesize findings. Read key files agents identified for planning details.

**Controlled follow-up reads:** only files agents flagged as critical but whose exact content you need. List files and why before reading. Do not launch additional agents.

Output: what the feature requires end-to-end, what's already done, what's missing.

Steps 2 and 3 are done by the main agent directly — no subagent delegation.

### Step 2: Use-case checklist

Produce a structured list grouped by category. Example:

```
### Basic
1. Simple usage: `{@html expr}`
2. With literal string

### Variants
3. With reactive state (`$state`)
4. With derived value (`$derived`)

### Edge cases
5. Empty expression
6. Nested inside component

### Interactions
7. Inside {#if} block
8. Inside {#each} block

### Validation / Errors
9. Invalid placement
10. Missing expression
```

Number every case. Mark which are already handled (from Agent 2 findings).

If 10 or fewer use cases total — present all at once. If more than 10 — present in batches of up to 4 per category via `AskUserQuestion` with `multiSelect: true`. After all rounds:

```
Selected for porting (N cases): ...
Deferred to ROADMAP (M cases): ...
```

Add deferred cases to **Deferred** section at the bottom of `ROADMAP.md` under `### <feature name> (Tier N)`.

### Step 3: Implementation plan

Produce a concrete plan:
- Files to create (test cases) and files to modify (AST, parser, analyze, codegen)
- Specific changes per layer: new types, new functions, modified match arms
- Order: AST types → parser → analysis → codegen

If the feature requires changes that don't fit the existing architecture (new crate, new pattern, new phase) — flag this explicitly and wait for approval. Do not improvise structural changes.

Write the plan to `specs/<feature-name>.md` following the `spec-template` skill. Current state section goes FIRST.

**Present the plan and wait for approval before proceeding.**

---

## EXECUTE PHASE (steps 4–8)

Start here after plan approval. Steps are sequential — run tests before moving to the next step.

### Step 4: Branch

Check current branch:
```
git branch --show-current
```

- If on a `claude/*` branch: stay on it — this is a Claude Code managed branch
- If on any other non-`master` branch: switch to master first, then create a new branch
- If on `master`: create a feature branch:
  ```
  git checkout master && git pull && git checkout -b port/<item>-<short-name>
  ```

Verify with `git branch --show-current`. If still on master, stop and fix before proceeding. Never commit directly to master.

### Step 5: Test cases

Create one test case per selected use case from Step 2:
1. `tasks/compiler_tests/cases2/<feature>_<variant>/case.svelte` — minimal component for that use case
2. Add test in `tasks/compiler_tests/test_v3.rs`: `#[rstest] fn <test_name>() { assert_compiler("<test_name>"); }`

After creating ALL case files, run `just generate` ONCE to generate all `case-svelte.js` files. If it fails, stop and report. Do not attempt to fix the generator.

After generation, read each `case-svelte.js` and verify the output matches expectations from Step 1 research (correct runtime calls, correct structure). If the output looks wrong, fix `case.svelte` now — before implementing anything. If the reference compiler output itself looks incorrect (bug in reference), note it and ask how to proceed.

After creating and running tests, update `specs/<feature>.md` Current state with progress.

Rules:
- **NEVER edit `case-svelte.js` or `case-rust.js`** — these are generated
- One thing per case, minimal component
- Snake_case names: `<feature>_<variant>` (e.g. `html_tag_basic`, `html_tag_in_if`)

### Step 6: Parser & AST

If new syntax is needed (identified in Step 3):
1. Add types to `crates/svelte_ast/src/lib.rs`
2. Add parsing to `crates/svelte_parser/src/lib.rs`
3. Add parser unit tests following the **Unit test pattern** in CLAUDE.md

### Step 7: Analysis & Codegen

If new metadata is needed (identified in Step 3):
1. Add or extend a pass in `crates/svelte_analyze/src/`
2. Add analyze unit tests following the **Unit test pattern** in CLAUDE.md

**Unit tests are mandatory for every new analysis pass or parser change.** Use existing `assert_*` helpers or add new ones — no manual field access in test bodies.

Implement codegen in the corresponding `svelte_codegen_client` module (see navigation table in CLAUDE.md).

Key differences from Svelte:
- Direct recursive functions, not AST walker (zimmerframe)
- `AnalysisData` side tables, not mutated AST metadata
- Store `Span`, re-parse in codegen via `svelte_types` — not stored expressions

Update `specs/<feature>.md` Current state with progress so far.

### Step 8: Verify & Finalize

**Verify each test case individually:**
```
just test-case <test_name>
```

**Cross-check against Step 2 checklist:** confirm every selected use case has a passing test. If any are missing, add them now.

**Run full suite:**
```
just test-compiler
```

If a test fails after 3 attempts, stop and report what you tried. Do NOT fix other tests in the same run.

**If implementation failed and cannot be completed:** run `git stash` to save partial work, report what was done and what remains. Do not leave a broken state on the branch.

**Update tracking:**
- Update `specs/<feature>.md`: mark completed tasks, update Current state section
- Move completed feature to **Done ✅** in `ROADMAP.md`
- Add any newly discovered deferred items to **Deferred** section

**Benchmark** (only if the feature adds new syntax — new AST node types, block types, directive types):
1. Add the construct to `tasks/generate_benchmark/src/main.rs`
2. Increment the version: rename the existing `big_vN.svelte` default in `justfile`, `tasks/benchmark/compare.mjs`, and `tasks/generate_benchmark/src/main.rs` to `big_v(N+1)`
3. Delete the old `big_vN.svelte` file from `tasks/benchmark/benches/compiler/`
4. Generate: `just generate-benchmark`

**Summary:**
```
## Done

### Changes
- [file:line] — what changed and why

### Decisions
- [decision and rationale]

### Tests
- [test_name] — pass/fail

### Next steps
- [what remains, if anything]

### Next
→ `/qa` to verify quality guidelines
→ `/sync-docs` to update ROADMAP/CODEBASE_MAP
```
