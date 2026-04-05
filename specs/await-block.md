# Await Block

## Current state

- **Working**: 24/25 use cases. One partial (`block_unexpected_character`).
- `block_unexpected_character` for `{ :then}` / `{ :catch}` whitespace is structurally implemented in the analyzer but cannot be triggered: the scanner dispatches immediately on the char after `{` and does not skip whitespace before `:`. The check will fire when the scanner is made permissive. Tracked as `[~]` below.
- **Next**: Fix scanner gap — make the scanner permissive to whitespace before `:then`/`:catch` so the existing `block_unexpected_character` analyzer check can be triggered.
- Last updated: 2026-04-03

## Source

Audit of existing implementation (2026-04-01)

## Syntax variants

```svelte
{#await expr}...{:then val}...{:catch err}...{/await}
{#await expr}...{/await}
{#await expr then val}...{/await}
{#await expr catch err}...{/await}
{#await expr then val}...{:catch err}...{/await}
{#await expr}...{:then}...{:catch}...{/await}
{#await expr}...{:then val}...{/await}
{#await expr}...{:catch err}...{/await}
{#await expr catch}...{/await}
```

## Use cases

- [x] Full form: `{#await expr}...{:then val}...{:catch err}...{/await}` (test: await_basic)
- [x] Pending only: `{#await expr}...{/await}` (test: await_pending_only)
- [x] Shorthand then: `{#await expr then val}...{/await}` (test: await_short_then)
- [x] Shorthand catch: `{#await expr catch err}...{/await}` (test: await_short_catch)
- [x] Shorthand then + separate catch: `{#await expr then val}...{:catch err}...{/await}` (test: await_then_catch)
- [x] No bindings (separate): `{#await expr}...{:then}...{:catch}...{/await}` (test: await_no_bindings)
- [x] Pending + separate then (no catch): `{#await expr}...{:then val}...{/await}` (test: await_pending_then)
- [x] Pending + separate catch (no then): `{#await expr}...{:catch err}...{/await}` (test: await_pending_catch)
- [x] Shorthand catch without binding: `{#await expr catch}...{/await}` (test: await_short_catch_no_binding)
- [x] Basic `$.await(anchor, thunk, pending, then, catch)` (test: await_basic)
- [x] Thunk optimization for call expressions (test: await_thunk_optimization)
- [x] Async thunk when expression has `await` keyword (test: async_await_has_await)
- [x] Destructured then: object `{a, b}` (test: await_destructured)
- [x] Destructured then: array `[a, b]` (test: await_array_destructured)
- [x] Reactive expression (test: await_reactive)
- [x] Await inside each (test: await_in_each)
- [x] Await inside if (test: await_in_if)
- [x] Each inside await (test: await_each_nested)
- [x] Rich content in all branches (test: await_nested_content)
- [x] Text before element in then (test: await_then_text_before_element)
- [x] Nested await (await inside await) (test: await_nested_await)
- [x] `$.async()` wrapping with blockers (test: async_await_has_await)
- [x] Pickled await in template (test: async_pickled_await_template)
- [x] `block_duplicate_clause` error for duplicate `:then`/`:catch`
- [~] `block_unexpected_character` validation for whitespace before `:then`/`:catch` (analyzer check implemented; blocked on scanner not parsing `{ :then val}` — scanner gap)

## Reference

### Svelte (reference compiler)
- `reference/compiler/phases/1-parse/state/tag.js` — `{#await}`, `{:then}`, `{:catch}` parsing
- `reference/compiler/phases/2-analyze/visitors/AwaitBlock.js` — validation, `mark_subtree_dynamic`
- `reference/compiler/phases/3-transform/client/visitors/AwaitBlock.js` — `$.await()` codegen, `create_derived_block_argument`
- `reference/compiler/types/template.d.ts` — `AwaitBlock` AST type definition

### Our code
- `crates/svelte_ast/src/lib.rs` — `AwaitBlock` AST node
- `crates/svelte_parser/src/scanner/token.rs` — `StartAwaitTag`, `AwaitClauseTag`
- `crates/svelte_parser/src/handlers.rs` — `handle_await_clause_tag`, `handle_end_await_tag`
- `crates/svelte_analyze/src/passes/reactivity.rs` — marks await as dynamic
- `crates/svelte_analyze/src/passes/template_scoping.rs` — then/catch variable scoping
- `crates/svelte_analyze/src/passes/lower.rs` — fragment lowering for pending/then/catch
- `crates/svelte_codegen_client/src/template/await_block.rs` — `gen_await_block`, `gen_await_callback`

## Test cases

- [x] async_await_has_await
- [x] async_pickled_await_template
- [x] await_array_destructured
- [x] await_basic
- [x] await_destructured
- [x] await_each_nested
- [x] await_in_each
- [x] await_in_if
- [x] await_nested_await
- [x] await_nested_content
- [x] await_no_bindings
- [x] await_pending_catch
- [x] await_pending_only
- [x] await_pending_then
- [x] await_reactive
- [x] await_short_catch
- [x] await_short_catch_no_binding
- [x] await_short_then
- [x] await_then_catch
- [x] await_then_text_before_element
- [x] await_thunk_optimization
