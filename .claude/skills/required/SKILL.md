---
name: required
description: Routes a compiler task to the PRDs that own its terms through the `topics:` contract. Use at the start of any compiler task, before touching parser/analyze/transform/ast/codegen/css/diagnostics code or proposing a design, and when the task grows into a new layer.
allowed-tools: Read, Bash
---

# required

Read before touching the compiler: the PRDs your task touches land in context by **matching the `topics:` contract**, never by judging a filename.

`context.md` is the glossary — your routing vocabulary. `map.md` indexes the code. Each PRD's `topics:` line lists the canonical glossary terms it owns.

1. **Read `context.md` and `map.md` in full** — the vocabulary the rest of this runs on.
2. **Name your task's terms** in glossary vocabulary. Unsure of the canonical term? Map through the term's `_Avoid_` synonyms.
3. **Match each term against `topics:`, not filenames.** `grep -il "^topics:.*<stem>" docs/*.md` — grep the bare stem (`derived`, `each`, `bind`), never the punctuated form (`$derived`, `{#each}`). Over-matching is free; a missed match is the failure. A term that matches no PRD is a routing gap — say so.
4. **Read every matched PRD in full, then list them.** Skipping a match is a stated decision, never a silent default.
5. **Re-match when the task grows.** New term in scope → grep again → read the new match.
