# PRD: props destructure alias

Дочерний PRD кластера `reactivity-semantics`, ось — runes (`$props()`).
Закрывает развилку парности на нетривиальных формах ключа в `$props()`-destructure.

## Назначение

Единый ответ компилятора на вопрос «как этот prop-биндинг идентифицируется в `$$props`», валидный для всех форм ключа `BindingProperty` под `$props()`-ObjectPattern: shorthand, ident-ключ, string-литерал с валидным JS-идентификатором, string-литерал-нестандарт, numeric-литерал. Сегодня форма ключа додумывается на стороне трансформа («анализ в кодгене»), что роняет компилятор на numeric-ключе и печатает невалидный JS на string-литерале-нестандарте.

## Формы ключа

- shorthand (`let { foo } = $props()`) — alias совпадает с именем биндинга.
- ident-ключ (`let { foo: x } = $props()`).
- string-литерал с валидным JS-идентификатором (`let { 'foo': x } = $props()`).
- string-литерал-нестандарт (`let { 'a-b': x } = $props()`, `let { 'ysc%%gibberish': x } = $props()`).
- numeric-литерал (`let { 0: x } = $props()`).

Любая форма допустима как в `.svelte`-компоненте, так и в standalone-модулях `.svelte.js` / `.svelte.ts` (там, где допустимо вообще `$props()` — `compile_module` не запрещает).

Computed-ключ (`let { [expr]: x } = $props()`) — отдельная ошибка валидации, остаётся вне scope этого PRD.

## Public API

`ComponentSemantics`:

- `binding_origin_key(SymbolId)` — точка входа резолюции «как этот биндинг идентифицируется в источнике destructure». Возвращает доменно-нейтральный ответ из generic JS-слоя: alias (строка-имя) + `OriginKind`. Источник ответа — walk-up по родителям JS-узла декларации до ближайшего `BindingProperty`; форма ключа диктует `OriginKind`. Контракт расширяется (сегодняшнее `Option<&str>` уходит); все потребители (analyze passthrough, emit-buiders, transform reference-дисптчер, transform-declarator-эмит) мигрируют на новую форму.

`ReactivitySemantics`:

- `ReferenceSemantics::PropRead` обогащается двумя сиблинг-вариантами внутри `PropReferenceSemantics`:
  - `NonSourceStatic` — для печати static-доступа.
  - `NonSourceComputed` — для печати computed-доступа.
  Существующее `NonSource` переименовывается в `NonSourceStatic`; `Source` сохраняется как есть. Identity в payload — только `SymbolId`. Строки и identity-указатели на JS-узлы не несутся.

## Архитектурные инварианты

1. **Identity by id.** Никаких строк в payload вариантов `ReactivitySemantics`. Alias добывается через `binding_origin_key(SymbolId)` на месте.
2. **Generic vs domain.** Walk по `BindingPattern`-родителю и форма ключа — generic JS, живёт в `ComponentSemantics`. Решение Source/NonSource/NonSourceStatic/NonSourceComputed — доменное, живёт в `ReactivitySemantics`.
3. **Один named variant на одно решение.** Static и Computed — два сиблинг-варианта в `PropReferenceSemantics`, не подоси внутри одного. Трансформ-дисптчер — плоский exhaustive-match, без compound-условий.
4. **Smart analyzer / dumb codegen.** Трансформ не walk'ает `BindingPattern.key` ради переоткрытия формы; `static_prop_key_name`-стиль удаляется как «анализ в кодгене».
5. **Не дерево destructure.** Анализ публикует per-leaf факты, ключевые по `SymbolId`. Walk самой destructure-формы остаётся обязанностью трансформа/кодгена через `walk_bindings` — инвариант `analyze.md` §«BindingPattern handling» сохранён.

## Карта решений (scope PRD)

- **Резолюция alias.** Через `binding_origin_key(SymbolId)` в generic JS-слое. Обслуживает все семейства destructure (не только `$props()`).
- **`OriginKind`.** Trit `Ident | String | Numeric`. Источник истины для:
  - выбора вариант `PropReferenceSemantics` в анализе (`Ident` → Static, `String|Numeric` → Computed);
  - формы литерала-аргумента в Source-declarator-эмите (`Numeric` → numeric, иначе string).
