---
name: dig
description: 🧩 PARITY · Use when a sweep finding or a pasted Svelte component diverges from the Original (wrong JS, panic, missing/extra diagnostic, CSS mismatch) and you need to scope the full fix before any code. Also when asked to investigate a parity gap.
allowed-tools: Bash, Read, Write, Edit, Grep, Glob, mcp__narsil-mcp__*, Agent
---

# dig

Scope a parity divergence — stop at a design gate before any **fix code** (parser/analyze/transform/codegen src). Probes are scope, not code — materialize them.

## Core principle

Unit of work = **one Original decision-fork**. Its branches are finite and enumerable → covering every branch is a completion guarantee. The finding is a **handle**, not the work-item — climb to the decision.

## Required reading (hard-require, step 0)

Read both before anything else, every invocation:

- `.claude/skills/_shared/parity-language.md` — how to talk to the user (domain terms only, internal vocabulary banned). Governs the Gate-1 message.
- `.claude/skills/_shared/grounding.md` — the boundary-grounding gate (no load-bearing assumption crosses a boundary unproven). Governs the A/B verdict and Gate-1: the verdict is a load-bearing fact, prove it by pasted `file:line` / run, never by a symbol's name or memory.
- `docs/context.md` — crate layers, principles (smart analyzer / dumb codegen, no analyze-in-codegen, identity by id); per-layer invariants in each layer's root PRD.
- `docs/context.md` §«Language» — ubiquitous language (Original, fork-set, deep-modules, HITL/AFK, tracer-bullet).
- If finding touches a cluster from CLAUDE.md «Architectural root PRDs» — read that root PRD. Its invariants narrow the fork-set and force a **Need research** verdict on violation.

Do not skip on the assumption "already in session" — re-read.

## When NOT to use

- Ad-hoc one-off parity check on a pasted/scratch component without a persisted finding → `/quick-check`.
- Directory-level survey of mismatches → `just sweep-run <dir> --dry-run`.
- Feature-first new test without a referenced Original divergence → `/add-test` or `/add-diagnostic-test`.

## Input

- **directory** → `just sweep-run <dir>` (stop-on-first). The `MISMATCH: <file>` line is the finding; patch + `options:` = observed divergence.
- **file** → `just quick-check <file> --print=both`.
- **inline** → write to scratch, then `quick-check`.

`just sweep-run <dir> --dry-run` is a separate survey, NOT dig.

## Flow

1. **Observe first.** Read the actual outcome from Input (panic / diagnostic / diff). Build the probe from the finding FILE, never the sample name.
2. **Handle → Original decision.** Find the ONE decision in `original/compiler/` the finding exercised; read its branch structure.
3. **Red-all from the Original's branches — REQUIRED before A/B and the gate.** Persist one case per branch under `cluster_cases/<feature>/` (or `diagnostic_tests/cases/`) via `just generate` (this writes the Original reference to disk). No `just generate` run ⇒ no red-all — a `/tmp` quick-check is step-1 observe, not this. Run → red/green table. Probes are the computed scope artifact — the gate shows this table. Fork-set from the Original's branches — NOT your memory, NOT our code's siblings. Always include **green guards** for must-not-change branches. Do not defer this step "pending go" — go is for fix code, not for scope.

   **Validate every signal before trusting it.** A red/green is evidence only if the comparison is real: both sides non-empty; the probe compiled in the **same mode/options as the divergence** (legacy needs `config.json {"runes": false}`; match experimental flags); and each green-guard binding is actually reactive (`$state` with no mutation const-folds → meaningless green — add a mutation). A green from empty output or the wrong mode is a **false negative**: confirm with `--print=both` + quick-check exit code, never a grep/`sed` slice. This check belongs in the harness — automate it when you next touch the harness, don't carry it as ritual prose forever.
