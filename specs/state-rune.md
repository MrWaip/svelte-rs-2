# $state rune

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
31. [ ] `$.tag_proxy(proxy, label)` in dev mode for proxied props (missing)
32. [~] `$.tag` label for destructured state — reference uses `[$state iterable]`/`[$state object]` labels (unknown — needs test)

### Validation errors (analyze phase)
33. [ ] `$state.frozen` → error: renamed to `$state.raw` (missing — no diagnostic)
34. [ ] `$state.is` → error: rune removed (missing — no diagnostic)
35. [ ] Placement validation: only in variable decl, class prop, constructor (missing — no diagnostic)
36. [ ] Argument count validation: 0-1 args for `$state`/`$state.raw` (missing — no diagnostic)

### Server-side transform
37. [ ] `$state(value)` → `value` on server (missing — no server codegen)
38. [ ] `$state.raw(value)` → `value` on server (missing — no server codegen)
39. [ ] `$state.snapshot(value)` → `$.snapshot(value)` on server (missing — no server codegen)
40. [ ] Server class body: no getter/setter generation (missing — no server codegen)

### Advanced / edge cases
41. [ ] `$.deep_read_state()` for bindable props in reactive statements (missing)
42. [ ] `$state.snapshot` with `is_ignored` flag (2nd arg `true` when warning ignored) (missing)
43. [~] Constructor member expression: `this.#field.v` inside constructor vs `$.get(this.#field)` outside (unknown — needs verification)

## Tasks (по слоям)

### analyze
1. [ ] Add diagnostic: `$state.frozen` → suggest `$state.raw`
2. [ ] Add diagnostic: `$state.is` → rune removed
3. [ ] Add placement validation for `$state`/`$state.raw` (variable decl, class prop, constructor only)
4. [ ] Add argument count validation (0-1 for `$state`/`$state.raw`)

### codegen
5. [ ] `$.tag_proxy()` in dev mode when proxying a prop initializer
6. [ ] `$.deep_read_state()` for bindable props in reactive statements
7. [ ] `$state.snapshot` — pass `is_ignored` flag as 2nd argument

### server codegen (deferred — no server codegen crate yet)
8. [ ] Server-side `$state`/`$state.raw` → unwrap to plain value
9. [ ] Server-side class body handling
10. [ ] Server-side `$state.snapshot` → `$.snapshot()`

### tests
11. [ ] Add test: `$state` dev mode tag_proxy
12. [ ] Add test: destructured $state dev labels
13. [ ] Add test: constructor member expression context (this.#field.v vs $.get)

## Discovered bugs (from new tests)

### BUG-1: Private class field method `this.#field += 1` not rewritten to `$.set()`
- **Test**: `state_constructor_private_read`
- **Expected**: `$.set(this.#elapsed, $.get(this.#elapsed) + 1)`
- **Got**: `this.#elapsed += 1` (raw compound assignment, not reactive)
- **Layer**: codegen — class method body compound assignment to private state field

### BUG-2: `$state.snapshot()` not rewritten in template expressions
- **Test**: `state_snapshot_in_template`
- **Expected**: `$.snapshot(items)` in template expression
- **Got**: `$state.snapshot(items)` — raw rune call passed through verbatim
- **Layer**: codegen — template expression rune call rewriting

### BUG-3: Constructor `$state([])` missing `$.proxy()` wrapping
- **Test**: `state_class_constructor_proxy`
- **Expected**: `$.state($.proxy([]))`
- **Got**: `$.state([])` — proxy wrapping skipped for array literal in constructor
- **Layer**: codegen — constructor state init proxy decision

## Current state
- **Working**: 29/43 use cases fully covered with passing tests (+ 2 new passing)
- **Bugs found**: 3 codegen bugs discovered by new tests
- **Partial**: 2 use cases (dev labels for destructured state, constructor member expression)
- **Missing**: 12 use cases (validation errors, server codegen, deep_read_state, tag_proxy, snapshot is_ignored)
- **Next**: Fix BUG-1 (private field compound assignment) — most impactful, breaks reactivity for common class patterns. Then BUG-3 (constructor proxy), BUG-2 (template snapshot). After bugs, validation diagnostics (#33-36).
