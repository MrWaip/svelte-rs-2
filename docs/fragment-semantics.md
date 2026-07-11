# PRD: FragmentSemantics

label: fragment-semantics
topics: fragment, whitespace, whitespace trimming, preserve whitespace, fragment semantics

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

Узкие предикаты — методы на enum (`is_preserved`, `is_removable`), не булевы поля.

## Построение

Пасс `BuildFragmentSemantics` (`fragment_semantics/builder.rs`), `requires: TemplateSideTables` (читает `fragment_namespaces`), `produces: FragmentSemantics`. Attribute-grammar walk по шаблонному дереву (зеркалит `collect_fragment_namespaces_in`): наследуемый контекст (`preserve`, `svg_text`, `removable`) течёт сверху вниз, вердикт синтезируется на каждом фрагменте. Element-граница пересчитывает removable по имени/namespace; block-граница наследует (removable «липнет» сквозь блоки, как в клиентском `FragmentCtx`); component/slot-граница сбрасывает по `children_are_svg`.

## Потребители

Потребитель вердикта — backend (client|server), один запрос `query(id).whitespace`: он даёт и `preserve` (boundary-window + trim), и `removable` (снятие одиночного пробела), поэтому проброс параметра `preserve_whitespace` через дерево кодгена не нужен.

## Инварианты

- Codegen-агностичность: `Preserve`/`Collapse`/`Remove` — про DOM-семантику whitespace, одинакова для client и server; подмена рантайма вердикт не меняет.
- Backend матчит вердикт, не пересобирает whitespace сам: обход дерева и `||`-сборка сырых флагов (`preserve_whitespace`/`inside_pre|textarea|script`) в кодгене запрещены (verdict-directed).
- Ось whitespace взаимоисключающа → enum. bitflags — только для **независимых** пер-фрагментных осей.
- `namespace` и `role` в вердикт не кладём — у них свои дома (`fragment_namespaces`, `FragmentRole`).
