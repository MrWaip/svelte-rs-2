# Legacy export let props

## Current state
- **Working**: 7/16 use cases
- **Tests**: 7/13 e2e compiler tests green (`svelte_options_runes_false_override`, `svelte_options_accessors_legacy`, `legacy_export_let_required`, `legacy_export_var_basic`, `legacy_export_specifier`, `legacy_export_specifier_alias`, `legacy_export_let_typed`); 6 remain `#[ignore]` because they need orthogonal infrastructure (immutable-mode `$.deep_read_state` template-effect wrappers, destructure helper codegen, `$.push`/`$.init` for member-mutation flow, $$sanitized_props / $$restProps lowering, child-component bind:value pass-through). 13 analyzer unit tests cover the classification surface.
- Last updated: 2026-04-26
- Use case 1 closed: every legacy bindable prop (`export let`/`export var`/`export { foo }`/`export { foo as bar }`/destructured leaves) is classified as `DeclarationSemantics::LegacyBindableProp(LegacyBindablePropSemantics { default_lowering, flags })`. `flags` is the precomputed `$.prop(...)` bitfield (`PROPS_IS_BINDABLE` always set; `PROPS_IS_IMMUTABLE` / `PROPS_IS_UPDATED` / `PROPS_IS_LAZY_INITIAL` derived from `script.immutable` / `script.accessors` / `is_mutated_any` / lazy-default). `$$props` / `$$restProps` identifier read sites carry `ReferenceSemantics::LegacyPropsIdentifierRead` / `LegacyRestPropsIdentifierRead`. Read/write/member-mutation sites for legacy bindable props reuse the runes `PropRead(Source)` / `PropMutation` / `PropSourceMemberMutationRoot` channels.
- Use case 2 partially closed: transform `process_legacy_export_props` lowers `export let foo` / `export var foo` / `export { foo }` / `export { foo as bar }` to `let foo = $.prop($$props, "foo", flags, default)`; ad-hoc destructure / sanitized props / `$.push` / immutable template-effect wrappers remain in their own use cases.
- Unified reactivity dependency status: satisfied. Remaining analyzer/materialization work should now build on the landed `ReactivitySemantics` model instead of adding a parallel legacy-prop semantic model.

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
- [ ] Destructured legacy prop exports treat leaf identifiers as prop names and lower path-based defaults through temporary/derived helpers like the reference compiler (test: `legacy_export_destructure`, `#[ignore]`, needs infrastructure)
- [ ] Legacy immutable mode still treats `export let` as a prop input and emits the `$.deep_read_state`/`$.untrack` template-effect wrappers around prop member reads; classification + push lowering already work via use cases 1–2, only the template-effect wrapping side remains (test: `svelte_options_immutable_legacy`, `#[ignore]`, moderate)
- [x] Legacy prop accessors expose getter/setter pairs for `export let` props when `accessors={true}` is enabled (test: `svelte_options_accessors_legacy`).
- [ ] Legacy `$$props` identifier/member reads lower through a sanitized props object that excludes `children`, `$$slots`, `$$events`, and `$$legacy` before downstream reads/spreads; current Rust emits raw `$$props` reads, omits the helper declaration, and can even skip the `$$props` function parameter when that is the only legacy special variable used (test: `legacy_props_basic`, `#[ignore]`, moderate)
- [ ] Legacy `$$restProps` lowers through `$.legacy_rest_props(...)` and excludes named legacy props declared with `export let`; current Rust never declares `$$sanitized_props` or `$$restProps` and instead leaves `...$$restProps` unresolved in output (test: `legacy_rest_props_basic`, `#[ignore]`, moderate)
- [ ] Runes mode rejects direct `$$props` usage with `legacy_props_invalid`; the diagnostic code exists in `svelte_diagnostics`, but current Rust reports no diagnostic (test: `validate_legacy_props_invalid_in_runes_mode`, `#[ignore]`, quick fix)
- [ ] Runes mode rejects direct `$$restProps` usage with `legacy_rest_props_invalid`; the diagnostic code exists in `svelte_diagnostics`, but current Rust reports no diagnostic (test: `validate_legacy_rest_props_invalid_in_runes_mode`, `#[ignore]`, quick fix)
- [ ] Runes-mode `export let` reports `legacy_export_invalid` before state-export diagnostics for both reassigned and non-reassigned `$state` exports (tests: `validate_state_invalid_export_for_reassigned_state`, `validate_state_invalid_export_for_reassigned_state_raw`, `validate_state_invalid_export_no_error_without_reassignment`, existing ignored diagnostic cases, moderate)
- [ ] Unused legacy props warn with `export_let_unused`, including the documented `= undefined` opt-out path for required-prop warnings; warning code exists in `svelte_diagnostics` but analyzer never emits it (test: `validate_export_let_unused`, `#[ignore]`, quick fix)

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

- [ ] `svelte_options_runes_false_override`
- [ ] `svelte_options_accessors_legacy`
- [ ] `svelte_options_immutable_legacy`
- [ ] `legacy_export_let_required`
- [ ] `legacy_export_var_basic`
- [ ] `legacy_export_specifier`
- [ ] `legacy_export_specifier_alias`
- [ ] `legacy_export_destructure`
- [ ] `legacy_props_basic`
- [ ] `legacy_rest_props_basic`
- [ ] `legacy_export_let_typed`
- [ ] `legacy_export_let_member_mutation`
- [ ] `legacy_export_let_bind_to_inner`

Diagnostic tests (`tasks/diagnostic_tests/cases/`):

- [ ] `props/validate_legacy_props_invalid_in_runes_mode`
- [ ] `props/validate_legacy_rest_props_invalid_in_runes_mode`
- [ ] `props/validate_export_let_unused`
- [ ] `runes/validate_state_invalid_export_for_reassigned_state`
- [ ] `runes/validate_state_invalid_export_for_reassigned_state_raw`
- [ ] `runes/validate_state_invalid_export_for_reassigned_state_export_specifier`
- [ ] `runes/validate_state_invalid_export_no_error_without_reassignment`
