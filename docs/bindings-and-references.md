# Биндинги, ссылки и BindingPattern (OXC)

topics: binding, reference, SymbolId/ReferenceId, BindingPattern, walk_bindings, identifier resolution, name vs alias, OXC binding API

Карта системы идентификаторов проекта: как читать биндинги/ссылки и обходить
`BindingPattern`. `ComponentSemantics` (раздел 1) — пасс, который всё это порождает.

Канон терминов — `context.md`. Реактивная семантика — `reactivity-semantics.md`.
Формы типов — из OXC `0.117.0`.

## 1. ComponentSemantics — корневой пасс

`ComponentSemantics` (крэйт `svelte_component_semantics`, `lib.rs`) — форк
`oxc_semantic` под Svelte. Первый analyze-пасс; от него работают все подсистемы
(реактивность, атрибуты, блоки, transform, codegen). Строится
`ComponentSemanticsBuilder`, после build — read-only. Держит свои таблицы
(symbol/reference/scope), id-типы реэкспортит из `oxc_syntax`.

Задача пасса — пройти **всё** комбинированное дерево (OXC JS + Svelte template) и:
- завести scope-дерево, единое для скрипта и шаблона (template-скоупы — потомки
  `instance_scope_id`, тот — `module_scope_id`);
- завести биндинг (`SymbolId`) на каждый `BindingIdentifier`;
- связать ссылки с биндингами — резолв `IdentifierReference`/`ReferenceId` → `SymbolId`;
- учесть ссылки в **шаблоне** компонента, не только в JS (`is_template_reference`,
  `create_template_reference`);
- учесть **неразрешённые** ссылки — глобалы, неизвестные импорты
  (`finalize_unresolved_references`, `root_unresolved_references`).

Ключевые точки доступа (`storage.rs`):
- резолв: `symbol_for_reference(ref_id) -> Option<SymbolId>`,
  `get_resolved_reference_ids(sym) -> &[ReferenceId]`;
- скоупы: `find_binding(scope, name)`, `instance_scope_id`, `module_scope_id`,
  `scope_parent_id`;
- природа ссылки: `is_instance_reference` / `is_module_reference` / `is_template_reference`.

**Имя vs алиас.** Две разные строки на одном биндинге:
- `symbol_name(sym)` — **локальное** имя биндинга. Для `const { a: b } = obj` это `b`.
- `binding_origin_key(sym) -> Option<(Cow<str>, OriginKind)>` — **origin**-ключ, то имя,
  под которым значение приходит снаружи. Для `const { a: b } = obj` это `a`
  (поднимается от декларации до `BindingProperty`, отдаёт ключ как `Ident`/`String`/
  `Numeric`). Для не-алиасного `const x` возвращает само имя; для object-rest и
  computed-ключа — `None`. По ссылке: `binding_origin_key_for_reference(ref_id)`,
  `binding_origin_key_for_identifier_reference(&id)`.

## 2. Биндинг vs Ссылка

Две разные сущности на двух разных сторонах:

- **Биндинг** *(declaration-site)* — место, где имя **объявляется**. В OXC это
  `BindingIdentifier` — лист дерева `BindingPattern`. Несёт `symbol_id`.
- **Ссылка** *(use-site)* — место, где имя **используется** (читается/пишется). В OXC
  это `IdentifierReference`. Несёт `reference_id`.

Ловушка первая: типа `ReferenceIdentifier` **не существует**. Канон OXC —
`IdentifierReference`. Всплыло «ReferenceIdentifier» — модель ещё не уложилась.

Ловушка вторая: оба типа сериализуются в ESTree как `"Identifier"` и оба имеют поле
`name`. По ESTree-дампу (skill `dump-ast`) их **не различить по тегу** — различает
только id-поле:

  IdentifierReference { name, reference_id: Cell<Option<ReferenceId>> }   -> use-site
  BindingIdentifier   { name, symbol_id:    Cell<Option<SymbolId>>    }   -> declaration

