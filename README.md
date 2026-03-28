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
обновить доку
пофиксить проблему в примере из playground
объединить тесты дубли
потом переписать ast на индекс


критерии 
системность
без костылей и временных решений
oxc visit / template visit по максимуму
работа с идентификаторами по symbolId / referenceId
полнота покрытия js синтаксиса
отсутствие неявных зависимостей и контрактов

Финальный список
Ad hoc решения — код туда, где удобно, а не где правильно
Не думаю перед кодом — нет фазы планирования
Повторяю ошибку в решении ошибки — ad hoc фикс для ad hoc проблемы
Документация устаревает — не обновляю после изменений
Не понимаю фичу комплексно — работаю с сиюминутным кейсом
Замедление — техдолг копится, скорость падает
Сопротивляюсь правильным решениям — спорю вместо того чтобы делать
Existing code as permission — нарушения = лицензия на новые
Монолитные команды — нет порционности, нет точек сохранения
Отложенное теряется — deferred items не трекаются
Quality checklist игнорируется — правила есть, но не проверяются
Фичи без инфраструктуры — каждый раз свой обход вместо расширения общего
Покрытие только happy path — работает на тестах, ломается на реальном коде
