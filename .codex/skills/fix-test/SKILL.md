---
name: fix-test
description: Diagnose and fix a single failing compiler test case. Use when the user names one test, asks to make one compiler case pass, or wants a focused single-test bugfix. Do not trigger for multi-test audits, feature-gap reviews, or broad component diagnosis.
---

# Fix Single Test

**Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.**

## 1) Reproduce and read the failure

Run:

```bash
.codex/scripts/fix-test.sh <test-name>
```

Then read:

- `tasks/compiler_tests/cases2/<test-name>/case.svelte`
- `tasks/compiler_tests/cases2/<test-name>/case-svelte.js`
- `tasks/compiler_tests/cases2/<test-name>/case-rust.js`

List the concrete mismatches between expected and actual output before editing code.

## 2) Diagnose the layer

Check the likely failure site in order:

1. parser / AST shape
2. analyze / side tables / classifications
3. transform
4. client codegen

Use `CLAUDE.md`, `CODEBASE_MAP.md`, and `phase-boundaries` to keep the fix in the correct crate.

## 3) Research before editing

If needed, inspect the corresponding reference compiler path in `reference/compiler/` and compare it with the matching Rust module. Focus on what behavior is missing, not on copying JS implementation patterns.

Use `svelte-reference-map` when you need the file mapping.

## 4) Implement the minimal correct fix

The plan text must include: **"Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries."**

- keep the change scoped to the named test unless investigation proves a shared root cause
- prefer existing accessors and enums over ad hoc conditions
- add or update unit tests if parser or analyze behavior changed
- never hand-edit `case-svelte.js` or `case-rust.js`

## 5) Verify narrowly, then slightly wider

Run:

```bash
just test-case <test-name>
```

Then run additional narrowly related verification, usually the closest crate tests or `just test-compiler` if codegen behavior changed broadly.

If the same approach failed three times, stop looping and report the blocker clearly.

## 6) Report

Provide:

- root cause
- changed layer and files
- commands run and outcomes
- any remaining follow-up work
