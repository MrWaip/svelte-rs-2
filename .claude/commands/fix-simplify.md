---
description: Fix a simplification opportunity from SIMPLIFY_REVIEW.md. Use after /review-simplify produces a report.
argument-hint: "[#N | all]"
allowed-tools: Bash, Read, Grep, Glob, Agent, LSP, Edit, Write
---

# Fix Simplification: $ARGUMENTS

Implement a simplification from the last `/review-simplify` run.

## Step 1: Locate the review

Read `SIMPLIFY_REVIEW.md` from the project root. If the file does not exist, stop and tell the user to run `/review-simplify` first.

If `$ARGUMENTS` is `all` — process all findings in order, highest impact first. Skip ~~strikethrough~~ findings.
If `$ARGUMENTS` is `#N` or `N` — process that finding only.

## Step 2: Understand the finding

For each target finding, read:
1. All files listed in **Files** — understand the full current logic
2. If Level 1 (cross-layer): read both the upstream and downstream code
3. If the finding references a helper that already exists — read it to confirm it's reusable

## Step 3: Plan

Produce a concrete plan based on the finding's **Level**:

**Level 1 (Cross-layer):**
- Add pre-computed data upstream (analyze accessor, parser AST field)
- Simplify downstream consumers
- Order: upstream first, then downstream

**Level 2 (Within-crate):**
- Extract helper, inline abstraction, remove dead weight, or flatten nesting
- Usually contained to 1-2 files
- Verify the extraction doesn't create a god-function or add obscuring parameters

**Level 3 (Cross-file):**
- Identify the canonical location for the shared logic
- Extract to a shared module or the file that already has the most complete version
- Update all call sites

For each change:
- Which files are affected
- What gets extracted/inlined/deleted
- Whether any public API changes

**Present the plan and wait for approval before writing any code.**

## Step 4: Implement

Apply the approved plan.

**Preserve functionality.** Change HOW, never WHAT. Generated JS must be identical.

After each meaningful change, verify: `cargo check -p <crate>`.

After all changes: run `just test-all`.

If tests fail: you changed behavior, not just structure. Investigate the diff. Fix and retry. **Stop after 3 failed attempts.**

## Step 5: Verify & Update

Run `just test-all` — all tests must pass.

Update `SIMPLIFY_REVIEW.md`: mark implemented findings with ~~strikethrough~~, add a one-line note:
```
~~### #1 — Directive dispatch duplicated between element.rs and attributes.rs~~
*Fixed: extracted `process_directive_attrs` helper, called from both spread and non-spread paths.*
```

Summarize:
- Which findings were implemented
- Which were skipped (and why)
- What was extracted/inlined/deleted
- Lines of code removed (approximate)

## Rules

- **Plan first.** Never write code without user-approved plan.
- **No JS output changes.** These are refactors. Snapshot tests must pass unchanged.
- **Balance check in plan.** If during planning you realize the simplification creates more complexity than it removes — skip it and explain why.
- **One finding at a time** (when running `all`). Run tests between findings.
- **Stop after 3 failures.** Report what you tried, do not loop.
