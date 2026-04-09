---
name: status
description: Project status and production-oriented next-step triage workflow. Use at session start or when the user asks what to do next, what is currently in progress, which skipped or ignored test clusters should be closed next, which spec should be advanced toward completion, what architecture or boundary review is most valuable now, or what roadmap/spec priority best moves the project toward production readiness.
---

# Project Status

1. Read `specs/*.md` and extract `Current state` plus the next concrete action for each incomplete spec.
2. Scan skipped or ignored compiler tests in `tasks/compiler_tests/test_v3.rs`, group them into related feature or architecture clusters, and identify which cluster is the most meaningful next production milestone.
3. Read `ROADMAP.md` and find the next incomplete item that best advances production readiness, not the smallest or easiest remaining item.
4. Read deferred work and a bounded set of `TODO` markers in `crates/`, focusing on architectural debt, missing infrastructure, or review-worthy boundary risks.
5. Prefer larger coherent work packages over isolated quick wins when they can be completed systematically without shortcuts.
6. Return a short prioritized list of runnable commands:
   - `fix-test <name>`
   - `port specs/<name>.md`
   - `port2 specs/<name>.md`
   - `audit <feature>`
   - `improve <path-or-description>`
   - `review-boundaries`
   - `review-simplify`

Do not optimize for the easiest passing test. Optimize for the next useful block of work that reduces real production risk or closes a meaningful feature gap.

Keep output short and command-oriented.
