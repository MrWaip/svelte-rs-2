# experimental.async

## Current state

- **Working**: Infrastructure, block wrapping for if/each/html/key/await/svelte:element, directive blockers, `$.template_effect()` blockers, shared async memoization plumbing for render/title/template-effect deps, generic async text/attribute memoization, `{@const}` async with `$.run()` + blocker propagation, `$derived` async basic + destructured, `{@render}` async with blockers + complex async args, `<title>` async with `async_values`, `<svelte:boundary>` async const/snippet scoping, `{await expr}` template syntax, pickled awaits (`$.save()`) in template/attr reactive expressions, dev-mode `$.track_reactivity_loss()` for script/template `await`, `$.async_derived()` label+location args in dev mode, `for await...of` dev wrapping with `$.for_await_track_reactivity_loss`, `$.trace` async function body handling, `svelte-ignore await_waterfall` suppression (omits location arg from `$.async_derived()`)
- **Not working**: —
- **Missing**: `const_tag_invalid_reference` diagnostic in async mode (use case 37)
- **Next**: Use case 37 — `is_template_declaration` tracking in analyze + identifier visitor scope check.
- Last updated: 2026-04-04

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
- `crates/svelte_codegen_client/src/template/const_tag.rs` — async `$.run()` const-tag accumulation + blocker propagation
- `crates/svelte_codegen_client/src/template/svelte_element.rs` — `$.async()` wrapping for dynamic tags
- `crates/svelte_codegen_client/src/template/render_tag.rs` — basic async `$.async()` wrapping for render tags (Memoizer async still partial)
- `crates/svelte_codegen_client/src/template/title_element.rs` — async-aware title memoization via `$.deferred_template_effect()`
- `crates/svelte_codegen_client/src/template/svelte_boundary.rs` — async const/snippet boundary handling

## Use cases

1. [x] `ExpressionInfo.has_await` — detect `await` in expression metadata (covered, tests: async_*)
2. [x] `has_blockers()` / `expression_blockers()` — blocker resolution (covered, tests: *_blockers)
3. [x] `CompileOptions.experimental.async_` option + flag import (covered, test: async_flag_import)
4. [x] Instance body splitting: sync/async segments → `var $$promises = $.run([thunks])` (covered, test: async_blockers_basic)
5. [x] Blocker tracking: `BlockerData.symbol_blockers` mapping (covered)
6. [x] `{#if}` — `$.async()` wrapping with has_await (covered, test: async_if_basic)
7. [x] `{#each}` — `$.async()` wrapping with has_await (covered, test: async_each_basic)
8. [x] `{@html}` — `$.async()` wrapping with has_await (covered, test: async_html_basic)
9. [x] `{#key}` — `$.async()` wrapping with has_await (covered, test: async_key_basic)
10. [x] `{#await}` — async thunk + `$.async()` for blockers (covered, test: async_await_has_await)
11. [x] Block wrapping with non-empty blockers (has_blockers but no has_await) (covered, test: async_blockers_basic)
12. [x] `<svelte:element>` — `$.async()` wrapping for dynamic tag with has_await/has_blockers (covered, test: async_svelte_element)
13. [x] `bind:` — `$.run_after_blockers()` wrapping (covered, test: async_bind_basic)
14. [x] `use:action` — `$.run_after_blockers()` wrapping (covered, test: action_blockers)
15. [x] `{@attach}` — `$.run_after_blockers()` wrapping (covered, test: attach_blockers)
16. [x] `transition:` — `$.run_after_blockers()` wrapping (covered, test: transition_blockers)
17. [x] `animate:` — `$.run_after_blockers()` wrapping (covered, test: animate_blockers)
18. [x] `{@const}` with async expression — `$.run()` accumulation with blockers and `has_await` (test: async_const_tag)
19. [x] `{@const}` blocker propagation — `promises[N]` in downstream template effects (test: async_const_tag)
20. [x] `$derived`/`$derived.by` with `await` → `$.async_derived()` call (covered, test: async_derived_basic)
21. [x] `$derived` async with destructured pattern → `$.async_derived()` + destructure (covered, test: async_derived_destructured)
22. [x] `Memoizer.async_values()` — shared codegen helper tracks async vs sync memoized expressions across render/title/generic template-effect paths
23. [x] `Memoizer.async_ids()` — shared callback param ordering covers render/title/generic template-effect paths
24. [x] `Memoizer.blockers()` — shared blocker collection covers render/title/generic template-effect paths
25. [x] `{@render}` — async wrapping with blockers plus complex-arg `async_values()` coverage (covered, tests: async_render_tag, async_render_tag_complex_args)
26. [x] `<title>` — `$.deferred_template_effect()` with Memoizer async_values/blockers (covered, test: async_title_basic)
27. [x] `<svelte:boundary>` — async-aware const tag + snippet handling (covered, test: async_boundary_const)
28. [x] Snippets not hoisted when `experimental.async && has_const` (covered, test: async_boundary_const)
29. [x] `$.template_effect()` with blockers argument — `emit_template_effect_with_blockers()` (covered)
30. [x] `$.template_effect()` with `async_values` argument (covered for generic memoized text/attr paths)
31. [x] `{await expr}` experimental template syntax — Svelte 5.36+ (covered: parser/analyze/codegen)
32. [x] `(await $.save(expr))()` — context preservation for awaits in reactive expressions (covered for template/attr expressions)
33. N/A `{#await}` dev mode — reference `AwaitBlock.js` does not use `$.apply()`; no action needed
34. [x] `$derived` async — `svelte-ignore await_waterfall` suppression (test: async_derived_dev_ignored; destructured test blocked on Tier 6c `$.tag()`)
35. [x] `$.track_reactivity_loss()` — script + template `await` wrapping, `$.async_derived()` label+location args, `for await...of` wrapping with `$.for_await_track_reactivity_loss` (tests: async_derived_dev, async_for_await_dev)
36. [x] `$.trace` with async function bodies — handled in `inspect.rs:89-103` via `async_thunk_block` + `await` of trace call
37. [ ] `const_tag_invalid_reference` — snippet reads out-of-scope `{@const}` binding in async mode; requires `is_template_declaration` flag on `{@const}` symbols in `AnalysisData` + scope-aware check in the identifier visitor (`Identifier.js:162`)

