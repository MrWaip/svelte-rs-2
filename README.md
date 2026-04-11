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

## Workflow

Проект использует Claude Code с набором специализированных команд и агентов.

### Начало сессии
`/status` — обзор проекта: активные спеки, ignored тесты, ROADMAP next, известный долг

### Портирование фичи
1. `/audit <feature>` — gap-анализ, создание spec + тестов
2. `/port specs/<file>.md` — реализация следующего slice по spec
3. `/qa` — проверка quality guidelines
4. `/sync-docs` — синхронизация ROADMAP/CODEBASE_MAP

### Починка теста
1. `/explain-test <name>` (опционально — понять что тестирует)
2. `/fix-test <name>` — диагностика + fix
3. `/qa` (опционально)

### Техдолг / рефакторинг
1. `/improve <description>` — диагностика + fix + тесты
2. `/qa`

### Исследование
- `/diagnose <component>` — прогнать repro через пайплайн, выделить root cause, добавить focused tests и записать follow-up в spec или `ROADMAP.md`
- `/audit <feature>` — gap-анализ vs reference compiler
- `/explain-test <name>` — что делает тест, почему падает
- `/bench` — Rust vs JS перформанс

### Обслуживание
- `/sync-docs` — синхронизация документации с кодом
- `/add-test <name>` — test-first: создать тест до реализации

---

## Building the WASM package

```sh
wasm-pack build --target web ./crates/wasm_compiler -d ../../docs/compiler
```
