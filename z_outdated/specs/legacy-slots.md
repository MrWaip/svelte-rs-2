# Legacy slots

## Current state
- **Working**: 52/52 use cases
- **Tests**: 77/77 green
- Last updated: 2026-05-25

## Source
- `ROADMAP.md` legacy item: `<slot>` + `let:` + `<svelte:fragment>` + `slot attribute`
- User request: `$audit <slot> + let: + <svelte:fragment> + slot attribute`

## Syntax variants
```svelte
<slot />
<slot name="footer" />
<slot>fallback</slot>
<slot name="footer">fallback</slot>
<slot item={entry} />
<slot {item} />
<slot {...props} />
<svelte:options customElement="my-element" /><slot />
<svelte:options customElement="my-element" /><slot name="actions" />
{#if $$slots.description}<slot name="description" />{/if}
<Component>default slot content</Component>
<Component let:item>default slot content</Component>
<Component let:item={processed}>default slot content</Component>
<Component><div slot="item">named slot content</div></Component>
<Component><div slot="item" let:item>{item.text}</div></Component>
<Component><svelte:fragment slot="item">named slot content</svelte:fragment></Component>
<Component><svelte:fragment slot="item" let:item>{item.text}</svelte:fragment></Component>
<Component><Child slot="footer" /></Component>
<Child slot="footer" let:item>{item}</Child>
```

## Use cases

- [x] Dedicated AST/parser infrastructure exists for legacy slot shapes instead of generic `Element`/attribute payloads
  - [x] `<slot>` is represented as a dedicated AST node at parse time instead of a generic `Element` (tests: `legacy_slot_element_converts_to_dedicated_node`, `slot_named_fallback`, `warn_slot_deprecated`)
  - [x] Analyze/codegen consume the dedicated `<slot>` AST node instead of re-discovering slot semantics from generic lowered `Element` assumptions (tests: `slot_element_legacy_root_fragment_uses_dedicated_lowered_item`, `legacy_slot_dev_mixed`, `warn_slot_deprecated`, `slot_named_fallback`)
  - [x] `<svelte:fragment>` is represented as a dedicated AST node at parse time instead of a generic `Element` (tests: `legacy_svelte_fragment_converts_to_dedicated_node`, `svelte_fragment_named_slot`)
  - [x] Analyze/codegen consume the dedicated `<svelte:fragment>` AST node instead of relying on generic lowered `Element` assumptions (tests: `component_named_slot_mapping_uses_svelte_fragment_legacy_wrapper_id`, `svelte_fragment_named_slot`)
  - [x] `let:` is represented as a dedicated AST directive at parse time instead of a generic attribute/directive payload (tests: `let_directive_legacy_without_expression`, `let_directive_legacy_with_expression`, `let_directive_legacy_converts_to_dedicated_attribute`)
- [x] Default component children lower to `children` plus `$$slots.default` for legacy child-content interop (tests: `component_children`, `component_element_children`)
- [x] Default `<slot>` lowers to `$.slot(..., "default", {}, fallback)` and keeps optional fallback content intact (test: warn_slot_deprecated)
- [x] Named `<slot name="...">` lowers correctly with fallback content (test: slot_named_fallback)
- [x] Direct child elements with `slot="..."` lower into parent `$$slots` entries (test: component_named_slot)
- [x] Direct child `<svelte:fragment slot="...">` lowers into parent `$$slots` entries without wrapper DOM (test: svelte_fragment_named_slot)
- [x] Child components with `slot="..."` participate in named-slot grouping instead of receiving a plain `slot` prop (tests: component_child_slot_attribute, svelte_self_slot)
- [x] Default-slot bindings remain scoped to the default slot and are not visible inside named-slot content, matching the Svelte 4 migration note (test: component_default_slot_bindings_do_not_leak_into_named_slot_scope)
- [x] `<slot>` emits slot props from attributes/spreads instead of always passing `{}` while excluding `name` from the props object, lowering spreads through `$.spread_props(...)`, and memoizing dynamic call-valued props through the legacy slot prelude when needed (tests: slot_props_default, slot_props_spread, slot_props_dynamic_state, slot_props_dynamic_call)
- [x] Parent default-slot `let:` directives lower to derived reads from `$$slotProps` inside the generated slot function, including alias form `let:item={processed}` (tests: component_default_slot_let, component_default_slot_let_alias)
- [x] Named-slot `let:` directives on direct child elements lower inside the generated named-slot function, including object destructuring and multiple `let:` directives on the same element (tests: component_named_slot_let_element, component_named_slot_let_element_destructure, component_named_slot_let_element_multiple)
- [x] Named-slot `let:` directives on `<svelte:fragment>` lower inside the generated named-slot function, including object destructuring (tests: component_named_slot_let_fragment, component_named_slot_let_fragment_destructure)
- [x] Direct `$$slots` reads lower through sanitized legacy slot bindings instead of unresolved raw identifiers
  - [x] Template direct `$$slots` reads lower through `$.sanitize_slots($$props)` so conditional checks like `$$slots.description` work in template code, including the reference compiler's untracked read wrapper (tests: `legacy_slots_if`, `legacy_slots_template_reads_require_sanitized_slots_binding`, `legacy_slot_elements_do_not_require_sanitized_slots_binding`)
  - [x] Instance-script direct `$$slots` reads inject `$.sanitize_slots($$props)` and keep the `$$props` function parameter when script reads are the only legacy special-variable consumer (tests: `legacy_slots_script_basic`, `legacy_slots_script_reads_require_sanitized_slots_binding`)
