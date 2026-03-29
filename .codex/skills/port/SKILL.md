---
name: port
description: Spec-driven implementation workflow. Trigger when user asks to continue/complete work from `specs/<feature>.md`. Do not trigger when no spec exists or when task is only diagnostics/review.
---

# /port workflow (Codex)

## 1) Load spec and scope
Read `specs/<feature>.md` from top to bottom, starting with `Current state` and `Tasks`.
Implement only the next bounded slice unless user asks for full completion.

## 2) Cross-check architecture before coding
Use `AGENTS.md` and `CLAUDE.md` rules to place logic in parser/analyze/codegen correctly.

## 3) Implement incrementally
- Apply small, reviewable changes per subtask.
- Keep diffs tight and avoid unrelated cleanup.

## 4) Validate with project commands
Run the narrowest meaningful sequence, usually:
```bash
just test-case <affected-case>
just test-compiler
```
Escalate to `just test-all` when cross-crate behavior changed.

## 5) Update tracking
Update the spec `Current state` and completed task checkboxes for work actually finished.
If scope changed, note deferred items explicitly.
