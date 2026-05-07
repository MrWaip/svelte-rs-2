# Analyzer Target Design

PRD: целевая форма `svelte_analyze` после рефакторинга. Закрепляет архитектурные решения, миграционный порядок и контракт между анализатором и потребителями (codegen / transform / checker).

## Problem Statement

`svelte_analyze` сейчас находится в промежуточном состоянии. Параллельно живут три формы данных:

1. Новые семантические системы — `ComponentSemantics` (готов), `ReactivitySemantics` (готов), `BlockSemantics` (частично).
2. Старая инфраструктура — 14 passes (`bind_semantics`, `dynamism`, `template_side_tables`, `element_flags`, `fragment_topology`, `content_types`, `bundles`, `js_analyze/expression_info`, `post_resolve`, `enrich_script_info`, `finalize_component_name`, `template_validation`, `collect_symbols`, `executor`) с DAG-резолвером в `passes/mod.rs`.
3. 18 промежуточных side-tables в `AnalysisData` (`ExpressionInfo`, `BindSemanticsData`, `TemplateSemanticsData`, `ElementFlags`, `ElementFacts`, `RichContentFacts`, `FragmentFacts`, `DirectiveModifierFlags`, `SnippetData`, `DebugTagData`, `TitleElementData`, `DynamismData`, `BlockerData`, `ScriptInfo` подмножества, `ProxyStateInits`, `PickledAwaitOffsets`, `OutputPlanData`, `expression_tags_by_fragment`).

Поверх этого — `AnalysisData` god-struct с ~70 ad-hoc helper-методов (`is_dynamic`, `class_needs_state`, `attr_is_function`, `expr_deps`, `parent_each_blocks`, `bind_target_symbol`, `each_is_destructured`, `expression_blockers`, …), которые читают факты из 2–4 разных side-table одновременно.

В результате потребитель (главным образом `svelte_codegen_client`) при попытке принять одно решение пишет цепочки вида:

```rust
if self.ctx.query.runes() { return expr; }
if !info.needs_legacy_coarse_wrap()
    && !info.uses_legacy_sanitized_props() { return expr; }
```

Эти `&&`-цепочки повторяются по всему codegen в десятках мест, потому что ни одна из текущих сущностей не отвечает за «один исчерпывающий ответ для одной единицы шаблона». Каждый новый случай заставляет либо добавлять ещё один helper в `AnalysisData`, либо новый side-table, либо новую `&&`-цепочку.

Результирующая боль:

- невозможно ответить codegen-у/transform-у одной query — единица распределена по нескольким сторам;
- `AnalysisData` растёт каждым новым case как god-объект, прочитать его нельзя — слишком много полей и методов с пересекающимися именами;
- любое изменение поведения требует синхронной правки в analyzer pass, side-table, helper-методе и потребителе — четыре места;
- порядок passes контролируется DAG-резолвером, который сам стал источником сложности (18 PassDescriptor + 5 hand-coded стейджей + `debug_assert_eq!` что обе формы совпадают);
- валидация (`validate/*` + `template_validation`) живёт внутри analyze и имеет доступ к `pub(crate)` internals — соблазн добавить «специальную» проверку, читающую внутренние passes напрямую, не сдерживается границей крейта.

## Solution

Реорганизовать `svelte_analyze` так, чтобы каждой доменной единице языка Svelte соответствовал ровно один кластер, отвечающий на любой вопрос потребителя за один query, исчерпывающе.

Принципы целевой формы:

1. **Кластер per Svelte kind.** Пять кластеров — Block, Attribute, Element, Expression, Reactivity. Идентичность каждого — `NodeId` (для template-узлов) или `SymbolId` / `ReferenceId` / `OxcNodeId` (для Reactivity).
2. **Thick payload.** Кластер пред-вычисляет каждое решение, которое потребитель иначе собрал бы `&&`-цепочкой из ≥2 сторов. Готовое решение лежит в payload как `enum` / `bitflags` / поле — потребитель читает один варинт. Исключения из правила Thick — точечные, согласуются явно при проектировании конкретного кластера.
3. **Один query на единицу.** Потребитель, держащий в руках `Block`/`Attribute`/`Element`/`Expression`-узел, делает один лукап в соответствующий стор и матчится один раз. Дальнейшего assembly из других сторов не требуется.
4. **Pipeline без DAG.** Шесть фаз с фиксированным порядком, выраженным через явные вызовы builder-функций. Зависимости между фазами обеспечиваются Rust типизацией (input — выход предыдущей фазы).
5. **Тонкая, законченная `AnalysisData`.** Несколько top-level полей в чётких категориях: scope-инфраструктура, кластеры, generic индексы, component-global сводки. Никаких ad-hoc helper-методов помимо доступа к этим полям.
6. **Standalone checker.** Валидация переезжает в отдельный crate `svelte_check`, читает готовую `AnalysisData` через публичный API. Cluster builders диагностики не эмитят — нашли невалидное состояние, возвращают `Unresolved`-вариант или ассертят (баг анализатора).
7. **Smart analyzer / dumb codegen.** Любое решение «как лежать в JS» собирается analyzer-ом. Codegen матчится по готовому варианту и эмитит — никакой композиции фактов.

