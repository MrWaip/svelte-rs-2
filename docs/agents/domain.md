# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

This is a **single-context** repo, but its domain docs do not use the default `CONTEXT.md` / `docs/adr/` layout. Read the repo's actual sources instead, mapped below.

## Before exploring, read these

- **`docs/context.md`** — the single entry point: architecture overview, dogmas, cross-cutting conventions, and the domain glossary (ubiquitous language). This plays the role of `CONTEXT.md`. Read it first.
- **`docs/map.md`** — codebase map: crates, entry points, files holding the main structures.
- **Per-layer root PRDs in `docs/`** — `ast`, `parser`, `analyze`, `component-semantics`, `reactivity-semantics` (child `state-rune`), `expression-semantics`, `attribute-semantics`, `block-semantics`, `transform`, `codegen`, `compiler`, `supporting-crates`. Read the ones that touch the area you're about to work in. Their invariants are hard constraints.
- **`docs/designs/`** — design PRDs (created via `/design`). Read only if scope overlaps.
- **`docs/adr/`** — architecture decision records. **Not present yet.** If/when it appears, read ADRs that touch the area you're working in.

If any of these don't exist, **proceed silently**. Don't flag their absence; don't suggest creating them upfront. The producer skill (`/grill-with-docs`) creates them lazily when terms or decisions actually get resolved.

## File structure

```
/
├── docs/
│   ├── context.md        ← canonical domain doc (CONTEXT.md role): overview, dogmas, glossary
│   ├── map.md            ← codebase map
│   ├── ast.md  parser.md  analyze.md  ...   ← per-layer root PRDs (invariants)
│   ├── designs/          ← design PRDs (read only if scope overlaps)
│   └── adr/              ← architecture decisions (not present yet)
├── crates/
└── original/compiler/    ← reference Svelte compiler (understand WHAT, not HOW)
```

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in `docs/context.md`. Prefer compiler terminology over other naming. Don't drift to synonyms the glossary explicitly avoids.

Note the project's banned identifiers (from `CLAUDE.md`): never coin new code identifiers using `leaf`, `leaves`, `plan`, `shape`, `shadow`, `lowering`/`lower`, `tpl`, `dyn`, `sym`.

If the concept you need isn't in the glossary yet, that's a signal — either you're inventing language the project doesn't use (reconsider) or there's a real gap (note it for `/grill-with-docs`).

## Flag ADR conflicts

If your output contradicts an existing ADR (once `docs/adr/` exists) or a per-layer root PRD invariant, surface it explicitly rather than silently overriding:

> _Contradicts the `transform` root PRD invariant (dumb codegen/transform) — but worth reopening because…_
