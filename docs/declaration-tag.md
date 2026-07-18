# Declaration tag

label: declaration-tag
topics: declaration tag, DeclarationTag

**declaration-тег** — шаблонные `{const x = …}` / `{let x = …}` (без `@`). Введён в Svelte 5.56.0. Сквозная фича: scanner → parser → первоклассный `DeclarationTag`-узел → штатная declarator/binding-классификация → кодген. Определение и отличия от **const-тега** — `context.md`.

## Назначение

Объявить локальный лексический биндинг (`const`/`let`), видимый siblings и потомкам фрагмента, тем же способом, что и объявление в скрипте.

## Инварианты

1. **Первоклассный узел; const-тег не переиспользуется.** `{const}`/`{let}` рождает собственный вариант `DeclarationTag` в `Node`; путь `{@const}` (**const-тег**) не трогается и не шарится.
2. **Scanner различает по границе слова в не-сигильной ветке `{`.** `const`/`let` как отдельное слово (word-boundary) в ветке `{` без `@` даёт declaration-тег токен; тело парсится напрямую как statement, без wrapper'а вокруг выражения. Сигильная ветка (`{@…}`) остаётся за @-тегами.
3. **Семантика биндинга — по инициализатору, штатной declarator/binding-машинерией; параллельной классификации нет.** Declaration-тег — обычная `VariableDeclaration`: рун-инициализатор (`{let a = $state(0)}`) выводит биндинг в рун-вариант (`State`/`Derived`), как скриптовое объявление; plain single-identifier — в `DeclarationTag`. Демотация plain-биндинга в `OptimizedDeclarationTag` — `reactivity-semantics.md` инвариант 9.
4. **Кодген эмитит плоскую декларацию verbatim — без `$.derived`-мемо.** `{const x = e}` → `const x = e;`, `{let x = e}` → `let x = e;` в тело фрагмента; реактивность чтения решается на сайте чтения, как у любой лексической переменной. Контраст: **const-тег** оборачивает значение в `$.derived`. Рун-инициализатор к моменту эмита уже переписан трансформом (инвариант 5), поэтому кодген не спецкейсит его — печатает результат тем же verbatim-путём. Async-инициализатор — исключение (инвариант 6).
5. **Рун-инициализатор тега переписывает трансформ тем же путём, что и скриптовые руны.** Declaration-тег с рун-инициализатором (`{let c = $state(0)}` → `let c = $.state(0)`) прогоняется через тот же `rewrite_binding_declarations`, что и скриптовое объявление; wrapping/демотация — по `state-rune.md`, чтения и записи тега (`{c}`, `c++`) — штатными диспетчерами по `ReferenceSemantics` из `reactivity-semantics.md`. Единственное следствие для этого PRD: клиентский трансформ трогает template-owned statement на top-level фрагмента, а не в скриптовом Program, — намеренное задокументированное отступление от разделения template/script; SSR-трансформ разворачивает сигнал в plain-значение симметрично.
6. **Async-инициализатор — run/blocker-форма по вердикту, не verbatim.** Declaration-тег с `await` в инициализаторе (`{const x = await f()}`, experimental async) эмитится обоими backend'ами не плоской декларацией (инвариант 4), а async-формой: биндинг объявляется пустым (`let x;`), присваивание уходит в `$.run([async () => x = …])` (SSR — `$$renderer.run`), потребители-чтения получают promise-слот блокером. Форму выбирает backend по доменному вердикту `BlockSemantics::DeclarationTag.async_kind` (`FragmentDeclarationAsyncKind` — `Sync`/`Awaited`/`Deferred`, общий с const-тегом), рождённому анализом из `ExpressionData`; кодген матчит один вердикт, не пересобирая `await`-факт (verdict-directed). Отличие от const-тега — присваивание плоское (`x = init`), без `$.derived`-обёртки.
