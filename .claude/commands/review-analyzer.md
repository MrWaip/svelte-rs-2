# Review: Analyzer Crate

You are reviewing the **analyzer crate** (`crates/svelte_analyze/`) of a Svelte compiler written in Rust, built on top of oxc. The analyzer walks the AST and produces **side-tables** (`AnalysisData`) that annotate nodes with information needed for transform and codegen.

## Preparation

Read `CODEBASE_MAP.md` and `CLAUDE.md`. Then read all `.rs` files in `crates/svelte_analyze/src/`. The author is a senior Rust developer — skip trivial suggestions.

Use up to 3 parallel Explore agents to read the source files and perform the review.

## Focus Areas (in priority order)

### 1. Side-Table Architecture (highest priority)

- **Completeness**: Is every piece of information that downstream phases (transform, codegen) need actually stored? Or do they have to re-derive results by walking the AST?
- **Consistency**: Are side-table lookups safe? What happens when codegen asks for data on a NodeId that wasn't visited? `Option`-based or `unwrap()`?
- **Key correctness**: Is `NodeId` stable and unique throughout the pipeline?
- **Data granularity**: Too coarse (one giant struct) or too fragmented (20 HashMaps to cross-reference)?

### 2. Svelte Reactivity Analysis

- **Reactive declarations** (`$:` labels): Dependencies tracked correctly? Complex cases — `$: foo = bar + baz`? Topological ordering?
- **Reactive assignments**: `count += 1` vs `obj.prop = val` vs `array.push(val)` vs `array[i] = val` — treated differently by Svelte.
- **Component bindings**: `bind:value`, `bind:this` — data flow correctly modeled?
- **Store subscriptions** (`$store`): Auto-subscription tracked? Edge cases?

### 3. Scope & Binding Resolution

- **Svelte-specific scopes**: `{#each items as item, index (key)}`, `{#await promise then value}` — correctly nested, don't leak?
- **`<script>` vs template scoping**: Clean boundary?
- **Shadowing**: Nested `{#each}` with same variable name?
- **Interaction with oxc's scope analysis**: Reusing or duplicating?

### 4. Analysis Phase Ordering

- **Dependencies between analyses**: Is ordering enforced or implicit/fragile?
- **Single pass vs multi-pass**: Each pass clearly scoped?
- **Visitor pattern**: Using the composite `TemplateVisitor` pattern correctly?

### 5. Error Reporting

- **Semantic errors**: Undefined variables, invalid directives — caught here?
- **Error spans**: Point to right location?
- **Warnings**: Infrastructure for Svelte-style warnings?

## Output Format

```
## Summary
[2-3 sentences: is the side-table architecture sound? Biggest risk area?]

## Critical Issues
[Soundness problems, incorrect analysis, data loss between phases]
### [Title]
- **File**: path:line
- **Problem**: ...
- **Suggestion**: ...

## Important Improvements
[Architectural concerns, missing analysis, fragile phase ordering]

## Minor Notes
[Max 3-5 items]

## Questions for the Author
[Intentional limitations? Why approach X over Y?]

## What's Done Well
[2-3 strengths]
```

## Rules

- Max ~15 findings total. Prioritize ruthlessly.
- Always include file paths and line numbers.
- The side-table pattern is an intentional choice — don't suggest replacing it with a transformed AST. Evaluate whether it's applied well.
- Check ROADMAP.md before flagging missing analyses — some may be explicitly planned.
- If something looks like it duplicates oxc Semantic, flag it specifically.
