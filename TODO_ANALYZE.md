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

### 1b. Export analysis
- [ ] `svelte_js/src/lib.rs` — экстракция `export const/function` деклараций
- [ ] `svelte_analyze/src/data.rs` — `exports: Vec<ExportDef>` в AnalysisData
- [ ] `svelte_analyze/src/exports.rs` — новый проход
- [ ] `svelte_codegen_client/src/script.rs` — `$$.exports` codegen
- [ ] Ref: `reference/compiler/phases/2-analyze/visitors/ExportNamedDeclaration.js`
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/Program.js` (append_exports)
- [ ] Test: `tasks/compiler_tests/cases2/exports/`

### 1c. Snippet & RenderTag
- [ ] `svelte_ast/src/lib.rs` — `Node::SnippetBlock`, `Node::RenderTag`
- [ ] `svelte_parser/src/` — парсинг `{#snippet name(params)}...{/snippet}`, `{@render snippet()}`
- [ ] `svelte_analyze/src/snippets.rs` — регистрация сниппетов (имя → span)
- [ ] `svelte_analyze/src/data.rs` — `snippets: HashMap<String, SnippetInfo>`
- [ ] `svelte_codegen_client/src/template/snippet.rs` — codegen
- [ ] Ref: `reference/compiler/phases/1-parse/state/tag.js` (snippet/render секции)
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/SnippetBlock.js`
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/RenderTag.js`
- [ ] Test: `tasks/compiler_tests/cases2/snippet_basic/`

### 1d. Component instantiation
- [ ] `svelte_ast/src/lib.rs` — `Node::Component` (отличается от `RegularElement`)
- [ ] `svelte_parser/src/` — различение Component vs Element (заглавная буква / точка / svelte:)
- [ ] `svelte_analyze/src/` — propagation metadata для компонентов
- [ ] `svelte_codegen_client/src/template/component.rs` — `$.component()` codegen
- [ ] Ref: `reference/compiler/phases/3-transform/client/visitors/Component.js`
- [ ] Ref: `reference/compiler/phases/1-parse/state/element.js` (component detection)
- [ ] Test: `tasks/compiler_tests/cases2/component_basic/`
- [ ] Зависит от: 1a (props), 1c (snippets как children)

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
- [ ] Codegen для `on:click` legacy директива
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
- [ ] 4f. Slot analysis ($$slots) — legacy, заменён сниппетами
- [ ] 4g. Function/closure warnings — DX only

---

## Архитектурные заметки

- Scope-система **НЕ НУЖНА** для Tiers 1-3 (runes mode). Текущий подход (OXC + side tables) достаточен.
- Критический путь: **Props → Exports → Snippets → Components**
- Каждая фича: test case → expected output через reference compiler → `cargo test`
