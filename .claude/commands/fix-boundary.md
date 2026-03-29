---
description: Fix a boundary/data flow violation from BOUNDARY_REVIEW.md. Use after /review-boundaries produces a report.
argument-hint: "[#N | all]"
allowed-tools: Bash, Read, Grep, Glob, Agent, LSP, Edit, Write
---

# Fix Boundary Violation: $ARGUMENTS

Implement a fix for a finding from the last `/review-boundaries` run.

## Step 1: Locate the review

Read `BOUNDARY_REVIEW.md` from the project root. If the file does not exist, stop and tell the user to run `/review-boundaries` first.

If `$ARGUMENTS` is `all` — process all findings in order, lowest number first. Skip ~~strikethrough~~ findings.
If `$ARGUMENTS` is `#N` or `N` — process that finding only.

## Step 2: Understand the finding

For each target finding, read:
1. All files listed in **Evidence** — understand the full current logic
2. `CODEBASE_MAP.md` — type signatures, module structure
3. The upstream crate's relevant files (data.rs, scope.rs, etc.) — understand what already exists
4. If the fix touches a new area — read `GOTCHAS.md`

## Step 3: Plan

Produce a concrete plan following the finding's **Class**:

**Class 1-2 (Re-parsing / String classification):**
- Often requires parser changes (new AST field) or analyze changes (new enum/accessor)
- Plan: add type upstream → populate in correct phase → replace downstream usage
- Order: parser/analyze first, then codegen consumers

**Class 3 (AST re-traversal):**
- Add pre-computed data in analyze → expose accessor → replace iteration in codegen
- Order: analyze side-table → accessor → codegen simplification

**Class 4 (Raw handoff):**
- Add enum or accessor in analyze → replace multi-flag decision in codegen
- Order: enum definition → analyze computation → codegen match

**Class 5 (Late knowledge):**
- Preserve the discarded data upstream → pass through to downstream
- Order: extend upstream type → populate → read downstream

**Class 6 (Types that lie):**
- Split type, add Option, or rename to reflect actual semantics
- Order: type change → update all consumers

For each change:
- Which files are affected and in what order
- What types/functions change and how
- What gets deleted vs what gets added

**Present the plan and wait for approval before writing any code.**

## Step 4: Implement

Apply the approved plan. Order: upstream types → computation → downstream consumers.

After each file change, verify it compiles: `cargo check -p <crate>`.

**Critical invariant: JavaScript output must not change.** These are pure refactors — generated JS must be identical. Snapshot tests are your proof.

After all changes: run `just test-all`.

If tests fail: diff expected vs actual JS output. Fix and retry. **Stop after 3 failed attempts** — report what you tried.

## Step 5: Verify & Update

Run `just test-all` — all tests must pass.

Update `BOUNDARY_REVIEW.md`: mark implemented findings with ~~strikethrough~~, add a one-line note:
```
~~### #3 — Event name re-derived in codegen~~
*Fixed: added `stripped_name` field to `EventHandlerMode`, removed `strip_capture_event` calls in codegen.*
```

Summarize:
- Which findings were implemented
- Which were skipped (and why)
- What was added (accessor / enum / AST field)
- What was removed from codegen

## Rules

- **Plan first.** Never write code without user-approved plan.
- **Upstream before downstream.** Always change the provider before the consumer.
- **No JS output changes.** These are refactors. Snapshot tests must pass unchanged.
- **One finding at a time** (when running `all`). Run tests between findings.
- **Stop after 3 failures.** Report what you tried, do not loop.
