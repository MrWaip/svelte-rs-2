---
name: add-diagnostic-test
description: Create a new diagnostic parity test case. Use when the user asks to add a diagnostic test, create a new `tasks/diagnostic_tests/cases/<name>` case, or capture a false positive/false negative against npm `svelte/compiler` before implementation.
---

# Add Diagnostic Test

## 1) Validate the name

Use snake_case. Stop if `tasks/diagnostic_tests/cases/<name>/` already exists.

## 2) Create a minimal case

Add `tasks/diagnostic_tests/cases/<name>/case.svelte` with the smallest component that isolates one diagnostic behavior.

Keep the component focused. Extend an existing case instead of creating a new one only when the same diagnostic code or syntax family is already covered and the existing `case.svelte` is still small.

Add `config.json` only when the diagnostic depends on compile options.

## 3) Generate reference snapshots

Run:

```bash
just generate
```

This should create `case-svelte.json` from npm `svelte/compiler`.
Never hand-edit `case-svelte.json` or `case-rust.json`.

## 4) Register the test

Add a `#[rstest]` case to `tasks/diagnostic_tests/test_diagnostics.rs`:

```rust
#[rstest]
fn <name>() {
    assert_diagnostics("<name>");
}
```

## 5) Run the test

```bash
just test-diagnostic-case <name>
```

The test writes `case-rust.json` for human comparison. A failure is normal in test-first workflow. Report the mismatch clearly rather than trying to “fix” the reference snapshots.

## 6) Report

Include:

- what the diagnostic test covers
- whether it passed or failed
- mismatch summary if it fails
- the next recommended command, usually `diagnose-diagnostics` or `port specs/<feature>.md`
