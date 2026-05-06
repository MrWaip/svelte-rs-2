# Legacy reactivity system

## Current state
- **Working**: 17/29 (8 base + 9 rune-in-legacy fallback). Umbrella `diagnose_legacy_dev_benchmark` byte-for-byte parity decomposed 2026-05-06 into 16 sub-cases under "Dev-mode legacy parity"; 5 store-related sub-cases moved to `specs/store-subscriptions.md` 2026-05-06, 11 remain here. Umbrella stays open until all sub-cases (here + store spec) close.
- **Tests**: 8/8 green for landed legacy-state lowering. Diagnostic snapshots `validate_derived_rune_invalid_usage_in_non_runes_mode` / `validate_derived_destructured_rune_invalid_usage_in_non_runes_mode` pass. e2e fixtures `derived_non_runes_invalid_usage` + `smoke_legacy_rune_fallback_all` register no `#[ignore]`, match reference byte-for-byte.
- **Synthetic store_sub (closed 2026-05-05)**: `crates/svelte_analyze/src/reactivity_semantics/builder_v2/store.rs::collect_store_declarations` declares synthetic `SymbolId` per unresolved `$name` reference in non-runes mode (sorted by source order via first reference's `ReferenceId`). Synthetic binding lives on root scope with `SymbolOwner::Synthetic`. Downstream `unresolved_store_base` finds it; `record_store_binding` activates legacy store fallback pipeline (`iter_store_bindings` → `lib.rs:154-201` thunks + `setup_stores` + `$$cleanup`) and `dispatch_identifier_read::StoreRead` on rune-shaped callees.
- **Mode-aware rune classification**: rune-shape recognition fully gated on `uses_runes()` in analyze. Transform never re-detects rune kind from AST shape; consults:
  - `crates/svelte_analyze/src/passes/js_analyze/script_runes.rs::collect_script_rune_call_kinds` — empty `script_rune_calls` in non-runes; `transformer/state.rs::rune_kind_from_expr` reads only this index (no `detect_class_field_rune_kind` AST fallback).
  - `crates/svelte_analyze/src/reactivity_semantics/builder_v2/mod.rs::record_rune_declarator` — early-return non-runes.
  - `crates/svelte_analyze/src/utils/script_info.rs::collect_var_declarations` — receives `runes: bool`, suppresses `is_rune` (and `props_declaration` from `$props()` calls) non-runes.
  - `crates/svelte_analyze/src/passes/post_resolve.rs::analyze_declarations` — `data.script.props_id` set only from rune declarations.
  - `crates/svelte_analyze/src/passes/js_analyze/script_body.rs::needs_context_for_program` — `body.has_effects` (from `$effect` rune-shaped callees) gated by `uses_runes`. Synthetic store-sub fallback non-runes does not leak `needs_context = true`, does not force `$.push`/`$.pop`.
  - `crates/svelte_analyze/src/validate/stores.rs::is_synthetic_origin` — `StoreRuneConflict` warning skips synthetic store_sub bindings (analyzer-created, not user-declared, cannot conflict).
  - `crates/svelte_transform/src/transformer/rewrites.rs::identifier_is_store_read` — single helper consulted by `rewrite_call_expression` + `rewrite_shared_call`, skips `$effect.*` / `$state.snapshot` / `$state.eager` / `$effect.pending` / `$host` rewrites when callee classified `StoreRead`. No `self.runes` mode-flag stitching in transform.
- Last updated: 2026-05-06
- Unified reactivity dependency: satisfied. Future legacy-reactivity work builds on landed `ReactivitySemantics` model. Keep explicit legacy-only hooks for containment + removability.
- Member-target legacy state mutations inside template expressions (`{obj.x++}`, `{obj.x += n}`) lower through `rewrite_legacy_state_member_update` / `rewrite_legacy_state_member_assignment`, dispatched from `template_rewrites::rewrite_template_enter` alongside existing deep-store member rewrites. Same helpers serve script-body traversal — identical lowering both contexts.
- Each-item indirect propagation: `crates/svelte_analyze/src/reactivity_semantics/builder_v2/contextual.rs::promote_each_sources_to_legacy_state` (`EachSourcePromoter::visit_each_block`). Each-item mutated → collection symbols promoted to legacy state, indirect links recorded via `add_each_item_indirect_source`. Member-mutations emit `$.invalidate_inner_signals(() => $.get(items))` through existing legacy coarse-wrap path.
- Rune-in-legacy fallback set (moved from `specs/unknown.md` 2026-05-04): non-runes components, reference compiler treats rune-shaped calls as ordinary identifiers — `$derived` resolves as store getter, `$derived(expr)` lowers to `$derived()(expr)`. Our compiler hard-errors at `crates/svelte_analyze/src/validate/runes.rs:744` for `$derived`, never reaches codegen for `diagnose_legacy_dev_benchmark`. Implementation must follow `.claude/skills/legacy-conventions` (Legacy suffix + `LEGACY(svelte4):` doc-comment per struct/function, isolated codepaths).

## Source

- ROADMAP: `Legacy Svelte 4 -> Legacy reactivity system: let var = ''`
- Constraint: keep legacy reactivity isolated behind clearly named legacy-only hooks. Removal mechanical: `grep LEGACY(svelte4)` → delete sites → compile. No smearing Svelte 4 behavior into runes path.
- Adjacent legacy specs:
  - `specs/legacy-reactive-assignments.md` for `$:` statements
  - `specs/legacy-export-let.md` for `export let` / `$$props` / `$$restProps`

## Implementation constraints

- Legacy reactivity on isolated path. Removal mechanical: grep `LEGACY(svelte4)` → delete → compile.
- No Svelte 4 reactivity branches across runes pipeline when dedicated legacy hook can contain.
- Prefer dedicated legacy data structures + helpers over overloading rune/state machinery with hidden mode checks.
- Any new top-level helper/struct/entry point uses explicit legacy naming. Ownership + future deletion obvious.
- Modern pass participates → keep legacy branch as narrow delegation point. Runes path = default flow.
- Legacy-only hooks populate + consume unified `ReactivitySemantics`. No second legacy-only semantic system.

## How It Works

- Applies legacy mode only (`runes={false}` or non-runes). Runes mode → top-level locals stay on rune/state path.
- `export let` / `export var` not owned here: legacy mode → props, not local legacy-reactive state. See `specs/legacy-export-let.md`.
- Reference analyzer starts from normal top-level instance-script bindings, upgrades to legacy `state` when updated and later read from template markup, `$:` statement, or other reactive consumer sites.
- Once classified legacy `state`, client transform registers read/write helpers:
  - `let` state reads → `$.get(name)`
  - `var` state reads → `$.safe_get(name)`
  - assignments → `$.set(name, value)`
  - member mutations → `$.mutate(name, mutation)`
  - updates (`++` / `--`) → `$.update(...)` / `$.update_pre(...)`
- Variable declarations wrapped in `$.mutable_source(...)` only for bindings classified legacy `state`. Plain top-level locals never reactive → plain JS declarations.
- Identifier declarators → `let name = $.mutable_source(init)` or `var name = $.mutable_source(init)`.
- Destructuring declarators → reference destructures through temporary, wraps each bound reactive target separately. Non-reactive destructured targets stay plain.
- Legacy `immutable` mode: binding can still classify `state`. `$.mutable_source(...)` used only when binding reassigned or accessors force source-style. Otherwise declaration stays plain even though binding tracked as legacy state for downstream decisions.
- Reactive reads from member expressions (`object.x`, `items.length`) paired with coarse tracking — typically `$.get(...)` / `$.safe_get(...)` plus `$.untrack(...)` or `$.deep_read_state(...)` where reference needs whole-object invalidation.
- `{#each}` extra rule: each-block context variable reassigned/mutated → collection expression feeding that each-block treated as mutated → can upgrade outer bindings to legacy `state`.

## Syntax variants

- `<script>let count = 0;</script><p>{count}</p>`
- `<script>var count = 0;</script><p>{count}</p>`
- `<script>let object = { x: 0 };</script><p>{object.x}</p>`
- `<script>let numbers = [1, 2, 3]; numbers.push(numbers.length + 1); numbers = numbers;</script><p>{numbers.length}</p>`
- `<script>let { left, right } = point;</script><p>{left}:{right}</p>`

## Use cases

- [x] Top-level legacy `let` lower through `$.mutable_source(...)`, `$.get(...)`, `$.set(...)` legacy mode. Pre: raw `let count = 0`, raw `count += 1`, static text for `{count}`. Test: `legacy_reactivity_let_basic`.
- [x] Top-level legacy `var` use same legacy-state lowering, preserve `$.safe_get(...)` reads for var-declared sources. Matches reference legacy `var` semantics. Test: `legacy_reactivity_var_basic`.
- [x] Member mutations of top-level legacy locals lower through `$.mutate(...)` + coarse member reads. `object.x += 1` invalidates template consumers via legacy runtime, not plain object mutation. Test: `legacy_reactivity_member_mutation`.
- [x] Array-method mutation + explicit self-assignment (`numbers.push(...); numbers = numbers;`) lowers through `$.get(...)` / `$.set(...)` + coarse member reads for `numbers.length`. Test: `legacy_reactivity_array_self_assign`.
- [x] Destructured top-level legacy declarations lower through legacy-state declarator path. Each bound name = own mutable source. Destructuring reassignment → `$.set(...)` updates. Test: `legacy_reactivity_destructure`.
- [x] Member update of top-level legacy state inside template expression (`{obj.x++}`) lowers to `($.get(obj), $.untrack(() => $.mutate(obj, $.get(obj).x++)))`. `template_rewrites::rewrite_template_enter` dispatches `rewrite_legacy_state_member_update` for `UpdateExpression`. Legacy coarse-wrap activates: `UpdateExpression` → `ExpressionKind::Update`. Test: `legacy_state_member_update_in_template`.
- [x] Compound member assignment to top-level legacy state inside template expression (`{obj.x += 5}`) lowers to `($.get(obj), $.untrack(() => $.mutate(obj, $.get(obj).x += 5)))` via same template-enter dispatch into `rewrite_legacy_state_member_assignment`. Test: `legacy_state_member_compound_in_template`.
- [x] Each-item member mutation through `{#each items as item}` propagates indirect-binding back to iterated collection. Collection declarator upgrades `let items = [...]` → `let items = $.mutable_source([...])`. Member-mutations in template effect emit `$.invalidate_inner_signals(() => $.get(items))` (mirrors reference `legacy_indirect_bindings`). Owner: `crates/svelte_analyze/src/reactivity_semantics/builder_v2/contextual.rs::EachSourcePromoter` + standard legacy coarse-wrap codegen. Test: `smoke_legacy_contextual_mutations_all`.

### Rune-in-legacy fallback (moved from specs/unknown.md 2026-05-04)

Reference compiler does not treat `$state`, `$derived`, `$props`, `$effect`, `$inspect`, `$bindable` (or member forms) as runes when `runes:false`. Instead degrade to ordinary identifier references — typically resolve to legacy store-getters (`$name → $.store_get(name)`). No `rune_invalid_usage` diagnostic emitted. Repro/test: `diagnose_legacy_dev_benchmark`. Owning layer split: hard-error removal in analyze (`validate/runes.rs:744`), call-site lowering in transform/codegen. All new code follows `.claude/skills/legacy-conventions`.

- [x] Remove `runes:false` hard-error for `$derived` at `crates/svelte_analyze/src/validate/runes.rs:744`. Reference emits no `rune_invalid_usage` non-runes. Tests `validate_derived_rune_invalid_usage_in_non_runes_mode`, `validate_derived_destructured_rune_invalid_usage_in_non_runes_mode` baselined empty, no `#[ignore]`.
- [x] `$derived(expr)` non-runes → `$derived()(expr)`. `$derived` resolves as synthetic store getter. No `$.derived(() => expr)` wrapping. Test: `derived_non_runes_invalid_usage`.
- [x] `$derived.by(fn)` non-runes → `$derived().by(() => fn)` (member-access on store value). Test: `smoke_legacy_rune_fallback_all`.
- [x] `$state(initial)` non-runes → `$state()(initial)`. No `$.state(...)` / `$.proxy(...)` wrapping. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$state.raw(initial)` + `$state.snapshot(value)` non-runes → `$state().raw(initial)` / `$state().snapshot(value)` — plain member calls on store value. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$props()`, `$props.id()` non-runes → `$props()()` and `$props().id()`. No `props_declaration`, `props_id`, `$.props_id()` hoist, `$$props` parameter, `$.push`/`$.pop`. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$effect(fn)`, `$effect.pre(fn)`, `$effect.tracking()` non-runes → `$effect()(fn)` / `$effect().pre(fn)` / `$effect().tracking()`. No `$.user_effect` / `$.user_pre_effect` / `$.effect_tracking` wrapping. `body.has_effects` does not propagate to `needs_context`. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$inspect(...)` non-runes → `$inspect()(value)` — plain store-thunk call. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$bindable(default)` non-runes → `$bindable()(default)` — plain store-thunk call. Test: `smoke_legacy_rune_fallback_all`.
- [ ] (umbrella) `diagnose_legacy_dev_benchmark` produces JS matching reference `case-svelte.js` byte-for-byte under `runes:false, dev:true`. Closes only when all "Dev-mode legacy parity" sub-cases close.

### Dev-mode legacy parity (decomposed from `diagnose_legacy_dev_benchmark` 2026-05-06)

Each sub-case = one independent divergence cluster from diff `tasks/compiler_tests/cases2/diagnose_legacy_dev_benchmark/{case-svelte.js,case-rust.js}`. Each gets minimal compiler test under `tasks/compiler_tests/cases2/legacy_dev_<slug>/`. New structs/functions follow `.claude/skills/legacy-conventions` (Legacy suffix + `LEGACY(svelte4):` doc-comment + isolated codepaths).

- [ ] **`bind:this` targets promote to legacy mutable_source**: `<el bind:this={target}>` with `let target` → `target` = legacy state. Declaration → `let target = $.mutable_source()`. Bind setter → `($$value) => $.set(target, $$value), () => $.get(target)`. Owner: analyze (legacy state classification). Test: `legacy_dev_bind_this_promotes_state`.
- [ ] **`$inspect(...)` falls back to synthetic-thunk call**: non-runes, `$inspect(a, b)` → `$inspect()(a, b)` (synthetic store thunk invoked, then called with args). Not `$.inspect(() => [...], handler, true)`. Owner: transform (rune call rewrite). Test: `legacy_dev_inspect_fallback`.
- [ ] **Legacy template effect uses `$.deferred_template_effect`**: legacy non-runes, catch-all template effect helper = `$.deferred_template_effect(...)`. Not `$.effect(...)`. Owner: codegen_client (template effect emission). Test: `legacy_dev_deferred_template_effect`.
- [ ] **Reactive text via `$.child` + `$.reset` + `$.set_text` for text-only elements**: legacy mode, element with one dynamic text child (`<p>{x}</p>`, `<strong>{x}</strong>`, `<svelte:element>...{x}</svelte:element>`) emits template HTML with single space placeholder (`<p> </p>`) + runtime sequence `var t_N = $.child(p); $.reset(p); ... $.template_effect(() => $.set_text(t_N, ...));`. Not `p.textContent = ...` / `text.nodeValue = ...`. Owner: codegen_client (template_html + text-only element handling). Test: `legacy_dev_reactive_text_only_element`.
- [ ] **Legacy `{@const}` deep-read coarse-tracking**: `{@const X = obj.member}` initializer legacy mode → `$.derived_safe_equal(() => ($.get(obj), $.untrack(() => $.get(obj).member)))` — coarse-read of source + untracked fine-grained access. Owner: analyze + transform/codegen (member-read coarse wrap on read sites). Test: `legacy_dev_const_deep_read_wrap`.
- [ ] **Each-item `{@const}` deep_read prelude with `$.deep_read_state`**: `{#each items as item, idx}{@const X = ...}` initializers coarse-read each-item locals via `$.deep_read_state($.get(idx))` + same coarse+untrack wrap from cluster #9. Owner: analyze (each-item indirect reads). Test: `legacy_dev_each_const_deep_read`.
- [ ] **Group consecutive attribute setters into one `$.template_effect`**: element with multiple dynamic attributes (`<input {title} {state} value={count}>`) → all emitted inside single `$.template_effect(() => { $.set_attribute(...); $.set_attribute(...); $.set_value(...); });`. Not one inline + separate effect for rest. Owner: codegen_client (attribute effect grouping). Test: `legacy_dev_attribute_effect_grouping`.
- [ ] **Component event-prop `getHandler()` uses `derived_safe_equal` + `untrack`**: callee-call `getHandler()` passed as component event prop (`onclick={getHandler()}`) → `let $0 = $.derived_safe_equal(() => $.untrack(getHandler));`. Not `$.derived(getHandler)`. Owner: codegen_client (component prop derived). Test: `legacy_dev_component_event_prop_derived`.
- [ ] **Component prop forwarding emits getters legacy mode**: shorthand `{title}` forwarded to child component → `{ get title() { return title; } }`. Not `{ title }`. Owner: codegen_client (component invocation). Test: `legacy_dev_component_prop_getter`.
- [ ] **Component invocation emits `$$legacy: true` flag**: props-object passed to `$.component(...)` for non-snippet child components includes literal key/value `$$legacy: true` legacy mode. Owner: codegen_client (component invocation). Test: `legacy_dev_component_legacy_flag`.
- [ ] **`export const` / `export function` emit `$.bind_prop` at script tail**: every `export const X = ...` / `export function X(...) { ... }` legacy mode appends `$.bind_prop($$props, "X", X);` at end of component function body. Owner: codegen_client (script tail). Test: `legacy_dev_export_bind_prop`.
- [ ] **Store-related dev-mode legacy parity sub-cases moved to `specs/store-subscriptions.md`** — synthetic store-thunk getters, no `mutable_source` promotion for `writable()` w/o reassign, store dereference call shape, `$X = expr` → `$.store_set`, `bind:` setter `$.store_unsub` wrap. Closure of `diagnose_legacy_dev_benchmark` umbrella requires those sub-cases to close in the store spec.

## Out of scope

- `$:` reactive statements + dependency graph (`specs/legacy-reactive-assignments.md`)
- Legacy prop bags + `export let` (`specs/legacy-export-let.md`)
- SSR behavior legacy mode

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
- [x] e2e smoke: `smoke_legacy_reactive_mutations_all` — covers every assignment + update operator (`=`, `+=`, `-=`, `++`, `--`, `++` prefix, `--` prefix, `&&=`, `||=`, `??=`) for legacy state identifier + member targets — static (`obj.x`), computed string (`obj["x"]`), computed dynamic (`obj[key]`) member access — script body + template expressions, alongside legacy bindable, store, deep-store paths.
- [ ] `diagnose_legacy_dev_benchmark` — umbrella, `#[ignore]`d. Closes when every "Dev-mode legacy parity" sub-case passes (including the store sub-cases tracked in `specs/store-subscriptions.md`).
- [ ] `legacy_dev_bind_this_promotes_state`
- [ ] `legacy_dev_inspect_fallback`
- [ ] `legacy_dev_deferred_template_effect`
- [ ] `legacy_dev_reactive_text_only_element`
- [ ] `legacy_dev_const_deep_read_wrap`
- [ ] `legacy_dev_each_const_deep_read`
- [ ] `legacy_dev_attribute_effect_grouping`
- [ ] `legacy_dev_component_event_prop_derived`
- [ ] `legacy_dev_component_prop_getter`
- [ ] `legacy_dev_component_legacy_flag`
- [ ] `legacy_dev_export_bind_prop`
