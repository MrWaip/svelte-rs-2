# Review Fix: $ARGUMENTS

Implement findings from the last `/review` run in this conversation.

## Input

`$ARGUMENTS`:
- **`#N`** or **`N`** → implement finding N only
- **`all`** → implement all findings, lowest number first

## Step 1: Locate the review

Find the most recent `/review` output in this conversation. If no review output exists, stop and tell the user.

## Step 2: Plan

For each target finding:

1. Read all files listed in the **Evidence** section
2. Read `CODEBASE_MAP.md` for type signatures and module structure
3. If the fix touches a new feature area, read `GOTCHAS.md`
4. Produce a concrete implementation plan:
   - What types/functions change and how
   - Which files are affected
   - Order of changes (respecting data flow: upstream types before downstream consumers)
   - What gets deleted vs what gets added

**Present the plan and wait for approval before writing any code.**

If the fix description is ambiguous, flag it here instead of guessing.

If a single finding requires changes across 5+ files, plan the first step only and explain what remains.

## Step 3: Implement

Apply the approved plan. Follow the design principle from CLAUDE.md: match JS output exactly, implement internals in idiomatic Rust.

After each finding (when running `all`): run `just test-all`. Tests must pass before moving to the next finding.

If tests fail: fix and retry. **Stop after 3 failed attempts** — report what you tried, do not loop.

If a finding conflicts with changes already made, skip it and explain the conflict.

## Step 4: Verify

Run `just test-all`.

Summarize: which findings were implemented, which were skipped (and why), what to verify manually.
