---
description: Review recent code changes against project guidelines. Use after /fix-test, /improve, /port, or any implementation work.
argument-hint: "[commit-hash | HEAD~N | empty]"
allowed-tools: Bash, Read, Grep, Glob, Agent, LSP, Edit, Write
---

# Check quality: $ARGUMENTS

Review recent code changes against project guidelines from CLAUDE.md. This command is read-only — it finds violations and produces a fix plan, but does NOT apply fixes.

## Step 1: Collect changes

Determine the diff to review based on `$ARGUMENTS`:

- **Commit hash provided** (e.g. `abc123`): `git diff abc123..HEAD`
- **Range provided** (e.g. `HEAD~3`): `git diff HEAD~3..HEAD`
- **Directory/file path** (e.g. `crates/svelte_codegen_client`): review ALL code in this path (not just diff)
- **No argument** — auto-detect in order:
  1. Uncommitted changes: `git diff --name-only HEAD` — if non-empty, use this
  2. Last commit: `git diff --name-only HEAD~1..HEAD`

Read every changed file in full to understand the context around each change.

## Step 2: Review with quality-reviewer

Launch **@quality-reviewer** agent on the diff from step 1.

The agent checks all project rules: architecture boundaries, visitor usage, naming, Rust idioms, edge cases. See `.claude/agents/quality-reviewer.md` for the full checklist.

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

After all items are DONE, stop. Do NOT re-review in this same run — the user will run `/qa` again for a fresh review.

Only fix reported violations — do not add improvements or refactoring beyond what was reported.

## Rules

- Check EVERY rule against EVERY changed file. Do not skip rules that "probably" pass.
- Read the actual code, not just the diff — context around the change matters.
- If unsure whether something is a violation — it's a violation. Report it.
- Do NOT declare PASS yourself after fixing — only a fresh `/qa` run can declare PASS.
