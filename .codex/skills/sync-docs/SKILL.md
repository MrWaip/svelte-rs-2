---
name: sync-docs
description: Synchronize project documentation with verified code changes. Use after implementation when `ROADMAP.md`, `CODEBASE_MAP.md`, `GOTCHAS.md`, or relevant specs need to reflect the current code and test reality. Do not trigger before verification or as a substitute for implementation.
---

# Sync Docs

1. Inspect recent commits and changed files to understand what actually changed.
2. Update `ROADMAP.md` only where completion is evidenced by code and passing tests.
3. Update `CODEBASE_MAP.md` for new or changed `pub` and `pub(crate)` APIs.
4. Update `GOTCHAS.md` only for real, non-obvious pitfalls learned from implementation.
5. Update relevant `specs/<feature>.md` sections, especially `Current state` and completed use cases.

Do not rewrite large doc sections unnecessarily; keep edits precise and evidence-backed.
