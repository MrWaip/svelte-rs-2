# $props / $bindable

## Current state
- **Working**: 27/27 use cases
- **Tests**: 66/66 green
- Last updated: 2026-05-27

## Source

ROADMAP.md — `$props` / `$bindable`

## Syntax variants

- `let { foo } = $props()`
- `let { foo = 1 } = $props()`
- `let { foo: local = 1 } = $props()`
- `let { foo, ...rest } = $props()`
- `const props = $props()`
- `let { value = $bindable() } = $props()`
- `let { value = $bindable('fallback') } = $props()`
- `const id = $props.id()`

## Use cases

- [x] Basic destructured props source: `let { x, y = 10 } = $props()` (test: `props_basic`)
- [x] Rest props lowering: `let { x, ...rest } = $props()` (test: `props_rest`)
- [x] Identifier pattern: `const props = $props()` (tests: `props_identifier_basic`, `props_identifier_await_expression`)
- [x] Non-bindable fallback values including lazy defaults (test: `props_lazy_default`)
- [x] Prop default that's a bare identifier referring to another reactive binding (another prop, `$state`, `$derived`, store, contextual) classified as Lazy: emits `() => <ident>` wrapper and `PROPS_IS_LAZY_INITIAL` flag bit (test: `diagnose_props_default_identifier_prop_reference`)
- [x] Prop default that's a bare identifier referring to a top-level non-reactive binding (import, function declaration, plain `const`) classified as Eager: emits the identifier directly with flag bit `3`, no `() => …` wrapper and no `PROPS_IS_LAZY_INITIAL` (test: `diagnose_props_default_identifier_non_reactive`).
- [x] Local mutation of a prop source produces updatable local state (test: `props_mutated`)
- [x] `$bindable()` defaults inside `$props()` destructuring (tests: `props_bindable`, `props_mixed`)
- [x] Proxy wrapping for bindable object/array defaults (test: `tag_bindable_proxy`)
- [x] Bindable prop forwarding through component bindings (tests: `component_bind_prop_forward`, `push_binding_group_order`)
- [x] Renamed/aliased props (test: `props_renamed`): `let { foo: local = 'default' } = $props()` uses prop key in `$.prop()` call
- [x] Renamed + bindable props (test: `props_renamed_bindable`): `let { value: local = $bindable('fallback') } = $props()`
- [x] `$props.id()` basic lowering (tests: `props_id_basic`, `props_id_with_props`)
- [x] `$props.id()` validation edge cases covered by compiler-level pipeline tests
- [x] `$bindable()` validation: `bindable_invalid_location` and argument-count checks
- [x] `$props()` validation: `props_invalid_placement`, `props_duplicate`, and rune argument-count checks
- [x] Identifier-pattern `$props()` bindings like `let props = $props()` must not emit a false-positive `store_rune_conflict` warning (diagnostic test: `props_identifier_no_store_rune_conflict`)
- [x] `$props()` and `$props.id()` rejected inside `<script module>` — reference: `ast_type !== 'instance'` check in `CallExpression.js`
- [x] `$props.id()` validation: `props_id_invalid_placement`, duplicate detection with `$props()`, zero-argument enforcement
- [x] `$props()` pattern validation: `props_invalid_pattern` and `props_invalid_identifier`
- [x] `props_illegal_name` for MemberExpression access on rest props
- [x] Custom-element warning: `custom_element_props_identifier` for identifier/rest `$props()` in custom elements
- [x] Dev-mode ownership mutation validation for prop / bindable-prop member writes via `$$ownership_validator.mutation(...)` (tests: `compile_dev_props_member_mutation_uses_ownership_validator`, `compile_dev_bindable_prop_member_mutation_uses_prop_alias`, `compile_dev_bindable_prop_member_update_uses_ownership_validator`, `compile_dev_props_member_mutation_in_return_uses_ownership_validator`, `compile_dev_shadowed_bindable_member_update_does_not_use_ownership_validator`, `props_member_mutation_computed`, `props_renamed_member_update_computed`)
- [x] Member-target update of a runes prop inside a template expression (`{obj.x++}`) lowers to `obj().x++` (root rewritten to the prop getter call) via the shared `rewrite_prop_member_update` dispatched from `template_rewrites::rewrite_template_enter` (test: `runes_prop_member_update_in_template`).
- [x] Member-target compound assignment to a runes prop inside a template expression (`{obj.x += 5}`) lowers to `obj().x += 5` via the shared `rewrite_prop_member_assignment` dispatched from `template_rewrites::rewrite_template_enter` (test: `runes_prop_member_compound_in_template`).
- [x] Runes-mode instance script that re-exports a binding via an `export { name }` specifier emits the correct `$$exports` entry per binding kind, and re-export marks the binding as `reassigned` (kept reactive everywhere — prop flag, state-source, template dynamism, constant-folding). Closed as a coherent family: `ComponentSemantics` records each source-less export-specifier local (already an `IdentifierReference`) into `reexported_specifier_locals`, exposed via `scoping.is_reexported_specifier_local(sym)`; `ExportInfo` now carries `local: SymbolId` + `reference_id: Option<ReferenceId>` + `alias` (all resolved by id, no string lookup); the `$$exports` codegen loop (`crates/svelte_codegen_client/src/lib.rs`) derives the getter read-form from `reference_semantics(reference_id)` (exhaustive match) and the setter from `binding_semantics(local)` — see Mechanism notes below. Sub-cases:
  - [x] Destructured `$props()` prop re-export (`let { count = 0 } = $props(); export { count };`) — flag `3 → 7` (`PROPS_IS_UPDATED` via prop-builder OR on re-export) and `get count(){return count()} / set count($$value){count($$value)}` accessor pair instead of shorthand `{ count }` (test: `diagnose_runes_prop_export_specifier`).
  - [x] Aliased prop re-export (`export { count as foo }`) — key is the export alias `foo`, getter/setter bodies reference the local thunk `count()` (test: `runes_prop_export_specifier_alias`).
  - [x] Dev-mode prop re-export — `...$.legacy_api()` spread plus the `count()` accessor pair (test: `runes_prop_export_specifier_dev`).
  - [x] `$state` re-export (`let count = $state(0); export { count };`) — re-export keeps the unmutated state a source (`$.state(0)`, not optimized to a plain value), reads lower to `$.get(count)`, `$$exports` entry is `get(){return $.get(count)} / set($$value){$.set(count, $.proxy($$value))}` (test: `runes_state_export_specifier`).
  - [x] `$state.raw` re-export — same but setter is `$.set(count, $$value)` (no proxy) (test: `runes_raw_state_export_specifier`).
  - [x] Plain `let`/`var` local re-export (runes) — `get(){return count} / set($$value){count = $$value}`, and the template read becomes dynamic (`template_effect` + `set_text`) because external writes can change it (test: `runes_local_let_export_specifier`).
  - [x] Plain `const` (or function/class) local re-export (runes) — shorthand `{ count }` (no setter, non-dev) yet still dynamic in the template (test: `runes_local_const_export_specifier`).
  - [x] `$derived` re-export (`let double = $derived(count * 2); export { double };`) — read-only getter `get double(){ return $.get(double) }` with **no setter** (a derived is not externally writable) (test: `runes_derived_export_specifier`).
  Mechanism notes: the `$$exports` entry is built on **two orthogonal axes**, mirroring the Original's `build_getter` + binding-kind split: (1) the getter body is the binding's *read-form*, taken from `reference_semantics(ExportInfo.reference_id)` via the exhaustive `export_reactive_read` match in `crates/svelte_codegen_client/src/lib.rs` (so every binding kind — incl. `$derived` → `$.get` — is covered, no silent fall-through); (2) the setter (and its shape) is decided by `binding_semantics(local)` + declaration kind (prop → `count($$value)`, `$state` → `$.set`+`$.proxy`, `$state.raw` → `$.set`, plain `let`/`var` → assignment, everything else incl. `$derived`/`const` → no setter). Declaration-form exports (`export const`/`function`/`class`) carry no specifier reference, so `reference_id` is `None` and the read-form is derived from `binding_semantics` via `declaration_export_semantics`. Re-export also feeds the `reassigned` input of state-source (`record_state_root_declaration`, `collect_state_binding_semantics_inner`, `references.rs` signal-source, transform `state.rs`), and is excluded from `is_init_known` / `eval_identifier` so the value is no longer constant-folded. `StateInvalidExport`/`DerivedInvalidExport` (`validate/runes.rs`) stay keyed on raw `is_mutated`, so specifier-exporting a non-mutated state/derived is not falsely rejected. Distinct from the legacy-export-let family in `specs/legacy-export-let.md` (legacy mode only) and from the `accessors`/`has_ce_props` accessor-pair path. Layer: 3.A.1 `ComponentSemantics` (re-export set) + 3.A.2 `ReactivitySemantics` + 3.B `ExportInfo` carrier (`SymbolId` + `ReferenceId`) + 4 codegen + transform state-source.
