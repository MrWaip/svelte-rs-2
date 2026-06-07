---
name: rust-style
description: Use when writing, editing, or refactoring any .rs file.
paths:
  - "**/*.rs"
---

# Rust Style

Non-negotiable practices for every `.rs` change.

## Early return (guard clauses)

Handle the exceptional/empty case first and `return` (or `?`/`continue`/`break`) immediately. Keep the happy path at the lowest indentation level. Never wrap the main logic in `else`.

❌ Nested — the real work is buried:
```rust
fn primary_email(user: Option<&User>) -> Result<String, Error> {
    if let Some(user) = user {
        if user.is_active {
            if let Some(email) = &user.email {
                Ok(email.clone())
            } else {
                Err(Error::NoEmail)
            }
        } else {
            Err(Error::Inactive)
        }
    } else {
        Err(Error::NotFound)
    }
}
```

✅ Early return — happy path is flat and last:
```rust
fn primary_email(user: Option<&User>) -> Result<String, Error> {
    let user = user.ok_or(Error::NotFound)?;
    if !user.is_active {
        return Err(Error::Inactive);
    }
    let email = user.email.as_ref().ok_or(Error::NoEmail)?;
    Ok(email.clone())
}
```

Tools: `?`, `ok_or`, `let ... else { return }`, early `continue`/`break` in loops.

## Return errors, never panic

Use `Result` instead of `panic!`, `unwrap`, `expect`, `unreachable!`, `unimplemented!`, `todo!`, or any other panicking construct. Propagate errors with `?`.

❌ Panics on bad input:
```rust
fn binding(&self, symbol: SymbolId) -> &Binding {
    self.by_symbol.get(&symbol).expect("missing binding")
}
```

✅ Returns an error the caller can handle:
```rust
fn binding(&self, symbol: SymbolId) -> Result<&Binding, AnalyzeError> {
    self.by_symbol
        .get(&symbol)
        .ok_or(AnalyzeError::MissingBinding(symbol))
}
```

## Exhaustive match over domain enums

Match every variant explicitly instead of falling back to `_`. A wildcard arm silently swallows new variants — when the enum grows, the compiler stays quiet and you get a bug. Spelling out each arm forces every call site to be revisited.

This applies to domain enums you own (e.g. `BindingSemantics` — cover every branch). It does not apply to huge foreign enums where listing all arms is noise rather than safety — `oxc::Expression` has 30+ variants, there a `_` is fine.

❌ Wildcard hides a missed variant:
```rust
match binding {
    BindingSemantics::Derived(_) => emit_derived(),
    _ => emit_plain(),
}
```

✅ Every domain variant handled:
```rust
match binding {
    BindingSemantics::Derived(_) => emit_derived(),
    BindingSemantics::State(_) => emit_state(),
    BindingSemantics::Prop(_) => emit_prop(),
    BindingSemantics::Plain => emit_plain(),
}
```

`matches!` has the same trap — anything not listed silently becomes `false`, so a new variant never lights up. For domain enums, prefer an exhaustive `match` returning `bool`.

❌ New variant silently falls through as `false`:
```rust
let is_reactive = matches!(binding, BindingSemantics::Derived(_) | BindingSemantics::State(_));
```

✅ Exhaustive — adding a variant forces a decision:
```rust
let is_reactive = match binding {
    BindingSemantics::Derived(_) | BindingSemantics::State(_) => true,
    BindingSemantics::Prop(_) | BindingSemantics::Plain => false,
};
```

## Human-readable code

Write human-friendly code, don't write one-liners like leet code.

❌ Multi-line closure crammed into a chain:
```rust
let active_ids: Vec<_> = sessions
    .iter()
    .filter(|session| {
        session.is_active
            && session.last_seen > cutoff
            && session.region == current_region
    })
    .map(|session| session.id)
    .collect();
```

✅ Explicit loop, each step named:
```rust
let mut active_ids = Vec::new();
for session in &sessions {
    let is_active = session.is_active && session.last_seen > cutoff;
    if is_active && session.region == current_region {
        active_ids.push(session.id);
    }
}
```
