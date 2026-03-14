# Review: Transform Crate

You are reviewing the **transform crate** (`crates/svelte_transform/`) of a Svelte compiler written in Rust. This pass runs between analyze and codegen: it rewrites pre-parsed OXC Expression ASTs in-place, replacing rune references, prop accesses, each-block variables, and snippet parameters with their runtime equivalents.

## Preparation

Read `CODEBASE_MAP.md` and `CLAUDE.md`. Then read all `.rs` files in `crates/svelte_transform/src/`. Also skim `crates/svelte_analyze/src/data.rs` (AnalysisData) and `crates/svelte_analyze/src/scope.rs` (Scoping) since transform depends heavily on them. The author is a senior Rust developer — skip trivial suggestions.

Use up to 2 parallel Explore agents to read the source files and perform the review.

## Focus Areas (in priority order)

### 1. Expression Coverage

The `walk_expr_children` function must recurse into all expression types that can contain identifiers. A missing arm means silently skipping transforms inside that expression — a subtle bug.

- **Exhaustiveness**: Compare the match arms in `walk_expr_children` against OXC's `Expression` enum. Are any expression types that can contain child expressions missing? Pay special attention to: `TaggedTemplateExpression`, `NewExpression`, `AwaitExpression`, `YieldExpression`, `SpreadElement`, `ChainExpression`, `TSAsExpression`.
- **Statement coverage in `transform_stmt`**: Same check — `VariableDeclaration`, `IfStatement`, `ForStatement`, `ThrowStatement` etc. Could arrow bodies contain these?
- **Wildcard arm**: The `_ => {}` catch-all in both functions silently skips unknown expression/statement types. Is this acceptable or a correctness risk?

### 2. Transform Correctness

Each rewrite rule must produce the exact runtime calls Svelte expects:

- **Rune get/set/update**: `$.get(name)` for mutated state / derived; `$.set(name, value)` for assignments; `$.update(name)` / `$.update_pre(name)` for `++`/`--`. Are edge cases handled? (`+=`, `||=`, compound assignments)
- **Prop sources → thunk call**: `name()`. Correct for all prop source scenarios?
- **Prop non-sources → `$$props.name`**: Is the prop name vs local name distinction correct?
- **Each-block vars → `$.get(name)`**: Does this apply only to each-block context variables, not the iterable expression itself?
- **Snippet params → thunk call**: `name()`. Are nested snippets handled?

### 3. Scope Tracking

- **`walk_node` scope transitions**: Does EachBlock correctly switch to child scope for body but parent scope for fallback?
- **Arrow function shadowing**: Does the shadow stack correctly prevent transforms inside `(x) => x + 1` where `x` shadows a rune?
- **Destructured arrow params**: Does `extract_arrow_params` handle all binding patterns (object, array, default values, rest)?
- **SnippetBlock scope**: Are snippet parameters correctly tracked and passed through to nested expressions?
- **Missing scope transitions**: Are there any node types that create new scopes but aren't handled in `walk_node`?

### 4. AnalysisData Coupling

- **Scoping lookups**: `find_binding`, `node_scope`, `rune_kind`, `is_mutated`, `symbol_scope_id` — are all these guaranteed to return valid data for every node the transform visits?
- **Props analysis**: What if `analysis.props` is `None`? Is the guard sufficient?
- **Snippet params**: What if a SnippetBlock's id is missing from `analysis.snippet_params`?
- **Missing entries**: Are there any `unwrap()` calls on analysis lookups that could panic?

### 5. AST Builder Usage (`rune_refs.rs`)

- **Correctness**: Do `make_rune_get`, `make_rune_set`, `make_rune_update`, `make_thunk_call`, `make_props_access` produce valid OXC AST nodes?
- **SPAN usage**: All nodes use `SPAN` (zero span). Is this correct for codegen, or will it cause issues with source maps / diagnostics?
- **Allocator lifetime**: Is the allocator lifetime `'a` threaded correctly through all builder calls?
- **`make_rune_set` placeholder**: The `std::mem::replace` with a dummy `make_rune_get(ctx.alloc, "")` — is this safe? Could the dummy node ever be observed?

## Output Format

```
## Summary
[2-3 sentences: is the transform pass sound? Biggest risk area?]

## Critical Issues
[Missing expression types, incorrect transforms, scope bugs]
### [Title]
- **File**: path:line
- **Problem**: ...
- **Suggestion**: ...

## Important Improvements
[Coverage gaps, coupling risks, robustness]

## Minor Notes
[Max 3-5]

## Questions for the Author
[Intentional limitations? Planned extensions?]

## What's Done Well
[2-3 strengths]
```

## Rules

- Max ~15 findings total. Prioritize ruthlessly.
- Always include file paths and line numbers.
- The transform pass is deliberately minimal — it only handles expression-level rewrites needed for Svelte's runtime. Don't suggest expanding scope beyond what Svelte requires.
- Check ROADMAP.md before flagging missing features.
- Expression coverage gaps are **critical** — they cause silent correctness bugs.
