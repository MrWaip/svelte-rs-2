# Legacy reactivity system

## Current state
- Working: 53/53
- Tests: 35/35
- Last updated: 2026-05-21

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

- [x] Top-level legacy `let` lower through `$.mutable_source(...)`, `$.get(...)`, `$.set(...)` legacy mode. Test: `legacy_reactivity_let_basic`.
- [x] Top-level legacy `var` use same legacy-state lowering, preserve `$.safe_get(...)` reads for var-declared sources. Test: `legacy_reactivity_var_basic`.
- [x] Member mutations of top-level legacy locals lower through `$.mutate(...)` + coarse member reads. Test: `legacy_reactivity_member_mutation`.
- [x] Array-method mutation + explicit self-assignment (`numbers.push(...); numbers = numbers;`) lowers through `$.get(...)` / `$.set(...)` + coarse member reads for `numbers.length`. Test: `legacy_reactivity_array_self_assign`.
- [x] Destructured top-level legacy declarations lower through legacy-state declarator path. Each bound name = own mutable source. Test: `legacy_reactivity_destructure`.
- [x] Member update of top-level legacy state inside template expression (`{obj.x++}`) lowers to `($.get(obj), $.untrack(() => $.mutate(obj, $.get(obj).x++)))`. Test: `legacy_state_member_update_in_template`.
- [x] Compound member assignment to top-level legacy state inside template expression (`{obj.x += 5}`) lowers to `($.get(obj), $.untrack(() => $.mutate(obj, $.get(obj).x += 5)))`. Test: `legacy_state_member_compound_in_template`.
- [x] Each-item member mutation through `{#each items as item}` propagates indirect-binding back to iterated collection. Collection declarator upgrades to `$.mutable_source([...])`; member-mutations emit `$.invalidate_inner_signals(() => $.get(items))`. Test: `smoke_legacy_contextual_mutations_all`.
- [x] Legacy `{#each}` iter param promoted to `mutable_source` when transitively mutated through `{@const}` + bind chain. Test: `legacy_const_each_bind_member_chain`.
- [x] Top-level `const` with member mutation promotes to legacy state (auto SoftLegacy + explicit `runes:false`). Test: `auto_softlegacy_const_member_mutation`.
- [x] Legacy `{#if expr}` condition reads of legacy-state member-expressions apply coarse-wrap at codegen. Test: `legacy_const_member_mutation_through_ts_non_null`.
- [x] TS wrappers (`!`, `as`, `<T>`) on the member-mutation target skip `const`/`let` legacy-state promotion. Test: `legacy_const_member_mutation_through_ts_non_null`.
- [x] Bind-setter `$.mutate` wrap for legacy-state member mutation. Test: `legacy_state_bind_member_mutate_wrap`.
- [x] Top-level `let` written + read only inside a single function called from template stays plain JS — not promoted to `$.mutable_source(...)`. Reference treats indirect reads via a function body as non-reactive consumer; only direct template/`$:`/other-reactive-site reads upgrade. Also: `{#if fn()}` derived condition for such a function emits `$.derived(() => $.untrack(fn))`. Test: `diagnose_legacy_local_var_not_promoted_to_state`.
- [x] Destructured `{@const}` initializer rooted on each-item reactive binding coarse-wraps: `{#each rows as row}{@const { x, y } = lookup[row.key]}` → `const { x, y } = ($.get(row), $.untrack(() => lookup[$.get(row).key]))`. Currently emits bare `lookup[$.get(row).key]` with no coarse wrap. Test: `diagnose_legacy_each_const_destructure_coarse_wrap`.
- [x] `{#if call(item.x)}` derived condition rooted on each-item reactive binding coarse-wraps: `var d = $.derived(() => ($.get(row), $.untrack(() => check($.get(row).key))))`. Currently emits bare call inside derived without coarse wrap. Test: `diagnose_legacy_each_if_condition_coarse_wrap`.
- [x] `{#if local && $store.length > 0}` legacy `$.if` predicate with store auto-subscription inside short-circuit position coarse-wraps: `if ($items(), $.untrack(() => local && $items().length > 0)) $$render(...)`. Currently emits bare `local && $items().length > 0` without hoisting `$items()` read or untracking. Test: `diagnose_legacy_if_store_short_circuit_coarse_wrap`.
- [x] Legacy `{#each items as item, index}` `index` binding stays a plain JS param — passing `{index + 1}` (or any pure expression in `index`) as a component prop must emit the value inline (`icon: index + 1`) and must NOT wrap it in `let $0 = $.derived_safe_equal(() => index + 1)` + getter. Reference treats `index` as non-reactive (no `$.get`), so the expression has no reactive root and the safe-equal derived hoist is wrong. Test: `diagnose_legacy_each_index_component_prop_plain`.
- [x] Legacy `--css-prop={expr}` on a component child of `{#each items as item}` coarse-wraps each-item member access inside the `$.css_props` callback: `--tone={item.muted ? undefined : "accent"}` → `"--tone": ($.get(item), $.untrack(() => $.get(item).muted ? undefined : "accent"))`. Currently emits bare `$.get(item).muted ? ...`. Same coarse-wrap family as the `{@const}` / `{#if}` cases above, extended to css-prop attribute expressions on components. Test: `diagnose_legacy_each_css_props_member_coarse_wrap`.
- [x] Legacy `{@html item.member}` inside `{#each items as item}` coarse-wraps the `$.html` getter: `{@html row.content}` → `$.html(p, () => ($.get(row), $.untrack(() => $.get(row).content)), true)`. Currently emits bare `() => $.get(row).content` without coarse wrap, mirroring the missing wrap on the `{@const}` / `{#if}` / `--css-prop` family for `{@html}` getter callbacks. Test: `diagnose_legacy_each_html_member_coarse_wrap`.
- [x] Top-level `let` reassigned at script top level and read only inside an `export const` arrow (never reached from template or other reactive consumer) stays plain JS — not promoted to `$.mutable_source(...)`. Reassignment alone is not a reactive consumer. Sibling of line 66 case but for an exported method body, where the function is exposed via component API rather than called from template. Test: `legacy_let_reassigned_unread_stays_plain`.
- [x] Top-level `let` written by a `$:` assignment and read inside a top-level function passed by name to a template event handler (`on:click={fn}`) promotes to `$.mutable_source(...)` — `$:` write counts as a reactive consumer for the binding even when the only template-facing read site is an indirect function reference. Initializer presence irrelevant. Inverse of line 66 (no `$:` write) and line 73 (no template-reachable read). Test: `diagnose_legacy_reactive_assignment_promotes_state_via_handler`.
- [x] Top-level `let` read + written only inside a nested function expression that is itself the RHS of a `$:` assignment (e.g. `$: handler = async () => { ...flag... flag = true }`) promotes to `$.mutable_source(...)`. The reactive consumer is the function body reachable from the template via the `$:`-assigned name, not a direct `$:` expression. Required emissions: declarator → `let flag = $.mutable_source(false)`; reads inside arrow → `$.get(flag)`; assignments inside arrow → `$.set(flag, ...)`; the surrounding `$.legacy_pre_effect` deps include `$.get(flag)`. Currently the analyzer does not recurse into nested function expressions of `$:` RHS, so the binding stays plain JS and the deps list misses it. Test: `diagnose_legacy_reactive_arrow_value_promotes_state`.
- [x] Plain (non-`$:`) array-pattern destructuring assignment inside a nested function body lowers to the IIFE+`$.to_array` setter wrapper whenever any LHS target is a legacy `mutable_source` binding, even when other targets are plain JS locals. Reference emits `(($$value) => { var $$array = $.to_array($$value, N); $.set(reactive, $$array[i]); plain = $$array[j]; ... })(rhs)`. Test: `diagnose_legacy_array_destructure_mixed_targets`.
- [x] Legacy `--css-prop="prefix-{item.x}-suffix"` (concat / string-template value) on a component child of `{#each items as item}` coarse-wraps each per-part each-item member access inside the `$.css_props` callback template literal: `--tone="prefix-{row.kind}-suffix"` → `"--tone": \`prefix-${($.get(row), $.untrack(() => $.get(row).kind)) ?? ""}-suffix\``. Sibling of line 71 — extends the same coarse-wrap family to the concat / string-template value shape. Test: `diagnose_legacy_each_css_props_concat_member_coarse_wrap`.

