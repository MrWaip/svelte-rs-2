# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `style:prop` directive

**Why first**: так же часто используется как `class:`, паттерн уже есть (ClassDirective).

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Attribute::StyleDirective { name, value, modifiers }`
- `svelte_parser/src/lib.rs`: парсинг `style:color={value}` / `style:color="red"` / `style:color` (shorthand)
- `svelte_codegen_client/src/template/attributes.rs`: генерация `$.set_style(element, "color", value)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js` (style directive section)

**Runtime**: `$.set_style()`

---

## 2. Event handlers (`on:event` → `onevent`)

**Why second**: базовая интерактивность уже работает через `onclick={handler}`, но `on:event` синтаксис Svelte 4 ещё не поддержан.

**What to change**:
- `svelte_parser/src/lib.rs`: парсинг `on:click={handler}` → `Attribute::OnDirective`
- `svelte_ast/src/lib.rs`: добавить `Attribute::OnDirective { name, expression, modifiers }`
- `svelte_codegen_client/src/template/attributes.rs`: генерация event listener

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/events.js`

---

## 3. Void HTML elements

**Why third**: `<input>`, `<br>`, `<img>` без `/>` сейчас не парсятся — критично для валидного HTML.

**What to change**:
- Добавить `VOID_ELEMENTS` constant и `is_void(name)` helper
- В scanner/parser: auto-set `self_closing: true` для void elements
- Validation: ошибка на `</input>` и child-контент внутри void elements

**Reference**: `reference/compiler/phases/1-parse/state/element.js` line 371, `reference/compiler/utils.js`

---

## 4. `{@const x = expr}` (ConstTag)

**Why fourth**: нужен для вычислений внутри {#each} и {#if} блоков.

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Node::ConstTag { id, span, declaration_span }`
- `svelte_parser/src/lib.rs`: парсинг `{@const ...}` с извлечением variable declaration
- `svelte_analyze/`: scope integration — const binding visible in same block
- `svelte_codegen_client/src/template/`: `const x = expr` (non-reactive) or `$.derived(() => expr)` (reactive)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/ConstTag.js` (~134 lines)

---

## 5. `$state.raw(val)` rune

**Why fifth**: простой рун, паттерн уже есть в script.rs.

**What to change**:
- `svelte_analyze/src/parse_js.rs`: добавить `RuneKind::StateRaw`
- `svelte_codegen_client/src/script.rs`: `$state.raw(val)` → `$.state(val)` (без `$.proxy()`)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js`

**Runtime**: `$.state()`
