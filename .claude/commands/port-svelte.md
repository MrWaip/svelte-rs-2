# Port Svelte feature: $ARGUMENTS

Reference Svelte compiler is in `reference/compiler/`. Our Rust compiler is in `crates/svelte_*`.

The command argument is a feature description (e.g. `$derived`, `{@html}`, `style:prop`).
Before starting, read `ROADMAP.md`, find the matching item, and use the listed files and reference links.

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

When launching Explore agents, exclude generated files from searches: `case-svelte.js`, `case-rust.js`.

Launch 3 Explore agents simultaneously:

1. **Agent 1 — Reference compiler**
   - Trace the feature through all 3 phases of the reference compiler:
     - `reference/compiler/phases/1-parse/` — syntax variants, how the feature is parsed
     - `reference/compiler/types/template.d.ts` — AST node shape, optional fields, union variants
     - `reference/compiler/phases/2-analyze/visitors/` — metadata, flags, special conditions
     - `reference/compiler/phases/3-transform/client/visitors/` — codegen branches, edge case handling
   - Focus on: `if`/`switch` branches (each = distinct use case), runtime `$.helper()` calls, diagnostics
   - When reading reference codegen, extract ONLY: what runtime functions are called, with what arguments, in what order. Ignore the visitor dispatch structure.

2. **Agent 2 — Our codebase**
   - `crates/svelte_ast/src/lib.rs` — what AST types we already have
   - `crates/svelte_analyze/src/` — what analysis passes we already have
   - `crates/svelte_codegen_client/src/` — what's already implemented
   - `tasks/compiler_tests/cases2/` — which test cases already cover this feature

3. **Agent 3 — Test examples**
   - `reference/compiler/tests/` — snapshot inputs and expected outputs for this feature
   - Search for the feature name (and aliases) across all reference files to catch cross-cutting concerns

After all agents complete, synthesize findings from agent results. Agents return summaries, not full file contents — you may need to read key files yourself for planning details.

**Controlled follow-up reads:** only files that agents identified as critical for the implementation plan but whose exact content (type signatures, function signatures, match arms) you need to see. List the files and why before reading. Do not re-read files that agents already summarized adequately. Do not launch additional agents (Plan, Explore, or otherwise).

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

### Step 3: Write spec file

Based on research from Steps 1-2, create the spec file using the `/spec` format.

Write `specs/<feature-slug>.md` with:
- **Expected output** — JS examples from reference compiler research
- **Tasks** — atomic, one layer each, ordered: test → parser/ast → analyze → transform → codegen → verify
- **Edge cases** — from reference research

For each task, include:
- Layer (parser/analyze/codegen/etc.)
- Files to modify
- Done condition
- Specific changes: new types, new functions, modified match arms

**Quality verification:** for each proposed task, verify it satisfies the **Quality checklist** in CLAUDE.md. If a task would violate any of the 5 points, redesign it before presenting.

If the feature requires changes that don't fit the existing architecture (new crate, new pattern, new phase) — flag this explicitly and wait for approval.

**Present the spec and wait for approval before proceeding.**

---

## EXECUTE PHASE (steps 4–9)

Start here after spec approval. Execute tasks from the spec file **one at a time, in order**.

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

### Step 5: Execute tasks from spec

Work through spec tasks **sequentially**. For each task:

1. **Read the task** from `specs/<feature>.md`
2. **Implement** — only the files listed for that task, only in the layer specified
3. **Test** — run the relevant tests for that layer:
   - parser task → parser unit tests
   - analyze task → analyze unit tests
   - codegen task → `just test-case <name>`
4. **Mark done** — check the box in the spec file
5. **Commit** — one commit per task, message references the spec task number

Task-specific rules:

**Test tasks** (usually first):
- Create `tasks/compiler_tests/cases2/<feature>_<variant>/case.svelte`
- Add test in `test_v3.rs`: `#[rstest] fn <test_name>() { assert_compiler("<test_name>"); }`
- Run `just generate` ONCE to generate all `case-svelte.js` files
- Read each `case-svelte.js` and verify correctness
- **NEVER edit `case-svelte.js` or `case-rust.js`** — generated files

**Parser/AST tasks:**
- Add parser unit tests following the **Unit test pattern** in CLAUDE.md

**Analyze tasks:**
- Add analyze unit tests following the **Unit test pattern** in CLAUDE.md
- **Unit tests are mandatory for every new analysis pass or parser change**

**Codegen tasks:**
- Direct recursive functions, not AST walker
- `AnalysisData` side tables, not mutated AST metadata

### Step 6: Verify & Finalize

After all spec tasks are done:

**Quality gate:** verify all new and modified code against the **Quality checklist** in CLAUDE.md. All 5 points must hold.

**Verify each test case individually:**
```
just test-case <test_name>
```

**Run full suite:**
```
just test-compiler
```

If a test fails after 3 attempts, stop and report what you tried. Do NOT fix other tests in the same run.

**If implementation failed and cannot be completed:** run `git stash` to save partial work, update spec progress section with what was done and what remains. Do not leave a broken state on the branch.

**Update tracking:**
- Mark spec status as `done`
- Move completed feature to **Done ✅** in `ROADMAP.md`
- Add any newly discovered deferred items to **Deferred** section

**Benchmark** (only if the feature adds new syntax — new AST node types, block types, directive types):
1. Add the construct to `tasks/generate_benchmark/src/main.rs`
2. Increment the version: rename the existing `big_vN.svelte` default in `justfile`, `tasks/benchmark/compare.mjs`, and `tasks/generate_benchmark/src/main.rs` to `big_v(N+1)`
3. Delete the old `big_vN.svelte` file from `tasks/benchmark/benches/compiler/`
4. Generate: `just generate-benchmark`