4. **Quick fix / Need research verdict — the orchestrator concludes from neutral research, never from its own hunch.** **Quick fix** = machinery exists, extend → fix the in-session cluster directly. **Need research** = net-new, or touches crate boundaries / data structures → STOP at the gate, settle the architecture first. Do **not** form a verdict and then hunt for support — that is exactly how a name (`..._legacy_..._exit`) becomes a false "net-new". The three diamonds are factual questions; gather grounded facts first, then conclude.

   **Gather facts — neutral research, no verdict.** Dispatch research sub-agent(s) with a **neutral** ask — "what does the `<feature>` / assignment path do today, and where? paste the deciding code with `file:line`" — never a leading one ("confirm there's no machinery"). The researcher returns **facts only**: every claim backed by `file:line` + the actual body, and **no verdict, no "net-new", no A/B** — those are the orchestrator's call, not the researcher's. (mcp-first — sub-agents don't inherit the project hook; tell them to use narsil-mcp, schemas via `ToolSearch`. Don't fan out a workflow when a few targeted reads suffice.)

   ```dot
   digraph ab_verdict {
       "Observed divergence" [shape=doublecircle];
       "Touches crate boundary or module dep?" [shape=diamond];
       "Adds or changes a shared data structure?" [shape=diamond];
       "Net-new machinery (no existing wire)?" [shape=diamond];
       "Need research — STOP at gate · settle architecture first" [shape=box];
       "Quick fix — wire/extend · fix in-session" [shape=box];

       "Observed divergence" -> "Touches crate boundary or module dep?";
       "Touches crate boundary or module dep?" -> "Need research — STOP at gate · settle architecture first" [label="yes"];
       "Touches crate boundary or module dep?" -> "Adds or changes a shared data structure?" [label="no"];
       "Adds or changes a shared data structure?" -> "Need research — STOP at gate · settle architecture first" [label="yes"];
       "Adds or changes a shared data structure?" -> "Net-new machinery (no existing wire)?" [label="no"];
       "Net-new machinery (no existing wire)?" -> "Need research — STOP at gate · settle architecture first" [label="yes"];
       "Net-new machinery (no existing wire)?" -> "Quick fix — wire/extend · fix in-session" [label="no"];
   }
   ```

   **Conclude — orchestrator only.** From the reported facts (not a function's name, not memory) answer each diamond; every "yes"/"no" must point to a fact with `file:line` — only a pasted body counts as evidence. Any "yes" → **Need research**. All three "no" → **Quick fix**. If a diamond can't be answered from the facts, the research is incomplete — ask again (neutral), don't guess.
5. **Gate-1 — plain domain language, no internal vocabulary** (per `_shared/parity-language.md`). Present facts, not a chosen fix: the Svelte input + options · our output ↔ Original output · what goes wrong in the running component (behavior) · how the Original does it (narrated mechanism, no file:line). State the verdict in plain terms — "scope is clear, ready to fix" or "new machinery / crosses boundaries — settle the architecture first". For the second, add **the pieces**, **how we'd split it**, and **where you'd decide vs. what runs unattended**. Banned from this message: Quick fix / Need research, fork-set, red-all, probes, green guard, deep-modules, blast radius, HITL/AFK, leaf, lowering — say the behavior, not the label. Wait for explicit go.

## Sub-agent dispatch — mcp-first

Sub-agents don't inherit the project's "use mcp" hook. Every dispatch prompt MUST tell them to use narsil-mcp for code search (load schemas via `ToolSearch` first).

## Worked example

For a concrete walkthrough of fork-set → red-all → A/B gate on one Original decision, see `examples/fork-set-example.md`.

## Handoff

- **Quick fix** → fix the in-session cluster directly — scope is clear.
- **Need research** → settle the architecture first, then implement. dig does NOT settle the design — it stops at the gate with the pieces, the split, and the decide-vs-unattended call laid out.

## Next-step line

- Quick fix: `Next: fix the cluster — scope is clear.`
- Need research: `Next: settle the architecture for <slug>, then implement.`

## Rationalizations

| Excuse | Reality |
|---|---|
| Sub-agent says "new machinery" | Absence-lists over-scope. Run our compiler + read the actual path first. |
| "These variants probably cover it" | Memory rots. Criterion: every Original branch ↔ a probe. |
| "Clearly a Quick fix, just wire it" | Hypothesis. A shared arm you reuse can carry a wrong clause. |
| "The finding is the work-item" | It's a handle. Fix the origin, not the symptom. |
| "It's green, so it's covered" | A green can be empty output or the wrong compile mode. Validate the signal (non-empty, mode matches, guard actually reactive) before it enters the table. |
| "Need research / fork-set are the skill's own words" | They're yours, not the user's. The gate is domain language — translate per `_shared/parity-language.md` or the user burns tokens decoding it. |

## Red flags — STOP

- Probe from sample name, not file.
- Architecture claim before observing actual output — or an A/B verdict read off a symbol's **name** instead of its body.
- Trusting a red/green without confirming the signal is real (non-empty both sides, mode/options match the divergence, green-guard binding actually reactive).
- Recursive AST shape (pattern / nested tree) covered by a single shallow probe — enumerate a probe per recursion shape.
- Fork-set from our code, not the Original's branches.
- Gate shows internal types / a chosen fix.
- Gate uses internal vocabulary (Quick fix / Need research, fork-set, probes, deep-modules, blast radius, HITL/AFK) instead of domain/behavior terms — see `_shared/parity-language.md`.
- A function or type name as the subject of a gate sentence (allowed only inside a pasted snippet).
- "Quick fix" used to skip the gate when it touches crate boundaries / data structures.
- Red-all "done" with no `cluster_cases/<feature>/` files on disk — a chat table isn't the artifact. Skipping red-all materialization "pending go" — probes are scope, not fix code.