Финальная форма `AnalysisData`:

```text
AnalysisData
├── scoping              ComponentSemantics
├── reactivity           ReactivitySemantics
├── blocks               BlockSemanticsStore
├── attributes           AttributeSemanticsStore
├── elements             ElementSemanticsStore
├── expressions          ExpressionSemanticsStore
├── topology             TemplateTopology
├── element_index        TemplateElementIndex
├── script_declarations  ScriptDeclarations
├── blockers             BlockerData
├── custom_element       CustomElementInfo
├── runtime              RuntimePlan
├── css                  CssAnalysis
└── component_name       String
```

Pipeline:

```text
Phase 1  scope & scripts
  ComponentSemanticsBuilder
  resolve_component_name (util)
  ScriptDeclarationsBuilder
  BlockerDataBuilder
  CustomElementInfoBuilder

Phase 2  reactivity
  ReactivitySemanticsBuilder         (включает rune-call map для transform)

Phase 3  generic indices
  GenericIndicesBuilder              (TemplateTopology + TemplateElementIndex)

Phase 4  clusters
  BlockSemanticsBuilder              (доделать DebugTag / HtmlTag варианты)
  ExpressionSemanticsBuilder         (новый, поглощает PickledAwaitOffsets)
  AttributeSemanticsBuilder          (новый)
  ElementSemanticsBuilder            (новый)

Phase 5  global rollup
  RuntimePlanBuilder                 (сводка component-level booleans)
  CssAnalysisBuilder                 (CSS pipeline целиком)

Phase 6  checker (crate svelte_check)
  ComponentChecker                   (читает готовую AnalysisData + AST)
```

Контракт потребителя после рефакторинга:

- codegen, держащий expression-узел: `data.expressions.get(id)` — один enum со всеми решениями;
- codegen, держащий attribute-узел: `data.attributes.get(id)` — один enum;
- codegen, держащий element-узел: `data.elements.get(id)` — один enum;
- codegen, держащий block-узел: `data.blocks.get(id)` — один enum;
- transform по reference: `data.reactivity.reference_semantics(ref_id)` — готовое правило rewrite-а;
- checker: читает все кластеры + AST через публичный API `svelte_analyze`.

Ноль `&&`-цепочек поверх 2+ сторов на одно решение. Ноль ad-hoc helper-методов на `AnalysisData`. Ноль читаемых из codegen «промежуточных» side-tables.

## User Stories

