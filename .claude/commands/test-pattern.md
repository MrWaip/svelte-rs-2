# Test Pattern для svelte_parser

Тесты используют span-based сравнение: каждый узел AST имеет `span()`, `Component::source_text(span)` возвращает оригинальный срез. Тест сравнивает исходный текст с ожидаемой строкой.

## Хелперы

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

## Пример теста

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

## Когда использовать

- Любые новые тесты в `crates/svelte_parser`
- При добавлении новых типов узлов — добавить соответствующий хелпер `assert_<node_type>`
- Не использовать inline `if let Node::...` структурные проверки — только хелперы
- Исключение: `assert!(result.is_err())` для тестов ошибок без хелпера
