# Tech debt

Every unfixed problem spotted mid-work goes here. New section per item. Describe what is wrong and where.

## `gen_unique_name` allocates 3x per call

`ComponentTransformer::gen_unique_name` (`crates/svelte_transform/src/transformer/state.rs:1239`) builds temp `String` via `format!`, caller proxies again through `builder.alloc_str` into arena `&str`. Plus global `IdentGen` (`svelte_analyze::IdentGen`) lives in parallel solving same task for other sites. One unique ident costs two heap allocs + arena copy instead of one arena alloc. Two independent counters diverge in semantics (per-transformer vs global).

Direction: collapse into a single arena-aware ident generator that writes directly into the bump arena and returns `&'a str`; merge per-transformer counter with `IdentGen` so naming stays consistent across passes.

## Legacy each-item member mutation does not upgrade collection or invalidate inner signals

Surfaced while extending `smoke_legacy_contextual_mutations_all` (ignored). When a legacy `let items = [...]` is iterated via `{#each items as item}` and an item member is mutated through the template (`{item.x = 1}`, `{item.x++}`, etc.), reference does two things our compiler skips:

1. Upgrades the collection declarator from `let items = [...]` to `let items = $.mutable_source([...])`, so the array itself is reactive.
2. Wraps each member-mutation in the template effect with `$.invalidate_inner_signals(() => $.get(items))` to propagate the indirect mutation to dependents reading the collection.

Owning area: `crates/svelte_analyze/src/reactivity_semantics/builder_v2/legacy.rs` for the `let` → mutable_source upgrade decision when each-item member mutation is observed; `crates/svelte_codegen_client/src/codegen/expr.rs::maybe_wrap_legacy_coarse_expr` (or a new helper) for emitting the `invalidate_inner_signals` tail when `legacy_indirect_bindings` carry the collection reference.

## `$state.eager(0)` declarator path panics in oxc_traverse

Surfaced by `smoke_runes_state_eager_panic` (ignored). For `let eager = $state.eager(0)`, our `rewrite_shared_call` replaces the call expression with a freshly-built `$.eager(thunk)` node that has no `oxc::NodeId` set. The oxc traverser then panics on `Option::unwrap()` at `oxc_traverse/walk.rs:2452` when descending into the replacement. Reference's behavior is also unusual (declaration silently dropped, identifier left dangling) so test parity is degenerate, but our panic blocks compile entirely. Owning area: `crates/svelte_transform/src/transformer/rewrites.rs::rewrite_shared_call` `$state.eager` branch — needs to either reuse the existing call's NodeId or bypass traversal of the replacement node.
