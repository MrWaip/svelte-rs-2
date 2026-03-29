---
name: review-simplify
description: Behavior-preserving simplification review workflow. Trigger when code became hard to read after multiple changes and user wants simplification opportunities. Do not trigger for ownership violations (use `review-boundaries`).
---

# /review-simplify workflow (Codex)

1. Define scope (crate/path or default codegen+transform).
2. Scan for repeated decision logic and overly complex branching.
3. Prioritize simplifications that improve readability without changing output.
4. Apply a balance check (avoid over-abstraction/god-functions).
5. Produce a ranked, actionable report (e.g., `SIMPLIFY_REVIEW.md`).

Rules:
- preserve behavior exactly
- prefer explicit readable code over clever abstractions
