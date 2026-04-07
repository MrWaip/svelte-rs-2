# Lower Pass Performance

## Current state
- **Working**: 8/8 use cases
- **Missing**: 0 use cases
- **Current slice**: `lower_fragment` scan merge in `crates/svelte_analyze/src/passes/lower.rs`
- **Why this slice was next**: it was the last remaining implementation change with meaningful payoff after the low-risk micro-optimizations were complete.
- **Done in this slice**: debug-tag and head-title collection now run inside the same fragment walk that already handles recursion and namespace bookkeeping; lowering output and side tables stayed stable under analyzer and compiler regression coverage.
- **Non-goals in this run**: no whitespace-helper rewrite, no benchmark-harness work, no new lowering cache structures.
- **Next**: no required implementation slice remains for this spec; the only follow-up worth considering is benchmark-driven measurement to confirm whether the scan merge materially improves end-to-end performance.
- Last updated: 2026-04-07

## Source

User request: investigate whether `crates/svelte_analyze/src/passes/lower.rs` can be made faster and more efficient, and prepare a dedicated spec for that work.

## Syntax variants

- Root fragment lowering
- Nested element fragment lowering
- Component child fragment lowering
- Named slot partitioning inside component children
- `if` / `{:else if}` / `{:else}` fragment lowering
- `each` body and fallback fragment lowering
- `await` pending / then / catch fragment lowering
- Text plus `ExpressionTag` concatenation and whitespace collapsing
- SVG / MathML / `<foreignObject>` / `<annotation-xml>` namespace-sensitive lowering
- Head-specific lowering for `title` collection

## Use cases

- [x] Replace quadratic blocker deduplication with append + sort + dedup without changing `fragment_blockers` semantics.
- [x] Remove redundant linear search when grouping component children by named slot element `NodeId`.
- [x] Reduce small-fragment allocation overhead in `build_items` by using inline storage where appropriate.
- [x] Avoid the extra pass over `TextConcat.parts` just to compute `has_expr`.
- [x] Avoid repeated invariant loads in hot loops where the value can be hoisted once per fragment or once per function call.
- [x] Merge compatible per-fragment scans in `lower_fragment` so the same `fragment.nodes` slice is not re-walked for unrelated side tables.
- [x] Preserve exact lowered output, including whitespace trimming, slot partitioning, debug/title collection, and recursive fragment coverage.
- [x] Add or update tests so performance-oriented refactors keep behavior stable across HTML, SVG, head, slot, and async-blocker cases.

## Out of scope

- Changing `FragmentItem`, `LoweredTextPart`, or `AnalysisData` public semantics for downstream consumers
- Moving lowering logic out of `svelte_analyze`
- Rewriting unrelated analyze passes for performance
- SSR-specific behavior
- Benchmark harness changes unrelated to validating this pass

## Reference

- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/passes/executor.rs`
- `crates/svelte_analyze/src/types/data/fragments.rs`
- `specs/text-expression-tag.md`
- `specs/element.md`
- `specs/if-block.md`
- `specs/await-block.md`
- `specs/debug-tag.md`
- `specs/const-tag.md`

## Tasks

- Analyze the current pass structure and confirm which repeated scans and allocations are on the hot path.
- Implement low-risk micro-optimizations first:
  - blocker dedup via `sort_unstable` + `dedup`
  - direct named-slot grouping without redundant search
  - inline storage for small temporary vectors
  - `has_expr` tracking during concat building
  - hoisting obviously invariant values out of inner loops
- Verify behavior with targeted tests in `lower.rs` and existing compiler/analyze coverage.
- Only after low-risk work is validated, collapse compatible fragment-local scans inside `lower_fragment` while keeping explicit data flow into `AnalysisData`.
- Re-run targeted tests and compare compiler output to confirm no JS behavior changes.
- If the structural merge produces marginal benefit or noticeably hurts readability, stop after the low-risk package and record that decision here.

## Implementation order

1. Micro-optimizations with no behavioral or data-flow changes.
2. Targeted test updates for coverage gaps exposed by refactor.
3. Structural per-fragment scan merge if still justified.
4. Verification and summary update in `Current state`.

## Discovered bugs

- None yet.

## Test cases

- Existing unit tests in `crates/svelte_analyze/src/passes/lower.rs`
- Existing compiler coverage from:
  - `text-expression-tag`
  - `element`
  - `if-block`
  - `await-block`
  - `debug-tag`
  - `const-tag`
- Add focused tests if needed for:
  - async blocker dedup stability
  - component named-slot grouping
  - head `title` collection after scan merging
  - SVG whitespace removal after temporary-vector changes
- Added in the redundant-work slice:
  - analyzer unit test for sorted/unique fragment blocker indices
  - targeted verification with `component_named_slot`, `async_blockers_basic`, and `if_elseif_new_blockers`
  - `svelte_fragment_named_slot` remains ignored as an existing known parity gap
- Added in the allocation slice:
  - `lower.rs` unit test confirming unchanged text still lowers to `TextSpan` after source/source-pointer hoisting
- Added in the scan-merge slice:
  - analyzer unit test for `debug_tags.by_fragment` on root fragments
  - analyzer unit test for `title_elements.by_fragment` on `<svelte:head>` fragments
  - analyzer unit test for `html_tag_in_svg` / `html_tag_in_mathml` namespace flags
