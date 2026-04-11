---
name: test-pattern
description: Pattern for writing or reviewing Rust unit tests in this project. Use when adding parser or analyzer tests, when existing tests became too verbose, when adding reusable assert helpers, or when deciding how to structure test modules and span-based node lookup.
---

# Rust Test Pattern

Write tests as: setup, navigate the tree, assert at the leaves with helpers.

## Core rule

Keep test bodies declarative. Infrastructure belongs in helpers, not inline in each test.

In this codebase, identify nodes by source text or stable structure, not by fragile AST indices or temporary ids.

## Helper layers

### 1) Setup helpers

Create small helpers that prepare parsed or analyzed data and panic on unexpected infrastructure errors.

```rust
fn parse(source: &str) -> Component { ... }
fn analyze_source(source: &str) -> (Component, AnalysisData) { ... }
```

### 2) Finder helpers

Traverse the tree and return `Option<_>`. Finder helpers should not panic; missing-node failure belongs in assertion helpers or the test body.

Naming:

- `find_element`
- `find_if_block`
- `find_each_block`
- `find_expr_tag`

### 3) Assertion helpers

Check one thing per helper and panic with contextual messages on mismatch.

Naming:

- `assert_symbol`
- `assert_is_rune`
- `assert_dynamic_tag`
- `assert_root_content_type`

## Navigation style

Chain small `find_*` helpers to document expected structure.

```rust
let list = find_each_block(&component.fragment, &component, "items").unwrap();
let row = find_element(&list.body, &component, "li").unwrap();
let badge = find_element(&row.fragment, &component, "span").unwrap();
assert_element_content_type(&data, &component, "span", ContentStrategy::DynamicText);
```

If the chain gets longer than three steps, split it into named variables. The variable names should explain the expected structure.

## Anti-patterns

- inline pattern matching in test bodies instead of helper-based assertions
- assertions on collection lengths when a named node-level assertion would be clearer
- node lookup by index or temporary id instead of source text or stable structure
- bare `unwrap()` without a panic message that explains what was expected

## Module structure

Prefer separate test files over inline tests in implementation modules.

Small test surface:

```text
crates/svelte_analyze/src/
  lib.rs
  tests.rs
```

Larger test surface:

```text
crates/svelte_analyze/src/
  lib.rs
  tests/
    mod.rs
    runes.rs
    content.rs
    lowering.rs
```

Put shared helpers in `mod.rs` or `helpers.rs`. Keep leaf test files focused on `#[test]` bodies.

## Compiler tests

End-to-end compiler tests are different from unit tests. For compiler output comparisons, use the existing `rstest` plus `pretty_assertions` pattern in `tasks/compiler_tests/test_v3.rs` and compare generated JS against the expected snapshot through the project helpers.

For diagnostic parity against npm `svelte/compiler`, use `tasks/diagnostic_tests/test_diagnostics.rs` and compare `case-rust.json` against `case-svelte.json`. Keep diagnostics-only behavior out of `tasks/compiler_tests/`.

## When to add a new helper

Add a helper when the same navigation or assertion logic appears in two or more tests, or when a test body starts exposing internal implementation details instead of expected behavior.