- **NonSource read.** Два named-варианта в `PropReferenceSemantics`. Дисптчер печатает по варианту — `$$props.<ident>` или `$$props[<literal>]`.
- **Source read.** Без изменений — thunk-call по локальному имени биндинга.
- **Source declarator emit.** Аргумент-ключ в `$.prop(...)` берётся из `binding_origin_key`. Для `OriginKind::Numeric` печатается numeric-литерал; для `Ident|String` — string-литерал. Парность с Оригиналом по типу литерала-аргумента.
- **Rest-prop excluded.** Список excluded-имён собирается через `binding_origin_key` по всем prop-биндингам, без переоткрытия ключа из AST.

## Client emit

- **Reads — `NonSourceStatic`.** `$$props.<ident>`. Используется для shorthand, ident-ключа и string-литерала с валидным JS-идентификатором.
- **Reads — `NonSourceComputed`.** `$$props[<literal>]`, где литерал — string-литерал alias-а. Используется для string-литерала-нестандарта и numeric-ключа (`$$props["0"]` в форме string-литерала по правилам Оригинала).
- **Reads — `Source`.** Без изменений — `<binding_name>()` через `$.prop`-результат.
- **Declarator — `Source`.** `$.prop($$props, <key>, <flags>, <default>?)`. `<key>` — numeric-литерал для `OriginKind::Numeric`, string-литерал для `Ident|String` (включая ident-ключ, как у Оригинала).
- **Rest excluded.** Все prop-биндинги перечислены через alias-строки.

## Server emit

Заглушка — ожидает `/audit`. Общие правила SSR для рун — в `ssr-pipeline`.

## Контракт с `ReactivitySemantics`

Использует варианты:

- `BindingSemantics::Prop` — без изменений payload-а.
- `DeclaratorSemantics::PropsObject` — без изменений.
- `ReferenceSemantics::PropRead(PropReferenceSemantics::Source)` — без изменений.
- `ReferenceSemantics::PropRead(PropReferenceSemantics::NonSourceStatic)` — новый. Замещает прежний `NonSource`.
- `ReferenceSemantics::PropRead(PropReferenceSemantics::NonSourceComputed)` — новый.
- `ReferenceSemantics::PropMutation`, `PropSourceMemberMutationRoot`, `PropNonSourceMemberMutationRoot` — без изменений payload-а; consume той же резолюции через `binding_origin_key`.

## Контракт с `ComponentSemantics`

`binding_origin_key(SymbolId)` — единственный публичный путь резолюции alias/`OriginKind` для prop- и любых будущих destructure-leaf-биндингов. Сегодняшние потребители (`emit_builders::binding`, `transform::rewrites`, `transform::assignments`, analyze passthrough) мигрируют на новую форму. Не-prop-потребители получают тот же ответ — `OriginKind` доменно-нейтрален.

## Связь

- Родитель: `reactivity-semantics`.
- Sweep-проба: `/workspaces/samples/runtime-runes/samples/props-alias-weird/` (Child + main).
- Кластер тестов: `tasks/compiler_tests/cluster_cases/props_destructure_alias/` (8 проб: numeric/gibberish/identifier/shorthand/string-valid-ident — NonSource; numeric/gibberish — Source; numeric — с rest-tail).
- Реализация в Оригинале:
  - `original/compiler/phases/2-analyze/visitors/VariableDeclarator.js` — присваивание `binding.prop_alias` через `String(key.value)`.
  - `original/compiler/phases/3-transform/client/visitors/Program.js` — регистрация read-transformer'а на `state.transform[name]` для prop-биндинга: ветка `is_prop_source`, ветка `prop_alias` (computed по `b.key(prop_alias).type === 'Literal'`), и дефолтная ветка static-доступа.
  - `original/compiler/phases/3-transform/client/visitors/VariableDeclaration.js` — эмит Source-declarator через `get_prop_source(binding, state, name, initial)`.
  - `original/compiler/phases/3-transform/client/utils.js` — `get_prop_source`, `is_prop_source`, `build_getter`.
  - `original/compiler/utils/builders.js` — `b.key(name)` (regex `regex_is_valid_identifier`).
  - `original/compiler/phases/patterns.js` — `regex_is_valid_identifier`.
