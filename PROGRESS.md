# Progress

## Script / JavaScript

- [x] `$state` rune
  - [x] Update (`count++`, `--count`)
  - [x] Assign (`count = 12`)
  - [x] Read (`{ name }`)
- [x] `$props` rune
  - [x] Update prop (`count++`)
  - [x] Assign prop (`name = 'world'`)
  - [x] Read prop (`{ name }`)
  - [x] Destructure (`let { name } = $props()`)
- [x] Hoist imports
- [x] Strip TypeScript

## Template

- [x] HTML Element
  - [ ] Self-closed tags (`<input />`)
  - [x] Simple attribute (`attr="name"`)
  - [x] Interpolation attribute (`attr={expression}`)
  - [x] Shorthand attribute (`{ name }`)
  - [x] Concatenation attribute (`attr="{1 + 1} = 2"`)
  - [x] Class directive (`class:toggle`)
  - [x] Bind directive (`bind:value`)
  - [ ] Use directive (`use:action`)
  - [ ] Event listeners (`onclick={handler}`)
  - [ ] Style directive (`style:color`)
  - [x] Spread attributes (`{...attrs}`)
- [x] Text node
- [x] Interpolation (`{name}`)
- [x] IfBlock (`{#if expr} … {:else} … {/if}`)
- [x] EachBlock
- [x] Component (with props and children)
- [x] Snippet (`{#snippet}` / `{@render}`)
- [ ] Script tag (inline `<script>` codegen)

## Optimizations

- [x] Whitespace trimming
- [x] Merge adjacent text/interpolation sequences
- [x] First-node-is-text optimization
- [x] Single-element optimization
- [x] Text-and-interpolation-only optimization
- [x] Non-reactive attribute optimization
- [ ] Skip wrapping runes that are never mutated

## WASM

- [x] Compiler compiled to WASM for browser use

## SSR

- [ ] Not implemented

## Style

- [ ] Not implemented

## Skipped / out of scope

- Namespace support
