# bind:*

## Current state
- **Working**: 49/49 use cases
- **Tests**: 97/97 green
- Last updated: 2026-05-23

## Source

User request: `$audit bind:*`

ROADMAP.md — Bindings

## Syntax variants

- `bind:name`
- `bind:name={identifier}`
- `bind:name={member.expression}`
- `bind:name={get, set}` function bindings without extra surrounding parentheses
- Element bindings on regular elements, `<svelte:window>`, `<svelte:document>`, `<svelte:element>`, and components (`bind:this`, component prop bindings)
- Group bindings inside `{#each}` and keyed `{#each}` blocks

## Use cases

- [x] `bind:value` on `<input>` and `<select>` including shorthand and function bindings
  Existing tests: `bind_directives`, `bind_directives_extended`, `bind_function_value`, `bind_select_value`
- [x] `<option value="...">` (StringAttribute) drops the literal `value=` from the template HTML and emits `option.value = option.__value = "<lit>"` JS initializer, matching reference `needs_special_value_handling`. Always-on, not gated on parent `<select bind:value>`. (test: `bind_select_static_option_value`)
- [x] `<option value={expr}>` (Expression): currently falls through the generic dynamic-attribute path; reference routes through `build_element_special_value_attribute` with the `__value` cache (init `{}`) + effect wrapping `option.value = (option.__value = expr) ?? ""`. (test: `option_expr_value`, S)
- [x] `<option value="prefix-{expr}">` (Concatenation): same routing but `is_defined` evaluates to true so no `??` wrap, and concat is interpolated via template literal. Multi-option coalescing into single `template_effect` is part of this slice. (test: `option_concat_value`, `option_expr_value_multi`)
- [x] `bind:value` on `<textarea>`
  Existing tests: `bind_textarea_value`, `textarea_child_value_dynamic`
- [x] `bind:checked`, `bind:group`, and `bind:files`
  Existing tests: `bind_directives_extended`, `bind_function_checked`, `bind_group_*`, `bind_files`, `push_binding_group_order`
- [x] `bind:group` with auto-subscribed stores in same component: the synthesized `const binding_group = [];` declaration must be emitted AFTER the store getter constants and `const [$$stores, $$cleanup] = $.setup_stores();` (reference order = `store_setup` → `store_init` → `group_binding_declarations`). Currently emitted before them. (test: `bind_group_order_with_stores`, S)
- [x] `bind:group` in a legacy component with `$:` reactive declarations: the synthesized `const binding_group = [];` declaration is emitted AFTER the `const <name> = $.mutable_source();` allocations that back legacy reactive bindings (order = `mutable_source_decls` → `group_binding_declarations` → `prop_decls` → `legacy_pre_effect`). Owned by transform `crates/svelte_transform/src/transformer/legacy_reactive.rs` — when `legacy_pre_effect` rewrites apply, the binding_group const block is materialized inside the script body between the implicit `mutable_source` decls and the slots; codegen `crates/svelte_codegen_client/src/lib.rs` skips its own top-of-body emit in that case to avoid duplication. Shared name helper `binding_group_name` lives in `svelte_analyze::types::data`. (test: `bind_group_order_with_legacy_reactive`)
- [x] Regular-element `bind:checked` targeting a `$bindable` prop source from `$props()` passes the prop accessor directly to `$.bind_checked(...)` instead of lowering through rune getter/setter closures (test: `props_bindable_checkbox_disabled_shorthand_ts`)
- [x] Contenteditable bindings: `bind:innerHTML`, `bind:innerText`, `bind:textContent`
  Existing tests: `bind_content_editable`, `bind_contenteditable_flag`, `bind_multiple_on_element`
- [x] Element size bindings: `bind:clientWidth`, `bind:clientHeight`, `bind:offsetWidth`, `bind:offsetHeight`
  Existing test: `bind_element_size`
- [x] Element size / resize-observer / focused / contenteditable / media bindings targeting a `$bindable` prop source (or legacy `export let`) must still go through the get/set transform — currently the `is_bindable_prop_source` short-circuit in `crates/svelte_transform/src/lib.rs::walk_attrs` skips ALL element binds whose expression resolves to a bindable prop, leaving the bare identifier in place. Only `bind:checked` survives via the dedicated `emit_bind_checked_shorthand` path; every other element-bind property panics in codegen with `bind without getter/setter must be bind:this`. Owner: transform — narrow the bindable-prop short-circuit to component binds, or extend codegen with bindable-prop shorthand paths for the affected element-bind properties. (test: `bind_element_size_bindable_prop_source`)
- [x] Resize observer bindings: `bind:contentRect`, `bind:contentBoxSize`, `bind:borderBoxSize`, `bind:devicePixelContentBoxSize`
  Existing tests: `bind_resize_observer`, `bind_resize_observer_border_box_size`, `bind_resize_observer_device_pixel_content_box_size`
