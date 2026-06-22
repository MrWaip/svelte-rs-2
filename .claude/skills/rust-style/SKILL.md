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

## No negated condition with an `else`

`if !cond { a } else { b }` makes the reader mentally flip the test to see which branch runs when. When both branches exist, lead with the positive condition.

❌ Negation forces a double-take:
```rust
if !user.is_active {
    deactivate()
} else {
    activate()
}
```

✅ Positive first:
```rust
if user.is_active {
    activate()
} else {
    deactivate()
}
```

(A lone negated guard with no `else` — `if !ok { return }` — is fine; that's the early-return pattern.)

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

## Unfold long boolean chains into guard clauses

A long `||`/`&&` chain is hard to read — worse with a multi-statement `.any()` closure spliced in. Give it a `bool` function and write one guard clause per condition; iterate with a plain `for` that returns early.

❌ One long `||` chain with a statement-closure inside:
```rust
TopLevelForm::Call => {
    facts.has_runtime_root
        || facts.has_store_ref
        || reads_legacy_props_object(facts)
        || facts.references.iter().any(|&sym| {
            let semantics = reactivity.binding_semantics(sym);
            if matches!(semantics, BindingSemantics::MaybeReactive) {
                return true;
            }
            reactivity.symbol_is_volatile(scoping, sym)
                || scoping.is_component_top_level_symbol(sym)
        })
}
```

✅ One guard clause per condition, `false` falls out the bottom:
```rust
fn call_form_is_reactive(facts: &TopLevelFacts, reactivity: &Reactivity, scoping: &Scoping) -> bool {
    if facts.has_runtime_root {
        return true;
    }
    if facts.has_store_ref {
        return true;
    }
    if reads_legacy_props_object(facts) {
        return true;
    }
    for &symbol in &facts.references {
        if symbol_forces_reactive(reactivity, scoping, symbol) {
            return true;
        }
    }
    false
}
```
