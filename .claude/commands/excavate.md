---
name: excavate
description: Manually invoked command to close a Svelte compiler spec use
             case. Before writing a plan, mandates backward research to a
             compiler-layer boundary and a reference coverage diff at that
             boundary, so the family of a bug is visible from the first
             instance instead of patched one symptom at a time.
disable-model-invocation: true
---

# Excavate

Closes one unchecked use case in a Svelte compiler spec under `specs/` — or
a tightly cohesive cluster if they close together.

ARG: spec path or feature description that maps to a spec.

IRON LAW
  No plan without research reaching a compiler-layer boundary.
  No plan without a reference coverage diff at that boundary.
  No plan without a `domain-anchor` verdict on layer and carrier.
  No scope expansion without user approval.

────────────────────────────────────────
1. Context
   Read ARCHITECTURE.md and CONTEXT.md if not in session.
   Read the spec. Pick the next [ ] or a small cluster.
   Use narsil-mcp for code search, not grep.

2. Closure selection
   Selected unit must close at least one use case completely.
   Too broad → decompose the spec into smaller [ ], STOP.

3. Failing artifact
   If /diagnose already created a test case, use it.
   Otherwise add `tasks/compiler_tests/cases2/<name>/case.svelte` and an
   entry in `test_v3.rs`. Run `just generate` for `case-svelte.js`.
   Run our compiler and confirm divergence or panic.

4. Domain anchor
   Invoke `domain-anchor` via the Agent tool (`subagent_type=domain-anchor`).
   Prompt: only `bullet_text` (verbatim [ ] quote) and `divergence`
   (input .svelte + reference .js + our .js, or test case path).
   No file paths, no pre-cooked classification.
   Wait for the tool result before proceeding. Verdict is binding.

5. GATE 1 — Backward research
   Walk upward from the divergence in `case-rust.js` until you can answer:
     - origin layer: parser | analyze | transform | codegen
     - file:line where the bad value first appears
     - why it is the origin, not a downstream symptom
   Any answer missing → gate closed, no plan.
   Disagrees with anchor verdict on layer → escalate.

6. GATE 2 — Reference coverage gap
   Locate the reference handler equivalent to the origin from gate 1 in
   `reference/compiler/`. List the cases it covers there. List the cases
   our origin covers. Compute the diff.
     empty diff     → narrow
     non-empty diff → family visible, members named explicitly

7. GATE 3 — Family triage
   narrow                 → 1 test, 1 fix, 1 [x]
   family, members named  → propose N tests + N [x], requires user approval
   family, scope unclear  → STOP, escalate
   Default narrow. No silent widening.

8. Plan
   Present a closure plan including:
     - Architect summary (3 lines max, plain language, no file paths or
        carrier vocabulary): what changes, in which layer, alternatives
        with one-line trade-off each.
     - `domain-anchor` verdict: verbatim quote from step 4 covering layer,
        domain unit, carrier, gap kind, prescribed action. Plan without
        this field is incomplete.
     - included use cases, excluded use cases, owning layer
     - files expected to change
     - root cause: one sentence
     - why root, not symptom: one sentence
     - closure condition

9. Parity check
   Critique the plan from step 8: does it reach 100% parity with the
   reference compiler for the domain unit, not just the failing input?
   Nth fix of the same shape across carriers → revise plan to consolidate.
   No deferrals to debt.md. One refinement loop max — then user approval.

10. Implementation
    Apply approved spec edits first. Then code, layer order
    parser → analyze → transform → codegen. Only what the plan covers.

11. Verify
    Diff matches the approved plan.
    Run `just test-compiler`, `just test-diagnostics`, `just lint`. Must be green.

12. Finalize
    Flip [ ] → [x] only for use cases with passing tests.
    Update Working/Tests counters and Last updated.
    Append newly discovered [ ] members if any.
    Discovered parity gaps go to the spec as `[ ]`, never to debt.md.

────────────────────────────────────────
If you reach for a match-arm patch, a special-case branch, or "make this
match reference here" — a gate failed. Return to gate 1.
