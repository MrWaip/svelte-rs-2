---
description: Compile a Svelte component through our pipeline, analyze failures, and add missing test coverage. Use when the user asks to "diagnose", "check component", or wants to find what's broken for a specific component.
argument-hint: "[component-source-or-path]"
---

# Diagnose Svelte component: $ARGUMENTS

Takes a Svelte component source, compiles it through our pipeline, analyzes failures, and adds missing test coverage.

## Step 1: Branch

```
git checkout master && git pull && git checkout -b diagnose/$(date +%s)
```

Verify you are on the new branch with `git branch --show-current`.

## Step 2: Create temporary test

Create `tasks/compiler_tests/cases2/_diagnose_tmp/case.svelte` with the component source from `$ARGUMENTS`.

If `$ARGUMENTS` looks like a file path, read the file and use its contents instead.

Generate expected output for all cases (including the new one):
```
just generate
```

If generation fails, stop and report the error — the component may use unsupported Svelte syntax.

Add a temporary test function in `tasks/compiler_tests/test_v3.rs`:
```rust
#[rstest]
fn _diagnose_tmp() { assert_compiler("_diagnose_tmp"); }
```

## Step 3: Run and analyze

Run the test:
```
just test-case-verbose _diagnose_tmp
```

Read the three files:
- `tasks/compiler_tests/cases2/_diagnose_tmp/case.svelte` — input
- `tasks/compiler_tests/cases2/_diagnose_tmp/case-svelte.js` — expected (Svelte v5)
- `tasks/compiler_tests/cases2/_diagnose_tmp/case-rust.js` — actual (our compiler)

Compare `case-rust.js` vs `case-svelte.js`. Identify every mismatch and classify by layer:

1. **Parser/AST** — node not parsed, wrong span, missing attribute
2. **Analysis** — missing/wrong AnalysisData (content_types, dynamic_nodes, runes, etc.)
3. **Transform** — expression not rewritten (rune get/set, prop thunk, each var)
4. **Codegen** — wrong JS output, missing runtime call, wrong argument order

## Step 4: Build a plan

Output a structured plan:

```
## Diagnosis

### Component features used
[List every Svelte feature the component exercises: reactivity, each blocks, bindings, etc.]

### Mismatches found
For each mismatch:
- **What**: [describe the diff]
- **Layer**: [parser / analyze / transform / codegen]
- **Root cause**: [brief explanation]
- **Fix complexity**: [trivial / moderate / significant]

### Suggested fix order
[Numbered list, ordered by dependency — parser fixes before analyze, analyze before transform, etc.]
```

## Step 5: Add test cases

For each identified issue, add a **focused** test case to existing tests:

- **Parser issues** → add test in `crates/svelte_parser/tests/` following `/test-pattern`
- **Analysis issues** → add test in `crates/svelte_analyze/tests/` following `/test-pattern`
- **Codegen issues** → first check if an existing test case covers the same feature area AND its `case.svelte` is < 30 lines. If yes, extend that test. If no, create a new compiler test case in `tasks/compiler_tests/cases2/<descriptive_name>/`:
  - `case.svelte` — minimal component isolating the single feature
  - Run `just generate` to generate `case-svelte.js` for all cases
  - Add `#[rstest]` test function in `tasks/compiler_tests/test_v3.rs`
  - **NEVER edit `case-svelte.js` or `case-rust.js`** — they are codegen outputs

Each new test case should test **one** feature or edge case. Do not create a single large test that exercises everything.

For each test that FAILS:
- Add `#[ignore = "missing: <description> (<layer>)"]` or `#[ignore = "bug: <description> (<layer>)"]` attribute

Name test cases descriptively: `each_block_nested`, `bind_value_input`, `derived_rune_chain`, etc.

## Step 6: Cleanup

1. Remove the temporary test directory: `rm -rf tasks/compiler_tests/cases2/_diagnose_tmp`
2. Remove the `_diagnose_tmp` test function from `tasks/compiler_tests/test_v3.rs`
3. Run all compiler tests to verify new test cases are in expected state:
   ```
   just test-compiler
   ```
4. New tests that expose known gaps are expected to fail — that's the point. Report which pass and which fail.

## Step 7: Report

Output a final summary:

```
## Results

### New test cases added
- `<test_name>` — [what it tests] — ✅ pass / ❌ fail ([layer]: [brief reason])

### Issues requiring fixes (by priority)
1. [issue] — [layer] — [complexity]
2. ...

### Suggested next steps
- [what to fix first and why]

### Next
→ `/fix-test <name>` for quick-fix and moderate issues
→ `/port` for infrastructure gaps
→ `/improve` for architectural issues
```

## Rules

- Do NOT fix the compiler in this command — only diagnose and add tests.
- Do NOT edit `case-svelte.js` or `case-rust.js` — they are codegen outputs.
- Max 5 new test cases per run. If more issues are found, list them in the report but only create tests for the top 5.
- If the component uses features not in ROADMAP.md, note them but don't create tests for them.
- If stuck after 3 attempts at any step, stop and report.
