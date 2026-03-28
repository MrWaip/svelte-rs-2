---
name: phase-boundaries
description: Detailed phase boundary rules, red/green flags for codegen, OXC visitor rules, and additional architectural constraints. Loaded automatically when working on codegen, transforms, or analyze passes.
user-invocable: false
paths: "crates/svelte_codegen_client/**/*.rs,crates/svelte_analyze/**/*.rs,crates/svelte_transform/**/*.rs"
---

# Phase Boundaries: Fat Analyze, Dumb Codegen

Each compiler phase has a strict responsibility.

## Parser (`svelte_parser`)
Returns structured data. If a new AST node contains JS, the parser must deliver it parsed (via `parse_js`), not as a raw Span for downstream re-parsing.

## Analyze (`svelte_analyze`)
Answers semantic questions. If codegen needs to decide between output modes based on 2+ flags, analyze should pre-compute that decision and expose it as an enum or accessor. Codegen should never dig deeper than one method call into `AnalysisData`.

## Codegen (`svelte_codegen_client`)
Flat mapper. Match on enums, format output. Zero decision logic.

## Red flags in codegen (do not introduce)
- `oxc_parser::Parser::new()` -- re-parsing in codegen means parser missed structure
- `starts_with('[')`, `split(',')`, `split_once(':')` -- string parsing means AST lost structure
- `.iter().find(...)` + `.iter().filter_map(...)` + `.iter().any(...)` on the same collection -- repeated traversal means analyze should provide a summary
- `ctx.analysis.foo(id).and_then(|x| x.bar.first()).and_then(|r| r.baz).is_some_and(|s| ...)` -- deep chaining means analyze should expose an accessor
- `let needs_X = flag_a || (flag_b && flag_c)` combining 2+ analysis flags -- means analyze should pre-compute the decision

## Green flags in codegen (this is what we want)
```rust
// Single accessor call -> flat match
match ctx.attr_output_mode(id) {
    AttrOutputMode::Static => { ... }
    AttrOutputMode::DynamicGetter => { ... }
}
```

## OXC expression traversal

All traversal of OXC `Expression`, `Statement`, `Program` trees MUST use OXC visitor infrastructure. Hand-written multi-level matching on `Expression::*` variants is prohibited.

Visitor types:
- `Visit` -- read-only traversal (analysis, classification)
- `VisitMut` -- in-place mutation (transforms)
- `Traverse` -- mutation with parent/scope context (complex transforms)

Allowed per crate:
- **`svelte_analyze`** -- `Visit` / `VisitMut`
- **`svelte_transform`** -- `Visit` / `VisitMut` / `Traverse`
- **`svelte_codegen_client`** -- `Visit` / `VisitMut` / `Traverse`

Allowed exceptions (no visitor needed):
- Shallow destructure of a known top-level shape without descending into child expressions
- AST construction/mutation in `builder.rs`

Existing violations: marked `// TODO(oxc-visit)`.

## Additional rules
- `FxHashMap`/`FxHashSet` everywhere instead of std `HashMap`.
- Sub-struct fields in `AnalysisData` (`ElementFlags`, `FragmentData`, etc.) are `pub(crate)` -- use accessor methods from outside `svelte_analyze`. In codegen, prefer `Ctx` shortcuts over chained access.
- AST stores `Span` for JS expressions. `ParsedExprs<'a>` caches parsed OXC `Expression<'a>` ASTs (populated in `svelte_parser::parse_js`, consumed in transform/codegen). No JS subtree copying between phases.
- OXC and `ComponentScoping` share the same `SymbolId` space for script-level bindings, so `SymbolId` from OXC can be used directly with `ComponentScoping` methods without name round-tripping.
