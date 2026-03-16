# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `$host()` rune

**Why first**: простая замена выражения, для custom elements.

**What to change**:
- `svelte_codegen_client/src/script.rs`: `$host()` → `$$props.$$host`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: expression replacement

---

## 2. `bind:this` directive

**Why second**: element reference binding, most common bind directive after `bind:value`.

**What to change**:
- `svelte_codegen_client/src/template/attributes.rs`: `bind:this={ref}` → `$.bind_this(el, ($$value) => ref = $$value, () => ref)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/BindDirective.js`

**Runtime**: `$.bind_this()`

---

## 3. `$inspect(vals)` rune

**Why third**: dev-mode debugging rune, needs `dev` compiler option.

**What to change**:
- `svelte_codegen_client/src/script.rs`: `$inspect(a, b)` → `$.inspect(a, b)` (dev mode) or strip (prod)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.inspect()`

---

## 4. `bind:value` directive

**Why fourth**: most common form binding, required for any form-based component.

**What to change**:
- `svelte_codegen_client/src/template/attributes.rs`: `bind:value={x}` → `$.bind_value(el, () => x, ($$value) => x = $$value)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/BindDirective.js`

**Runtime**: `$.bind_value()`

---

## 5. `use:action={params}` directive

**Why fifth**: action directive, commonly used for DOM manipulation hooks.

**What to change**:
- Parser: `Attribute::UseDirective { name, expression_span }`
- `svelte_codegen_client/src/template/`: `use:action={params}` → `$.action(el, () => actionFn, () => params)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/UseDirective.js`

**Runtime**: `$.action()`

---
