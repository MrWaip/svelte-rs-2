---
name: test-pattern
description: >
  Pattern for writing Rust unit tests. Core idea: setup, navigate the tree step-by-step,
  assert at leaves with reusable helpers. MUST consult before writing or modifying any test.
  Also use when existing tests are too verbose, when adding a new assert helper, or when
  reviewing test code.
---

# Rust Test Pattern

## Concept

**Setup → navigate the tree → assert at leaves with helpers.**

Every test reads as a top-down walk through the expected shape.
Setup and navigation live in small reusable helpers; the test body
is a concise declaration of intent.

In this project, nodes are identified by source text (span) rather than
by indices or NodeId, because AST indices break when the parser is refactored
while source text stays stable.

---

## Three layers of helpers

### Layer 1: Setup — prepare input data

One function per pipeline stage. Panics on unexpected errors so the test
body never deals with infrastructure.

```rust
fn parse(source: &str) -> Component {
    let (component, _) = Parser::new(source).parse();
    component
}

fn analyze_source(source: &str) -> (Component, AnalysisData) {
    let alloc = oxc_allocator::Allocator::default();
    let (component, js_result, parse_diags) = svelte_parser::parse_with_js(&alloc, source);
    assert!(parse_diags.is_empty(), "unexpected parse diagnostics: {parse_diags:?}");
    let (data, _parsed, diags) = analyze(&component, js_result);
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
    (component, data)
}
```

### Layer 2: Finders — navigate to a specific node

Recursive tree traversal, identification via `component.source_text(span)`.
Return `Option` — the panic on absence belongs in the assert helper, not here.

**Naming:** `find_*` — returns `Option<NodeId>` or `Option<&Node>`.

```rust
fn find_expr_tag(fragment: &Fragment, component: &Component, target: &str) -> Option<NodeId>
fn find_element<'a>(fragment: &'a Fragment, component: &'a Component, tag_name: &str) -> Option<&'a Element>
fn find_if_block<'a>(fragment: &'a Fragment, component: &'a Component, test_text: &str) -> Option<&'a IfBlock>
fn find_each_block<'a>(fragment: &'a Fragment, component: &'a Component, expr_text: &str) -> Option<&'a EachBlock>
```

### Layer 3: Assertions — leaf-level checks

Each helper checks one aspect. Panics with a contextual message on failure.

**Naming:** `assert_*` — returns nothing, panics on mismatch.

```rust
// --- Symbols & runes ---
fn assert_symbol(data: &AnalysisData, name: &str)
fn assert_is_rune(data: &AnalysisData, name: &str)
fn assert_rune_kind(data: &AnalysisData, name: &str, expected: RuneKind)
fn assert_rune_is_mutated(data: &AnalysisData, name: &str)
fn assert_rune_not_mutated(data: &AnalysisData, name: &str)

// --- Dynamic nodes ---
fn assert_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str)
fn assert_not_dynamic_tag(data: &AnalysisData, component: &Component, expr_text: &str)
fn assert_dynamic_if_block(data: &AnalysisData, component: &Component, test_text: &str)
fn assert_dynamic_each(data: &AnalysisData, component: &Component, expr_text: &str)

// --- Content strategy ---
fn assert_root_content_type(data: &AnalysisData, expected: ContentStrategy)
fn assert_content_strategy_variant(data: &AnalysisData, key: FragmentKey, variant: &str)
fn assert_element_content_type(data: &AnalysisData, component: &Component, tag_name: &str, expected: ContentStrategy)
fn assert_consequent_content_type(data: &AnalysisData, component: &Component, test_text: &str, expected: ContentStrategy)

// --- Lowered fragments ---
fn assert_lowered_item_count(data: &AnalysisData, key: FragmentKey, expected_count: usize)
fn assert_item_is_text_concat(data: &AnalysisData, key: FragmentKey, index: usize)

// --- Expression info ---
fn assert_expr_tag_has_call(data: &AnalysisData, component: &Component, expr_text: &str)
fn assert_expr_tag_no_call(data: &AnalysisData, component: &Component, expr_text: &str)

// --- Parser ---
fn assert_node(c: &Component, index: usize, expected: &str)
fn assert_script(c: &Component, expected: &str)
fn assert_if_block(c: &Component, index: usize, expected_test: &str)
```

---

## Chained navigation

For deeply nested structures, chain `find_*` calls. The chain itself
documents the expected shape:

```rust
let el = find_element(&component.fragment, &component, "div").unwrap();
let inner_block = find_if_block(&el.fragment, &component, "show").unwrap();
assert_dynamic_if_block_by_id(&data, inner_block.id);
```

If a chain gets longer than 3 levels, break it into named variables —
each variable name documents what that node represents in the test:

```rust
let list = find_each_block(&component.fragment, &component, "items").unwrap();
let row = find_element(&list.body, &component, "li").unwrap();
let badge = find_element(&row.fragment, &component, "span").unwrap();
assert_element_content_type(&data, &component, "span", ContentStrategy::DynamicText);
```

The navigation path IS the test specification — it tells you exactly what
shape the tree should have. Don't hide this path inside a setup helper.

---

## Examples

### Parser

```rust
#[test]
fn smoke() {
    let c = parse("prefix <div>text</div>");
    assert_node(&c, 0, "prefix ");
    assert_node(&c, 1, "<div>text</div>");
}
```

### Analyzer

