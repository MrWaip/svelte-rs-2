---
name: audit
description: Feature completeness audit workflow. Trigger when user asks what is missing for a feature versus reference behavior. Do not trigger when user asks to directly implement a known fix.
---

# /audit workflow (Codex)

1. Load matching `specs/*.md` if it exists; continue from `Current state`.
2. Compare reference behavior (`reference/compiler/`) vs current Rust implementation.
3. Classify each use case as Covered / Partial / Missing / Unknown.
4. Add or propose focused tests for Missing/Unknown cases (without editing generated snapshots manually).
5. Produce a prioritized fix order and point to next commands:
   - `/fix-test <name>` for focused failures
   - `/port specs/<name>.md` for broader infra gaps

Keep audit read-focused; do not mix large implementation work into this step.
