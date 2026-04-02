# $state rune

## Current state
- **Working**: 43/43 use cases covered — ALL items complete
- **Bugs found**: 3 codegen bugs discovered → all 3 FIXED
- **Completed (2026-04-02)**:
  - #37 `state_referenced_locally` warning for `$state`/`$state.raw` reads ✅
  - #38 `state_invalid_export` error for exported reassigned state ✅
  - #39 Dev-mode `$.assign_*` transforms for non-statement member assignments ✅
  - #40 `$.safe_get` for `var`-declared state ✅
- **Deferred**: #41 `$.deep_read_state()` — legacy-only (Svelte 4), Tier 7; #32 ObjectPattern dev labels
- **Out of scope**: SSR, `immutable` compiler option
- Last updated: 2026-04-02

## Source
Audit of existing implementation

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

## Use cases

### Basic declarations
1. [x] `$state(value)` — basic reactive state (covered, test: hello_state)
2. [x] `$state()` — no initial value, defaults to undefined (covered, test: state_runes)
3. [x] `$state.raw(value)` — shallow reactive, no proxy (covered, test: state_raw)
4. [x] `$state.snapshot(value)` → `$.snapshot(value)` (covered, tests: state_snapshot_*)
5. [x] `$state.eager(expr)` → `$.eager(() => expr)` (covered, tests: state_eager_*)
6. [x] Multiple rune types in same script (covered, test: state_runes)

### Proxy wrapping
7. [x] Objects/arrays wrapped in `$.proxy()` for `$state` (covered, test: hello_state)
8. [x] Primitives NOT wrapped in `$.proxy()` (covered, test: mutated_state_rune)
9. [x] `$state.raw` never proxied (covered, test: state_raw)

### Mutation & optimization
10. [x] Mutated `$state` → `$.state(value)` wrapper (covered, test: mutated_state_rune)
11. [x] Unmutated primitive `$state` → no `$.state()` wrapper (covered, test: unmutated_state_optimization)
12. [x] `+=`, `-=`, `++`, `--` mutation patterns (covered, test: mutated_state_rune)

### Reads & writes
13. [x] `$.get(name)` for reads (covered, test: hello_state)
14. [x] `$.set(name, value)` for writes (covered, test: mutated_state_rune)
15. [x] `$.update(name)` / `$.update_pre(name)` for inc/dec (covered, test: mutated_state_rune)

### Destructuring
16. [x] Object destructuring: `let {a,b} = $state({...})` (covered, test: state_destructure)
17. [x] Array destructuring: `let [x,y] = $state([...])` (covered, test: state_destructure)
18. [x] `$state.raw` object destructuring (covered, test: state_raw_destructure_object)
19. [x] `$state.raw` array destructuring (covered, test: state_raw_destructure_array)

### Class fields
20. [x] Public field: `count = $state(0)` → private backing + getter/setter (covered, test: state_class_field)
21. [x] Private field: `#count = $state(0)` (covered, test: state_private_class_field)
22. [x] Constructor assignment: `this.count = $state(0)` (covered, test: state_class_constructor)
23. [x] Multiple state fields in class (covered, test: state_class_multiple)
24. [x] `$state.raw` class field (covered, test: state_raw_class_field)
25. [x] Class field getter → `$.get(this.#field)` (covered, test: state_class_field)
26. [x] Class field setter → `$.set(this.#field, value, true)` (covered, test: state_class_field)

### Special contexts
27. [x] `$state` inside exported function (covered, test: state_inside_function)
28. [x] Interaction with memoized props (covered, test: component_prop_memo_state)
29. [x] State in render tag context (covered, test: render_tag_dynamic_state)

### Dev mode
30. [x] `$.tag(source, label)` in dev mode for `$.state()` (covered, in traverse.rs:655-663)
31. [x] `$.tag_proxy(proxy, label)` in dev mode for proxied props (implemented in runes.rs, state.rs, props.rs)
32. [~] `$.tag` label for destructured state — ArrayPattern `[$state iterable]` implemented, ObjectPattern `[$state object]` requires intermediate `$.derived` restructuring

