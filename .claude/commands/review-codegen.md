# Review: Codegen Crate

You are reviewing the **codegen crate** (`crates/svelte_codegen_client/`) of a Svelte compiler written in Rust, built on top of oxc. It consumes the parsed AST + analysis side-tables + pre-transformed expressions and produces JavaScript output.

## Preparation

Read `CODEBASE_MAP.md` and `CLAUDE.md`. Then read all `.rs` files in `crates/svelte_codegen_client/src/`. The author is a senior Rust developer — skip trivial suggestions.

Use up to 3 parallel Explore agents to read the source files and perform the review.

## Focus Areas (in priority order)

### 1. Output Correctness (highest priority)

- **Svelte runtime contract**: Does generated code match what Svelte runtime expects? Component initialization, update functions, reactivity invalidation, lifecycle hooks, binding setup, event handlers.
- **Variable reference integrity**: Template vars vs script vars, destructured props, each-block variables — always refer to the right thing?
- **Control flow correctness**: `{#if}`, `{#each}`, `{#await}`, `{#key}` — DOM fragments created/destroyed/updated correctly? Keyed vs unkeyed each?
- **Attribute & directive output**: Static vs dynamic attributes, spread, `class:`, `style:`, event handlers with modifiers, `bind:`, `transition:`, `animate:`, `use:` — each has different codegen rules.
- **Compare with reference output**: Check test fixtures in `tasks/compiler_tests/cases2/` for expected vs actual output.

### 2. Coupling to Analysis Side-Tables

- **Lookup safety**: Every `analysis.get(node_id)` is a potential crash. Are there missing entries that would cause panics?
- **Data completeness**: Does codegen re-walk the AST to derive info the analyzer should have provided?
- **Stale data risk**: If NodeIds could theoretically change between phases, does codegen guard against this?

### 3. Code Generation Quality

- **Output readability**: Is generated JS reasonably readable? (Svelte's output is famously readable.)
- **String building approach**: Proper code builder with indentation tracking, or raw `String::push_str`?
- **oxc codegen reuse**: Using oxc's codegen/printer for JS expressions, or reimplementing?

### 4. Source Map Support

- **Presence**: Any source map infrastructure?
- **Span propagation**: Source spans carried through to output?

### 5. Structure & Maintainability

- **Template codegen organization**: Manageable pieces, or monolithic functions?
- **Helper/runtime references**: `import` statements for runtime helpers centralized or scattered as string literals?

## Output Format

```
## Summary
[2-3 sentences: does codegen produce correct output? Biggest risk?]

## Critical Issues
[Wrong output, runtime crashes, broken reactive updates]
### [Title]
- **File**: path:line
- **Problem**: ...
- **Suggestion**: ...

## Important Improvements
[Coupling issues, missing source maps, structural problems]

## Minor Notes
[Max 3-5]

## Questions for the Author
[Which Svelte version targeted? CSS handling deferred?]

## What's Done Well
[2-3 strengths]
```

## Rules

- Max ~15 findings total. Prioritize ruthlessly.
- Always include file paths and line numbers.
- The most important thing is **output correctness**. A beautifully structured codegen that emits wrong JS is worthless.
- Check ROADMAP.md for what's explicitly not yet implemented.
- When flagging incorrect output, show both what's generated and what should be generated.
- Do not suggest switching to a template engine or code generation framework unless the current approach is demonstrably broken.
