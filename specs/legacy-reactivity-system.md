# Legacy reactivity system

## Current state
- **Working**: 8/8 use cases
- **Tests**: 8/8 green
- Last updated: 2026-05-01
- Unified reactivity dependency status: satisfied. Future legacy-reactivity work should build on the landed `ReactivitySemantics` model while keeping explicit legacy-only hooks for containment and removability.
- Member-target legacy state mutations inside template expressions (`{obj.x++}`, `{obj.x += n}`) lower through `rewrite_legacy_state_member_update` / `rewrite_legacy_state_member_assignment`, dispatched from `template_rewrites::rewrite_template_enter` alongside the existing deep-store member rewrites. Same helpers serve script-body traversal, ensuring identical lowering across both contexts.
- Each-item indirect propagation lives in `crates/svelte_analyze/src/reactivity_semantics/builder_v2/contextual.rs::promote_each_sources_to_legacy_state` (`EachSourcePromoter::visit_each_block`): when an each-item is mutated, collection symbols are promoted to legacy state and indirect links are recorded via `add_each_item_indirect_source`, so member-mutations emit `$.invalidate_inner_signals(() => $.get(items))` through the existing legacy coarse-wrap path.

## Source

- ROADMAP item: `Legacy Svelte 4 -> Legacy reactivity system: let var = ''`
- Implementation constraint: keep the legacy reactivity path isolated behind clearly named legacy-only analysis/codegen hooks so removal is mechanical later (`grep LEGACY(svelte4)` -> delete those sites -> compile), without smearing Svelte 4 behavior into the runes path
- Adjacent legacy specs:
  - `specs/legacy-reactive-assignments.md` for `$:` statements
  - `specs/legacy-export-let.md` for `export let` / `$$props` / `$$restProps`

## Implementation constraints

- Keep the legacy reactivity system on an isolated legacy-only path so removal is mechanical later: grep `LEGACY(svelte4)`, delete those sites, compile.
- Do not smear Svelte 4 reactivity branches across the normal runes pipeline when a dedicated legacy analysis/codegen hook can contain them.
- Prefer dedicated legacy data structures and helpers over partially overloading modern rune/state machinery with hidden mode checks.
- Any new top-level helper, struct, or entry point added for this feature should use explicit legacy naming so ownership and future deletion are obvious.
- If a modern pass must participate, keep the legacy branch as a narrow delegation point with the main runes path remaining the default flow.
- The legacy-only hooks should populate and consume the unified `ReactivitySemantics` model rather than inventing a second legacy-only semantic classification system.

## How It Works

- This system only applies in legacy mode (`runes={false}` or otherwise non-runes components). In runes mode, top-level locals stay on the normal rune/state path instead.
- `export let` / `export var` are not owned by this system: in legacy mode they become props, not local legacy-reactive state. Their behavior belongs in `specs/legacy-export-let.md`.
- The reference analyzer starts from normal top-level instance-script bindings and upgrades them to legacy `state` when they are updated and later read from template markup, a `$:` statement, or certain other reactive consumer sites.
- Once a binding is classified as legacy `state`, client transform registers read/write helpers for it:
  - `let` state reads become `$.get(name)`
  - `var` state reads become `$.safe_get(name)`
  - assignments become `$.set(name, value)`
  - member mutations become `$.mutate(name, mutation)`
  - updates (`++` / `--`) become `$.update(...)` / `$.update_pre(...)`
- Variable declarations are wrapped in `$.mutable_source(...)` only for bindings that are actually classified as legacy `state` sources. Plain top-level locals that never become reactive should remain plain JS declarations.
- For identifier declarators, the declaration becomes `let name = $.mutable_source(init)` or `var name = $.mutable_source(init)`.
- For destructuring declarators, the reference first destructures through a temporary and then wraps each bound reactive target separately; non-reactive destructured targets remain plain values.
- In legacy `immutable` mode, a binding can still be classified as `state`, but `$.mutable_source(...)` is only used when the binding is reassigned or accessors force source-style behavior. Otherwise the declaration may stay a plain value even though the binding is still tracked as legacy state for downstream decisions.
- Reactive reads from member expressions (`object.x`, `items.length`) are paired with coarse tracking in emitted expressions, typically `$.get(...)` / `$.safe_get(...)` plus `$.untrack(...)` or `$.deep_read_state(...)` where the reference needs whole-object invalidation semantics.
- `{#each}` adds one extra legacy rule: if an each-block context variable is reassigned or mutated, the collection expression feeding that each-block is treated as mutated too, which can upgrade outer bindings into legacy `state`.

## Syntax variants

- `<script>let count = 0;</script><p>{count}</p>`
- `<script>var count = 0;</script><p>{count}</p>`
- `<script>let object = { x: 0 };</script><p>{object.x}</p>`
- `<script>let numbers = [1, 2, 3]; numbers.push(numbers.length + 1); numbers = numbers;</script><p>{numbers.length}</p>`
- `<script>let { left, right } = point;</script><p>{left}:{right}</p>`

## Use cases

