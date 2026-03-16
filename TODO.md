# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `$effect.root(fn)` rune

**Why first**: простая перезапись вызова, аналогична `$effect.pre`.

**What to change**:
- `svelte_codegen_client/src/script.rs`: `$effect.root(fn)` → `$.effect_root(fn)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.effect_root()`

---

## 2. `$host()` rune

**Why second**: простая замена выражения, для custom elements.

**What to change**:
- `svelte_codegen_client/src/script.rs`: `$host()` → `$$props.$$host`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: expression replacement

---

## 3. `bind:this` directive

**Why third**: element reference binding, most common bind directive after `bind:value`.

**What to change**:
- `svelte_codegen_client/src/template/attributes.rs`: `bind:this={ref}` → `$.bind_this(el, ($$value) => ref = $$value, () => ref)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/BindDirective.js`

**Runtime**: `$.bind_this()`

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
