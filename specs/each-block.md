# Each Block

## Current state
- **Working**: 40/41 use cases
- **Tests**: 54/55 green
- Last updated: 2026-05-20

## Source

- ROADMAP template item: `{#each}`
- Audit request: `/audit {#each}`

## Syntax variants

- `{#each expression as item}...{/each}`
- `{#each expression as item, index}...{/each}`
- `{#each expression as item (key)}...{/each}`
- `{#each expression as item, index (key)}...{/each}`
- `{#each expression as { id, ...rest }}...{/each}`
- `{#each expression as [id, ...rest]}...{/each}`
- `{#each expression}...{/each}`
- `{#each expression, index}...{/each}`
- `{#each expression as item}...{:else}...{/each}`
- `{#each await expression as item}...{/each}` under experimental async

## Use cases

- [x] Basic item iteration: `{#each items as item}`.
- [x] Item iteration with index: `{#each items as item, i}`.
- [x] Non-keyed each-block index identifier in interpolated text is NOT wrapped in `?? ""` (reference treats bare `index` as `is_defined`). Keyed-by-expression blocks (key reads non-index symbols) wrap the index in `$.get(i)` so the `?? ""` fallback still applies; keyed-by-index blocks (key is exactly the index identifier) leave the index as a plain local, same as unkeyed. (test: `each_index_text_no_coalesce`)
- [x] Keyed each blocks, including key expressions that reference the index.
- [x] Keyed each blocks where the key expression is exactly the index identifier should emit `$.index` and the same each flags as the reference compiler (test: `each_key_is_index_literal_diagnose`).
- [x] Key-is-item optimization in runes mode.
- [x] Destructured object and array patterns.
- [x] Destructured defaults inside each context.
- [x] Item-less each blocks: `{#each items}`.
- [x] Item-less each blocks with index: `{#each { length: 8 }, rank}`.
- [x] `{:else}` fallback blocks for empty collections.
- [x] Bind/group and bind:this interactions with parent each scopes.
- [x] `animate:` codegen flags for keyed each blocks that already satisfy placement constraints.
- [x] Diagnostic: keyed each without `as` should raise `each_key_without_as`.
- [x] Diagnostic: `animate:` outside a keyed each or on a non-sole child should raise `animation_invalid_placement`.
- [x] Diagnostic: `animate:` inside an unkeyed each should raise `animation_missing_key`.
- [x] Diagnostic: runes-mode reassignment or binding to an each item should raise `each_item_invalid_assignment`.
- [x] Inner-scope shadowing: when an each block's inner scope declares a binding that shadows an outer scope name, emit `$$index, $$array` as extra render-callback params (reference: `collection_id` logic in `EachBlock.js` lines 112–123 and 316–318). Runes-only: legacy `transitive_deps`/reassigned-item rewrites are tracked separately. (test: `each_inner_shadow`)
- [x] Parser support for item-less each blocks with index: `{#each expression, index}`. Compiler coverage exists via `each_block_no_item_with_index`; the stale ignored parser unit test should not keep the roadmap feature open.
- [x] Nested each callback params in runes mode remain plain identifiers in template-attribute expressions (no `$.get(...)` wrapping and no extra fallback coercion noise) when the collection expression is non-reactive literals. (test: `clock_svg_derived_onmount`)
- [x] Legacy-mode each-collection call expression that reads a reactive binding wraps the getter as `($.deep_read_state(<state-read>), $.untrack(() => <call>))` (test: `each_collection_call_reads_state`)
- [x] Legacy-mode each-collection member chain over an auto-subscribed store thunk (`{#each $store.member as item}`) wraps the iterable as `($store(), $.untrack(() => $store().member))` (test: `store_legacy_each_member_iterable`)
- [x] Nested each: inner each with non-shadowing destructure pattern (e.g. `{#each links as { name, href }}` inside `{#each groups as [group, links]}`) must NOT add `$$index, $$array` to the inner render-callback. Our codegen currently emits them, which also forces the outer destructure helper to rename `$$array` → `$$array_1`. Reference keeps `($$anchor, $$item)` and `$$array`. (test: `each_nested_array_destructure_no_inner_shadow`)
- [x] Legacy-mode each-collection member chain over a non-reactive top-level `const` (e.g. `{#each meta.items as item}` where `const meta = { items: [...] }`) wraps the iterable as `() => $.untrack(() => meta.items)`. Plain-identifier collections stay un-wrapped; only member-chain (and call) forms in HardLegacy mode need the wrap (SoftLegacy / runes mode stay un-wrapped). (test: `legacy_each_collection_member_const_wraps_untrack`)
- [x] Legacy-mode each-collection member chain over a reassigned `let` binding (mutable_source, e.g. `{#each modeData.data as item}` where `let modeData = ...` is later reassigned) wraps the iterable as `() => ($.get(modeData), $.untrack(() => $.get(modeData).data))` — the outer state read registers the each-block as a dependency of the reactive source. Currently emits only `() => $.untrack(() => $.get(modeData).data)`, breaking re-iteration on reassignment. (test: `legacy_each_collection_member_reactive_let_wraps_with_read`)
- [x] Legacy-mode shadowing each iterator combined with an array assignment pattern in the script (e.g. `function f(e) { [target] = e }` alongside `{#each items as item (item.id)}`) must rename the synthetic `$$array` callback parameter to `$$array_1`. Reference reserves the `$$array` name during analyze whenever the script contains an array assignment pattern, regardless of whether the destructure is later transformed; our codegen `gen_ident("$$array")` does not see the reservation and emits a bare `$$array`. (test: `each_legacy_shadow_with_script_array_assign`)
- [x] Keyed-by-index each block (`{#each ... as item, i (i)}`, key expression is exactly the index identifier) treats the index as a plain local in body references — matches reference behavior. Other keyed forms (key expression references the index but is not just the identifier, e.g. `(item.id)` with body reads of `i`) keep the reactive `$.get(i)` wrap, matching reference. (test: `each_keyed_index_plain_in_body`)
- [x] Legacy-mode each-collection member chain over an `export let` prop (e.g. `{#each item.list as entry}` where `item` is a legacy prop accessed via `$.prop`) wraps the iterable as `() => ($.deep_read_state(item()), $.untrack(() => item().list))`. Same trigger also forces the component to emit the legacy boot scaffolding (`$.push($$props, false)` / `$.init()` / `$.pop()`) that is otherwise omitted for plain prop reads. (test: `diagnose_legacy_each_collection_member_export_let_prop_wraps`)
- [x] Legacy-mode each-collection member chain whose root is an imported module-scope binding (e.g. `{#each LINKS.list as link}` where `LINKS` comes from `import { LINKS } from './links.js'`) wraps the iterable as `() => ($.deep_read_state(LINKS), $.untrack(() => LINKS.list))`. Classifier looks up the root symbol's `BindingSemantics::MaybeReactive` to pick the variant; codegen emits `$.deep_read_state` with the bare identifier (no getter call). (test: `diagnose_legacy_each_collection_member_imported_wraps_with_read`)
- [x] Legacy-mode each-collection that is a bare call expression over an imported module-scope binding (e.g. `{#each items() as item}` where `items` is `import { items } from './data'`) wraps the iterable as `() => ($.deep_read_state(items), $.untrack(items))` — `$.deep_read_state` on the bare callee, `$.untrack` taking the callee directly (no IIFE around the call). Trigger fires in HardLegacy mode (component has at least one `export let`); same trigger also forces emission of the legacy boot scaffolding `$.push($$props, false)` / `$.init()` / `$.pop()`. (test: `diagnose_legacy_each_call_imported_wraps`)
- [x] Legacy-mode each-collection whose root expression is a `NewExpression` (e.g. `{#each new Array(4).fill(null) as _, i}`) wraps the iterable as `() => $.untrack(() => new Array(4).fill(null))`. Same trigger also forces emission of the legacy boot scaffolding `$.push($$props, false)` / `$.init()` / `$.pop()`, mirroring the existing imported-call and `export let` member-chain rules. Today our analyzer classifies the collection as `EachCollectionKind::Regular` (no `LegacyNew…` variant) so codegen emits the bare expression with no untrack wrap and `OutputPlanData::needs_context` stays `false`. Layer: 3.A.5 `BlockSemantics` (new `EachCollectionKind` variant) + 3.A.2 `ReactivitySemantics` / `NeedsContextVisitor` parity for template-side `NewExpression` roots in each-collection (script-body visitor already flips `needs_context` on `visit_new_expression`). Test: `diagnose_legacy_each_collection_new_expression_wraps`.
- [x] Legacy-mode each-collection whose root expression is a `NewExpression` whose arguments read a reactive binding (e.g. `{#each new Array(size).fill(null) as _, i}` where `size` is an `export let` prop) wraps the iterable as `() => ($.deep_read_state(size()), $.untrack(() => new Array(size()).fill(null)))` — the outer `$.deep_read_state` collector registers each-block as a dependency of the prop so re-render fires when the prop changes. `EachCollectionKind::LegacyNewExpression` carries `deep_read_symbols` (parity with `LegacyCallReadsState`); codegen `each_block.rs` emits the sequence wrap when non-empty, bare `$.untrack` otherwise. Test: `diagnose_legacy_each_collection_new_expression_prop_arg_wraps`.
- [x] Legacy-mode each-collection member chain whose root identifier is an outer-each iteration binding (e.g. `{#each item.kids as kid}` inside `{#each items as item}` with `<svelte:options runes={false}/>` plus an `export let`) wraps the iterable as `() => ($.get(item), $.untrack(() => $.get(item).kids))`. The outer each-item is a reactive mutable-source binding in HardLegacy mode, so it must be threaded as a dependency identical to the reassigned-`let` rule on line 52. Today the classifier in `crates/svelte_analyze/src/block_semantics/builder/each.rs` does not recognise an outer-each binding as a `deep_read_symbols` source, so codegen emits only `() => $.untrack(() => $.get(item).kids)` — the inner each-block never registers as a dependency of the outer iteration item, breaking re-iteration when the outer item changes. Layer: 3.A.5 `BlockSemantics` (classifier). Test: `diagnose_legacy_each_collection_member_outer_each_item_wraps`.
- [x] Legacy-mode each-collection that is a bare call expression over a non-reactive local function (e.g. `{#each items() as item}` where `function items() { ... }` lives in the same script) wraps the iterable as `() => $.untrack(items)` — bare `$.untrack` of the callee identifier, no `$.deep_read_state` collector (callee is `BindingSemantics::Normal`). Today `collection_kind_of` in `crates/svelte_analyze/src/block_semantics/builder/each.rs` checks `LegacyCallReadsState` (empty `deep_read_symbols` → skipped) and `LegacyCallReadsMaybeReactive` (callee not `MaybeReactive` → skipped) and falls through to `EachCollectionKind::Regular`, so codegen emits the bare thunk `items` with no `$.untrack` wrap. Need a new `EachCollectionKind::LegacyCall { callee_sym }` variant (or extension of an existing one) gated on `HardLegacy` + bare-call form, with codegen path `b.thunk(b.call_expr("$.untrack", [Arg::Ident(callee)]))`. Layer: 3.A.5 `BlockSemantics` (classifier) + `crates/svelte_codegen_client/src/codegen/blocks/each_block.rs` for the new arm. Test: `diagnose_legacy_each_call_local_fn_wraps`.
- [x] Legacy-mode each-collection that is a call expression over an imported callee with reactive arguments (e.g. `{#each pick(kind, $count) as item}` where `pick` is `import { pick } from './pick'`, `kind` is `export let kind`, `count` is a store) wraps the iterable as `() => ($.deep_read_state(pick), $.deep_read_state(kind()), $count(), $.untrack(() => pick(kind(), $count())))` — the dependency sequence must include the imported callee (`$.deep_read_state` on bare identifier), every reactive arg root (`$.deep_read_state(prop())` for `export let`, raw `$store()` call for auto-subscribed stores), then the `$.untrack` wrap. Same trigger also forces the legacy boot scaffolding (`$.push($$props, false)` / `$.init()` / `$.pop()`). Today the classifier in `crates/svelte_analyze/src/block_semantics/builder/each.rs` only collects deep-read symbols from the callee for the bare-`items()` form (spec line 57) and from a single state-read root, not from a multi-arg CallExpression mixing imported callee + prop-call args + store-call args. Layer: 3.A.5 `BlockSemantics` (classifier — extend `LegacyCallReadsState` / `LegacyCallReadsMaybeReactive` to walk all argument roots and collect store-subscription identifiers alongside `deep_read_symbols`) + `crates/svelte_codegen_client/src/codegen/blocks/each_block.rs` (emit raw `$store()` call entries inline in the sequence, not wrapped in `$.deep_read_state`). Test: `diagnose_legacy_each_call_imported_args_wrap`.
- [x] Legacy-mode each-collection member chain with optional chaining over an auto-subscribed store (e.g. `{#each $store?.list as item}`) wraps the iterable identically to the non-optional sibling on line 49: `() => ($store(), $.untrack(() => $store()?.list))`. Layer: 3.A.5 `BlockSemantics`. Test: `diagnose_legacy_each_collection_store_member_optional_wraps`.
- [x] Member access on an auto-subscribed store whose underlying binding is an `import` / `prop` / `bindable_prop` / `rest_prop` in non-runes mode forces `needs_context = true` on the component, driving the `$$props` parameter and `$.push` / `$.init` / `$.pop` boot scaffolding (parity with reference `MemberExpression` analyze-visitor + `is_safe_identifier`). Layer: 3.A.2 `ReactivitySemantics`. Test: `diagnose_soft_legacy_each_store_member_emits_boot_scaffolding`.
- [x] An each-block whose iterable depends on an auto-subscribed store in non-runes mode classifies the item binding as `EachItemStrategy::Signal`, emitting `$.get(item)` in the body (parity with HardLegacy). Layer: 3.A.2 `ReactivitySemantics` (`each_collection_has_external_deps` consults `ReferenceSemantics::StoreRead` for unresolved store references). Test: `diagnose_soft_legacy_each_store_item_reactive_read`.
- [x] Legacy-mode each-collection member chain over an auto-subscribed store whose underlying binding is a local `const` (e.g. `const { state } = makeAdapter(); {#each $state.items as item}`) emits the iterable as bare `() => $state().items` — no `($store(), $.untrack(...))` wrap. The wrap on line 49 only fires when the store binding itself is reactive (`import` / `prop` / `bindable_prop` / `rest_prop`); a plain local `const` (destructured from a function call or otherwise) has no extra dependency to register, so reference omits the sequence + untrack. Trigger fires in both auto-detected legacy (`runes: null` with `$store` syntax) and explicit `runes: false` modes when there is no `export let`. Today the classifier in `crates/svelte_analyze/src/block_semantics/builder/each.rs` flags any store-member each-collection as the wrapped variant regardless of the underlying store binding kind, so codegen emits `() => ($state(), $.untrack(() => $state().items))` for local-const stores too. Layer: 3.A.5 `BlockSemantics` (classifier — consult `BindingSemantics` of the store root identifier and skip the wrap variant when it is `Normal`). Test: `diagnose_legacy_each_collection_store_member_local_const_no_wrap`.
- [ ] Legacy-mode each-collection whose root expression is a `LogicalExpression` with a nullish/short-circuit fallback over a reactive member chain (e.g. `{#each modeData.data ?? [] as item}` where `let modeData = ...` is a reassigned `let` → `mutable_source`) wraps the iterable as `() => ($.get(modeData), $.untrack(() => $.get(modeData).data ?? []))` — same dependency-registration rule as line 52, just with a `??` (also `||` / `&&`) wrapper around the member chain. Today `collection_kind_of` in `crates/svelte_analyze/src/block_semantics/builder/each.rs` only peels `StaticMemberExpression` / `ComputedMemberExpression` / `ChainExpression` before bottoming out at the root identifier; a `LogicalExpression` short-circuits the loop into the `_ => break` arm and the function falls through to `EachCollectionKind::Regular`, so codegen emits the bare thunk `() => $.get(modeData).data ?? []` with no untrack wrap. Layer: 3.A.5 `BlockSemantics` (classifier — descend into `LogicalExpression.left`, optionally also `ConditionalExpression.consequent`, before identifier classification). Test: `diagnose_legacy_each_collection_member_nullish_fallback_wraps_with_read`.
- [x] Rest element in each-header destructure pattern (`{#each items as { ...item }}`, `{#each items as [...item]}`, mixed `{ a, ...rest }`, mixed `[a, ...rest]`) emits a thunk binding for the rest symbol. Object pattern → `let <name> = () => $.exclude_from_object($.get($$item), [<own-keys>])`; the key list is the set of static keys of sibling properties (empty for pure `{ ...x }`). Array pattern → `let <name> = () => $.get($$array).slice(<offset>)` where offset is the count of non-rest elements before the rest; the parent `$.to_array($$item)` call drops the trailing count argument when a rest element is present. Body reads on the rest symbol use the `name()` form via `EachItemStrategy::Accessor`. Tests: `diagnose_each_rest_only_pattern_binding`, `each_destructured_obj_with_rest`, `each_destructured_array_with_rest`, `each_destructured_array_rest_only`.

