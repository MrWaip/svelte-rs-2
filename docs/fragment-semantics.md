# PRD: FragmentSemantics

label: fragment-semantics
topics: fragment, whitespace, whitespace trimming, preserve whitespace, fragment semantics, fragment content, slot content, const tag placement

Дочерний PRD слоя анализа (`analyze.md`). 3.A-подсистема: per-fragment доменные вердикты, потребляемые обоими backend'ами одним запросом. Владелец — **фрагмент** (не элемент): решение живёт на последовательности сиблингов под общим родителем, а последовательность сиблингов и есть фрагмент.

## Владелец и ключ

Ключ вердикта — `FragmentId`. Элемент — лишь **источник** наследуемого контекста (namespace, content-model), но им владеет фрагмент: фрагменты образуют полное множество контейнеров (root, ветки `{#if}`/`{#each}`/`{#await}`, тело сниппета, слот), а элементы — только их часть; ключ по элементу оставил бы блок/слот/root-фрагменты без ключа и вынудил бы потребителя идти вверх по дереву — то есть re-derive в кодгене.

## Вердикт

`FragmentSemanticsStore::query(FragmentId) -> FragmentSemantics` — пучок ортогональных пер-фрагментных осей (форма как у `ExpressionData`). Каждая ось — своя именованная enum; несвязанные факты под один ключ не сливаются.

Оси:

- `whitespace: FragmentWhitespace` — как трактуется whitespace-only текст между сиблингами фрагмента. Взаимоисключающий режим (потому enum, а не bitflags):
  - `Preserve` — verbatim: опция `preserve_whitespace` **или** внутри `pre`/`textarea`/`script` (наследуется вниз). Схлопывает четыре сырых флага клиентского `FragmentCtx` (`preserve_whitespace || inside_pre || inside_textarea || inside_script`) в один вариант.
  - `Collapse` — обычный HTML flow: whitespace-only схлопывается в один пробел и остаётся.
  - `Remove` — svg (кроме `text`) / HTML `select|tr|table|tbody|thead|tfoot|colgroup|datalist`: whitespace-only выкидывается целиком.

- `content: FragmentContent` — чем фрагмент приходится своему владельцу. Взаимоисключающий вопрос, потому enum:
  - `Markup` — фрагмент есть разметка объемлющего рендера: root, тело обычного элемента и `<svelte:element>` без `slot`-атрибута, `<svelte:head>`, `<slot>`, дети `<svelte:self>`.
  - `SelfContained` — фрагмент есть собственное содержимое конструкции: ветки `{#if}`/`{#each}`/`{#await}`/`{#key}`, тело сниппета, дети компонента и `<svelte:component>`, `<svelte:fragment>`, тело `<svelte:boundary>`, тело элемента с `slot`-атрибутом.

  Ось отвечает на доменный вопрос «рендерится ли содержимое фрагмента как самостоятельная единица», а не на вопрос конкретного потребителя. Сейчас потребитель один — валидация размещения **const-тега** (`{@const}` легален только в `SelfContained`-фрагменте); множество потребителей открыто. **declaration-тег** этой оси не спрашивает: у `{const}`/`{let}` ограничения на размещение нет (`declaration-tag.md`).

- `bindings: FragmentBindings` — объявляет ли фрагмент локальный биндинг (`None` / `Local`); источник — наличие **declaration-тега** среди сиблингов.

- `script: FragmentScript` — содержит ли фрагмент (в том числе транзитивно через потомков-элементов) `<script>`.

Узкие предикаты — методы на enum (`is_preserved`, `is_removable`, `is_self_contained`, `declares_local`, `has_script`), не булевы поля.

Имя `SelfContained` намеренно не `Standalone`: `is_standalone` в обоих backend'ах уже занято под другой смысл (одиночный ребёнок фрагмента владеет якорем, маркер `<!---->` не нужен).

## Построение

Пасс `BuildFragmentSemantics` (`fragment_semantics/builder.rs`), `requires: TemplateSideTables` (читает `fragment_namespaces` для whitespace и `attr_index` для `content`), `produces: FragmentSemantics`.

Ось `content` синтезируется отдельно от whitespace-обхода: это чистая функция от владельца фрагмента (`Fragment::owner` + вид узла-владельца, плюс наличие `slot`-атрибута для элементов), а не наследуемый сверху контекст. Поэтому она не течёт через `WsContext` и не зависит от порядка обхода. Имя `slot`-атрибута берётся из лексикона `svelte_ast::SLOT_ATTRIBUTE`, не литералом (`analyze.md` §«Валидация (3.C)», правило spelling). Attribute-grammar walk по шаблонному дереву (зеркалит `collect_fragment_namespaces_in`): наследуемый контекст (`preserve`, `svg_text`, `removable`) течёт сверху вниз, вердикт синтезируется на каждом фрагменте. Element-граница пересчитывает removable по имени/namespace; block-граница наследует (removable «липнет» сквозь блоки, как в клиентском `FragmentCtx`); component/slot-граница пересчитывает removable по namespace самого фрагмента (`fragment_is_svg` из `fragment_namespaces`, который инферит svg и сквозь блоки-потомки — `{#each}`/`{#if}`/…), с shallow-`children_are_svg` как fallback.

## Потребители

Основной потребитель — backend (client|server), один запрос `query(id).whitespace`: он даёт и `preserve` (boundary-window + trim), и `removable` (снятие одиночного пробела), поэтому проброс параметра `preserve_whitespace` через дерево кодгена не нужен.

Backend'ом потребители не исчерпываются: ось `content` сейчас читает только валидация (3.C) внутри самого слоя анализа. Это законно — 3.C читает построенные подсистемы и не пере-выводит их факты (`analyze.md`); «один запрос на одно решение» действует и для внутрислойного потребителя. Пасс `Validate` объявляет `requires: FragmentSemantics`, поэтому порядок держит контракт пассов, а не совпадение стейджей.

## Инварианты

- Codegen-агностичность: `Preserve`/`Collapse`/`Remove` — про DOM-семантику whitespace, одинакова для client и server; подмена рантайма вердикт не меняет.
- Backend матчит вердикт, не пересобирает whitespace сам: обход дерева и `||`-сборка сырых флагов (`preserve_whitespace`/`inside_pre|textarea|script`) в кодгене запрещены (verdict-directed).
- Ось whitespace взаимоисключающа → enum. bitflags — только для **независимых** пер-фрагментных осей.
- `namespace` и `role` в вердикт не кладём — у них свои дома (`fragment_namespaces`, `FragmentRole`).
- Codegen-агностичность `content`: `Markup`/`SelfContained` — про то, чем фрагмент приходится своему владельцу в языке Svelte; подмена рантайма вердикт не меняет. Вариант не называет ни диагностику, ни runtime-вызов.
- `content` — не пере-encoded спек-таблица потребителя. Список допустимых родителей **const-тега** живёт здесь как доменный вердикт; переписывать его литеральным `matches!` по видам родителя в валидаторе запрещено — именно такой список молча разошёлся с Оригиналом (`<svelte:component>` выпал из него).
- `FragmentContent` синтезируется из вида узла-владельца, а не из `FragmentRole`: у `FragmentRole` парсерная гранулярность (`<svelte:fragment>` и обычный элемент делят `Element`), которой для этого вопроса не хватает.
