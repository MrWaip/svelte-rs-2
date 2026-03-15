# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `{@const x = expr}` (ConstTag)

**Why first**: нужен для вычислений внутри {#each} и {#if} блоков.

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Node::ConstTag { id, span, declaration_span }`
- `svelte_parser/src/lib.rs`: парсинг `{@const ...}` с извлечением variable declaration
- `svelte_analyze/`: scope integration — const binding visible in same block
- `svelte_codegen_client/src/template/`: `const x = expr` (non-reactive) or `$.derived(() => expr)` (reactive)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/ConstTag.js` (~134 lines)

---

## 2. `$state.raw(val)` rune

**Why second**: простой рун, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: добавить `RuneKind::StateRaw`
- `svelte_codegen_client/src/script.rs`: `$state.raw(val)` → `$.state(val)` (без `$.proxy()`)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`

**Runtime**: `$.state()`

---

## 3. `class` attribute — Object/array syntax (Svelte 5)

**Why third**: новый синтаксис `class={{ active: isActive }}` и `class={[base, active && "active"]}`.

**What to change**:
- `svelte_parser/src/lib.rs`: detect object/array expression in `class` attribute value
- `svelte_codegen_client/src/template/attributes.rs`: генерация `$.set_class(el, ...)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js`

---

## 4. `$state.snapshot(val)` rune

**Why fourth**: простая перезапись вызова, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$state.snapshot(val)` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$state.snapshot(val)` → `$.snapshot(val)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.snapshot()`

---

## 5. `$effect.tracking()` rune

**Why fifth**: тривиальная перезапись вызова, без аргументов.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$effect.tracking()` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$effect.tracking()` → `$.effect_tracking()`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.effect_tracking()`
