# TODO: Портирование недостающих фич анализа

## Tier 1 — Разблокирует новый codegen

### 1a. Props ($props, $bindable) ✅
- [x] `svelte_js/src/lib.rs` — экстракция $props() destructuring: имена, дефолты, $bindable() флаги
- [x] `svelte_analyze/src/data.rs` — `PropsAnalysis { props: Vec<PropAnalysis>, has_bindable }` в AnalysisData
- [x] `svelte_analyze/src/props.rs` — новый проход: парсинг `let { x, y = default } = $props()`
- [x] `svelte_analyze/src/lib.rs` — подключить props pass
- [x] `svelte_codegen_client/src/script.rs` — `$.prop($$props, "x", ...)` codegen
- [x] `svelte_codegen_client/src/lib.rs` — `$.push/$.pop` для bindable props
- [x] Test: `props_basic` — non-source, source с default
- [x] Test: `props_rest` — rest props, `$.rest_props()`, excluded names
- [x] Test: `props_bindable` — `$bindable()` без/с default, push/pop
- [x] Test: `props_lazy_default` — thunk wrapping, zero-arg optimization
- [x] Test: `props_mutated` — `$.update_prop()`, PROPS_IS_UPDATED
- [x] Test: `props_mixed` — все типы в одном компоненте

### 1b. Export analysis ✅
- [x] `svelte_js/src/lib.rs` — экстракция `export const/function` деклараций
- [x] `svelte_analyze/src/data.rs` — `exports: Vec<ExportDef>` в AnalysisData
- [x] `svelte_analyze/src/exports.rs` — новый проход
- [x] `svelte_codegen_client/src/script.rs` — `$$.exports` codegen
- [x] Ref: `reference/compiler/phases/2-analyze/visitors/ExportNamedDeclaration.js`
- [x] Ref: `reference/compiler/phases/3-transform/client/visitors/Program.js` (append_exports)
- [x] Test: `tasks/compiler_tests/cases2/exports/`

### 1c. Snippet & RenderTag ✅
- [x] `svelte_ast/src/lib.rs` — `Node::SnippetBlock`, `Node::RenderTag`
- [x] `svelte_parser/src/` — парсинг `{#snippet name(params)}...{/snippet}`, `{@render snippet()}`
- [x] `svelte_analyze/src/snippets.rs` — регистрация сниппетов (имя → span)
- [x] `svelte_analyze/src/data.rs` — `snippets: HashMap<String, SnippetInfo>`
- [x] `svelte_codegen_client/src/template/snippet.rs` — codegen
- [x] Ref: `reference/compiler/phases/1-parse/state/tag.js` (snippet/render секции)
- [x] Ref: `reference/compiler/phases/3-transform/client/visitors/SnippetBlock.js`
- [x] Ref: `reference/compiler/phases/3-transform/client/visitors/RenderTag.js`
- [x] Test: `tasks/compiler_tests/cases2/snippet_basic/`

### 1d. Component instantiation (частично ✅)
- [x] `svelte_ast/src/lib.rs` — `Node::ComponentNode` (отличается от `Element`)
- [x] `svelte_parser/src/` — различение Component vs Element (`is_component_name`: заглавная буква / точка)
- [x] `svelte_analyze/src/` — `FragmentItem::ComponentNode`, `FragmentKey::ComponentNode`, все проходы обновлены
- [x] `svelte_codegen_client/src/template/component.rs` — `Name($$anchor, {})` codegen
- [x] Test: `component_basic` — `<Button />`
- [x] Test: `component_non_self_closing` — `<Modal></Modal>`
- [x] Test: `component_in_element` — `<div><Button /></div>`
- [x] Test: `component_mixed` — `<h1>Title</h1><Button /><Icon />`

Не реализовано (подзадачи):
- [x] **1d-props** — передача props в компонент: `<Button onclick={handler} label="text" />` ✅
  - [x] `parse_js.rs` — рефакторинг `walk_attr_expressions` → `walk_attrs(owner_id, attrs)`, парсит атрибуты и для Element, и для ComponentNode
  - [x] `walker.rs` — `visit_component_attribute` в TemplateVisitor + dispatch + composite macro
  - [x] `reactivity.rs` — `component_attr_is_dynamic` (любой rune делает prop динамическим, не только mutated)
  - [x] `builder.rs` — `ObjProp::Getter` → `get name() { return expr }`
  - [x] `component.rs` — сборка props объекта (string, boolean, expression, concatenation, shorthand)
  - [x] Test: `component_props` — `label="Click me"`, `onclick={handler}`, `count={count}` (getter)
  - Не включено: spread props (`$.spread_props()`), bind directives на компонентах
  - Ref: `reference/compiler/phases/3-transform/client/visitors/shared/component.js`