- [x] `bind:this` on elements, components, `<svelte:element>`, and getter/setter sequence form
  Existing tests: `bind_this`, `bind_this_sequence`, `component_bind_this`, `component_bind_this_variants`, `svelte_element_bind`
- [x] `bind:this` on a regular element with children and a class/style directive must be emitted after the element's `$.reset(...)`
  Existing test: `bind_this_with_children_and_class_directive`
- [x] Media read/write bindings
  Existing tests: `bind_media_rw`, `bind_media_ro`, `bind_media_property`, `bind_img`
- [x] `<svelte:window>` and `<svelte:document>` bindings
  Existing tests: `svelte_window_bind_scroll`, `svelte_window_bind_size`, `svelte_window_reactive`, `svelte_window_bind_online`, `svelte_window_combined`, `svelte_document_bindings`, `svelte_document_combined`
- [x] `bind:focused`
  Existing test: `bind_focused`
- [x] Bind validation parity in analyze:
  `bind_invalid_name`, `bind_invalid_target`, `bind_invalid_expression`, `bind_invalid_parens`, `bind_invalid_value`, `bind_group_invalid_expression`, `bind_group_invalid_snippet_parameter`
- [x] Attribute validation coupled to bindings:
  `attribute_contenteditable_missing`, `attribute_contenteditable_dynamic`, `attribute_invalid_type`, `attribute_invalid_multiple`
- [x] Runes-mode validation for binding each-item arguments
  `each_item_invalid_assignment`
- [x] Warning parity for rest-pattern each bindings
  `bind_invalid_each_rest`
