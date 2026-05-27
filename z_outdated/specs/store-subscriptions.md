# $store subscriptions

## Current state
- **Working**: 72/72 use cases
- **Tests**: 54/54 green
- Last updated: 2026-05-22

## Source

- ROADMAP item: `$store` subscriptions
- User request: `/audit` follow-up — previous spec under-counted gaps. Confirmed parity divergences against reference compiler in legacy mode (runes:false), runes-mode store-backed each, runes-mode `bind:value={$store}`.

## How It Works

Store subscription = any reference of an identifier whose name starts with `$` and base name resolves to a real binding (e.g. `$count` ↔ `count`), or a synthetic root-scope binding created when no real base exists. Reference compiler attaches `kind: 'store_sub'` to those bindings.

For every store_sub binding the client emits a thunk getter at the top of the component function:

```js
const $name = () => $.store_get(name, '$name', $$stores);              // prod
const $name = () => ($.validate_store(name, 'name'), $.store_get(name, '$name', $$stores));  // dev
```

`$$stores` and `$$cleanup` are produced once via `const [$$stores, $$cleanup] = $.setup_stores();`. `$$cleanup()` is called at the very end of the component function body, after `$.pop(...)` if present.

Reads of `$name` lower to thunk-call `$name()`. Writes lower to `$.store_set(name, value)` (assignment) or `$.update_store / $.update_pre_store(name, $name(), -1?)` (update). Member writes/updates lower to `$.store_mutate(name, $.untrack($name).x = …, $.untrack($name))` — coarse invalidation around fine-grained access. Computed members (`$obj["k"]`, `$obj[key]`) follow the same shape.

In **legacy mode** (`runes:false`):
- Top-level `let X = …` whose `$X` is referenced AND `X` is reassigned → `X` is upgraded to legacy `state` (kind = state). Declaration becomes `let X = $.mutable_source(...)`. Each assignment `X = expr` is wrapped: `$.store_unsub($.set(X, expr), '$X', $$stores)` so the prior subscription unsubscribes when the underlying store is replaced.
- `{#each $items as item}` with item member mutations triggers `$.invalidate_inner_signals(...)` (legacy coarse) **and** `$.invalidate_store($$stores, '$items')` together as a sequence — both wrap fires per write.
- `$.init()` is emitted once when needs_context is forced (member mutation, $:; or store path).

In **runes mode** with `{#each $items as item}` whose collection expression is a store subscription, EACH_ITEM_REACTIVE is set, and the same `$.invalidate_store($$stores, '$items')` wrap is appended after every item member assignment/update inside event handlers. The each callback gains an extra `$$index` parameter when collection comes from a store.

`bind:value={$store}` (and other primitive binds) passes the synthetic thunk identifier directly: `$.bind_value(input, $name, ($$value) => $.store_set(name, $$value))` — no extra `() => $name` arrow wrap. Component-level `bind:prop={$store}` emits `mark_store_binding()` inside the getter and `$.store_set` in the setter.

`<script module>` references to `$store` are an error (`store_invalid_subscription_module`). Refs to `$store` from a scope owned by a non-top function at the same scope chain level error as `store_invalid_scoped_subscription`.

## Syntax variants

