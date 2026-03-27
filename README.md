# svelte-rs — Rust implementation of the Svelte compiler (WIP)

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/MrWaip/svelte-rs-2)

## Demo

https://mrwaip.github.io/svelte-rs-2/

## Architecture overview

https://excalidraw.com/#json=tPR4IJ3ZQmfRfF0xW1fif,Qw3c1g41YuyCLz1XmRcujw

---

## Feature checklist

See [ROADMAP.md](./ROADMAP.md) for the full feature checklist.

---

## Building the WASM package

```sh
wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
```



определение мутации и рун кривое сейчас.
тк мы сделали нормальный scoping в шаблоне и в скрипте мы можем юзать RefrenceFlags
нужно переписть руны и сторы на RefrenceFlags

также какжется можно чекать референсы быстрее чем через массив

потом нужно починить все тесты
потом переписать ast на индекс
