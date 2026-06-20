---
name: required
description: Use when starting any compiler task or session before touching parser/analyze/transform/ast/codegen/css/diagnostics code, before proposing a design or writing code, and when the task grows into a new layer and you need its PRD invariants.
allowed-tools: Read, Bash
---

# required

Read before touching the compiler. Goal: the PRDs your task touches land in context — by deterministic match, not by judging a filename.

`context.md` carries the glossary (the routing vocabulary). `map.md` indexes the code. Each PRD declares a `topics:` line — the canonical glossary terms it owns.

Process:

1. Read `context.md` and `map.md` in full.
2. **Name your task's terms.** List the domain terms your task involves, in glossary vocabulary. Unsure of the canonical term? A term's synonyms live in its glossary `_Avoid_` list — map through it.
3. **Resolve terms → PRDs by grep, never by filename.** For each term:

   ```
   grep -rin "^topics:" docs/*.md          # see every PRD's topics at once
   grep -ril "^topics:.*<term>" docs/*.md  # PRDs that own <term>
   ```

   Grep the **bare token**, not the punctuated form: `$`, `{`, `:` are regex metachars — match `derived` / `each` / `bind`, never `$derived` / `{#each}`. Over-matching is safe (read the extra PRD); a missed match is the failure.

   The `topics:` line is the relevance contract. A PRD matches or it doesn't — there is no "looks unrelated by its name."
4. **Read every matched PRD in full.** Then, before writing any code, list the PRDs you resolved (the commitment — skipping a matched PRD must be a stated decision, not a silent default). A term that matches no PRD's topics = a routing gap — say so out loud.
5. **Re-resolve when the task grows.** New term in scope → grep again → read the newly matched PRD.

The recurring failure this kills: glance at the index, judge a doc by its name, call it "not relevant," proceed to wrong conclusions. You no longer judge — you grep the `topics:` contract and read what matches.