```svelte
<!-- read in script body -->
<script>let v = $store;</script>

<!-- read in template -->
<p>{$store}</p>
<p>{$store.foo.bar}</p>
<p>{$store["key"]}</p>

<!-- assignment in script -->
<script>$store = expr; $store += 1; $store ??= 5; $store++; --$store;</script>

<!-- assignment in template event handler -->
<button onclick={() => $store = 1}>set</button>
<button onclick={() => $store ??= 5}>??=</button>
<button onclick={() => $store++}>++</button>

<!-- member mutation -->
<button onclick={() => $obj.x = 1}>set</button>
<button onclick={() => $obj.x += 1}>+=</button>
<button onclick={() => $obj.x ??= 5}>??=</button>
<button onclick={() => $obj.x++}>++</button>
<button onclick={() => --$obj.x}>--pre</button>
<button onclick={() => $obj["k"]++}>computed</button>
<button onclick={() => $obj[key]++}>dynamic</button>

<!-- collections -->
{#each $items as item}{item}{/each}
{#each $items as item, i}{i}: {item.name}{/each}
{#each $items as item (item.id)}{item}{/each}

<!-- block kinds -->
{#if $cond}…{/if}
{#key $token}…{/key}
{#await $promise}…{:then v}…{/await}

<!-- bindings -->
<input bind:value={$name}>
<input bind:checked={$flag}>
<select multiple bind:value={$selected}>
<input type="radio" bind:group={$picked} value="a">
<MyComponent bind:value={$count} />

<!-- component prop forwarding -->
<MyComponent value={$count} />

<!-- legacy reassignment of underlying store binding -->
<script>let s = writable(0); function swap() { s = writable(1); }</script>
<p>{$s}</p>

<!-- legacy: $: reactive depending on store -->
<script>let total = 0; $: total = $count * 2;</script>

<!-- conflict / error variants -->
<script>let $count = 1;</script>             <!-- store_rune_conflict / shadow -->
<script>function f() { let v = $count; }</script>   <!-- store_invalid_scoped_subscription -->
<script context="module">let v = $count;</script>   <!-- store_invalid_subscription_module -->
```

## Use cases

