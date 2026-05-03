---
name: profile
description: Профайлинг компилятора и анализ горячих точек. Только по явному вызову `/profile`.
argument-hint: "[fresh]"
allowed-tools: Bash, Read, Grep, Glob, mcp__narsil-mcp__*
---

# Задача

Провести профайлинг компилятора

# Структура `./profile/`

`./profile/all/` — все кейсы × prod/dev (`just bench-flame-all`):
- `aggregate.top.txt` — агрегированный top по всем кейсам
- `<slug>_<mode>/top.txt` — top одного кейса
- `<slug>_<mode>/profile.folded` — folded stacks (может весить до 6 мб; НЕ читать целиком, только `Grep` по имени функции)
- `<slug>_<mode>/profile.json`, `profile.json.syms.json` — сырые дампы, НЕ читать целиком; работай через `jq`/`grep` точечно

`./profile/one/` — один кейс (`just bench-flame <path> [--dev]`):
- те же артефакты что в `<slug>_<mode>/` выше

`slug` = путь из `tasks/benchmark/benches/compiler/` с `/` → `_`, без `.svelte`/`.svelte.js`.
`mode` = `prod` | `dev`.

## Шаг 1

Если `$ARGUMENTS == "fresh"` — запусти `just bench-flame-all`. Скажи пользователю что прогон занимает ~4 мин и сразу запускай, не спрашивая подтверждения. После прогона — прочитай `./profile/all/aggregate.top.txt`.

Иначе — прочитай существующий `./profile/all/aggregate.top.txt`. Если файла нет — скажи пользователю запустить `/profile fresh` или `just bench-flame-all` вручную и остановись.

## Шаг 2

Проанализируй `./profile/all/`.
Для каждого кандидатного hotspot из `aggregate.top.txt` найди heaviest case — сравни `<slug>_<mode>/top.txt` подкаталогов и выбери где self-% этой функции максимален. Это driver-кейс.

## Шаг 3

Расскажи про горячие точки: что горячее, почему и что с этим делать.

## Дальше в сессии

Когда пользователь выберет опцию — углубись в driver-кейс этого hotspot и предложи конкретные изменения.

Профайлить один кейс отдельно: `just bench-flame <path> [--dev]` → результат в `./profile/one/<slug>_<mode>/`.
