# svelte-rs — Rust implementation of the Svelte compiler (WIP)

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs-2)

## Demo

https://mrwaip.github.io/svelte-rs-2/

## Architecture overview

https://excalidraw.com/#json=tPR4IJ3ZQmfRfF0xW1fif,Qw3c1g41YuyCLz1XmRcujw

---

## Feature checklist

See [PROGRESS.md](./PROGRESS.md) for the full feature checklist.

---

## Building the WASM package

```sh
wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
```
