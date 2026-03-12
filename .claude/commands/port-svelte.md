# Port Svelte feature: $ARGUMENTS

Reference Svelte compiler is in `reference/compiler/`. Our Rust compiler is in `crates/svelte_*`.

The command argument is a roadmap item from `TODO_ANALYZE.md` (e.g. `1a`, `2b`, `3e`).
Before starting, read `TODO_ANALYZE.md`, find the item, and use the listed files and reference links.

## Approach

Use Svelte reference to understand the **expected output**, not to copy the implementation.

Do NOT port:
- Visitor/walker dispatch patterns (we use direct recursive functions)
- Mutable AST metadata (we use AnalysisData side tables)
- JS-specific workarounds (nullish checks on arrays, var hoisting patterns)
- Intermediate abstractions that only exist for zimmerframe compatibility

DO:
- Match the JS output exactly (same function calls, same argument order)
- Simplify control flow when Rust makes it natural (match, iterators, Option)
- Keep functions short and focused — if a Svelte visitor does 5 things, split into clear helpers

## Step 1: Branch

Create a feature branch from the latest master:
```
git checkout master && git pull && git checkout -b port/<item>-<short-name>
```
where `<item>` is the roadmap item (e.g. `1a`) and `<short-name>` is a brief kebab-case description of the feature. All work must be done on this branch.

## Step 2: Test case

- Check `reference/compiler/tests/` for existing test examples of this feature
- Create `tasks/compiler_tests/cases2/<test_name>/case.svelte` with a minimal example of the feature
- Run `cargo run -p generate_test_cases` to generate `case-svelte.js` (expected output from Svelte v5). If this fails, stop and report the error.
- Add test function in `tasks/compiler_tests/test_v3.rs`: `fn <test_name>() { assert_compiler("<test_name>"); }`

## Step 3: Parser & AST

Check if `case.svelte` uses syntax not yet supported by our parser.

- Compare with `reference/compiler/types/template.d.ts` for AST node shapes
- If new node types, attributes, or directives are needed:
  1. Add types to `crates/svelte_ast/src/lib.rs`
  2. Add parsing to `crates/svelte_parser/src/lib.rs` (and scanner if new tokens needed)
  3. Add parser tests following `/test-pattern`

## Step 4: Analysis

Read the Svelte analysis visitors in `reference/compiler/phases/2-analyze/visitors/` to understand what metadata the feature needs.

- Check what `AnalysisData` fields the codegen needs for this feature
- Verify our `AnalysisData` has equivalent data
- If not, add or extend a pass in `crates/svelte_analyze/src/`
- Add analyze tests following `/test-pattern`

## Step 5: Codegen

Read the Svelte transform visitor in `reference/compiler/phases/3-transform/client/visitors/`.

See the navigation table in CLAUDE.md to find the corresponding Svelte reference and our module.

Implement in the corresponding `svelte_codegen_client` module.

Key differences from Svelte:
- We use direct recursive functions, not AST walker (zimmerframe)
- We use `AnalysisData` side tables, not mutated AST metadata
- We store `Span` and re-parse in codegen via `svelte_js`, not stored expressions
- Our `$.template()` builds equivalent calls to Svelte's `html` tagged template

## Step 6: Verify

Run the test:
```
cargo test -p compiler_tests --test compiler_tests_v3 <test_name>
```

Compare `case-rust.js` vs `case-svelte.js`. Fix mismatches. Ensure all existing tests still pass:
```
cargo test -p compiler_tests --test compiler_tests_v3
```

If the test still fails after 3 attempts, stop and report what you've tried.

## Step 7: Update tracking

Update `TODO_ANALYZE.md`:
- Mark `[x]` for completed checkboxes
- If partially implemented — mark the heading as `(partial)` and add a "Not implemented" subsection with specific `- [ ]` items
- If new tests were added — record their names
- If new subtasks were discovered — add them as `- [ ]`

This is critical: without an up-to-date `TODO_ANALYZE.md`, the next session won't know what's done and what's not.