- [x] Custom-element `<slot>` and named `<slot name="...">` are lowered to CE slot calls and emitted in the wrapper slot-name array (test: custom_element_slots)
- [x] Non-custom-element legacy `<slot>` keeps runes-mode deprecation warning ownership while still lowering through the legacy runtime path (test: warn_slot_deprecated)
- [x] Element-child `slot="..."` diagnostics cover static-value, placement, duplicate-name, default-slot-conflict, and slotted-`{@const}` allowances (test: slots/slot_attribute_static_value_ok, slots/slot_attribute_invalid_expression_value, slots/slot_attribute_invalid_placement_root, slots/slot_attribute_invalid_placement_nested_inside_component, slots/slot_attribute_duplicate_reports_second_named_slot, slots/slot_default_duplicate_reports_implicit_default_content, slots/slot_default_duplicate_ignores_whitespace_and_other_named_slots, slots/const_tag_inside_slotted_element_is_allowed)
- [x] Duplicate/default slot-conflict diagnostics include child components with `slot="..."` instead of only element-like wrappers (tests: `slots/slot_attribute_duplicate_component_child_reports_second_named_slot`, `slots/slot_default_duplicate_component_child_reports_slotted_component_conflict`)
- [x] `<slot>` validation matches reference behavior for invalid `name`, reserved `name="default"`, invalid non-attribute directives, and slot/render conflicts
  - [x] `<slot name={expr}>` and other non-text `name` forms report `slot_element_invalid_name` (test: `slots/slot_element_invalid_name_dynamic`)
  - [x] `<slot name="default">` reports `slot_element_invalid_name_default` (test: `slots/slot_element_invalid_name_default`)
  - [x] `<slot>` rejects non-attribute directives other than spread and `let:` (test: `slots/slot_element_invalid_attribute_class`)
  - [x] `<slot>` and `{@render ...}` conflicts report `slot_snippet_conflict` (test: `slots/slot_snippet_conflict`)
- [x] `<svelte:fragment>` validation matches reference behavior for invalid placement and invalid attributes other than `slot` plus optional `let:`
  - [x] Root or otherwise non-component `<svelte:fragment>` reports `svelte_fragment_invalid_placement` (test: `slots/svelte_fragment_invalid_placement_root`)
  - [x] `<svelte:fragment>` rejects attributes other than `slot` plus optional `let:` (test: `slots/svelte_fragment_invalid_attribute_class`)
- [x] `let:` invalid-placement diagnostics match the reference owner matrix for default slots, named slots, and slotted child components
  - [x] `let:` on `<svelte:window>` reports `let_directive_invalid_placement` instead of a generic illegal-attribute diagnostic (test: `slots/let_directive_invalid_placement_svelte_window`)
  - [x] `let:` on `<svelte:body>` reports `let_directive_invalid_placement` instead of a generic illegal-attribute diagnostic (test: `slots/let_directive_invalid_placement_svelte_body`)