- [x] Source declarator kind is preserved when destructuring `$props()` — `const { x = 0 } = $props()` emits `const x = $.prop(...)`. `DeclaratorSemantics::PropsObject`/`PropsIdentifier` carry `PropsDeclKind` (Const/Let/Var), filled from the source `VariableDeclaration.kind` and consumed by `var_decl_multi_stmt` in transform (test: `props_const_destructured_with_default`).
- [x] Identifier-pattern `$props()` member access inside a `{#snippet}` body rewrites `props.X` → `$$props.X` and keeps the snippet non-hoisted. Closed by moving `ComponentNode.name` from `String` to `ExprRef`: the JS expression in the tag name flows through the same `ComponentSemantics` / `ReactivitySemantics` pipeline as any template expression, so the root identifier's `ReferenceId` is tainted as an instance reference (driving hoist classification) and the transform's reference dispatchers rewrite `props` → `$$props` uniformly inside snippet bodies. (test: `diagnose_props_identifier_in_snippet_body`)

## Reference

- `original/docs/02-runes/05-$props.md`
- `original/docs/02-runes/06-$bindable.md`
- `original/compiler/phases/2-analyze/visitors/CallExpression.js` — `$props`, `$props.id`, `$bindable` placement and arity validation
- `original/compiler/phases/2-analyze/visitors/VariableDeclarator.js` — props pattern validation, bindable default stripping, custom-element warning
- `original/compiler/phases/2-analyze/visitors/MemberExpression.js` — `props_illegal_name`
- `original/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`
- `original/compiler/phases/3-transform/client/transform-client.js`
- `original/compiler/phases/3-transform/client/visitors/shared/utils.js` — `validate_mutation`, `$$ownership_validator.mutation`
- `original/compiler/phases/3-transform/client/utils.js`
- `crates/svelte_analyze/src/utils/script_info.rs` — structural extraction of props declarations/defaults
- `crates/svelte_analyze/src/passes/post_resolve.rs` — `PropsAnalysis` construction and bindable/runtime-plan flags
- `crates/svelte_analyze/src/passes/js_analyze/needs_context.rs` — marks props/rest access as context-sensitive
- `crates/svelte_analyze/src/validate/runes.rs` — currently validates `$state`/`$derived` only
- `crates/svelte_codegen_client/src/script/props.rs` — `$.prop`, `$.rest_props`, bindable default proxying
- `crates/svelte_codegen_client/src/script/traverse/statement_passes.rs` — props declaration replacement/removal
- `crates/svelte_diagnostics/src/lib.rs` — already contains the missing `$props`/`$bindable` diagnostics and warning codes

