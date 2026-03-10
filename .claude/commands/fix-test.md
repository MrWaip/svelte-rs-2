# Fix failing test: $ARGUMENTS

Fix a single compiler test case. The test name is provided as argument.

## Approach

When reading Svelte reference visitors to understand the fix, focus on **what** output they produce, not **how** they're structured. Don't port visitor patterns, mutable metadata, or JS workarounds. Use our existing architecture: direct recursion, AnalysisData side tables, match/Option.

## Step 1: Understand the failure

Run the test to see the diff:
```
cargo test -p compiler_tests --test compiler_tests_v3 $ARGUMENTS -- --nocapture
```

Read the three files in `tasks/compiler_tests/cases2/$ARGUMENTS/`:
- `case.svelte` — input component
- `case-svelte.js` — expected output (from Svelte v5 compiler)
- `case-rust.js` — actual output (from our compiler, written during test run)

Compare `case-rust.js` vs `case-svelte.js` to understand exactly what's wrong.

## Step 2: Diagnose the root cause

The issue is in one of these layers (check in order):

1. **Parser/AST** (`svelte_parser`, `svelte_ast`) — is the input parsed correctly? Check if all nodes, attributes, and expressions are captured. Reference: `reference/compiler/phases/1-parse/`.
2. **Analysis** (`svelte_analyze`) — are the analysis results correct? Check `expressions`, `dynamic_nodes`, `content_types`, `lowered_fragments`, `runes`, `mutated_runes`. Reference: `reference/compiler/phases/2-analyze/visitors/`.
3. **Codegen** (`svelte_codegen_client`) — is the JS output generated correctly? Compare codegen logic against the Svelte reference visitor. Reference: `reference/compiler/phases/3-transform/client/visitors/`.

Use the navigation table in CLAUDE.md to find the exact file.

For detailed type signatures and module structure, read `CODEBASE_MAP.md`.

## Step 3: Fix

Apply the minimal fix in the appropriate layer. Do NOT fix multiple test cases at once — focus only on `$ARGUMENTS`.

## Step 4: Verify

Run the single test:
```
cargo test -p compiler_tests --test compiler_tests_v3 $ARGUMENTS
```

Then run ALL tests to check for regressions:
```
cargo test -p compiler_tests --test compiler_tests_v3
```

If the fix breaks other tests, stop and report — do NOT fix other tests in the same run.

If the test still fails after 3 fix attempts, stop and report what you've tried.
