---
name: legacy-conventions
description: Conventions for Svelte 4 legacy features and deprecated syntax. Use when touching `on:` directives, legacy `bind:this` behavior, code marked `LEGACY(svelte4)`, or any implementation that must keep Svelte 4 logic isolated from the Svelte 5 path so it can be deleted cleanly later.
---

# Legacy Conventions

Keep Svelte 4 legacy code easy to find and easy to delete later.

## Naming

Use the `Legacy` suffix in type and function names:

- `OnDirectiveLegacy`
- `gen_on_directive_legacy`
- `build_legacy_event_handler`

## Comments

Annotate every legacy struct, enum variant, or top-level function with:

```rust
/// LEGACY(svelte4): on:directive syntax. Deprecated in Svelte 5, remove in Svelte 6.
```

Use the same tag for short inline comments when needed:

```rust
// LEGACY(svelte4): handled separately from the Svelte 5 path
```

## Isolation

Keep legacy logic self-contained. Do not smear Svelte 4 branches across modern code paths if a dedicated helper or block can isolate them.

The ideal cleanup path is:

1. grep `LEGACY(svelte4)`
2. delete those sites
3. compile
