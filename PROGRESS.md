# Progress

## Script / JavaScript

- [ ] `$state` rune
  - [ ] Update (`count++`, `--count`)
  - [ ] Assign (`count = 12`)
  - [ ] Read (`{ name }`)
- [ ] `$props` rune
  - [ ] Update prop (`count++`)
  - [ ] Assign prop (`name = 'world'`)
  - [ ] Read prop (`{ name }`)
  - [ ] Destructure (`let { name } = $props()`)
- [ ] Hoist imports
- [ ] Strip TypeScript

## Template

- [ ] HTML Element
  - [ ] Self-closed tags (`<input />`)
  - [x] Simple attribute (`attr="name"`)
  - [x] Interpolation attribute (`attr={expression}`)
  - [x] Shorthand attribute (`{ name }`)
  - [x] Concatenation attribute (`attr="{1 + 1} = 2"`)
  - [ ] Class directive (`class:toggle`)
  - [ ] Bind directive (`bind:value`)
  - [ ] Use directive (`use:action`)
  - [ ] Event listeners (`onclick={handler}`)
  - [ ] Style directive (`style:color`)
  - [ ] Spread attributes (`{...attrs}`)
- [x] Text node
- [x] Interpolation (`{name}`)
- [x] IfBlock (`{#if expr} … {:else} … {/if}`)
- [ ] EachBlock
- [ ] Component
- [ ] Script tag

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
