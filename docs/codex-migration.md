# Claude Code → Codex migration note

## What was found

This repository already includes a mature Claude-oriented setup:

- Global project operating guide in `CLAUDE.md` (architecture boundaries, spec lifecycle, naming and review rules).
- A command suite in `.claude/commands/` covering status, audit/port, test-fix, QA, docs sync, benchmarking, and review flows.
- Sub-agents in `.claude/agents/` for codebase analysis, reference tracing, and quality review.
- Reusable skills in `.claude/skills/` for phase boundaries, OXC APIs, spec template, legacy conventions, and test patterns.
- Deterministic workflow commands in `justfile` used by those workflows.

## What was mapped from Claude to Codex

The migration keeps existing Claude files intact and adds a Codex-native layer:

1. **Repo-level instruction entrypoint**
   - Added `AGENTS.md` at repository root for Codex-wide expectations (exploration, architecture, minimal diffs, validation commands).

2. **Codex skill bundles**
   - Added `.codex/skills/` with focused initial skills:
     - `status` (maps `/status` intent)
     - `qa` (maps `/qa` intent)
     - `fix-test` (maps `/fix-test` intent)
     - `port` (maps `/port specs/...` continuation intent)
     - `audit` (maps `/audit` intent)
     - `diagnose` (maps `/diagnose` intent)
     - `explain-test` (maps `/explain-test` intent)
     - `review-boundaries` (maps `/review-boundaries` intent)
     - `review-simplify` (maps `/review-simplify` intent)
     - `sync-docs` (maps `/sync-docs` intent)

3. **Deterministic helpers moved to scripts**
   - Added `.codex/scripts/qa.sh` for consistent QA scope detection + recommended checks.
   - Added `.codex/scripts/fix-test.sh` for reproducible single-test diagnosis loops.

This preserves workflow semantics while removing Claude-specific ceremony such as slash-command framing and agent-role chatter.

## What remains manual

- Multi-agent delegation from Claude commands (e.g., explicit `@reference-tracer` + `@codebase-analyzer` parallel dispatch) is not auto-recreated; Codex now uses direct repo analysis plus focused skills.
- Existing Claude command files remain as historical reference and can still inform future Codex skill additions.
- If you want 1:1 parity for every legacy slash command, add additional small skills over time instead of one mega-skill.

## How to invoke the new setup

- Start with repository guidance in `AGENTS.md`.
- Use skills from `.codex/skills/` based on task intent:
  - Session triage → `status`
  - Post-change validation/review → `qa`
  - Single failing compiler test → `fix-test`
  - Continue feature spec work → `port`
- Run deterministic commands from `justfile` (and optional wrappers in `.codex/scripts/`).

## Suggested next incremental migrations

1. Add `bench`, `improve`, and `add-test` Codex skills for full high-frequency parity.
2. Add optional structured templates for `BOUNDARY_REVIEW.md` and `SIMPLIFY_REVIEW.md`.
3. Add optional check scripts that parse ignored test reasons and roadmap priority automatically.
## Command reference verification

All `just` commands referenced in `AGENTS.md`, `.codex/skills/`, and `.codex/scripts/` were checked against `justfile` and exist:
- `test-case`
- `test-case-verbose`
- `test-parser`
- `test-analyzer`
- `test-compiler`
- `test-all`

