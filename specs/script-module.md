# `<script module>` in Components

## Current state
- **Status**: COMPLETE — 10/10 use cases done, ROADMAP item moved to Done
- **`ast_type = 'module'` tracking**: split to owning specs — `$props`/`$props.id` in module → `props-bindable.md`; `$host` in module → `host-rune.md`; TLA and `$:` in module → out of scope here
- Last updated: 2026-04-04

## Source
- ROADMAP: `## <script module> in Components`

## Syntax variants

```svelte
<script module>...</script>
<script context="module">...</script>
<script module lang="ts">...</script>
<script context="module" lang="ts">...</script>
<script module></script>
```

## Use cases

- [x] Module-level named exports pass through as-is (`export const`, `export function`) (test: `script_module_exports`)
- [x] Module-level `$state` / `$derived` runes transform to `$.state()` / `$.derived()` at module level (test: `script_module_runes`)
- [x] Instance code can reference module-level variables (scope chain: instance parent = module) (test: `script_module_instance_ref`)
- [x] Module script only (no instance script) — component function still emitted, module code at top level (test: `script_module_only`)
- [x] Both module + instance scripts — module body before component function, instance body inside (test: `script_module_with_instance`)
- [x] Module-level export specifiers (`export { foo, bar }`) emit at top level (test: `script_module_export_specifiers`)
- [x] Module-level imports merge with runtime imports at top of output (test: `script_module_imports`)
- [x] Empty module script produces no extra output (test: `script_module_empty`)
- [x] `export default` in module script emits `module_illegal_default_export` diagnostic (unit tests in `svelte_analyze`)
- [x] Module-level variable declarations (non-export, non-rune) emit at top level (covered by `script_module_instance_ref`)

## Out of scope

- SSR codegen for module scripts
- Legacy mode (non-runes) `export let` prop semantics in module scripts
- `.svelte.js` standalone module files (already working via `generate_module()`)
- `$props()` in module script (invalid by design, no explicit compile-time guard in reference)
- Store subscriptions in module script (`store_invalid_subscription` diagnostic — separate feature)

## Reference

- Svelte reference:
  - `reference/compiler/phases/1-parse/read/script.js` — attribute parsing, `context` field
  - `reference/compiler/phases/1-parse/state/element.js` — `root.module` vs `root.instance` placement
  - `reference/compiler/phases/2-analyze/index.js` — scope hierarchy (module → instance → template), `ast_type` assignment
  - `reference/compiler/phases/2-analyze/visitors/ExportNamedDeclaration.js` — export validation by `ast_type`
  - `reference/compiler/phases/2-analyze/visitors/ExportDefaultDeclaration.js` — `module_illegal_default_export`
  - `reference/compiler/phases/3-transform/client/transform-client.js` — module AST walk, body merge (lines 502-514)
  - `reference/compiler/phases/3-transform/client/visitors/ExportNamedDeclaration.js` — pass-through for module exports
  - `reference/compiler/phases/3-transform/client/visitors/Program.js` — `is_instance` branching
- Our code:
  - `crates/svelte_ast/src/lib.rs` — `Script`, `ScriptContext`, `Component.module_script`
  - `crates/svelte_parser/src/scanner/mod.rs` — `is_module` detection
  - `crates/svelte_parser/src/lib.rs` — module/instance dispatch, duplicate detection
  - `crates/svelte_parser/src/walk_js.rs` — `module_program` parsing
  - `crates/svelte_parser/src/types.rs` — `ParserResult.module_program`, `module_script_content_span`
  - `crates/svelte_codegen_client/src/lib.rs` — `generate()` (gap: doesn't process module script), `generate_module()` (works for .svelte.js)
  - `crates/svelte_codegen_client/src/script/pipeline.rs` — `transform_module_script()` (exists but not called from `generate()`)
  - `crates/svelte_analyze/src/validate/mod.rs` — only snippet-export validation on module_program

## Tasks

### Analysis
- [x] Run analysis passes over `module_program` in component context, with module bindings merged into component scoping through a real parent scope chain
- [x] Add `module_illegal_default_export` diagnostic for `export default` in module script
- [~] `ast_type = 'module'` context tracking — split to owning specs: `props-bindable.md` (`$props`/`$props.id`) and `host-rune.md` (`$host`); TLA + `$:` in module are out of scope

### Codegen
- [x] Emit component module scripts from `generate()` using a component-safe passthrough transform path
- [x] Merge module body into output: imports lifted to top, non-import statements before component function
- [x] Ensure runtime import (`import * as $ from 'svelte/internal/client'`) merges correctly with module imports
- [x] Add explicit e2e coverage for module export specifiers and empty-module output

## Implementation order

1. Codegen first — `transform_module_script()` already exists, wire it into `generate()` and merge output
2. Analysis — add module program processing for diagnostics and scoping
3. Diagnostics — `module_illegal_default_export` and related validations
