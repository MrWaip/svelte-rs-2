---
name: excavate
disable-model-invocation: true
---

# Excavate

Picks an unchecked use case in a Svelte compiler spec under `specs/` and closes
the whole domain unit behind it — the coherent family the gates reveal, not just
the failing case.

ARG: spec path or feature description that maps to a spec.

## Invariants
- No plan until research reaches a compiler-layer boundary and names the origin.
- Fix the origin, not a symptom.
- Scope is the whole domain unit, solved completely — not a one-case patch.
- Too big = won't close in one session before context fills and quality drops. Then don't split or narrow yourself — surface it, the user sets scope. Decompose only along independent seams; if it won't split cleanly, say that.
- No edits until an explicit go-signal. Agent questions aren't approval.
- Close means green: `just test-compiler`, `just test-diagnostics`, `just lint` — with the case un-ignored and actually running. A skipped/ignored test is not a pass.

## Flow

**Frame.** Read ARCHITECTURE.md and CONTEXT.md if not in session. Read the spec, pick the next `[ ]`. Use narsil-mcp for search, not grep.

**Failing artifact.** Reuse a /diagnose case if present; else create one with `/add-test` (or `/add-diagnostic-test` for diagnostics). Confirm divergence or panic.

**Anchor.** Invoke `domain-anchor` (Agent tool) with only the verbatim `[ ]` quote and the divergence. It rules on layer and carrier.

**Origin.** Walk up from the divergence until you can state: the origin layer, the file:line where the bad value first appears, and why it's the origin not a symptom. Disagree with the anchor on layer → escalate.

**Sibling audit.** Invoke `sibling-auditor` (Agent tool) with only the emit/dispatch function from the origin. Verdict is binding: paste the table verbatim into the plan; every `divergence` row becomes an included or excluded use case, each backed by a quote from the auditor — never bare "out of scope".

**Coverage gap.** Find the Original's handler for this origin in `original/compiler/`. Diff what it covers against ours. That gap plus the sibling-audit family is the domain unit to close.

**Plan.** Produce a closure plan per `plan-template.md`.

**Self-critique.** Once, before presenting. Revise if either fails:
- **Parity** (fills the plan's parity-check slot): full parity for the whole domain unit, not just the failing input? If the fix lands below analyze, can analyze precompute it?
- **Invariants**: does the design hold against CONTEXT.md's anti-patterns — id-only resolution (never name strings), smart-analyzer/dumb-codegen, no emit-shaped semantics, no codegen-side analysis?

**Approval.** Present and wait. User may question (revise, re-present), widen scope (re-run sibling-auditor on the new function first), or challenge layer/carrier (re-run domain-anchor first). Proceed only on an explicit go-signal.

**Implement.** Apply approved spec edits, then code in layer order parser → analyze → transform → codegen. Only what the plan covers. Run the case after each layer.

**Capture.** For every excluded item and every unincluded `divergence` row: find its owning spec (this spec, or a sibling named in Source / Related). If no matching `[ ]` already exists there, add one with the auditor's verbatim wording + Original quote + file:line. No deferrals to debt.md.

**Finalize.** Flip `[ ]` → `[x]` only for use cases with passing tests. Update counters and Last updated. The three `just` checks green.
