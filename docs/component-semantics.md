# PRD: ComponentSemantics (корневой)

label: component-semantics

Корневой PRD для кластера `svelte_component_semantics` (3.A.1) — generic JS scope-граф.
Дочерний по слою: `analyze.md`.

## Назначение

По образцу `oxc`-модуля `Scoping`, адаптированного под Svelte-компонент (multi-program: `<script module>`, `<script>`, шаблон). Один проход по AST строит JS-уровневый scope/symbol/reference-граф и биндит `OxcNodeId`-ы.

Единый ответ компилятора на JS-уровне: что объявлено, что на что ссылается, какой `OxcNodeId` у узла.

## Что делает

- Обходит AST и собирает: **скоупы** (`ScopeId`), **биндинги** (`SymbolId`), **ссылки** (`ReferenceId`).
- Присваивает `OxcNodeId` каждому релевантному `oxc_ast`-узлу, contiguous через `<script module>` → `<script>` → шаблон (offset-bookkeeping в `ComponentSemanticsBuilder::next_node_id`).
- Биндит `OxcNodeId` в `ExprRef` / `StmtRef` на узлах Svelte AST (резолвит `Cell<OxcNodeId>::DUMMY`-слоты, выставленные парсером).
- Трекает per-binding usage-факты: read, write, mutate (member-мутация, update-выражение).
- Строит `ClassTable` (форк `oxc_semantic .../class/`) — структурный реестр полей класса, public и private. Собирается в том же проходе `JsSemanticVisitor` (отдельного walk нет). Декларации: body-`PropertyDefinition` (`field` и `#field`) и first-assignment в конструкторе (`this.field` / `this.#field`). Две оси:
  - `field_access_target(access_node) -> Option<OxcNodeId>` — узел доступа (`this.field` / `this.#field`) → узел декларации поля. Ключ — `OxcNodeId` (у приватных полей нет `SymbolId`/`ReferenceId`); строковый матч имени изолирован здесь (как в Оригинале). Дедуп по имени: body-поле предпочтительнее ctor-assignment.
  - `class_fields(class_body_node) -> &[ClassFieldDecl]` — все поля (`name`, `is_private`, `decl_node`, `from_constructor`) в порядке объявления; источник истины для генерации аксессоров в трансформе. Семантику поля по `decl_node` даёт `ReactivitySemantics::declarator_semantics`, композицию «доступ → семантика» — `class_field_semantics`.

## Что НЕ делает

- Никакой Svelte-специфичной классификации (руны, store-сигилы, prop-kinds, each-block-kinds). Это `reactivity-semantics` / `block-semantics`.

## Архитектурные инварианты

1. **Generic.** Ничего не знает про `$state`, `$props`, сторы, сниппеты, each-блоки.
2. **Один источник `OxcNodeId`-ов.** Даунстрим-подсистемы индексируют по ним; никто другой не раздаёт свежие.
3. **Read-only над AST** после build.
4. **Identity by id, не по строке.** Резолвинг symbol / binding / reference — через `OxcNodeId` / `ReferenceId` / `SymbolId`. Никаких `find_binding_by_name("foo")` для реальных лукапов. Сравнение имени допустимо только для синтаксических предикатов (детект `$state`-callee, `$$props`).
5. **JS-span'ы относительны содержимому скрипта.** `oxc_parser` парсит `<script>` / `<script module>` как standalone JS — span'ы на символах/биндингах/ссылках zero-based от тела скрипта. Потребители file-relative span'а лениво сдвигают через `Span::shifted_from_oxc(script_offset, oxc_span)` в точке вызова — без глобального rewrite, без скрытого состояния.

## Связь с другими документами

- `context.md` §«Скоупы, биндинги, ссылки».
- `analyze.md` — место в build order (строится первым).
- `reactivity-semantics.md` — главный потребитель scope-графа и `OxcNodeId`-биндингов.
- `ast.md` — `ExprRef`/`StmtRef`, late-bound `OxcNodeId`.
- `bindings-and-references.md` — биндинг vs ссылка, `SymbolId`/`ReferenceId`, имя vs алиас, traversal.