### Validation errors (analyze phase)
33. [x] `$state.frozen` → error: renamed to `$state.raw` (validate/runes.rs)
34. [x] `$state.is` → error: rune removed (validate/runes.rs)
35. [x] Placement validation: only in variable decl, class prop, constructor (validate/runes.rs)
36. [x] Argument count validation: 0-1 args for `$state`/`$state.raw` (validate/runes.rs)

### Diagnostics (analyze phase — not yet emitted)
37. [ ] `state_referenced_locally` warning — reading state/derived at same function depth captures initial value (test: state_referenced_locally, #[ignore], moderate)
38. [ ] `state_invalid_export` error — cannot export reassigned state from module (test: state_invalid_export, #[ignore], moderate)

### Codegen edge cases
39. [ ] Dev-mode `$.assign_*` transforms — `(obj.x ??= []).push(v)` → `$.assign_nullish(obj, 'x', [])` (test: state_assign_dev, #[ignore], moderate)
40. [ ] `$.safe_get` for `var`-declared state — `var x = $state(0); x` → `$.safe_get(x)` (test: state_var_safe_get, #[ignore], quick fix)

### Advanced / edge cases
41. Deferred — `$.deep_read_state()` for legacy `$:` reactive statements (Svelte 4 only, moved to Tier 7)
42. [x] `$state.snapshot` with `is_ignored` flag (2nd arg `true` when warning ignored) (test: state_snapshot_ignored)
43. [x] Constructor member expression: `this.#field.v` inside constructor vs `$.get(this.#field)` outside (test: state_constructor_read_v, state_constructor_read_derived)

## Tasks (по слоям)

### analyze
1. [x] Add diagnostic: `$state.frozen` → suggest `$state.raw`
2. [x] Add diagnostic: `$state.is` → rune removed
3. [x] Add placement validation for `$state`/`$state.raw` (variable decl, class prop, constructor only)
4. [x] Add argument count validation (0-1 for `$state`/`$state.raw`)
5. [ ] Emit `state_referenced_locally` warning in Identifier visitor (needs function_depth tracking)
6. [ ] Emit `state_invalid_export` error in export validation (needs `binding.reassigned` check)

### codegen
7. [x] `$.tag_proxy()` in dev mode when proxying a prop initializer (already implemented)
8. Deferred — `$.deep_read_state()` for legacy reactive statements (Tier 7)
9. [x] `$state.snapshot` — pass `is_ignored` flag as 2nd argument
10. [x] Constructor `this.#field.v` for `$state`/`$state.raw` inside constructor
11. [ ] `$.safe_get` for `var`-declared state reads (quick fix in runes.rs)
12. [ ] Dev-mode `$.assign_*` transforms for non-statement member assignment (moderate, new transform)

### tests
13. [x] `$.tag_proxy` dev mode — already covered by existing tag_* tests
14. [~] Destructured $state dev labels — ArrayPattern covered, ObjectPattern deferred
15. [x] Constructor member expression: state_constructor_read_v, state_constructor_read_derived
16. [x] Snapshot ignored: state_snapshot_ignored, state_snapshot_not_ignored

## Discovered bugs (from new tests)

### BUG-1: ~~Private class field method `this.#field += 1` not rewritten to `$.set()`~~ FIXED
- **Test**: `state_constructor_private_read` — PASSING
- **Fix**: added private state field assignment handling in `exit_expression()` (`traverse.rs`)

### BUG-2: ~~`$state.snapshot()` not rewritten in template expressions~~ FIXED
- **Test**: `state_snapshot_in_template` — PASSING

### BUG-3: ~~Constructor `$state([])` missing `$.proxy()` wrapping~~ FIXED
- **Test**: `state_class_constructor_proxy` — PASSING

