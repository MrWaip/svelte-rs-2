---
name: phase-boundaries
description: Strict compiler phase-boundary rules for parser, analyze, transform, and client codegen. Use when deciding where new logic belongs, when adding or changing `AnalysisData` accessors, when reviewing boundary hygiene, or when writing OXC traversal in `svelte_analyze`, `svelte_transform`, or `svelte_codegen_client`.
---

# Phase Boundaries

Keep logic in the correct layer.

## Parser (`svelte_parser`)

Return structured data. If a new AST node contains JS, parse it in the parser and hand downstream the parsed result instead of a raw string or span that must be re-parsed later.

## Analyze (`svelte_analyze`)

Own semantic questions and derived facts. If codegen would need to combine multiple flags, re-traverse AST, or classify identifiers/tags/attributes, compute that once in analyze and expose an enum or accessor on `AnalysisData`.

## Codegen (`svelte_codegen_client`)

Stay flat. Consume AST plus analysis data and emit JS. Match on enums, use accessors, and format output. Do not rediscover facts that analyze already knows.

## Transform (`svelte_transform`)

Rewrite parsed JS AST. Follow the same boundary rules as codegen: no semantic rediscovery by string matching, no parser recreation, no duplicated side tables outside analysis.

## Red flags

- `oxc_parser::Parser::new()` in analyze, transform, or codegen
- string parsing such as `starts_with`, `split`, `split_once`, or ad hoc text classification for identifiers, tags, or attributes
- repeated `.iter().find(...)`, `.iter().filter_map(...)`, `.iter().any(...)` over the same collection to gather derived facts
- deep chaining into `AnalysisData` internals instead of using a dedicated accessor
- combining 2+ analysis booleans in codegen/transform to decide an output mode
- codegen-local enums or structs that cache facts analyze should already own

## Green flags

```rust
match ctx.attr_output_mode(id) {
    AttrOutputMode::Static => { ... }
    AttrOutputMode::DynamicGetter => { ... }
}
```

If an output decision looks like this, the boundary is usually correct: one accessor call, one flat match, no rediscovery.

## OXC traversal rules

Use OXC visitor infrastructure for real JS traversal.

- `svelte_analyze`: `Visit` or `VisitMut`
- `svelte_transform`: `Visit`, `VisitMut`, or `Traverse`
- `svelte_codegen_client`: `Visit`, `VisitMut`, or `Traverse`

Allowed exceptions:

- shallow destructure of a known top-level shape without descending into children
- AST construction or mutation in `builder.rs`

If you need method signatures, load:

- `.codex/skills/oxc-analyze-api/references/visit-methods.txt`
- `.codex/skills/oxc-codegen-api/references/traverse-methods.txt`
- `crates/svelte_component_semantics/src/lib.rs` (scoping API)

## Additional rules

- Prefer `SymbolId` and analysis accessors over identifier-name strings.
- Use `FxHashMap` and `FxHashSet` instead of std hash collections.
- Treat existing violations as debt, not precedent.
- If codegen needs a summary of structure, add it in analyze instead of caching it locally in codegen.
