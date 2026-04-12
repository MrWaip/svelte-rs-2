# Legacy export let props

## Current state
- **Working**: 3/10 use cases
- **Completed (2026-04-12)**: added five focused compiler parity cases for required `export let`, `export var`, export specifier props, aliased export specifier props, and destructured legacy export props; all currently fail and are marked ignored in `tasks/compiler_tests/test_v3.rs`
- **Confirmed gap (2026-04-12)**: `collect_legacy_export_props` only seeds legacy props from `export let` binding identifiers, and `post_resolve` only marks legacy props as prop sources when they have defaults or mutations, so required legacy props still read raw `$$props` while `var` / export-specifier / destructured forms stay plain exports or locals
- **Confirmed gap (2026-04-12)**: runes-mode `export let` diagnostics still mismatch reference parity; existing focused diagnostic cases expect `legacy_export_invalid`, but Rust currently reports `state_invalid_export` or nothing
- **Next**: promote required legacy props and export-specifier legacy props into the same prop-source pipeline before touching destructured exports
- Last updated: 2026-04-12

## Source

ROADMAP.md — Legacy Svelte 4: `export let` props

## Syntax variants

- `<script>export let foo;</script>`
- `<script>export let bar = 'default value';</script>`
- `<script>export let foo = undefined;</script>`
- `<script>export var count = 1;</script>`
- `<script>let foo = 1; export { foo };</script>`
- `<script>let className; export { className as class };</script>`
- `<script>export let { x: foo, z: [bar] } = expr;</script>`
- `<svelte:options accessors={true} /><script>export let count = 1;</script>`

## Use cases

- [x] Explicit legacy mode with a defaulted `export let` lowers through the prop pipeline instead of staying a plain export (test: `svelte_options_runes_false_override`)
- [x] Legacy prop accessors expose getter/setter pairs for `export let` props when `accessors={true}` is enabled (test: `svelte_options_accessors_legacy`)
- [x] Legacy immutable mode still treats `export let` as a prop input and keeps the runtime-plan push/init behavior aligned with reference output (test: `svelte_options_immutable_legacy`)
- [ ] Required legacy props without defaults still lower through `$.prop(...)` and template getter calls rather than reading raw `$$props` (test: `legacy_export_let_required`, `#[ignore]`, moderate)
- [ ] `export var` declarations become legacy bindable props instead of plain mutable exports (test: `legacy_export_var_basic`, `#[ignore]`, moderate)
- [ ] Separate instance-script export specifiers on `let` bindings promote those bindings to legacy props rather than component exports (test: `legacy_export_specifier`, `#[ignore]`, moderate)
- [ ] Export-specifier aliases use the exported name as the prop key while keeping the local binding inside the component (`export { className as class }`) (test: `legacy_export_specifier_alias`, `#[ignore]`, moderate)
- [ ] Destructured legacy prop exports treat leaf identifiers as prop names and lower path-based defaults through temporary/derived helpers like the reference compiler (test: `legacy_export_destructure`, `#[ignore]`, needs infrastructure)
- [ ] Runes-mode `export let` reports `legacy_export_invalid` before state-export diagnostics for both reassigned and non-reassigned `$state` exports (tests: `validate_state_invalid_export_for_reassigned_state`, `validate_state_invalid_export_for_reassigned_state_raw`, `validate_state_invalid_export_no_error_without_reassignment`, existing ignored diagnostic cases, moderate)
- [ ] Unused legacy props warn with `export_let_unused`, including the documented `= undefined` opt-out path for required-prop warnings (diagnostic coverage still missing from this audit run because of the 5-case cap, quick fix)

## Out of scope

- SSR output for legacy props
- `$$props` / `$$restProps` / `$$slots` legacy compatibility
- Component API exports from `export const`, `export function`, and `export class`

## Reference

Svelte:
- `reference/docs/99-legacy/03-legacy-export-let.md`
- `reference/compiler/phases/1-parse/read/options.js`
- `reference/compiler/phases/2-analyze/index.js`
- `reference/compiler/phases/2-analyze/visitors/ExportNamedDeclaration.js`
- `reference/compiler/phases/2-analyze/visitors/ExportSpecifier.js`
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
- `crates/svelte_codegen_client/src/script/model.rs`
- `crates/svelte_codegen_client/src/script/props.rs`
- `crates/svelte_codegen_client/src/script/pipeline.rs`
- `crates/svelte_codegen_client/src/script/traverse/statement_passes.rs`
- `crates/svelte_analyze/src/tests.rs`
- `tasks/compiler_tests/test_v3.rs`

## Test cases

- [x] `svelte_options_runes_false_override`
- [x] `svelte_options_accessors_legacy`
- [x] `svelte_options_immutable_legacy`
- [ ] `legacy_export_let_required`
- [ ] `legacy_export_var_basic`
- [ ] `legacy_export_specifier`
- [ ] `legacy_export_specifier_alias`
- [ ] `legacy_export_destructure`
- [ ] `validate_state_invalid_export_for_reassigned_state`
- [ ] `validate_state_invalid_export_for_reassigned_state_raw`
- [ ] `validate_state_invalid_export_no_error_without_reassignment`
