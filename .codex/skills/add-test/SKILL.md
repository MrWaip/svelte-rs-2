---
name: add-test
description: Create a new compiler test case for test-first work. Use when the user asks to add a compiler test, create a new `tasks/compiler_tests/cases2/<name>` case, or capture a feature/bug with a focused end-to-end snapshot before implementation.
---

# Add Compiler Test

## 1) Validate the name

Use snake_case. Stop if `tasks/compiler_tests/cases2/<name>/` already exists.

## 2) Create a minimal case

Add `tasks/compiler_tests/cases2/<name>/case.svelte` with the smallest component that isolates one feature or edge case.

Keep the component focused. Extend an existing case instead of creating a new one only when the same feature is already covered and the existing `case.svelte` is still small.

## 3) Generate expected output

Run:

```bash
just generate
```

This should create `case-svelte.js` from the reference compiler. Never hand-edit `case-svelte.js` or `case-rust.js`.

## 4) Register the test

Add a `#[rstest]` case to `tasks/compiler_tests/test_v3.rs`:

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

A failure is normal in test-first workflow. Report the failure clearly rather than trying to “fix” the expected snapshots.

## 6) Report

Include:

- what the test covers
- whether it passed or failed
- diff summary if it fails
- the next recommended command, usually `fix-test <name>` or `port specs/<feature>.md`
