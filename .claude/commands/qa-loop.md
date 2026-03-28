---
description: Review recent code changes against project guidelines. Use after /fix-test, /fix-debt, /port-svelte, or any implementation work.
argument-hint: "[commit-hash | HEAD~N | empty]"
allowed-tools: Bash, Read, Grep, Glob, Agent, LSP, Edit, Write
---

# Check quality: $ARGUMENTS

Review recent code changes against project guidelines from CLAUDE.md. This command is read-only — it finds violations and produces a fix plan, but does NOT apply fixes.

## Step 1: Collect changes

Determine the diff to review based on `$ARGUMENTS`:

- **Commit hash provided** (e.g. `abc123`): `git diff abc123..HEAD`
- **Range provided** (e.g. `HEAD~3`): `git diff HEAD~3..HEAD`
- **No argument** — auto-detect in order:
  1. Uncommitted changes: `git diff --name-only HEAD` — if non-empty, use this
  2. Last commit: `git diff --name-only HEAD~1..HEAD`

Read every changed file in full to understand the context around each change.

**Load OXC API references** — read all three files:
- `.claude/skills/oxc-codegen-api/references/traverse-methods.txt`
- `.claude/skills/oxc-analyze-api/references/visit-methods.txt`
- `.claude/skills/oxc-analyze-api/references/scoping-api.txt`

## Step 2: Check each rule

For EACH changed file, check ALL of the following. Report every violation found — do not stop at the first one.

### Architecture boundaries

1. **Correct layer** — is the code in the right crate?
   - `svelte_parser` — immutable AST, JS expression pre-parsing
   - `svelte_analyze` — derived data, classifications, flags → `AnalysisData`
   - `svelte_codegen_client` — JS output construction only
2. **Immutable AST** — no derived data stored in AST nodes
3. **Analysis owns classification** — codegen does not re-traverse AST to collect/classify data
4. **JS parsing in parser** — no JS expression parsing in analyze or codegen
5. **SymbolId over strings** — no `FxHashSet<String>` or `FxHashMap<String, _>` keyed by identifier names (string literals in JS output are OK)
6. **No codegen data caching** — no codegen-internal enums/structs that duplicate AST data

### Visitor usage

7. **OXC Visit/Traverse for JS AST traversal** — any new JS AST traversal uses OXC Visit/Traverse, not manual match. Consult the OXC API skill references for the correct visitor method. Exceptions: shallow destructure, `builder.rs`.

### Naming conventions

8. **Function naming** — `gen_*` returns statements, `process_*` mutates `&mut Vec`, `emit_*` appends to `&mut Vec`
9. **Visibility** — `pub(crate)` by default; `pub` only for entry points and types

### Rust idioms

10. **Early return** over deep nesting
11. **Exhaustive match** for enums; `if let` when only one variant matters
12. **Option idioms** — `.copied()`, `.is_some_and()`, `.map_or()` over verbose match/if-let
13. **Ownership transfer** — `.remove()` from side tables, not `.get().cloned()`
14. **Error handling** — `unwrap_or_else(|| panic!(...))` for internal invariants only, `Diagnostic` for user errors
15. **Comments** — answer "why", never describe what the line does

### Edge cases

16. **JS syntax coverage** — does the change handle all JS syntax variants, not just the tested one? (e.g., all assignment operators, all expression types)
17. **No implicit dependencies** — data flows through explicit types and function signatures

## Step 3: Verdict

Output in this exact format:

```
STATUS: PASS | FAIL

VIOLATIONS:
1. [file:line] — [rule number + name] — [what's wrong and why]
2. ...

FIX PLAN:
1. [file:line] — [exact change to make]
2. ...
```

Rules:
- If ANY violation exists → `STATUS: FAIL`
- If ZERO violations → `STATUS: PASS`, skip VIOLATIONS and FIX PLAN sections
- No "warnings only" mode — every issue is a violation or not reported

If `STATUS: PASS` — done, stop here.

If `STATUS: FAIL` — proceed to Step 4.

## Step 4: Fix

FIX PLAN is a strict contract. For each item:
1. Apply the fix
2. Mark the item as DONE

After all items are DONE, stop. Do NOT re-review in this same run — the user will run `/check-quality` again for a fresh review.

Only fix reported violations — do not add improvements or refactoring beyond what was reported.

## Rules

- Check EVERY rule against EVERY changed file. Do not skip rules that "probably" pass.
- Read the actual code, not just the diff — context around the change matters.
- If unsure whether something is a violation — it's a violation. Report it.
- Do NOT declare PASS yourself after fixing — only a fresh `/check-quality` run can declare PASS.