```rust
#[test]
fn rune_detection() {
    let (c, data) = analyze_source(
        r#"<script>let count = $state(0); count = 1;</script><p>{count}</p>"#,
    );
    assert_symbol(&data, "count");
    assert_is_rune(&data, "count");
    assert_dynamic_tag(&data, &c, "count");
}

#[test]
fn if_block_test_is_dynamic() {
    let (c, data) = analyze_source(
        r#"<script>let show = $state(true); show = false;</script>{#if show}<p>hi</p>{/if}"#,
    );
    assert_symbol(&data, "show");
    assert_is_rune(&data, "show");
    assert_dynamic_if_block(&data, &c, "show");
}
```

---

## Test module structure

Tests live in a separate file, not inline in the source module. This keeps
the main source focused and avoids blowing up context when reading implementation code.

```
crates/svelte_analyze/src/
  lib.rs            // mod tests;
  tests.rs          // helpers + #[test] functions
```

When the number of tests grows large, split by domain into a directory:

```
crates/svelte_analyze/src/
  lib.rs            // mod tests;
  tests/
    mod.rs          // shared helpers (setup, finders, assertions)
    runes.rs        // rune-related tests
    content.rs      // content strategy tests
    lowering.rs     // fragment lowering tests
```

Helpers stay in `mod.rs` (or a dedicated `helpers.rs` re-exported from `mod.rs`)
so all test files can import them. Test files contain only `#[test]` functions
and any test-local constants.

### Compiler tests (codegen)

Compiler tests live in `tasks/compiler_tests/test_v3.rs` and use a different pattern:
`rstest` + `pretty_assertions` for colored diff output when comparing generated JS
against expected output. Each test calls `assert_compiler("case_name")` which reads
a `.svelte` file, compiles it, and diffs the JS output against a snapshot.

```rust
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
fn some_feature() {
    assert_compiler("some_feature");
}
```

This pattern is specific to end-to-end compiler output comparison —
unit tests in parser/analyze crates use the span-based helpers described above.

---

## When to write a new helper

- **New `find_*`**: when you first encounter a new node type in tests
- **New `assert_*`**: when the same check appears in 2+ tests
- **New setup fn**: when a different pipeline depth is needed (e.g., analysis expecting errors)

Each helper is 5-10 lines. Write them as you go.

Template for a new `find_*`:
```rust
fn find_<node_type><'a>(
    fragment: &'a Fragment,
    component: &'a Component,
    identifying_text: &str,
) -> Option<&'a NodeType> {
    let store = &component.store;
    for &id in &fragment.nodes {
        if let Node::NodeType(n) = store.get(id) {
            if component.source_text(n.relevant_span) == identifying_text {
                return Some(n);
            }
        }
    }
    None
}
```

Template for a new `assert_*`:
```rust
fn assert_<aspect>(data: &AnalysisData, component: &Component, identifying_text: &str) {
    let node = find_<node_type>(&component.fragment, component, identifying_text)
        .unwrap_or_else(|| panic!("no <NodeType> with source '{identifying_text}'"));
    assert!(
        data.some_property.contains(&node.id),
        "expected <NodeType> '{identifying_text}' to have <property>"
    );
}
```

---

## Anti-Patterns

### Inline pattern match instead of a helper

```rust
// BAD: hides intent, duplicates logic across tests
if let Node::IfBlock(ref ib) = component.store.get(fragment.nodes[0]) {
    assert_eq!(component.source_text(ib.test_span), "show");
    assert!(data.dynamic_nodes.contains(&ib.id));
}

// GOOD: helpers make intent obvious
assert_dynamic_if_block(&data, &c, "show");
```

### Direct collection length check

```rust
// BAD: uninformative panic — "assertion failed: 1 != 2"
assert_eq!(data.dynamic_nodes.len(), 1);

// GOOD: checks a specific node, panic tells you WHAT went wrong
assert_dynamic_tag(&data, &c, "count");
```

### Identification by index or NodeId

```rust
// BAD: breaks when parse order changes
let node_id = component.fragment.nodes[2];

// GOOD: stable — tied to source text
find_expr_tag(&component.fragment, &component, "count")
```

### `unwrap()` without context

```rust
// BAD: "called unwrap() on None" — useless for debugging
let sym = data.scoping.find_binding(root, name).unwrap();

// GOOD: tells you exactly what wasn't found
let sym = data.scoping.find_binding(root, name)
    .unwrap_or_else(|| panic!("no symbol '{name}'"));
```

### Hiding navigation in a setup helper

```rust
// BAD: hides the expected tree shape — the reader can't see
// that we expect an IfBlock inside a div
fn get_nested_if(source: &str) -> &IfBlock { ... }

// GOOD: navigation chain IS the test spec
let el = find_element(&component.fragment, &component, "div").unwrap();
let block = find_if_block(&el.fragment, &component, "show").unwrap();
```

---

## Rules

1. Use `find_*` / `assert_*` helpers — no inline pattern matching in test bodies
2. Identify nodes by source text (span), not by indices or NodeId
3. When adding a new node type — add a `find_*` helper
4. When a check repeats in 2+ tests — add an `assert_*` helper
5. Panic messages always include the identifying text and what was expected
6. Exception: `assert!(result.is_err())` for error tests — no helper needed
7. **Existing tests are not a reference.** Some older tests were written before these
   conventions — with inline matches, indices, no helpers. Do not copy their style.
   When modifying an old test — rewrite it to follow this pattern.
   When writing a new test — follow only this skill, not the surrounding code.
