# Rust implementation of svelte compile (WIP)

# Demo

https://mrwaip.github.io/svelte-rs-2/

# Checklist

## Script Tag / JavaScript

- [x] $state rune
- [x] Update runes (`count++, --count`)
- [x] Assign runes (`count = 12`)
- [x] Referencing runes (`{ name }`)

## Template

- [ ] HTML Element
  - [x] Self-closed tags (`<input />`)
  - [x] Simple attribute (`attr="name"`)
  - [x] Interpolation attribute (`attr={some expression}`)
  - [x] Shorthand identifier attribute (`{ name }`)
  - [x] Concatenation attribute (`attr="{1 + 1} = 2"`)
  - [ ] Svelte directives (`use:action={}`)
  - [ ] Class shortcut (`class:visible`)
  - [ ] Spread attributes (`{...attrs}`)
- [x] Text
- [x] Interpolation (`{name}`)
- [x] IfBlock (`{#if expression} a {:else} b {/if}`)
- [x] Script Tag
- [ ] Component
- [ ] EachBlock

## Svelte optimization

- [x] trimming whitespaces
- [x] compress / merge sequence of interpolation and text
- [x] text as first node optimization
- [x] single element optimization
- [x] only text and interpolation nodes optimization
- [x] not wrap runes if not mutated

## WASM

- [x] Compiler in wasm, for browser

## Style

- [-] unimplemented at all

## SSR

- [-] unimplemented at all
