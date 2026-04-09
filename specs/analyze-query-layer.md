# Analyze Query Layer

## Current state
- **Status**: the refactoring tracked by this spec is complete. There is no remaining in-scope query-layer migration debt hidden behind `out of scope`.
- **Final completed slice**: slice 12 added concrete sibling-oriented element queries to `TemplateElementIndex`, and `css_prune` now uses them for `+` and `~` instead of conservative unconditional matches.
- **What this means**:
  - late-pass analyzer consumers that previously open-coded repeated query logic now read sanctioned structures in `AnalysisData`
  - `CodegenView` no longer leaks low-level analyzer storage for the migrated query families
  - `ElementFlags` no longer duplicates shared normalized element facts
  - CSS analyze/prune no longer maintain a parallel template element model for the migrated selector/query cases
- **No remaining debt inside this spec**:
  - producer-side `VisitContext` usage in `template_side_tables` and `collect_symbols` remains intentional because those passes are building analysis data rather than consuming reusable query APIs
  - any broader selector-completeness work beyond concrete recorded element siblings is a separate feature/spec, not unfinished query-layer migration from this document
  - broader future API polish, if desired, should be tracked as a new simplification spec instead of being treated as an unfinished item here
- **What changed in slice 12**:
  - `TemplateElementIndex` now records direct previous/next element siblings and exposes previous-sibling traversal queries
  - analyzer regressions now cover sibling indexing and CSS prune `+` / `~` matching
  - `css_prune` no longer treats sibling combinators as unconditional matches for concrete recorded template elements
- **What changed in slice 10**:
  - `FragmentFacts` now records raw direct-child count plus non-trivial direct-child count alongside the existing single-child/single-non-trivial-child shape queries
  - `AnalysisData` now exposes those counts through sanctioned fragment accessors
  - `template_validation::component_has_implicit_children` now uses `FragmentFacts` for `snippet_conflict` instead of open-coding a direct `component.fragment.nodes` scan
  - analyzer regressions now cover non-trivial direct-child counting and the “children snippet with no other implicit content” case
- **Previously completed in slice 9**:
  - `element_flags` now resolves attribute owner elements through `AnalysisData::nearest_element(...)` instead of `VisitContext::nearest_element()`
  - `hoistable` now resolves expression-site parent/ancestor state through `AnalysisData::expr_parent(...)` / `expr_ancestors(...)` instead of `VisitContext::parent()` / `ancestors()`
  - `template_side_tables` and `collect_symbols` remain on walker-stack topology intentionally because they still sit on the data-production side of the pipeline
  - existing analyzer and compiler coverage for topology-backed attribute ownership and snippet hoistability passes unchanged
- **Previously completed in slice 8**:
  - removed duplicated `ElementFlags` storage for per-element `AttrIndex` and spread presence
  - `template_side_tables` now records normalized attribute facts only through `ElementFacts`
  - `element_flags.rs` now reads the remaining `value`-attribute query through `AnalysisData::attr_index(...)` instead of pass-local duplicate storage
  - `ElementFlags` now documents that shared normalized attribute facts belong in `ElementFacts`, leaving `ElementFlags` limited to downstream derived flags
  - existing analyzer and compiler coverage for `ElementFacts`, `textarea`, and `option` behavior passes unchanged
- **Previously completed in slice 7**:
  - added analyzer-backed codegen queries for static string attribute lookup and CSS-facing `css_hash` / scoped-element / inject-styles access
  - removed `CodegenView` exposure of raw `AttrIndex` and raw `IgnoreData`
  - `svelte:element` codegen now queries the static `xmlns` attribute through the sanctioned facade instead of pulling `AttrIndex`
  - script codegen/dev transforms now carry a small `IgnoreQuery` wrapper backed by `CodegenView` methods instead of threading `IgnoreData`
  - existing compiler coverage for `svelte_element_xmlns`, injected CSS, and ignored async-derived diagnostics passes unchanged
- **Previously completed in slice 6**:
  - added `EachContextIndex` as the sanctioned each-block ownership query/index structure in `AnalysisData`
  - moved each-block context/index metadata, key/body index-usage tracking, animate flags, and bind/every-context ownership queries into the shared index
  - `bind_semantics` now records matching ancestor each ownership and `bind:this` each-context references through `EachContextIndex` instead of local ancestor/scope glue storage
  - `collect_symbols` now resolves index-symbol ownership through `EachContextIndex` instead of rebuilding pass-local lookup state from low-level storage
  - analyzer regressions now cover `EachContextIndex` index-symbol lookup and bind:this each-context capture
