# Состояние проекта

## Что готово

### AST (`svelte_ast`)
- `Text`, `Element`, `ComponentNode`, `Comment`
- `ExpressionTag` — `{expr}`
- `IfBlock`, `EachBlock`, `SnippetBlock`, `RenderTag`
- Атрибуты: `StringAttribute`, `ExpressionAttribute`, `BooleanAttribute`, `ConcatenationAttribute`, `ShorthandOrSpread`, `ClassDirective`, `BindDirective`

### Parser (`svelte_parser`)
- Все перечисленные узлы AST
- 43 теста, 100% проходят

### Analyze (`svelte_analyze`)
9 проходов в одном обходе дерева через composite visitor (`walker.rs`):
1. `parse_js` — парсинг JS-выражений, детекция rune-переменных
2. `scope` — OXC-скоупинг (script + template)
3. `mutations` — какие rune-переменные мутируются
4. `known_values` — статическое вычисление const-литералов
5. `props` — разбор `$props()` destructuring ($bindable, дефолты, rest)
6. `lower` — trim пробелов, слияние соседних text+expr
7. `reactivity` + `elseif` — разметка динамических узлов и атрибутов
8. `content_types` — классификация фрагментов (single-element, text-only и др.)
9. `needs_var` — какие элементы требуют JS-переменной

### Codegen (`svelte_codegen_client`)
- Script: `$state`, `$props` (с дефолтами, $bindable, rest, mutated), hoist imports, TypeScript strip, exports
- Template: Element, Component (props + children-as-snippet), IfBlock, EachBlock, SnippetBlock, RenderTag
- Атрибуты: string, boolean, expression, concatenation, shorthand, spread, `class:`, `bind:`
- Оптимизации: trim whitespace, merge text/expr, first-text-node, single-element, text-only, non-reactive attrs

---

## Следующие шаги (Tier 2 из `TODO_ANALYZE.md`)

| ID | Фича | Примечание |
|----|------|------------|
| 2a | `{@html expr}` | `$.html()` codegen, новый узел AST |
| 2b | `{#key expr}` | `$.key()` codegen, новый узел AST |
| 2c | Event handlers (`onclick={handler}`) | Парсер готов, нужен codegen |
| 2d | Style directive (`style:color={value}`) | Новый узел AST + codegen |
| 2e | Transitions / Animations | `transition:`, `in:`, `out:`, `animate:` |

Детали по каждому пункту — в `TODO_ANALYZE.md`.

---

## Отложено

**Tier 3 — Валидация** (все пункты — заглушки в `validate.rs`):
- bind:value на несовместимых элементах
- мутация const/import
- неверное расположение директив
- некорректные аргументы rune

**Tier 4 — Edge cases:**
- `{@const}` — inline const в шаблоне
- `{#await}` — асинхронные блоки
- CSS analysis (pruning, :global)
- A11y предупреждения
- Legacy mode (`$:`, `export let`, stores)
- Полная scope-система (нужна только для legacy)

---

## Архитектурные решения

- **OXC** — парсинг и скоупинг JS-выражений, хранятся только `Span`
- **Side tables** (`AnalysisData`) — никаких мутаций AST
- **Analyze**: composite visitor (tuple impl `TemplateVisitor`) — один обход дерева для всех проходов
- **Codegen**: прямая рекурсия, никакого visitor-паттерна
