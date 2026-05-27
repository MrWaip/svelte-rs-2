---
name: add-test
description: Create a new compiler test case for test-first work. Use when the user asks to add a compiler test, create a new `tasks/compiler_tests/cases2/<name>` case, or capture a feature/bug with a focused end-to-end snapshot before implementation.
---

# Add Compiler Test

## 1) Validate the name

snake_case. Stop if `tasks/compiler_tests/cases2/<name>/` exists.

## 2) Create a minimal case

Add `tasks/compiler_tests/cases2/<name>/case.svelte` with smallest component isolating one feature or edge case.

Keep focused. Extend existing case instead of new one only when same feature already covered and existing `case.svelte` still small.

## 3) Generate expected output

Run:

```bash
just generate
```

Creates `case-svelte.js` from reference compiler. Never hand-edit `case-svelte.js` or `case-rust.js`.

## 4) Register the test

Add `#[rstest]` case to `tasks/compiler_tests/test_v3.rs`:

```rust
#[rstest]
fn <name>() {
    assert_compiler("<name>");
}
```

## 5) Run the test

```bash
just test-case <name>
```

Failure normal in test-first workflow. Report failure clearly. Do not "fix" expected snapshots.

## 6) Report

Include:

- what test covers
- pass/fail
- diff summary if fails
- next recommended command, usually `/fix-test <name>` or `/port specs/<feature>.md`
