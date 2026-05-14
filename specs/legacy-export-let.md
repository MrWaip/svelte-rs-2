# Legacy export let props

## Current state
- **Working**: 40/40 use cases (analyzer + transform + codegen)
- **Tests**: 35/35 e2e compiler tests green; analyzer unit tests cover the classification surface.
- Last updated: 2026-05-22
- Architecture: every legacy bindable prop is classified as `DeclarationSemantics::LegacyBindableProp(LegacyBindablePropSemantics { default_lowering, flags })`. `flags` is the precomputed `$.prop(...)` bitfield, with `PROPS_IS_LAZY_INITIAL` always set for destructured leaves. `$$props` / `$$restProps` reads carry `ReferenceSemantics::LegacyPropsIdentifierRead` / `LegacyRestPropsIdentifierRead`. Read/write/member-mutation sites reuse the runes `PropRead(Source)` / `PropMutation` / `PropSourceMemberMutationRoot` channels. Aggregates (`legacy_bindable_prop_symbols`, `legacy_uses_props`, `legacy_uses_rest_props`, `legacy_has_member_mutated`) live on `ReactivitySemantics`; `RuntimePlan` carries a precomputed `LegacyInit` enum + `has_legacy_runtime_init` summary; codegen reads dumb. `ExpressionInfo.uses_legacy_sanitized_props` drives the `$.deep_read_state` / `$.untrack` coarse-wrap around member reads of `$$sanitized_props`. Transform `process_legacy_export_props` lowers inline + specifier + destructured forms to `let foo = $.prop(...)` (destructure: `tmp = init` + `$$array = $.derived(() => $.to_array(tmp.<key>, len))` helpers + per-leaf `$.prop($$props, "<name>", flags, () => $.fallback(tmp.<key>, default))`). Identifier-target `PropMutation` rewrites live in shared `rewrite_prop_identifier_assignment` / `rewrite_prop_identifier_update` helpers in `transformer/rewrites.rs`, called from both `transform_assignment` / `transform_update` (script body) and `template_rewrites::rewrite_template_enter` / `rewrite_template_exit` (template). Assignment helper preserves compound operators via `build_compound_value` so `count -= 7` lowers to `count(count() - 7)`. Update helper emits `$.update_prop(name)` or `$.update_pre_prop(name[, -1])`, with the legacy coarse-wrap kicking in via the new `ExpressionKind::Update` (parity with reference's `metadata.has_assignment`) so `{count++}` becomes `($.deep_read_state(count()), $.untrack(() => $.update_prop(count)))`.
- Unified reactivity dependency status: satisfied.

## Source

ROADMAP.md — Legacy Svelte 4: `export let` props

## Syntax variants

- `<script>export let foo;</script>`
- `<script>export let bar = 'default value';</script>`
- `<script>export let foo = undefined;</script>`
- `<script lang="ts">export let name: String | undefined;</script>`
- `<script lang="ts">export let name: SomeType | null | (() => void) = null;</script>`
- `<script>export var count = 1;</script>`
- `<script>let foo = 1; export { foo };</script>`
- `<script>let className; export { className as class };</script>`
- `<script>export let { x: foo, z: [bar] } = expr;</script>`
- `<button {...$$props} class={$$props.class ?? ''}>click me</button>`
- `<button {...$$restProps} class="variant-{variant}">click me</button>`
- `<svelte:options accessors={true} /><script>export let count = 1;</script>`

## Use cases

- [x] `ReactivitySemantics` builder classifies every legacy prop binding through `DeclarationSemantics::LegacyBindableProp(LegacyBindablePropSemantics)` (real symbols: `export let` / `export var` / `export { foo }` / `export { foo as bar }` / destructured leaves) and `ReferenceSemantics::LegacyPropsIdentifierRead` / `LegacyRestPropsIdentifierRead` (synthetic `$$props` / `$$restProps` identifier reads keyed by `ReferenceId`). Tests: 13 analyzer unit tests in `crates/svelte_analyze/src/tests.rs` (`legacy_export_let_classifies_as_legacy_bindable_prop` … `legacy_classification_skipped_in_runes_mode`).
- [x] Analyzer materializes dedicated legacy-prop entities for `export let` / `export var` / export-specifier / destructured legacy exports through the `ReactivitySemantics` records above; transform/codegen consumers read declaration_semantics + AST only.
- [x] Explicit legacy mode with a defaulted `export let` lowers through the prop pipeline instead of staying a plain export (test: `svelte_options_runes_false_override`).
- [x] Required legacy props without defaults still lower through `$.prop(...)` and template getter calls rather than reading raw `$$props` (test: `legacy_export_let_required`).
- [x] Typed legacy `export let` declarations preserve their TS annotation shape while still materializing the same dedicated legacy-prop entity and runtime prop lowering as untyped declarations (test: `legacy_export_let_typed`).
- [x] `export var` declarations become legacy bindable props instead of plain mutable exports (test: `legacy_export_var_basic`).
- [x] Separate instance-script export specifiers on `let` bindings promote those bindings to legacy props rather than component exports (test: `legacy_export_specifier`).
- [x] Export-specifier aliases use the exported name as the prop key while keeping the local binding inside the component (`export { className as class }`) (test: `legacy_export_specifier_alias`).
- [x] Destructured legacy prop exports treat leaf identifiers as prop names and lower path-based defaults through `tmp` + `$.derived(() => $.to_array(...))` + `$.fallback(...)` helpers (test: `legacy_export_destructure`).
- [x] Legacy immutable mode still treats `export let` as a prop input and emits the `$.deep_read_state`/`$.untrack` template-effect wrappers around prop member reads (test: `svelte_options_immutable_legacy`).
- [x] Legacy prop accessors expose getter/setter pairs for `export let` props when `accessors={true}` is enabled (test: `svelte_options_accessors_legacy`).
- [x] Legacy `$$props` identifier/member reads lower through `$$sanitized_props`: identifier rewrite via `ReferenceSemantics::LegacyPropsIdentifierRead`, declaration via `OutputPlanData::needs_sanitized_legacy_props`, coarse-wrap via `ExpressionInfo::uses_legacy_sanitized_props` for unresolved member reads (test: `legacy_props_basic`).
- [x] Legacy `$$restProps` lowers through `$.legacy_rest_props($$sanitized_props, [keys])` and excludes named legacy props declared with `export let` (test: `legacy_rest_props_basic`).
- [x] Runes mode rejects direct `$$props` usage with `legacy_props_invalid` (test: `validate_legacy_props_invalid_in_runes_mode`).
- [x] Runes mode rejects direct `$$restProps` usage with `legacy_rest_props_invalid` (test: `validate_legacy_rest_props_invalid_in_runes_mode`).
- [x] Runes-mode `export let` reports `legacy_export_invalid` before state-export diagnostics; `state_invalid_export` skips legacy `let` declarators in runes mode (tests: `validate_state_invalid_export_for_reassigned_state`, `validate_state_invalid_export_for_reassigned_state_raw`, `validate_state_invalid_export_no_error_without_reassignment`).
- [x] Unused legacy props warn with `export_let_unused` (test: `validate_export_let_unused`).
- [x] Compound assignments to a legacy `export let` prop expand the original value: `count -= 7` lowers to `count(count() - 7)`. `rewrite_prop_identifier_assignment` builds the compound value via `build_compound_value(operator, call_expr(name), right)` (test: `legacy_export_let_compound_assign_prop`).
- [x] `++` / `--` on a legacy `export let` prop inside template expressions wraps as `($.deep_read_state(<prop>()), $.untrack(() => $.update_prop(<prop>)))`. `template_rewrites::rewrite_template_enter` dispatches `rewrite_prop_identifier_update` before signal/store/deep-store; legacy coarse-wrap activates because UpdateExpression now maps to `ExpressionKind::Update`, parity with reference's `metadata.has_assignment` (test: `legacy_export_let_update_prop_in_template`).
- [x] Plain assignment to a legacy `export let` prop inside a template expression (`{count = 42}`) lowers through the same `rewrite_prop_identifier_assignment` path during template-exit and is wrapped as `($.deep_read_state(count()), $.untrack(() => count(42)))` by the legacy coarse-wrap (test: `legacy_export_let_assign_prop_in_template`).
- [x] Member-target update of a legacy bindable `export let` prop inside a template expression (`{obj.x++}`) lowers through the shared `rewrite_prop_member_update` (extracted from `transform_update`), wrapping the update as `obj(obj().x++, true)` for bindable sources and emitting the legacy coarse-wrap as `($.deep_read_state(obj()), $.untrack(() => obj(obj().x++, true)))`. The dev ownership-validator hook `rewrite_prop_update_ownership_exit` is now also wired into `rewrite_template_exit` (test: `legacy_export_let_member_update_in_template`).
- [x] TS-cast default value on a function-typed `export let` collapses through the cast — `export let cb: Cb = (() => {}) as unknown as Cb` lowers as `$.prop($$props, "cb", 8, () => {})`, not as lazy-initial flag 24 wrapping `() => (() => {})` (test: `legacy_export_let_default_typed_cast_arrow`).
- [x] `{#key prop.field}` over a legacy `export let` prop member emits the legacy coarse-wrap in the key expression itself — `$.key(node, () => ($.deep_read_state(prop()), $.untrack(() => prop().field)), …)` — same wrap shape applied inside `template_effect` (test: `legacy_export_let_key_block_member_coarse_wrap`).
- [x] `bind:this={prop}` on a regular element where `prop` is a legacy bindable `export let` emits `$.bind_this(div, ($$value) => prop($$value), () => prop())` — the prop accessor is called as a function in both setter and getter callbacks. Test: `diagnose_legacy_export_let_element_bind_this`.
- [x] Legacy `export let` with a default expression that references another legacy bindable prop emits `PROPS_IS_LAZY_INITIAL` (flag `8 | 16 = 24`) and passes the referenced accessor directly without invoking it: `export let paddingX = offsetX` → `$.prop($$props, "paddingX", 24, offsetX)`. Classifier `classify_expression_default` resolves an `Identifier` default to its symbol and returns the new `PropDefaultEmit::LazyAccessor` variant when the symbol carries `BindingSemantics::LegacyBindableProp`. Transform's `build_prop_call` arm for `LazyAccessor` accepts both the raw `Identifier` and the post-rewrite `<ident>()` shape (legacy prop reads are rewritten before `exit_statements` runs), extracts the accessor name and re-emits a fresh `IdentifierReference` (no `reference_id` ⇒ not re-rewritten). Test: `diagnose_legacy_export_let_default_prop_reference`.
- [x] Multiple legacy `export let` props next to an `export const` API export must not leak any `export let` symbol into the `$$exports` object or its `$.bind_prop($$props, "...", ...)` tail; only `export const`/`export function`/`export class` exports do. Test: `diagnose_legacy_export_let_leaks_into_exports`.
- [x] Legacy script with both `export const` API and a trigger for `LegacyInit::Plain` (e.g. `setContext`/`getContext` so `OutputPlanData::needs_context = true`, or member-mutated legacy state) emits `$.init()` BEFORE the `$.bind_prop($$props, "...", ...)` tail. Test: `diagnose_legacy_export_const_init_order`.
- [x] Store-prefixed writes on a legacy bindable `export let` do not classify the underlying prop as `PROPS_IS_UPDATED` and invoke the prop accessor for the `$.store_mutate` base. `$store = undefined` lowers to `$.store_set(store(), undefined)`; `$store.x = 1` lowers to `$.store_mutate(store(), $.untrack($store).x = 1, $.untrack($store))`; `$.prop($$props, "store", 8)` (no `PROPS_IS_UPDATED` bit). `finalize_legacy_aggregates` filters `prop_member_mutation_root_refs` through `ReferenceSemantics::Store{Read,Write,Update}` before contributing to the prop `updated` bit; `make_store_mutate` takes the base as `Expression<'a>` built via `make_store_base_expr` so `LegacyBindableProp` routes to `store()` and plain stores to the bare identifier (test: `diagnose_legacy_export_let_store_prop_writes`).
- [x] `NeedsContextVisitor` must classify member access on the synthetic legacy props identifiers (`$$props.foo`, `$$restProps.bar`, computed `$$props["x"]`, etc.) as a `needs_context = true` trigger, matching reference `is_safe_identifier`'s `rest_prop`-binding rule (returns `false` for member-roots whose `Identifier` resolves to a `rest_prop` binding). Today our visitor calls `is_safe_sym` which dereferences `ReferenceId → SymbolId`; `$$props` / `$$restProps` have no real `SymbolId` (they are surfaced via `ReferenceSemantics::LegacyPropsIdentifierRead` / `LegacyRestPropsIdentifierRead`), so the visitor falls into the safe branch and undercounts. Layer: 3.A.2 / 3.B (`NeedsContextVisitor` in `crates/svelte_analyze/src/passes/js_analyze/needs_context.rs` driving `OutputPlanData::needs_context`). Tests: `legacy_props_basic` (regression sentinel — must keep `$.init()` for `$$props.class` member access); unit tests `needs_context_set_by_member_access_on_legacy_props`, `needs_context_set_by_member_access_on_legacy_rest_props`, `needs_context_stays_false_for_bare_legacy_props_identifier_read`.
- [x] Template-expression member access on the synthetic legacy props identifiers (`{$$props.class}`, `<x style="…{$$props.foo}…">`, computed `$$props["x"]`, and the `$$restProps`-counterparts) must drive `ExpressionSemantics` to note a `ContextSignal::REST_PROP_MEMBER` so `is_context_required()` flips `OutputPlanData::needs_context = true`, mirroring the script-body parity already established for `NeedsContextVisitor`. Today the collector pushes `member_or_call_roots` only when the member-root resolves to a real `SymbolId`; synthetic `$$props` / `$$restProps` (no `SymbolId`, carried by `ReferenceSemantics::LegacyPropsIdentifierRead` / `LegacyRestPropsIdentifierRead`) bypass this path. Layer: 3.A.3 `ExpressionSemantics` (collector + walker `update_aggregates` / `store_render_tag` in `crates/svelte_analyze/src/expression_semantics/builder/`). Tests: `legacy_props_basic`, `legacy_rest_props_basic` (regression sentinels — must keep `$.init()` for `{$$props.class}` / `{$$restProps.foo}` member access without relying on `has_legacy_props_read` in the gate); unit tests `needs_context_set_by_template_member_access_on_legacy_props`, `needs_context_set_by_template_member_access_on_legacy_rest_props`.
- [x] After `OutputPlanData::needs_context` becomes the only source of legacy-init emission for `$$props`/`$$restProps` on both script-body and template-expression paths, `build_runtime_info` must drop `has_legacy_props_read` from the `LegacyInit::Plain` gate so a bare `$:` read of `$$props` (no member access, no contextful calls, no `new`) does NOT emit `$.init()`. Layer: 3.A.2 `ReactivitySemantics` aggregates (gate at `crates/svelte_analyze/src/lib.rs:349-357`); 3.B `RuntimePlan.legacy_init` is the dumb carrier. Depends on the previous two use cases (`needs_context` parity for synthetic legacy props on script + template). Test: `diagnose_legacy_props_spread_no_init`.
- [x] Member-target plain assignment to a legacy bindable `export let` prop with a computed key that is itself a member expression (`prop[item.id] = value`) emits the bindable-source wrap `prop(prop()[item.id] = value, true)`. `rewrite_prop_member_assignment` now collects `prop_alias` independently of `prop_mutation_segments_from_member`; `finish_semantic_prop_member_assignment` takes `segments: Option<Vec<Expression>>` and skips `wrap_prop_mutation_validation` / `pending_prop_update_validations` when the key cannot be decomposed (e.g. `StaticMemberExpression` index), while the bindable-source wrap and LHS rewrite still fire on `ReferenceSemantics::PropSourceMemberMutationRoot { bindable: true }`. Test: `diagnose_legacy_export_let_member_assign_computed_member_key`.
- [x] `class={expr}` on a regular element where `expr` is a member read of a legacy bindable `export let` prop (`class={config.cls}`, `class={\`a ${cfg.flag && 'b'}\`}`) coarse-wraps the class expression inside the `$.clsx` argument (or directly when `needs_clsx` is false): `$.set_class(div, 1, $.clsx(($.deep_read_state(config()), $.untrack(() => config().cls))))`. `build_class_attr_value` (`ExpressionAttribute` arm) now reads `ExpressionData.legacy_wrap` via `maybe_wrap_legacy_coarse_expr` before the optional `$.clsx`-wrap — same carrier consumed by `emit_attr_expression`, key block, html-tag, and concat-attribute emit sites. Test: `diagnose_legacy_class_attribute_prop_member_coarse_wrap`.
- [x] Legacy `export const` / `export function` / `export class` API names contribute to the same `ReactivitySemantics.legacy_bindable_prop_symbols` aggregate as `export let`, so the `$.legacy_rest_props($$sanitized_props, [...])` exclusion list filters readonly component-API names alongside bindable props. `classify_export_named_declaration` records the underlying binding into the aggregate without creating a `LegacyBindablePropSemantics` (these are not bindable props). Test: `diagnose_legacy_export_const_excluded_from_rest_props`.
- [x] Legacy `export const` API exports must emit their `$.bind_prop($$props, "<name>", <name>)` tail AFTER the template render call (after `$.append($$anchor, ...)` / element creation), not before. Reference orders the bind-prop tail at the very end of the component body, just before `$.pop($$exports)`. Today we emit it before the template render block, breaking parity for any legacy component that combines `export const` with template output. Layer: codegen (`crates/svelte_codegen_client/src/script/props.rs` ordering vs template emit). Test: `diagnose_legacy_export_const_bind_prop_after_template_render`.
- [x] Coarse-wrap of a template attribute expression that reads a member of the synthetic `$$restProps` identifier (`<div id={$$restProps.id || name}>`) must (a) pass the user-facing `$$restProps` ident — not the internal `$$sanitized_props` — as the deep-read carrier and (b) order the synthetic-props carrier BEFORE any legacy `export let` accessor reads: `$.set_attribute(div, "id", ($.deep_read_state($$restProps), $.deep_read_state(name()), $.untrack(() => $$restProps.id || name())))`. Same rule applies to `$$props.<member>` (carrier = `$$sanitized_props` is correct there, since reference uses the same internal symbol — verify per-side at impl time). Today the carrier choice and ordering both diverge from reference. Sibling of line 60 (`needs_context` parity for synthetic legacy props) — this use case extends the same family to the coarse-wrap arg list emit, not the gate. Layer: 3.A.3 `ExpressionSemantics` (carrier+order classification for `ReferenceSemantics::LegacyPropsIdentifierRead` / `LegacyRestPropsIdentifierRead` in the coarse-wrap deep-read aggregation) + codegen consumer printing the args. Test: `diagnose_legacy_attr_expression_restprops_member_coarse_wrap`.
- [x] Same `$$restProps`-member-plus-legacy-prop coarse-wrap must fire when the attribute is emitted through `$.attribute_effect(...)` instead of `$.set_attribute(...)` — i.e. when the element mixes a `{...$$restProps}` spread, a `bind:<prop>`, and a dynamic attribute (`<input type="checkbox" bind:checked id={$$restProps.id || name} {...$$restProps} />`). Reference emits the property as `id: ($.deep_read_state($$restProps), $.deep_read_state(name()), $.untrack(() => $$restProps.id || name()))` inside the `attribute_effect` object literal; today we inline the bare expression `id: $$restProps.id || name()` and skip the wrap. Same family as line 66, extended to the `attribute_effect` property-bag emit site. Layer: 4 codegen (attribute lowering — property emit inside the `attribute_effect` object literal in `crates/svelte_codegen_client/src/codegen/attributes/`); analyzer-side `ExpressionData.legacy_wrap` is already populated by line-66 work. Test: `diagnose_legacy_attribute_effect_expression_restprops_coarse_wrap`.
- [x] Template-only `{...$$props}` spread (no script body, or script with only type-imports) on a regular/SVG element must NOT emit `$.push($$props, false)` / `$.pop()` boot scaffolding. Reference omits push/pop because bare-identifier `$$props` reads are consumed wholesale by `$.legacy_rest_props($$props, [...])` and need no per-prop reactive-context. Today the `needs_push` gate at `crates/svelte_analyze/src/lib.rs:322-324` OR-s `has_legacy_props_read`, so any `$$props` reference flips push on — even pure spread. Sibling of use case 61 (which already dropped `has_legacy_props_read` from the `LegacyInit::Plain` gate via `needs_context`): the equivalent fix on `needs_push` is to gate on `data.output.needs_context` for synthetic-props reads instead of the raw aggregate, preserving push/pop for member access (`{$$props.foo}`) which already flips `needs_context = true` via use cases 59/60. Layer: 3.A.2 `ReactivitySemantics` aggregates → `build_runtime_info` gate (`crates/svelte_analyze/src/lib.rs`); 3.B `RuntimePlan.needs_push` is the dumb carrier. Test: `diagnose_legacy_props_spread_only_no_push`.
- [x] Legacy `export let` whose default expression is a non-trivial composite (e.g. `ConditionalExpression`/`BinaryExpression`/`LogicalExpression`) that contains a read of another legacy bindable prop must lower as `PROPS_IS_LAZY_INITIAL` (flag `8 | 16 = 24`) with the default wrapped in a thunk: `export let label = kind === 'a' ? 'first' : 'second'` → `$.prop($$props, "label", 24, () => kind() === "a" ? "first" : "second")`. Reference reaches this because `is_simple_expression` is consulted after legacy prop-read rewriting turns `kind` into the call `kind()` (CallExpression, non-simple), so the composite becomes non-simple. Our `classify_expression_default` runs in analyze on the raw AST where `kind` is still a bare `Identifier`, the composite passes `is_simple_expression`, and the prop falls into `PropDefaultEmit::Eager` (flag `8`, no thunk). Sibling of use case 55 (bare-identifier → `LazyAccessor`); this case is the composite-expression variant. Layer: 3.A.2 `ReactivitySemantics` — `classify_expression_default` in `crates/svelte_analyze/src/reactivity_semantics/builder_v2/legacy.rs` needs to treat the default as non-simple whenever any identifier inside it resolves to a `BindingSemantics::LegacyBindableProp` symbol (or a `LegacyApiExport`), and select `PropDefaultEmit::Lazy`. Test: `diagnose_legacy_export_let_default_prop_reference_in_conditional`.
- [x] Legacy component that uses `$$restProps` only through a template expression (no `export let`, no script-body `$$props`/`$$restProps` reads, no `$:`) must still emit `$$props` as the second parameter of the component function — reference yields `export default function App($$anchor, $$props)` because the body declares `const $$sanitized_props = $.legacy_rest_props($$props, [...])`. Today we emit `App($$anchor)` while still emitting the `legacy_rest_props($$props, ...)` body, leaving `$$props` undefined at runtime. The gate that decides whether to append `$$props` to the function signature in codegen `script/props.rs` (or its analyzer carrier on `RuntimePlan`) reads only the script-body legacy-signal aggregates and misses the template-side `ReferenceSemantics::LegacyRestPropsIdentifierRead`. Layer: 3.A.2 `ReactivitySemantics` aggregates → `RuntimePlan` carrier feeding 4 codegen function-signature emission. Test: `diagnose_legacy_restprops_template_only_function_param`.

- [x] Legacy `export let` prop read inside a closure (e.g. `const opts = () => ({ duration })` plus a template tag `{opts().duration}`) must emit the legacy boot scaffolding `$.push($$props, false)` / `$.init()` / `$.pop()` around the component body — same shape produced when a prop is read directly. Real trigger lives in template-side `ExpressionSemantics`, not in the legacy aggregates: reference's `is_safe_identifier` flips `analysis.needs_context = true` whenever a `MemberExpression`'s object peels (through static/computed/private member chain only) to a non-`Identifier` root, e.g. a `CallExpression` (`opts().duration`). Our `Collector::visit_member_expression` only recorded a root when `expression_root_sym(obj)` resolved to a `SymbolId` (or `$$props`/`$$restProps`); a `CallExpression`-rooted member fell through silently. Fix: new `ExprFacts.has_unsafe_member_root` flag set by the collector when the peeled member-object is not an `Identifier`, consumed in `update_aggregates` as `ContextSignal::IMPORT_OR_PROP_MEMBER`. `OutputPlanData::needs_context` then flips, and the existing `needs_push` + `LegacyInit::Plain` gates in `build_runtime_info` emit push/init/pop. Layer: 3.A.3 `ExpressionSemantics` collector → walker aggregate; 3.A.2 `ReactivitySemantics`/`OutputPlanData` consumer unchanged. Sibling of use cases 60, 66 (other `needs_context` parity gaps for template member roots). Test: `diagnose_legacy_export_let_closure_capture_needs_push_init_pop`.

## Out of scope

- SSR output for legacy props
- Component API exports from `export const`, `export function`, and `export class`

## Implementation note

- **Hard rule**: every legacy prop entity (`export let`, `export var`, separate `export { foo }`, `export { foo as bar }`, destructured `export let { … }`, `$$props`, `$$restProps`) must be classified inside `ReactivitySemantics` (`PropDeclarationSemantics` / `PropDeclarationKind` in `crates/svelte_analyze/src/reactivity_semantics/data.rs`). Implementation is allowed and expected to extend that enum (e.g. add `LegacySource { default, required, bindable, accessor }`, `LegacyRest`, `LegacySanitizedProps`) rather than introduce a parallel legacy-prop classifier. Downstream transform/codegen reads only from `ReactivitySemantics`; no second source of truth.
- Legacy prop hooks at the codegen layer (e.g. `script/props.rs`) may stay explicit and legacy-named for containment, but their inputs must be the `ReactivitySemantics` records described above.

## Reference

Svelte:
- `reference/docs/99-legacy/03-legacy-export-let.md`
- `reference/docs/99-legacy/04-legacy-$$props-and-$$restProps.md`
- `reference/compiler/phases/1-parse/read/options.js`
- `reference/compiler/phases/2-analyze/index.js`
- `reference/compiler/phases/2-analyze/visitors/Identifier.js`
- `reference/compiler/phases/2-analyze/visitors/ExportNamedDeclaration.js`
- `reference/compiler/phases/2-analyze/visitors/ExportSpecifier.js`
- `reference/compiler/phases/3-transform/client/visitors/Program.js`
- `reference/compiler/phases/3-transform/client/visitors/Identifier.js`
- `reference/compiler/phases/3-transform/client/visitors/ExportNamedDeclaration.js`
- `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`
- `reference/compiler/phases/3-transform/client/transform-client.js`
- `reference/compiler/errors.js`
- `reference/compiler/warnings.js`

Our code:
- `crates/svelte_analyze/src/utils/script_info.rs`
- `crates/svelte_analyze/src/passes/post_resolve.rs`
- `crates/svelte_analyze/src/passes/js_analyze/script_body.rs`
- `crates/svelte_analyze/src/passes/js_analyze/needs_context.rs`
- `crates/svelte_analyze/src/validate/runes.rs`
- `crates/svelte_codegen_client/src/lib.rs`
- `crates/svelte_codegen_client/src/script/model.rs`
- `crates/svelte_codegen_client/src/script/props.rs`
- `crates/svelte_codegen_client/src/script/pipeline.rs`
- `crates/svelte_codegen_client/src/script/traverse/statement_passes.rs`
- `crates/svelte_analyze/src/tests.rs`
- `tasks/compiler_tests/test_v3.rs`

## Test cases

Compiler tests (`tasks/compiler_tests/cases2/`):

- [x] `svelte_options_runes_false_override`
- [x] `svelte_options_accessors_legacy`
- [x] `svelte_options_immutable_legacy`
- [x] `legacy_export_let_required`
- [x] `legacy_export_var_basic`
- [x] `legacy_export_specifier`
- [x] `legacy_export_specifier_alias`
- [x] `legacy_export_destructure`
- [x] `legacy_props_basic`
- [x] `legacy_rest_props_basic`
- [x] `legacy_export_let_typed`
- [x] `legacy_export_let_member_mutation`
- [x] `legacy_export_let_bind_to_inner`
- [x] `legacy_export_let_compound_assign_prop`
- [x] `legacy_export_let_update_prop_in_template`
- [x] `legacy_export_let_assign_prop_in_template`
- [x] `legacy_export_let_member_update_in_template`
- [x] `legacy_export_let_default_typed_cast_arrow`
- [x] `legacy_export_let_key_block_member_coarse_wrap`
- [x] `diagnose_legacy_export_let_element_bind_this`
- [x] `diagnose_legacy_export_let_store_prop_writes`
- [x] `diagnose_legacy_export_let_default_prop_reference`
- [x] `diagnose_legacy_export_let_leaks_into_exports`
- [x] `diagnose_legacy_export_const_init_order`
- [x] `diagnose_legacy_props_spread_no_init`
- [x] `diagnose_legacy_export_let_member_assign_computed_member_key`
- [x] `diagnose_legacy_class_attribute_prop_member_coarse_wrap`
- [x] `diagnose_legacy_export_const_excluded_from_rest_props`
- [x] `diagnose_legacy_export_const_bind_prop_after_template_render`
- [x] `diagnose_legacy_attr_expression_restprops_member_coarse_wrap`
- [x] `diagnose_legacy_attribute_effect_expression_restprops_coarse_wrap`
- [x] `diagnose_legacy_props_spread_only_no_push`
- [x] `diagnose_legacy_restprops_template_only_function_param`
- [x] `diagnose_legacy_export_let_default_prop_reference_in_conditional`
- [x] `diagnose_legacy_export_let_closure_capture_needs_push_init_pop`
- [x] e2e smoke: `smoke_legacy_reactive_mutations_all` — covers every assignment + update operator (`=`, `+=`, `-=`, `++`, `--`, `++` prefix, `--` prefix, `&&=`, `||=`, `??=`) for legacy bindable `export let` identifier and member targets — including static (`obj.x`), computed string (`obj["x"]`), computed dynamic (`obj[key]`), and deep chains (`obj.a.b.c.x`, `obj["a"]["b"]["c"]["x"]`, mixed `obj[k1].b[k2]`) plus optional-chain reads (`obj?.a?.b?.c?.x`) — in both script body and template expressions, alongside legacy state, store, and deep-store paths. Companion `smoke_legacy_contextual_mutations_all` (ignored) extends the matrix to `{#each}` items, `{#snippet}` params, `{@const}` aliases, `{#await}` resolved/error values, and exposes the legacy mutable_source upgrade + invalidate_inner_signals chain gap tracked in debt.md.

Diagnostic tests (`tasks/diagnostic_tests/cases/`):

- [x] `props/validate_legacy_props_invalid_in_runes_mode`
- [x] `props/validate_legacy_rest_props_invalid_in_runes_mode`
- [x] `props/validate_export_let_unused`
- [x] `runes/validate_state_invalid_export_for_reassigned_state`
- [x] `runes/validate_state_invalid_export_for_reassigned_state_raw`
- [ ] `runes/validate_state_invalid_export_for_reassigned_state_export_specifier` (out of scope: `<script module>` validation pipeline)
- [x] `runes/validate_state_invalid_export_no_error_without_reassignment`
