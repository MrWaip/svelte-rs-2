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

---

## Сравнение поколений: v1 → v2 → v3

### Объём кода

| Поколение | Crates | ~LOC |
|-----------|--------|------|
| **v1** (ast, parser, analyzer, transformer, compiler) | 5 | ~4,800 |
| **v2** (hir, ast_to_hir, analyze_hir, transform_hir) | 4 | ~2,800 |
| **v3** (svelte_*) | 8 | ~5,800 |

v3 больше по LOC, но делает значительно больше, и при этом проще в каждом отдельном модуле.

### Lifetime management — главный скачок

| | v1 | v2 | v3 |
|-|----|----|-----|
| **AST** | `Node<'a>`, `Expression<'a>` — OXC lifetime пронизывает всё | `Node<'hir>` — отдельный lifetime, но всё ещё параметризован | Owned types, zero lifetimes — `Span` вместо `Expression<'a>` |
| **Передача между crate'ами** | Требует `'a` на каждой границе | `'hir` чуть лучше, но всё равно заражает | Свободная передача — `Component` и `AnalysisData` полностью owned |
| **OXC изоляция** | OXC types везде | OXC types в HIR store (`RefCell<Expression>`) | OXC строго внутри svelte_js и svelte_codegen_client |

Span-based подход — правильное решение для Svelte, где JS выражения нужно re-parse в codegen с другим контекстом.

### Мутабельность AST

| v1 | v2 | v3 |
|----|----|----|
| `RcCell<T>` на каждом узле — interior mutability, `.borrow()` повсюду, паники при двойном borrow | `RefCell<Expression>` в HIR store — лучше, но всё ещё runtime borrow checking | Иммутабельный AST — анализ пишет в отдельные side tables (`AnalysisData`) |

v3 полностью разделил данные (AST) и метаданные (AnalysisData). Устраняет целый класс runtime-ошибок.

### Анализ

| v1 | v2 | v3 |
|----|----|----|
| `SvelteTable` — OXC symbol table + ручной visitor | 5 pass'ов: content_type, dynamic_markers, script, scope, rune_reference | 8 явных pass'ов с чётким порядком и typed side tables |

Ключевые улучшения v3:
- `SymbolId(u32)` вместо строковых ключей
- `FragmentKey` enum — точная адресация любого фрагмента
- `attr_expressions: HashMap<(NodeId, usize), ExpressionInfo>` — атрибуты по (element, index)
- `lowered_fragments` — whitespace trimming как отдельный pass

### Codegen

| v1 | v2 | v3 |
|----|----|----|
| `FragmentContext` с before_init/init/update/after_update | Похожий паттерн на HIR | Content-type driven strategies — паттерн-матчинг по ContentType |
| Повторные обходы дерева | ID-based lookup'ы | O(1) HashMap'ы по NodeId |

v3 codegen чище: стратегия выбирается один раз по ContentType (Empty/StaticText/DynamicText/SingleElement/SingleBlock/Mixed), нет re-traversal дерева.

### Итоговая оценка

| Критерий | v1 | v2 | v3 |
|----------|:--:|:--:|:--:|
| Простота типов | 4/10 | 6/10 | 9/10 |
| OXC изоляция | 2/10 | 5/10 | 9/10 |
| Модульность | 5/10 | 7/10 | 8/10 |
| Корректность reactivity | 5/10 | 7/10 | 6/10 |
| Error recovery | 3/10 | 3/10 | 3/10 |
| Тестируемость | 5/10 | 6/10 | 8/10 |
| Расширяемость | 4/10 | 6/10 | 8/10 |

### Архитектурный долг v3

1. **Scope-aware reactivity.** Pass 6 проверяет по именам — false positive при shadowing each-переменных. v2 имел scope tree через OXC SemanticBuilder; v3 потерял это ради простоты.
2. **`mutated_runes: HashSet<String>`** — трекинг по именам, а не по `SymbolId`. Inconsistency с остальным дизайном.
3. **`validate` pass пуст.** Нет семантической валидации (дупликат атрибутов, невалидные bind-targets).
4. **Нет error recovery.** Парсер падает на первой ошибке — наследие v1. Для LSP нужен partial AST.
5. **Re-parse в codegen.** Span-based = повторный парсинг выражений. Осознанный trade-off (простота > производительность), оправдан.