### Rune-in-legacy fallback

Reference compiler does not treat `$state`, `$derived`, `$props`, `$effect`, `$inspect`, `$bindable` (or member forms) as runes when `runes:false`. Instead degrade to ordinary identifier references — typically resolve to legacy store-getters. No `rune_invalid_usage` diagnostic emitted.

- [x] Remove `runes:false` hard-error for `$derived`. Tests: `validate_derived_rune_invalid_usage_in_non_runes_mode`, `validate_derived_destructured_rune_invalid_usage_in_non_runes_mode`.
- [x] `$derived(expr)` non-runes → `$derived()(expr)`. Test: `derived_non_runes_invalid_usage`.
- [x] `$derived.by(fn)` non-runes → `$derived().by(() => fn)`. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$state(initial)` non-runes → `$state()(initial)`. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$state.raw(initial)` + `$state.snapshot(value)` non-runes → plain member calls on store value. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$props()`, `$props.id()` non-runes → `$props()()` and `$props().id()`. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$effect(fn)`, `$effect.pre(fn)`, `$effect.tracking()` non-runes → plain store-thunk member calls. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$inspect(...)` non-runes → `$inspect()(value)`. Test: `smoke_legacy_rune_fallback_all`.
- [x] `$bindable(default)` non-runes → `$bindable()(default)`. Test: `smoke_legacy_rune_fallback_all`.
- [ ] (umbrella) `diagnose_legacy_dev_benchmark` produces JS matching reference `case-svelte.js` byte-for-byte under `runes:false, dev:true`. Closes only when all "Dev-mode legacy parity" sub-cases close.

### Dev-mode legacy parity

Each sub-case = one independent divergence cluster from diff `tasks/compiler_tests/cases2/diagnose_legacy_dev_benchmark/{case-svelte.js,case-rust.js}`.

- [x] `bind:this` targets promote to legacy mutable_source. Test: `legacy_dev_bind_this_promotes_state`.
- [x] `$inspect(...)` falls back to synthetic-thunk call. Test: `legacy_dev_inspect_fallback`.
- [x] Legacy template effect uses `$.deferred_template_effect` for `<svelte:head><title>{expr}</title>` reading legacy reactive state. Test: `legacy_dev_deferred_template_effect`.
- [x] Reactive text via `$.child` + `$.reset` + `$.set_text` for text-only elements. Test: `legacy_dev_reactive_text_only_element`.
- [x] Legacy `{@const}` deep-read coarse-tracking: `{@const X = obj.member}` initializer → `$.derived_safe_equal(() => ($.get(obj), $.untrack(() => $.get(obj).member)))`. Test: `legacy_dev_const_deep_read_wrap`.
- [x] Each-item `{@const}` initializer in legacy mode coarse-wraps reactive contextual member roots: `{#each items as item, idx}{@const X = ...}` whose initializer roots a member/call expression on `item`/`idx`/snippet-param/let-directive/await-value/await-error promotes to `$.derived_safe_equal(() => (<dep-reads>, $.untrack(() => <orig>)))`. Test: `legacy_dev_each_const_deep_read`.
- [x] Group consecutive attribute setters into one `$.template_effect`. Test: `legacy_dev_attribute_effect_grouping`.
- [x] Component event-prop `getHandler()` uses `derived_safe_equal` + `untrack`: `onclick={getHandler()}` → `let $0 = $.derived_safe_equal(() => $.untrack(getHandler));`. Test: `legacy_dev_component_event_prop_derived`.
- [x] Component prop forwarding emits getters legacy mode: shorthand `{title}` → `{ get title() { return title; } }`. Test: `legacy_dev_component_prop_getter`.
- [ ] Component invocation emits `$$legacy: true` flag. Test: `legacy_dev_component_legacy_flag`.
- [x] `export const` / `export function` emit `$.bind_prop` at script tail in non-runes mode. Test: `legacy_export_const_emits_bind_prop`.
- [x] Destructured `const` legacy-state promotion with member mutation. Test: `legacy_const_destructured_member_bind`.
- [x] Destructured `const` with mixed promoted/non-promoted bindings keeps non-promoted siblings in the rewritten declarator list across plain/nested object, array (`$$array_N = $.derived(() => $.to_array(...))` wrap), and key-rename patterns. Test: `diagnose_legacy_const_destructure_keeps_siblings`.
- [x] HardLegacy import-base call-expression coarse wrap via `$.deep_read_state`. Test: `auto_hardlegacy_import_call_coarse_wrap`.
- [x] `legacy_pre_effect` block placement is after script function declarations. Test: `diagnose_legacy_pre_effect_order_after_functions`.
- [x] `$.template_effect`/`derived_safe_equal` coarse wrap for prop call + member. Test: `diagnose_legacy_template_effect_prop_call_coarse_wrap`.
- [x] Component prop initialized by a member-call on a non-reactive `const` binding in legacy mode wraps the call body in `$.untrack`: `let $0 = $.derived_safe_equal(() => $.untrack(() => tracker.click()));`. Test: `diagnose_legacy_component_prop_const_call_safe_equal_untrack`.
- [x] Component string-interpolation prop with a call expression whose argument is a legacy `export let` prop coarse-wraps inside the `derived_safe_equal` getter: `<Badge text="a {f(x)} b" />` → `let $0 = $.derived_safe_equal(() => ($.deep_read_state(x()), $.untrack(() => f(x()))))`. Test: `diagnose_legacy_component_prop_call_with_prop_arg_coarse_wrap`.
- [x] `<svelte:head><title>{call($store.member)}</title>` single-expression title in legacy mode coarse-wraps the `$.deferred_template_effect` dep getter: emit `[() => ($store(), $.untrack(() => call($store().member)))]`. Test: `diagnose_legacy_head_title_store_deps_coarse_wrap`.
- [x] Component prop value `ConditionalExpression` whose consequent/alternate is an `ArrowFunctionExpression` (closure deferring all reactive reads) must emit the prop derived without coarse wrapping: `<Child onclick={isButton ? () => onAction(item) : undefined} />` → `let $0 = $.derived_safe_equal(() => isButton() ? () => onAction()(item()) : undefined)`. Distinguishing rule: member/call roots inside `ArrowFunctionExpression`/`FunctionExpression` bodies are deferred and must not drive the outer `LegacyWrap::CoarseWrap` decision. Test: `diagnose_legacy_component_prop_ternary_arrow_no_coarse_wrap`.
- [ ] Store-related dev-mode legacy parity sub-cases moved to `specs/store-subscriptions.md` — closure of `diagnose_legacy_dev_benchmark` umbrella requires those sub-cases to close in the store spec.

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
- [x] `smoke_legacy_reactive_mutations_all`
- [x] `auto_softlegacy_const_member_mutation`
- [x] `legacy_const_each_bind_member_chain`
- [x] `legacy_state_bind_member_mutate_wrap`
- [x] `legacy_const_member_mutation_through_ts_non_null`
- [x] `legacy_const_destructured_member_bind`
- [x] `diagnose_legacy_const_destructure_keeps_siblings`
- [ ] `diagnose_legacy_dev_benchmark` — umbrella, `#[ignore]`d.
- [x] `legacy_dev_bind_this_promotes_state`
- [x] `legacy_dev_inspect_fallback`
- [x] `legacy_dev_deferred_template_effect`
- [x] `legacy_dev_reactive_text_only_element`
- [x] `legacy_dev_const_deep_read_wrap`
- [x] `legacy_dev_each_const_deep_read`
- [x] `legacy_dev_attribute_effect_grouping`
- [x] `legacy_dev_component_event_prop_derived`
- [x] `diagnose_legacy_component_prop_const_call_safe_equal_untrack`
- [x] `legacy_dev_component_prop_getter`
- [ ] `legacy_dev_component_legacy_flag`
- [x] `legacy_export_const_emits_bind_prop`
- [x] `auto_hardlegacy_import_call_coarse_wrap`
- [x] `diagnose_legacy_pre_effect_order_after_functions`
- [x] `diagnose_legacy_template_effect_prop_call_coarse_wrap`
- [x] `diagnose_legacy_local_var_not_promoted_to_state`
- [x] `diagnose_legacy_reactive_arrow_value_promotes_state`
- [x] `diagnose_legacy_each_const_destructure_coarse_wrap`
- [x] `diagnose_legacy_each_if_condition_coarse_wrap`
- [x] `diagnose_legacy_if_store_short_circuit_coarse_wrap`
- [x] `diagnose_legacy_each_index_component_prop_plain`
- [x] `diagnose_legacy_each_css_props_member_coarse_wrap`
- [x] `diagnose_legacy_each_html_member_coarse_wrap`
- [x] `diagnose_legacy_component_prop_call_with_prop_arg_coarse_wrap`
- [x] `diagnose_legacy_component_prop_ternary_arrow_no_coarse_wrap`
- [x] `diagnose_legacy_head_title_store_deps_coarse_wrap`
- [x] `legacy_let_reassigned_unread_stays_plain`
- [x] `diagnose_legacy_reactive_assignment_promotes_state_via_handler`
- [x] `diagnose_legacy_array_destructure_mixed_targets`
- [x] `diagnose_legacy_each_css_props_concat_member_coarse_wrap`
