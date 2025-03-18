# Rust implementation of svelte compile (WIP)

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs-2)

# Demo

https://mrwaip.github.io/svelte-rs-2/

# Checklist

# Passes

https://excalidraw.com/#json=tPR4IJ3ZQmfRfF0xW1fif,Qw3c1g41YuyCLz1XmRcujw

## Script Tag / JavaScript

- [ ] $state rune
  - [ ] Update rune (`count++, --count`)
  - [ ] Assign rune (`count = 12`)
  - [ ] Read rune (`{ name }`)
- [ ] $props rune
  - [ ] Update prop (`count++`)
  - [ ] Assign prop (`name = 'world'`)
  - [ ] Read prop (`{ name }`)
  - [ ] Destructure prop (`let {name} = $props()`)
- [ ] Hoist imports
- [ ] Omit typescript

## Template

- [ ] HTML Element
  - [ ] Self-closed tags (`<input />`)
  - [x] Simple attribute (`attr="name"`)
  - [x] Interpolation attribute (`attr={some expression}`)
  - [x] Shorthand identifier attribute (`{ name }`)
  - [x] Concatenation attribute (`attr="{1 + 1} = 2"`)
  - [ ] Class directive (`class:toggle`)
  - [ ] Bind directive (`bind:value`)
  - [ ] Use directive (`use:action`)
  - [ ] Event listeners (`onclick={handler}`)
  - [ ] Style directive (`style:toggle`)
  - [ ] Spread attributes (`{...attrs}`)
- [x] Text
- [x] Interpolation (`{name}`)
- [x] IfBlock (`{#if expression} a {:else} b {/if}`)
- [ ] Script Tag
- [ ] Component
- [ ] EachBlock

## Svelte optimization

- [x] trimming whitespaces
- [x] compress / merge sequence of interpolation and text
- [x] text as first node optimization
- [x] single element optimization
- [x] only text and interpolation nodes optimization
- [ ] not wrap runes if not mutated
- [x] Not reactive attributes optimization

## Skipped

- Namespace support

## WASM

- [x] Compiler in wasm, for browser

## Style

- [-] unimplemented at all

## SSR

- [-] unimplemented at all
