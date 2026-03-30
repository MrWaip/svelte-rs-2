---
name: status
description: Project status and next-step triage workflow. Use at session start or when the user asks what to do next, what is currently in progress, which ignored tests are easiest, or what the next roadmap/spec priority should be.
---

# Project Status

1. Read `specs/*.md` and extract `Current state` plus the next concrete action for each incomplete spec.
2. Scan ignored compiler tests in `tasks/compiler_tests/test_v3.rs`, group them by reason and effort, and highlight the likely quick wins.
3. Read `ROADMAP.md` and find the first unchecked item in the lowest active tier.
4. Read deferred work and a bounded set of `TODO` markers in `crates/`.
5. Return a short prioritized list of runnable commands:
   - `fix-test <name>`
   - `port specs/<name>.md`
   - `audit <feature>`
   - `improve <path-or-description>`

Keep output short and command-oriented.
