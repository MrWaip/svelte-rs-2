---
name: qa
description: Post-change quality gate for recent diffs, commits, or paths. Use after implementation, refactors, or before commit/PR to review changes against `CLAUDE.md` and `AGENTS.md`, produce a PASS/FAIL verdict, and list exact violations and fixes. Do not trigger for read-only research.
---

# QA Review

## 1) Identify review scope

- commit hash or range: review that diff
- file or directory path: review the whole path
- no argument: prefer uncommitted diff, else `HEAD~1..HEAD`

Optional helper:
```bash
.codex/scripts/qa.sh [<base-ref>]
```

## 2) Read the changed files in full

Do not review only hunks. Context around the change matters.

## 3) Check project rules

Review against `CLAUDE.md` and `AGENTS.md`, including:

- layer ownership
- visitor usage
- symbol handling and no string-based reclassification
- test and spec hygiene
- no edits to generated snapshots

## 4) Execute focused validation
Run the narrowest commands that prove safety:
- single test: `just test-case <name>`
- crate-level: `just test-parser` or `just test-analyzer`
- integration: `just test-compiler`
- broad sweep when needed: `just test-all`

## 5) Report in decision format
- `STATUS: PASS` only when no known violations remain.
- If issues exist, report `STATUS: FAIL` with:
  - concrete violations (file + why)
  - minimal fix plan
  - recommended exact commands

If you applied fixes during the same run, do not self-certify a final PASS. A fresh `qa` run is still the clean confirmation.
