---
name: status
description: Session triage workflow. Trigger when the user asks for project status, next priorities, or what to do next. Do not trigger when the user already requested a concrete implementation/fix task.
---

# /status workflow (Codex)

1. Read `specs/*.md` and extract each spec's `Current state` and next action.
2. Scan ignored compiler tests in `tasks/compiler_tests/test_v3.rs` (`#[ignore = ...]`) and list likely quick wins.
3. Read `ROADMAP.md` and identify the first unchecked item in the lowest active tier.
4. Return a prioritized list of concrete next commands, preferring:
   1) quick test fixes,
   2) in-progress specs,
   3) next roadmap item,
   4) debt cleanup.

Keep output short and command-oriented.
