# Grounding at boundaries (shared by all PARITY skills)

Single source of truth for the discipline every PARITY phase (`/dig`) runs at its
**boundaries**. Edit here once — every skill that points at this
file inherits the change. Sibling of `parity-language.md`: that one governs *how you talk to the
user*; this one governs *what must be true before you stop, decide, or hand off*.

## The rule

**No load-bearing assumption crosses a boundary unproven.**

A boundary is any of: asking the user a decision, declaring a phase done / settled, or handing off
to the next skill (the `Next:` line). The failure this prevents: a phase looks complete, but a fact
it silently rests on was never checked — and the handoff carries it forward, where it detonates one
or more phases later.

## Why this exact shape, not "be careful"

Telling a model to re-read its own work and "verify" — intrinsic self-correction — does **not** work:
across studies it degrades or fails to help, and models miss roughly two-thirds of their own errors.
The gains appear only when the check is against **external ground truth** — running code, the actual
source, a written invariant. On frontier models (Opus 4.6+) this matters *more*, not less: they
self-verify better, but when wrong they are more *convincing*, so a soft self-check rubber-stamps a
plausible-but-wrong answer exactly when the stakes are highest. The gate below is external-grounding
and adversarial by construction — never "look again."

## The gate — run at every boundary

1. **Enumerate.** Name each fact the output rests on — the ones that, if false, collapse it. The
   decision the whole phase rests on (the dispatch axis, the carrier, the scope verdict) comes first.
2. **Anchor each to external ground truth.** A fact counts only with one of: the actual code at
   `file:line` (a **pasted body**, never a symbol's name or your memory), a **run** result (a probe /
   `quick-check` / suite output), or a **written invariant** in the project docs. No anchor → it is
   an **open question**, not a decision.
3. **Certify by disproof, not confirmation.** For the load-bearing fact, actively hunt the
   counterexample: the case it can't handle, the stage where it's absent, the invariant it breaks. A
   hunt that fails certifies; "looks right" does not.
4. **Fresh context for the load-bearing check.** Same-context review inherits the wrong framing.
   Where the whole output rests on one decision, verify it in a **fresh context** — a sub-agent told
   to assume it's wrong until proven from its own evidence (the skeptic-subagent pattern).
5. **Open ≠ decided.** Any unproven assumption is surfaced as open — never laundered into a user
   choice, never handed off. The boundary does not open until the fact is proven or the user
   explicitly defers it.

## Anti-ritual

The gate is satisfied only by **pasted external evidence**. "✅ grounded" / "verified" / "confirmed"
with no `file:line` and no run output is the ritual that looks like the gate and isn't — it is
exactly the self-check the research says fails. If you cannot point to a body or a run, you have not
done it.
