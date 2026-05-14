# ComponentNode

## Current state
- **Working**: 27/27 use cases
- **Tests**: 36/36 green
- Last updated: 2026-05-13

## Source

- User request: `/audit component`

## Syntax variants

- `<Component />`
- `<Component></Component>`
- `<Component foo="x" bar={expr} {...spread} />`
- `<Component bind:x={value} bind:this={ref} />`
- `<Component on:done={handler} />`
- `<Component>{@snippet children()}</Component>`
- `<foo.bar />`
- `<registry_name.Widget />`
- `<registry.Widget />`
- `<Derived_1 bind:this={refs[1]} />`
- `{#if cond}{@const Const_0 = Widget}<Const_0 bind:this={refs[0]} />{/if}`

## Use cases

- [x] Basic uppercase component tag lowers to `Component($$anchor, {})`
- [x] Non-self-closing component tag lowers the same as self-closing form
- [x] String, boolean, expression, shorthand, concatenation, and spread props preserve order (tests: `component_props`, `component_spread_props`)
- [x] `bind:this` on components lowers through `$.bind_this(...)` (tests: `component_bind_this`, `component_bind_this_variants`)
- [x] Non-`this` component bindings lower to getter/setter props (tests: `component_bind_prop_forward`)
- [x] Snippet children and snippet props lower correctly (tests: `component_snippet_prop`, `component_snippet_with_children`, `component_multiple_snippets`, `component_snippet_only`)
- [x] Complex expression props memoize when needed (tests: `component_prop_has_call`, `component_prop_has_call_multi`, `component_prop_has_call_mixed`, `component_prop_memo_state`)
- [x] String-template (concat) component prop with non-pure call expression memoizes the call into a derived and reads via `$.get` inside the getter, e.g. `<Comp title="{getName()} suffix" />` lowers to `{ let $0 = <derived_helper>(getName); Comp(..., { get title() { return `${$.get($0) ?? ""} suffix`; } }); }`. Helper is routed through `CodegenView::derived_helper()` (`$.derived` in runes, `$.derived_safe_equal` in SoftLegacy/HardLegacy). Test: `component_prop_concat_call_memo`.
- [x] String-template (concat) component prop where some Dynamic parts evaluate to known constants (`Evaluation::Known`) folds those parts into static text instead of memoizing them, e.g. `<Comp title="x{0}{getName()}" />` lowers to `let $0 = $.derived(getName); ... `${\`x0\`}${$.get($0) ?? ""}` `. Owned by analyzer: `derive_component_concat_semantics` publishes a per-part `plan: SmallVec<[ConcatPartEmit; 4]>` (`Static | Inline | HoistDerived`) that codegen dispatches over without re-probing. Per-part decision mirrors reference `Memoizer.add` + `should_wrap_in_derived` (any non-Identifier/StaticMember Dynamic part) so call expressions and reactive reads beside complex parts memoize uniformly. Test: `component_prop_concat_call_with_literal`.
- [x] Forwarding a top-level `const` whose initializer is not statically known (e.g. `const x = foo()`) emits a `get`-style component prop instead of an inline value (test: `component_prop_const_call_init_getter`)
- [x] Component prop expression that reads a `{@const}`-derived through a member access (`<Comp name={item.name}>` where `item` lowers to `$.derived(...)`) emits a `get name()` getter rather than an inline `name: $.get(item).name`. Owned by analyze: `references_need_wrap` reads `ConstBindingSemantics::ConstTag.reactive` (always `true` for `{@const}`) instead of treating all const-tag bindings as non-reactive. (test: `component_prop_const_tag_member`)
- [x] Same wrapping must apply when the `{@const}` initializer is a member/call expression read (e.g. `const store = getStore(); {@const a = store.sel}` followed by `<Comp foo={a.y} />`) — reference treats this as `has_state` because the init is impure. Fix: `compute_const_tag_reactivity` (`crates/svelte_analyze/src/reactivity_semantics/builder_v2/mod.rs`) flags `reactive=true` for const-tag bindings whose init expression is a `MemberExpression` / `CallExpression` / `NewExpression` / `TaggedTemplateExpression` (peeling TS `as` / `!` / `satisfies` and parens), in addition to the existing reactive-reference check. Test: `diagnose_component_prop_const_tag_member_init`.
- [x] Component prop that is a bare identifier or shorthand reference to a destructured `{@const}` binding whose RHS is a store autosubscription (`<Comp x={a} {b} />` after `{@const { a, b } = $store}`) emits getter wrappers (`get x() { return $.get(computed_const).a; } / get b() { return $.get(computed_const).b; }`) reading through the synthesized `computed_const` derived. Today our compiler emits inline `x: $.get(computed_const).a, b` (shorthand passthrough); reference always wraps. The `$state`-rooted RHS path already works — only the store-autosubscribe RHS leaves the destructured names tagged non-reactive. Layer: 3.A.4 `AttributeSemantics` (component prop memo), same fact as the member-access entry above. Test: `diagnose_component_prop_const_tag_destructured_shorthand`.
- [x] String-template (concat) component prop whose Dynamic part is a bare imported identifier inlines the identifier directly inside the getter template literal instead of memoizing through `$.derived(() => Imported)` (e.g. `<Comp title="prefix {BRAND}" />` -> `` `prefix ${BRAND ?? ""}` ``). The decision lives entirely in `derive_component_concat_semantics` and is keyed off `ExprKind::SimpleRead | Computed | Call | Async` — Computed/Call/Async parts force their `SimpleRead` siblings to memoize uniformly, while pure `SimpleRead` (Identifier or StaticMember chain) parts inline. Tests: `component_prop_concat_import_identifier`, `component_prop_concat_import_and_call`
- [x] Inline callback component props that mutate `$state` (for example `onclick={() => count++}`) should stay as direct callback expressions instead of being memoized through derived getter wrappers (test: `diagnose_component_onclick_state`)
- [x] Component prop whose value is a bare reference to a top-level non-reactive const/non-mutated binding initialized with an arrow or function literal (e.g. `const save = () => {}; <Btn onclick={save} />`, `<Comp foo={save} />`) emits `{ name: ident }` inline instead of `get name() { return ident; }`. The decision is the existing `ComponentPropMemo` answer that `AttributeSemantics` already publishes; the classifier composes it locally from `ExpressionData` (kind/evaluation/references from `ExpressionSemantics`) and `BindingSemantics` (from `ReactivitySemantics`). For the `NonReactive` reference branch the helper accepts the read as stable when either `Scoping::is_init_known(sym)` (literal init) holds or `evaluation.class() == Some(ValueClass::Function)` — the latter is already produced by `eval_identifier` via `bindings_init.get(sym) → ArrowFunctionExpression/FunctionExpression`. Reactive bindings (e.g. `let h = $state(arrow)`) still wrap because their `BindingSemantics` arm forces unstable regardless of expression class. The same composition serves the sibling classifiers (component spread / attach / html-concat) inside `attribute_semantics::builder`. The `on*`-prefix in the original symptom was unrelated — the gap was that function-typed reads weren't recognized as stable. (test: `diagnose_component_onclick_const_arrow`)
- [x] `on:` directives on components serialize into `$$events`, including dev-mode shared-handler wrapping parity (tests: `component_events`, `component_events_dev_apply`)
- [x] Runes-mode dotted or stateful component references lower through `$.component(...)`, including dotted roots whose first segment is a lowercase JS identifier containing `_` or `$` and dynamic refs read from `$$props` (tests: `component_dynamic_dotted`, `component_dynamic_dotted_identifier_root`, `component_dynamic_props_access`)
- [x] Runes-mode dotted dynamic component refs whose root binding is non-normal use the same template binding read semantics as ordinary template reads, including `$props()`-backed roots like `<registry.Widget />` (test: `component_dynamic_dotted_props_root`)
- [x] Runes-mode local component bindings whose tag names include `_` or digits, including `<Derived_1>` from `$derived(Widget)` and `<Const_0>` from `{@const}`, parse and lower through the dynamic-component path with `bind:this` (test: `component_local_underscored_bind_this`)
- [x] Analyze emits component-specific validation/warnings for invalid directives and attribute edge cases (component-tag directives/modifiers plus direct attribute-name/value checks)
- [x] Dev-mode synthesized default children slot prop on a static component is wrapped with `$.wrap_snippet($name, ($$anchor, $$slotProps) => { ... })` (test: `component_dev_default_children_wrap_snippet`)
- [x] Component prop whose value is a member-access chain rooted in a `MetaProperty` (e.g. `<Comp url={import.meta.env.VITE_X} />`) emits a `get url() { return import.meta.env.VITE_X; }` getter instead of an inline `{ url: import.meta.env.VITE_X }`. Reference treats any `MetaProperty`-rooted read as runtime-evaluated. Owning layer: 3.A.3 `ExpressionSemantics`. The collector exposes a builder-private fact (peer of `has_call` / `has_await` / `has_store_ref`) marking that the peeled root of the expression is `MetaProperty`; `derive::is_dynamic_template` ORs it into the `TopLevelForm::Member`/`Call` dynamism check so the published `ExprKind` for the expression is `SimpleRead { reactive: true }`. `references_need_wrap` early-returns on that canonical kind when `references` is empty, and `update_aggregates` notes `ContextSignal::IMPORT_OR_PROP_MEMBER` so `$.push/$.pop` emit. No sidecar boolean on `ExpressionData`. Test: `component_prop_import_meta_getter`.
- [x] Component prop whose value is a computed-member access chain (bracket notation, e.g. `<Comp dataTestid={obj['data-testid']} />` where `obj` is `$derived` / `$state`) emits a `get dataTestid() { return $.get(obj)["data-testid"]; }` getter inline instead of hoisting through `let $0 = $.derived(() => $.get(obj)["data-testid"])`. Owning layer: 3.A.4 `AttributeSemantics`. Fix: `derive_component_prop_memo_for_expression` (`crates/svelte_analyze/src/attribute_semantics/builder/mod.rs`) treats only `Expression::Identifier | Expression::StaticMemberExpression` as `simple_shape`; add `Expression::ComputedMemberExpression` so the `needs_wrap` branch resolves to `ComponentPropMemo::Getter` rather than `ComponentPropMemo::Derived` for the same pure-read shape that reference inlines. Test: `diagnose_component_prop_computed_member_getter`.
- [x] Component prop with a hyphenated key whose value is a reactive expression (e.g. `<Child aria-disabled={!count} />` with `count = $state(0)`) must emit the same `let $0 = $.derived(() => ...); Child(node, { get "aria-disabled"() { return $.get($0); }, ... })` shape reference does. Today the entire output is empty — the component-prop serialization path for the `ComponentPropMemo::Derived` branch silently drops props whose key requires quoting. Owning layer: codegen (component prop serialization). Test: `diagnose_component_prop_hyphenated_key_derived`.
- [x] Component spread attribute whose argument is a call expression (e.g. `<Child {...build(value)} />`, including legacy-mode `<Child {...wrap($$props)} />`) memoizes through a hoisted `derived_safe_equal` and reads via the thunk passed to `spread_props`: `{ let $0 = $.derived_safe_equal(() => build(value)); Child($$anchor, $.spread_props(() => $.get($0))); }`. Rust currently emits a bare `$.spread_props(() => build(value))` thunk without the memo binding, so re-renders re-invoke the call instead of comparing the previous spread snapshot. Owning layer: 3.A.4 `AttributeSemantics` — extend the component-spread memo classifier (peer of `derive_component_prop_memo_for_expression`) so call expressions resolve to `Memo::Derived` with the safe-equal helper, then codegen wraps the spread argument the same way string-template props already do. Helper selection mirrors `CodegenView::derived_helper()` (`$.derived` in runes, `$.derived_safe_equal` in SoftLegacy/HardLegacy). Test: `diagnose_component_spread_call_memo`.
- [x] Legacy-mode child-component prop getter whose body is a `MetaProperty`-rooted read (e.g. `import.meta.env.VITE_X`) and which is emitted inside a fragment scope (consequent of `{#if}`, snippet body, or a parent component's children slot) wraps the body in `$.untrack(() => ...)` — `get url() { return $.untrack(() => import.meta.env.VITE_X); }`. Top-level legacy component invocations and the runes-mode equivalent stay bare. Companion to the `MetaProperty` getter classification above: the existing entry handles "emit a getter at all"; this entry handles "wrap the body in untrack" when the getter executes inside an effect-tracked fragment under legacy reactivity. Layer: codegen (component prop serialization) consuming an `ExpressionData`/scope fact that already distinguishes fragment vs. instance scope. Test: `diagnose_legacy_component_prop_import_meta_getter_in_if`.
- [x] Shorthand `on:event` directive on a component (no value, e.g. `<Widget on:focus on:keydown />`) forwards the event from the parent: each entry serializes into `$$events: { <event>($$arg) { $.bubble_event.call(this, $$props, $$arg); } }` and the host function gains the `$$props` parameter even if no other prop is referenced. Owning layer: codegen (component event serialization) + analyze. Analyze: `build_runtime_plan` (`crates/svelte_analyze/src/lib.rs`) walks `as_component_like` nodes for any `OnDirectiveLegacy` with `expression.is_none()` and ORs the result into `needs_props_param`. Codegen: `build_component_events` (`crates/svelte_codegen_client/src/codegen/component_props/events.rs`) branches on `ev.expr_id`; the `None` branch emits a `function ($$arg) { $.bubble_event.call(this, $$props, $$arg); }` method via the private `build_bubble_event_method_legacy` helper. The dead raw fact `EventRaw.has_expression` was removed (the `expr_id: Option<OxcNodeId>` already carries presence). Test: `diagnose_component_on_directive_shorthand_forward`.

## Out of scope (tracked elsewhere)

- Dev-mode `bind_*` helpers emitting named `function get/set` form on element bindings (`bind_checked`, `bind_group`, `bind_element_size`, `bind_content_editable`, `bind_volume`, `bind_paused`) — bindings spec.
- `$.create_custom_element(App, …)` export when `customElement: true` — custom-element spec.

## Reference

- Reference analyze:
  - `reference/compiler/phases/2-analyze/visitors/Component.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/component.js`
  - `reference/compiler/phases/2-analyze/visitors/shared/attribute.js`
- Reference client transform:
  - `reference/compiler/phases/3-transform/client/visitors/Component.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- Rust implementation:
  - `crates/svelte_parser/src/lib.rs`
  - `crates/svelte_analyze/src/passes/template_scoping.rs`
  - `crates/svelte_analyze/src/passes/element_flags.rs`
  - `crates/svelte_codegen_client/src/template/component.rs`
  - `crates/svelte_codegen_client/src/template/traverse.rs`
  - `crates/svelte_diagnostics/src/lib.rs`
  - `tasks/compiler_tests/cases2/component_*`

## Test cases

- [x] `component_basic`
- [x] `component_non_self_closing`
- [x] `component_props`
- [x] `component_bind_this`
- [x] `component_bind_prop_forward`
- [x] `component_snippet_prop`
- [x] `component_snippet_with_children`
- [x] `component_multiple_snippets`
- [x] `component_spread_props`
- [x] `component_events`
- [x] `component_events_dev_apply`
- [x] `component_dynamic_dotted`
- [x] `component_dynamic_dotted_identifier_root`
- [x] `component_dynamic_props_access`
- [x] `component_dynamic_dotted_props_root`
- [x] `component_local_underscored_bind_this`
- [x] `diagnose_component_onclick_state`
- [x] `component_prop_const_call_init_getter`
- [x] `component_prop_const_tag_member`
- [x] `diagnose_component_prop_const_tag_member_init`
- [x] `component_prop_concat_call_memo`
- [x] `component_prop_concat_call_with_literal`
- [x] `component_prop_concat_import_identifier`
- [x] `component_prop_concat_import_and_call`
- [x] analyzer unit tests: component invalid directive, component `on:` modifier validation, component illegal colon warning, component unquoted attribute sequence
- [x] `component_invalid_directive_use`
- [x] `component_on_modifier_only_allows_once`
- [x] `component_dev_default_children_wrap_snippet`
- [x] `diagnose_component_onclick_const_arrow`
- [x] `component_prop_import_meta_getter`
- [x] `diagnose_component_prop_computed_member_getter`
- [x] `diagnose_component_on_directive_shorthand_forward`
- [x] `diagnose_component_prop_hyphenated_key_derived`
- [x] `diagnose_legacy_component_prop_import_meta_getter_in_if`
- [x] `diagnose_component_prop_const_tag_destructured_shorthand`
- [x] `diagnose_component_spread_call_memo`