1. Как разработчик codegen, я хочу читать одно поле кластерного payload вместо комбинации `runes()` + `info.needs_legacy_coarse_wrap()` + `info.uses_legacy_sanitized_props()`, чтобы решение «оборачивать ли expression в legacy coarse wrap» принималось одним матчем по `LegacyWrap` enum.
2. Как разработчик codegen, я хочу читать `data.attributes.get(attr_id)` и матчиться один раз вместо прыжков `bind_target_semantics(id)` + `attr_expression(id)` + `event_modifiers(id)`, чтобы любая работа с атрибутом начиналась с одного варианта enum.
3. Как разработчик codegen, я хочу читать `data.elements.get(el_id)` вместо AST-node-kind дисптача `match node` для всех форм element-узлов, чтобы один semantic-дисптач закрывал HTML, `<svelte:element>`, `ComponentNode`, `<svelte:boundary>`, `<svelte:head>`, и остальные специальные таргеты.
4. Как разработчик codegen, я хочу читать `data.blocks.get(block_id)` для всех `{#...}` и `{@...}` тегов, включая `{@debug}` и `{@html}`, чтобы block-обработка была единообразной.
5. Как разработчик codegen, я хочу, чтобы async-факты для блока (has_await, blockers) были полем кластерного payload, а не отдельным `AsyncEmissionPlan`, чтобы async-ветка решалась одним полем варианта.
6. Как разработчик codegen, я хочу, чтобы `is_dynamic_attr(id)` / `is_dynamic_node(id)` стали полем `dynamism: bool` в payload соответствующего кластера, чтобы dynamism читался по той же ручке что и остальные факты единицы.
7. Как разработчик codegen, я хочу, чтобы `class_needs_state(id)`, `has_dynamic_class_directives(id)`, `has_runtime_attrs(id)` были полями `ElementSemantics::Html`, чтобы пред-вычисленная сводка читалась без повторной композиции из `ElementFlags` + `DynamismData` + `AttrIndex`.
8. Как разработчик codegen, я хочу, чтобы fragment-level факты (`has_children`, `non_trivial_child_count`, `has_expression_child`, `has_animate_child`, `has_rich_content_by_id`) лежали в payload родителя fragment-а как `children_summary`, чтобы codegen, обрабатывающий родителя, читал готовую сводку по детям.
9. Как разработчик codegen, я хочу, чтобы `parent_each_blocks(id)` исчезла как helper, а each-context-vars лежали явным полем в `AttributeSemantics::HtmlBind { each_context_vars }` и других payload-ах, использующих этот факт.
10. Как разработчик codegen, я хочу, чтобы все факты для одного expression-узла (legacy_wrap, async_kind, memoization, role, references, is_pickled_await) лежали в одном `ExpressionSemantics`, чтобы любое expression-решение было одним матчем без чтения `ExpressionInfo`.
11. Как разработчик codegen, я хочу, чтобы codegen больше не читал `passes/dynamism.rs`, `passes/element_flags.rs`, `ExpressionInfo` или другие internal-структуры analyze, чтобы граница интерфейса между крейтами была явной.
12. Как разработчик transform, я хочу, чтобы `ScriptRuneCalls` стал side-output `ReactivitySemantics`, чтобы один pass классифицировал rune-call-узлы для rewrite-ов, а не два независимых.
13. Как разработчик transform, я хочу читать `data.expressions.get(node_id).is_pickled_await` вместо `is_pickled_await(span.start)`, чтобы лукап шёл по NodeId как и остальные expression-факты.
14. Как разработчик transform, я хочу продолжить работать per-reference через `data.reactivity.reference_semantics(ref_id)` без необходимости знать про template-кластеры, чтобы трансформер оставался узким на rewrite-ы JS identifier-ов.
15. Как разработчик checker, я хочу читать готовую `AnalysisData` через публичный API без доступа к `pub(crate)` internals analyze, чтобы валидационные правила не зависели от internal представления.
16. Как разработчик checker, я хочу, чтобы свои правила (runes vs legacy mismatches, store usage, non-reactive update, experimental_async, a11y, template_validity) жили в одном крейте `svelte_check` с одним template walk и одним script walk, чтобы добавление нового правила не требовало правки analyze pipeline.
17. Как разработчик checker, я хочу, чтобы `IgnoreData` (`<!-- svelte-ignore -->` сканер) и `warning_filter` жили внутри `svelte_check`, чтобы analyze не отвечал за пользовательскую suppression-семантику.
18. Как разработчик analyze, я хочу, чтобы добавление нового factа для нового case требовало правки ровно одного builder-а кластера (например, добавил поле в `AttributeSemantics::Event`), а не коррекции в pass + side-table + helper + потребитель.
19. Как разработчик analyze, я хочу удалить DAG-резолвер `passes/mod.rs`, чтобы порядок выполнения был очевиден из чтения `analyze()` сверху вниз, без необходимости держать в голове `requires/produces` токены.
20. Как разработчик analyze, я хочу, чтобы каждый builder был чистой функцией `(input) -> output` с входом из конкретных полей `AnalysisData`, чтобы зависимости между фазами проверялись Rust компилятором, а не runtime-валидатором.
21. Как разработчик analyze, я хочу, чтобы каждый builder делал не больше чем один walk instance script + один walk module script + один walk template, чтобы overall analyze-время оставалось контролируемым.
22. Как разработчик analyze, я хочу, чтобы 14 текущих passes (`bind_semantics`, `dynamism`, `template_side_tables`, `element_flags`, `fragment_topology`, `content_types`, `bundles`, `js_analyze/expression_info`, `post_resolve`, `enrich_script_info`, `finalize_component_name`, `template_validation`, `collect_symbols`, `executor`) исчезли, чтобы навигация по analyze шла через 12 builder-ов вместо 14 passes + 18 side-tables.
23. Как разработчик analyze, я хочу, чтобы 18 side-tables либо растворились в payload своего кластера, либо стали top-level полем `AnalysisData` с явной семантикой, чтобы вопрос «где живёт этот факт» имел один ответ.
24. Как разработчик analyze, я хочу, чтобы `ProxyStateInits`, `has_class_state_fields`, `has_store_member_mutations` были удалены как самостоятельные сущности, потому что у них нет внешних потребителей.
25. Как разработчик analyze, я хочу, чтобы builder-ы оставались `Send` и не имели shared mutable state помимо своего выхода, чтобы оставалась возможность позже добавить точечный rayon-параллелизм без переписывания.
26. Как будущий мейнтейнер, я хочу читать `analyze()` как линейный список из 6 фаз без условной логики выбора порядка passes, чтобы понимать pipeline за один проход.
27. Как будущий мейнтейнер, я хочу видеть в `debt.md` явные ссылки на оставшийся технический долг (например, parent-link в svelte_ast убирает `TemplateTopology`), чтобы знать, какие части ещё могут быть упрощены.
28. Как ревьюер, я хочу видеть, что новый case «legacy coarse wrap» добавляется одним полем в `ExpressionSemantics` payload и одним матчем в codegen, без правки analyze internals или helpers.
29. Как ревьюер, я хочу видеть, что новый builder регистрируется добавлением одного вызова в `analyze()` с явными зависимостями через типизацию, без работы с PassDescriptor / DataToken.
30. Как разработчик codegen, я хочу, чтобы `CodegenView` (`types/data/codegen_view.rs`) исчез, потому что вся работа с `AnalysisData` идёт через публичный API кластеров.
31. Как разработчик codegen, я хочу, чтобы текущие helper-методы `AnalysisData` (`is_dynamic`, `class_needs_state`, `attr_is_function`, `expr_deps`, `parent_each_blocks`, `bind_target_symbol`, `each_is_destructured`, `expression_blockers`, `attr_expression_blockers`, `node_ref_symbols`, `shorthand_symbol`, `has_attribute`, `static_text_attribute_value`, …) исчезли, и их работа была доступна как поле в payload соответствующего кластера.
32. Как разработчик performance-чувствительной части (бенчмарки, wasm), я хочу пропустить Phase 6 checker, чтобы получить только семантику без диагностик за минимальное время.
33. Как разработчик infrastructure (compiler-level), я хочу, чтобы один analyze-вызов на один компонент был чистой функцией без shared state, чтобы тривиально параллелить compile N файлов через rayon на верхнем уровне.

