# Current TODO

Next up: **Tier 2** — features needed for real apps.

## Active

| ID | Feature | Key references |
|----|---------|---------------|
| 2a | `{@html expr}` — `$.html()` codegen | `HtmlTag.js` |
| 2b | `{#key expr}` — `$.key()` codegen | `KeyBlock.js` |
| 2c | Event handlers (`onclick={handler}`) | `shared/events.js` |
| 2d | Style directive (`style:color={value}`) | `StyleDirective.js` |
| 2e | Transitions / Animations (`transition:`, `in:`, `out:`, `animate:`) | `TransitionDirective.js` |
| 2f | Component events & spread props | `Component.js` |
| 2g | `use:action` directive | `UseDirective.js` |

All references relative to `reference/compiler/phases/3-transform/client/visitors/`.

## Blocked / Deferred

- **3a-3d** Validation passes — not blocking codegen, do after Tier 2
- **3e** `{@const}` — needs scope-like handling, lower priority
