# experimental.async

## Current state

- **Working (18/34 use cases)**: Infrastructure (5), block wrapping for if/each/html/key/await (6), directive blockers (5), template_effect blockers (1), block wrapping with blockers-only (1)
- **Not working (16/34)**: `<svelte:element>` async, `{@const}` async, `$derived` async, Memoizer async, `{@render}`/`<title>` async, `<svelte:boundary>` async, `{await expr}` syntax, pickled awaits (`$.save()`), dev mode, tracing
- **Out of scope**: SSR (`$.await()` server-side — will be separate phase)
- **Next**: `<svelte:element>` async (#12) — simplest, follows exact same pattern as if/each/html/key
- Last updated: 2026-03-29

## Source

Audit of existing implementation (2026-03-28)

## Reference

### Svelte (reference compiler)
- `reference/compiler/phases/nodes.js` — `ExpressionMetadata`: `has_await`, `blockers()`, `is_async()`
- `reference/compiler/phases/2-analyze/index.js` — blocker assignment, `async_deriveds` set, `pickled_awaits`
- `reference/compiler/phases/2-analyze/visitors/CallExpression.js` — `$derived` async detection
- `reference/compiler/phases/3-transform/client/visitors/shared/utils.js` — `Memoizer` (async/sync tracking, `blockers()`, `async_values()`, `async_ids()`)
- `reference/compiler/phases/3-transform/client/visitors/javascript.js` — sync/async segment splitting, `$.run()` generation
- `reference/compiler/phases/3-transform/client/visitors/IfBlock.js`
- `reference/compiler/phases/3-transform/client/visitors/EachBlock.js`
- `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`
- `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js`
- `reference/compiler/phases/3-transform/client/visitors/AwaitBlock.js`
- `reference/compiler/phases/3-transform/client/visitors/SvelteElement.js` — `$.async()` wrapping for dynamic tag
- `reference/compiler/phases/3-transform/client/visitors/ConstTag.js` — `$.run()` async const accumulation, blocker propagation
- `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js` — `$.async_derived()` for async `$derived`
- `reference/compiler/phases/3-transform/client/visitors/RenderTag.js` — `Memoizer` blockers/async for render tags
- `reference/compiler/phases/3-transform/client/visitors/SlotElement.js` — `Memoizer` blockers/async for slots
- `reference/compiler/phases/3-transform/client/visitors/TitleElement.js` — `Memoizer` blockers/async for title
- `reference/compiler/phases/3-transform/client/visitors/SvelteBoundary.js` — async-aware const tag + snippet handling
- `reference/compiler/phases/3-transform/client/visitors/BindDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/UseDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/AttachTag.js`
- `reference/compiler/phases/3-transform/client/visitors/TransitionDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/AnimateDirective.js`
- `reference/compiler/phases/3-transform/client/visitors/BlockStatement.js` — async tracing (`$.trace` with await)

### Our code
- `crates/svelte_codegen_client/src/template/if_block.rs` — `$.async()` wrapping
- `crates/svelte_codegen_client/src/template/each_block.rs` — `$.async()` wrapping
- `crates/svelte_codegen_client/src/template/html_tag.rs` — `$.async()` wrapping
- `crates/svelte_codegen_client/src/template/key_block.rs` — `$.async()` wrapping
- `crates/svelte_codegen_client/src/template/await_block.rs` — async thunk + `$.async()` for blockers
- `crates/svelte_codegen_client/src/template/expression.rs` — `emit_template_effect_with_blockers()`
- `crates/svelte_codegen_client/src/template/const_tag.rs` — no async handling
- `crates/svelte_codegen_client/src/template/svelte_element.rs` — no async handling
- `crates/svelte_codegen_client/src/template/render_tag.rs` — no async handling
- `crates/svelte_codegen_client/src/template/title_element.rs` — no async handling

## Use cases

### Infrastructure
1. [x] `ExpressionInfo.has_await` — detect `await` in expression metadata (covered, tests: async_*)
2. [x] `has_blockers()` / `expression_blockers()` — blocker resolution (covered, tests: *_blockers)
3. [x] `CompileOptions.experimental.async_` option + flag import (covered, test: async_flag_import)
4. [x] Instance body splitting: sync/async segments → `var $$promises = $.run([thunks])` (covered, test: async_blockers_basic)
5. [x] Blocker tracking: `BlockerData.symbol_blockers` mapping (covered)

### Block wrapping (`$.async()`)
6. [x] `{#if}` — `$.async()` wrapping with has_await (covered, test: async_if_basic)
7. [x] `{#each}` — `$.async()` wrapping with has_await (covered, test: async_each_basic)
8. [x] `{@html}` — `$.async()` wrapping with has_await (covered, test: async_html_basic)
9. [x] `{#key}` — `$.async()` wrapping with has_await (covered, test: async_key_basic)
10. [x] `{#await}` — async thunk + `$.async()` for blockers (covered, test: async_await_has_await)
11. [x] Block wrapping with non-empty blockers (has_blockers but no has_await) (covered, test: async_blockers_basic)
12. [ ] `<svelte:element>` — `$.async()` wrapping for dynamic tag with has_await/has_blockers (missing)

### Directive blocker wrapping (`$.run_after_blockers()`)
13. [x] `bind:` — (covered, test: async_bind_basic)
14. [x] `use:action` — (covered, test: action_blockers)
15. [x] `{@attach}` — (covered, test: attach_blockers)
16. [x] `transition:` — (covered, test: transition_blockers)
17. [x] `animate:` — (covered, test: animate_blockers)

### `{@const}` async handling
18. [ ] `{@const}` with async expression — `$.run()` accumulation with blockers and `has_await` (missing)
19. [ ] `{@const}` blocker propagation — `binding.blocker = member(run.id, ...)` (missing)

### `$derived` async
20. [ ] `$derived`/`$derived.by` with `await` → `$.async_derived()` call (missing — no `async_deriveds` tracking)
21. [ ] `$derived` async with destructured pattern → `$.async_derived()` + destructure (missing)

### Memoizer async support
22. [ ] `Memoizer.async_values()` — separate tracking of async vs sync memoized expressions (missing — no Memoizer concept)
23. [ ] `Memoizer.async_ids()` — parameter names for async-resolved values (missing)
24. [ ] `Memoizer.blockers()` — blocker collection from expression dependencies (missing in Memoizer context)

### `{@render}` / `<slot>` async
25. [ ] `{@render}` — `$.async()` wrapping with Memoizer blockers/async_values (missing)
26. [ ] `<slot>` — `$.async()` wrapping with Memoizer blockers/async_values (missing — `<slot>` codegen not implemented)

### `<title>` async
27. [ ] `<title>` — `$.deferred_template_effect()` with Memoizer async_values/blockers (missing)

### `<svelte:boundary>` async
28. [ ] `<svelte:boundary>` — async-aware const tag + snippet handling (missing)
29. [ ] Snippets not hoisted when `experimental.async && has_const` (missing)

### `$.template_effect` async
30. [x] `$.template_effect()` with blockers argument — `emit_template_effect_with_blockers()` (covered)
31. [ ] `$.template_effect()` with `async_values` argument (partial — blockers work but separate async_values memoization missing)

### `{await expr}` template syntax
32. [ ] `{await expr}` experimental template syntax — Svelte 5.36+ (missing — new syntax not parsed)

### Pickled awaits (`$.save()`)
33. [ ] `(await $.save(expr))()` — context preservation for awaits in reactive expressions (missing — no `pickled_awaits` tracking)

### Dev mode
34. [ ] `{#await}` — dev-mode `$.apply()` wrapping for await expression (missing)
35. [ ] `$derived` async — `await_waterfall` warning with location (missing)
36. [ ] `$.track_reactivity_loss()` — dev-mode warning for reactivity loss in await (missing)

### Tracing
37. [ ] `$.trace` with async function bodies — `b.thunk(body, is_async)` + `b.await(call)` (missing)

## Tasks

### Missing: `<svelte:element>` async (#12)
1. [ ] codegen: `svelte_element.rs` — add `$.async()` wrapping when `has_await || has_blockers`

### Missing: `{@const}` async (#18, #19)
1. [ ] analyze: track `async_consts` state per fragment — accumulate `$.run()` thunks
2. [ ] codegen: `const_tag.rs` — `let` instead of `const`, `$.run()` accumulation, blocker member expr propagation

### Missing: `$derived` async (#20, #21)
1. [ ] analyze: `async_deriveds` set — track which `$derived` calls contain `await`
2. [ ] codegen: `VariableDeclaration` handling — `$.async_derived()` generation

### Missing: Memoizer async (#22-24, #25-27)
1. [ ] codegen: implement Memoizer-like pattern for async/sync separation
2. [ ] codegen: `render_tag.rs` — `$.async()` with Memoizer blockers/async_values
3. [ ] codegen: `title_element.rs` — `$.deferred_template_effect()` with async support

### Missing: `<svelte:boundary>` async (#28, #29)
1. [ ] codegen: `svelte_boundary` — don't hoist const tags/snippets in async mode

### Missing: `{await expr}` syntax (#32)
1. [ ] parser: parse `{await expr}` template syntax
2. [ ] analyze + codegen: handle new syntax

### Missing: Dev mode (#33, #34)
1. [ ] codegen: `$.apply()` wrapping in dev mode for `{#await}`
2. [ ] codegen: `await_waterfall` warning for async `$derived`

### Missing: Tracing (#35)
1. [ ] codegen: `$.trace` with async body handling


## Test cases

### Existing (all pass)
- `action_blockers`, `animate_blockers`, `async_await_has_await`, `async_bind_basic`
- `async_blockers_basic`, `async_each_basic`, `async_flag_import`, `async_html_basic`
- `async_if_basic`, `async_key_basic`, `attach_blockers`, `transition_blockers`
- `await_array_destructured`, `await_basic`, `await_destructured`, `await_in_each`
- `await_in_if`, `await_nested_content`, `await_no_bindings`, `await_pending_only`
- `await_reactive`, `await_short_catch`, `await_short_then`, `await_then_catch`

### New (to be added)
- `async_svelte_element` — `<svelte:element>` with await in tag expression
- `async_const_tag` — `{@const}` with async expression and blocker propagation
- `async_derived_basic` — `$derived` containing `await`
- `async_render_tag` — `{@render}` with async memoized args
- `async_boundary_const` — `<svelte:boundary>` with const tags in async mode
