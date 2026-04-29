# $: reactive assignments

## Current state
- **Working**: 12/12 use cases
- **Tests**: 5/5 e2e + 3/3 diagnostics green
- Last updated: 2026-04-29

## Source

- ROADMAP item: `Legacy Svelte 4 -> $: reactive assignments`
- Moved out of `specs/state-rune.md` during spec normalization on 2026-04-12

## Syntax variants

- `$: doubled = count * 2;`
- `let count = 0; $: doubled = count * 2;`
- `var step = 1; $: total = doubled + step;`
- `$: console.log(items.length);`
- `$: doubled = double();`
- `$: z = y; $: setY(x);`
- `$: if (condition) { total = a + b; } else { total = 0; }`
- `$: switch (condition) { case 'a': value = 1; break; default: value = 0; }`
- `$: ((param) => { console.log(param); })(reactiveVariable);`
- `$: { total = 0; for (const item of items) total += item.value; }`
- `$: ({ value } = source);`
- `function fn() { $: value = count; }`
- `<script context="module">let shared = 0;</script><script>$: total = shared;</script>`
- `$: a = b; $: b = a;`

## Use cases

- [x] Analyzer materializes dedicated legacy reactive declaration entities from top-level `$:` statements, capturing dependencies, assignments, statement kind, and implicit reactive targets instead of leaving them as raw JS `LabeledStatement`s for downstream rediscovery (test: analyze unit tests in `crates/svelte_analyze/src/tests.rs`)
- [x] Top-level legacy `$:` statements and assignments in instance scripts are discovered and lowered to client-side `$.legacy_pre_effect(...)` calls, with backing `$.mutable_source(...)` declarations for implicitly introduced reactive targets (test: `legacy_reactive_assignment_basic`)
- [x] Legacy `$:` dependency capture treats top-level declared legacy `let` / `var` locals as reactive state sources, so dependency thunks and assignment bodies read them through `$.get(...)` / `$.safe_get(...)` instead of plain identifiers (test: `legacy_reactive_assignment_declared_dependency`)
- [x] Legacy `$:` block bodies and destructuring assignment targets participate in the same dependency and implicit-binding flow as simple assignments (test: `legacy_reactive_assignment_block_destructure`)
- [x] `export let` props with simple defaults and no template references promote to `LegacyBindableProp` and emit as `$.prop($$props, name, BINDABLE, default)` rather than degrading to plain `let` with `$$exports` accessor wrapper (test: covered via `legacy_reactive_assignment_coarse_deps`)
- [x] `$$props` and `$$restProps` AST identifiers in instance script body are rewritten to `$$sanitized_props` / `$$restProps` runtime constants, with `legacy_rest_props` bootstrap declarations emitted at the top of the component function (test: subset of `legacy_reactive_assignment_coarse_deps`)
- [x] Legacy `$:` dependency capture uses coarse-grained reads for `LegacyBindableProp` (`$.deep_read_state(name())`), `$$props` (`$.deep_read_state($$sanitized_props)`), and `$$restProps` (`$.deep_read_state($$restProps)`) instead of fine-grained identifier reads (test: `legacy_reactive_assignment_coarse_deps`)
- [x] Downstream legacy `$:` assignments are emitted in topological order, and mutated instance imports use the legacy reactive-import wrapper when they participate in `$:` dependencies (test: `legacy_reactive_assignment_import_topology`)
- [x] Compile-time dependency capture remains intentionally shallow for indirect calls, so `$: doubled = double()` does not subscribe to `count` when `double` closes over it; this needs explicit parity coverage because the reference docs call it out as a non-obvious legacy limitation (test: `legacy_reactive_indirect_call_does_not_subscribe_to_closure_state`)
- [x] Topological ordering only follows visible dependency edges, so indirect writes like `$: z = y; $: setY(x);` preserve the reference compiler's documented non-update behavior until source order is changed (test: `legacy_reactive_indirect_write_preserves_source_order`)
- [x] Validation emits `reactive_declaration_invalid_placement` when `$:` appears outside top-level instance script, rather than treating nested labeled statements as reactive declarations (test: `validate_reactive_declaration_invalid_placement`)
- [x] Validation emits `reactive_declaration_module_script_dependency` when a reactive statement depends on reassigned module-script state, and emits `reactive_declaration_cycle` for cyclic reactive assignment graphs (test: `validate_reactive_declaration_module_script_dependency`, `validate_reactive_declaration_cycle`)
- [x] Implicit reactive locals introduced by direct `$:` assignments are materialized into a `LegacyReactivitySemantics::is_implicit_reactive_local(sym)` set; nested-function writes (e.g. inside arrow IIFE inside `$:`) intentionally stay unresolved globals (test: `legacy_reactive_skips_implicit_decl_in_nested_iife`, `legacy_reactive_marks_implicit_reactive_local`)

## Out of scope

- SSR behavior for legacy reactive statements
- Extending `$state` rune semantics beyond their existing client-side behavior
- Unowned legacy features outside `$:` reactive assignments

## Reference

- Reference compiler:
  - `reference/docs/99-legacy/02-legacy-reactive-assignments.md`
  - `reference/compiler/phases/1-parse/read/script.js`
  - `reference/compiler/phases/1-parse/acorn.js`
  - `reference/compiler/phases/scope.js`
  - `reference/compiler/phases/2-analyze/index.js`
  - `reference/compiler/phases/2-analyze/visitors/LabeledStatement.js`
  - `reference/compiler/phases/3-transform/client/visitors/LabeledStatement.js`
  - `reference/compiler/phases/3-transform/client/visitors/Program.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/utils.js`
  - `reference/compiler/phases/3-transform/client/visitors/shared/declarations.js`
  - `reference/compiler/phases/3-transform/client/transform-client.js`
- Existing Rust behavior:
  - `crates/svelte_codegen_client/src/template/expression.rs`
  - `crates/svelte_diagnostics/src/lib.rs`
  - `tasks/compiler_tests/test_v3.rs`

## Test cases

E2E:
- [x] `legacy_reactive_assignment_basic`
- [x] `legacy_reactive_assignment_declared_dependency`
- [x] `legacy_reactive_assignment_block_destructure`
- [x] `legacy_reactive_assignment_coarse_deps`
- [x] `legacy_reactive_assignment_import_topology`

Diagnostics:
- [x] `validate_reactive_declaration_invalid_placement`
- [x] `validate_reactive_declaration_cycle`
- [x] `validate_reactive_declaration_module_script_dependency`
