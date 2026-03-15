# Current TODO

Next 5 features to implement, in priority order.

---

## 1. Void HTML elements

**Why first**: `<input>`, `<br>`, `<img>` без `/>` сейчас не парсятся — критично для валидного HTML.

**What to change**:
- Добавить `VOID_ELEMENTS` constant и `is_void(name)` helper
- В scanner/parser: auto-set `self_closing: true` для void elements
- Validation: ошибка на `</input>` и child-контент внутри void elements

**Reference**: `reference/compiler/phases/1-parse/state/element.js` line 371, `reference/compiler/utils.js`

---

## 2. `{@const x = expr}` (ConstTag)

**Why second**: нужен для вычислений внутри {#each} и {#if} блоков.

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Node::ConstTag { id, span, declaration_span }`
- `svelte_parser/src/lib.rs`: парсинг `{@const ...}` с извлечением variable declaration
- `svelte_analyze/`: scope integration — const binding visible in same block
- `svelte_codegen_client/src/template/`: `const x = expr` (non-reactive) or `$.derived(() => expr)` (reactive)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/ConstTag.js` (~134 lines)

---

## 3. `$state.raw(val)` rune

**Why third**: простой рун, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: добавить `RuneKind::StateRaw`
- `svelte_codegen_client/src/script.rs`: `$state.raw(val)` → `$.state(val)` (без `$.proxy()`)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`

**Runtime**: `$.state()`

---

## 4. `class` attribute — Object/array syntax (Svelte 5)

**Why fourth**: новый синтаксис `class={{ active: isActive }}` и `class={[base, active && "active"]}`.

**What to change**:
- `svelte_parser/src/lib.rs`: detect object/array expression in `class` attribute value
- `svelte_codegen_client/src/template/attributes.rs`: генерация `$.set_class(el, ...)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js`

---

## 5. `$state.snapshot(val)` rune

**Why fifth**: простая перезапись вызова, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: detect `$state.snapshot(val)` as inline call rewrite
- `svelte_codegen_client/src/script.rs`: `$state.snapshot(val)` → `$.snapshot(val)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`

**Runtime**: `$.snapshot()`
