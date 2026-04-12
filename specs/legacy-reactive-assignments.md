# $: reactive assignments

## Current state
- **Working**: 0/10 use cases
- **Tests**: 0/5 green
- Last updated: 2026-04-12

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

- [ ] Analyzer materializes dedicated legacy reactive declaration entities from top-level `$:` statements, capturing dependencies, assignments, statement kind, and implicit reactive targets instead of leaving them as raw JS `LabeledStatement`s for downstream rediscovery (test: none yet, needs infrastructure)
- [ ] Top-level legacy `$:` statements and assignments in instance scripts are discovered and lowered to client-side `$.legacy_pre_effect(...)` calls, with backing `$.mutable_source(...)` declarations for implicitly introduced reactive targets (test: `legacy_reactive_assignment_basic`, `#[ignore]`, needs infrastructure)
- [ ] Legacy `$:` dependency capture treats top-level declared legacy `let` / `var` locals as reactive state sources, so dependency thunks and assignment bodies read them through `$.get(...)` / `$.safe_get(...)` instead of plain identifiers (test: `legacy_reactive_assignment_declared_dependency`, `#[ignore]`, needs infrastructure)
- [ ] Legacy `$:` block bodies and destructuring assignment targets participate in the same dependency and implicit-binding flow as simple assignments (test: `legacy_reactive_assignment_block_destructure`, `#[ignore]`, needs infrastructure)
- [ ] Legacy `$:` dependency capture uses coarse-grained reads for legacy prop sources and reserved prop bags (`export let`, `$$props`, `$$restProps`) instead of plain identifier reads (test: `legacy_reactive_assignment_coarse_deps`, `#[ignore]`, needs infrastructure)
- [ ] Downstream legacy `$:` assignments are emitted in topological order, and mutated instance imports use the legacy reactive-import wrapper when they participate in `$:` dependencies (test: `legacy_reactive_assignment_import_topology`, `#[ignore]`, needs infrastructure)
- [ ] Compile-time dependency capture remains intentionally shallow for indirect calls, so `$: doubled = double()` does not subscribe to `count` when `double` closes over it; this needs explicit parity coverage because the reference docs call it out as a non-obvious legacy limitation (test: none yet, moderate)
- [ ] Topological ordering only follows visible dependency edges, so indirect writes like `$: z = y; $: setY(x);` preserve the reference compiler's documented non-update behavior until source order is changed (test: none yet, moderate)
- [ ] Validation emits `reactive_declaration_invalid_placement` when `$:` appears outside top-level instance script, rather than treating nested labeled statements as reactive declarations (test: existing diagnostic coverage not yet added to this spec, moderate)
- [ ] Validation emits `reactive_declaration_module_script_dependency` when a reactive statement depends on reassigned module-script state, and emits `reactive_declaration_cycle` for cyclic reactive assignment graphs (test: existing diagnostic coverage not yet added to this spec, moderate)

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

- [ ] `legacy_reactive_assignment_basic`
- [ ] `legacy_reactive_assignment_declared_dependency`
- [ ] `legacy_reactive_assignment_block_destructure`
- [ ] `legacy_reactive_assignment_coarse_deps`
- [ ] `legacy_reactive_assignment_import_topology`
