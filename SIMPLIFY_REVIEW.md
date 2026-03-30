# SIMPLIFY_REVIEW

## Audit coverage (to avoid superficial review)

Reviewed all source files under:
- `crates/svelte_analyze/src` (31 files)
- `crates/svelte_transform/src` (3 files)
- `crates/svelte_codegen_client/src` (33 files)

Total: **67 files**, ~**21,225 LOC**.

Additionally inspected complexity hotspots by size and role:
- `svelte_analyze`: `lib.rs`, `types/data.rs`, `passes/js_analyze.rs`, `walker.rs`, `scope.rs`
- `svelte_transform`: `lib.rs` (main walker)
- `svelte_codegen_client`: `context.rs`, `template/*`, `script/traverse.rs`, `builder.rs`

---

## Prioritized refactor roadmap (highest long-term leverage first)

### 1) Typed pass graph for `svelte_analyze` orchestration

**Why #1:** the biggest systemic risk is hidden pass dependencies.

**Evidence observed**
- `analyze_with_options` manually encodes pass order and invariants.
- multiple independent `walk_template(...)` phases in one function.
- comments carry dependency rules instead of types/contracts.

**Proposal**
- Introduce pass descriptors with explicit `requires` / `produces`.
- Keep current runtime order, but derive it from declared dependencies.
- Enforce ordering correctness in one place (scheduler) rather than in comments.

**Expected benefit**
- fewer regressions during feature ports.
- easier onboarding (dependency graph beats large imperative function).

**Primary targets**
- `crates/svelte_analyze/src/lib.rs`
- `crates/svelte_analyze/src/passes/mod.rs`

---

### 2) Split codegen `Ctx` into `Query` + `State`

**Why #2:** current context is a high-coupling center (read/write/lookup/emission all mixed).

**Evidence observed**
- `context.rs` combines:
  - AST accessors,
  - analysis forwarders,
  - generation mutable state,
  - async blocker plumbing,
  - event delegation dedup.
- many thin forwarding methods mirror internal layout of `AnalysisData`.

**Proposal**
- `CodegenQuery<'a>`: immutable semantic/AST queries.
- `CodegenState<'a>`: mutable emission state only.
- `Ctx` as transitional wrapper for incremental migration.

**Expected benefit**
- lower cognitive load per module.
- easier isolated tests for query logic and for emission logic.
- less accidental cross-feature coupling.

**Primary targets**
- `crates/svelte_codegen_client/src/context.rs`
- `crates/svelte_codegen_client/src/template/*.rs`

---

### 3) Unified template scope resolver in transform

**Why #3:** repeated fallback policy is a correctness trap when scope rules evolve.

**Evidence observed**
- repeated `fragment_scope(...).unwrap_or(scope)` / `node_scope(...).unwrap_or(scope)` across node kinds.
- await branches use multiple scope sources with similar fallback semantics.

**Proposal**
- Add `TemplateScopeResolver` with explicit APIs:
  - `scope_for_if_consequent(...)`
  - `scope_for_if_alternate(...)`
  - `scope_for_each_body(...)`
  - `scope_for_await_then/catch/pending(...)`
- keep fallback logic centralized.

**Expected benefit**
- fewer scope regressions, easier audits.

**Primary targets**
- `crates/svelte_transform/src/lib.rs`

---

### 4) Replace offset plumbing with typed expression keys

**Why #4:** positional contracts (`span.start`) leak parser internals into downstream phases.

**Evidence observed**
- widespread map access by offsets for expr/stmts.
- helper methods exist to bridge node-id to offset, indicating impedance mismatch.

**Proposal**
- parser assigns stable typed keys (`ExprKey`, `StmtKey`).
- analyze stores mapping `NodeId/AttrId -> ExprKey`.
- transform/codegen consume typed keys only (no raw offsets).

**Expected benefit**
- cleaner inter-phase API, fewer brittle lookups.