- [x] Dev-mode component prop bindings validate ownership via `$$ownership_validator.binding(...)` — analyze flag `output.needs_component_bind_ownership` raised on PropSource bind in dev+runes; codegen wraps component call with `$$ownership_validator.binding(name, Comp, source)` prefix inside a BlockStatement. (test: `bind_component_prop_dev_ownership`)
- [x] Dev-mode component prop bindings to non-bindable plain `prop` (not `$bindable`) also emit `$$ownership_validator.binding(...)`. Reference fires for both `bindable_prop` and plain `prop`. Already covered: analyze promotes non-bindable prop to `PropBindingKind::Source { bindable: false }` when used as bind target, so the existing `PropSource` mode path catches it. (test: `bind_component_plain_prop_dev_ownership`)
- [x] Dev-mode `<svelte:component>` prop bindings emit `$$ownership_validator.binding(name, intermediate_name, source)` using the synthesized intermediate ident inside the dynamic component callback, not the literal tag name. (test: `bind_dynamic_component_dev_ownership`)
- [x] Dev-mode `$$ownership_validator.binding(...)` honors `<!-- svelte-ignore ownership_invalid_binding -->` on the binding directive — when ignored, analyze sets `requires_ownership_emit = false` on the `ComponentPropKind::Bind` and codegen skips both the validator var decl and the binding stmt. (test: `bind_component_dev_ownership_ignore`)
- [x] Component prop bindings with explicit identifier source (`<Comp bind:value={foo}>` where local var `foo` ≠ prop name `value`) — analyze stores trimmed `expr_text` as `expr_name` for simple-identifier expressions and uses it for both binding-semantics lookup and codegen source ident. (test: `bind_component_explicit_source`)
- [x] Component prop binding with member-expression source (`<Comp bind:value={store.inner.value}>`) emits the full path inside both `get value()` and `set value($$value)` instead of using the prop name. Owned by transform: `bind_expr_handles` loop branches on `AttributeSemantics::ComponentBind` and lowers the get/set pair into a synthetic `ObjectExpression { get name() {...}, set name($$value) {...} }` (instead of the element-bind `SequenceExpression([thunk, arrow])`). Codegen splices the object's properties into the component props literal via `ObjProp::Raw`. Single shape works for both prod and dev (transform owns the dev-vs-prod function-shape choice). Tests: `component_bind_member_path`. Dev-mode `$.validate_binding(...)` emission for member-path component bind is a separate follow-up (test: `component_bind_member_path_dev`).
- [x] Dev-mode `$.validate_binding(<source>, <each_ids>, () => <object_path>, () => <leaf>, line, col)` is emitted before the component call for member-path component bindings (`<Comp bind:value={store.inner.value}>`) (test: `component_bind_member_path_dev`)
- [x] Component prop bind to a `$derived` identifier source (`<Child bind:value={derivedFlag}>` where `let derivedFlag = $derived(...)`) emits the writeback as `$.set(derivedFlag, $$value)` without the third `true` proxy flag. Owner: analyze 3.A.4 — `ComponentBindTarget` split into `Rune` (writable `$state` / `OptimizedRune`) and `RuneDerived` (read-only `$derived`); `derive_component_bind_target` routes `BindingSemantics::Derived` to `RuneDerived`. Codegen `bind_prop.rs::emit_bind_identifier` branches on `RuneDerived` and emits `$.set(source, $$value)` without `Arg::Bool(true)`; `bind_this.rs` collapses `Rune | RuneDerived` to the same `SignalShape::Rune`. (test: `diagnose_component_bind_derived_target_no_proxy_flag`)
- [x] Component prop binding with member-expression source whose root is a `$bindable` prop (`let { store = $bindable() } = $props(); <Comp bind:value={store.inner.value}>`) rewrites the root identifier to its accessor call in the synthetic `get` body — `return store().inner.value;`, matching the setter side which already wraps via `wrap_bindable_prop_source_mutation`. Owner: transform — `dispatch_identifier_read` in `crates/svelte_transform/src/transformer/rewrites.rs` gained an arm for `ReferenceSemantics::PropSourceMemberMutationRoot { bindable: true, .. }` guarded by `!self.in_bind_setter_traverse` that emits `make_thunk_call(name)`. Setter-side path remains unchanged: `dispatch_member_assignment` → `wrap_bindable_prop_source_mutation` already replaces the LHS root and wraps the assignment. (test: `component_bind_member_path_bindable_root`)
- [x] Function-binding form on a component prop (`<Comp bind:value={() => v, (n) => v = n} />`) lowers the user-supplied get/set pair into the synthetic `ObjectExpression { get value() { return bind_get(); }, set value($$value) { bind_set($$value); } }` and hoists `var bind_get = <get>; var bind_set = <set>;` aliases at the parent function's top. Owned by analyze + codegen: a 2-element `SequenceExpression` bind expression on a component classifies as `ComponentBindKind::FunctionPair`; codegen reserves fresh `bind_get`/`bind_set` idents via `IdentGen`, drains them via `ComponentPropsOutput::bind_init_stmts` into `state.init`, and emits the getter/setter pair through `ObjProp::Getter`/`ObjProp::Setter`. (test: `component_bind_function`)
- [x] Function-binding form on a component prop preserves the **source position** of the synthetic `get/set <prop>` pair relative to sibling props in the literal — `<Comp bind:value={() => v, set} label="x" id="y" />` emits `{ get value, set value, label, id }`. Single-identifier `bind:value={v}` keeps the existing "deferred to end" placement (covered by `component_bind_prop_order`); only the function-pair (`SequenceExpression`) form is in-place. Owned by codegen `dispatch_component_bind` in `crates/svelte_codegen_client/src/codegen/component_props/dispatch.rs`: `ComponentBindKind::FunctionPair` appends through `out.items` directly, mirroring reference `push_prop(get); push_prop(set);` for `SequenceExpression` bind expressions, while ident/expression/store bind kinds still go through `out.deferred_items`. (test: `diagnose_component_bind_function_props_position`)
- [x] `bind_get` / `bind_set` aliases for a component function-binding are emitted **after** the parent fragment's anchor declaration (`var node = $.child(div);`), not before. Owned by codegen `containers/component.rs`: bind-init and event-init drain into `state.init` after `direct_anchor_expr` (static-component path) and after `comment_anchor_node_name` (`emit_dynamic_component`). Drain order: bind-init → event-init → component call. (test: `component_bind_function_anchor_order`)
- [x] Multiple `bind:group` directives with distinct group identities allocate distinct `binding_group_N` const declarations (`binding_group`, `binding_group_1`, …) at the top of the component body — one per group key (bound symbols + parent-each chain). Owned by analyze 3.A.4 + 3.B `BindSemanticsData`: `attribute_semantics::builder` walks both element and component bind directives, assigning stable group ids via `BindingGroupTable` (keyed by `references: SmallVec<SymbolId>` + `parent_each_blocks`); ids land in `BindSemanticsData::binding_group_id_by_attr` and on `ElementBindSemantics::group_id`. Codegen `lib.rs` iterates `CodegenView::binding_group_count()` to emit one `const binding_group_N = []` per group; `emit_bind_group` formats the per-attr name via `binding_group_name(id)`. (test: `component_bind_group_multiple_targets`)
- [x] `bind:group` on a component (`<Child bind:group={value}>`) synthesizes the top-level `const binding_group = [];` declaration even when no element-level `bind:group` is present. Owned by analyze: `BindSemanticsData::any_bind_group: bool` is raised by `ElementFlagsVisitor` for both regular elements (`visit_element`) and components (`visit_component_node` / `visit_svelte_component_legacy`) whenever a `BindDirective` named `group` is attached. Codegen reads `CodegenView::has_any_bind_group()` at the top of `compile_component`; the previous `EmitState::needs_binding_group` codegen-side flag was removed. (test: `diagnose_component_bind_group_emits_array`)
- [x] Element `bind:value` on a `<textarea>`/`<input>` whose source is a legacy `export let` prop (or runes `$bindable`) lowers to `$.bind_value(el, value)` through the bindable-prop shorthand path (test: `diagnose_legacy_bind_value_textarea_export_let_prop`)
- [x] Element `bind:group={value}` on a legacy `export let value` (component without runes) panics in codegen with `bind without getter/setter must be bind:this`. Owned by codegen `emit_bind_bindable_prop_shorthand` in `crates/svelte_codegen_client/src/codegen/attributes/bind/mod.rs`: the missing `ElementBindPropertyKind::Group` arm passes `var_alloc` directly as both the getter and setter when no `group_value_attr` is present, and as `() => var_alloc()` when `group_value_attr` is set so `emit_bind_group` can splice the val_stmt into the arrow body. Both delegate to `emit_bind_group`, producing the `$.bind_group(binding_group, [...], el, value, value)` or `$.bind_group(binding_group, [...], el, () => …, value)` shape the reference compiler emits for bindable-prop sources. (test: `diagnose_legacy_bind_group_radio_export_let_prop`)
- [x] `bind:group` inside a legacy `{#each}` with a sibling `value={item.member}` attribute placed before the bind directive keeps the value expression available for the `bind:group` val_stmt (test: `bind_group_value_attr_before_bind`)
- [x] Legacy `{#each items as item}` member-read of `item.member` inside `$.template_effect` for the `bind:group` value cache wraps as `($.get(item), $.untrack(() => $.get(item).member))` to keep the member read outside the template effect's reactive dependency set while still subscribing to `item` once (test: `bind_group_each_legacy_item_member_untrack`)
- [x] Dev-mode element bind helpers (`$.bind_value`, `$.bind_checked`, `$.bind_group`, `$.bind_select_value`, `$.bind_content_editable`, `$.bind_volume`, `$.bind_paused`, `$.bind_element_size`) take named `function get() {...}` / `function set($$value) {...}` declarations as get/set callbacks instead of arrow expressions. Implemented at shorthand bind lowering in `transform/template_entry.rs`: in dev mode synthesize `named_function_expr("get", …)` / `named_function_expr("set", …)` instead of `b.thunk` / `b.arrow_expr`. (test: `bind_value_dev_named_fns`)
- [x] Component `bind:value` against a legacy implicit reactive declaration target (`$: value = expr;` in non-runes mode) compiles — analyze treats `LegacyState` and `LegacyBindableProp` as writable bind targets alongside `State` / `Prop` in `validate_bind_identifier_value` (test: `diagnose_legacy_bind_value_on_implicit_reactive_declaration`)
- [x] Legacy `bind:this={refs[item.key]}` on an element inside `{#each items as item}` where `refs` is a top-level legacy `let` (mutable_source) emits the per-item form: `$.bind_this(div, ($$value, item) => $.mutate(refs, $.get(refs)[item.key] = $$value), (item) => $.get(refs)?.[item.key], () => [$.get(item)])` — the setter routes through `$.mutate` on the reactive container, the getter uses optional-chaining on `$.get(refs)`, and the trailing dependency thunk threads the each-item id into both callbacks. Currently emits the plain `($$value) => refs[item.key] = $$value`, `() => refs[item.key]` form with no `$.mutate`, no optional chain, and no item dep array. Test: `diagnose_legacy_each_bind_this_indexed_reactive`.
- [x] Legacy `bind:this={refs[idx]}` on an element inside `{#each items as item, idx}` where `idx` is the each-block index identifier emits the dependency thunk with the bare local: `() => [idx]`, not `() => [$.get(idx)]`. The each-block index is a plain JS parameter of the render callback and never carries a `mutable_source`; the inner setter/getter closures already shadow `idx` correctly, only the trailing dep thunk needed routing through the existing reactive-dep dispatcher. Owner: 4 codegen — `try_emit_bind_this_each_reactive_member` in `crates/svelte_codegen_client/src/codegen/attributes/bind/this.rs` builds the dep-array per symbol through `build_reactive_dep_expr_legacy` in `expr.rs`, which already maps `Contextual(EachIndex(Direct))` → bare ident and `Contextual(*(Signal))` → `$.get(name)`. Test: `diagnose_legacy_each_bind_this_indexed_by_index_variable`.
- [x] Component prop `bind:<name>={item.member}` inside `{#if item.member}` nested in a legacy `{#each $store as item (item.id)}` emits the getter `return $.get(item).<member>;`, not bare `return item.<member>;`. Setter side was already correct via `rewrite_each_item_member_store_invalidate_assignment` / `rewrite_legacy_each_item_member_assignment`; only the getter-side read was unwired. Owner: transform — `dispatch_identifier_read` in `crates/svelte_transform/src/transformer/rewrites.rs` gained an arm covering both `ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }` and `ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }` guarded by `!self.in_bind_setter_traverse`, emitting `make_rune_get(name)`. Family-wide: same arm closes `bind:value`, `bind:checked`, `bind:group` (all component-prop variants). Tests: `diagnose_legacy_each_store_bind_value_item_member`, `diagnose_legacy_each_store_bind_checked_item_member`, `diagnose_legacy_each_store_bind_group_item_member`.
- [x] Element `bind:value={item.member}` on `<input>` inside legacy `{#each $store as item (item.id)}` emits `$.bind_value(input, () => $.get(item).<member>, ($$value) => ($.get(item).<member> = $$value, $.invalidate_inner_signals(() => $store()), $.invalidate_store($$stores, "$store")))` — getter path closed by the same `dispatch_identifier_read` arm covering `LegacyEachItemMemberMutationRoot` / `EachItemMemberMutationStoreInvalidate` that the component-prop variants use; element-bind shares the same get/set traverse. (test: `diagnose_legacy_each_store_bind_value_element_item_member`)
- [x] Element `bind:checked={item.member}` on `<input type="checkbox">` inside legacy `{#each $store as item (item.id)}` — same coverage; `$.bind_checked` getter/setter shapes share the same lowering path. (test: `diagnose_legacy_each_store_bind_checked_element_item_member`)