## Implementation Decisions

### Архитектурные

1. **Пять кластеров.** Block, Attribute, Element, Expression, Reactivity. Identity — NodeId для template-узлов; SymbolId / ReferenceId / OxcNodeId для Reactivity. Strings никогда не публичные ключи. Решение основано на: потребитель уже знает категорию AST-узла, по которому делает лукап; единый `Semantics` enum привёл бы к двухуровневому матчу с 5 верхнеуровневыми вариантами и обязательными отвергнутыми ветками.

2. **Thick payload по умолчанию.** Каждый payload пред-вычисляет любое решение, которое потребитель иначе собрал бы из ≥2 сторов или повторил бы в ≥2 потребительских сайтах. Решения, которые остаются «тонкими» (не пред-вычисленными), помечаются явно при проектировании конкретного кластера и согласуются с разработчиком analyze.

3. **Identity model.** `OxcNodeId` — единственный AST-хук в payload-ах для ссылок на JS AST. `JsAst` keyed by `OxcNodeId` напрямую (исторические `StmtHandle` / `ExprHandle` уже удалены). Никаких параллельных handle-систем.

4. **No binding-pattern repack.** `BindingPattern`-поддеревья (`$props`, `$state`, `$derived`, `{@const}`, `{#snippet}` params, `{#each ... as pat}`, `{#await ... then pat}`, `let:` directives) не переупаковываются в кластер-локальные DTO. Cluster payload несёт `OxcNodeId` pattern-а; потребитель ходит по pattern-у через единый `walk_bindings` helper из `svelte_component_semantics`.

5. **Composite answers.** Cluster builder композирует факты из allowed inputs (AST, `ComponentSemantics`, `ReactivitySemantics`, narrow analyzer-output tables вроде `BlockerData`) в одно высокоуровневое решение. Потребитель не пере-собирает то же решение из мелких фактов.

6. **Dependency boundary.** Cluster builder принимает только: AST (`Component`, `JsAst`), `ComponentSemantics`, `ReactivitySemantics`, и узкие generic analyzer-output tables (`BlockerData`, `TemplateTopology`, `TemplateElementIndex`, `ScriptDeclarations`). Не принимает `&AnalysisData` целиком, не читает другой cluster payload без явного декларирования зависимости.

7. **Cluster traversal budget.** Каждый cluster builder использует не более 1 walk instance script + 1 walk module script + 1 walk template. Sub-walks по template-owned subtrees разрешены, если факт не достижим через `ComponentSemantics`. Re-walking тех же узлов внутри одного кластера запрещён.

8. **`ExpressionTag` владелец — `ExpressionSemantics`.** Это пятый кластер. Ключевой потребитель — codegen. Используется и для standalone `{expr}` в фрагменте, и для expression внутри атрибута (Attribute payload содержит NodeId, тянет факты отсюда), и для collection `{#each expr as ...}`, и для аргумента `{@render fn(arg)}`. Transform expression-узлы как единицы не классифицирует — продолжает работать per-reference через ReactivitySemantics.

9. **`{@debug}` и `{@html}` — варианты `BlockSemantics`.** Логически они block-теги (форма `{@...}`), payload каждого нетривиален: `DebugTagSemantics { identifiers: SmallVec<[NodeId; 4]>, dev_only: bool }`, `HtmlTagSemantics { expression_node_id, parent_strategy }`. Документация `SEMANTIC_LAYER_ARCHITECTURE.md` (раздел про out-of-scope строки 162–163) обновляется.

10. **Async — поле кластерного payload, не peer-кластер.** Подтверждение существующего решения в SEMANTIC_LAYER_ARCHITECTURE.md. `EachAsyncKind`, `IfAsyncKind`, `AwaitAsyncKind`, etc. — поля в существующих вариантах. Top-level `await` в script и async-mode runtime harness живут в `BlockerData`, не в cluster payload.

### Pipeline

11. **Шесть фаз без DAG-резолвера.** Фиксированный порядок вызовов в `analyze()`. Зависимости проверяются Rust типизацией. `passes/mod.rs` PassDescriptor-резолвер удаляется.

12. **Phase 1 — scope & scripts.** Пять единиц:
    - `ComponentSemanticsBuilder` (живёт в `svelte_component_semantics` крейте, остаётся как есть).
    - `resolve_component_name()` — util-функция, считает финальное имя резолвом конфликтов с symbols + reserved keywords. Не builder.
    - `ScriptDeclarationsBuilder` — узкий: `props_declaration`, `exports`, `props_id`. Полный `ScriptInfo` god-struct исчезает; внутренние поля либо переезжают в `ComponentSemantics` (если про symbols), либо удаляются.
    - `BlockerDataBuilder` — async barriers, per-symbol map. Два потребителя в разных доменах (codegen для script body splitting; ExpressionSemantics для per-expression rollup).
    - `CustomElementInfoBuilder` — `is_custom_element_target`, `custom_element_compile_flag`, `ce_config` (parsed `<svelte:options customElement>`), `custom_element_slot_names`.

