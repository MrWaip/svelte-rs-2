# Audit feature: $ARGUMENTS

Gap analysis for an existing feature: compare our implementation against the reference Svelte compiler and produce a spec file with what's missing.

## Step 1: Research reference compiler

Launch 2 Explore agents:

1. **Agent 1 -- Reference compiler**: Trace `$ARGUMENTS` through all 3 phases:
   - `reference/compiler/phases/1-parse/` -- all syntax variants
   - `reference/compiler/phases/2-analyze/visitors/` -- all metadata, flags, conditions
   - `reference/compiler/phases/3-transform/client/visitors/` -- all codegen branches, every `if`/`switch`, every runtime `$.helper()` call
   - Focus on exhaustive enumeration: every code path = one use case

2. **Agent 2 -- Our codebase**: Find everything related to `$ARGUMENTS`:
   - AST types, parser code, analyze passes, codegen
   - Existing test cases in `tasks/compiler_tests/cases2/`
   - Run each existing test: `just test-case <name>` -- which pass, which fail?

## Step 2: Gap analysis

For each use case from the reference compiler, classify:
- **Covered** -- we handle it and have a passing test
- **Partial** -- we handle some aspect but output differs
- **Missing** -- not implemented at all
- **Unknown** -- can't determine without a test

## Step 3: Write spec file

Create `specs/<feature-name>.md`:

```
# <feature-name>

## Source
Audit of existing implementation

## Reference
- Svelte: [files from Agent 1]
- Our code: [files from Agent 2]

## Use cases
1. [x] Basic -- description (covered, test: test_name)
2. [~] Variant -- description (partial: what works, what doesn't)
3. [ ] Edge case -- description (missing)

## Tasks (по слоям)
[only for missing/partial items]
1. [ ] parser: ...
2. [ ] analyze: ...
3. [ ] codegen: ...
4. [ ] tests: ...

## Current state
- Working: [list of covered use cases]
- Not working: [list of missing/partial]
- Next: [recommended first task]
```

## Step 4: Add missing test cases

For each **Missing** or **Unknown** use case, create a test case:
- `tasks/compiler_tests/cases2/<feature>_<variant>/case.svelte`
- Run `just generate` once for all new cases
- Add `#[rstest]` functions in `test_v3.rs`
- Run tests, report which pass and which fail

Rule: if an existing test case covers the same feature and `case.svelte` < 30 lines, extend it instead of creating a new one.

## Step 5: Report

```
## Audit: <feature>

### Coverage: N/M use cases (X%)
### Passing tests: K
### Failing tests: L (expected -- gaps)

### Recommended fix order
1. [use case] -- [layer] -- [why first]
2. ...

Spec file: specs/<feature-name>.md
```

## Rules
- Do NOT fix the compiler -- only audit and add tests.
- Do NOT edit `case-svelte.js` or `case-rust.js`.
- Max 5 new test cases per run.
- If stuck after 3 attempts, stop and report.