- **Previously completed in slice 5**:
  - added `EachContextIndex` as the sanctioned each-block ownership query/index structure in `AnalysisData`
  - moved each-block context/index metadata, key/body index-usage tracking, animate flags, and bind/every-context ownership queries into `EachContextIndex`
  - `bind_semantics` now records matching ancestor each ownership and `bind:this` each-context references through `EachContextIndex` instead of local ancestor/scope glue storage
  - `collect_symbols` now resolves index-symbol ownership through `EachContextIndex` instead of rebuilding pass-local lookup state from low-level storage
  - analyzer regressions now cover `EachContextIndex` index-symbol lookup and bind:this each-context capture
- **What changed in this slice**:
  - added `TemplateElementIndex` as the sanctioned CSS-oriented query/index structure in `AnalysisData`
  - `template_side_tables` now records template element tag/class/id metadata and nearest parent-element ownership into the shared index
  - `css_analyze` now marks scoped elements through `TemplateElementIndex` tag lookup instead of walking template fragments directly
  - `css_prune` now reads shared element inventory and class/id parent queries through `TemplateElementIndex` instead of building a pass-local `TemplateElement` model
  - analyzer regressions now cover `TemplateElementIndex` CSS candidate semantics and parent-element preservation across block boundaries
- **Previously completed**:
  - slice 4 introduced `FragmentFacts` for direct-child shape queries
  - `ElementFacts` now captures per-element normalized attribute facts (`AttrIndex`, spread presence, runtime-attribute presence)
  - `TemplateTopology` now captures reusable parent and nearest-element relationships for nodes, attrs, and expression sites
  - `reactivity` no longer depends on walker-stack topology for its `parent` / `ancestor` / `nearest_element` queries
  - `content_types` no longer rescans raw attrs to decide whether an element has runtime attrs
  - `template_validation` now reads common attribute and topology queries through `AnalysisData` accessors backed by `ElementFacts` and `TemplateTopology`
- **Remaining problem**: none. The original scoped query-layer migration is complete.
- **Sanctioned direction**: introduce a small fixed query/index layer in `AnalysisData`. Raw `FxHashMap` / `FxHashSet` remain allowed only as private implementation details inside those index structs. Pass consumers should read through typed accessors instead of building local indexes.
- **Attribute-query rule**: `AttrIndex` is an internal primitive of `ElementFacts`, not a consumer-facing pass API. Analyzer consumer passes should read attribute facts only through `AnalysisData` / `CodegenView` accessors such as presence checks, typed attribute lookup, static text lookup, and boolean-like attribute helpers.
- **Current sanctioned infrastructure set**:
  - `AttrIndex` as the low-level by-name primitive
  - `ElementFacts` for normalized per-node attribute facts
  - `TemplateTopology` for parent / ancestor / sibling / nearest-element queries
  - `FragmentFacts` for direct-child shape queries
  - `TemplateElementIndex` for CSS-oriented element candidate lookup
  - `EachContextIndex` for each-block ownership and bind/each context queries
- **Current slice**: completed sibling-oriented `TemplateElementIndex` queries and removed conservative `css_prune` matching for `+` / `~`.
- **Why slice 1 came first**: it was the smallest cohesive proof of both sanctioned structures. `reactivity` exercised topology (`parent`, `ancestor`, `nearest element`) and `content_types` exercised normalized runtime-attribute facts without widening into the larger diagnostic surface of `template_validation`.
- **Why the previous slice came next**: after the analyzer-owned query structures landed, the public codegen facade was the main remaining boundary leak for consumers outside analyze.
- **Next**:
  - treat this spec as done
  - if broader CSS selector completeness needs more than concrete recorded element siblings, create a separate bounded spec instead of reopening this one
- **Non-goals for any follow-up from this spec**: no broad query-layer redesign, no speculative “mega index”, and no rewriting producer-side traversal that still legitimately owns data construction
- **Implementation rule**: Changes must be systematic, without workarounds or temporary solutions, respecting crate and module boundaries.
- Last updated: 2026-04-06

## Source

User request: pause feature work and research a comprehensive `svelte_analyze` query/index refactor so new `$port2` slices stop creating local infrastructure and pass-local workarounds.

## Syntax variants

- Template AST queries over `Element`, `ComponentNode`, `SvelteElement`, `SvelteWindow`, `SvelteDocument`, `SvelteBody`, `IfBlock`, `EachBlock`, `SnippetBlock`, `KeyBlock`, `AwaitBlock`
- Attribute queries over `StringAttribute`, `BooleanAttribute`, `ExpressionAttribute`, `ConcatenationAttribute`, `Shorthand`, `SpreadAttribute`, directives
- Fragment child-shape queries over direct `fragment.nodes`
- CSS selector matching queries over tag / class / id / parent / sibling relationships
- Each-block ownership queries over context/index symbols and ancestor each-blocks