13. **Phase 2 — reactivity.** `ReactivitySemanticsBuilder` остаётся как есть, дополняется side-output-ом `FxHashMap<OxcNodeId, RuneCallKind>` для transform. Старый `passes/js_analyze/script_runes.rs` удаляется; transform читает `data.reactivity.rune_calls()`.

14. **Phase 3 — generic indices.** Один `GenericIndicesBuilder` с двумя выходами: `TemplateTopology` (parent / ancestor граф) + `TemplateElementIndex` (селектор-индекс tag/class/id для CSS pruning). Один template walk, два map-выхода. Поглощает `passes/fragment_topology.rs` и часть `passes/template_side_tables.rs`.

15. **Phase 4 — clusters.** Четыре builder-а в порядке: Block → Expression → Attribute → Element. Порядок обусловлен зависимостями: Attribute payload содержит NodeId expression-а → Expression должен быть готов раньше. Element payload содержит NodeId атрибутов → Attribute раньше Element.

16. **Phase 5 — global rollup.** Два builder-а:
    - `RuntimePlanBuilder` — агрегатор всех component-level booleans (`needs_push`, `has_component_exports`, `has_bindable`, `has_stores`, `needs_props_param`, `legacy_init`, `needs_context`, `needs_sanitized_legacy_slots`, `needs_component_bind_ownership`). Текущая `build_runtime_plan` функция в `lib.rs` вытаскивается в свой builder; inline-вычисления `needs_context` и связанных bool-ов теперь рождаются здесь.
    - `CssAnalysisBuilder` — поглощает `passes/css_analyze.rs`, `passes/css_prune_index.rs`, `passes/css_prune.rs`. Один builder с тремя внутренними этапами (parse → index → prune).

17. **Phase 6 — checker.** Отдельный crate `svelte_check`. Один `ComponentChecker` с visitor-ами по правилам. Поглощает `validate/{runes,legacy,stores,non_reactive_update,experimental_async}` + `passes/template_validation` + `passes/template_validation/a11y` + `IgnoreData` сканер + `warning_filter` логику.

### Обработка текущих side-tables

18. **`ExpressionInfo` (per-expression bag-of-facts) исчезает.** Поля переезжают в `ExpressionSemantics` payload как enum / bitflags / поля.

19. **`BindSemanticsData`, `TemplateSemanticsData`, `DirectiveModifierFlags`** растворяются в `AttributeSemantics` payload (поля `target`, `each_context_vars`, `event_modifiers`).

20. **`ElementFacts`, `ElementFlags`, `html_tag_in_svg/mathml`, `TitleElementData`** растворяются в `ElementSemantics` payload.

21. **`SnippetData`, `DebugTagData`** растворяются в `BlockSemantics` payload (соответствующие variants).

22. **`FragmentFacts`, `RichContentFacts`, `FragmentNamespaces`, `fragment_blockers`** сворачиваются в `children_summary` payload-а родителя fragment-а (Element / Block / Component-root).

23. **`DynamismData`** распадается на bit `dynamism: bool` в payload-ах Element / Attribute / Expression. Сама таблица удаляется.

24. **`PickledAwaitOffsets`** становится полем `is_pickled_await: bool` в `ExpressionSemantics` для тех expressions, что содержат `await`. Transform (`template_rewrites.rs`) переписывается на лукап по NodeId вместо span. Текущая offset-таблица удаляется.

25. **`ProxyStateInits`, `has_class_state_fields`, `has_store_member_mutations`** удаляются полностью. У `ProxyStateInits` нет внешних потребителей. Остальные были derivation steps для RuntimePlan — пересчитываются inline в `RuntimePlanBuilder`.

26. **`OutputPlanData`** расщепляется: `RuntimePlan` и `CssAnalysis` становятся top-level полями `AnalysisData`; `IgnoreData` уезжает в `svelte_check`; `needs_context`, `needs_sanitized_legacy_slots`, `needs_component_bind_ownership` — inline в `RuntimePlanBuilder`; `custom_element_*` уходят в `CustomElementInfo`; `component_name` — top-level поле.

27. **`BlockAnalysis`** (пустой struct) удаляется.

28. **`CodegenView`** (`types/data/codegen_view.rs` обёртка над AnalysisData) удаляется. Codegen читает `AnalysisData` через публичный API кластеров напрямую.

29. **`ComponentScoping`** обёртка над `ComponentSemantics` растворяется. analyze / transform / codegen используют `ComponentSemantics` напрямую. Helper-методы из `ComponentScoping` либо переезжают в сам `ComponentSemantics`, либо в `TemplateTopology` / `TemplateElementIndex` где уместнее.

### Удаляемые passes

30. **Удаляются целиком:** `passes/bind_semantics`, `passes/build_component_semantics`, `passes/bundles`, `passes/collect_symbols`, `passes/content_types`, `passes/dynamism`, `passes/element_flags`, `passes/enrich_script_info`, `passes/executor`, `passes/finalize_component_name`, `passes/fragment_topology`, `passes/js_analyze` (все 6 sub-passes), `passes/post_resolve`, `passes/template_side_tables`, `passes/template_validation` (целиком включая `a11y`).