## Out of scope

- Parser diagnostics for malformed `{#each expression, index (key)}`-style headers that are unreachable from currently accepted template syntax; these parser-strictness differences are not tracked as remaining roadmap work for `{#each}`

## Reference

- Reference docs:
  - `reference/docs/03-template-syntax/03-each.md`
- Reference compiler:
  - `reference/compiler/phases/1-parse/state/tag.js`
  - `reference/compiler/phases/2-analyze/visitors/EachBlock.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/utils.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/element.js`
  - `reference/compiler/phases/3-transform/client/visitors/EachBlock.js`
- Rust implementation:
  - `crates/svelte_parser/src/scanner/mod.rs`
  - `crates/svelte_parser/src/handlers.rs`
  - `crates/svelte_parser/src/walk_js.rs`
  - `crates/svelte_analyze/src/passes/template_scoping.rs`
  - `crates/svelte_analyze/src/passes/template_semantic.rs`
  - `crates/svelte_analyze/src/passes/template_side_tables.rs`
  - `crates/svelte_analyze/src/passes/collect_symbols.rs`
  - `crates/svelte_analyze/src/passes/bind_semantics.rs`
  - `crates/svelte_analyze/src/validate/mod.rs`
  - `crates/svelte_codegen_client/src/template/each_block.rs`

