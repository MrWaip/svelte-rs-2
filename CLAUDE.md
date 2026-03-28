# Project Instructions

Detailed crate API and type reference: `CODEBASE_MAP.md` (read when you need type signatures or module structure).
Gotchas, data flow per pass, node-type checklist, output examples: `GOTCHAS.md` (read when adding a new feature or debugging unexpected output).

## Code Navigation

Use LSP first (go-to-definition, references, hover, symbols). grep/ripgrep only as fallback when LSP returns nothing.

## Spec files

Before implementing any ROADMAP item or complex feature, check if `specs/<feature>.md` exists.
If yes — read it first and continue from where the last session stopped.
If no — create it during planning (see `/port-svelte` step 3 or `/audit-feature`).

## Architecture boundaries — STRICT ENFORCEMENT

**Before writing code, verify it goes in the correct layer.**

Layers:
- `svelte_parser` — produces immutable AST. Owns shared domain types and JS expression pre-parsing (`parse_js` -> `JsParseResult`).
- `svelte_analyze` — multi-pass pipeline. Owns ALL derived data, classifications, flags, precomputation -> `AnalysisData` side tables (keyed by `NodeId`). Also owns expression analysis types (`ExpressionInfo`, `Reference`, `ReferenceFlags`, `ExpressionKind`).
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

To port a new feature: `/port-svelte <feature>`. To audit existing feature completeness: `/audit-feature <feature>`.
To fix existing code problems (bugs, workarounds, missing tests): `/fix-debt <description>`.
Read `ROADMAP.md` for the full feature catalog and current priorities.

When discovering deferred items, add them to the **Deferred** section of `ROADMAP.md`.

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
3. If blocker is a separate task — add to ROADMAP.md Deferred
4. Report to user: what was tried, what failed, what the blocker is
5. Move to next task or end session

Never loop on the same failing approach. Never silently skip the problem.

## No resistance, no excuses

Never push back on a task with "this is too complex", "not worth the effort", "this would require too many changes", or similar. If the user asks for something — do it. If genuinely blocked — follow the "When blocked" process above.

Never use existing code violations as permission for new violations. "The existing code already does X" is not a valid justification for adding more X. Either fix the violation as part of your change or flag it — but never extend it.

Never argue for a simpler/shorter approach when the user has specified the correct one. If you think there's a better way — state it once, briefly. If the user disagrees, do it their way.