31. **Удаляется DAG-резолвер:** `passes/mod.rs` целиком (PassKey, DataToken, PassDescriptor, `resolve_execution_order`, `default_stage_execution_order`, `PRE_TEMPLATE_SCRIPT_STAGE` / `INDEX_BUILD_STAGE` / `POST_TEMPLATE_ANALYSIS_STAGE` / `TEMPLATE_EXECUTION_STAGE` / `VALIDATION_STAGE`).

32. **Сохраняется `utils/`** как shared библиотека helper-ов (`script_info`, `simple_expression`, `attributes`, `binding_pattern`, `ce_config`, `events`, `html_tree_validation`, `ident_gen`, `legacy_slot`, `property_key`, `var_decl_kind`).

### Граница crate-ов

33. **Зависимости:** `svelte_check` зависит от `svelte_analyze`. `svelte_analyze` НЕ зависит от `svelte_check`. `svelte_compiler` оркестрирует pipeline: parse → analyze → check → transform → codegen.

34. **Публичный API `AnalysisData`** должен быть достаточен для всех внешних потребителей (`svelte_check`, `svelte_transform`, `svelte_codegen_client`, `svelte_compiler`) без `pub(crate)` хаков.

### Migration mechanics

35. **Per-cluster, end-to-end.** Один кластер за раз, целиком: builder support → public answer variant → real consumer migrated to new answer → old consumer-side meaning assembly removed → tests green.

36. **`#[deprecated]` как opening step.** Старая API-поверхность для kind-а помечается deprecated до начала переписывания потребителей. Новый код, добавленный во время миграции, видит warning и выбирает новый API.

37. **Новый builder заводится как новый pass рядом со старыми.** Во время миграции DAG-резолвер продолжает оркестрировать оба набора. Codegen постепенно переключается с deprecated API на новый. Когда последний потребитель ушёл — старый pass удаляется.

38. **DAG-резолвер удаляется последним.** После того, как все 14 старых passes исчезли и все builder-ы кластеров стали top-level вызовами в `analyze()`, `passes/mod.rs` удаляется и `analyze()` переписывается на 6 фаз.

39. **Порядок миграции.** `Block (доделать DebugTag/HtmlTag)` → `Expression` → `Attribute` → `Element`. После всех четырёх кластеров — создаётся `svelte_check` крейт, validation переезжает. Затем удаляются осиротевшие passes. Последним удаляется DAG.

40. **Каждая миграционная единица фиксируется в `debt.md`.** Между шагами в коде живут две формы (старая + новая) — это технический долг, в `debt.md` фиксируется отдельной строкой на каждый кластер. Строка удаляется, когда старая форма удалена.

### Concurrency

41. **Within-component concurrency не закладывается.** Builder-ы single-threaded. Causes: типичный component = ≤ нескольких сотен AST-узлов, overhead thread spawn / sync доминирует над выигрышем.

42. **Builder-ы остаются `Send` и pure** (input → output, без shared mutable state помимо своего выхода). Это сохраняет потенциал на точечный `rayon::join` (например, CSS pipeline параллельно с Phase 4 кластерами) если профайлинг покажет узкое место.

43. **Compiler-level parallelism (N-files in parallel) — out of scope этого PRD.** Это работа `svelte_compiler` / `napi_compiler` / `wasm_compiler`-оркестраторов, не analyze.

### Документация

44. **`SEMANTIC_LAYER_ARCHITECTURE.md` обновляется.** Правки: раздел про DebugTag / HtmlTag (out-of-scope → in-scope как варианты `BlockSemantics`); добавление `ExpressionSemantics` как пятого кластера; перевод validation в отдельный crate (Cluster builders диагностики не эмитят).

45. **`debt.md` получает новые строки.** Каждая удаляемая side-table или pass — строка с описанием состояния миграции и точкой удаления. Строка про parent-link в svelte_ast — отдельный долгосрочный пункт ("AST должен хранить parent_id и parent_fragment_id, после этого `TemplateTopology` исчезает").

46. **`CODEBASE_MAP.md` обновляется** по мере появления новых cluster-ов и удаления passes.

## Testing Decisions

### Что делает тест хорошим

Тестируем внешнее поведение, не имплементационные детали. Для cluster builder — это форма payload, в которую builder отображает заданный AST-вход. Для checker — это набор диагностик, которые он эмитит на заданный component. Не тестируем внутреннюю структуру (например, в каком порядке builder вызывает helper-ы — это implementation detail, изменится при первой переработке).

Уровни тестов:

1. **End-to-end snapshot тесты.** `tasks/compiler_tests/cases2/` (existing) и `tasks/diagnostic_tests/cases/` (existing) остаются source of truth корректности. Любая миграция должна оставлять `just test-compiler && just test-diagnostics` зелёными на каждом merge-able commit.
2. **Unit тесты на cluster builder-ы.** Per-builder тест: small `Component` → ожидаемая форма payload для всех variants. Должен ловить регрессии payload-формы без необходимости гонять полный compiler.
3. **Unit тесты на checker-rules.** Per-rule тест: small `AnalysisData` + AST → ожидаемый набор диагностик.
4. **Property тесты — out of scope** этого PRD, не блокируют рефакторинг.

