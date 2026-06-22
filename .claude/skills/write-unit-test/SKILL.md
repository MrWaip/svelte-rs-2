---
name: write-unit-test
description: Use when writing, editing, or reviewing a Rust `#[test]`, a test helper, or an `assert_*` in `crates/**/src/**`. The required unit-test format for this repo. Not for `tasks/compiler_tests/` (use add-test) or `tasks/diagnostic_tests/` (use add-diagnostic-test).
paths:
  - "crates/**/src/**/*.rs"
---

# Write unit test

Body is at most three lines — setup, optional execute, one custom assert:

```rust
#[test]
fn dynamic_expr_inside_svelte_element() {
    let (c, data) = analyze_source(r#"...<svelte:element this={"div"}>{name}</svelte:element>"#);
    assert_dynamic_tag(&data, &c, "name");
}
```

1. **setup** — a Creation Method (`analyze_source`, `build_instance`, `parse`) hiding parse/mocks/fixtures behind an intent-revealing name.
2. **execute** — optional; omit when the factory already runs the action (`analyze_source` parses *and* analyzes), keep it on its own line when the action is the point of the test (transform, codegen).
3. **assert** — required, exactly one custom `assert_*`.

Blank line between tests. No comments in the body.

## Rules

- **One assert per test.** Never stack `assert_a(); assert_b();`. Bundle the checks into one `assert_*` — it holds as many raw `assert_eq!`/`assert!` inside as needed.
- **Assert per entity, expectations as args.** Prefer one reusable assert keyed to the thing under test (`assert_start_tag(tok, src, name, self_closing)`) over a one-off helper per test. Converges to a shared vocabulary, not a long tail.
- **`#[track_caller]` on every `assert_*`.** Required — else the panic points inside the helper instead of the failing test.
- **Explicit messages.** Every inner check names the expectation and prints the actual: `assert_eq!(got, expected, "name: expected {expected:?}, got {got:?}")`. No bare `assert!(cond)`.
- **No raw asserts in the body** — only behind a named `assert_*`.
- **Name the test after the behaviour**, not the function: `reports_diagnostic_when_prop_unknown`.

## Helpers

Live in the same `#[cfg(test)] mod tests` / `tests.rs`. **Reuse before adding** — the repo has dozens of `assert_*` and `analyze_*`/`build_*` factories; grep `fn assert_` / `fn build_` in the target crate first.

## More

[EXAMPLES.md](EXAMPLES.md) — eight cases in one shape plus a well-formed `#[track_caller]` assert (names marked **real** if in-repo, **target** if to-be-added).

Terminology (Meszaros' xUnit / AAA): setup = **Creation Method**, the assert = **Custom Assertion**, one-assert rule = **Single-Condition Test**.
