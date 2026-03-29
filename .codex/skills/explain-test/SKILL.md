---
name: explain-test
description: Read-only test explanation workflow. Trigger when user asks what a test does or why it fails. Do not trigger when user asks to implement a fix.
---

# /explain-test workflow (Codex)

1. Validate test case exists in `tasks/compiler_tests/cases2/<name>/`.
2. Read `case.svelte`, `case-svelte.js`, and `case-rust.js` (if present).
3. Run `just test-case <name>` to confirm current status.
4. Summarize exercised features and (if failing) mismatch classification by layer.
5. Return likely code paths to inspect next and recommended follow-up command.

Read-only by default: do not edit files in this workflow.
