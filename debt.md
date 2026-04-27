# Tech debt

Every unfixed problem spotted mid-work goes here. New section per item. Describe what is wrong and where.

## `gen_unique_name` allocates 3x per call

`ComponentTransformer::gen_unique_name` (`crates/svelte_transform/src/transformer/state.rs:1239`) builds temp `String` via `format!`, caller proxies again through `builder.alloc_str` into arena `&str`. Plus global `IdentGen` (`svelte_analyze::IdentGen`) lives in parallel solving same task for other sites. One unique ident costs two heap allocs + arena copy instead of one arena alloc. Two independent counters diverge in semantics (per-transformer vs global).

Direction: collapse into a single arena-aware ident generator that writes directly into the bump arena and returns `&'a str`; merge per-transformer counter with `IdentGen` so naming stays consistent across passes.
