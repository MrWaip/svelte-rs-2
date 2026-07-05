# Examples

Shape: setup → optional execute → one `#[track_caller]` assert with explicit messages. **real** = name exists in repo; **target** = helper the layer must add.

## 1. analyze, setup+execute folded (real)

`crates/svelte_analyze/src/tests.rs` — `analyze_source` parses and analyzes, so two lines:

```rust
#[test]
fn dynamic_expr_inside_svelte_element() {
    let (c, data) = analyze_source(r#"<script>let { name } = $props();</script><svelte:element this={"div"}>{name}</svelte:element>"#);
    assert_dynamic_tag(&data, &c, "name");
}
```

## 2. parser, entity assert (real)

`scanner/tests.rs` — `tokenize` factory, `assert_start_tag` takes `expected_*`:

```rust
#[test]
fn parses_self_closing_div() {
    let (tokens, source) = tokenize("<div/>");
    assert_start_tag(&tokens[0], source, "div", true);
}
```

## 3. diagnostics, expected and none (real)

`assert_diag_codes` checks the exact code set; `&[]` asserts none:

```rust
#[test]
fn reports_invalid_each_binding() {
    let (_c, _data, diags) = analyze_source_with_diags("{#each items}{item}{/each}");
    assert_diag_codes(&diags, &["each_item_invalid_assignment"]);
}

#[test]
fn no_diagnostics_for_valid_each() {
    let (_c, _data, diags) = analyze_source_with_diags("{#each items as item}{item}{/each}");
    assert_diag_codes(&diags, &[]);
}
```

## 4. distinct execute, transform before→after (target)

When the action is the point of the test, execute stays on its own line. The 14-line `crates/svelte_transform_client/src/lib.rs:541` arrange/act block folds into a factory + execute, raw checks move inside the assert:

```rust
#[test]
fn snippet_default_label_uses_signal_get() {
    let mut ctx = setup_transform(r#"{#snippet withDefault({ label = "default" })}<span>{label}</span>{/snippet}"#);
    let expr = transform_and_find_expr(&mut ctx, "label");
    assert_is_call_expression(&expr);
}
```

## 5. collapse several asserts into one

Bad — two asserts, two behaviours:

```rust
assert_symbol(&data, "show");
assert_dynamic_if_block(&data, &c, "show");
```

Good — one entity assert owns both:

```rust
assert_dynamic_if_block_for_symbol(&data, &c, "show");
```

## 6. raw asserts in the body

Bad — mechanics leak into the test:

```rust
let (tokens, _source) = tokenize("{ id }");
assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
assert!(tokens[1].token_type == TokenType::EOF);
```

Good:

```rust
let (tokens, _source) = tokenize("{ id }");
assert_only_interpolation(&tokens);
```

## Well-formed custom assert

`#[track_caller]` + a message per check naming expected and printing actual:

```rust
#[track_caller]
fn assert_start_tag(tag: &StartTag, source: &str, expected_name: &str, expected_self_closing: bool) {
    assert_eq!(tag.name_span.source_text(source), expected_name,
        "start tag name: expected {expected_name:?}, got {:?}", tag.name_span.source_text(source));
    assert_eq!(tag.self_closing, expected_self_closing,
        "<{expected_name}> self_closing: expected {expected_self_closing}, got {}", tag.self_closing);
}
```

A composite reuses other asserts and stays `#[track_caller]`:

```rust
#[track_caller]
fn assert_dynamic_if_block_for_symbol(data: &AnalysisData, c: &Component, name: &str) {
    assert_symbol(data, name);
    assert_dynamic_if_block(data, c, name);
}
```

## Reuse before adding

- Factories: `analyze_source`, `analyze_source_with_diags`, `analyze_source_experimental_async`, `analyze_source_with_css`, `build_instance`, `build_instance_ts`, `parse`, `tokenize`, `parse_with_js`.
- Asserts: `assert_node`, `assert_if_block`, `assert_dynamic_tag`, `assert_each_block`, `assert_rune_kind`, `assert_diag_codes`, `assert_start_tag`, `assert_attributes`, … Grep `fn assert_` in the target crate first.
