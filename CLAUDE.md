# Project Instructions

**Goal: production-ready Svelte compiler in Rust, targeting large enterprise codebases.** Performance at scale (thousands of components per build) is a first-class concern — every per-symbol allocation and lookup matters.

Detailed crate API and type reference: `CODEBASE_MAP.md` (read when you need type signatures or module structure).
Gotchas, data flow per pass, node-type checklist, output examples: `GOTCHAS.md` (read when adding a new feature or debugging unexpected output).

## Tool Priority: LSP FIRST

**When navigating rust code, ALWAYS use the cclsp MCP tools before grep, glob, bash, or Read-and-scan.** They are faster, more accurate, and cheaper on tokens. If a cclsp tool fails or times out, retry it at least once before falling back — and only fall back for that specific failed operation, not for the entire task. Use grep/glob only for non-code text search or when the cclsp equivalent consistently fails. This is a hard rule, not a suggestion.

Available cclsp MCP tools (`mcp__cclsp__*`):
- `find_workspace_symbols` — find a symbol by name across the entire workspace
- `find_definition` — go to definition
- `find_references` — all use-sites of a symbol across the workspace
- `find_implementation` — trait/interface implementations
- `prepare_call_hierarchy` + `get_incoming_calls`/`get_outgoing_calls` — call graph for a function
- `rename_symbol` / `rename_symbol_strict` — rename across the entire workspace (supports dry_run)
- `get_diagnostics` — errors and warnings for a file
- `get_hover` — type and signature of a symbol (may time out on a cold rust-analyzer)


## Spec files

A spec is a working document for a feature. Created on first contact with the task, lives until the implementation is complete. Read top to bottom: status → scope → details.

### When to create
- Task spans 2+ layers (parser + analyze + codegen)
- Or task doesn't fit in a single session
- Small fixes (one file, one session) — no spec needed

### Naming
File name = feature name in kebab-case: `state-rune.md`, `each-block.md`, `diagnostics-infrastructure.md`.
For ROADMAP tier items: `<tier-id>-<short-name>.md` (e.g. `5a-diagnostics-infrastructure.md`).
If the feature has no tier in ROADMAP — just `<feature-name>.md`.

### Structure

Section order is fixed. Most important things go at the top.

| Section | Purpose | Required |
|---------|---------|----------|
| Current state | **First thing a reader sees.** What's done, what's next, blockers. Date updated. | Required |
| Source | Link to ROADMAP item or request | Required |
| Syntax variants | All syntactic forms of the feature (from docs and reference compiler parser) | Required |
| Use cases | Flat checklist: `[ ]`, `[x]` — no subsections | Required |
| Out of scope | Plain list of things explicitly excluded: SSR, removed features, future tiers | Optional |
| Reference | Reference compiler files + our files — so the next session doesn't have to search | Required |
| Tasks | Implementation plan per layer, with specific files and functions | Required |
| Implementation order | Execution order of Tasks (inter-layer dependencies) | Optional |
| Discovered bugs | Bugs found during work (marked FIXED/OPEN) | As needed |
| Test cases | List of tests: existing + planned | Optional |

### Scope rules
- **Client-side only.** Do NOT include SSR use cases — SSR is a separate phase after client is complete.
- Use case marked `[ ]` = still open for current work; if partially implemented, keep it unchecked and describe what works / what is missing inline
- Use case marked `[x]` = implemented and covered by a test
- `Use cases` — flat checklist, no `###` subsections
- `Out of scope` — plain list for explicitly excluded things (SSR, removed features, future tiers)

### Lifecycle
1. Created: `/audit` step 3 (template: `spec-template` skill). `/port` resumes from an existing spec.
2. Updated: after each session — Current state section (at the top!)
3. Completed: when all Use cases are `[x]` → feature is Done in ROADMAP
4. Not deleted — kept as reference

### Rules
- Before implementing: `Glob("specs/*.md")` — check if a spec exists
- If it exists — read Current state (first section) and continue
- If not — create one during planning

## Architecture boundaries — STRICT ENFORCEMENT

**Before writing code, verify it goes in the correct layer.**

Layers:
- `svelte_parser` — produces immutable AST. Owns shared domain types and JS expression pre-parsing (`parse_js` -> `JsParseResult`).
- `svelte_component_semantics` — **single source of truth** for scopes, symbols, references, and per-symbol state across module script + instance script + template. Owns its own builder (`ComponentSemanticsBuilder`) that traverses JS AST via OXC `Visit` and template via `TemplateWalker` trait. Replaces `oxc_semantic::Scoping` / `SemanticBuilder` entirely — OXC provides AST + Visit trait only. Does **not** depend on Svelte AST for template traversal.
- `svelte_analyze` — multi-pass pipeline. Owns ALL derived data, classifications, flags, precomputation -> `AnalysisData` side tables (keyed by `NodeId`). Also owns expression analysis types (`ExpressionInfo`, `Reference`, `ReferenceFlags`, `ExpressionKind`). Svelte-specific symbol classifications (runes, props, stores, etc.) live in `ComponentScoping` which wraps `ComponentSemantics` via `Deref`.
- `svelte_codegen_client` — consumes AST + AnalysisData + ParsedExprs to produce JS output. Owns only JS output construction logic.