### На какие модули пишутся unit-тесты

- **`BlockSemanticsBuilder`** — новые DebugTag и HtmlTag варианты обязательны к юнит-тестам. Существующие variants (Each, If, Await, Key, Snippet, ConstTag, Render) уже покрыты `compiler_tests/cases2/`.
- **`ExpressionSemanticsBuilder`** — новый, центральный кластер. Юнит-тесты на каждое поле payload (`legacy_wrap`, `async_kind`, `memoization`, `role`, `shape`, `is_pickled_await`).
- **`AttributeSemanticsBuilder`** — новый. Юнит-тесты по одному на каждый variant enum (`HtmlBind`, `Event`, `ComponentProp`, `ComponentBind`, `ComponentSpread`, `Use`, `Transition`, `Animate`, `Attach`, `BoundaryHandler`, etc.).
- **`ElementSemanticsBuilder`** — новый. Юнит-тесты на каждый variant (`Html`, `SvelteElement`, `Component`, `DynamicComponent`, `SelfComponent`, `Boundary`, `SpecialTarget` × 4 sub-variants).
- **`ComponentChecker` (crate `svelte_check`)** — per-rule unit-тесты.

### Не пишутся отдельные юнит-тесты

- `ComponentSemanticsBuilder`, `ReactivitySemanticsBuilder` — уже существуют и покрыты.
- `ScriptDeclarationsBuilder`, `BlockerDataBuilder`, `CustomElementInfoBuilder`, `GenericIndicesBuilder`, `RuntimePlanBuilder`, `CssAnalysisBuilder` — узкие builder-ы, покрываются end-to-end тестами компилятора. Если профайлинг или баг покажет необходимость — точечный юнит добавляется.

### Прецедент

- Снапшот-тесты compiler/diagnostics: `tasks/compiler_tests/test_v3.rs`, `tasks/diagnostic_tests/test_diagnostics.rs`. Канонический формат для end-to-end проверок.
- Существующие модульные тесты analyze: `crates/svelte_analyze/src/tests.rs` — пример формата для cluster-builder-тестов.
- Тесты parser-а в `crates/svelte_parser/` — пример узкоформатных юнит-тестов на builder-крейтов.

### Per-step acceptance gate

Каждый миграционный шаг (см. Implementation Decision 39) считается merge-able только когда:

- `just test-compiler` зелёный
- `just test-diagnostics` зелёный
- `just clippy-strict` зелёный
- удалена ≥1 deprecated сущность (pass / side-table / helper-метод)
- количество `&&`-цепочек, читающих ≥2 стора, в codegen не больше предыдущего шага (мониторится через grep на review)

### Финальные acceptance criteria

- 0 чтений `ExpressionInfo` (как типа) из codegen и transform
- 0 чтений deprecated side-tables (`BindSemanticsData`, `DynamismData`, `ElementFlags`, etc.) из любого внешнего крейта
- `crates/svelte_analyze/src/passes/` каталог удалён
- `AnalysisData` имеет ровно перечисленные в Solution top-level поля, без legacy
- 0 ad-hoc helper-методов на `AnalysisData` сверх accessor-ов к этим полям
- `crates/svelte_analyze/src/validate/` каталог удалён, `crates/svelte_check/` существует и содержит все правила
- `just test-compiler && just test-diagnostics && just clippy-strict` зелёные

## Out of Scope

1. **Рефакторинг `svelte_ast`.** Parent-link / parent-fragment-id в `Node` — отдельный долгосрочный пункт `debt.md`. После него `TemplateTopology` исчезает, но это работа уровня parser-а / AST крейта, не analyze.

2. **Compiler-level concurrency.** Параллельный analyze N компонентов через rayon — задача `svelte_compiler` / `napi_compiler` / `wasm_compiler`-оркестратора. Этот PRD только обеспечивает, что один analyze-вызов остаётся `Send + чисто функциональным`.

3. **Within-component concurrency.** Точечный `rayon::join` (например, CSS pipeline параллельно с Phase 4) — потенциально в будущем по результатам профайлинга. Не закладывается архитектурно в этом PRD.

4. **Property-based / fuzz тесты на cluster builder-ы.** End-to-end snapshot и unit-тесты — достаточны для acceptance этого PRD.

5. **Изменение публичного API `svelte_diagnostics`.** Checker-крейт использует существующие `Diagnostic` / `Severity` типы.

6. **Миграция transform-а.** Transform работает per-reference через `ReactivitySemantics` и не классифицирует expression-узлы как единицы. Никаких изменений в transform-крейте этот PRD не требует, кроме точечного rewrite-а `is_pickled_await(span.start)` → лукап по NodeId (decision 24).

7. **Изменение CSS preprocessor / output формата.** `CssAnalysisBuilder` поглощает существующие 3 css-pass-а без изменения CSS-семантики или output-формата.

8. **Property тесты на фрагменты codegen-а.** Codegen меняется только в части переключения с deprecated AnalysisData helper-ов на cluster API; behavior сохраняется (проверяется существующими `compiler_tests/cases2/`).

