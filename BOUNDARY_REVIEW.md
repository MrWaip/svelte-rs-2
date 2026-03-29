# Boundary & Data Flow Review — `svelte_analyze`, `svelte_transform`

Generated: 2026-03-29

---

### #1 — Proxyability predicate duplicated across analyze and transform

**Class**: 5 (Late knowledge)
**Impact**: warning
**Occurrences**: 2 locations across 2 crates
**Evidence**:
- `crates/svelte_analyze/src/passes/js_analyze.rs:343-361` — `is_proxyable_state_init` extracts the first call argument, then checks: `is_literal → false`, `TemplateLiteral|Arrow|Function|Unary|Binary → false`, `Identifier("undefined") → false`, else `true`
- `crates/svelte_transform/src/rune_refs.rs:214-234` — `should_proxy` applies the exact same predicate: `is_literal → false`, same 5-variant match `→ false`, `Identifier("undefined") → false`, else `true`

**Verified**:
- The predicates are character-for-character identical in their core logic.
- `is_proxyable_state_init` is private to analyze, called once in `ScriptBodyAnalyzer` to classify init expressions.
- `should_proxy` is `pub` in transform, called once in `ExprTransformer` for rune assignment RHS.
- They operate on DIFFERENT expressions (init-time vs assignment-time), so neither can replace the other — but they encode the same domain knowledge.

**Root cause**: The "is this expression proxyable?" predicate was independently implemented in both crates. Analyze runs it on `$state(init)` to set `is_proxy_init`; transform runs it on assignment RHS to decide `$.set(name, val, true)`. Same concept, no shared implementation.

**Fix**: Move the core predicate to `svelte_analyze` as a `pub` function (e.g. `pub fn is_proxyable_value(expr: &Expression) -> bool` in a utils module). Transform already depends on analyze, so it can import directly. `is_proxyable_state_init` becomes a thin wrapper that extracts the call argument and delegates. `should_proxy` in transform becomes a re-export or direct call.

**Simplifies**: Eliminates 20 lines of duplicated logic. Future changes to proxyability rules only need one update.

---

### #2 — `node_scope`/`await_catch_scope` shims discard caller's FragmentKey knowledge

**Class**: 5 (Late knowledge)
**Impact**: suggestion
**Occurrences**: 4 locations in 1 file (+ 2 shim definitions)
**Evidence**:
- `crates/svelte_transform/src/lib.rs:111` — `node_scope(block.id)` for EachBlock — caller knows it's `FragmentKey::EachBody`
- `crates/svelte_transform/src/lib.rs:120` — `node_scope(block.id)` for SnippetBlock — caller knows it's `FragmentKey::SnippetBody`
- `crates/svelte_transform/src/lib.rs:182` — `node_scope(block.id)` for AwaitBlock/then — caller knows it's `FragmentKey::AwaitThen`
- `crates/svelte_transform/src/lib.rs:186` — `await_catch_scope(block.id)` for AwaitBlock/catch — caller knows it's `FragmentKey::AwaitCatch`
- `crates/svelte_analyze/src/scope.rs:100-104` — `node_scope` tries 3 FragmentKey variants sequentially
- `crates/svelte_analyze/src/scope.rs:108-109` — `await_catch_scope` wraps a single `fragment_scope` call

**Verified**:
- `node_scope` does up to 3 hash lookups (`EachBody`, then `SnippetBody`, then `AwaitThen`) when the caller already knows which one is correct.
- Both shims have `TODO: migrate callers in svelte_transform to use fragment_scope() directly` comments.
- `fragment_scope(&FragmentKey::X(id))` is the correct API — it's already public and used throughout analyze's own walker.

**Root cause**: The shims were introduced as transitional API during the scope system migration. Callers in transform were not updated.

**Fix**: Replace each call site with the direct `fragment_scope` call:
```rust
// EachBlock (line 111):
ctx.analysis.scoping.fragment_scope(&FragmentKey::EachBody(block.id)).unwrap_or(scope)
// SnippetBlock (line 120):
ctx.analysis.scoping.fragment_scope(&FragmentKey::SnippetBody(block.id)).unwrap_or(scope)
// AwaitBlock then (line 182):
ctx.analysis.scoping.fragment_scope(&FragmentKey::AwaitThen(block.id)).unwrap_or(scope)
// AwaitBlock catch (line 186):
ctx.analysis.scoping.fragment_scope(&FragmentKey::AwaitCatch(block.id)).unwrap_or(scope)
```
Then delete `node_scope` and `await_catch_scope` from `ComponentScoping`.

**Simplifies**: Removes 2 shim methods (10 lines), eliminates up to 2 unnecessary hash lookups per call, removes the TODO comments.

---

## Cross-layer scan evidence

### What was checked (mandatory)

**Parser → Analyze:**
- No re-parsing found. JS expressions are pre-parsed by `parse_with_js`. Analyze reads structured AST only.
- `classify_render_tags` mutates `parsed.exprs` (unwraps ChainExpression → CallExpression). This is AST normalization, not re-parsing — the expression was already parsed; analyze just simplifies the wrapper for downstream convenience. Borderline but justified.

**Analyze → Transform:**
- Transform reads scoping data through proper `ComponentScoping` accessors (`rune_kind`, `is_mutated`, `is_prop_source`, `find_binding`, etc.) — no string-based classification beyond what the scoping API provides.
- `$state.eager`, `$state.snapshot`, `$effect.pending` string matching in transform (lib.rs:449-486) is justified: these are unresolved global magic names with no SymbolId. The compiler treats them as special forms, not as bindings.
- Transform's template walker (lib.rs:66-196) mirrors analyze's walker but is necessarily separate — transform needs `VisitMut` (mutable AST access) while analyze's walker is read-only.

**Multi-flag decisions in transform:**
- Transform's identifier classification cascade (lib.rs:307-377) reads ONE scoping flag per branch. No multi-flag combinations that should be pre-computed as an enum. Each flag is already pre-computed by analyze.
- `let needs_get = is_mutated(sym) || kind.is_derived()` (lib.rs:371) is a 2-flag check, but it's the only consumer of this particular combination. Pre-computing would be over-engineering.

**AST re-traversal:**
- Transform's attribute iteration (lib.rs:212-248) is for OUTPUT MUTATION, not data collection — it must visit each attribute to transform expressions. Not a violation.
- No `iter().find().filter_map()` patterns for data collection in transform.

---

## Summary

| Impact | Count |
|--------|-------|
| critical | 0 |
| warning | 1 |
| suggestion | 1 |
| **Total** | **2** |

The `svelte_analyze` ↔ `svelte_transform` boundary is clean. Transform correctly consumes analyze's pre-computed data through proper accessors. The two findings are:
1. A duplicated predicate that encodes the same domain knowledge independently
2. A documented TODO for migrating away from compatibility shims
