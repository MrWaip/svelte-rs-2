# Plan: Single-Pass Composite Visitor для svelte_analyze

## Проблема

6 проходов по template AST с идентичным boilerplate рекурсии (~200 строк).
Каждый pass копирует один и тот же match по Node, логику scope tracking для EachBlock,
рекурсию в Element/IfBlock/EachBlock/SnippetBlock. Легко забыть рекурсию (баг snippet_params).

## Ключевая идея: Single-Pass Composite Visitor (как в oxc)

Каждый analysis pass реализует свой `TemplateVisitor` — только те visit-методы,
которые ему интересны. Но под капотом несколько passes запускаются в **одном обходе дерева**
через composite visitor (tuple). Один `walk_template()` вызов — все совместимые passes
выполняются параллельно за один проход.

## Дизайн

### 1. Trait `TemplateVisitor`

```rust
pub(crate) trait TemplateVisitor {
    // Leaf visitors — default no-op
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_each_block(&mut self, block: &EachBlock, body_scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_snippet_block(&mut self, block: &SnippetBlock, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_bind_directive(&mut self, dir: &BindDirective, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
    fn visit_attribute(&mut self, attr: &Attribute, idx: usize, el: &Element, scope: ScopeId, data: &mut AnalysisData) {}
}
```

Ключевые решения:
- `data: &mut AnalysisData` передаётся в каждый visit-метод (не хранится в visitor) — позволяет composite
- `scope: ScopeId` передаётся walker-ом автоматически — scope tracking в одном месте
- Все методы default no-op — каждый pass реализует только то, что ему интересно

### 2. `walk_template()` — единственная функция обхода

```rust
pub(crate) fn walk_template<V: TemplateVisitor>(
    fragment: &Fragment,
    component: &Component,
    data: &mut AnalysisData,
    scope: ScopeId,
    visitor: &mut V,
) {
    for node in &fragment.nodes {
        match node {
            Node::ExpressionTag(tag) => {
                visitor.visit_expression_tag(tag, scope, data);
            }
            Node::Element(el) => {
                visitor.visit_element(el, scope, data);
                for (idx, attr) in el.attributes.iter().enumerate() {
                    visitor.visit_attribute(attr, idx, el, scope, data);
                    if let Attribute::BindDirective(dir) = attr {
                        visitor.visit_bind_directive(dir, el, scope, data);
                    }
                }
                walk_template(&el.fragment, component, data, scope, visitor);
            }
            Node::IfBlock(block) => {
                visitor.visit_if_block(block, scope, data);
                walk_template(&block.consequent, component, data, scope, visitor);
                if let Some(alt) = &block.alternate {
                    walk_template(alt, component, data, scope, visitor);
                }
            }
            Node::EachBlock(block) => {
                let body_scope = data.scoping.node_scope(block.id).unwrap_or(scope);
                visitor.visit_each_block(block, body_scope, data);
                walk_template(&block.body, component, data, body_scope, visitor);
                if let Some(fb) = &block.fallback {
                    walk_template(fb, component, data, scope, visitor); // fallback = parent scope
                }
            }
            Node::SnippetBlock(block) => {
                visitor.visit_snippet_block(block, scope, data);
                walk_template(&block.body, component, data, scope, visitor);
            }
            Node::RenderTag(tag) => {
                visitor.visit_render_tag(tag, scope, data);
            }
            Node::Text(_) | Node::Comment(_) => {}
        }
    }
}
```

Scope tracking встроен: EachBlock автоматически переключает scope, fallback остаётся на parent.

### 3. Composite visitor — tuple impls

```rust
impl<A: TemplateVisitor, B: TemplateVisitor> TemplateVisitor for (A, B) {
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {
        self.0.visit_expression_tag(tag, scope, data);
        self.1.visit_expression_tag(tag, scope, data);
    }
    // ... аналогично для всех методов
}
```

Реализовать для кортежей (A, B), (A, B, C), (A, B, C, D) — покрывает все нужды.
Можно через macro_rules! чтобы не дублировать.

### 4. Рефакторинг passes

**Фаза 1 — walker utility + рефакторинг отдельных passes:**

| Pass | Visitor struct | Реализует |
|------|---------------|-----------|
| mutations (template) | `TemplateMutationVisitor` | `visit_expression_tag` |
| mutations (binds) | `BindMutationVisitor` | `visit_bind_directive` |
| reactivity | `ReactivityVisitor` | `visit_expression_tag`, `visit_element`, `visit_if_block`, `visit_each_block`, `visit_render_tag` |
| elseif | `ElseifVisitor` | `visit_if_block` |

**Фаза 2 — composite merges (один обход вместо нескольких):**

- `mutations`: `(TemplateMutationVisitor, BindMutationVisitor)` — один обход вместо двух
- `post_lower`: `(ReactivityVisitor, ElseifVisitor)` — один обход вместо двух

**Не трогаем:**
- `parse_js` — особый pass, создаёт expressions и diagnostics, сложная логика парсинга. Можно перевести позже, но выигрыш минимален.
- `build_scoping` — мутирует scoping (`add_child_scope`, `add_binding`), нужен `&mut ComponentScoping` напрямую. Можно перевести, но scope creation логика уникальна.
- `lower` — работает с FragmentKey/LoweredFragment, другая модель обхода (не по Node, а по items).
- `content_types` — работает с lowered fragments.

### 5. snippet_params в reactivity

Сейчас reactivity передаёт `snippet_params: &[String]` вручную. С walker-ом:
- `visit_snippet_block` может push params в стек внутри `ReactivityVisitor`
- Или walker сам трекает snippet_params (добавить в context)

Предпочтительно: walker трекает snippet context автоматически, передаёт в visit-методы.

## Шаги реализации

1. Создать `crates/svelte_analyze/src/walker.rs` — trait + walk_template + tuple impls
2. Рефакторинг `mutations.rs` — два visitor struct, composite, один `walk_template` вызов
3. Рефакторинг `reactivity.rs` — visitor struct
4. Рефакторинг `elseif.rs` — visitor struct
5. Объединить reactivity + elseif в один обход в `lib.rs`
6. Обновить CLAUDE.md — ослабить ограничение для analyze
7. Прогнать тесты

## Изменение CLAUDE.md

Текущее ограничение (строки 38-40):
```
Design the internals for Rust: direct recursion over side tables,
no visitor/walker patterns, no mutable AST metadata.
```

Заменить на:
```
Design the internals for Rust: direct recursion over side tables,
no mutable AST metadata. Don't replicate JS workarounds,
intermediate abstractions, or patterns that exist only because of zimmerframe/estree-walker.

Exception: `svelte_analyze` uses a single-pass composite visitor (`walker.rs`)
to eliminate boilerplate in template analysis passes. Each pass implements
`TemplateVisitor` for only the nodes it cares about. Independent passes are
combined into a single tree traversal via tuple composite visitors.
Codegen (`svelte_codegen_client`) uses direct recursion — no visitor pattern.
```

## Что НЕ меняется

- codegen — прямая рекурсия остаётся (каждая нода = уникальная генерация)
- parse_js, build_scoping, lower, content_types — остаются как есть
- Публичный API `analyze()` — без изменений
- Все тесты должны проходить без изменений
