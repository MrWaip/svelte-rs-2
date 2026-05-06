# Legacy reactivity system

## Current state
- **Working**: 17/18 use cases (8 base + 9 rune-in-legacy fallback). Only `diagnose_legacy_dev_benchmark` byte-for-byte parity remains open (use case №9).
- **Tests**: 8/8 green for landed legacy-state lowering; diagnostic snapshots `validate_derived_rune_invalid_usage_in_non_runes_mode` / `validate_derived_destructured_rune_invalid_usage_in_non_runes_mode` pass; e2e fixtures `derived_non_runes_invalid_usage` and `smoke_legacy_rune_fallback_all` register without `#[ignore]` and match reference byte-for-byte.
- **Synthetic store_sub mechanism (closed 2026-05-05)**: `crates/svelte_analyze/src/reactivity_semantics/builder_v2/store.rs::collect_store_declarations` declares a synthetic `SymbolId` for every unresolved `$name` reference in non-runes mode (sorted by source order via the first reference's `ReferenceId`). The synthetic binding lives on the root scope with `SymbolOwner::Synthetic`; downstream `unresolved_store_base` finds it and `record_store_binding` activates the legacy store fallback pipeline (`iter_store_bindings` → `lib.rs:154-201` thunks + `setup_stores` + `$$cleanup`) and `dispatch_identifier_read::StoreRead` on rune-shaped callees.
- **Mode-aware rune classification**: rune-shape recognition is fully gated on `uses_runes()` in analyze. Transform never re-detects rune kind from AST shape; instead it consults:
  - `crates/svelte_analyze/src/passes/js_analyze/script_runes.rs::collect_script_rune_call_kinds` — empty `script_rune_calls` in non-runes; `transformer/state.rs::rune_kind_from_expr` reads only this index (no `detect_class_field_rune_kind` AST fallback).
  - `crates/svelte_analyze/src/reactivity_semantics/builder_v2/mod.rs::record_rune_declarator` — early-return in non-runes.
  - `crates/svelte_analyze/src/utils/script_info.rs::collect_var_declarations` — receives `runes: bool`, suppresses `is_rune` classification (and `props_declaration` population from `$props()` calls) in non-runes.
  - `crates/svelte_analyze/src/passes/post_resolve.rs::analyze_declarations` — `data.script.props_id` is only set from rune declarations.
  - `crates/svelte_analyze/src/passes/js_analyze/script_body.rs::needs_context_for_program` — `body.has_effects` (from `$effect` rune-shaped callees) is gated by `uses_runes` so synthetic store-sub fallback in non-runes does not leak `needs_context = true` and does not force `$.push`/`$.pop`.
  - `crates/svelte_analyze/src/validate/stores.rs::is_synthetic_origin` — `StoreRuneConflict` warning skips synthetic store_sub bindings (they are analyzer-created, not user-declared, so they cannot conflict).
  - `crates/svelte_transform/src/transformer/rewrites.rs::identifier_is_store_read` — single helper consulted by `rewrite_call_expression` and `rewrite_shared_call` to skip `$effect.*` / `$state.snapshot` / `$state.eager` / `$effect.pending` / `$host` rewrites when the callee identifier is classified as `StoreRead` by analyze. No `self.runes` mode-flag stitching in transform.
- Last updated: 2026-05-05
- Unified reactivity dependency status: satisfied. Future legacy-reactivity work should build on the landed `ReactivitySemantics` model while keeping explicit legacy-only hooks for containment and removability.
- Member-target legacy state mutations inside template expressions (`{obj.x++}`, `{obj.x += n}`) lower through `rewrite_legacy_state_member_update` / `rewrite_legacy_state_member_assignment`, dispatched from `template_rewrites::rewrite_template_enter` alongside the existing deep-store member rewrites. Same helpers serve script-body traversal, ensuring identical lowering across both contexts.
- Each-item indirect propagation lives in `crates/svelte_analyze/src/reactivity_semantics/builder_v2/contextual.rs::promote_each_sources_to_legacy_state` (`EachSourcePromoter::visit_each_block`): when an each-item is mutated, collection symbols are promoted to legacy state and indirect links are recorded via `add_each_item_indirect_source`, so member-mutations emit `$.invalidate_inner_signals(() => $.get(items))` through the existing legacy coarse-wrap path.
- Rune-in-legacy fallback set (moved from `specs/unknown.md` on 2026-05-04): in non-runes components reference compiler treats rune-shaped calls as ordinary identifiers — `$derived` resolves as a store getter and `$derived(expr)` lowers to `$derived()(expr)`. Our compiler currently hard-errors at `crates/svelte_analyze/src/validate/runes.rs:744` for `$derived` and never reaches codegen for `diagnose_legacy_dev_benchmark`. Implementation must follow `.claude/skills/legacy-conventions` (Legacy suffix + `LEGACY(svelte4):` doc-comment on every new struct/function, isolated codepaths).

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

### Rune-in-legacy fallback (moved from specs/unknown.md on 2026-05-04)

Reference compiler does not treat `$state`, `$derived`, `$props`, `$effect`, `$inspect`, `$bindable` (or their member forms) as runes when `runes:false`. Instead they degrade to ordinary identifier references — typically resolving to legacy store-getters (`$name → $.store_get(name)`). No `rune_invalid_usage` diagnostic is emitted. Repro/test: `diagnose_legacy_dev_benchmark`. Owning layer is split: hard-error removal in analyze (`validate/runes.rs:744`), call-site lowering in transform/codegen. All new code must follow `.claude/skills/legacy-conventions`.

- [x] Remove the `runes:false` hard-error for `$derived` at `crates/svelte_analyze/src/validate/runes.rs:744`. Reference does not emit `rune_invalid_usage` in non-runes mode; tests `validate_derived_rune_invalid_usage_in_non_runes_mode`, `validate_derived_destructured_rune_invalid_usage_in_non_runes_mode` are baselined empty and registered without `#[ignore]`.
- [x] `$derived(expr)` in non-runes mode lowers to `$derived()(expr)` where `$derived` resolves as a synthetic store getter (no `$.derived(() => expr)` wrapping). Test: `derived_non_runes_invalid_usage`.
- [x] `$derived.by(fn)` in non-runes mode lowers to `$derived().by(() => fn)` (member-access on the store value). Test: `smoke_legacy_rune_fallback_all`.
- [x] `$state(initial)` in non-runes mode passes through as `$state()(initial)`; no `$.state(...)` / `$.proxy(...)` wrapping is generated. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$state.raw(initial)` and `$state.snapshot(value)` in non-runes mode lower to `$state().raw(initial)` / `$state().snapshot(value)` — plain member calls on the store value. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$props()`, `$props.id()` in non-runes mode degrade to `$props()()` and `$props().id()`; no `props_declaration`, `props_id`, `$.props_id()` hoist, `$$props` parameter, or `$.push`/`$.pop`. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$effect(fn)`, `$effect.pre(fn)`, `$effect.tracking()` in non-runes mode lower to `$effect()(fn)` / `$effect().pre(fn)` / `$effect().tracking()`; no `$.user_effect` / `$.user_pre_effect` / `$.effect_tracking` wrapping; `body.has_effects` flag does not propagate to `needs_context`. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$inspect(...)` in non-runes mode lowers to `$inspect()(value)` — plain store-thunk call. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$bindable(default)` in non-runes mode lowers to `$bindable()(default)` — plain store-thunk call. Test: `smoke_legacy_rune_fallback_all`.
- [ ] After fallback emission lands, `diagnose_legacy_dev_benchmark` must produce JS matching reference `case-svelte.js` byte-for-byte under `runes:false, dev:true`.

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
- [ ] `diagnose_legacy_dev_benchmark` — currently `#[ignore]`d; tracks rune-in-legacy fallback set above.