**Primary targets**
- `svelte_parser::types` (`ParserResult`)
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/context.rs`
- `crates/svelte_transform/src/lib.rs`

---

### 5) Normalize async codegen policy into one reusable abstraction

**Why #5:** async wrapping pattern is repeated across multiple template modules.

**Evidence observed**
- repeated pattern: `has_await`, `needs_async`, build thunk, emit block, wrap via async helper.
- appears in `if_block`, `each_block`, `key_block`, `html_tag`, `svelte_element`, `render_tag` paths.

**Proposal**
- add `AsyncEmissionPlan` (enum/struct) prepared once per node.
- expose one method that returns either plain call args or async wrapper statement.

**Expected benefit**
- fewer drift bugs between block generators.
- easier to change async semantics globally.

**Primary targets**
- `crates/svelte_codegen_client/src/template/*.rs`
- `crates/svelte_codegen_client/src/context.rs`

---

### 6) Pass-bundle API for multi-visitor analyze walks

**Why #6:** visitors are grouped implicitly today; this should be explicit and typed.

**Evidence observed**
- walk setup manually assembles `v1/v2/...` per phase.
- dependencies between bundles currently documented in comments.

**Proposal**
- create bundle structs: `SemanticBundle`, `ReactivityBundle`, `ElementBundle`.
- bundle constructor validates required prerequisites.

**Expected benefit**
- clearer ownership of each walk.
- smaller `lib.rs` orchestration surface.

**Primary targets**
- `crates/svelte_analyze/src/lib.rs`
- `crates/svelte_analyze/src/walker.rs`

---

### 7) Expand enum-first decision modeling beyond current islands

**Why #7:** project already proved this pattern works (`RenderTagCalleeMode`, `EventHandlerMode`).

**Evidence observed**
- several domains still encode decisions via bool combinations (dynamic/state/await/blocker style flags).

**Proposal**
- define domain enums for multi-flag decisions where invalid combinations exist.
- keep booleans only for truly independent orthogonal facts.

**Expected benefit**
- fewer impossible states.
- better `match`-driven readability and compiler assistance.

**Primary targets**
- `crates/svelte_analyze/src/types/data.rs`
- async + binding-related template emission modules

---

### 8) Stabilize downstream-safe analysis query surface

**Why #8:** downstream modules are coupled to inner `AnalysisData` layout.

**Evidence observed**
- many forwarders in `Ctx` directly mirror inner tables.
- changing a table shape risks broad churn in codegen.

**Proposal**
- define `analysis_queries` module/trait with semantic questions.
- treat nested tables as analyze-internal implementation detail.

**Expected benefit**
- lower churn during data model refactors.
- cleaner boundary between analyze and codegen.

**Primary targets**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/context.rs`

---

### 9) Introduce a shared template dispatch skeleton (without violating phase boundaries)

**Why #9:** transform and codegen both manually route by node kind.

**Evidence observed**
- `match Node::...` traversal skeleton duplicated conceptually across crates.

**Proposal**
- share only dispatch skeleton + child traversal hooks.
- keep semantics in each crate (no cross-phase leakage).

**Expected benefit**
- lower cost of adding new node kinds.
- fewer missed branches during feature additions.

**Primary targets**
- `crates/svelte_transform/src/lib.rs`
- `crates/svelte_codegen_client/src/template/*`

---

### 10) Decompose selected mega-files into concern-oriented modules

**Why #10:** high LOC concentration increases local complexity and slows reviews.

**Evidence observed**
- large files: `builder.rs`, `types/data.rs`, `script/traverse.rs`, `js_analyze.rs`, `template/expression.rs`.

**Proposal**
- split by concern (e.g., builder literals/calls/patterns; expression lowering vs memoization vs async helpers).
- preserve public API while moving internal pieces.

**Expected benefit**
- faster navigation and more focused code reviews.

**Primary targets**
- `crates/svelte_codegen_client/src/builder.rs`
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/script/traverse.rs`
- `crates/svelte_analyze/src/passes/js_analyze.rs`

---

## Suggested rollout order

1. Architecture safety rails first: #1 + #2.
2. Contract cleanup: #3 + #4 + #8.
3. Duplication reduction: #5 + #6 + #9.
4. Readability scaling: #7 + #10.

## Notes
- Recommendations are intentionally long-term and refactor-friendly.
- No generated snapshots touched.

See also focused follow-up: `DATA_CODEGEN_SIMPLIFY.md`.
