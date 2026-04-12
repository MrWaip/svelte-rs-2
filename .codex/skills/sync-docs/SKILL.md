---
name: sync-docs
description: Synchronize project documentation with verified code changes. Use after implementation when `ROADMAP.md`, `CODEBASE_MAP.md`, `GOTCHAS.md`, or relevant specs need to reflect the current code and test reality. Do not trigger before verification or as a substitute for implementation.
---

# Sync Docs

1. Inspect recent commits and changed files to understand what actually changed.
2. Inspect `tasks/compiler_tests/test_v3.rs` and `tasks/diagnostic_tests/test_diagnostics.rs` before editing specs. Treat registered cases there, including `#[ignore]` reasons, as the authoritative test inventory for compiler and diagnostic parity coverage.
3. Reconcile relevant `specs/<feature>.md` files against that inventory. Add missing existing tests to the spec's test inventory/reference sections, note ignored cases when they explain remaining gaps, and add unchecked use cases when tests expose coverage the spec does not yet track.
4. Update `ROADMAP.md` only where completion is evidenced by code and passing tests.
5. Update `CODEBASE_MAP.md` for new or changed `pub` and `pub(crate)` APIs.
6. Update `GOTCHAS.md` only for real, non-obvious pitfalls learned from implementation.
7. Update relevant `specs/<feature>.md` sections, especially `Use cases`, `Test cases`, and the terse `Current state` resume header.

Do not rewrite large doc sections unnecessarily; keep edits precise and evidence-backed.
Do not turn `Current state` into a changelog with dated `Completed (...)` or `Confirmed gap (...)` bullets.
