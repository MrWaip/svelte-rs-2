# `<script module>` in Components

## Current state
- **Status**: 10/11 use cases done — one open ordering/identifier bug surfaced on 2026-04-08 diagnose.
- **`ast_type = 'module'` tracking**: split to owning specs — `$props`/`$props.id` in module → `props-bindable.md`; `$host` in module → `host-rune.md`; TLA and `$:` in module → out of scope here
- **Open bug (2026-04-08 diagnose)**: when `<script module>` declares exported `const` + `function` and the instance template uses a `{#snippet}` that references the module-exported function from its render call site, two problems show up:
  1. Module exports are hoisted to the very top of the output instead of being emitted between snippet `const` allocations and `var root_N = $.from_html(...)` template allocations (codegen ordering).
  2. The module-exported function `label` is wrapped in `$.get(label)(...)` at the snippet render call site, even though it is a plain function declaration in module scope — `is_getter` (or rune classification) is misclassifying module-level function exports as reactive getters.
  Isolated by `script_module_exports_ordering_with_snippets` (`#[ignore]`). Likely layer: codegen ordering + analyze (rune classification of module exports).
- **Next**: no action needed for `ast_type = 'module'` — see `host-rune.md`. For the ordering/getter bug, start at codegen `generate()` module-script stitching and analyze rune classification for module symbols.
- Last updated: 2026-04-08

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
- [ ] Module exports ordering when instance template uses snippets — module exports must sit between snippet `const` allocations and `var root_N = $.from_html(...)` template allocations; also a module-exported plain function must NOT be wrapped in `$.get(...)` at call sites in template scope (test: `script_module_exports_ordering_with_snippets`, `#[ignore]`, M)

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

## Test cases

- [x] `script_module_exports`
- [x] `script_module_runes`
- [x] `script_module_instance_ref`
- [x] `script_module_only`
- [x] `script_module_with_instance`
- [x] `script_module_export_specifiers`
- [x] `script_module_imports`
- [x] `script_module_empty`
- [ ] `script_module_exports_ordering_with_snippets` (`#[ignore]`)
