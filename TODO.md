# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `$effect.tracking()` rune

**Why first**: тривиальная перезапись вызова, без аргументов.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$effect.tracking()` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$effect.tracking()` → `$.effect_tracking()`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.effect_tracking()`

---

## 2. `$effect.root(fn)` rune

**Why second**: простая перезапись вызова, аналогична `$effect.pre`.

**What to change**:
- `svelte_codegen_client/src/script.rs`: `$effect.root(fn)` → `$.effect_root(fn)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.effect_root()`

---

## 3. `$host()` rune

**Why third**: простая замена выражения, для custom elements.

**What to change**:
- `svelte_codegen_client/src/script.rs`: `$host()` → `$$props.$$host`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: expression replacement

---

---

# Deprioritized (Dev-mode features)

Requires `dev` compiler option infrastructure. Will be implemented after core features are complete.

## `$inspect(vals)` rune

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$inspect(...)` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$inspect(val)` → `$.inspect(val)` (dev) / strip (prod)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.inspect()`

---

## `{@debug vars}` — Dev-mode debugger

**What to change**:
- Parser: parse `{@debug x, y}`
- AST: `Node::DebugTag { id, span, identifiers: Vec<Span> }`
- Codegen: `debugger` statement + `console.log` (dev) / nothing (prod)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/DebugTag.js`
