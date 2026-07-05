---
name: dig-better
description: 🧩 PARITY · Turn one divergence from the Original into a test per affected case (does not fix).
disable-model-invocation: true
allowed-tools: Bash, Read, Write, Edit, Grep, Glob, mcp__narsil-mcp__*, Agent
---

# dig-better

Find the one place in the Original that caused a divergence, discover everything else in that same area that also diverges, and leave a test on disk for each case — without fixing the compiler.

## Read first (every time — don't skip because it's "already in session")

- `docs/context.md` §«Language» — the real project vocabulary (Original, parser / analyze /
  transform / codegen) and the layer dogmas (smart analyzer, dumb codegen).
- If the area belongs to a cluster with a root PRD (CLAUDE.md «Architectural root PRDs»),
  read it — its invariants may mean the fix is bigger than a local tweak.

## When not to use

- One-off parity check on a scratch component, no persistent tests wanted → `/quick-check`.
- Only the list of what mismatches, with no cluster to pin yet → `just sweep-run <dir> --dry-run` (`--ssr` for server output).
- A brand-new feature test with no Original divergence to chase → `/add-test` or
  `/add-diagnostic-test`.

## Steps

1. **Reproduce what the user gave you.** The input is whatever they hand over — a file, inline
   source, a path, a `just sweep-run` you kick off, a sweep log already produced, a single diff.
   Normalize it to one diverging FILE and read its actual outcome side by side with the
   reference — the wrong JS, the panic, the missing/extra diagnostic, the CSS.
   - one file or inline source → `/quick-check`
   - only a directory in hand → `just sweep-run <dir>`, then `/quick-check` the file it flags.
   - a **server (SSR) divergence** → hold the generate axis: `just sweep-run <dir> --ssr`,
     then `/quick-check <file> --generate=server`.
   - **several divergences at once** (a sweep log, a directory) → before digging, pick ONE
     cluster: scan for a shared root across the mismatches, then take the one that covers the
     most. That file is your input.

   Build everything from the input FILE, not from the sample's name.
   **Done when** you have both outputs in front of you in the divergence's generate mode and
   can say exactly what differs.

2. **Follow the divergence to the Original.** Find the ONE decision in `original/compiler/`
   whose logic produced the difference. The branch is not always an `if` / `switch` / early
   return — often it is a **strategy**: which function, visitor, or lambda gets selected,
   wrapped, nested, or substituted for this input is itself the branch. Read how that decision
   is composed, not just the line that fired.
   For a **server (SSR) divergence** the deciding code lives in the Original's *server*
   transform (`phases/3-transform/server/`), a visitor set wholly separate from the client one
   — follow it there, not into the client transform.
   **Done when** you can paste the deciding code (`file:line` + the real body) and name the
   finite set of branches it chooses between — explicit conditionals or swapped strategies
   alike. Never conclude from a name or from memory.

3. **Collect every case in that area — this is the point of the skill.** The divergence the
   user hit is one branch of a finite set: cover every branch and you are done, miss one and
   it stays broken. Turn each branch from step 2 into a case, plus the edge cases and
   options the Original handles there. Add the cases that already match and MUST stay matching
   after a fix (`_guard` cases). For recursive structures (destructuring patterns, nested
   nodes) — one case per form; a single shallow case does not cover a tree.
   **Done when** every branch from step 2 maps to a case in your list, plus the guard cases.

4. **Write a test for every case — do not skip this.**
   Go through the sanctioned path — that is where the mechanics live, and they are easy to get
   subtly wrong from memory. Do not hand-roll a case:
   - compiler-output cases → `/add-test`
   - diagnostic cases → `/add-diagnostic-test`

   Run each case in the same mode/options — including the **generate axis** (client vs
   server) — as the divergence, or its red/green means nothing. A server divergence's red
   lives in the `::ssr` / `::ssr_dev` variant, which `/add-test` ships `#[ignore]`d by default
   — activate it there, or the red never shows and you misread it as "nothing to fix".
   Validate every green before trusting it: both sides non-empty, and the case actually
   exercises its branch — an input the compiler trivializes (a const-folded value, a
   statically-known condition, a binding that never updates) matches for the wrong reason.
   Force the branch live, or its green proves nothing.
   **Done when** every branch has a real case on disk that is registered, runs, and shows
   red = the divergences / green = the guards. A table in chat is not the artifact — a case
   that isn't registered and run does not exist.

5. **Summarize — short, plain, free-form.** In Svelte and behavior terms only — no internal
   jargon, and never make a function or type name the subject of a sentence (the user
   reads behavior, not identifiers): what the user pasted + options ·
   what our output does wrong versus the Original, as the running component would behave · how
   the Original does it (narrate the mechanism, no `file:line`) · the tests you created and
   which are red vs green. If the fix looks bigger than a local tweak (touches several crates
   or needs machinery that isn't there yet), say so in one plain sentence.
   **Done when** the summary names the tests that now exist on disk — which you cannot write
   honestly if step 4 was skipped.

## Sub-agents — mcp-first

If you dispatch a sub-agent to explore the Original or our code, tell it to use narsil-mcp
(sub-agents don't inherit the project hook; they load schemas via `ToolSearch` first). Ask
for facts backed by `file:line` + the real body — never for a conclusion.