9. **Изменение представления async (separate cluster).** Async-поля живут в payload каждого кластера. Это уже зафиксировано в `SEMANTIC_LAYER_ARCHITECTURE.md` и не пересматривается в этом PRD.

10. **Удаление `ComponentSemantics` крейта.** Crate `svelte_component_semantics` остаётся как есть. Изменения только в обёртке `ComponentScoping` в analyze (она растворяется).

## Further Notes

### Связь с существующей документацией

`SEMANTIC_LAYER_ARCHITECTURE.md` — предшествующий рабочий документ. Часть его решений (Block/Attribute/Element как кластеры; Identity Model; Storage Content Rule; Lowering Boundary; Composite answers; Dependency Boundary; Escalation On Missing Facts; Traversal Budget; No binding-pattern repack) переносятся в этот PRD как Implementation Decisions. Расхождения:

- Исходный документ выделял Async как «possibly fourth cluster»; финальное решение — async-поле каждого кластера, не peer-кластер. Это было зафиксировано в самом документе (раздел Async Semantics после `EachBlock` слайса) и здесь подтверждено.
- Исходный документ оставлял ExpressionTag «open»; здесь решено — пятый кластер `ExpressionSemantics`, владелец одного NodeId.
- Исходный документ оставлял `{@debug}` и `{@html}` out-of-scope для Block; здесь — варианты `BlockSemantics`.
- Исходный документ говорил «cluster builders могут эмитить diagnostics»; здесь — нет, валидация standalone в `svelte_check`.

После принятия этого PRD `SEMANTIC_LAYER_ARCHITECTURE.md` обновляется до соответствия (правки строк 162–163 про debug/html; добавление раздела про Expression cluster; правка про diagnostics в cluster builders).

### Связь с `CLAUDE.md`

Решения PRD соответствуют ключевым догмам из `CLAUDE.md`:

- **Smart analyzer / dumb codegen.** Реализуется Thick payload: каждое решение пред-вычислено в analyzer-е, codegen эмитит без композиции.
- **Existing weird/smell/garbage code do not allow write same shit code.** PRD удаляет 14 passes, 18 side-tables, 70 helper-ов, DAG-резолвер, `CodegenView`-обёртку, `ComponentScoping`-обёртку.
- **Prefer compiler terminology over framework-specific naming.** Используются compiler-термины: builder, store, cluster, payload, reference, identity. Запрещённые в `CLAUDE.md` термины не вводятся для новых сущностей. Сохраняются исторические имена `RuntimePlan` (используется в коде) — переименование out of scope.

### Метрики successful миграции

- Размер `passes/` каталога: 14 → 0.
- Размер `validate/` + `passes/template_validation*` в analyze: 7 файлов → 0 (всё в `svelte_check`).
- Размер `types/data/` в analyze: 18 side-tables → cluster store-структуры + 7 top-level фактов.
- Количество ad-hoc helper-методов на `AnalysisData`: 70 → 0 (только accessor-ы к 12 полям).
- Количество `&&`-цепочек, читающих ≥2 стора, в `svelte_codegen_client`: текущая base-line → 0.
- Количество template walks в analyze: текущая (≥6) → ≤ 5 (по одному на cluster Phase 4 + один в Phase 3).

### Риски

1. **Большие cluster payload-ы.** Thick payload приводит к крупным enum-вариантам с много полей. Компенсация: каждый variant отвечает на конкретный consumer-сайт, поля документируются inline в коде. При признаках «слишком большой variant» — кандидат на расщепление variant-а или на explicit `manual exception` от Thick правила.
2. **Long-running migration.** Между шагами в коде живут две формы. Защита: `#[deprecated]` warning + `debt.md` строки + acceptance gate на тесты + правило «удалить старое до закрытия cluster-миграции».
3. **Boundary `svelte_check`.** Чтобы checker мог читать всё нужное через публичный API без `pub(crate)` хаков, придётся явно расширить публичный API analyze. Это может потребовать дополнительных аксессоров, что увеличит surface. Защита: каждый новый pub-метод проходит ревью.
4. **`PickledAwaitOffsets` миграция transform-а.** Transform лукапит по `span.start`, что требует пере-привязки на NodeId. Узкая правка в `transform/template_rewrites.rs`, не блокирующая, но требующая внимательности.
5. **Удаление `ComponentScoping`.** Обёртка имеет помимо delegation-методов несколько кэшей и helper-ов. Каждый — point-by-point переезжает либо в `ComponentSemantics`, либо в `TemplateTopology`. Возможны регрессии перформанса если кэш был критичный.

### Ссылки

- `CLAUDE.md` — догмы и глоссарий.
- `ARCHITECTURE.md` — инварианты по крейтам.
- `CODEBASE_MAP.md` — текущая карта типов.
- `SEMANTIC_LAYER_ARCHITECTURE.md` — рабочий документ semantic layer (обновляется при принятии PRD).
- `debt.md` — известный технический долг (получает новые строки на каждый умирающий side-table).
- `ROADMAP.md` — статус портирования.
- `crates/svelte_codegen_client/src/template/each_block.rs` — reference implementation для consumer-side decision composition (`gen_each_block` / `EachPlan`).
