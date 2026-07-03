# Handoff: обновление `svelte@5.55.5 → 5.56.4`

Дата: 2026-07-03. Ветка: `polish-v4`. Коммита нет — все изменения в рабочем дереве.

Механика обновления (skill `update-svelte`) выполнена полностью. `just lint` зелёный.
`just test-compiler` и `just test-diagnostics` показывают **новые расхождения parity, внесённые самим svelte 5.56.4** — это отдельная компиляторная работа, ради которой и нужен этот документ.

## Состояние рабочего дерева

Изменено (не закоммичено):

- `package.json`, `package-lock.json` — pin `5.56.4`. Заодно из lock удалены 3 мёртвые `extraneous`‑записи (`packages/svelte-rs2-sweep`, `tasks/benchmark`, `tasks/generate_test_cases` — директории без `package.json`, не входят в workspaces `["packages/svelte-rs2"]`).
- `README.md` — версия в prose.
- `playground/index.html` — import map перегенерирован через JSPM, метка `v5.56.4`. Транзитивные обновления: `@sveltejs/acorn-typescript 1.0.9→1.0.10`, `acorn 8.16.0→8.17.0`, `esrap 2.2.5→2.2.13`.
- `original/compiler/`, `original/package.json` — перевендорены из `node_modules/svelte` (diff чистый — это источник истины для parity).
- `original/docs/` — из тега GitHub `svelte@5.56.4`. Новые файлы: `07-misc/.generated/`, `07-misc/05-browser-support.md`, `03-template-syntax/11-declaration-tags.md`.
- `tasks/compiler_tests/cases2/*/case-svelte.js` — 641 эталонов перегенерированы `just generate` (новый reference output 5.56.4).
- `tasks/diagnostic_tests/cases/template/expression_tag_js_parse_error_*/case-svelte.json` — 3 эталона перегенерированы.

Файлы `case-rust.js` / `case-rust.json` (наш выход) **не менялись** — расхождение между ними и `case-svelte.*` и есть parity‑gap.

## Как воспроизвести и диагностировать

- `just test-compiler` → 696 passed / **303 failed**.
- `just test-diagnostics` → 494 passed / **3 failed**.
- Диагностика одного кейса: `diff tasks/compiler_tests/cases2/<case>/case-rust.js tasks/compiler_tests/cases2/<case>/case-svelte.js` (`case-rust` = наш вывод, `case-svelte` = эталон 5.56.4).
- `WHAT` изменилось искать в `original/compiler/` (уже 5.56.4), `HOW` — по нашей архитектуре.

## Parity‑расхождения к закрытию (по убыванию охвата)

### Codegen — 303 упавших теста (~331 расходящийся кейс)

1. **Именование root‑шаблона внутри блоков: `root_N → root`** — ~302 кейса, доминирующая причина почти всех падений в кластерах `each`, `const_tag`, `await_block`, `snippets`, `each_item_writeback`.
   Внутри блока эталон называет локальный шаблон просто `root` вместо `root_1`/`root_2`.
   Примеры: `animate_basic`, `animate_with_const_tag`. Diff вида `var root_1 = $.from_html(...)` → `var root = ...` и `root_1()` → `root()`.
   **Закрытие этого одного расхождения снимает большинство падений.**

2. **Hoisting исключений `rest_props` в `new Set(...)`** — 22 кейса.
   Было: `$.rest_props($$props, ["$$slots","$$events",…], "rest")`.
   Стало: модульная `var rest_excludes = new Set([…]);` + `$.rest_props($$props, rest_excludes, "rest")`.
   Примеры: `custom_element_dev_exports_legacy_api`, `state_raw_dev_ce_with_props_rest`, кластер `runes_props`.

3. **Скобки вокруг optional‑chain перед доступом к члену/вызовом** — ~10–15 кейсов.
   `$$arg?.().values → ($$arg?.()).values`; `store()?.reset?.apply(…) → (store()?.reset)?.apply(…)`; `obj()?.x?.y → (obj()?.x)?.y`.
   Примеры: `ts_strip_as_paren_optional_chain`, `diagnose_on_directive_optional_chain_handler_wrap`, кластер `snippets`.

4. **`$.tag(...)`‑обёртка mutable sources (dev)** — 8 кейсов.
   `$.mutable_source() → $.tag($.mutable_source(), "count")`.
   Примеры: `diagnose_dev_benchmark`, `diagnose_legacy_dev_benchmark`.

5. **Реструктуризация async** — несколько кейсов.
   `$.await(…)` теперь оборачивается в `$.async(node, [], [], (node) => {…})`; `(await $.save($.async_derived(…)))() → await $.async_derived(…)`.
   Примеры: `async_await_has_await`, `async_derived_nested_function`, `async_derived_nested_function_destructured`.

### Диагностики — 3 упавших теста

`{const y = 2}` / `{let x = 1}` / `{let a, b}` в expression tag в 5.56.4 **больше не** дают `js_parse_error` (референс: `compile(...).warnings === []`), а мы всё ещё эмитим ошибку в позиции `1:1`.
Кейсы: `tasks/diagnostic_tests/cases/template/expression_tag_js_parse_error_{const_declaration,let_declaration,let_multiple_declarators}`.

## С чего начать следующей сессии

Рекомендуемый порядок — от максимального охвата: (1) `root_N → root`, затем (2) `rest_props`/`new Set`, (3) optional‑chain скобки, (4) `$.tag`, (5) async, (6) диагностика declaration в expression tag. Каждое расхождение — отдельная задача; скоупить через skill `dig` от кейса из списка выше.

## Suggested skills

- **`required`** — обязательно в начале любой компиляторной задачи (инварианты PRD затрагиваемого слоя).
- **`dig`** — скоупинг каждого parity‑gap из списка выше перед кодом (передать кейс, напр. `animate_with_const_tag`).
- **`quick-check`** — разовая проверка одного `.svelte`/`.svelte.js` против `svelte/compiler` без регистрации кейса.
- **`add-test`** / **`add-diagnostic-test`** — если для фикса понадобится новый кейс.

## Проверка перед завершением любой задачи

`just test-compiler`, `just test-diagnostics`, `just lint` — все должны быть зелёными.
