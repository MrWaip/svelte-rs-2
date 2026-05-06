# Domain Docs

How the engineering skills consume this repo's domain documentation.

## Before exploring, read

- **`CONTEXT.md`** at the repo root — index of domain documents.
- **`ARCHITECTURE.md`** — per-crate rules and invariants. Read sections relevant to the layer you're touching (ast, parser, analyze, transform, codegen).
- **`CODEBASE_MAP.md`** — crate API + type reference.
- **`CLAUDE.md`** — project dogmas, terminology, banned glossary terms.
- **`ROADMAP.md`** — feature porting status with links to specs.
- **`debt.md`** — known tech debt. Check before proposing refactors — the problem may already be logged.
- **`specs/<feature>.md`** — per-feature porting spec. Read when working on that feature.

## Layout

Single-context repo. No `docs/adr/` — architectural decisions live inline in `ARCHITECTURE.md` and `debt.md`. If a skill expects ADRs, treat their absence as "decisions are recorded elsewhere", not as a gap.

## Vocabulary

Use the terminology from `CLAUDE.md` (Glossary section). Banned terms: `leaf`, `leaves`, `plan`, `shape`, `shadow`, `lowering`, `lower`. Prefer compiler terminology over framework-specific naming.

If the term you need isn't covered, check `CODEBASE_MAP.md` for the actual type/module name in the codebase before inventing one.

## Flag conflicts

If your proposal contradicts a rule in `ARCHITECTURE.md` or a debt entry's stated direction, surface it explicitly rather than silently overriding:

> _Contradicts ARCHITECTURE.md §3 (smart analyzer / dumb codegen) — but worth reopening because…_
