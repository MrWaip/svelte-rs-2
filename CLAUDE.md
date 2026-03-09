# Project Instructions

## Testing

All tests in `crates/svelte_parser` must follow the span-based pattern described in `/test-pattern`.

Rules:
- Use `assert_node`, `assert_script`, `assert_if_block` helpers (defined in the test module)
- No inline `if let Node::...` structural checks — use helpers instead
- Add new `assert_<node_type>` helpers when new node types need testing
- Exception: `assert!(result.is_err())` for error tests needs no helper

When writing or modifying any test in `svelte_parser`, apply `/test-pattern` automatically.
