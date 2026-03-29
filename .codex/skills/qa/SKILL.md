---
name: qa
description: Post-change quality gate. Trigger after code edits, refactors, or before commit/PR to produce PASS/FAIL plus a fix plan. Do not trigger for read-only research or backlog planning.
---

# /qa workflow (Codex)

## 1) Identify review scope
- If user provides commit/range/path, use it.
- Else prefer uncommitted diff; if clean, review `HEAD~1..HEAD`.

Optional helper:
```bash
.codex/scripts/qa.sh [<base-ref>]
```

## 2) Review for architecture correctness
Check changed files against `AGENTS.md` + `CLAUDE.md` constraints:
- parser/analyze/codegen layer ownership
- no new semantic reclassification in codegen
- no string-based symbol classification where analysis should own decisions
- no edits to generated `case-svelte.js` / `case-rust.js`

## 3) Execute focused validation
Run the narrowest commands that prove safety:
- single test: `just test-case <name>`
- crate-level: `just test-parser` or `just test-analyzer`
- integration: `just test-compiler`
- broad sweep when needed: `just test-all`

## 4) Report in decision format
- `STATUS: PASS` only when no known violations remain.
- If issues exist, report `STATUS: FAIL` with:
  - concrete violations (file + why)
  - minimal fix plan
  - recommended exact commands
