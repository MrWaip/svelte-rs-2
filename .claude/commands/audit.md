---
description: Gap analysis for an existing feature vs reference Svelte compiler. Use when the user asks "what's missing in our X implementation", "audit feature", or "check feature completeness".
argument-hint: "[feature-name]"
allowed-tools: Bash, Read, Grep, Glob, Write, Edit, Agent
---

# Audit feature: $ARGUMENTS

Gap analysis for an existing feature: compare our implementation against the reference Svelte compiler and produce a spec file with what's missing.

## Session continuation

Run `Glob("specs/*.md")` and scan the results for a file matching this feature (names may differ from the argument — e.g. argument `$state` → file `state-rune.md`). If a matching spec exists:
1. Read the spec file
2. Check the **Current state** section — what's done, what's next
3. Skip to the appropriate step (likely Step 4 or Step 5)
4. Do NOT re-run Steps 1–3 unless the spec says the audit needs revision

## Step 1: Research

Launch 2 agents in parallel:

1. **@reference-tracer** — trace `$ARGUMENTS` through all 3 phases. Focus on exhaustive enumeration: every code path = one use case.
2. **@codebase-analyzer** — find everything related to `$ARGUMENTS` in our compiler. Run each existing test: `just test-case <name>` — which pass, which fail?

After agents complete, synthesize findings. Read key files agents flagged for details.

**Controlled follow-up reads:** only files agents identified as critical. List files and why before reading. Do not launch additional agents.

## Step 2: Gap analysis

For each use case from the reference compiler, classify:
- **Covered** -- we handle it and have a passing test
- **Partial** -- we handle some aspect but output differs
- **Missing** -- not implemented at all
- **Unknown** -- can't determine without a test

## Step 3: Write spec file

Write the spec following the `spec-template` skill.

## Step 4: Add missing test cases

For each **Missing** or **Unknown** use case, create a test case:
- `tasks/compiler_tests/cases2/<feature>_<variant>/case.svelte`
- Run `just generate` once for all new cases
- Add `#[rstest]` functions in `test_v3.rs`
- Run tests, report which pass and which fail

For each test that FAILS:
- Add `#[ignore = "missing: <description> (<layer>)"]` attribute
- Classify effort: quick fix / moderate / needs infrastructure

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

### Test results by effort

#### Quick fix (one file, existing patterns)
- `test_name` — layer: description → `/fix-test test_name`

#### Moderate (2-3 files, existing infrastructure)
- `test_name` — layer: description → `/fix-test test_name`

#### Requires new infrastructure
- `test_name` — layer: description → `/port specs/<name>.md` (use case #N)

Spec file: specs/<feature-name>.md
```

## Rules
- Do NOT fix the compiler -- only audit and add tests.
- Do NOT edit `case-svelte.js` or `case-rust.js`.
- Max 5 new test cases per run.
- If stuck after 3 attempts, stop and report.

### Next
→ `/port specs/<name>.md` to implement missing use cases (infrastructure)
→ `/fix-test <name>` for quick-fix and moderate tests
