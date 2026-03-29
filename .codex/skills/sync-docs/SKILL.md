---
name: sync-docs
description: Documentation sync workflow. Trigger after verified code changes when docs/specs must reflect current behavior. Do not trigger before tests/verification or as a substitute for implementation.
---

# /sync-docs workflow (Codex)

1. Inspect recent commits and changed files.
2. Update `ROADMAP.md` only where completion is evidenced by code/tests.
3. Update `CODEBASE_MAP.md` for new/changed public or `pub(crate)` APIs.
4. Update `GOTCHAS.md` only for real, non-obvious implementation pitfalls.
5. If spec progress changed, update `specs/<feature>.md` `Current state` and completed checkboxes.

Do not rewrite large doc sections unnecessarily; keep edits precise and evidence-backed.
