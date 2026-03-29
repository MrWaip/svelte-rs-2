---
name: fix-test
description: Single failing compiler test fix workflow. Trigger when one named test case must be diagnosed/fixed. Do not trigger for multi-test audits or broad feature completeness reviews.
---

# /fix-test workflow (Codex)

## 1) Reproduce
Run:
```bash
.codex/scripts/fix-test.sh <test-name>
```
Or:
```bash
just test-case-verbose <test-name>
```

## 2) Diff-driven diagnosis
Read:
- `tasks/compiler_tests/cases2/<test>/case.svelte`
- `tasks/compiler_tests/cases2/<test>/case-svelte.js` (expected)
- `tasks/compiler_tests/cases2/<test>/case-rust.js` (actual)

Classify mismatch origin: parser, analyze, transform, or codegen.

## 3) Implement minimal fix in the right layer
- Prefer existing helpers/accessors.
- Add/adjust tests only for the changed behavior.
- Never hand-edit `case-svelte.js` or `case-rust.js`.

## 4) Verify
Run:
```bash
just test-case <test-name>
```
Then run additional narrowly related tests.

## 5) Report
Provide:
- root cause summary
- files changed
- commands run + outcomes
- any remaining follow-up work
