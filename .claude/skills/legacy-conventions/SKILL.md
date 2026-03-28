---
name: legacy-conventions
description: MUST consult before writing any code related to Svelte 4 legacy features. Contains mandatory naming conventions, doc-comment format, and isolation rules for legacy code. Use this skill whenever touching on:directive, bind:this legacy syntax, Svelte 4 deprecated features, code marked LEGACY(svelte4), or when porting/implementing any feature that has both a Svelte 5 and a Svelte 4 variant. Failure to follow these conventions creates tech debt that blocks future cleanup.
paths:
  - "**/*.rs"
---

# Legacy Feature Conventions (Svelte 4)

Legacy Svelte 4 syntax (deprecated in Svelte 5, scheduled for removal in Svelte 6) is ported with isolation in mind so it can be cleanly deleted later.

## Conventions

1. **`Legacy` suffix** in all type/function names: `OnDirectiveLegacy`, `gen_on_directive_legacy`, `build_legacy_event_handler`.
2. **`LEGACY(svelte4):` doc-comment** on every struct, enum variant, and top-level function:
   ```rust
   /// LEGACY(svelte4): on:directive syntax. Deprecated in Svelte 5, remove in Svelte 6.
   pub struct OnDirectiveLegacy { ... }
   ```
   Short inline comments use the same tag: `// LEGACY(svelte4): on:directive handled separately`.
3. **Easy removal** — keep legacy code in self-contained blocks/functions. Avoid mixing legacy logic into non-legacy code paths. Ideal: grep `LEGACY(svelte4)` -> delete all hits -> compile -> done.
