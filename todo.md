# TODO: Reference-код Svelte компилятора для портирования

## Задача

Подготовить reference-структуру из Svelte compiler, чтобы AI мог самостоятельно читать оригинальный код при портировании.

## План

### 1. Создать `reference/svelte-compiler/` с урезанной копией

Нужные файлы из `packages/svelte/src/compiler/`:

```
reference/
  svelte-compiler/
    parse/          # парсер (phases/1-parse)
    analyze/        # анализ (phases/2-analyze)
    transform/      # client codegen (phases/3-transform/client)
    types/          # AST types (types/template.d.ts)
```

~15-20 файлов вместо всего репозитория.

### 2. Создать `PORTING_GUIDE.md` с маппингом

```markdown
## Маппинг Svelte → svelte-rs

| Svelte JS | Наш crate | Статус |
|-----------|-----------|--------|
| phases/1-parse | svelte_parser | ✅ done |
| phases/2-analyze/index.ts | svelte_analyze | ✅ done |
| phases/3-transform/client/visitors/template.js | svelte_codegen_client/template | ✅ partial |
| phases/3-transform/client/visitors/SpreadAttribute.js | — | ❌ TODO |

## Как портировать новую фичу
1. Найти visitor в reference/svelte-compiler/transform/
2. Понять какие analysis-данные он использует
3. Проверить есть ли эти данные в AnalysisData
4. Если нет — добавить pass в svelte_analyze
5. Реализовать в svelte_codegen_client
```

### 3. Решить по git

- Вариант A: добавить `reference/` в `.gitignore` (не засоряет repo)
- Вариант B: оставить в git (документация проекта)

## Почему не submodule / полная копия

- Submodule: ~50k LOC JS засоряет поиск, Grep ловит JS-код
- Полная копия: избыточно, нужно синхронизировать
- Урезанная копия: AI видит reference через Read, ~15-20 файлов достаточно