Канон терминов (синхронно с глоссарием `context.md`):

| Домен        | OXC-тип                          | Идентичность                | Роль             |
|--------------|----------------------------------|-----------------------------|------------------|
| Биндинг      | `BindingIdentifier` (лист)       | `symbol_id` → `SymbolId`     | declaration-site |
| Ссылка       | `IdentifierReference`            | `reference_id` → `ReferenceId` | use-site      |
| (LHS целиком)| `BindingPattern` (дерево)        | —                           | контейнер биндингов |

Цепочка резолва (см. `context.md` → Relationships): ссылка → биндинг → декларатор →
декларация. Анализ резолвит `ReferenceId` в `SymbolId`; имя — атрибут таблицы, не
часть id.

## 3. SymbolId vs ReferenceId

Две оси идентичности, обе реэкспортятся из `oxc_syntax` через
`svelte_component_semantics`:

- **`SymbolId`** — одна объявленная сущность (один биндинг). Один на декларацию имени.
  Все use-sites этого имени резолвятся в один и тот же `SymbolId`.
- **`ReferenceId`** — одно конкретное вхождение использования. Их много на один
  `SymbolId`.

Отношение: `SymbolId` 1 — N `ReferenceId`. `symbol_for_reference(ref_id) -> Option<SymbolId>`
идёт N→1; `get_resolved_reference_ids(sym) -> &[ReferenceId]` идёт 1→N. `Option` у
первого — потому что ссылка может быть неразрешённой (глобал/неизвестный импорт).