## Test cases

- [x] `each_block`
- [x] `each_keyed_index`
- [x] `each_key_uses_index`
- [x] `each_key_is_index_literal_diagnose`
- [x] `each_key_is_item`
- [x] `each_destructured_obj`
- [x] `each_destructured_array`
- [x] `each_destructured_default`
- [x] `each_keyed_destructure`
- [x] `each_block_no_item`
- [x] `each_block_no_item_multi`
- [x] `each_fallback`
- [x] `async_each_basic`
- [x] `animate_basic`
- [x] `animate_params`
- [x] `animate_dotted_name`
- [x] `animate_reactive_params`
- [x] `animate_blockers`
- [x] `animate_with_spread`
- [x] `each_inner_shadow`
- [x] `each_block_shadowing_does_not_mutate_rune`
- [x] `each_block_no_item_with_index`
- [x] `validate_each_animation_missing_key`
- [x] `validate_each_animation_invalid_placement`
- [x] `validate_each_item_invalid_assignment`
- [x] `validate_each_item_invalid_assignment_bind_identifier`
- [x] `validate_each_item_bind_member_expression_no_invalid_assignment`
- [x] `validate_each_item_invalid_assignment_array_destructure`
- [x] `validate_each_item_invalid_assignment_nested_object_destructure`
- [x] `validate_each_key_without_as`
- [x] `clock_svg_derived_onmount`
- [x] `each_collection_call_reads_state`
- [x] `store_legacy_each_member_iterable`
- [x] `each_nested_array_destructure_no_inner_shadow`
- [x] `each_legacy_shadow_with_script_array_assign`
- [x] `legacy_each_collection_member_const_wraps_untrack`
- [x] `each_keyed_index_plain_in_body`
- [x] `legacy_each_collection_member_reactive_let_wraps_with_read`
- [x] `diagnose_legacy_each_collection_member_export_let_prop_wraps`
- [x] `diagnose_legacy_each_collection_member_imported_wraps_with_read`
- [x] `diagnose_legacy_each_call_imported_wraps`
- [x] `diagnose_each_rest_only_pattern_binding`
- [ ] `diagnose_legacy_each_collection_member_nullish_fallback_wraps_with_read`
- [x] `diagnose_legacy_each_collection_new_expression_wraps`
- [x] `diagnose_legacy_each_collection_new_expression_prop_arg_wraps`
- [x] `diagnose_legacy_each_call_local_fn_wraps`
- [x] `each_destructured_obj_with_rest`
- [x] `each_destructured_array_with_rest`
- [x] `each_destructured_array_rest_only`
- [x] `diagnose_legacy_each_collection_member_outer_each_item_wraps`
- [x] `diagnose_legacy_each_call_imported_args_wrap`
- [x] `diagnose_legacy_each_collection_store_member_optional_wraps`
- [x] `diagnose_soft_legacy_each_store_member_emits_boot_scaffolding`
- [x] `diagnose_soft_legacy_each_store_item_reactive_read`
- [x] `diagnose_legacy_each_collection_store_member_local_const_no_wrap`