- [x] Top-level legacy `let` bindings lower through `$.mutable_source(...)`, `$.get(...)`, and `$.set(...)` in legacy mode instead of remaining plain locals; current Rust still emits raw `let count = 0`, raw `count += 1`, and static text output for `{count}` (test: `legacy_reactivity_let_basic`)
- [x] Top-level legacy `var` bindings use the same legacy-state lowering but preserve `$.safe_get(...)` reads for var-declared sources, matching the reference compiler's legacy `var` semantics (test: `legacy_reactivity_var_basic`)
- [x] Member mutations of top-level legacy locals lower through `$.mutate(...)` and coarse member reads, so `object.x += 1` invalidates template consumers via the legacy runtime instead of mutating a plain object local (test: `legacy_reactivity_member_mutation`)
- [x] Array-method mutation plus explicit self-assignment (`numbers.push(...); numbers = numbers;`) lowers through `$.get(...)` / `$.set(...)` and coarse member reads for dependent expressions like `numbers.length` (test: `legacy_reactivity_array_self_assign`)
- [x] Destructured top-level legacy declarations lower through the legacy-state declarator path so each bound name becomes its own mutable source and destructuring reassignment lowers to `$.set(...)` updates, rather than staying plain destructured locals (test: `legacy_reactivity_destructure`)
- [x] Member update of a top-level legacy state inside a template expression (`{obj.x++}`) lowers to `($.get(obj), $.untrack(() => $.mutate(obj, $.get(obj).x++)))`. `template_rewrites::rewrite_template_enter` dispatches `rewrite_legacy_state_member_update` for `UpdateExpression`; legacy coarse-wrap activates because `UpdateExpression` maps to `ExpressionKind::Update` (test: `legacy_state_member_update_in_template`).
- [x] Compound member assignment to a top-level legacy state inside a template expression (`{obj.x += 5}`) lowers to `($.get(obj), $.untrack(() => $.mutate(obj, $.get(obj).x += 5)))` via the same template-enter dispatch into `rewrite_legacy_state_member_assignment` (test: `legacy_state_member_compound_in_template`).
- [x] Each-item member mutation through `{#each items as item}` propagates an indirect-binding back to the iterated collection. Collection declarator upgrades from `let items = [...]` to `let items = $.mutable_source([...])` and member-mutations in the template effect emit `$.invalidate_inner_signals(() => $.get(items))` (mirrors reference `legacy_indirect_bindings`). Owning area: `crates/svelte_analyze/src/reactivity_semantics/builder_v2/contextual.rs::EachSourcePromoter` + standard legacy coarse-wrap codegen (test: `smoke_legacy_contextual_mutations_all`).

## Out of scope

- `$:` reactive statements and their dependency graph (`specs/legacy-reactive-assignments.md`)
- Legacy prop bags and `export let` (`specs/legacy-export-let.md`)
- SSR behavior for legacy mode

## Reference
### Svelte

- `reference/docs/99-legacy/01-legacy-let.md`
- `reference/docs/99-legacy/00-legacy-overview.md`
- `reference/compiler/phases/1-parse/read/script.js`
- `reference/compiler/phases/1-parse/acorn.js`
- `reference/compiler/phases/scope.js`
- `reference/compiler/phases/2-analyze/index.js`
- `reference/compiler/phases/3-transform/client/utils.js`
- `reference/compiler/phases/3-transform/client/transform-client.js`
- `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/utils.js`
- `reference/compiler/phases/3-transform/client/visitors/shared/declarations.js`

### Our code

- `crates/svelte_analyze/src/scope.rs`
- `crates/svelte_analyze/src/utils/script_info.rs`
- `crates/svelte_analyze/src/passes/js_analyze/script_body.rs`
- `crates/svelte_analyze/src/passes/post_resolve.rs`
- `crates/svelte_analyze/src/reactivity_semantics/builder_v2/legacy.rs`
- `crates/svelte_analyze/src/passes/js_analyze/expression_info.rs`
- `crates/svelte_analyze/src/passes/dynamism.rs`
- `crates/svelte_codegen_client/src/lib.rs`
- `crates/svelte_codegen_client/src/script/model.rs`
- `crates/svelte_codegen_client/src/template/expression.rs`
- `crates/svelte_codegen_client/src/template/html.rs`
- `tasks/compiler_tests/test_v3.rs`

## Test cases

- [x] `legacy_reactivity_let_basic`
- [x] `legacy_reactivity_var_basic`
- [x] `legacy_reactivity_member_mutation`
- [x] `legacy_reactivity_array_self_assign`
- [x] `legacy_reactivity_destructure`
- [x] `legacy_state_member_update_in_template`
- [x] `legacy_state_member_compound_in_template`
- [x] e2e smoke: `smoke_legacy_reactive_mutations_all` — covers every assignment + update operator (`=`, `+=`, `-=`, `++`, `--`, `++` prefix, `--` prefix, `&&=`, `||=`, `??=`) for legacy state identifier and member targets — including static (`obj.x`), computed string (`obj["x"]`), and computed dynamic (`obj[key]`) member access — in both script body and template expressions, alongside legacy bindable, store, and deep-store paths.