## Use cases

- [x] The original query-layer migration is complete for all confirmed analyzer and codegen consumers that motivated this spec
- [x] `ElementFacts` centralizes normalized per-node attribute facts so passes stop re-scanning `attributes` for presence, static values, runtime-ness, class/id matching, and spread handling
- [x] `TemplateTopology` centralizes parent / ancestor / sibling / nearest-element queries so passes stop depending on walker-stack-only topology access
- [x] `FragmentFacts` centralizes repeated direct-child shape queries for current analyze consumers (`has_expression_child`, `single_expression_child`, `has_direct_animate_child`, single-child / single-non-trivial-child, direct-child counts, and the current late-pass consumers are all covered)
- [x] `TemplateElementIndex` centralizes CSS-oriented element candidate lookup by tag / static class / static id plus element parent ownership for current analyze consumers
- [x] `EachContextIndex` exposes stable each-block ownership queries instead of repeated ancestor + scope glue in consumers
- [x] `template_validation` consumes query APIs instead of raw attr scans and walker-stack glue for common structural checks
- [x] `reactivity` consumes query APIs instead of raw `ctx.parent()` / `ctx.ancestors()` / `ctx.nearest_element()` combinations for common topology checks
- [x] `bind_semantics` consumes query APIs instead of local attr lookups and repeated ancestor each-block scans
- [x] `content_types` consumes query APIs instead of raw attribute scans for runtime-attribute checks
- [x] CSS analyze/prune consume shared query/index APIs instead of building a parallel template element model inside `css_analyze.rs` / `css_prune.rs`
- [x] `CodegenView` exposes query-layer accessors instead of leaking lower-level storage structure details
- [x] New analyze work follows a rule: if a repeated query is needed and fits an existing sanctioned structure, extend that structure's API instead of creating a local index
- [x] `TemplateElementIndex` exposes sibling-oriented queries when selector-completeness work needs them
- [x] analyzer consumers stop open-coding recursive fragment “rich content” classification when a sanctioned structure for that behavior exists

## Out of scope

- Replacing `ComponentScoping` or `svelte_component_semantics`
- Full rewrite of fragment lowering or `FragmentData`
- Full CSS selector engine completeness work beyond the concrete recorded-element queries implemented here
- Eliminating every internal hash collection in analyze; the goal is to hide them behind sanctioned index APIs, not to ban them as implementation details
- Broad codegen API redesign unrelated to query access

## Reference

- `crates/svelte_analyze/src/types/data/attr_index.rs` — existing low-level attribute name index
- `crates/svelte_analyze/src/types/data/elements.rs` — current mixture of low-level facts and pass-specific derived flags
- `crates/svelte_analyze/src/walker/context.rs` — current walker-stack-only parent / ancestor / nearest-element access
- `crates/svelte_analyze/src/passes/template_side_tables.rs` — current early side-table construction site; likely owner for new query infrastructure build
- `crates/svelte_analyze/src/passes/element_flags.rs` — repeated attr/child-shape extraction and existing per-element data
- `crates/svelte_analyze/src/passes/template_validation.rs` — heavy consumer of attr/topology queries
- `crates/svelte_analyze/src/passes/reactivity.rs` — heavy consumer of parent / ancestor / nearest-element queries
- `crates/svelte_analyze/src/passes/bind_semantics.rs` — repeated attr lookup plus each-ancestor ownership glue
- `crates/svelte_analyze/src/passes/content_types.rs` — repeated runtime-attribute checks
- `crates/svelte_analyze/src/passes/css_analyze.rs` — CSS pass building its own template queries
- `crates/svelte_analyze/src/passes/css_prune.rs` — CSS pass building a local template element index and doing repeated attr scans
- `crates/svelte_analyze/src/types/data/template_data.rs` — existing `EachBlockData` and related template-side tables
- `crates/svelte_analyze/src/types/data/codegen_view.rs` — current public read facade over `AnalysisData`
- `CLAUDE.md` — analyze owns derived facts and accessors; repeated traversal and duplicated side tables are boundary smells

## Test cases

- [x] New unit tests for `ElementFacts` query semantics
- [x] New unit tests for `TemplateTopology` parent / sibling / nearest-element queries
- [x] New unit tests for `FragmentFacts` child-shape queries
- [x] New unit tests for `TemplateElementIndex` CSS candidate lookup
- [x] New unit tests for `EachContextIndex` each ownership queries
- [x] Targeted analyzer regression tests proving migrated consumers preserve existing behavior
- [x] Additional analyzer regressions for nested slot placement, topology-backed invalid text placement, and slotted `{@const}` placement
- [x] Additional analyzer regressions for bind-group parent each discovery, bind-group value attr capture, and bound-contenteditable detection