- [x] Components with both default-slot content and a named slot containing a dynamic expression codegen correctly: the default-slot traversal skips children carrying a `slot="..."` attribute so the named-slot arrow still owns their expressions (test: `diagnose_component_default_and_named_slot_expr`)
- [x] Child component used as a named-slot fill (`<Inner slot="..." x={expr} />`) keeps non-slot expression props classified as `ComponentProp::Expression` instead of falling through to `NonSpecial`, so codegen does not panic with "unsupported NonSpecial attribute on ComponentNode" (test: `diagnose_component_named_slot_child_with_expression_prop`)
- [x] `<slot name="X" slot="X" />` used as a named-slot fill inside a child component forwards the parent slot through a generated `$$slots.X` entry that calls `$.slot(node, $$props, "X", {}, null)`, instead of being dropped from the child component's call arguments (test: `legacy_slot_forward_named_into_child_component`)
- [x] `<slot>` placeholder node id is allocated in the outer template scope before the slot fallback's inner template scope, so `var node = $.child(parent); $.slot(node, ...)` keeps the unsuffixed id and the fallback body re-uses the counter from zero (test: `legacy_slot_fallback_if_sibling_node_naming`)
- [x] Child component used as a named-slot fill (`<Inner slot="image" />`) reserves a `root_N` template id in the outer template scope to match the reference compiler's numbering, so sibling named-slot template ids remain aligned with reference output (test: `diagnose_component_named_slot_child_inflates_template_root_ids`)
- [x] `<slot name="X" slot="X">` used as a named-slot forwarding fill (with or without fallback content) reserves a `root_N` template id in the outer template scope to match the reference compiler's numbering, so any inner-template ids inside the fallback body stay aligned with reference output (test: `diagnose_legacy_slot_forward_inflates_template_root_ids`)
- [x] Sibling named-slot fill bodies do not share `fragment_N` ident counter state with each other — when one named-slot fill is a component with default children that need a `$.comment()` anchor (e.g. `{#if}`), and a preceding sibling slot is also a component fill, the `fragment_N` counter in the second slot body must reset/scope per slot body. Layer: codegen — fragment-id generation scope leaks across sibling named-slot snippet functions on the same parent component. (test: `diagnose_fragment_id_in_sibling_named_slot_after_component`)
- [x] `<slot>` prop getters whose value reads a store subscription thunk lower through the same store-tracking comma-expression wrapper used for component props: getter body becomes `$store(), $.untrack(() => <expr>)` and `derived_safe_equal` getters for call-expression slot prop values wrap the inner expression as `($store(), $.untrack(() => <call>))` (test: `diagnose_legacy_slot_props_store_member`)
- [x] `let:<alias>` whose alias starts with `on` (e.g. `let:onClick`) does not trigger `attribute_invalid_event_handler` — the event-handler string-value check belongs only to DOM element string/boolean/concatenation attributes and must skip `LetDirectiveLegacy` (and other directives) (test: `component_let_directive_name_starts_with_on`)
- [x] Self-closing / content-less child elements carrying `slot="X"` (e.g. `<div slot="foo" />`) still register as named-slot fills in the parent component's `$$slots` map instead of being dropped during named-slot grouping. Layer: codegen — `is_slot_fragment_empty` для `Node::Element` смотрело на дочерний фрагмент элемента, тогда как сам элемент-носитель `slot="X"` всегда является контентом named-slot fill. (test: `diagnose_component_named_slot_empty_element_child_kept`)
- [x] Component callee identifier that resolves to a slot `let:` binding (`BindingSemantics::Contextual(ContextualBindingSemantics::LetDirective)`) — e.g. `<Outer let:value={Inner}><Inner /></Outer>` — must read the callee through `$.get(Inner)($$anchor, {})` instead of `Inner($$anchor, {})`. The `let:` alias lowers to `const Inner = $.derived_safe_equal(() => $$slotProps.value)` so a bare identifier read is stale. Reference rewrites every reactive identifier (including the callee) via `$.get`, while our codegen builds the component call from a raw `&str` callee at `crates/svelte_codegen_client/src/codegen/containers/component.rs` (~lines 188–196), bypassing the JS-AST rewriter that already handles `LetDirective` reads (`crates/svelte_transform/src/transformer/rewrites.rs::ContextualReadKind::LetDirective`). Layer: codegen — component callee emission must check the resolved binding kind and emit `$.get(callee)(...)` for `LetDirective` (mirrors the `HandlerEmit::WrappedInert` decision used for `on:` handlers backed by the same binding kind). Test: `diagnose_component_callee_from_slot_let`.
- [x] Component callee identifier whose `let:value` alias name collides with an identifier declared anywhere inside the `<script>` (including inside a nested function body — e.g. `const { default: Inner } = await import('./Inner.svelte')` inside an async helper) must still resolve to the slot's `LetDirective` binding and emit `$.get(Inner)($$anchor, {})`. Layer: codegen — `emit_component_impl` теперь резолвит callee через scope-chain `find_binding` от `fragment_scope_by_id(node_fragment(el_id))` вместо плоского `find_binding_in_any_scope`, так что `LetDirective`-биндинг в скоупе слот-fill всегда побеждает любой одноимённый script-биндинг (test: `diagnose_component_callee_from_slot_let_shadowed_by_script_binding`).
- [x] `<svelte:component slot="X" this={expr} />` as a direct child of a parent component registers as a named-slot fill in the parent's `legacy_slots` partition at parse time. `Node::SvelteComponentLegacy` is constructed directly at parse time (no `Element` → svelte-element conversion step), so the named-slot partitioner must list it among slot-fill carriers alongside `Element`, `ComponentNode`, `SvelteSelf`. Layer: parser — `Parser::slot_name_of` in `crates/svelte_parser/src/lib.rs` gains a `Node::SvelteComponentLegacy(c) => &c.attributes` arm. (test: `slot_on_svelte_component_legacy_is_bucketed_into_legacy_slots`)
- [x] Named-slot fill body whose only child is `<svelte:component>` lowers through the `$.comment()` envelope used at fragment-root for dynamic components: `var fragment_N = $.comment(); var node = $.first_child(fragment_N); $.component(node, …, …); $.append($$anchor, fragment_N);`. Layer: codegen — `SvelteComponentLegacy` is always dynamic, so `emit_slot_fragment_legacy_component_only_dont_use` in `crates/svelte_codegen_client/src/codegen/fragment/legacy_slot_fragment.rs` must drop it from the `append_inside = true` set, letting `FragmentAnchor::CallbackParam { append_inside: false }` drive `comment_anchor_node_name` into the envelope branch. (test: `diagnose_component_named_slot_svelte_component_child_kept`)
- [x] `<slot ... {ident} />` shorthand attribute whose identifier resolves to a top-level non-reactive `const` initialized with an arrow / function expression (e.g. `const onClose = () => goback(); <slot name="header" {onClose} />`) serializes as a plain shorthand entry `{ onClose }` in the `$.slot(node, $$props, "header", { onClose }, …)` props object — no `get onClose() { return onClose; }` getter. Same `ComponentPropMemo::Inline` answer that item 43 (`diagnose_component_onclick_const_arrow`) restored for child-component props must also reach `<slot>` element serialization. Today our compiler emits the getter for every named slot attribute regardless of value reactivity because the slot-prop classifier in `crates/svelte_codegen_client/src/codegen/containers/legacy_slot.rs` either re-derives memo locally or treats every attribute as `Derived`/`Getter`; reference inlines whenever `references_need_wrap` (`crates/svelte_analyze/src/attribute_semantics/builder/mod.rs`) reports the value as stable. Owning layer: 3.A.4 `AttributeSemantics` + codegen consumer — same fact already published for child-component props must drive the slot-prop emission branch. Test: `diagnose_slot_attribute_const_arrow_shorthand`.
- [x] `<svelte:fragment let:y>` inside a child component that is itself placed in a parent's named slot (`<Outer><Inner slot="action"><svelte:fragment let:y>…</svelte:fragment></Inner></Outer>`) must lower the inner component's call to `children: $.invalid_default_snippet, $$slots: { default: ($$anchor, $$slotProps) => { const y = $.derived_safe_equal(() => $$slotProps.y); … } }` — the same shape used when the `<svelte:fragment let:>` sits in a top-level child component. Today the named-slot grouping arrow inlines the fragment body directly as `children: ($$anchor, $$slotProps) => { … }` and reduces `$$slots: { default: true }`, dropping the `let:y` derivation. Layer: codegen (suspected) — child-component slot-fill lowering inside an outer `$$slots.<name>` arrow does not re-route `<svelte:fragment let:>` through the named-slot/default-slot function branch. Test: `diagnose_svelte_fragment_let_inside_named_slot_component`.
- [x] Legacy `let:value={[a, b, …]}` array-destructure aliases on a component slot fill must rewrite the dependency-tracking reads inside the body's `$.derived_safe_equal` comma-prefix the same way the `$.untrack(() => …)` body is rewritten — `$.deep_read_state($.get(value).<alias>)` rather than the unresolved `$.deep_read_state($.get(<alias>))`. The destructuring lowers to `const value = $.derived(() => { let [a, b, …] = $$slotProps.value; return { a, b, … }; })`, so the alias identifiers are no longer bound at the slot-body scope. Layer: analyze — `ContextualBindingSemantics` carries a dedicated `LetDirectiveCarrierMember { carrier_symbol }` variant for destructure aliases instead of collapsing `BindingFacts::CarrierAlias` into bare `LetDirective`; codegen `build_reactive_dep_expr_legacy` and `uses_deep_read_state` consume the new variant. Test: `diagnose_legacy_slot_let_array_destructure_dep_read`.
- [x] `<slot>` element carrying a `let:<alias>` directive lowers the alias to a `const <alias> = $.derived_safe_equal(() => $$slotProps.<alias>);` declaration emitted before the `$.slot(...)` call so the shorthand attribute `<slot ... {<alias>} />` (and any other reference to `<alias>` in sibling getters/props) resolves to the declared identifier. Today our codegen drops the alias binding entirely: `<slot name="cell" slot="cell" let:week {week} />` emits `$.slot(node, $$props, "cell", { get week() { return $.get(week); } }, null)` where `week` is never declared, producing a ReferenceError at runtime. Reference treats `let:` on a `<slot>` element the same way it treats `let:` on a child element inside a named-slot fill — the `$$slotProps` consumed by the `<slot>` is the *parent slot fill's* `$$slotProps`, so the alias is a forwarding read. Layer: codegen (`crates/svelte_codegen_client/src/codegen/containers/legacy_slot.rs`) — `emit_slot_element_legacy` теперь вызывает `emit_let_directive_legacy_stmts(el_id)` и эмитит результат во flat-позиции перед `$.slot(...)`, а `emit_let_directive_legacy_stmts` распознаёт `Node::SlotElementLegacy` наряду с `Element`/`SvelteFragmentLegacy`. (test: `diagnose_slot_element_let_directive_alias_in_named_slot_fill`)
- [x] `<slot title={expr}>` whose dynamic value is a non-call computed expression referencing a legacy reactive source (ternary, binary, unary on `export let` / `$:` derived) lowers to a plain `get title() { return <expr>; }` getter — codegen must NOT wrap the value in a `derived_safe_equal` IIFE prelude. `derived_safe_equal` for legacy `<slot>` outputs is reserved for call-expression values (where call identity differs across reads, see `slot_props_dynamic_call` and `diagnose_legacy_slot_props_store_member`). Layer: codegen (and/or 3.A.4 analyze) — `crates/svelte_codegen_client/src/codegen/containers/legacy_slot.rs` currently reuses `ComponentPropMemo::Derived` from `derive_component_prop_memo_for_expression` in `crates/svelte_analyze/src/attribute_semantics/builder/mod.rs`, which is calibrated for child-component props where `Derived` enforces prop-identity equality across renders; for `<slot>` outputs the consumer's getter already runs in its own reactive scope, so non-call dynamic values must stay inline inside the getter body. (test: `diagnose_legacy_slot_prop_conditional_no_derived_wrap`)
- [x] DOM element carrying a `class:`/`style:`/`bind:`/`on:`/`let:` directive whose local name collides with a real HTML attribute name (e.g. `<div class:slot={cond}>`, `<textarea bind:value={x}>foo</textarea>`) must NOT trigger attribute-keyed validators that key off plain attribute presence (`slot_attribute_invalid_placement`, `textarea_invalid_content`, etc.). Today `AttrIndex` indexes every `Attribute::name()` (including directives) into one name-space, so `has_attribute(el.id, "slot")` returns true for `class:slot={...}` and the placement check fires against a plain `<div>`. Sweep on `@ozon-ob/azkaban .../SidebarModal.svelte` reproduces with `<div class="header" class:slot={$$slots.header}>...</div>`. Layer: 3.B `ElementAnalysis` (`crates/svelte_analyze/src/types/data/attr_index.rs::attr_index_name`) — index only `StringAttribute`/`ExpressionAttribute`/`BooleanAttribute`/`ConcatenationAttribute` names; directives stay reachable only through their dedicated getters. Test: `diagnose_class_directive_named_like_slot_attribute_no_placement_error`.
- [x] Named-slot fill body that combines `let:<alias>` directives with one or more `{@const ...}` declarations must emit the `@const`-derived `$.derived_safe_equal(...)` bindings BEFORE the slot body's DOM root (`var div = root_N();`) and the `let:`-alias `const <alias> = $.derived_safe_equal(() => $$slotProps.<alias>);` declarations. Layer: codegen — `emit_single_slot_element` в `crates/svelte_codegen_client/src/codegen/fragment/legacy_slot_fragment.rs` теперь читает `legacy_slot_const_tag_end`, выставленный `emit_fragment` сразу после `bucket.const_tags` loop, и использует эту позицию (вместо `init_len_before`) для вставки DOM-root и `let:`-aliases — так const-tag derivations остаются в самом начале `state.init`. (test: `diagnose_legacy_slot_let_const_tag_ordering`)

