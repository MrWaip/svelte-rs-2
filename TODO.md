# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `$state.raw(val)` rune

**Why first**: простой рун, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: добавить `RuneKind::StateRaw`
- `svelte_codegen_client/src/script.rs`: `$state.raw(val)` → `$.state(val)` (без `$.proxy()`)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`

**Runtime**: `$.state()`

---

## 2. `class` attribute — Object/array syntax (Svelte 5)

**Why third**: новый синтаксис `class={{ active: isActive }}` и `class={[base, active && "active"]}`.

**What to change**:
- `svelte_parser/src/lib.rs`: detect object/array expression in `class` attribute value
- `svelte_codegen_client/src/template/attributes.rs`: генерация `$.set_class(el, ...)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js`

---

## 3. `$state.snapshot(val)` rune

**Why fourth**: простая перезапись вызова, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$state.snapshot(val)` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$state.snapshot(val)` → `$.snapshot(val)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.snapshot()`

---

## 4. `$effect.tracking()` rune

**Why fifth**: тривиальная перезапись вызова, без аргументов.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$effect.tracking()` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$effect.tracking()` → `$.effect_tracking()`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.effect_tracking()`