## Test cases

- [x] `props_basic`
- [x] `props_rest`
- [x] `props_identifier_basic`
- [x] `props_identifier_await_expression`
- [x] `props_lazy_default`
- [x] `props_mutated`
- [x] `props_bindable`
- [x] `props_mixed`
- [x] `tag_bindable_proxy`
- [x] `component_bind_prop_forward`
- [x] `push_binding_group_order`
- [x] `props_id_basic`
- [x] `props_id_with_props`
- [x] `props_renamed`
- [x] `props_renamed_bindable`
- [x] analyze unit: `bindable_invalid_location`
- [x] analyze unit: `rune_invalid_arguments_length` on `$bindable`
- [x] analyze unit: `props_invalid_placement`
- [x] analyze unit: `props_duplicate`
- [x] analyze unit: `$props.id()` duplicate handling against `$props()`
- [x] analyze unit: `props_id_invalid_placement`
- [x] analyze unit: `props_invalid_pattern`
- [x] analyze unit: `props_illegal_name` MemberExpression on rest_prop (3 tests)
- [x] analyze unit: `custom_element_props_identifier` warning (4 tests)
- [x] analyze unit: `validate_props_identifier_no_store_rune_conflict`
- [x] `props_identifier_no_store_rune_conflict`
- [x] `validate_props_invalid_placement_inside_function`
- [x] `validate_props_duplicate`
- [x] `validate_props_and_props_id_coexist`
- [x] `validate_props_invalid_pattern_computed_key`
- [x] `validate_props_id_invalid_placement_inside_function`
- [x] `validate_props_illegal_name_rest_member_access`
- [x] `validate_props_illegal_name_identifier_pattern_member_access`
- [x] `validate_props_normal_member_access_no_error`
- [x] diagnostic parity: `validate_props_invalid_placement_in_module_script`
- [x] diagnostic parity: `validate_props_id_invalid_placement_in_module_script`
- [x] diagnostic parity: `validate_props_invalid_arguments_in_module_script`
- [x] diagnostic parity: `validate_props_id_invalid_arguments_in_module_script`
- [x] `validate_custom_element_props_identifier_warns`
- [x] `validate_custom_element_props_rest_warns`
- [x] `validate_custom_element_props_destructured_no_warn`
- [x] `validate_custom_element_with_explicit_props_config_no_warn`
- [x] `validate_bindable_invalid_location`
- [x] `validate_bindable_invalid_location_inside_arrow`
- [x] `validate_bindable_too_many_args`
- [x] compiler unit: `compile_dev_props_member_mutation_uses_ownership_validator`
- [x] compiler unit: `compile_dev_bindable_prop_member_mutation_uses_prop_alias`
- [x] compiler unit: `compile_dev_bindable_prop_member_update_uses_ownership_validator`
- [x] compiler unit: `compile_dev_props_member_mutation_in_return_uses_ownership_validator`
- [x] compiler unit: `compile_dev_shadowed_bindable_member_update_does_not_use_ownership_validator`
- [x] `props_member_mutation_computed`
- [x] `props_renamed_member_update_computed`
- [x] `runes_prop_member_update_in_template`
- [x] `runes_prop_member_compound_in_template`
- [x] `props_const_destructured_with_default`
- [x] `diagnose_props_identifier_in_snippet_body`
- [x] `diagnose_props_default_identifier_prop_reference`
- [x] `diagnose_props_default_identifier_non_reactive`
- [x] `diagnose_runes_prop_export_specifier`
- [x] `runes_prop_export_specifier_alias`
- [x] `runes_prop_export_specifier_dev`
- [x] `runes_state_export_specifier`
- [x] `runes_raw_state_export_specifier`
- [x] `runes_local_let_export_specifier`
- [x] `runes_local_const_export_specifier`
- [x] `runes_derived_export_specifier`
- [x] e2e smoke: `smoke_runes_reactive_mutations_all` — covers every assignment + update operator (`=`, `+=`, `-=`, `++`, `--`, `++` prefix, `--` prefix, `&&=`, `||=`, `??=`) for runes prop / `$bindable` identifier and member targets — including static (`obj.x`), computed string (`obj["x"]`), computed dynamic (`obj[key]`), and deep chains (`obj.a.b.c.x`, `obj["a"]["b"]["c"]["x"]`, mixed `obj[k1].b[k2]`) plus optional-chain reads (`obj?.a?.b?.c?.x`) — in both script body and template expressions, alongside `$state`, `$state.raw`, store, and deep-store paths. The smoke also exercises every contextual reactive reference: `{#each}` items, `{#snippet}` params, `{@const}` aliases, `{#await}` resolved/error values. Companion `smoke_runes_declarator_gaps_all` (ignored) captures three declarator gaps tracked in debt.md: `var foo = $state(0)`, `let { rawProp } = $props()` (NonSource→Source upgrade on mutation), `$state.eager(0)`.
