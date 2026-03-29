---
name: diagnose
description: Component diagnosis workflow. Trigger when user provides a component/path and asks what is broken and where. Do not trigger when a single failing test already isolates the issue (use `fix-test`).
---

# /diagnose workflow (Codex)

1. Reproduce with a temporary or existing focused compiler test case.
2. Compare `case-svelte.js` (expected) vs `case-rust.js` (actual).
3. Classify each mismatch by layer: parser / analyze / transform / codegen.
4. Produce a fix plan ordered by dependency (upstream before downstream).
5. Convert major mismatches into focused test cases if missing.

Rules:
- never hand-edit generated `case-svelte.js` / `case-rust.js`
- keep diagnosis and implementation separated unless user explicitly asks to fix now
