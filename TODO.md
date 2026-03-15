# Current TODO

Next 5 features to implement, in priority order.

---

## 1. Event handlers (`on:event` → `onevent`)

**Why first**: базовая интерактивность уже работает через `onclick={handler}`, но `on:event` синтаксис Svelte 4 ещё не поддержан.

**What to change**:
- `svelte_parser/src/lib.rs`: парсинг `on:click={handler}` → `Attribute::OnDirective`
- `svelte_ast/src/lib.rs`: добавить `Attribute::OnDirective { name, expression, modifiers }`
- `svelte_codegen_client/src/template/attributes.rs`: генерация event listener

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/events.js`

---

## 2. Void HTML elements

**Why second**: `<input>`, `<br>`, `<img>` без `/>` сейчас не парсятся — критично для валидного HTML.

**What to change**:
- Добавить `VOID_ELEMENTS` constant и `is_void(name)` helper
- В scanner/parser: auto-set `self_closing: true` для void elements
- Validation: ошибка на `</input>` и child-контент внутри void elements

**Reference**: `reference/compiler/phases/1-parse/state/element.js` line 371, `reference/compiler/utils.js`

---

## 3. `{@const x = expr}` (ConstTag)

**Why third**: нужен для вычислений внутри {#each} и {#if} блоков.

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Node::ConstTag { id, span, declaration_span }`
- `svelte_parser/src/lib.rs`: парсинг `{@const ...}` с извлечением variable declaration
- `svelte_analyze/`: scope integration — const binding visible in same block
- `svelte_codegen_client/src/template/`: `const x = expr` (non-reactive) or `$.derived(() => expr)` (reactive)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/ConstTag.js` (~134 lines)

---

## 4. `$state.raw(val)` rune

**Why fourth**: простой рун, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: добавить `RuneKind::StateRaw`
- `svelte_codegen_client/src/script.rs`: `$state.raw(val)` → `$.state(val)` (без `$.proxy()`)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`

**Runtime**: `$.state()`

---

## 5. `class` attribute — Object/array syntax (Svelte 5)

**Why fifth**: новый синтаксис `class={{ active: isActive }}` и `class={[base, active && "active"]}`.

**What to change**:
- `svelte_parser/src/lib.rs`: detect object/array expression in `class` attribute value
- `svelte_codegen_client/src/template/attributes.rs`: генерация `$.set_class(el, ...)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js`