## Reference

- `reference/compiler/phases/bindings.js` — canonical binding property matrix
- `reference/compiler/phases/2-analyze/visitors/BindDirective.js` — analyzer validation and group-binding metadata rules
- `reference/compiler/errors.js` — bind and attribute diagnostic definitions
- `reference/compiler/warnings.js` — `bind_invalid_each_rest`
- `reference/compiler/phases/3-transform/client/visitors/BindDirective.js` — reference client transform surface
- `reference/compiler/phases/3-transform/client/visitors/shared/component.js` — `$$ownership_validator.binding(...)` for component bindings
- `crates/svelte_parser/src/scanner/mod.rs` — parser support for `BindDirective`
- `crates/svelte_analyze/src/passes/template_semantic.rs` — bind expressions participate in template semantic analysis
- `crates/svelte_analyze/src/passes/bind_semantics.rs` — bind/group metadata currently precomputed for codegen
- `crates/svelte_analyze/src/tests.rs` — analyzer validation coverage, including ignored gaps
- `crates/svelte_codegen_client/src/template/bind.rs` — regular element bind codegen
- `crates/svelte_codegen_client/src/template/component.rs` — component `bind:this` and prop binding codegen
- `crates/svelte_codegen_client/src/template/svelte_window.rs` — `<svelte:window>` binding codegen
- `crates/svelte_codegen_client/src/template/svelte_document.rs` — `<svelte:document>` binding codegen
- `tasks/compiler_tests/cases2/` — current positive coverage for bind codegen parity

