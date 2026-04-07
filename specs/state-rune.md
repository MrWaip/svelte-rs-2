# $state rune

## Current state
- **Working**: 40/43 use cases
- **Bugs found**: 3 codegen bugs discovered → all 3 FIXED
- **Completed (2026-04-02)**:
  - #37 `state_referenced_locally` warning for `$state`/`$state.raw` reads
  - #38 `state_invalid_export` error for exported reassigned state
  - #39 Dev-mode `$.assign_*` transforms for non-statement member assignments
  - #40 `$.safe_get` for `var`-declared state
- **Missing**: #41 `$.deep_read_state()` (legacy, Tier 7); #32 ObjectPattern dev labels
- Last updated: 2026-04-02

## Source
Audit of existing implementation

## Use cases

- [x] `$state(value)` — basic reactive state (covered, test: hello_state)
- [x] `$state()` — no initial value, defaults to undefined (covered, test: state_runes)
- [x] `$state.raw(value)` — shallow reactive, no proxy (covered, test: state_raw)
- [x] `$state.snapshot(value)` → `$.snapshot(value)` (covered, tests: state_snapshot_*)
- [x] `$state.eager(expr)` → `$.eager(() => expr)` (covered, tests: state_eager_*)
- [x] Multiple rune types in same script (covered, test: state_runes)
- [x] Objects/arrays wrapped in `$.proxy()` for `$state` (covered, test: hello_state)
- [x] Primitives NOT wrapped in `$.proxy()` (covered, test: mutated_state_rune)
- [x] `$state.raw` never proxied (covered, test: state_raw)
- [x] Mutated `$state` → `$.state(value)` wrapper (covered, test: mutated_state_rune)
- [x] Unmutated primitive `$state` → no `$.state()` wrapper (covered, test: unmutated_state_optimization)
- [x] `+=`, `-=`, `++`, `--` mutation patterns (covered, test: mutated_state_rune)
- [x] `$.get(name)` for reads (covered, test: hello_state)
- [x] `$.set(name, value)` for writes (covered, test: mutated_state_rune)
- [x] `$.update(name)` / `$.update_pre(name)` for inc/dec (covered, test: mutated_state_rune)
- [x] Object destructuring: `let {a,b} = $state({...})` (covered, test: state_destructure)
- [x] Array destructuring: `let [x,y] = $state([...])` (covered, test: state_destructure)
- [x] `$state.raw` object destructuring (covered, test: state_raw_destructure_object)
- [x] `$state.raw` array destructuring (covered, test: state_raw_destructure_array)
- [x] Public field: `count = $state(0)` → private backing + getter/setter (covered, test: state_class_field)
- [x] Private field: `#count = $state(0)` (covered, test: state_private_class_field)
- [x] Constructor assignment: `this.count = $state(0)` (covered, test: state_class_constructor)
- [x] Multiple state fields in class (covered, test: state_class_multiple)
- [x] `$state.raw` class field (covered, test: state_raw_class_field)
- [x] Class field getter → `$.get(this.#field)` (covered, test: state_class_field)
- [x] Class field setter → `$.set(this.#field, value, true)` (covered, test: state_class_field)
- [x] `$state` inside exported function (covered, test: state_inside_function)
- [x] Interaction with memoized props (covered, test: component_prop_memo_state)
- [x] State in render tag context (covered, test: render_tag_dynamic_state)
- [x] `$.tag(source, label)` in dev mode for `$.state()` (covered, in traverse.rs:655-663)
- [x] `$.tag_proxy(proxy, label)` in dev mode for proxied props (implemented in runes.rs, state.rs, props.rs)
- [ ] `$.tag` label for destructured state — ArrayPattern `[$state iterable]` is implemented, but ObjectPattern `[$state object]` still requires intermediate `$.derived` restructuring
- [x] `$state.frozen` → error: renamed to `$state.raw` (validate/runes.rs)
- [x] `$state.is` → error: rune removed (validate/runes.rs)
- [x] Placement validation: only in variable decl, class prop, constructor (validate/runes.rs)
- [x] Argument count validation: 0-1 args for `$state`/`$state.raw` (validate/runes.rs)
- [x] `state_referenced_locally` warning — reading state/derived at same function depth captures initial value (covered by analyzer tests: `validate_state_referenced_locally_*`)
- [x] `state_invalid_export` error — cannot export reassigned state from module (covered by analyzer tests: `validate_state_invalid_export_*`)
- [x] Dev-mode `$.assign_*` transforms — `(obj.x ??= []).push(v)` → `$.assign_nullish(obj, 'x', [])` (covered, test: state_assign_dev)
- [x] `$.safe_get` for `var`-declared state — `var x = $state(0); x` → `$.safe_get(x)` (covered, test: state_var_safe_get)
- [x] `$state.snapshot` with `is_ignored` flag (2nd arg `true` when warning ignored) (test: state_snapshot_ignored)
- [x] Constructor member expression: `this.#field.v` inside constructor vs `$.get(this.#field)` outside (test: state_constructor_read_v, state_constructor_read_derived)

## Out of scope

- `$.deep_read_state()` for legacy `$:` reactive statements (Svelte 4, Tier 7)

## Reference
- Svelte reference:
  - `reference/compiler/phases/2-analyze/visitors/VariableDeclarator.js` — binding.kind assignment
  - `reference/compiler/phases/2-analyze/visitors/CallExpression.js` — placement validation
  - `reference/compiler/phases/2-analyze/visitors/Identifier.js` — deprecation/removal errors
  - `reference/compiler/phases/3-transform/client/visitors/CallExpression.js` — $.state/$.proxy generation
  - `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js` — variable wrapping, destructuring
  - `reference/compiler/phases/3-transform/client/visitors/ClassBody.js` — class field transforms
  - `reference/compiler/phases/3-transform/client/visitors/MemberExpression.js` — private field reads
  - `reference/compiler/phases/3-transform/client/visitors/AssignmentExpression.js` — $.set generation
  - `reference/compiler/phases/3-transform/server/visitors/CallExpression.js` — server unwrap

- Our code:
  - `crates/svelte_analyze/src/types/script.rs` — RuneKind enum
  - `crates/svelte_analyze/src/utils/script_info.rs` — detect_rune_from_call()
  - `crates/svelte_analyze/src/scope.rs` — Rune struct, mutation tracking
  - `crates/svelte_codegen_client/src/script/state.rs` — class field transforms, destructuring
  - `crates/svelte_codegen_client/src/script/traverse.rs` — variable declaration transforms
  - `crates/svelte_transform/src/rune_refs.rs` — should_proxy(), runtime helper builders

## Test cases

- [x] `hello_state`
- [x] `state_runes`
- [x] `state_raw`
- [x] `state_snapshot_ignored`
- [x] `state_snapshot_not_ignored`
- [x] `state_eager_basic`
- [x] `mutated_state_rune`
- [x] `unmutated_state_optimization`
- [x] `state_destructure`
- [x] `state_raw_destructure_object`
- [x] `state_raw_destructure_array`
- [x] `state_class_field`
- [x] `state_private_class_field`
- [x] `state_class_constructor`
- [x] `state_class_multiple`
- [x] `state_raw_class_field`
- [x] `state_inside_function`
- [x] `component_prop_memo_state`
- [x] `render_tag_dynamic_state`
- [x] `state_assign_dev`
- [x] `state_var_safe_get`
- [x] `state_constructor_read_v`
- [x] `state_constructor_read_derived`
- [x] `validate_state_referenced_locally_basic`
- [x] `validate_state_invalid_export_basic`
