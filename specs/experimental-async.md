# experimental.async

## Source

ROADMAP Tier 1.1 — all features gated behind `experimental.async` compiler option.

## Reference files

### Infrastructure
- `reference/compiler/phases/nodes.js` — `ExpressionMetadata`: `has_await`, `blockers()`, `is_async()`
- `reference/compiler/phases/2-analyze/index.js` — blocker assignment to bindings
- `reference/compiler/phases/3-transform/client/visitors/shared/utils.js` — `Memoizer.check_blockers()`

### Instance body splitting
- `reference/compiler/phases/3-transform/client/visitors/javascript.js` — sync/async segment splitting, `$.run()` generation

### Block wrapping (`$.async()`)
- `reference/compiler/phases/3-transform/client/visitors/IfBlock.js`
- `reference/compiler/phases/3-transform/client/visitors/EachBlock.js`
- `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`
- `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js`
- `reference/compiler/phases/3-transform/client/visitors/AwaitBlock.js`

### Directive blocker wrapping (`$.run_after_blockers()`)
- `reference/compiler/phases/3-transform/client/visitors/BindDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/UseDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/AttachTag.js`
- `reference/compiler/phases/3-transform/client/visitors/TransitionDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/AnimateDirective.js`

## Implemented

### Infrastructure
- [x] `ExpressionInfo.has_await` — detect `await` in expression metadata
- [x] `has_blockers()` / `expression_blockers()` — blocker resolution from `SymbolId → BlockerIndex`
- [x] `attr_expression_blockers()` — same for attribute/directive expressions
- [x] `CompileOptions.experimental.async_` option + `import "svelte/internal/flags/async"` flag import
- [x] Instance body splitting: sync/async segments → `var $$promises = $.run([thunks])`
- [x] Blocker tracking: `BlockerData.symbol_blockers` mapping

### Block wrapping
- [x] `$.async()` wrapping for if/each/html/key blocks with `has_await` expressions
- [x] `{#await}` — async thunk for expression with `has_await`
- [x] Block wrapping with non-empty blockers (has_blockers but no has_await)

### Directive blocker wrapping
- [x] `bind:` — `$.run_after_blockers()` via `bind_semantics.bind_blockers`
- [x] `use:action` — `$.run_after_blockers()` via `attr_expression_blockers()`
- [x] `{@attach}` — `$.run_after_blockers()` via `attr_expression_blockers()`
- [x] `transition:` — `$.run_after_blockers()` via `attr_expression_blockers()`
- [x] `animate:` — `$.run_after_blockers()` via `attr_expression_blockers()`

## Not yet implemented

- [ ] Full blocker tracking: const tags with async expressions → `binding.blocker` propagation
- [ ] Function blocker analysis: deferred max-blocker tracking for function declarations
- [ ] `{await expr}` experimental template syntax (Svelte 5.36+)
- [ ] `<svelte:boundary>` — `experimental.async` handling for const tag scoping changes
- [ ] `{#await}` — dev-mode `$.apply()` wrapping for await expression

## Known gaps

- Directive **name** expression blockers (e.g., dynamically-imported transition function) — not tracked in `attr_expressions`, only value expressions are. Rare in practice.
- `|| $.noop` fallback for non-function dynamic `{@attach}` on components — reference uses `scope.evaluate().is_function` which we don't replicate. Current behavior: always wrap as `($$node) => expr($$node)` without null guard. Matches reference output for identifier/derived cases.

## Test cases

- `async_bind_basic` — bind + blocker
- `action_blockers` — `use:action` + blocker
- `attach_blockers` — `{@attach}` + blocker
- `transition_blockers` — `transition:` + blocker
- `animate_blockers` — `animate:` + blocker
- `async_flag_import`, `async_html_basic`, `async_each_basic`, `async_key_basic` — block wrapping tests
