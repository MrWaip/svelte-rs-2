# Codex Agent Guide

**Goal: production-ready Svelte compiler in Rust, targeting large enterprise codebases.** Performance at scale matters.

`AGENTS.md` is the concise Codex entrypoint. `CLAUDE.md` remains the detailed source of truth for architecture, workflow, spec lifecycle, blocking rules, and quality checks. If they ever diverge, follow `CLAUDE.md`.

## Mandatory pre-edit reads
1. `README.md` (workflow context)
2. `CLAUDE.md` (architecture and boundary rules)
3. `CODEBASE_MAP.md` when touching APIs/types
4. Relevant `specs/*.md` `Current state` for multi-session feature work

## Rust LSP
- When navigating Rust code, always use the `cclsp` MCP tools before grep, glob, shell search, or manual read-and-scan. If a `cclsp` tool fails or times out, retry it at least once before falling back, and only fall back for that specific failed operation.
- Use `rg` or other text search only for non-code text search or when the `cclsp` equivalent consistently fails.
- Available `cclsp` MCP tools (`mcp__cclsp__*`):
  - `find_workspace_symbols` — find a symbol by name across the workspace
  - `find_definition` — go to definition
  - `find_references` — all use-sites of a symbol across the workspace
  - `find_implementation` — trait or interface implementations
  - `prepare_call_hierarchy` + `get_incoming_calls`/`get_outgoing_calls` — call graph for a function
  - `rename_symbol` / `rename_symbol_strict` — rename across the workspace, including dry-run support
  - `get_diagnostics` — errors and warnings for a file
  - `get_hover` — type and signature of a symbol; may time out on a cold `rust-analyzer`

## Hard architecture rules
- Keep logic in the correct layer:
  - `svelte_parser`: parse + JS pre-parse, immutable AST only
  - `svelte_component_semantics`: scopes, symbols, references, per-symbol state bits — single source of truth for the entire component. Replaces `oxc_semantic::Scoping`/`SemanticBuilder`. OXC provides AST + Visit only.
  - `svelte_analyze`: derived/classification data in `AnalysisData`. Svelte-specific classifications (runes, props, stores) in `ComponentScoping` which `Deref`s to `ComponentSemantics`.
  - `svelte_transform`: JS AST rewrites only; no semantic rediscovery
  - `svelte_codegen_client`: JS emission from AST + analysis data
- Do not add string-based identifier classification where `SymbolId`/analysis accessors are required.
- Do not hand-edit generated snapshots:
  - `tasks/compiler_tests/cases2/*/case-svelte.js`
  - `tasks/compiler_tests/cases2/*/case-rust.js`
- These generated files are intentionally committed so the author can compare reference and Rust output directly in the repo; regenerate them through project commands instead of editing them by hand.

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
- `rust-lsp-workflow`
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
