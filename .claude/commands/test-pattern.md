# Test Pattern для svelte_parser и svelte_analyze

Тесты используют span-based сравнение: каждый узел AST имеет `span()`, `Component::source_text(span)` возвращает оригинальный срез. Тест сравнивает исходный текст с ожидаемой строкой — не NodeId, не структурные поля.

---

## svelte_parser — хелперы

```rust
fn assert_node(c: &Component, index: usize, expected: &str) {
    assert_eq!(c.source_text(c.fragment.nodes[index].span()), expected);
}

fn assert_script(c: &Component, expected: &str) {
    let script = c.script.as_ref().expect("expected script");
    assert_eq!(c.source_text(script.content_span), expected);
}

fn assert_if_block(c: &Component, index: usize, expected_test: &str) {
    if let Node::IfBlock(ref ib) = c.fragment.nodes[index] {
        assert_eq!(c.source_text(ib.test_span), expected_test);
    } else {
        panic!("expected IfBlock at index {index}");
    }
}
```

## svelte_parser — пример

```rust
#[test]
fn smoke() {
    let c = parse("prefix <div>text</div>");
    assert_node(&c, 0, "prefix ");
    assert_node(&c, 1, "<div>text</div>");
}

#[test]
fn if_block() {
    let c = parse("{#if true}<div>title</div>{/if}");
    assert_node(&c, 0, "{#if true}<div>title</div>{/if}");
    assert_if_block(&c, 0, "true");
}
```

---

## svelte_analyze — хелперы

Анализ идентифицирует узлы через исходный текст выражения (`source_text(expression_span)`), а не через NodeId.

```rust
fn analyze_source(source: &str) -> (Component, AnalysisData) {
    let component = Parser::new(source).parse().expect("parse failed");
    let (data, diags) = analyze(&component);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    (component, data)
}

fn assert_root_content_type(data: &AnalysisData, expected: ContentType) {
    let actual = data.content_types.get(&ROOT_FRAGMENT_ID).expect("no root content type");
    assert_eq!(*actual, expected);
}

fn assert_symbol(data: &AnalysisData, name: &str) {
    assert!(data.symbol_by_name.contains_key(name), "expected symbol '{name}'");
}

fn assert_is_rune(data: &AnalysisData, name: &str) {
    let idx = data.symbol_by_name.get(name).unwrap_or_else(|| panic!("no symbol '{name}'"));
    assert!(data.runes.contains_key(idx), "expected '{name}' to be a rune");
}

fn assert_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str) {
    let id = find_expr_tag(&component.fragment, component, expr_text)
        .unwrap_or_else(|| panic!("no ExpressionTag with source '{expr_text}'"));
    assert!(data.dynamic_nodes.contains(&id), "expected ExpressionTag '{expr_text}' to be dynamic");
}

fn assert_lowered_item_count(data: &AnalysisData, owner_id: NodeId, expected_count: usize) {
    let lf = data.lowered_fragments.get(&owner_id).expect("no lowered fragment");
    assert_eq!(lf.items.len(), expected_count);
}

// Рекурсивный поиск ExpressionTag по тексту выражения
fn find_expr_tag(fragment: &Fragment, component: &Component, target: &str) -> Option<NodeId> {
    for node in &fragment.nodes {
        match node {
            Node::ExpressionTag(t) if component.source_text(t.expression_span) == target => {
                return Some(t.id);
            }
            Node::Element(el) => {
                if let Some(id) = find_expr_tag(&el.fragment, component, target) {
                    return Some(id);
                }
            }
            Node::IfBlock(b) => {
                if let Some(id) = find_expr_tag(&b.consequent, component, target) {
                    return Some(id);
                }
                if let Some(alt) = &b.alternate {
                    if let Some(id) = find_expr_tag(alt, component, target) {
                        return Some(id);
                    }
                }
            }
            Node::EachBlock(b) => {
                if let Some(id) = find_expr_tag(&b.body, component, target) {
                    return Some(id);
                }
            }
            _ => {}
        }
    }
    None
}
```

## svelte_analyze — пример

```rust
#[test]
fn rune_detection() {
    let (c, data) = analyze_source(r#"<script>let count = $state(0);</script><p>{count}</p>"#);
    assert_symbol(&data, "count");
    assert_is_rune(&data, "count");
    assert_dynamic_tag(&data, &c, "count");
}

#[test]
fn lowered_fragment_groups_text_and_expr() {
    let (_c, data) = analyze_source(r#"<script>let x = $state(1);</script>Hello {x} world"#);
    assert_lowered_item_count(&data, ROOT_FRAGMENT_ID, 1);
}
```

---

## Правила (оба крейта)

- Использовать `assert_<aspect>` хелперы — не писать inline `if let Node::...` или прямые обращения к `.len()` / `.contains()`
- Идентификация узлов — через source text (span), не через индексы или NodeId
- При добавлении нового типа узла или нового аспекта анализа — добавить хелпер `assert_<node_type>` или `assert_<aspect>`
- Исключение: `assert!(result.is_err())` для тестов ошибок хелпера не требует
