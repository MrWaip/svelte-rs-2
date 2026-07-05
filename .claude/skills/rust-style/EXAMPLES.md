# Examples

Worked ❌/✅ pairs for each rule in [SKILL.md](SKILL.md), in the same order.

## Early return (guard clauses)

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

## Return errors, never panic

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

## Human-readable code

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