**Identity by id.** Ключи семантики — `SymbolId` / `ReferenceId` / `OxcNodeId`.
Имён в payload нет, `find_binding_by_name` нет (инвариант
`reactivity-semantics.md` #1). Имя достаётся через `ComponentSemantics::symbol_name(sym)` —
это и есть «Resolve identifier name» из глоссария, единственный валидный способ.
Никаких параллельных стеков строк в visitor-state.

## 4. BindingPattern — это дерево

Главный сдвиг: `let x = …`, `let { a, b } = …`, `let [{ name }, ...rest] = …` — это
всё `BindingPattern`. Плоское имя `x` — лишь вырожденный случай дерева из одного узла.
Любой код, который трогает LHS декларатора/параметра, обязан покрыть **всю**
вариативность дерева.

`BindingPattern` — enum, ровно 4 варианта в 0.117:

  BindingIdentifier(Box<BindingIdentifier>)   лист, несёт symbol_id
  ObjectPattern(Box<ObjectPattern>)           { ... }
  ArrayPattern(Box<ArrayPattern>)             [ ... ]
  AssignmentPattern(Box<AssignmentPattern>)   pat = default

Вложенные формы, каждую из которых легко пропустить:

- **`ObjectPattern`** — `properties: Vec<BindingProperty>` + `rest: Option<BindingRestElement>`.
  - `BindingProperty { key, value: BindingPattern, shorthand, computed }`.
    - `value` — рекурсивно `BindingPattern` (вложенность произвольной глубины).
    - `shorthand` — `{ a }` против `{ a: b }` (алиас); в обоих биндинг сидит в `value`.
    - `computed` — `{ [k]: v }`, ключ — выражение, не статическое имя.
  - `rest` — `{ a, ...rest }`; в `rest.argument` — снова `BindingPattern`.
- **`ArrayPattern`** — `elements: Vec<Option<BindingPattern>>` + `rest`.
  - `Option` у элемента — **дырки**: `[a, , c]` даёт `[Some, None, Some]`. `None`
    пропускаем, индекс — не пропускаем.
  - `rest` — `[a, ...rest]`.
- **`AssignmentPattern { left: BindingPattern, right: Expression }`** — дефолт.
  `right` — значение по умолчанию; биндинги — рекурсивно в `left`. Дефолт может стоять
  и на промежуточном узле (`{ a: { b } = {} }`), и на листе (`{ a = 5 }`).
- **`BindingRestElement { argument: BindingPattern }`** — `...rest` для Object и Array.

Чеклист полноты обхода (если ловишь руками — проверь все): лист · object-проп ·
алиас (`shorthand=false`) · computed-ключ · вложенный object/array · array-дырка ·
object-rest · array-rest · дефолт на листе · дефолт на промежуточном узле.

## 5. Обход: walk_bindings

Чтобы не катать `match` по дереву руками (и не забыть половину чеклиста выше), есть
канонический хелпер `svelte_component_semantics::walk_bindings`. Он используется в
нескольких десятках мест по analyze/transform/codegen — это де-факто единственный санкционированный
способ обойти `BindingPattern`.

  walk_bindings(pat, |v: BindingVisit| {
      v.symbol     SymbolId этого биндинга-листа
      v.path       &[Step] — путь от корня: Access::Key{key, computed} | Access::Index{index, len, has_rest} | Access::Slice{from};
                   Step.default: Option<&Expression> — дефолт на этом шаге
      v.is_rest    это ...rest-биндинг
      v.excluded   для object-rest: ключи, исключённые из rest (&[&PropertyKey])
  });

`path` даёт точное «как добраться до значения этого биндинга» — например
`let { users: [{ name }, second] }` даёт `name` с путём `.users[0].name`, а `second`
с путём `.users[1]`. Дефолты и rest/excluded — там же, отдельные поля собирать не надо.

Дисциплина: **не катай `match BindingPattern` руками** ради обхода. Ручной `match`
санкционирован только когда задача — не «обойти все биндинги», а распознать
конкретную форму (например, отличить чистый `BindingIdentifier` от деструктуринга
на верхнем уровне).

## 5.1 Обход write-sites: walk_assignment_targets

LHS присваивания (`x = …`, `({ a } = …)`, `[a, ...b] = …`) — это **не** `BindingPattern`,
а `AssignmentTarget` (другой OXC-тип). Декларация заводит биндинги; присваивание —
мутирует уже существующие. Не путать оси: лист write-дерева — это **use-site**
(`IdentifierReference` → `ReferenceId`, либо member-таргет `obj.x` / `obj[k]`), а не
объявленный `BindingIdentifier`/`SymbolId`. Семантику листа берут через
`reference_semantics(ReferenceId)`, не `binding_semantics`/`declarator_semantics`.

Канонический обходчик write-дерева — `walk_assignment_targets`, сестринский к
`walk_bindings` по схеме (рекурсия → плоские листья + путь). На каждый лист отдаёт
`AssignmentTargetVisit`:

  walk_assignment_targets(target, |v: AssignmentTargetVisit| {
      v.target    WriteTarget::Identifier(&IdentifierReference) | WriteTarget::Member(&AssignmentTarget)
      v.path      &[WriteStep] — путь от корня:
                    WriteAccess::Index { index, len, has_rest }  элемент массива (len/has_rest — для $.to_array)
                    WriteAccess::Slice { from }                  array-rest: arr.slice(from)
                    WriteAccess::Key { name }                    obj.name (shorthand или статический ключ)
                    WriteAccess::Computed { key }                obj[expr]
                  WriteStep.default: Option<&Expression> — дефолт на этом шаге
      v.excluded  для object-rest: статические ключи, исключённые из rest
  });

Отличия payload от `BindingVisit` осевые: `target` (use-site) вместо `symbol`;
`WriteAccess::Index` несёт `len`/`has_rest` (лоуэрингу нужна арность `$.to_array`);
array-rest — это шаг `Slice` (а не флаг), чтобы вложенный rest-паттерн (`[x, ...{ z }]`)
получил префикс среза в пути. TS-варианты `AssignmentTarget` тут не встречаются (TS
стрипается на парсере) — match исчерпывающий, TS-ветки `unreachable!`.

Узкий хелпер `walk_assignment_target_idents` — gate «простая ли форма» (сдаётся на
rest/default/computed/member), отдаёт только identifier-листья; используется legacy
`$:`-трекингом зависимостей. Для обхода ради разворота — только `walk_assignment_targets`.

Дисциплина та же, что для `walk_bindings`: **не катай `match AssignmentTarget` руками**
ради обхода.

## 6. Плоская семантика (ключевой инвариант)

Мы **не зеркалим** древовидную структуру `BindingPattern` в семантиках. Нет
`BindingPatternSemantics`-дерева. Семантика хранится **плоско**, ключом по id:

- `binding_semantics(sym: SymbolId) -> BindingSemantics`
- `reference_semantics(ref_id: ReferenceId) -> ReferenceSemantics`
- `declarator_semantics(node: OxcNodeId) -> DeclaratorSemantics`

(точки расширения `ReactivitySemantics`, см. `reactivity-semantics.md`).

Контракт потребителя (transform/codegen): **сам обходишь дерево** через `walk_bindings`
→ на каждый лист берёшь `v.symbol` → дёргаешь `binding_semantics(v.symbol)`. Дерево
живёт только в OXC-AST; семантика отвечает на вопрос про один биндинг за один `match`,
независимо от того, насколько глубоко он сидел в деструктуринге.

Для write-оси симметрично: обходишь `AssignmentTarget` через `walk_assignment_targets`
→ на каждый `v.target` (use-site) берёшь `reference_semantics(ReferenceId)`. Дерево —
только в OXC-AST; семантика записи — плоская по `ReferenceId`.

Почему так, а не зеркало дерева:
- **Один источник истины по форме.** Форма деструктуринга уже есть в OXC-AST;
  дублировать её в семантике — риск расхождения.
- **Totality по id.** `binding_semantics`/`reference_semantics` всегда возвращают
  вариант (не `Option`), ключ — стабильный id, а не позиция в дереве.
- **Композиция запрещена.** Если ответ собирается из 2+ полей — это разрыв контракта
  семантики, а не повод тащить структуру дерева в потребителя.

## 7. Конструирование (кратко)

Когда transform/codegen **создаёт** новые узлы — через `svelte_ast_builder`:

- `b.bid(name)` / `b.bid_at(name, span)` → `BindingIdentifier`
- `b.rid(name)` / `b.rid_at(name, span)` → `IdentifierReference`

Грабли seeding: и `symbol_id` у `BindingIdentifier`, и `reference_id` у
`IdentifierReference` — это `Cell<Option<…>>`, **пустые после конструирования** (как и
после парсинга — OXC выставляет их в bind-step семантики). Свежесозданный узел в
id-таблицах анализа не существует. Если новому узлу нужна семантика/резолв — id
придётся завести и засидить явно; нельзя рассчитывать, что
`symbol_id.get()`/`reference_id.get()` вернёт что-то осмысленное само.

Перед конструированием новой JS-формы — skill `dump-ast`: он показывает, как OXC
представляет конкретную конструкцию (деструктуринг, spread, optional chaining и т.п.),
чтобы не угадывать форму узла.

## 8. Частые грабли

- «ReferenceIdentifier» — нет такого; `IdentifierReference`.
- По ESTree-дампу `BindingIdentifier` и `IdentifierReference` оба — `"Identifier"`;
  различай по `symbol_id` vs `reference_id`.
- `let x` — это тоже `BindingPattern` (вырожденное дерево), не «просто имя».
- Array-дырки: `elements` — `Vec<Option<…>>`, `None` пропускать, индекс держать.
- Object-rest требует `excluded`-ключей — `walk_bindings` отдаёт их готовыми.
- LHS присваивания — `AssignmentTarget`, не `BindingPattern`; биндинги там не
  объявляются.
- Не ищи дерево в семантике — его там нет; обходи AST, спрашивай по `SymbolId`.
- Имя биндинга — только `symbol_name(sym)`; никаких имён в payload семантики и
  никаких параллельных стеков строк.