- [x] `<slot>` legacy fill props with a non-simple stateful expression (`!Identifier && !MemberExpression`, `references_need_wrap == true`) — binary (`a + b`), unary (`!flag`), logical (`a && b`), conditional (`flag ? a : b`) — lower to a plain `get name() { return <expr>; }` getter with inline reactive reads, no `$.derived` wrap. The `ComponentPropCarrier::{Component, SlotLegacy}` split in `derive_component_prop_memo_for_expression` (`crates/svelte_analyze/src/attribute_semantics/builder/mod.rs:1161`–`1168`) is justified by reference: `<Component>` props go through `build_component` (`original/compiler/phases/3-transform/client/visitors/shared/component.js:188`) with `should_wrap_in_derived = !Identifier && !MemberExpression` → `$.derived`; `<slot>` props go through `SlotElement` (`original/compiler/phases/3-transform/client/visitors/SlotElement.js:36`) with `metadata.has_call || metadata.has_await` → inline getter. Earlier sibling-auditor verdict ("no `SlotLegacy` distinction in the reference") looked only at the `build_component` call-site and missed the separate `SlotElement` visitor; folding would regress test 47. (test: `diagnose_legacy_slot_prop_non_simple_stateful_shapes`)

## Out of scope

- Snippet interop beyond the legacy slot conflict diagnostics already referenced above
- SSR slot generation
- Shared legacy special-variable deconfliction tracked by `specs/legacy-export-let.md`

