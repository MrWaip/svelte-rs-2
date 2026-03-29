---
name: review-boundaries
description: Boundary and data-flow review workflow. Trigger for architecture hygiene reviews across parser/analyze/codegen ownership. Do not trigger for stylistic cleanup or simplification-only passes.
---

# /review-boundaries workflow (Codex)

1. Define scope (file/path/crate or default codegen+transform).
2. Review against boundary rules from `CLAUDE.md` and `AGENTS.md`.
3. Find and group violations by root cause (not per-line spam).
4. For each finding, include evidence, impact, and exact upstream-first fix strategy.
5. Save/report findings in a structured review note (e.g., `BOUNDARY_REVIEW.md`).

Focus on ownership/data-flow correctness, not formatting or micro-optimizations.
