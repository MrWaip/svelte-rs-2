---
name: writing-docs
description: PRD editorial standard for docs/. Use when writing a new PRD, extending one, or reviewing changes under docs/ — and when another skill needs the PRD rules.
allowed-tools: Read, Edit, Write, Grep, Bash
---

# writing-docs

A PRD records a layer's **invariants** — what stays true across every implementation of that layer. This skill is the standard that keeps a PRD worth reading: **durable**, **load-bearing**, one fact in one place. A PRD is not an API reference, not a code map, not a changelog, not a task tracker.

The agent's default is to write down whatever seems useful *right now*. But a stale or duplicated line is not free storage — it is a **distractor** that pulls the model off the invariant that matters. Two gates and one home rule replace that default.

Pick your branch:

- **New PRD** → the Anatomy template, every line through The two gates.
- **Extending a PRD** → run The two gates on what you add; before inserting, check Single home (the fact doesn't already live elsewhere).
- **Reviewing `docs/` changes** → Reviewing a diff.

## The two gates

A line earns its place only if it passes **both** gates. If it fails, delete it — don't reword it.

1. **Durable, not status.** The line is true independently of the current code. A sentence a merged PR could falsify is status, and status has another home. Fails the gate: line numbers (`lib.rs:108`), "current state / first vertical slice / WIP / TODO / stub / filled in by later slices / awaiting `/audit`", percent-done, "not yet implemented", inventories of what already emits vs. what will. Write the target invariant — true in six months ("the template is emitted here, the script in the transform") — not a snapshot of progress ("only static attributes are emitted so far"). Partial coverage of a feature goes in an issue link, never an inline slice list.

2. **Load-bearing.** The line changes a decision when writing or reviewing code. Test it: delete the line — does any implementation choice change? No → it's a **no-op** (a default the agent already honors: "code must be correct", "follow the dogmas") — cut it. A weak invariant is sharpened into a checkable one, not kept for volume. A checkable invariant carries its mechanical check where one exists: "`grep Emit` over the crate is empty", "enforced by the `Node` enum macro". Where none exists, anchor it with one canonical example, not an enumeration of every form — an example is a compressed check read at a glance; the exhaustive list is the wall of text that rots first.

## Single home

Every fact lives in exactly one document. A copy in two places bloats context and desyncs first.

| Fact | Home |
| --- | --- |
| Glossary term / ubiquitous-language definition | `context.md` |
| Where code lives (crate, module, entry point) | `map.md` (crate/module — no `:line`, it drifts) |
| A layer's invariant | that layer's PRD, and only there |
| Status / progress / TODO / work plan | GitHub Issues |
| Why a decision was made (rationale) | `docs/adr/` or `docs/designs/` — the PRD carries the *rule*, the ADR the *reason* |

A PRD **names** a glossary term (bold on first use) but never redefines it. Moving a fact to another home means deleting it from the old one — one copy in two places is a defect, not a backup.

## The topics contract

The lines right under a PRD's title:

```
label: <slug>
topics: <canonical glossary terms, comma-separated>
```

`topics:` exists for one consumer: the `required` skill greps this line to pull the PRD into context by a task's terms, never judging by filename. How it greps is `required`'s to own; how you write the line follows from it:

- **List every canonical term the PRD owns.** A term you omit makes the PRD unreachable for it — a routing gap. Over-listing is free (an extra grep hit costs one glance); a missing term is the failure, so bias toward more.
- **Canonical glossary term, not an `_Avoid_` synonym.** The searcher names terms in glossary vocabulary (`required` maps synonyms through `_Avoid_`), so a topic must match that vocabulary. Term not in the glossary yet → add the article to `context.md` first, then list it here.
- **Bare, greppable stem.** `required` greps the stem (`derived`, `each`, `bind`), not the decorated form. Make the stem a literal substring: `derived` already covers `$derived` / `OptimizedDerived`; write `each` (or `EachBlock`), never only `{#each}`.

Add a concept to a PRD → add its term here in the same edit. A PRD without `topics:` is invisible to routing.

## Anatomy

Title and frame, then the sections the layer needs — take what applies, not all of them.

<prd-template>
# PRD: <Name> (root)

label: <slug>
topics: <canonical glossary terms>

<1–2 sentences of frame: the crate(s), the place in the pipeline, the layer's dogma, the client/server counterpart.>

## Purpose
The single question this layer answers. One paragraph.

## <substance — one or two of these>
## What lives here   ·   ## Public API   ·   ## Traversal skeleton   ·   ## Inputs / outputs

## Architectural invariants
Numbered. Each opens with a **bold** lead and passes The two gates. Mechanical check inline where one exists.

## Anti-patterns
Concrete violations to scan for (`*Emit*` reads, template knowledge in the transform) — the recognizable bad forms, not a re-wording of the invariants above.

## Cross-document links
Cross-links with section anchors (`context.md §"Dogmas"`).
</prd-template>

Prose: compiler terminology over anything else; a glossary term bold on first use; no connective filler; the `CLAUDE.md` identifier ban holds in examples too.

## Reviewing a diff

Run **The two gates** on every added line, and check **Single home** and **The topics contract** — most review findings are just these three by name. Two the gates don't cover: the Anatomy prose conventions hold (glossary term bold, from the glossary not an `_Avoid_` synonym, no banned identifier); anchors in Cross-document links resolve to sections that exist.
