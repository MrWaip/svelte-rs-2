---
name: add-test
description: Create a new compiler test case for test-first development. Use when the user asks to "add a test", "create test case", or wants to test a specific Svelte feature before implementing it.
user-invocable: true
argument-hint: "[test-case-name]"
allowed-tools: Bash, Read, Write, Edit, Grep, Glob
---

# Add compiler test case: $ARGUMENTS

Create a new compiler test case for test-first development. The test is expected to fail initially.

## Step 1: Validate name

Check that `tasks/compiler_tests/cases2/$ARGUMENTS/` does not already exist. If it does, stop and report.

## Step 2: Create case.svelte

Create `tasks/compiler_tests/cases2/$ARGUMENTS/case.svelte` with a minimal Svelte component that exercises the feature described by the test name. Include `<script>` block with relevant runes if needed, and template markup.

Keep it minimal — just enough to test the feature. Look at existing cases for style reference.

## Step 3: Generate expected output

Run:
```
just generate
```

This creates `case-svelte.js` (expected output from reference Svelte compiler). Verify the file was created.

## Step 4: Add test function

Append a new `#[rstest]` test function to `tasks/compiler_tests/test_v3.rs`:

```rust
#[rstest]
fn $ARGUMENTS() {
    assert_compiler("$ARGUMENTS");
}
```

## Step 5: Run the test

```
just test-case $ARGUMENTS
```

## Step 6: Report

Report the result. A failing test is expected and normal — this is test-first workflow. Show the diff summary if the test fails.

## Rules

- NEVER edit `case-svelte.js` or `case-rust.js` — these are generated files
- Keep `case.svelte` minimal and focused on one feature
- The test name must use snake_case