- [ ] **1d-children** — children как snippet: `<Button>content</Button>`
  - Генерация `children: ($$anchor, $$slotProps) => { ... }` из `ComponentNode.fragment`
  - `$$slots: { default: true }` маркер
  - Ref: `reference/compiler/phases/3-transform/client/visitors/Component.js` (children секция)
- [ ] **1d-events** — event forwarding на компонентах
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/Component.js`
- [ ] Ref: `reference/compiler/phases/1-parse/state/element.js` (component detection)

---

## Tier 2 — Нужно для реальных приложений

### 2a. {@html expr}
- [ ] `svelte_ast/src/lib.rs` — `Node::HtmlTag { expression: Span }`
- [ ] `svelte_parser/src/` — парсинг `{@html ...}`
- [ ] `svelte_codegen_client/src/template/` — `$.html()` codegen
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/HtmlTag.js`
- [ ] Test: `tasks/compiler_tests/cases2/html_tag/`

### 2b. {#key expr}
- [ ] `svelte_ast/src/lib.rs` — `Node::KeyBlock { expression: Span, fragment: Fragment }`
- [ ] `svelte_parser/src/` — парсинг `{#key ...}...{/key}`
- [ ] `svelte_codegen_client/src/template/key_block.rs` — `$.key()` codegen
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/KeyBlock.js`
- [ ] Test: `tasks/compiler_tests/cases2/key_block/`

### 2c. Event handlers
- [ ] Codegen для `onclick={handler}` как атрибут (уже парсится?)
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/shared/events.js`
- [ ] Test: `tasks/compiler_tests/cases2/event_handler/`

### 2d. Style directive
- [ ] `svelte_ast/src/lib.rs` — `Node::StyleDirective` (аналог ClassDirective)
- [ ] `svelte_parser/src/` — парсинг `style:color={value}`
- [ ] `svelte_codegen_client/src/template/attributes.rs` — `$.style()` codegen
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/StyleDirective.js`
- [ ] Test: `tasks/compiler_tests/cases2/style_directive/`

### 2e. Transition/Animation
- [ ] `svelte_ast/src/lib.rs` — `TransitionDirective`, `AnimateDirective`
- [ ] `svelte_parser/src/` — парсинг `transition:fade`, `in:`, `out:`, `animate:`
- [ ] `svelte_codegen_client/src/template/` — `$.transition()`, `$.animate()` codegen
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/TransitionDirective.js`
- [ ] Test: `tasks/compiler_tests/cases2/transition/`

---

## Tier 3 — Валидация и корректность

### 3a. Bind directive validation
- [ ] `svelte_analyze/src/validate.rs` — ошибка на bind:value на неподходящих элементах
- [ ] Ref: `reference/compiler/phases/2-analyze/visitors/BindDirective.js`

### 3b. Assignment validation
- [ ] `svelte_analyze/src/validate.rs` — ошибка на мутацию const/import
- [ ] Ref: `reference/compiler/phases/2-analyze/visitors/AssignmentExpression.js`

### 3c. Directive placement validation
- [ ] `svelte_analyze/src/validate.rs` — ошибка на transition: на компоненте
- [ ] Ref: `reference/compiler/phases/2-analyze/visitors/Component.js` (validation секция)

### 3d. Rune argument validation
- [ ] `svelte_analyze/src/validate.rs` — ошибка на `$state(a, b)` и подобное
- [ ] Ref: `reference/compiler/phases/2-analyze/visitors/CallExpression.js`

### 3e. {@const}
- [ ] `svelte_ast/src/lib.rs` — `Node::ConstTag`
- [ ] `svelte_parser/src/` — парсинг `{@const x = expr}`
- [ ] `svelte_analyze/src/` — scope-like обработка
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/ConstTag.js`
- [ ] Test: `tasks/compiler_tests/cases2/const_tag/`

---

## Tier 4 — Edge cases, legacy, DX

- [ ] 4a. CSS analysis (pruning, :global, unused) — `svelte_analyze/src/css.rs`
- [ ] 4b. A11y warnings — `svelte_analyze/src/a11y.rs`
- [ ] 4c. Legacy mode ($:, stores, export let) — требует scope system
- [ ] 4d. Full scope system — `svelte_analyze/src/scope.rs` (~1300 строк в reference)
- [ ] 4e. AwaitBlock — `svelte_ast`, парсер, codegen
- ~~4f. Slot analysis ($$slots) — legacy, заменён сниппетами~~ — не нужен, используем snippets
- [ ] 4g. Function/closure warnings — DX only

---

## Архитектурные заметки

- Scope-система **НЕ НУЖНА** для Tiers 1-3 (runes mode). Текущий подход (OXC + side tables) достаточен.
- Критический путь: **Props → Exports → Snippets → Components**
- Каждая фича: test case → expected output через reference compiler → `cargo test`
