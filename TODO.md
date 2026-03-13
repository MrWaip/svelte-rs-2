# Current TODO

Next 5 features to implement, in priority order.

---

## 1. `$derived` / `$derived.by`

**Why first**: третий по частоте рун после `$state`/`$props`, чисто script codegen — без изменений AST/parser.

**What to change**:
- `svelte_js/src/lib.rs` → `detect_rune()`: добавить обработку member expressions (`$derived.by`, `$state.raw`, `$effect.pre`)
- `svelte_codegen_client/src/script.rs` → `enter_variable_declarator`: различать `RuneKind::State` vs `RuneKind::Derived`
  - `$derived(expr)` → `$.derived(() => expr)` (wrap в arrow function)
  - `$derived.by(fn)` → `$.derived_by(fn)` (rename, pass through)

**Reference**: `reference/compiler/phases/3-transform/client/visitors/VariableDeclaration.js` (lines handling `$derived`)

**Runtime**: `$.derived()`, `$.derived_by()`

---

## 2. `$effect` / `$effect.pre`

**Why second**: необходим для side effects, чисто script codegen.

**What to change**:
- `detect_rune()`: member expression support (уже нужно для п.1, тут переиспользуем)
- `svelte_codegen_client/src/script.rs`: новый обработчик для expression statements вида `$effect(() => ...)` / `$effect.pre(() => ...)`
  - `$effect(fn)` → `$.user_effect(fn)`
  - `$effect.pre(fn)` → `$.user_pre_effect(fn)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/CallExpression.js`, `ExpressionStatement.js`

**Runtime**: `$.user_effect()`, `$.user_pre_effect()`

---

## 3. `{@html expr}` (HtmlTag)

**Why third**: очень частая фича (markdown, CMS контент), первая фича требующая parser+AST+codegen.

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Node::HtmlTag { expression: Span }`
- `svelte_parser/src/lib.rs`: обработка `{@html ...}` в scanner
- `svelte_codegen_client/src/template/`: генерация `$.html(anchor, () => expr)`
- Analyze: отметить как dynamic expression

**Reference**: `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`

**Runtime**: `$.html()`

---

## 4. `{#key expr}` (KeyBlock)

**Why fourth**: нужен для роутеров (ремаунт при смене route) и анимаций.

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Node::KeyBlock { expression: Span, fragment: Fragment }`
- `svelte_parser/src/lib.rs`: парсинг `{#key expr}...{/key}`
- `svelte_analyze/`: content_types для KeyBlock fragment
- `svelte_codegen_client/src/template/`: `$.key(anchor, () => expr, ($$anchor) => { ... })`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js`

**Runtime**: `$.key()`

---

## 5. `style:prop` directive

**Why fifth**: так же часто используется как `class:`, паттерн уже есть (ClassDirective).

**What to change**:
- `svelte_ast/src/lib.rs`: добавить `Attribute::StyleDirective { name, value, modifiers }`
- `svelte_parser/src/lib.rs`: парсинг `style:color={value}` / `style:color="red"` / `style:color` (shorthand)
- `svelte_codegen_client/src/template/attributes.rs`: генерация `$.set_style(element, "color", value)`

**Reference**: `reference/compiler/phases/3-transform/client/visitors/shared/element.js` (style directive section)

**Runtime**: `$.set_style()`
