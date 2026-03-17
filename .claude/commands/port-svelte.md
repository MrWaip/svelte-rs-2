# Port Svelte feature: $ARGUMENTS

Reference Svelte compiler is in `reference/compiler/`. Our Rust compiler is in `crates/svelte_*`.

The command argument is a feature description (e.g. `$derived`, `{@html}`, `style:prop`).
Before starting, read `ROADMAP.md`, find the matching item, and use the listed files and reference links.

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

## Step 1: Branch (MANDATORY — do this FIRST)

**You MUST create a feature branch before doing anything else. Do NOT skip this step. Do NOT work on master.**

```
git checkout master && git pull && git checkout -b port/<item>-<short-name>
```
where `<short-name>` is a brief kebab-case description of the feature (e.g. `derived-rune`, `html-tag`).

After running the command, verify you are on the new branch with `git branch --show-current`. If you are still on master, stop and fix this before proceeding.

## Step 2: Analysis & Checklist

Before writing any code, perform a comprehensive analysis of the feature in the Svelte reference compiler. This step discovers ALL use cases and edge cases so nothing is missed.

### 2a: Deep reference scan

Launch Explore agents (up to 3 in parallel) to search the Svelte reference for ALL use cases:

- `reference/compiler/phases/1-parse/` — how the feature is parsed, all syntax variants
- `reference/compiler/phases/2-analyze/visitors/` — analysis metadata, flags, special conditions
- `reference/compiler/phases/3-transform/client/visitors/` — codegen branches, edge case handling, conditional logic
- `reference/compiler/types/template.d.ts` — AST shape, optional fields, union variants
- `reference/compiler/tests/` — existing test examples and snapshot inputs
- Search for the feature name (and aliases) across ALL files to catch cross-cutting concerns

Pay special attention to:
- `if` / `switch` branches in transform visitors — each branch is often a distinct use case
- Optional AST fields — each represents a variant that may need separate handling
- Runtime helper calls — each distinct `$.helper()` call is a behavior to port
- Error/warning diagnostics — validation cases

### 2b: Produce categorized use-case list

Output a structured list grouped by category. Example:

```
### Basic
1. Simple usage: `{@html expr}`
2. With literal string

### Variants
3. With reactive state (`$state`)
4. With derived value (`$derived`)

### Edge cases
5. Empty expression
6. Nested inside component

### Interactions with other features
7. Inside {#if} block
8. Inside {#each} block
9. With spread attributes on parent

### Validation / Errors
10. Invalid placement (e.g. inside <script>)
11. Missing expression
```

Number every case sequentially. Mark cases we already handle (check existing test cases in `tasks/compiler_tests/cases2/`).

### 2c: Interactive checklist (multiple rounds)

Present discovered cases in **batches of up to 4** via multiple `AskUserQuestion` calls with `multiSelect: true`.

Each round covers one category:
- Round 1: "Basic & common" — up to 4 core use cases
- Round 2: "Variants & edge cases" — up to 4 cases
- Round 3: "Interactions with other features" — up to 4 cases
- Round 4: "Validation & errors" — up to 4 cases
- (additional rounds if more cases were discovered)

Each option = one specific use case with a short description. User multi-selects within each round.

After all rounds, output a summary:
```
Selected for porting (N cases): ...
Deferred to ROADMAP (M cases): ...
```

### 2d: Record deferred items in ROADMAP

For cases NOT selected by the user, add them to the **Deferred** section at the bottom of `ROADMAP.md`:
- Find or create a sub-heading `### <feature name> (Tier N)` under the **Deferred** section
- Add each deferred case as `- [ ] <description>`
- Do NOT scatter deferred items inside completed feature sections — they all go in **Deferred**

### 2e: Check what already exists

Before creating test cases, check what our compiler already handles:
- Run existing related tests if any
- Check if parser/AST already supports the syntax
- Note what's already done vs what needs new work

## Step 3: Test cases

Create one test case per selected use case from Step 2c.

For each case:
1. Create `tasks/compiler_tests/cases2/<feature>_<variant>/case.svelte` with a minimal Svelte component exercising that specific use case
2. Add test function in `tasks/compiler_tests/test_v3.rs`: `#[rstest] fn <test_name>() { assert_compiler("<test_name>"); }`

After creating ALL case files, run `just generate` ONCE to generate all `case-svelte.js` files. If this fails, stop and report the error.

Rules:
- **NEVER edit `case-svelte.js` or `case-rust.js` files.** These are generated.
- Keep each `case.svelte` minimal — test one thing per case
- Use snake_case for test names: `<feature>_<variant>` (e.g. `html_tag_basic`, `html_tag_reactive`, `html_tag_in_if`)

## Step 4: Parser & AST

Check if `case.svelte` uses syntax not yet supported by our parser.

- Compare with `reference/compiler/types/template.d.ts` for AST node shapes
- If new node types, attributes, or directives are needed:
  1. Add types to `crates/svelte_ast/src/lib.rs`
  2. Add parsing to `crates/svelte_parser/src/lib.rs` (and scanner if new tokens needed)
  3. Add parser tests following `/test-pattern`

## Step 5: Analysis

Read the Svelte analysis visitors in `reference/compiler/phases/2-analyze/visitors/` to understand what metadata the feature needs.

- Check what `AnalysisData` fields the codegen needs for this feature
- Verify our `AnalysisData` has equivalent data
- If not, add or extend a pass in `crates/svelte_analyze/src/`
- Add analyze tests following `/test-pattern`

## Step 6: Codegen

Read the Svelte transform visitor in `reference/compiler/phases/3-transform/client/visitors/`.

See the navigation table in CLAUDE.md to find the corresponding Svelte reference and our module.

Implement in the corresponding `svelte_codegen_client` module.

Key differences from Svelte:
- We use direct recursive functions, not AST walker (zimmerframe)
- We use `AnalysisData` side tables, not mutated AST metadata
- We store `Span` and re-parse in codegen via `svelte_js`, not stored expressions
- Our `$.template()` builds equivalent calls to Svelte's `html` tagged template

## Step 7: Verify

Run each test case:
```
just test-case <test_name>
```

Compare `case-rust.js` vs `case-svelte.js`. Fix mismatches. Ensure all existing tests still pass:
```
just test-compiler
```

If the test still fails after 3 attempts, stop and report what you've tried.

## Step 8: Update tracking

Update `ROADMAP.md`:
- Move the completed feature to the **Done ✅** section
- If new deferred items were discovered during implementation — add them to the **Deferred** section at the bottom


## Step 9: Benchmark

If the ported feature adds new syntax or constructs (new AST node types, new block types,
new directive types), update the benchmark generator to include them:

1. Add the new construct to `tasks/generate_benchmark/src/main.rs` (in the chunk template)
2. Bump the benchmark version: generate a new `big_vN.svelte`:
   ```
   just generate-benchmark big_vN
   ```
   where N is the next version number (check existing files in `tasks/benchmark/benches/compiler/`)
3. Verify the new benchmark file compiles: `cargo bench -p benchmark -- --test`
4. Do NOT modify or delete previous `big_vN.svelte` files — their CodSpeed history must remain valid

Skip this step if the feature only changes codegen output without adding new syntax.
