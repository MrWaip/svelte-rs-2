# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `class` attribute — Object/array syntax (Svelte 5)

**Why first**: новый синтаксис `class={{ active: isActive }}` и `class={[base, active && "active"]}`.

**What to change**:
- `svelte_parser/src/lib.rs`: detect object/array expression in `class` attribute value
- `svelte_codegen_client/src/template/attributes.rs`: генерация `$.set_class(el, ...)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js`

---

## 2. `$state.snapshot(val)` rune

**Why second**: простая перезапись вызова, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$state.snapshot(val)` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$state.snapshot(val)` → `$.snapshot(val)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.snapshot()`

---

## 3. `$effect.tracking()` rune

**Why third**: тривиальная перезапись вызова, без аргументов.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$effect.tracking()` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$effect.tracking()` → `$.effect_tracking()`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.effect_tracking()`

---

## 4. `$effect.root(fn)` rune

**Why fourth**: простая перезапись вызова, аналогична `$effect.pre`.

**What to change**:
- `svelte_codegen_client/src/script.rs`: `$effect.root(fn)` → `$.effect_root(fn)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.effect_root()`

---

## 5. `$inspect(vals)` rune

**Why fifth**: dev-mode only — strip in prod. Needs `dev` compiler option.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$inspect(...)` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$inspect(val)` → `$.inspect(val)` (dev) / strip (prod)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.inspect()`
