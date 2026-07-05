---
name: rust-style
description: Rust style rules for any .rs change.
paths:
  - "**/*.rs"
---

# Rust Style

Non-negotiable. Worked ❌/✅ pairs for each rule: [EXAMPLES.md](EXAMPLES.md).

## Early return (guard clauses)

Handle the exceptional/empty case first and `return` (or `?`/`continue`/`break`) immediately. Keep the happy path at the lowest indentation level. Never wrap the main logic in `else`. Tools: `?`, `ok_or`, `let ... else { return }`, early `continue`/`break` in loops.

## Return errors, never panic

Use `Result` instead of `panic!`, `unwrap`, `expect`, `unreachable!`, `unimplemented!`, `todo!`, or any other panicking construct. Propagate errors with `?`. This is absolute — even a branch you can prove unreachable returns an error, never `unreachable!`. Scope is production code; under `#[cfg(test)]` / `tests.rs`, `assert!`/`assert_eq!` and fixture `unwrap` are the expected mechanism (see write-unit-test).

## Exhaustive match over domain enums

Match every variant explicitly instead of falling back to `_`. A wildcard arm silently swallows new variants — when the enum grows, the compiler stays quiet and you get a bug. Spelling out each arm forces every call site to be revisited. This applies to domain enums you own (e.g. `BindingSemantics` — cover every branch). It does not apply to huge foreign enums where listing all arms is noise rather than safety — `oxc::Expression` has 30+ variants, there a `_` is fine.

`matches!` has the same trap — anything not listed silently becomes `false`, so a new variant never lights up. For domain enums, prefer an exhaustive `match` returning `bool`.

## No negated condition with an `else`

`if !cond { a } else { b }` makes the reader mentally flip the test to see which branch runs when. When both branches exist, lead with the positive condition. A lone negated guard with no `else` — `if !ok { return }` — is fine; that's the early-return pattern.

## Human-readable code

Prefer an explicit loop with named steps over a crammed iterator chain — no code-golf one-liners.

## Unfold long boolean chains into guard clauses

The early-return move from above, applied to boolean logic. A long `||`/`&&` chain is hard to read — worse with a multi-statement `.any()` closure spliced in. Give it a `bool` function and write one guard clause per condition; iterate with a plain `for` that returns early.
