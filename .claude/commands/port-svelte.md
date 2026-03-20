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

### Step 3: Implementation plan

Produce a concrete plan:
- Files to create (test cases) and files to modify (AST, parser, analyze, codegen)
- Specific changes per layer: new types, new functions, modified match arms
- Order: AST types → parser → analysis → codegen

If the feature requires changes that don't fit the existing architecture (new crate, new pattern, new phase) — flag this explicitly and wait for approval. Do not improvise structural changes.

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

Rules:
- **NEVER edit `case-svelte.js` or `case-rust.js`** — these are generated
- One thing per case, minimal component
- Snake_case names: `<feature>_<variant>` (e.g. `html_tag_basic`, `html_tag_in_if`)

### Step 6: Parser & AST

If new syntax is needed (identified in Step 3):
1. Add types to `crates/svelte_ast/src/lib.rs`
2. Add parsing to `crates/svelte_parser/src/lib.rs`
3. Add parser tests following `/test-pattern`

### Step 7: Analysis & Codegen

If new metadata is needed (identified in Step 3):
1. Add or extend a pass in `crates/svelte_analyze/src/`
2. Add analyze tests following `/test-pattern`

Implement codegen in the corresponding `svelte_codegen_client` module (see navigation table in CLAUDE.md).

Key differences from Svelte:
- Direct recursive functions, not AST walker (zimmerframe)
- `AnalysisData` side tables, not mutated AST metadata
- Store `Span`, re-parse in codegen via `svelte_js` — not stored expressions

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
- Move completed feature to **Done ✅** in `ROADMAP.md`
- Add any newly discovered deferred items to **Deferred** section

**Benchmark** (only if the feature adds new syntax — new AST node types, block types, directive types):
1. Add the construct to `tasks/generate_benchmark/src/main.rs`
2. Generate new `big_vN.svelte`: `just generate-benchmark big_vN`
3. Verify: `cargo bench -p benchmark -- --test`
4. Do NOT modify previous `big_vN.svelte` files