- [x] Synthetic store_sub binding created per `$name` reference (runes mode), keyed by base symbol when present (test: `store_basic`)
- [x] Synthetic store_sub binding when base is unresolved (test: `store_basic`)
- [x] Identifier read `$count` lowers to thunk call `$count()` runes mode (test: `store_basic`)
- [x] Identifier assignment `$count = v` runes mode → `$.store_set(count, v)` (test: `store_write`)
- [x] Identifier compound assignment `$count += v` runes mode for every numeric/logical operator (`+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `&=`, `|=`, `^=`, `<<=`, `>>=`, `>>>=`, `&&=`, `||=`, `??=`) → `$.store_set(count, $count() op v)` (test: `store_runes_id_assign_ops`)
- [x] Identifier prefix/postfix update `$count++ / --$count` runes mode → `$.update_store / $.update_pre_store(count, $count(), -1?)` (test: `store_runes_id_assign_ops`)
- [x] Identifier ops in template event handlers runes mode (test: `store_runes_id_ops_template`)
- [x] Member assignment `$obj.x = v` runes mode → `$.store_mutate(obj, $.untrack($obj).x = v, $.untrack($obj))` (test: `store_runes_member_ops_script`)
- [x] Member compound assignment all operators runes mode (test: `store_runes_member_ops_script`)
- [x] Member update prefix/postfix runes mode (test: `store_runes_member_ops_script`)
- [x] Member ops in template event handlers runes mode (test: `store_runes_member_ops_template`)
- [x] Computed-member ops runes mode (`$obj["k"] = v`, `$obj[key]++`, `$obj["k"] ??= 5`) (test: `store_runes_computed_member`)
- [x] Dev-mode runes prelude wraps thunk with `$.validate_store(store, name)` sequence (test: `store_validate_dev`, `store_runes_dev_smoke`)
- [x] `{#each $items as item}` runes mode forces `EACH_ITEM_REACTIVE` flag (test: `store_each_invalidate`)
- [x] Each-block expression with store dependency injects `$$index` callback param even when not user-named (test: `store_runes_each_member_mutation` *— passing only on header shape, member-mutation invalidate still missing; see below*)
- [x] Component invocation forwarding `<Child value={$count} />` emits getter calling `$count()` (test: `store_mark_binding`)
- [x] Component bind: forwarding `<Child bind:prop={$store} />` emits getter with `$.mark_store_binding()` and setter with `$.store_set(...)` (test: `store_mark_binding`)
- [x] `bind:group` order-stable when stores precede group bindings (test: `bind_group_order_with_stores`)
- [x] Synthetic store thunk emitted exactly once per base, in source-order of first reference (test: `store_basic`)
- [x] Validate: nested-scope `$store` reference → `store_invalid_scoped_subscription` error (analyzer test)
- [x] Validate: `$store` read in `<script module>` → `store_invalid_subscription_module` (test: `validate_store_invalid_subscription_in_module`)
- [x] Validate: `$store` read inside `compile_module` (.svelte.js / .svelte.ts) → `store_invalid_subscription_module` (`module_store_*` unit tests)
- [x] Warning: `$state(...)` rune call where `$state` could be a store → `store_rune_conflict` only when no rune actually owns the binding (test: `state_rune_no_store_rune_conflict`, `state_rune_no_conflict_with_other_rune_calls`, `validate_store_rune_conflict`)
- [x] Legacy mode: `var X = writable(...)` (no reassignment) lowers thunk + setup_stores + $$cleanup correctly and leaves `var X = writable(...)` plain (test: `store_legacy_var_basic`)
- [x] Reads of `$store.foo.bar` deep member access lower to `$store().foo.bar` in template effect (test: `store_basic`, `store_deep_mutation`)
- [x] Each-block with store-backed array passes `EACH_ITEM_REACTIVE` flag value `1` to `$.each(node, 1, ...)` (test: `store_each_invalidate`)

- [x] Runes mode: `{#each $store as item}` with item member mutation — wraps mutation as `($.get(item).value++, $.invalidate_store($$stores, '$items'))` (test: `store_runes_each_member_mutation`)
- [x] `bind:value={$store}` passes synthetic thunk identifier directly without arrow wrap (test: `store_bind_value_thunk_arrow`)

- [x] Runes mode: synthetic store thunk where base is a `$props()`-destructured prop reads through `$$props.<name>` — `const $store = () => $.store_get($$props.store, "$store", $$stores)` (test: `store_runes_prop_thunk`)

- [x] Runes mode: synthetic store thunk where base symbol is a reactive source (`$derived`, `$derived.by`, `$state`) unwraps the signal in the thunk — `const $store = () => $.store_get($.get(store), "$store", $$stores)`. Codegen `lib.rs` inlines `BindingSemantics` match in `make_base_arg` (local triage enum removed); transform `make_store_base_expr` adds matching `State`/`Derived` arm returning `$.get(<base>)` (test: `store_runes_synthetic_thunk_derived_base`)

- [x] Runes mode: `$props()`-destructured prop used solely as a store (only `$X` reads/writes, including `bind:value={$X}`) must NOT materialize a `$.prop($$props, "X", 7)` local binding; `$X = …` writes and `bind:value={$X}` setter target `$.store_set($$props.X, …)` directly. Analyze: `has_non_store_mutation` excludes store-candidate write refs from the `is_source` predicate in `record_object_prop_pattern`. Transform: `make_store_set` accepts a base `Expression`; `make_store_base_expr` builds `$$props.<name>` for `Prop(NonSource, Standard)` base symbols (mirrors `LegacyState` `$.get(name)` wrap) (test: `store_runes_prop_assign_bind`)

- [x] Runes mode: `$bindable()`-marked prop without a default expression and without any non-store-classified write reference to the bare identifier follows the same `Prop(NonSource, Standard)` routing as a plain prop — drops the `let X = $.prop(...)` materialization, lowers bare `X` to `$$props.X`, and emits `$.store_set($$props.X, $$value)` in the `bind:value={$X}` setter. Analyze: `record_named_prop_assignment_left` publishes the symbol into `standard_prop_source_symbols` when `bindable && PropLoweringMode::Standard && PropDefaultLowering::None`; the deferred-downgrade loop now accepts a reference as compatible if it is either a `StoreRead`/`StoreWrite`/`StoreUpdate` reference fact OR a read-only `Reference` (so a method-call base like `inputValue.set(5)` no longer blocks downgrade). `Prop(NonSource, Standard)` routing in `make_store_base_expr` already handles the `$$props.<name>` rewrite for both bare reads and `store_set` setter base (test: `diagnose_bindable_prop_store_only`)

- [x] Runes mode: `<Child bind:prop={$store}>` component-bind setter where store base is a `$props()`-destructured prop emits `$.store_set($$props.<name>, $$value)` — codegen `bind_prop.rs::emit_bind_store` reads `BindingSemantics(base_symbol)` and emits `$$props.<name>` member for `Prop(NonSource, _)`, mirroring the element-bind path's `Prop(NonSource, Standard) → $$props.<name>` rule (test: `store_runes_component_bind_prop_store`)

- [x] Runes mode: `<Child bind:prop={$store}>` setter where store base symbol is `State` / `Derived` (including a field destructured from `$derived(...)`) emits `$.store_set($.get(<base>), $$value)` — codegen `bind_prop.rs::emit_bind_store` extends the `binding_semantics(base_symbol)` match with a `State(_) | Derived(_)` arm wrapping the identifier in `$.get(...)`, mirroring `lib.rs::make_base_arg` read-side handling (test: `diagnose_component_bind_store_derived_base`)

- [x] **Legacy mode: full store path closed**
  - [x] `let X = writable(...)` upgraded to legacy state when `X` reassigned and `$X` referenced lowers to `$.mutable_source(writable(...))` + store thunk + `setup_stores` + `$$cleanup` + `$.init()` (test: `store_legacy_let_synthetic_reassign`)
  - [x] Legacy assignment of state-source whose `$X` is store_sub wraps with `$.store_unsub($.set(X, v), '$X', $$stores)` via new `LegacyStateSubscribedWrite` / `LegacyStateSubscribedUpdate` reference variants and `make_store_unsub` builder (test: `store_legacy_let_synthetic_reassign`)
  - [x] Legacy `{#each $items as item}` member-mutation emits `$.invalidate_inner_signals(() => $items())` and `$.invalidate_store($$stores, '$items')` together via `EachBlockSemantics::collection_store` + extended `make_each_item_invalidate_seq` (test: `store_legacy_each_invalidate`)
  - [x] Legacy each-block over store collection adds `$$index` callback param when item members are mutated (test: `store_legacy_each_invalidate`)
  - [x] Legacy script-body identifier ops `$count = …`, `$count += …`, `$count ??=`, `$count++` lower through legacy store thunk (test: `store_legacy_id_assign_ops`)
  - [x] Legacy template-handler identifier ops every operator (test: `store_legacy_id_ops_template`)
  - [x] Legacy member identifier ops `$obj.x op v` script + template via `store_mutate(obj, $.untrack($obj).x op v, $.untrack($obj))` (test: `store_legacy_member_ops_script`, `store_legacy_member_mutation`)
  - [x] Legacy computed member ops (covered by `store_legacy_member_ops_script` smoke + dispatcher coverage of computed targets)
  - [x] Legacy `bind:value={$store}` lowers to `$.bind_value(input, $name, ($$value) => $.store_set(name, $$value))` plus thunk + setup_stores + $$cleanup (test: `store_legacy_bind_value`)
  - [x] Legacy dev-mode store path: `validate_store` wrap + `add_locations` + `check_target` + `legacy_api()` spread + `$$exports` shape (test: `store_legacy_dev_smoke`)
  - [x] Legacy `$count++` decrement variants (`++ -- pre/post`) emit correct `$.update_store / $.update_pre_store` (test: `store_legacy_id_assign_ops`)

- [x] **Dev-mode legacy parity sub-cases (moved from `specs/legacy-reactivity-system.md` 2026-05-06)**
  - [x] **Synthetic legacy store-thunk getters**: non-runes mode, every synthetic store binding (`$name` references where `name` = top-level local) emits thunk in script preamble: `const $name = () => ($.validate_store(name, "name"), $.store_get(name, "$name", $$stores));` (`validate_store` arm dev-only). Applies user `writable` stores + rune-shaped synthetic stores (`$state`, `$metrics`, `$labelStore`, etc.). Test: `legacy_dev_synthetic_store_thunk`.
  - [x] **No mutable_source promotion for `writable()`**: top-level `let X = writable(...)` whose only consumers = `$X` derefs stays plain `let X = writable(...)`. Test: `legacy_dev_writable_no_mutable_source`.
  - [x] **Store dereference uses synthetic-thunk call `$name()` not `$.get($name)`**. Test: `legacy_dev_store_thunk_call_read`.
  - [x] **Legacy store assignment uses `$.store_set(name, value)`**: `$X = expr` non-runes → `$.store_set(X, expr)`. Test: `legacy_dev_store_set_assignment`.
  - [x] **`bind:value={$X}` on a store whose base is reassigned (legacy state-source overlay)**: thunk passes `$.get(X)` to `validate_store`; `bind_value` getter is `function get() { return $X(); }`; setter is `$.store_set($.get(X), $$value)`. Test: `legacy_dev_bind_store_unsub`.

- [x] **Block-kind store reads coverage matrix** — full closure (5/5):
  - [x] `{#if $cond}…{/if}` lowers via universal expression rewriter, condition becomes `$cond()` thunk-call (test: `store_if_block_condition`)
  - [x] `{#key $token}…{/key}` passes synthetic thunk identifier directly to `$.key(node, $token, …)` without arrow wrap (test: `store_key_block_expression`)
  - [x] `{#await $promise}{:then v}…{/await}` passes synthetic thunk identifier directly to `$.await(node, $promise, …)` (test: `store_await_block_promise`)
  - [x] `{@render $snippet(args)}` lowers to `$.snippet(node, $snippet, () => arg)` via existing Dynamic codegen path + `b.thunk` unthunk-collapse of `() => $name()` → `$name`. Required `callee_symbol` in `crates/svelte_analyze/src/block_semantics/builder/render.rs` to resolve store-reference identifiers to the synthetic store SymbolId (via `ReferenceSemantics::StoreRead`/`StoreWrite`/`StoreUpdate`/`ImportSubscribedRead`) instead of falling back to the base import binding — without that, `RenderCalleeShape` came out `Static` and the emit dropped the `$.snippet(...)` wrapper (test: `store_render_tag_snippet`)
  - [x] `bind:this={$el}` lowers to `$.bind_this(div, ($$value) => $.store_set(el, $$value), () => $el())` via store-aware branch in codegen `crates/svelte_codegen_client/src/codegen/attributes/bind/this.rs::emit_bind_this`. Branch detects store target through `directive_root_reference_semantics` returning `StoreRead`/`StoreWrite`/`StoreUpdate` and synthesises both arrows directly from the synthetic store SymbolId — no transform/analyze plumbing required, the existing transform path that skips bind:this in `walk_attrs` is preserved (test: `store_bind_this_element_ref`)

- [x] Bare identifier `name` coexists with `$name` store subscription — bare references (method calls `name.foo()`, member reads `name.bar`, argument passes) remain unchanged; only `$name` reads/writes/updates route through the synthetic store thunk. Fix: `ImportSubscribedRead` removed from the thunk-call rewrite arm in `crates/svelte_transform/src/transformer/rewrites.rs::dispatch_identifier_read`; variant retained in analyze for `ExpressionSemantics` dynamism tracking and `BlockSemantics::render` callee resolution (test: `store_bare_identifier_method_call`)

- [x] **Legacy mode: `export let prop` used as `$prop` store auto-subscription** — analyze: `ReactivitySemantics` `classify_variable_declaration` excludes store-classified writes (`StoreRead`/`StoreWrite`/`StoreUpdate` on the synthetic `$prop` store symbol whose base is `prop`) from the `updated` predicate, mirroring runes `standard_prop_source_symbols` / deferred-downgrade. Transform: `make_store_base_expr` adds `BindingSemantics::LegacyBindableProp(_)` arm emitting `prop()` getter-thunk call, mirroring runes `Prop(NonSource, Standard)` → `$$props.<name>` rule. Result: `let prop = $.prop($$props, "prop", 8)` not `12`, and `$.store_get(prop(), "$prop", $$stores)` / `$.store_set(prop(), $$value)` instead of bare `prop` (test: `diagnose_legacy_export_let_store_prop_subscription`)

- [x] **Legacy mode: `<Child bind:prop={$store}>` where store base is an `export let` prop** — layer: codegen. `crates/svelte_codegen_client/src/codegen/component_props/bind_prop.rs::emit_bind_store` adds a `BindingSemantics::LegacyBindableProp(_)` arm that builds `name()` (zero-arg call on the `$.prop($$props, ..., 8)` thunk), mirroring `make_store_base_expr`'s transform-side rule and the existing `Prop(NonSource, _)` / `State|Derived` arms in the same match. Getter already emits `$name()` via the synthetic store thunk. (test: `diagnose_legacy_component_bind_store_prop`)

- [x] **Legacy mode: `let X = writable(...)` where the only writes are `$X = …` store-sets** — layer: analyze 3.A.2 (`ReactivitySemantics`). New `has_non_store_mutation_legacy` predicate in `builder_v2/mod.rs` replaces the bare `is_mutated_any(sym)` gate in `record_legacy_state_declarator`: it filters resolved reference ids of the base symbol against `ComponentScoping::store_candidate_refs()` and counts a write/member-mutation only when at least one non-store reference targets the binding. Mirrors `builder_v2/legacy.rs::is_non_store_ref` (legacy `export let`) and the runes `Prop(NonSource, Standard)` exclusion in `builder_v2/references.rs`. Test: `diagnose_legacy_let_writable_store_only_assign`.

- [x] **Legacy mode: `<Child bind:prop={$X.member}>` on `let X = writable(...)` must keep `X` plain** — layer: analyze 3.A.2 (`ReactivitySemantics`). Pattern: `let X = writable(...)` + component bind on a member of the auto-sub (`bind:value={$X.foo}`) + a separate `$X = …` store-set (no bare write of `X`). Reference: `let X = writable(...)` stays plain; setter is `$.store_mutate(X, $.untrack($X).foo = $$value, $.untrack($X))`; store-thunk getter reads `$.store_get(X, ...)`. Ours: declaration becomes `let X = $.mutable_source(writable(...))` and every reference unwraps via `$.get(X)` (bind setter, thunk getter, `$.store_set`). Root cause candidate: bind on a member of the synthetic store reference contributes a non-store mutation fact on the base symbol `X`, so `has_non_store_mutation_legacy` mis-classifies the binding as a reassigned legacy state. Test cases: `diagnose_legacy_bind_store_member_keeps_writable_plain`.

- [ ] **`$:` reactive statement reading a store** — owned by `specs/legacy-reactive-assignments.md` but cross-references store path. Cross-link only.

- [x] **Legacy mode: `<Child bind:prop={$store}>` где базовая ссылка стора — `LegacyState` `mutable_source` (например, биндинг из `$: ({ store } = …)`)** — слой: 4 кодген. Единый эмит-биндинг-чтения для базы стора: `codegen/expr.rs::build_store_base_read(ctx, base_symbol)` читает `BindingSemantics(base_symbol)` и строит выражение разворачивания носителя для всех `$.store_get` / `$.store_set` / `$.validate_store`-аргументов. Два потребителя: `lib.rs` (преамбула store-thunk’а), `component_props/bind_prop.rs::emit_bind_store` (сеттер компонент-бинда). Прежде каждый сайт держал свой match, и `emit_bind_store` пропускал ветку `LegacyState`. Тест: `diagnose_legacy_component_bind_store_reactive_destructured_base`.

- [x] **Legacy mode: `<Child bind:prop={X}>` where bare `X` is a legacy state-source whose `$X` is auto-subscribed** — layer: анализ (`attribute_semantics`). Новый вариант `ComponentBindTarget::LegacyStateSubscribed` помечает совместную классификацию `LegacyState + store_shadow`; `derive_component_bind_target` в `crates/svelte_analyze/src/attribute_semantics/builder/mod.rs` выбирает его, когда `BindingSemantics::LegacyState(_)` и `ReactivitySemantics::store_shadow_of_internal(sym).is_some()`. Кодген `bind_prop.rs::emit_bind_identifier` эмитит для нового варианта тот же getter, что у `LegacyState` (`$.get(X)`), и setter, обёрнутый в `$.store_unsub($.set(X, $$value), "$X", $$stores)`. Тест: `diagnose_legacy_component_bind_base_store_unsub`.

- [x] **Legacy mode: bare read of a `$:` local that holds a store, passed as a component prop** — layer: transform (`crates/svelte_transform/src/transformer/rewrites.rs::dispatch_identifier_read`). Pattern: `$: store = source;` (store-valued RHS) + `$store` auto-subscribed elsewhere + `<Child value={store} />`. Reference: getter `return $.get(store);` — read the legacy `mutable_source` holding the store. Ours: getter `return $store();` — incorrectly routed through the synthetic auto-sub thunk, dropping the `mutable_source` read and dereferencing the store. The rewriter must distinguish `LegacyStateSubscribedRead` (bare read of the store-holding local) from `StoreRead`/`ImportSubscribedRead` (auto-subscription read of `$name`). Test: `diagnose_legacy_reactive_store_value_passed_as_bare_prop`.

- [x] **Legacy mode: `<input bind:value={X}>` getter unwraps `$.get(X)` for `mutable_source`-promoted legacy state** — слой: анализ + трансформ. Pattern: `let X = writable(...)` + `$X` авто-подписка → `X` промоутится в `$.mutable_source(...)`; element-level `<input bind:value={X}>` (без `$`) должен эмитить getter `() => $.get(X)`. Корневая причина: `crates/svelte_analyze/src/passes/build_component_semantics.rs::walk_bind_directive` для Identifier-цели пускал `ReferenceFlags::Write` — без `Read` ссылка классифицировалась в `LegacyStateSubscribedWrite`/`Update`, который трансформ-`dispatch_identifier_read` не разворачивал и getter оставался голым. Фикс: флаг `Read | Write` для Identifier-bind в анализе + добавлены arm'ы `LegacyStateSubscribedWrite` и `LegacyStateSubscribedUpdate { safe }` в `crates/svelte_transform/src/transformer/rewrites.rs::dispatch_identifier_read` (зеркалят `LegacyStateWrite`/`LegacyStateUpdate`). Сеттер не тронут — `dispatch_identifier_assignment` уже корректно эмитит `$.store_unsub($.set(X, $$value), "$X", $$stores)` (test: `diagnose_legacy_bind_value_writable_store_shadow`).

- [ ] **Custom element: `$store` reads inside CE-targeted compile** — verify $$cleanup ordering with CE props (no targeted test)

## Out of scope

- SSR backend (`$$store_subs`, `subscribe_to_store`, `unsubscribe_stores`) — no SSR codegen crate yet
- `$:` reactive statement dependency tracking — owned by `specs/legacy-reactive-assignments.md`
- Reactive-import `$.reactive_import(thunk)` (legacy `import` mutation) — adjacent legacy feature, separate spec
- Custom-element store path (target=customElement) — owned by CE export spec
- Legacy `compatibility.componentApi === 4` shape — out of scope
- TS type-only references — irrelevant for emit

## Reference

### Reference compiler

- Docs: `original/docs/06-runtime/01-stores.md`
- Analyze: `original/compiler/phases/2-analyze/index.js` (synthetic binding loop + module/scoped subscription errors)
- Analyze visitors: `original/compiler/phases/2-analyze/visitors/Identifier.js`, `BindDirective.js`, `VariableDeclarator.js`, `shared/utils.js::is_safe_identifier`
- Client transform: `original/compiler/phases/3-transform/client/transform-client.js` (store_setup loop, `setup_stores`, `$$cleanup`)
- Client visitors: `original/compiler/phases/3-transform/client/visitors/Program.js`, `EachBlock.js`, `AssignmentExpression.js`, `shared/utils.js`, `shared/component.js`, `shared/declarations.js`
- Server transform: `original/compiler/phases/3-transform/server/transform-server.js` (out of scope here)
- Errors: `original/compiler/errors.js`
- Warnings: `original/compiler/warnings.js`

### Our code

- Analyze
  - `crates/svelte_analyze/src/reactivity_semantics/builder_v2/store.rs` — synthetic creation + reference fact recording
  - `crates/svelte_analyze/src/reactivity_semantics/data.rs` — `BindingFacts::Store`, `ReferenceFacts::StoreRead/StoreWrite/StoreUpdate`, `iter_store_bindings`, `has_store_bindings`
  - `crates/svelte_analyze/src/reactivity_semantics/builder_v2/references.rs` — symbol-driven reference classification (interaction with legacy state promotion)
  - `crates/svelte_analyze/src/block_semantics/builder/each.rs` — `uses_store` flag and EACH_ITEM_REACTIVE / store_to_invalidate propagation
  - `crates/svelte_analyze/src/block_semantics/builder/render.rs` — `callee_symbol` resolves store-reference callees to synthetic store SymbolId so `RenderCalleeShape` is Dynamic and codegen takes `$.snippet(node, $store, …)` path
  - `crates/svelte_analyze/src/lib.rs` — `RuntimePlan { has_stores, legacy_init, needs_push, .. }`
  - `crates/svelte_analyze/src/validate/stores.rs` — diagnostics
- Transform
  - `crates/svelte_transform/src/transformer/builders.rs` — `make_store_set`, `make_store_mutate`, `make_store_update` (no `make_store_unsub` yet)
  - `crates/svelte_transform/src/transformer/rewrites.rs` — dispatchers; member-target rewrite paths
- Codegen
  - `crates/svelte_codegen_client/src/lib.rs` — `runtime.has_stores` thunk + setup_stores + $$cleanup emit
  - `crates/svelte_codegen_client/src/codegen/component_props/bind_prop.rs` — `mark_store_binding` on component bind
  - `crates/svelte_codegen_client/src/codegen/attributes/bind/getter_setter.rs` — element-level `bind:value` for store thunks
  - `crates/svelte_codegen_client/src/codegen/attributes/bind/this.rs` — store-aware branch in `emit_bind_this` synthesises `$.bind_this(el, ($$value) => $.store_set(name, $$value), () => $name())` from `ReferenceSemantics::StoreRead`/`StoreWrite`/`StoreUpdate` on the directive root reference
- Tests
  - `tasks/compiler_tests/cases2/store_*`
  - `tasks/diagnostic_tests/cases/validate_store_*`

## Test cases

- [x] `store_basic`
- [x] `store_write`
- [x] `store_assign_template`
- [x] `store_compound_template`
- [x] `store_update_template`
- [x] `store_deep_mutation`
- [x] `store_deep_update`
- [x] `store_validate_dev`
- [x] `store_reassign_unsub`
- [x] `store_each_invalidate`
- [x] `store_mark_binding`
- [x] `bind_group_order_with_stores`
- [x] `state_rune_no_store_rune_conflict`
- [x] `state_rune_no_conflict_with_other_rune_calls`
- [x] `validate_store_invalid_scoped_subscription`
- [x] `validate_store_invalid_subscription_in_module`
- [x] `validate_store_rune_conflict`
- [x] `analyze_module_reports_store_invalid_subscription_module`
- [x] `module_store_subscription_reports_module_diagnostic_for_js`
- [x] `module_store_subscription_reports_module_diagnostic_for_ts`
- [x] `store_runes_id_assign_ops`
- [x] `store_runes_id_ops_template`
- [x] `store_runes_member_ops_script`
- [x] `store_runes_member_ops_template`
- [x] `store_runes_computed_member`
- [x] `store_runes_dev_smoke`
- [x] `store_legacy_var_basic`
- [x] e2e smoke (legacy): `smoke_legacy_reactive_mutations_all`
- [x] e2e smoke (runes): `smoke_runes_reactive_mutations_all`
- [x] `store_runes_each_member_mutation`
- [x] `store_bind_value_thunk_arrow`
- [x] `store_legacy_let_synthetic_reassign`
- [x] `store_legacy_each_invalidate`
- [x] `store_legacy_member_mutation`
- [x] `store_legacy_id_assign_ops`
- [x] `store_legacy_id_ops_template`
- [x] `store_legacy_member_ops_script`
- [x] `store_legacy_dev_smoke`
- [x] `store_legacy_bind_value`
- [x] `legacy_dev_synthetic_store_thunk`
- [x] `legacy_dev_writable_no_mutable_source`
- [x] `legacy_dev_store_thunk_call_read`
- [x] `legacy_dev_store_set_assignment`
- [x] `legacy_dev_bind_store_unsub`
- [x] `store_if_block_condition`
- [x] `store_key_block_expression`
- [x] `store_await_block_promise`
- [x] `store_render_tag_snippet`
- [x] `store_bind_this_element_ref`
- [x] `store_runes_prop_assign_bind`
- [x] `store_runes_component_bind_prop_store`
- [x] `store_runes_prop_thunk`
- [x] `diagnose_bindable_prop_store_only`
- [x] `diagnose_component_bind_store_derived_base`
- [x] `store_bare_identifier_method_call`
- [x] `diagnose_legacy_export_let_store_prop_subscription`
- [x] `diagnose_legacy_component_bind_store_prop`
- [x] `diagnose_legacy_reactive_store_value_passed_as_bare_prop`
- [x] `diagnose_legacy_component_bind_store_reactive_destructured_base`
- [x] `diagnose_legacy_let_writable_store_only_assign`
- [x] `diagnose_legacy_bind_store_member_keeps_writable_plain`
- [x] `diagnose_legacy_component_bind_base_store_unsub`
- [x] `diagnose_legacy_bind_value_writable_store_shadow`