## Reference
### Svelte
- `original/docs/99-legacy/20-legacy-slots.md`
- `original/docs/99-legacy/21-legacy-$$slots.md`
- `original/docs/99-legacy/22-legacy-svelte-fragment.md`
- `original/docs/07-misc/06-v4-migration-guide.md`
- `original/docs/07-misc/07-v5-migration-guide.md`
- `original/docs/07-misc/04-custom-elements.md`
- `original/compiler/phases/1-parse/state/element.js`
- `original/compiler/utils/slot.js`
- `original/compiler/phases/2-analyze/visitors/shared/attribute.js`
- `original/compiler/phases/2-analyze/visitors/shared/component.js`
- `original/compiler/phases/2-analyze/visitors/SlotElement.js`
- `original/compiler/phases/2-analyze/visitors/Identifier.js`
- `original/compiler/phases/2-analyze/visitors/SvelteFragment.js`
- `original/compiler/phases/3-transform/client/visitors/Program.js`
- `original/compiler/phases/3-transform/client/visitors/Identifier.js`
- `original/compiler/phases/3-transform/client/visitors/SlotElement.js`
- `original/compiler/phases/3-transform/client/visitors/LetDirective.js`
- `original/compiler/phases/3-transform/client/visitors/shared/component.js`
- `original/compiler/phases/3-transform/server/visitors/shared/component.js`