Boundary rules:
1. **Immutable AST** — derived data goes into `AnalysisData`, never into AST nodes.
2. **Analysis owns classification** — if codegen would need to re-traverse AST to collect/classify data, that data must be computed in analyze instead.
3. **JS parsing in parser** — JS expression parsing belongs in `svelte_parser`, not in analyze or codegen.
4. **SymbolId over strings** — all identifier lookups must go through `SymbolId`. `FxHashSet<String>` and `FxHashMap<String, _>` must never be keyed by identifier names. Only exception: string literals in JS output generation.
5. **No codegen data caching** — codegen-internal enums/structs that cache or duplicate AST data are a smell. The classification belongs in `AnalysisData`.
6. **Correct over minimal** — never propose a "simple" fix in the wrong layer when a correct approach exists.
7. **Existing violations are not precedent** — never use "the existing code already does this" as justification. Either fix the violation or flag it.

For detailed red/green flags, OXC visitor rules, and additional rules, see the `phase-boundaries` skill.

## Porting from Svelte compiler

Reference Svelte compiler source is in `reference/compiler/`. Use it to understand **what** output to produce, not **how** to implement it.

Match the JS output exactly. Design internals for Rust: direct recursion over side tables, no mutable AST metadata. Don't replicate JS workarounds or patterns that exist only because of zimmerframe/estree-walker.

**Exception** — `svelte_analyze` uses a single-pass composite visitor (`walker.rs`). Codegen uses direct recursion.

To start a new feature: `/audit <feature>`. To implement the next approved slice from a spec: `/port specs/<file>.md`.
To fix existing code problems (bugs, workarounds, missing tests): `/improve <description>`.
Read `ROADMAP.md` for the full feature catalog and current priorities.

When discovering new items, add them to the matching spec `Use cases` as unchecked checkboxes . If there is no matching spec, report that explicitly to the user instead of recording the item elsewhere.

For legacy Svelte 4 features, see the `legacy-conventions` skill.

## Naming

- `gen_*` — creates and returns statements.
- `process_*` — mutates provided `&mut Vec` in-place.
- `emit_*` — appends specialized statements to a `&mut Vec`.
- `pub(crate)` by default; `pub` only for entry points and types.

## Rust idioms

- Early return over deep nesting.
- Exhaustive `match` for enums; `if let` when only one variant matters.
- `.copied()`, `.is_some_and()`, `.map_or()` over verbose match/if-let for simple Option ops.
- `.remove()` for ownership transfer from side tables (not `.get().cloned()`).
- `unwrap_or_else(|| panic!(...))` only for internal invariants, never for user errors. User errors -> `Diagnostic`.
- Repeating `match` patterns on an enum -> extract into a method on that enum.
- Comments answer "why", never describe what the line does.

## Pre-commit self-check

Before every commit, verify:
1. **Correct layer** — is this code in the right crate? (parser/analyze/codegen)
2. **No new boundary violations** — no re-parsing in codegen, no string lookups, no repeated traversal
3. **Visitor usage** — any new JS AST traversal uses OXC Visit/Traverse, not manual match. Consult the OXC API skill references for the correct visitor method. Exceptions: shallow destructure, `builder.rs`.
4. **SymbolId** — no new string-based identifier comparisons
5. **Edge cases** — does the change handle all JS syntax variants, not just the tested one?
6. **No implicit dependencies** — data flows through explicit types and function signatures

If any check fails — fix before committing. Don't create a TODO.

## When blocked

If implementation fails after 3 attempts on the same approach:
1. Commit what works (WIP commit if partial)
2. Document the blocker in `specs/<feature>.md` Current state section
3. If blocker is a separate task — add it to the spec `Use cases` as an unchecked checkbox ; if there is no matching spec, report that explicitly to the user
4. Report to user: what was tried, what failed, what the blocker is
5. Move to next task or end session

Never loop on the same failing approach. Never silently skip the problem.

## No resistance, no excuses

Never push back on a task with "this is too complex", "not worth the effort", "this would require too many changes", or similar. If the user asks for something — do it. If genuinely blocked — follow the "When blocked" process above.

Never use existing code violations as permission for new violations. "The existing code already does X" is not a valid justification for adding more X. Either fix the violation as part of your change or flag it — but never extend it.

Never argue for a simpler/shorter approach when the user has specified the correct one. If you think there's a better way — state it once, briefly. If the user disagrees, do it their way.
