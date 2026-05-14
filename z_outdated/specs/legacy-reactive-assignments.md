# $: reactive assignments

## Current state
- **Working**: 24/25 use cases
- **Tests**: 17/17 e2e + 3/3 diagnostics green
- Last updated: 2026-05-19

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
- `$: [a, b] = source;`
- `$: ({ store } = source); // auto-subscribed store on LHS`
- `$: ({ a: renamed, nested: { deep } } = source); // object renaming + nested object`
- `$: [[a, b], [c, d]] = source; // nested arrays`
- `$: ({ users: [{ name: first }, { name: second }] } = source); // mixed object/array nesting`
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
- [x] Legacy `$:` dependency capture treats `$store` auto-subscriptions as deps — `$: x = fn($store)` lowers the deps thunk to `() => $store()` (calling the store-thunk getter), not collapsing it to a bare identifier read of the called function (test: `legacy_pre_effect_store_subscription_dep`)
- [x] Array-pattern destructuring on the LHS of a top-level `$:` assignment registers each pattern identifier as an implicit reactive local, so dependency capture wraps reads in `$.get(...)`, the assignment lowers to a `$.to_array(...)` setter wrapper around the RHS, and downstream auto-subscribed stores from those locals route through `$.store_get($.get(name), ...)` (test: `diagnose_legacy_reactive_array_destructure_with_store`)
- [x] Object-pattern destructuring on the LHS of a top-level `$:` assignment routes auto-subscribed store targets through `$.store_unsub($.set(sym, ...), "$sym", $$stores)`, symmetrically with the array form (test: `legacy_reactive_assignment_object_destructure_with_store`)
- [x] Nested destructuring on the LHS of a top-level `$:` assignment supports arbitrary mixed object/array nesting and object property renaming (`{ a: x }`); array patterns at any depth lower to `var $$array[_N] = $.to_array(parent, N)` declarations hoisted above leaf setters, and the destructure-assignment emit-shape decision (sequence vs IIFE) follows the reference rule `inserts.length > 0 || !is_identifier(rhs)` (test: `legacy_reactive_assignment_mixed_destructure`)
- [ ] Default values (`[a = 1] = src`, `({ a = 1 } = src)`) and rest patterns (`[...rest]`, `({ ...rest })`) on the LHS of a top-level `$:` assignment
- [x] Top-level `let` written only inside an arbitrarily deep nested function (e.g. arrow inside `onMount`'s arrow callback) and read inside the IIFE body of a `$:` statement promotes to `$.mutable_source(...)`. Declarator → `$.mutable_source(false)`; nested writes → `$.set(flag, true)`; reads inside the `$:` IIFE → `$.get(flag)`; the surrounding `$.legacy_pre_effect` deps include `() => $.get(flag)`. Currently the analyzer does not classify the binding as legacy state when the writes are buried beyond the direct `$:` RHS expression, so the declarator stays plain JS and the deps thunk collapses to `() => {}`. Sibling of line 75 (write reachable through `$:`-assigned name) but for writes never reachable from any `$:`-bound name. Layer: 3.A.2 `ReactivitySemantics` (legacy state classification). Test: `diagnose_legacy_iife_read_nested_write_promotes_state`.
- [x] Topological ordering of `$:` `legacy_pre_effect` emission must follow store-subscription write→read edges. `$: derived = $w * 2;` followed by `$: (() => { $w = 1; })();` reorders so the IIFE block (writer of `$w`) emits before the `derived` block (reader of `$w`), even though source order is the opposite. Currently the analyzer preserves source order, treating store-subscription writes inside a nested function as opaque. Distinct from line 46 (visible vs invisible dep edges for plain locals) — here the connecting binding is an auto-subscribed store and the writer is wrapped in an IIFE. Layer: 3.A.2 `ReactivitySemantics` (topological sort over `$:` dep graph). Test: `diagnose_legacy_pre_effect_store_write_read_topological_order`.
- [x] Destructuring LHS on a top-level `$:` assignment whose RHS is a plain identifier reuses that identifier as both the IIFE wrapper parameter and the `$.to_array(...)` first argument — `(($$value) => { var $$array = $.to_array($$value, N); … })(src)` must instead emit `((src) => { var $$array = $.to_array(src, N); … })(src)`. Reference `shared/assignments.js` sets `rhs = should_cache ? b.id('$$value') : value`, so when `value.type === 'Identifier'` the original identifier is used; we always cache to `$$value` regardless. Layer: transform (`state_legacy.rs::transform_legacy_destructuring` / wrapper construction); repro/test: `diagnose_legacy_reactive_destructure_identifier_rhs`; candidate specs: `legacy-reactive-assignments.md`; suggested spec: `legacy-reactive-assignments.md`.
- [x] When a `<script context="module">` is present alongside legacy `$:` statements in the instance script, the `$.legacy_pre_effect_reset()` call must only be emitted inside the component function (alongside the `$.legacy_pre_effect(...)` calls), not orphaned at module top level (test: `diagnose_legacy_pre_effect_reset_with_module_script`)
- [x] Object-pattern destructuring with a single leaf that routes through `$.store_unsub(...)` keeps the IIFE wrapper arrow as a block body — the single setter must not collapse to an expression-body arrow (`(($$value) => $.store_unsub(...))(...)`), since the reference always emits `{ … }` here. Transform `state_legacy.rs::transform_legacy_destructuring` builds the wrapper via `arrow_expr`, which auto-shortens 1-statement bodies to expression form; the IIFE wrapper path needs the block form unconditionally (test: `legacy_reactive_object_destructure_single_store_leaf`)
- [x] Legacy `$:` dependency capture skips identifiers occurring in TS type positions (`TSAsExpression`, `TSSatisfiesExpression`, `TSTypeAssertion`, `TSInstantiationExpression`, `TSTypeAnnotation`, `TSType` subtrees, generic type arguments) — `$: x = e as Pick<typeof data, 'k'>` does not emit `data` into the `legacy_pre_effect` deps thunk (test: `diagnose_legacy_reactive_ts_type_position_dep`)
- [x] Legacy `$:` dependency capture visits `SwitchCase.consequent` before `SwitchCase.test`, matching the reference scope-walker order, so imports referenced both as case discriminants and as return values land in the `legacy_pre_effect` deps sequence in reference order (test: `diagnose_legacy_pre_effect_dep_order_switch_externals`)
- [x] Legacy `$:` dependency capture registers a binding on its first walker encounter — including assignment LHS targets — so a local var written before a prop is later read appears before the prop in the `legacy_pre_effect` deps thunk; bindings that are only written (never read) inside the block are excluded (test: `diagnose_legacy_pre_effect_dep_order_lhs_write_before_prop_read`)

## Out of scope

- SSR behavior for legacy reactive statements
- Extending `$state` rune semantics beyond their existing client-side behavior
- Unowned legacy features outside `$:` reactive assignments

## Reference

- Reference compiler:
  - `original/docs/99-legacy/02-legacy-reactive-assignments.md`
  - `original/compiler/phases/1-parse/read/script.js`
  - `original/compiler/phases/1-parse/acorn.js`
  - `original/compiler/phases/scope.js`
  - `original/compiler/phases/2-analyze/index.js`
  - `original/compiler/phases/2-analyze/visitors/LabeledStatement.js`
  - `original/compiler/phases/3-transform/client/visitors/LabeledStatement.js`
  - `original/compiler/phases/3-transform/client/visitors/Program.js`
  - `original/compiler/phases/3-transform/client/visitors/shared/utils.js`
  - `original/compiler/phases/3-transform/client/visitors/shared/declarations.js`
  - `original/compiler/phases/3-transform/client/transform-client.js`
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
- [x] `legacy_pre_effect_store_subscription_dep`
- [x] `diagnose_legacy_reactive_ts_type_position_dep`
- [x] `diagnose_legacy_reactive_array_destructure_with_store`
- [x] `legacy_reactive_assignment_object_destructure_with_store`
- [x] `legacy_reactive_assignment_mixed_destructure`
- [x] `legacy_reactive_object_destructure_single_store_leaf`
- [x] `diagnose_legacy_pre_effect_dep_order_switch_externals`
- [x] `diagnose_legacy_pre_effect_dep_order_lhs_write_before_prop_read`
- [x] `diagnose_legacy_pre_effect_reset_with_module_script`
- [x] `diagnose_legacy_reactive_destructure_identifier_rhs`
- [x] `diagnose_legacy_iife_read_nested_write_promotes_state`
- [x] `diagnose_legacy_pre_effect_store_write_read_topological_order`

Diagnostics:
- [x] `validate_reactive_declaration_invalid_placement`
- [x] `validate_reactive_declaration_cycle`
- [x] `validate_reactive_declaration_module_script_dependency`
