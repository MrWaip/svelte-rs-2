---
name: explain-test
description: Read-only workflow for explaining what a compiler test case covers and why it passes or fails. Use when the user asks what a test does, what feature it exercises, or why it is failing. Do not trigger when the user wants the fix implemented immediately.
---

# Explain Test

1. Validate that `tasks/compiler_tests/cases2/<name>/` exists.
2. Read `case.svelte`, `case-svelte.js`, and `case-rust.js` if present.
3. Run `just test-case <name>` if current status is unknown.
4. List the Svelte features exercised by the input.
5. If failing, classify meaningful mismatches by parser, analyze, transform, or codegen.
6. Show the likely code path in our compiler that handles those features.
7. Recommend the next command, usually `fix-test <name>` or `port specs/<feature>.md`.

Read-only by default: do not edit files in this workflow.
