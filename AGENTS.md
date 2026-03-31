# Codex Agent Guide

`AGENTS.md` is the concise Codex entrypoint. `CLAUDE.md` remains the detailed source of truth for architecture, workflow, spec lifecycle, blocking rules, and quality checks. If they ever diverge, follow `CLAUDE.md`.

## Mandatory pre-edit reads
1. `README.md` (workflow context)
2. `CLAUDE.md` (architecture and boundary rules)
3. `CODEBASE_MAP.md` when touching APIs/types
4. Relevant `specs/*.md` `Current state` for multi-session feature work

## Hard architecture rules
- Keep logic in the correct layer:
  - `svelte_parser`: parse + JS pre-parse, immutable AST only
  - `svelte_analyze`: derived/classification data in `AnalysisData`
  - `svelte_transform`: JS AST rewrites only; no semantic rediscovery
  - `svelte_codegen_client`: JS emission from AST + analysis data
- Do not add string-based identifier classification where `SymbolId`/analysis accessors are required.
- Do not hand-edit generated snapshots:
  - `tasks/compiler_tests/cases2/*/case-svelte.js`
  - `tasks/compiler_tests/cases2/*/case-rust.js`

## Additional alignment notes from `CLAUDE.md`
- Use `spec-template` when creating or reshaping files under `specs/`.
- Use `phase-boundaries`, `test-pattern`, `svelte-reference-map`, `oxc-analyze-api`, `oxc-codegen-api`, and `legacy-conventions` as supporting knowledge skills when relevant.
- Before commits, verify correct layer ownership, no new boundary violations, correct visitor usage, `SymbolId` usage over strings, and explicit data flow.
- When blocked after repeated failed attempts, document the blocker in the relevant spec and report it instead of looping.

## Change policy
- Keep diffs minimal and task-scoped.
- Do not perform unrelated refactors or formatting churn.

## Validation commands (verified in `justfile`)
- `just test-case <name>`
- `just test-case-verbose <name>`
- `just generate`
- `just compare-benchmark [name]`
- `just test-parser`
- `just test-analyzer`
- `just test-compiler`
- `just test-all`

## Codex skills
Use `.codex/skills/` workflows with original command naming:
- `status`
- `audit`
- `port`
- `add-test`
- `explain-test`
- `fix-test`
- `diagnose`
- `improve`
- `qa`
- `review-boundaries`
- `fix-boundary`
- `review-simplify`
- `fix-simplify`
- `sync-docs`
- `bench`

Supporting reference skills:
- `phase-boundaries`
- `test-pattern`
- `spec-template`
- `svelte-reference-map`
- `oxc-analyze-api`
- `oxc-codegen-api`
- `legacy-conventions`
- `dump-ast`

## Comments
Comments must answer "why is this done?" — never describe what the line does.
Only add them where the intent isn't obvious from the code itself.
