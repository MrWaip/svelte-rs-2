# Migrate next OXC Visit stage

Execute the next incomplete stage from `MIGRATION_OXC_VISIT.md`.

## Step 1: Determine next stage

Read `MIGRATION_OXC_VISIT.md`. Find the first stage that is NOT marked `✅ DONE`.

If `$ARGUMENTS` is provided (e.g. `1`, `3`, `5`), execute that specific stage instead.

If all stages are done, report completion and stop.

## Step 2: Read current code

For each function listed in the target stage's table:

1. Read the full function implementation (use LSP `documentSymbol` + Read)
2. Read all call sites (use LSP `findReferences`)
3. Understand the exact semantics — what inputs, what outputs, what edge cases

Also read:
- `CODEBASE_MAP.md` for type signatures
- The existing `Visit`/`VisitMut` implementations in the crate (understand the patterns already in use)

## Step 3: Design the visitor

Produce a concrete design:

1. **Struct fields** — what data the visitor accumulates
2. **Which `visit_*` methods** — list each method and what it replaces
3. **Walk calls** — which methods call `walk_*` for recursion, which don't (leaf handlers)
4. **Entry point** — function signature that creates the visitor, runs it, returns results
5. **Call site changes** — for each current call site, show before → after

**Present the design and wait for approval before writing code.**

## Step 4: Implement

Apply in this order:

1. **Add the visitor struct + impl** in the target file
2. **Add entry point function**
3. **Replace call sites one at a time** — after each replacement, run `just test-all`
4. **Delete old functions** — only after all call sites are migrated
5. **Final `just test-all`** — must be green

**Critical invariant**: JS output must not change. These are pure refactors.

If tests fail: diff expected vs actual output, understand the semantic difference, fix. Stop after 3 failed attempts — report what you tried.

## Step 5: Mark TODO on remaining manual code

For any manual `Expression::` matching that remains in the modified files but is NOT
part of this stage, add `// TODO(oxc-visit): replace with Visit` if not already present.

## Step 6: Update migration plan

Edit `MIGRATION_OXC_VISIT.md`:
- Mark the completed stage with `✅ DONE` in the heading
- Update line numbers if they shifted

Run `just test-all` one final time.

## Step 7: Report

Summarize:
- Stage completed
- Functions replaced (count)
- Lines removed vs added
- Any issues encountered
- Next stage to tackle
