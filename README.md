# svelte-rs — Rust implementation of the Svelte compiler (WIP)

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs-2)

## Demo

https://mrwaip.github.io/svelte-rs-2/

## Architecture overview

https://excalidraw.com/#json=tPR4IJ3ZQmfRfF0xW1fif,Qw3c1g41YuyCLz1XmRcujw

---

## Feature checklist

### Script / JavaScript

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

### Template

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

### Optimizations

- [x] Whitespace trimming
- [x] Merge adjacent text/interpolation sequences
- [x] First-node-is-text optimization
- [x] Single-element optimization
- [x] Text-and-interpolation-only optimization
- [x] Non-reactive attribute optimization
- [ ] Skip wrapping runes that are never mutated

### WASM

- [x] Compiler compiled to WASM for browser use

### SSR

- [ ] Not implemented

### Style

- [ ] Not implemented

### Skipped / out of scope

- Namespace support

---

## Building the WASM package

```sh
wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
```
