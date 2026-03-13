# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `$effect` / `$effect.pre`

**Why first**: необходим для side effects, чисто script codegen.

**What to change**:
- `detect_rune()`: member expression support (уже готово — `$derived.by` паттерн)
- `svelte_codegen_client/src/script.rs`: новый обработчик для expression statements вида `$effect(() => ...)` / `$effect.pre(() => ...)`
  - `$effect(fn)` → `$.user_effect(fn)`
  - `$effect.pre(fn)` → `$.user_pre_effect(fn)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`, `ExpressionStatement.js`

**Runtime**: `$.user_effect()`, `$.user_pre_effect()`

---

## 2. `{@html expr}` (HtmlTag)

**Why second**: очень частая фича (markdown, CMS контент), первая фича требующая parser+AST+codegen.

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Node::HtmlTag { expression: Span }`
- `svelte_parser/src/lib.rs`: обработка `{@html ...}` в scanner
- `svelte_codegen_client/src/template/`: генерация `$.html(anchor, () => expr)`
- Analyze: отметить как dynamic expression

**Reference**: `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`

**Runtime**: `$.html()`

---

## 3. `{#key expr}` (KeyBlock)

**Why third**: нужен для роутеров (ремаунт при смене route) и анимаций.

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Node::KeyBlock { expression: Span, fragment: Fragment }`
- `svelte_parser/src/lib.rs`: парсинг `{#key expr}...{/key}`
- `svelte_analyze/`: content_types для KeyBlock fragment
- `svelte_codegen_client/src/template/`: `$.key(anchor, () => expr, ($$anchor) => { ... })`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js`

**Runtime**: `$.key()`

---

## 4. `style:prop` directive

**Why fourth**: так же часто используется как `class:`, паттерн уже есть (ClassDirective).

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Attribute::StyleDirective { name, value, modifiers }`
- `svelte_parser/src/lib.rs`: парсинг `style:color={value}` / `style:color="red"` / `style:color` (shorthand)
- `svelte_codegen_client/src/template/attributes.rs`: генерация `$.set_style(element, "color", value)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js` (style directive section)

**Runtime**: `$.set_style()`

---

## 5. Event handlers (`on:event` → `onevent`)

**Why fifth**: базовая интерактивность уже работает через `onclick={handler}`, но `on:event` синтаксис Svelte 4 ещё не поддержан.

**What to change**:
- `svelte_parser/src/lib.rs`: парсинг `on:click={handler}` → `Attribute::OnDirective`
- `svelte_ast/src/lib.rs`: добавить `Attribute::OnDirective { name, expression, modifiers }`
- `svelte_codegen_client/src/template/attributes.rs`: генерация event listener

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/events.js`
