# Codex Agent Guide

## Mandatory pre-edit reads
1. `README.md` (workflow context)
2. `CLAUDE.md` (architecture and boundary rules)
3. `CODEBASE_MAP.md` when touching APIs/types
4. Relevant `specs/*.md` `Current state` for multi-session feature work

## Hard architecture rules
- Keep logic in the correct layer:
  - `svelte_parser`: parse + JS pre-parse, immutable AST only
  - `svelte_analyze`: derived/classification data in `AnalysisData`
  - `svelte_codegen_client`: JS emission from AST + analysis data
- Do not add string-based identifier classification where `SymbolId`/analysis accessors are required.
- Do not hand-edit generated snapshots:
  - `tasks/compiler_tests/cases2/*/case-svelte.js`
  - `tasks/compiler_tests/cases2/*/case-rust.js`

## Change policy
- Keep diffs minimal and task-scoped.
- Do not perform unrelated refactors or formatting churn.

## Validation commands (verified in `justfile`)
- `just test-case <name>`
- `just test-case-verbose <name>`
- `just test-parser`
- `just test-analyzer`
- `just test-compiler`
- `just test-all`

## Codex skills
Use `.codex/skills/` workflows with original command naming: `status`, `qa`, `fix-test`, `port`, `audit`, `diagnose`, `explain-test`, `review-boundaries`, `review-simplify`, `sync-docs`.