## Out of scope

- `<slot>` async — not part of Svelte 5 runes model (legacy feature)

## Tasks

### Done: `<svelte:element>` async (#12)
1. [x] codegen: `svelte_element.rs` — add `$.async()` wrapping when `has_await || has_blockers`

### Done: `{@const}` async (#18, #19)
1. [x] codegen: `const_tag.rs` — `let` instead of `const`, `$.run()` accumulation, blocker member expr propagation

### Partial: `$derived` async (#20, #21)
1. [x] codegen: `VariableDeclaration` handling — `$.async_derived()` generation for basic awaited initializers
2. [x] destructured `$derived` async via block rewrite + derived member signals

### Partial: Memoizer async (#22-25, #27, #31)
1. [x] codegen: implement shared Memoizer-like pattern for async/sync separation
2. [x] codegen: `render_tag.rs` — add Memoizer `async_values()` / `blockers()` support for complex args
3. [x] codegen: `title_element.rs` — `$.deferred_template_effect()` with async support
4. [x] codegen: generic `$.template_effect()` async memoization path beyond title

### Done: `<svelte:boundary>` async (#28, #29)
1. [x] codegen: `svelte_boundary` — async const/snippet handling covered by `async_boundary_const`

### Missing: `{await expr}` syntax (#32)
1. [x] parser: parse `{await expr}` template syntax
2. [x] analyze + codegen: handle new syntax

### Done: Dev mode (#36)
1. [x] codegen: `$.track_reactivity_loss()` in script + template `await` (dev mode)
2. [x] codegen: `$.async_derived()` label+location args in dev mode
3. [x] codegen: `for await...of` dev wrapping with `$.for_await_track_reactivity_loss`

### Done: Tracing (#37)
1. [x] codegen: `$.trace` with async body — `async_thunk_block` + `await` in `inspect.rs`

### Done: Dev-mode waterfall warning suppression (#35)
1. [x] diagnostics: `await_waterfall` + `await_reactivity_loss` added to `IGNORABLE_RUNTIME_WARNINGS`
2. [x] codegen: scan JS comments for `// svelte-ignore await_waterfall`, omit location arg from `$.async_derived()` when present
3. [x] codegen: fix destructured async `$derived` predicate to handle dev-transformed `await` form


## Test cases

### Existing (all pass)
- `action_blockers`, `animate_blockers`, `async_await_has_await`, `async_bind_basic`
- `async_blockers_basic`, `async_each_basic`, `async_flag_import`, `async_html_basic`
- `async_if_basic`, `async_key_basic`, `attach_blockers`, `transition_blockers`
- `await_array_destructured`, `await_basic`, `await_destructured`, `await_in_each`
- `await_in_if`, `await_nested_content`, `await_no_bindings`, `await_pending_only`
- `await_reactive`, `await_short_catch`, `await_short_then`, `await_then_catch`

### New (to be added)
- `async_svelte_element` — `<svelte:element>` with await in tag expression ✅
- `async_const_tag` — `{@const}` with async expression and blocker propagation ✅
- `async_boundary_const` — `{@const}` in boundary, const not leaking into snippets ✅
- `async_derived_basic` — `$derived` containing `await` ✅
- `async_derived_destructured` — destructured async `$derived` ✅
- `async_title_basic` — `<title>` with awaited expression ✅
- `async_render_tag` — `{@render}` with async blockers/basic wrapping ✅
- `async_render_tag_complex_args` — `{@render}` with async memoized args ✅
- `inline_await_basic` — basic `{await expr}` template syntax ✅
- `inline_await_text_concat` — `{await expr}` inside text concat ✅
- `inline_await_attr` — `{await expr}` in attribute position ✅
- `async_pickled_await_template` — template pickled await via `$.save()` ✅
- `async_derived_dev` — `$.async_derived()` with dev label+location args ✅
- `async_for_await_dev` — `for await...of` dev wrapping with `$.for_await_track_reactivity_loss` ✅
- `async_derived_dev_ignored` — `svelte-ignore await_waterfall` suppresses location arg ✅
- `async_derived_dev_ignored_destructured` — destructured variant (ignored: blocked on Tier 6c `$.tag()`) ⏸
