---
name: add-diagnostic-test
description: Create a new diagnostic parity test case. Use when the user asks to add a diagnostic test, create a new `tasks/diagnostic_tests/cases/<name>` case, or capture a false positive/false negative against npm `svelte/compiler` before implementation.
---

# Add Diagnostic Test

## 1) Validate the name

snake_case. Stop if `tasks/diagnostic_tests/cases/<name>/` exists.

## 2) Create a minimal case

Add `tasks/diagnostic_tests/cases/<name>/case.svelte` with smallest component isolating one diagnostic behavior.

Keep focused. Extend existing case instead of new one only when same diagnostic code or syntax family already covered and existing `case.svelte` still small.

Add `config.json` only when diagnostic depends on compile options.

## 3) Generate reference snapshots

Run:

```bash
just generate
```

Creates `case-svelte.json` from npm `svelte/compiler`.
Never hand-edit `case-svelte.json` or `case-rust.json`.

## 4) Register the test

Add `#[rstest]` case to `tasks/diagnostic_tests/test_diagnostics.rs`:

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

Test writes `case-rust.json` for human comparison. Failure normal in test-first workflow. Report mismatch clearly. Do not "fix" reference snapshots.

## 6) Report

Include:

- what diagnostic test covers
- pass/fail
- mismatch summary if fails
- next recommended command, usually `/diagnose-diagnostics` or `/port specs/<feature>.md`
