# Migrate Boundary Violation: $ARGUMENTS

Lift a phase boundary violation from codegen/transform into parser or analyze.

## Input

`$ARGUMENTS`:
- **`#N`** or **`N`** → implement finding N only
- **`all`** → implement all findings, lowest class number first, then highest occurrence count

## Step 1: Locate the audit

Read `AUDIT.md` from the project root. If the file does not exist, stop and tell the user to run the audit prompt first.

## Step 2: Understand the current state

For each target finding:

1. Read all files listed in the **Where** section — understand the full codegen logic being replaced
2. Read the corresponding analyze code to understand what data IS already available
3. Read `CODEBASE_MAP.md` for type signatures and module structure
4. If the finding touches a new area, read `GOTCHAS.md`
5. Identify ALL consumers of the current pattern — grep for the same flag combination, function name, or string operation across the entire codebase. Missing a consumer means a silent regression.

## Step 3: Plan the migration

Produce a concrete plan with these sections:

### What to add

Calibrate the solution to the problem size:
- **Single bool decision** → accessor method on AnalysisData (e.g. `fn needs_getter(&self, id: NodeId) -> bool`). No new type needed.
- **3+ distinct modes** → enum in analyze (e.g. `enum AttrOutputMode { Static, Shorthand, DynamicGetter, Memoized }`). Stored in side-table, exposed via accessor.
- **Structured data lost by parser** (Class 1-2) → new AST type in `svelte_ast` (e.g. `struct DestructuredBinding`). Parser populates it, codegen consumes it. Analyze may not be involved at all.

Show the full type definition (if any) and where it lives.

### Computation
- Which phase computes the new data: parser (Class 1-2) or analyze pass (Class 3-4)
- What raw facts feed into it (list the flags/checks from the finding)
- The decision logic, written as a pure function: `(flag_a, flag_b, ...) → result`

### Codegen simplification
- For EACH occurrence: show the before (current branching) and after (match on enum / single accessor call)
- The after code should be a flat match or single `if` with no residual flag logic
- If an occurrence has additional target-specific logic (e.g. argument formatting), that stays in codegen — only the DECISION moves out

### Data flow
- Order of changes: upstream type → computation → codegen consumer
- List files in modification order

**Present the plan and wait for approval before writing any code.**

If the finding spans Class 1 or 2 (re-parse / string re-parse):
- The plan must include parser changes (new AST node or extended existing node)
- Flag if this requires changes to `svelte_ast` public types — these affect all downstream crates

If a single finding requires changes across 5+ files, plan the first phase only and explain what remains.

If the proposed type doesn't cleanly replace ALL occurrences, flag the outliers — don't force them into the abstraction.

## Step 4: Implement

Apply the approved plan in this order:

1. **Add the type** (if needed) — enum/struct in `svelte_ast` or `svelte_analyze`
2. **Add computation** — parser populates AST field (Class 1-2) or analyze pass computes side-table entry (Class 3-4)
3. **Expose accessor** on AnalysisData or Ctx (e.g. `pub fn attr_needs_getter(&self, id: NodeId) -> bool`)
4. **Replace codegen logic** — one occurrence at a time, test between each:
   - Replace the flag combination / re-traversal / re-parse with a match on enum or accessor call
   - Delete dead code (unused flag reads, helper functions that only existed for the old pattern)
5. Run `just test-all` after each occurrence replacement

**Critical invariant: JavaScript output must not change.** These are pure refactors — the generated JS must be byte-identical before and after. If snapshot tests exist, they are your proof. If they don't exist for the affected code path, say so.

If tests fail: diff the expected vs actual JS output to understand what semantic change was introduced. Fix and retry. **Stop after 3 failed attempts** — report what you tried.

## Step 5: Clean up

- Remove any now-unused flags, helper functions, or intermediate variables from codegen
- If a `ctx` method in codegen was only used for the removed pattern, delete it
- If analyze computed flags that are now superseded by the new type, consider deprecating them (but only if no other consumer uses them — grep first)

## Step 6: Verify & Update

Run `just test-all`.

Update `AUDIT.md`: mark implemented findings with ~~strikethrough~~, add a one-line note:
```
~~#3 attr_needs_getter — added accessor to AnalysisData, replaced 3 occurrences in attributes.rs and boundary.rs~~
```

Skip strikethrough findings when running `all`.

Summarize:
- Which findings were implemented
- Which were skipped (and why)
- Whether JS output changed (it shouldn't)
- What was added: accessor method / enum / AST type
- Any new analyze pass added or existing pass modified
- What to verify manually (if anything)