## Test cases

- [x] `bind_content_editable`
- [x] `bind_contenteditable_flag`
- [x] `bind_directives`
- [x] `bind_directives_extended`
- [x] `bind_element_size`
- [x] `bind_element_size_bindable_prop_source`
- [x] `bind_files`
- [x] `bind_focused`
- [x] `bind_function_checked`
- [x] `bind_function_value`
- [x] `bind_group_each`
- [x] `bind_group_each_var`
- [x] `bind_group_each_var_keyed`
- [x] `bind_group_keyed_each`
- [x] `bind_group_nested_each`
- [x] `bind_group_radio_basic`
- [x] `bind_group_value_attr`
- [x] `bind_img`
- [x] `bind_media_property`
- [x] `bind_media_ro`
- [x] `bind_media_rw`
- [x] `bind_multiple_on_element`
- [x] `bind_property`
- [x] `bind_resize_observer`
- [x] `bind_resize_observer_border_box_size`
- [x] `bind_resize_observer_device_pixel_content_box_size`
- [x] `bind_select_value`
- [x] `bind_textarea_value`
- [x] `bind_this`
- [x] `bind_this_sequence`
- [x] `bind_this_with_children_and_class_directive`
- [x] `bind_use_deferral`
- [x] `component_bind_prop_forward`
- [x] `component_bind_this`
- [x] `component_bind_this_variants`
- [x] `push_binding_group_order`
- [x] `bind_group_order_with_stores`
- [x] `bind_group_order_with_legacy_reactive`
- [x] `diagnose_component_bind_group_emits_array`
- [x] `component_bind_group_multiple_targets`
- [x] `diagnose_legacy_bind_group_radio_export_let_prop`
- [x] `diagnose_legacy_bind_value_on_implicit_reactive_declaration`
- [x] `diagnose_legacy_bind_value_textarea_export_let_prop`
- [x] `bind_group_value_attr_before_bind`
- [x] `bind_group_each_legacy_item_member_untrack`
- [x] `diagnose_legacy_each_bind_this_indexed_reactive`
- [x] `diagnose_legacy_each_bind_this_indexed_by_index_variable`
- [x] `diagnose_legacy_each_store_bind_value_item_member`
- [x] `diagnose_legacy_each_store_bind_checked_item_member`
- [x] `diagnose_legacy_each_store_bind_group_item_member`
- [x] `diagnose_legacy_each_store_bind_value_element_item_member`
- [x] `diagnose_legacy_each_store_bind_checked_element_item_member`
- [x] `props_bindable_checkbox_disabled_shorthand_ts`
- [x] `svelte_document_bindings`
- [x] `svelte_element_bind`
- [x] `svelte_window_bind_online`
- [x] `svelte_window_bind_scroll`
- [x] `svelte_window_bind_size`
- [x] `textarea_child_value_dynamic`
- [x] `validate_bind_invalid_name`
- [x] `validate_bind_invalid_name_with_special_element_candidates`
- [x] `validate_bind_invalid_target`
- [x] `validate_bind_invalid_expression`
- [x] `validate_bind_invalid_parens`
- [x] `validate_bind_invalid_value`
- [x] `validate_bind_plain_let_is_valid`
- [x] `validate_bind_getter_setter_without_parens`
- [x] `validate_bind_group_invalid_expression`
- [x] `validate_bind_sequence_reports_all_relevant_errors`
- [x] `validate_bind_group_invalid_snippet_parameter`
- [x] `validate_bind_invalid_each_rest`
- [x] `validate_bind_checked_radio_target`
- [x] `validate_bind_files_wrong_input_type`
- [x] `validate_attribute_contenteditable_missing`
- [x] `validate_attribute_contenteditable_dynamic`
- [x] `validate_attribute_invalid_type`
- [x] `validate_attribute_invalid_multiple`
- [x] `validate_bind_member_expression_no_error`
- [x] `validate_bind_getter_setter_no_error`
- [x] `bind_select_static_option_value`
- [x] `option_expr_value`
- [x] `option_concat_value`
- [x] `option_expr_value_multi`
- [x] `bind_value_dev_named_fns`
- [x] `bind_component_prop_dev_ownership`
- [x] `bind_component_plain_prop_dev_ownership`
- [x] `bind_dynamic_component_dev_ownership`
- [x] `bind_component_dev_ownership_ignore`
- [x] `bind_component_explicit_source`
- [x] `component_bind_member_path`
- [x] `component_bind_member_path_bindable_root`
- [x] `component_bind_member_path_dev`
- [x] `diagnose_component_bind_derived_target_no_proxy_flag`
- [x] `component_bind_function`
- [x] `component_bind_function_anchor_order`
- [x] `diagnose_component_bind_function_props_position`