### Our code
- `crates/svelte_ast/src/lib.rs`
- `crates/svelte_parser/src/attr_convert.rs`
- `crates/svelte_analyze/src/passes/lower.rs`
- `crates/svelte_analyze/src/passes/template_validation.rs`
- `crates/svelte_analyze/src/tests.rs`
- `crates/svelte_codegen_client/src/template/slot.rs`
- `crates/svelte_codegen_client/src/template/component.rs`
- `crates/svelte_codegen_client/src/lib.rs`
- `crates/svelte_codegen_client/src/custom_element.rs`
- `tasks/compiler_tests/test_v3.rs`
- `tasks/diagnostic_tests/test_diagnostics.rs`

## Test cases

- [x] warn_slot_deprecated
- [x] component_children
- [x] component_element_children
- [x] slot_named_fallback
- [x] component_named_slot
- [x] svelte_fragment_named_slot
- [x] slots/slot_attribute_static_value_ok
- [x] slots/slot_attribute_invalid_expression_value
- [x] slots/slot_attribute_invalid_placement_root
- [x] slots/slot_attribute_invalid_placement_nested_inside_component
- [x] slots/slot_attribute_duplicate_reports_second_named_slot
- [x] slots/slot_default_duplicate_reports_implicit_default_content
- [x] slots/slot_distinct_named_slots_do_not_conflict
- [x] slots/slot_default_duplicate_ignores_whitespace_and_other_named_slots
- [x] slots/const_tag_inside_slotted_element_is_allowed
- [x] legacy_slot_element_converts_to_dedicated_node
- [x] legacy_svelte_fragment_converts_to_dedicated_node
- [x] let_directive_legacy_without_expression
- [x] let_directive_legacy_with_expression
- [x] let_directive_legacy_converts_to_dedicated_attribute
- [x] slot_element_legacy_root_fragment_uses_dedicated_lowered_item
- [x] component_named_slot_mapping_uses_svelte_fragment_legacy_wrapper_id
- [x] legacy_slot_dev_mixed
- [x] component_default_slot_bindings_do_not_leak_into_named_slot_scope
- [x] slot_props_default
- [x] slot_props_spread
- [x] slot_props_dynamic_state
- [x] slot_props_dynamic_call
- [x] component_default_slot_let
- [x] component_default_slot_let_alias
- [x] component_named_slot_let_element
- [x] component_named_slot_let_element_destructure
- [x] component_named_slot_let_element_multiple
- [x] component_named_slot_let_fragment
- [x] component_named_slot_let_fragment_destructure
- [x] component_child_slot_attribute
- [x] svelte_self_slot
- [x] legacy_slots_if
- [x] legacy_slots_script_basic
- [x] custom_element_slots
- [x] legacy_slots_template_reads_require_sanitized_slots_binding
- [x] legacy_slot_elements_do_not_require_sanitized_slots_binding
- [x] legacy_slots_script_reads_require_sanitized_slots_binding
- [x] slots/slot_attribute_duplicate_component_child_reports_second_named_slot
- [x] slots/slot_default_duplicate_component_child_reports_slotted_component_conflict
- [x] slots/slot_element_invalid_name_dynamic
- [x] slots/slot_element_invalid_name_default
- [x] slots/slot_element_invalid_attribute_class
- [x] slots/slot_snippet_conflict
- [x] slots/svelte_fragment_invalid_placement_root
- [x] slots/svelte_fragment_invalid_attribute_class
- [x] slots/let_directive_invalid_placement_svelte_window
- [x] slots/let_directive_invalid_placement_svelte_body
- [x] diagnose_component_default_and_named_slot_expr
- [x] diagnose_component_named_slot_child_with_expression_prop
- [x] legacy_slot_forward_named_into_child_component
- [x] legacy_slot_fallback_if_sibling_node_naming
- [x] diagnose_component_named_slot_child_inflates_template_root_ids
- [x] diagnose_fragment_id_in_sibling_named_slot_after_component
- [x] diagnose_legacy_slot_forward_inflates_template_root_ids
- [x] diagnose_legacy_slot_props_store_member
- [x] component_let_directive_name_starts_with_on
- [x] diagnose_component_named_slot_empty_element_child_kept
- [x] diagnose_component_callee_from_slot_let
- [x] diagnose_component_callee_from_slot_let_shadowed_by_script_binding
- [x] slot_on_svelte_component_legacy_is_bucketed_into_legacy_slots
- [x] diagnose_component_named_slot_svelte_component_child_kept
- [x] diagnose_legacy_slot_prop_conditional_no_derived_wrap
- [x] diagnose_slot_attribute_const_arrow_shorthand
- [x] diagnose_legacy_slot_let_array_destructure_dep_read
- [x] diagnose_slot_element_let_directive_alias_in_named_slot_fill
- [x] diagnose_svelte_fragment_let_inside_named_slot_component
- [x] diagnose_class_directive_named_like_slot_attribute_no_placement_error
- [x] diagnose_legacy_slot_let_const_tag_ordering
- [x] diagnose_legacy_slot_prop_non_simple_stateful_shapes
