---
name: add-diagnostic-test
description: Creates a diagnostic parity test case under `tasks/diagnostic_tests/cases/<name>`. Use when adding a diagnostic test, or when capturing a false positive/false negative against npm `svelte/compiler` before implementation.
---

# Add Diagnostic Test

> **Capture utility — not part of the PARITY chain.**
> Creates a persistent diagnostic case for feature-first work. Do NOT use when you already have a `dig-better` finding — that work lives under `tasks/diagnostic_tests/cases/` and is scoped by `/dig-better`. Use this skill only when registering a fresh, standalone diagnostic case before implementation.

## 1) Validate the name

snake_case. Stop if `tasks/diagnostic_tests/cases/<name>/` exists.

## 2) Create a minimal case

Add `tasks/diagnostic_tests/cases/<name>/case.svelte` — the smallest component isolating one diagnostic behavior. Extend an existing case instead only when it already covers the same diagnostic code or syntax family and its `case.svelte` stays small.

Add `config.json` only when the diagnostic depends on compile options.

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
