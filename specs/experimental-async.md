# experimental.async

## Current state
- **Working**: 36/36 use cases
- **Tests**: 43/45 green
- Last updated: 2026-04-07

## Source

Audit of existing implementation (2026-03-28)

## Use cases

- [x] `ExpressionInfo.has_await` — detect `await` in expression metadata (test: async_*)
- [x] `has_blockers()` / `expression_blockers()` — blocker resolution (tests: *_blockers)
- [x] `CompileOptions.experimental.async_` option + flag import (test: async_flag_import)
- [x] Instance body splitting: sync/async segments → `var $$promises = $.run([thunks])` (test: async_blockers_basic)
- [x] Blocker tracking: `BlockerData.symbol_blockers` mapping
- [x] `{#if}` — `$.async()` wrapping with has_await (test: async_if_basic)
- [x] `{#each}` — `$.async()` wrapping with has_await (test: async_each_basic)
- [x] `{@html}` — `$.async()` wrapping with has_await (test: async_html_basic)
- [x] `{#key}` — `$.async()` wrapping with has_await (test: async_key_basic)
- [x] `{#await}` — async thunk + `$.async()` for blockers (test: async_await_has_await)
- [x] Block wrapping with non-empty blockers (has_blockers but no has_await) (test: async_blockers_basic)
- [x] `<svelte:element>` — `$.async()` wrapping for dynamic tag with has_await/has_blockers (test: async_svelte_element)
- [x] `bind:` — `$.run_after_blockers()` wrapping (test: async_bind_basic)
- [x] `use:action` — `$.run_after_blockers()` wrapping (test: action_blockers)
- [x] `{@attach}` — `$.run_after_blockers()` wrapping (test: attach_blockers)
- [x] `transition:` — `$.run_after_blockers()` wrapping (test: transition_blockers)
- [x] `animate:` — `$.run_after_blockers()` wrapping (test: animate_blockers)
- [x] `{@const}` with async expression — `$.run()` accumulation with blockers and `has_await` (test: async_const_tag)
- [x] `{@const}` blocker propagation — `promises[N]` in downstream template effects (test: async_const_tag)
- [x] `$derived`/`$derived.by` with `await` → `$.async_derived()` call (test: async_derived_basic)
- [x] `$derived` async with destructured pattern → `$.async_derived()` + destructure (test: async_derived_destructured)
- [x] `Memoizer.async_values()` — shared codegen helper tracks async vs sync memoized expressions across render/title/generic template-effect paths
- [x] `Memoizer.async_ids()` — shared callback param ordering covers render/title/generic template-effect paths
- [x] `Memoizer.blockers()` — shared blocker collection covers render/title/generic template-effect paths
- [x] `{@render}` — async wrapping with blockers plus complex-arg `async_values()` coverage (tests: async_render_tag, async_render_tag_complex_args)
- [x] `<title>` — `$.deferred_template_effect()` with Memoizer async_values/blockers (test: async_title_basic)
- [x] `<svelte:boundary>` — async-aware const tag + snippet handling (test: async_boundary_const)
- [x] Snippets not hoisted when `experimental.async && has_const` (test: async_boundary_const)
- [x] `$.template_effect()` with blockers argument — `emit_template_effect_with_blockers()`
- [x] `$.template_effect()` with `async_values` argument (covered for generic memoized text/attr paths)
- [x] `{await expr}` experimental template syntax — Svelte 5.36+ (covered: parser/analyze/codegen)
- [x] `(await $.save(expr))()` — context preservation for awaits in reactive expressions (covered for template/attr expressions)
- N/A `{#await}` dev mode — reference `AwaitBlock.js` does not use `$.apply()`; no action needed
- [x] `$derived` async — `svelte-ignore await_waterfall` suppression (test: async_derived_dev_ignored; destructured test blocked on Tier 6c `$.tag()`)
- [x] `$.track_reactivity_loss()` — script + template `await` wrapping, `$.async_derived()` label+location args, `for await...of` wrapping with `$.for_await_track_reactivity_loss` (tests: async_derived_dev, async_for_await_dev)
- [x] `$.trace` with async function bodies — handled in `inspect.rs:89-103` via `async_thunk_block` + `await` of trace call
- [x] `const_tag_invalid_reference` — snippet reads out-of-scope `{@const}` binding in async mode; implemented via per-symbol template-declaration marking plus scope-aware identifier validation in analyze

## Out of scope

- Legacy Svelte 4 slot async behavior is owned by `specs/legacy-slots.md`, not this runes-mode spec

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

## Test cases

- [x] action_blockers
- [x] animate_blockers
- [x] async_await_has_await
- [x] async_bind_basic
- [x] async_blockers_basic
- [x] async_boundary_const
- [x] async_const_tag
- [x] async_derived_basic
- [x] async_derived_destructured
- [x] async_derived_dev
- [x] async_derived_dev_ignored
- [x] async_each_basic
- [x] async_flag_import
- [x] async_for_await_dev
- [x] async_html_basic
- [x] async_if_basic
- [x] async_key_basic
- [x] async_pickled_await_template
- [x] async_render_tag
- [x] async_render_tag_complex_args
- [x] async_svelte_element
- [x] async_title_basic
- [x] attach_blockers
- [ ] `validate_const_tag_invalid_reference_component_children_async`
- [x] `validate_const_tag_invalid_reference_boundary_failed_async`
- [x] `validate_const_tag_invalid_reference_boundary_pending_async`
- [x] `validate_const_tag_invalid_reference_skipped_without_async`
- [x] `validate_const_tag_reference_inside_snippet_scope_is_allowed_async`
- [x] await_array_destructured
- [x] await_basic
- [x] await_destructured
- [x] await_in_each
- [x] await_in_if
- [x] await_nested_content
- [x] await_no_bindings
- [x] await_pending_only
- [x] await_reactive
- [x] await_short_catch
- [x] await_short_then
- [x] await_then_catch
- [x] inline_await_attr
- [x] inline_await_basic
- [x] inline_await_text_concat
- [x] transition_blockers
- [ ] async_derived_dev_ignored_destructured — destructured variant (ignored: blocked on Tier 6c `$.tag()`)
