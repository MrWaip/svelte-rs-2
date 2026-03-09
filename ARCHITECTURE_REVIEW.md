# Architecture Review Notes

## svelte_compiler
Чистый orchestration-слой: parse → analyze → fatal diag check → generate. Никакой аналитики не перетекло.

## svelte_codegen_client
Граница с analyze соблюдена. Codegen только читает AnalysisData, не вычисляет новую семантику.

- `has_dynamic_children` дублирован в `element.rs:142` и `html.rs:105` — стоит оставить одну копию
- `rune_names: HashSet<String>` в `context.rs:28` — производная от `symbol_by_name` + `runes`, можно было бы добавить в AnalysisData
- `element_needs_var` (`element.rs:151-180`) — codegen-решение (нужна ли DOM-переменная), правильно живёт в codegen

## svelte_analyze

### Архитектурные замечания

1. **`lower` не зависит от пассов 1–4, но стоит 5-м.** Читает только AST, не обращается к expressions/runes/symbols. Мог бы быть первым пассом.

2. **Name-based reactivity — без scope awareness.** `reactivity.rs:63-66` проверяет ссылки по имени через `symbol_by_name`. False positive при shadowing (e.g. `{#each items as count}` где `count` — и rune, и each-контекст). Известное ограничение V3.

3. **Spread offset `+3` дублируется.** `parse_js.rs:143` и `codegen/attributes.rs:165` оба делают `start + 3` для пропуска `...`. Лучше убрать в парсер — пусть `ShorthandOrSpread` хранит span без `...`.

4. **`rune_names` вычисляется дважды.** `mutations.rs:9-13` и `codegen/context.rs:49-53`. Можно добавить в AnalysisData.

5. **Clone ScriptInfo в `symbols.rs:6` и `runes.rs:5`.** Клонируют весь ScriptInfo для обхода borrow checker. Достаточно клонировать только `declarations`.

6. **`content_types.rs:19` — classify для несуществующего alternate.** Вызывается даже без `block.alternate`, записывает Empty. Безвредно, но мусорит.

7. **5 отдельных обходов дерева.** parse_js, mutations, lower, reactivity, content_types — каждый ходит рекурсивно. Для текущего масштаба нормально (SRP > performance).

### Что хорошо
- Каждый pass — один файл, одна функция, одна ответственность
- AnalysisData — flat struct, все owned, zero lifetimes
- Pass order задокументирован в lib.rs
- FragmentKey и SymbolId — typed keys
- content_types работает поверх lowered_fragments, а не raw AST — правильная layering
