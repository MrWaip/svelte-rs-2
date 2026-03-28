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
