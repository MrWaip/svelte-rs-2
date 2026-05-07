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
├── reactivity           ReactivitySemantics        (+ rune_calls() accessor)
├── blocks               BlockSemanticsStore        (+ ignore-флаги для влияющих на JS-output кодов)
├── attributes           AttributeSemanticsStore
├── elements             ElementSemanticsStore     (+ slot_target / wrapped_in_css_container и т.д.)
├── expressions          ExpressionSemanticsStore
├── fragments            FragmentSemanticsStore     (per-fragment context-bundle)
├── topology             TemplateTopology
├── element_index        TemplateElementIndex
├── script_declarations  ScriptDeclarations
├── blockers             BlockerData                (+ await_reactivity_loss_ignored)
├── custom_element       CustomElementInfo
├── runtime              RuntimePlan                (+ ParamsShape / EpilogueKind / bitflags / Option-структуры)
├── css                  CssAnalysis
└── component_name       String
```

`FragmentSemanticsStore` — utility store рядом с `TemplateTopology` / `TemplateElementIndex`. Не «кластер per Svelte kind» (Decision 1 не нарушается). Хранит per-fragment контекст-bundle: `preserve_ws`, `inside_pre`, `inside_textarea`, `inside_script`, `inside_head`, `namespace`, `parent_tag`, `needs_text_first_next`. Codegen читает один лукап вместо протаскивания флагов через рекурсивный `FragmentCtx`.

`ScriptTargets` (отдельное поле для transform-rewrite-таргетов) НЕ создаётся: текущий публичный API `ReactivitySemantics` (с Decision 13: `rune_calls()` accessor) + AST-shape матч для one-node решений достаточен. Transform остаётся single-match consumer-ом.

Pipeline:

```text
Phase 1  scope & scripts
  ComponentSemanticsBuilder
  resolve_component_name (util)
  ScriptDeclarationsBuilder
  BlockerDataBuilder                 (+ await_reactivity_loss_ignored поднятый из IgnoreData)
  CustomElementInfoBuilder

Phase 2  reactivity
  ReactivitySemanticsBuilder         (включает rune-call map; pub fn rune_calls() accessor для transform)

Phase 3  template-walk indices
  TemplateIndicesBuilder             (TemplateTopology + TemplateElementIndex, один walk)
  FragmentSemanticsBuilder           (per-fragment context-bundle, отдельный walk)

Phase 4  clusters
  BlockSemanticsBuilder              (доделать DebugTag / HtmlTag варианты; HtmlTag получает hydration_html_changed_ignored)
  ExpressionSemanticsBuilder         (новый, поглощает PickledAwaitOffsets)
  AttributeSemanticsBuilder          (новый)
  ElementSemanticsBuilder            (новый; slot_target / wrapped_in_css_container и весь exhaustive mapping)

Phase 5  global rollup
  RuntimePlanBuilder                 (полное emit-описание component-функции:
                                      ParamsShape / EpilogueKind / BareImportsFlags / ExportsFlags /
                                      StoresSetup / needs_ownership_validator)
  CssAnalysisBuilder                 (CSS pipeline целиком)

Phase 6  checker (crate svelte_check)
  ComponentChecker                   (читает готовую AnalysisData + AST; владеет IgnoreData приватно)
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
34. Как разработчик codegen-prepare-pipeline-а, я хочу читать `data.fragments.get(fid).preserve_ws` (и связанные флаги `inside_pre`/`inside_textarea`/`inside_script`/`inside_head`/`namespace`/`parent_tag`) одним лукапом, чтобы перестать тащить контекст через рекурсивный `FragmentCtx`.
35. Как разработчик codegen, я хочу читать `data.runtime.fn_params` (`ParamsShape::AnchorOnly | AnchorAndProps`) и матчиться один раз вместо ИЛИ-цепочки `runtime.needs_props_param || has_bubble_events || has_legacy_slots || needs_sanitized_legacy_slots()`, чтобы решение «какие параметры у компонент-функции» приходило в готовом виде.
36. Как разработчик codegen, я хочу читать `data.runtime.epilogue` (`EpilogueKind`) — единый enum для матрицы `(needs_push, needs_pop_with_return, has_stores)`, чтобы хвост компонент-функции эмитился одним матчем без 4-way `if/else`-цепочки.
37. Как разработчик codegen, я хочу читать `data.runtime.bare_imports` (bitflags) и `data.runtime.exports_flags` (bitflags) вместо 4-х/3-х независимых bool-проверок для prelude-импортов и экспортов, чтобы каждое решение шло одним bit-тестом.
38. Как разработчик codegen, я хочу, чтобы `data.runtime.stores_setup: Option<StoresSetup>` нёс уже enriched per-store факт `base_via_legacy_state: bool`, чтобы codegen не делал per-store повторный лукап `binding_semantics(base_symbol)`.
39. Как разработчик codegen, я хочу, чтобы `has_bubble_events` и `has_legacy_slots` НЕ были полями `RuntimePlan` (они нужны только для вычисления `ParamsShape`), чтобы публичный API не содержал нечитаемых нигде bool-ов.
40. Как разработчик transform, я хочу, чтобы classification rune-call-узлов (включая class-state-fields, `$inspect`-calls, rune-инициализаторов в declarators) шла через единственный API `data.reactivity.rune_calls().kind(oxc_node_id)`, чтобы transform не делал AST-walks для классификации и не повторял работу analyzer-а.
41. Как разработчик transform, я хочу, чтобы решение «оборачивать ли `for-of await` в `$.for_await_track_reactivity_loss`» шло через узкое поле `data.blockers.await_reactivity_loss_ignored: FxHashSet<OxcNodeId>` вместо чтения внутренней `IgnoreData` со строковым ключом, чтобы граница analyze→check→transform была чёткой и не делилась за свою сторону.
42. Как разработчик transform, я хочу, чтобы решения уровня AST-shape (assign vs assign_async, console.log dev wrap, binary equals dev wrap) делались transform-ом single-match без предсчёта в analyzer, чтобы analyzer не предсчитывал тривиально-derive-уемые факты.
43. Как разработчик checker, я хочу, чтобы `IgnoreData` (`<!-- svelte-ignore -->` сканер) был приватной частью `svelte_check`, не утекал в `AnalysisData`, чтобы codegen и transform не имели соблазна читать ignore-коды как строки.
44. Как разработчик analyze, я хочу, чтобы для каждого ignore-кода, влияющего на JS-output, был узкий пред-вычисленный fact как поле payload соответствующего кластера (`BlockSemantics::HtmlTag.hydration_html_changed_ignored`, `BlockerData.await_reactivity_loss_ignored`), чтобы добавление нового runtime-зависимого ignore-кода требовало явного добавления поля и review-чека.
45. Как ревьюер, я хочу видеть, что новый ignore-код, влияющий на runtime, добавляется одним полем в payload соответствующего кластера, плюс одним матчем в codegen/transform — никакого `is_ignored(id, "code-string")` в potрebителях.
46. Как разработчик codegen-attribute-pipeline-а, я хочу, чтобы `iter_store_bindings()` возвращал enriched `StoreBinding { base_symbol, store_symbol, base_via_legacy_state }`, чтобы codegen читал готовый bool вместо повторного `binding_semantics`-лукапа в loop-е.

## Implementation Decisions

### Архитектурные

1. **Пять кластеров.** Block, Attribute, Element, Expression, Reactivity. Identity — NodeId для template-узлов; SymbolId / ReferenceId / OxcNodeId для Reactivity. Strings никогда не публичные ключи. Решение основано на: потребитель уже знает категорию AST-узла, по которому делает лукап; единый `Semantics` enum привёл бы к двухуровневому матчу с 5 верхнеуровневыми вариантами и обязательными отвергнутыми ветками.

2. **Thick payload по умолчанию.** Каждый payload пред-вычисляет любое решение, которое потребитель иначе собрал бы из ≥2 сторов или повторил бы в ≥2 потребительских сайтах. Решения, которые остаются «тонкими» (не пред-вычисленными), помечаются явно при проектировании конкретного кластера и согласуются с разработчиком analyze.

3. **Identity model.** `OxcNodeId` — единственный AST-хук в payload-ах для ссылок на JS AST. `JsAst` keyed by `OxcNodeId` напрямую (исторические `StmtHandle` / `ExprHandle` уже удалены). Никаких параллельных handle-систем.

4. **No binding-pattern repack.** `BindingPattern`-поддеревья (`$props`, `$state`, `$derived`, `{@const}`, `{#snippet}` params, `{#each ... as pat}`, `{#await ... then pat}`, `let:` directives) не переупаковываются в кластер-локальные DTO. Cluster payload несёт `OxcNodeId` pattern-а; потребитель (codegen или transform) ходит по pattern-у сам через единый `walk_bindings` helper из `svelte_component_semantics`, делая точечные lookup-ы на пройденных узлах / symbols через `ComponentSemantics` / `ReactivitySemantics` / cluster API. Цель — analyzer не зеркалит структуру AST в payload, потребитель не воспроизводит работу analyzer-а.

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

13. **Phase 2 — reactivity.** `ReactivitySemanticsBuilder` остаётся как есть, дополняется side-output-ом `ScriptRuneCalls` (per-OxcNodeId rune-classification). Старый `passes/js_analyze/script_runes.rs` удаляется; `ScriptRuneCalls` перестаёт быть top-level полем `AnalysisData` и становится приватной структурой внутри `ReactivitySemantics`. Появляется новый pub-метод `ReactivitySemantics::rune_calls(&self) -> &ScriptRuneCalls` (либо узкий `rune_kind_at(node) -> Option<RuneKind>`). Через этот accessor transform делает classification всех rune-зависимых сайтов: class-state-fields в class body, `this.x = $state(...)` в конструкторах, rune-инициализаторов в declarators, `$inspect`-calls. Никаких classification AST-walks transform-side не остаётся.

14. **Phase 3 — template-walk indices.** Два builder-а с одним walk-ом каждый:
    - `TemplateIndicesBuilder` — `TemplateTopology` (parent / ancestor граф) + `TemplateElementIndex` (селектор-индекс tag/class/id для CSS pruning). Поглощает `passes/fragment_topology.rs` и часть `passes/template_side_tables.rs`.
    - `FragmentSemanticsBuilder` — `FragmentSemanticsStore`, per-fragment context-bundle: `preserve_ws`, `inside_pre`, `inside_textarea`, `inside_script`, `inside_head`, `namespace`, `parent_tag`, `needs_text_first_next`. Один отдельный walk сверху-вниз с накоплением context-флагов. Не объединяется с `TemplateIndicesBuilder` — раздельные builder-ы для cohesion (один index, один context-bundle).

15. **Phase 4 — clusters.** Четыре builder-а в порядке: Block → Expression → Attribute → Element. Порядок обусловлен зависимостями: Attribute payload содержит NodeId expression-а → Expression должен быть готов раньше. Element payload содержит NodeId атрибутов → Attribute раньше Element.

16. **Phase 5 — global rollup.** Два builder-а:
    - `RuntimePlanBuilder` — производит **полное** emit-описание component-функции для codegen-а. Кодген матчится по полям/вариантам без ИЛИ-цепочек и без AST-walks за component-level фактами. Текущая `build_runtime_plan` функция в `lib.rs` вытаскивается в свой builder; inline-сборки в `lib.rs::generate` (включая `has_bubble_events`, `has_legacy_slots`, `iter_store_bindings` enrichment, prelude-imports composition, exports composition, epilogue matrix) рождаются здесь. Промежуточные bool-ы, нечитаемые потребителем (`has_bubble_events`, `has_legacy_slots`, `needs_pop_with_return` сами по себе), остаются приватными intermediate-значениями `RuntimePlanBuilder`-а; в публичный API не попадают. См. Decision 47 — конкретная форма полей.
    - `CssAnalysisBuilder` — поглощает `passes/css_analyze.rs`, `passes/css_prune_index.rs`, `passes/css_prune.rs`. Один builder с тремя внутренними этапами (parse → index → prune).

17. **Phase 6 — checker.** Отдельный crate `svelte_check`. Один `ComponentChecker` с visitor-ами по правилам. Поглощает `validate/{runes,legacy,stores,non_reactive_update,experimental_async}` + `passes/template_validation` + `passes/template_validation/a11y` + `IgnoreData` сканер + `warning_filter` логику. **`IgnoreData` — приватная структура `svelte_check`**, не утекает в `AnalysisData`. Каждый ignore-код, влияющий на shape JS-output (текущие — `hydration_html_changed`, `await_reactivity_loss`), пред-вычисляется analyzer-ом как **узкое поле в payload соответствующего кластера** (см. Decision 48), а не лукапится по строковому коду на стороне consumer-а. Защита от регресса: добавление нового ignore-кода, влияющего на runtime, требует явного добавления поля в payload, что делает решение видимым в review.

### Обработка текущих side-tables

18. **`ExpressionInfo` (per-expression bag-of-facts) исчезает.** Поля переезжают в `ExpressionSemantics` payload как enum / bitflags / поля.

19. **`BindSemanticsData`, `TemplateSemanticsData`, `DirectiveModifierFlags`** растворяются в `AttributeSemantics` payload (поля `target`, `each_context_vars`, `event_modifiers`).

20. **`ElementFacts`, `ElementFlags`, `html_tag_in_svg/mathml`, `TitleElementData`** растворяются в `ElementSemantics` payload.

21. **`SnippetData`, `DebugTagData`** растворяются в `BlockSemantics` payload (соответствующие variants).

22. **`FragmentFacts`, `RichContentFacts`, `FragmentNamespaces`, `fragment_blockers`** сворачиваются в `children_summary` payload-а родителя fragment-а (Element / Block / Component-root).

23. **`DynamismData`** распадается на bit `dynamism: bool` в payload-ах Element / Attribute / Expression. Сама таблица удаляется.

24. **`PickledAwaitOffsets`** становится полем `is_pickled_await: bool` в `ExpressionSemantics` для тех expressions, что содержат `await`. Transform (`template_rewrites.rs`) переписывается на лукап по NodeId вместо span. Текущая offset-таблица удаляется.

25. **`ProxyStateInits`, `has_class_state_fields`, `has_store_member_mutations`** удаляются полностью. У `ProxyStateInits` нет внешних потребителей. Остальные были derivation steps для RuntimePlan — пересчитываются inline в `RuntimePlanBuilder`.

26. **`OutputPlanData`** расщепляется: `RuntimePlan` (расширенный — см. Decision 47) и `CssAnalysis` становятся top-level полями `AnalysisData`; `IgnoreData` уезжает в `svelte_check` (приватно), runtime-влияющие ignore-коды поднимаются как поля payload (см. Decision 48); `needs_context`, `needs_sanitized_legacy_slots`, `needs_component_bind_ownership` — inline в `RuntimePlanBuilder`; `custom_element_*` уходят в `CustomElementInfo`; `component_name` — top-level поле.

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

39. **Порядок миграции.** `Block (доделать DebugTag/HtmlTag)` → `Expression` → `Attribute` → `Element` → `FragmentSemantics` (после Element, потому что FragmentSemantics упрощает codegen `prepare`/`FragmentCtx` уже в условиях, когда Element/Attribute читаются через новый API). После — расширение `RuntimePlanBuilder` (`ParamsShape` / `EpilogueKind` / bitflags / `StoresSetup`) и параллельно — приватизация `IgnoreData` с поднятием runtime-влияющих ignore-кодов как полей payload. Затем создаётся `svelte_check` крейт, validation переезжает (включая полный `IgnoreData`-сканер). Затем удаляются осиротевшие passes. Последним удаляется DAG.

40. **Каждая миграционная единица фиксируется в `debt.md`.** Между шагами в коде живут две формы (старая + новая) — это технический долг, в `debt.md` фиксируется отдельной строкой на каждый кластер. Строка удаляется, когда старая форма удалена.

### Concurrency

41. **Within-component concurrency не закладывается.** Builder-ы single-threaded. Causes: типичный component = ≤ нескольких сотен AST-узлов, overhead thread spawn / sync доминирует над выигрышем.

42. **Builder-ы остаются `Send` и pure** (input → output, без shared mutable state помимо своего выхода). Это сохраняет потенциал на точечный `rayon::join` (например, CSS pipeline параллельно с Phase 4 кластерами) если профайлинг покажет узкое место.

43. **Compiler-level parallelism (N-files in parallel) — out of scope этого PRD.** Это работа `svelte_compiler` / `napi_compiler` / `wasm_compiler`-оркестраторов, не analyze.

### Документация

44. **`SEMANTIC_LAYER_ARCHITECTURE.md` обновляется.** Правки: раздел про DebugTag / HtmlTag (out-of-scope → in-scope как варианты `BlockSemantics`); добавление `ExpressionSemantics` как пятого кластера; перевод validation в отдельный crate (Cluster builders диагностики не эмитят).

45. **`debt.md` получает новые строки.** Каждая удаляемая side-table или pass — строка с описанием состояния миграции и точкой удаления. Строка про parent-link в svelte_ast — отдельный долгосрочный пункт ("AST должен хранить parent_id и parent_fragment_id, после этого `TemplateTopology` исчезает").

46. **`CODEBASE_MAP.md` обновляется** по мере появления новых cluster-ов и удаления passes.

### `RuntimePlan` модели

47. **`RuntimePlan` форма — enum/bitflags/Option-struct, не параллельные bool-ы.** Codegen `lib.rs::generate` сейчас собирает component-функцию из ИЛИ-цепочек 4-source bool-композиций и AST-walks. Финальная форма `RuntimePlan` следующая:
    - `fn_params: ParamsShape::AnchorOnly | AnchorAndProps` — сводный enum для `(needs_props_param || has_bubble_events || has_legacy_slots || needs_sanitized_legacy_slots)`. `has_bubble_events` и `has_legacy_slots` остаются приватными intermediates `RuntimePlanBuilder`-а: их единственный потребитель — формула `ParamsShape`, и они не попадают в публичный API.
    - `epilogue: EpilogueKind { Nothing, Cleanup, PopExpression, PopAndCleanup, PopWithReturn, PopWithReturnAndCleanup }` — единый enum для матрицы `(needs_push, needs_pop_with_return, has_stores)`. Невалидные комбинации делает невозможными по типу.
    - `bare_imports: BareImportsFlags { ASYNC, LEGACY, TRACING, DEV_FILENAME }` — bitflags для prelude-импортов.
    - `exports_flags: ExportsFlags { HAS_EXPORTS, HAS_CE_PROPS, HAS_LEGACY_ACCESSOR_PROPS }` — bitflags. Сводный `has_explicit_exports = !flags.is_empty()` — derive-метод.
    - `stores_setup: Option<StoresSetup { bindings: Vec<EnrichedStoreBinding> }>`. `EnrichedStoreBinding { base_symbol, store_symbol, base_via_legacy_state }` — per-store enriched факт. `iter_store_bindings()` возвращает уже enriched `StoreBinding`, codegen не делает повторный `binding_semantics(base_symbol)`-лукап в loop-е.
    - `needs_ownership_validator: bool` — orthogonal feature, остаётся plain bool. Решение «нужен ли validator» принимается analyzer-ом (а не транзит-ИЛИ из transform-output + analyzer-fact как сейчас).
    - `delegated_events` НЕ поле `RuntimePlan`. Это codegen-state-аккумулятор по факту emit-а событий — не «решение», просто bookkeeping. Перенос в analyzer создал бы дублирующий event-walk без пользы.

### IgnoreData boundary

48. **Runtime-влияющие ignore-коды поднимаются как поля payload, `IgnoreData` приватна для `svelte_check`.** Текущий список таких кодов:
    - `hydration_html_changed` → поле `hydration_html_changed_ignored: bool` в `BlockSemantics::HtmlTag` payload (codegen `emit_html_tag` читает это поле).
    - `await_reactivity_loss` → поле `await_reactivity_loss_ignored: FxHashSet<OxcNodeId>` в `BlockerData` (transform `enter_for_of_statement` читает это множество). `BlockerData` — естественное место, потому что концепт async-barrier-ов уже там.

    Правило: каждый новый ignore-код, влияющий на shape JS-output, проходит явное добавление поля в соответствующий payload. Никакого `view.is_ignored(id, "literal_code_string")` в codegen / transform не остаётся.

### Transform single-match consumer

49. **Transform — single-match consumer `ReactivitySemantics` + AST-shape, без classification AST-walks.** Конкретно:
    - **Rune-classification по AST-узлу** идёт через `data.reactivity.rune_calls().kind(oxc_node_id)`. Этого хватает для: class-state-fields scan, `this.x = $state(...)` в конструкторах, rune-инициализаторов в declarators, `$inspect`-call recognition. Никаких AST-walks с `rune_kind_from_expr`-помощниками внутри transform-а не остаётся.
    - **Per-reference rewrite** идёт через `data.reactivity.reference_semantics(ref_id)` — как сейчас.
    - **AST-shape decisions** (assign vs assign_async по `is_expression_async(right)`, `wrap_binary_equals_dev` по форме `BinaryExpression with == or ===`, `console.log` rewrite) делаются transform-ом single-match на одном узле без предсчёта в analyzer. Эти решения не композируют ≥2 источника, и предсчёт не даёт выгоды.
    - **Backing-private-имена** для class-state-fields (`#count` / `#_count` если коллизия) генерируются transform-state-side; analyzer не генерирует JS-идентификаторы.
    - Никакого нового поля `ScriptTargets` / `ScriptRewritePlan` в `AnalysisData` не вводится. Текущего публичного API `ReactivitySemantics` (с расширением Decision 13: `rune_calls()` accessor) достаточно для всех transform-сценариев.

### Cluster precompute правило

50. **Правило пред-вычисления:** cluster builder пред-вычисляет факт в payload **только если потребитель иначе композирует ≥2 источника или повторяет ту же derive-логику в ≥2 местах**. Single-field / single-match derive (например `matches!(sem.flavor, EachFlavor::BindGroup)`) остаётся в codegen — это форматирование, не «факт». Имена JS-идентификаторов (`$$index_3`, `$$array_2`) генерирует codegen — это форма output, не семантика. Конкретный пример (EachBlock): добавляется `EachBlockSemantics::collection_store: Option<EachCollectionStore { store_symbol, item_member_mutated: bool }>` (composition `collection_store.is_some() && scoping().is_member_mutated(item_sym)` уезжает в analyzer); `needs_render_index` / `needs_group_index` остаются в codegen как single-match derive.

### FragmentSemantics

51. **`FragmentSemanticsStore` — utility store, не «кластер per Svelte kind».** Хранит per-fragment context-bundle, накапливаемый сверху-вниз по template-walk-у. Поля:
    - `preserve_ws: bool` (composition `preserve_whitespace || inside_pre || inside_textarea || inside_script` родителя).
    - `inside_pre: bool`, `inside_textarea: bool`, `inside_script: bool`, `inside_head: bool` — стек-флаги.
    - `namespace: Namespace` — текущий namespace (Html / Svg / MathML).
    - `parent_tag: Option<&str>` — имя родительского HTML-тега (для специальных правил, например `parent="pre"` whitespace-strip).
    - `needs_text_first_next: bool` — derive из `FragmentRole`.

    Codegen `prepare.rs` остаётся в codegen (в Q1-обсуждении подтверждено: prepare/hoist эффективны за один проход, переносить целиком нет смысла), но FragmentSemantics-store избавляет codegen-prepare-pipeline от протаскивания этих флагов через рекурсивный `FragmentCtx` — codegen читает один лукап на каждый fragment-вход. Migration FragmentSemantics — последний шаг по Decision 39, после Element-кластера.

### Element / Attribute / Expression payload mapping

52. **Exhaustive mapping helper-ов в payload-поля.** Все существующие helper-методы `Ctx`/`AnalysisData` (с префиксами `is_` / `has_` / `needs_` / `class_` / `static_` / `attr_` / `bind_` / `expr_` / `directive_`) растворяются в payload-поля соответствующего кластера. Текущий снапшот mapping-а (информативный, не authoritative — обновляется по мере migration; authoritative-правило ниже):

    Element (`ElementSemantics::Html`): `slot_target`, `wrapped_in_css_container` (для `Component`-варианта), `is_customizable_select`, `needs_textarea_value_lowering`, `is_bound_contenteditable`, `needs_var`, `needs_input_defaults`, `is_void`, `is_custom_element`, `option_synthetic_value_expr`, `creation_namespace`, `is_css_scoped`, `class_needs_state`, `has_class_attribute`, `has_class_directives`, `has_style_directives`, `has_spread`, `has_use_directive`, `has_bind_group`, `has_attribute_named(...)`, `static_class`, `static_style`, `is_dynamic` (per-element). Element (`Component`-варианты): `is_dynamic_component`, `has_component_css_props`, `component_needs_legacy_props_marker`, `component_binding_sym`, `component_snippets`.

    Attribute: `attr_is_function`, `attr_is_import`, `bind_target_semantics`, `bind_blockers`, `bind_group_value_attr`, `bind_each_context`, `class_directive_info`, `event_handler_mode`, `directive_root_reference_semantics`, `static_text_of`, `style_directives`.

    Expression (`ExpressionSemantics`): `expr_deps`, `expr_has_await`, `expr_has_blockers`, `needs_expr_memoization`, `needs_clsx`, `is_dynamic_attr`, `is_each_index_sym`, `is_expression_shorthand`, `legacy_wrap` (composition `runes() + needs_legacy_coarse_wrap + uses_legacy_sanitized_props`), `async_kind`, `role`, `references`, `is_pickled_await`.

    **Authoritative-правило (acceptance gate):** после миграции каждого кластера на `Ctx`/`AnalysisData` НЕТ helper-методов с указанными префиксами; каждый такой query — поле в payload соответствующего кластера. Acceptance-чек: `grep -E "fn (is_|has_|needs_|class_|static_|attr_|bind_|expr_|directive_)[a-z_]+" crates/svelte_codegen_client/src/context.rs` возвращает 0 строк после финальной миграции. Mapping выше — снапшот PRD-time, может устареть; новые helper-ы, появляющиеся в codegen в течение migration, проходят то же правило.

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
- `ScriptDeclarationsBuilder`, `BlockerDataBuilder`, `CustomElementInfoBuilder`, `TemplateIndicesBuilder`, `FragmentSemanticsBuilder`, `RuntimePlanBuilder`, `CssAnalysisBuilder` — узкие builder-ы, покрываются end-to-end тестами компилятора. Если профайлинг или баг покажет необходимость — точечный юнит добавляется. Расширение `RuntimePlan` (`ParamsShape`, `EpilogueKind`, bitflags, `StoresSetup`) проверяется снапшот-тестами компилятора: переход с ИЛИ-цепочек / AST-walks в codegen на готовые поля payload не должен менять JS-output.

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
- `AnalysisData` имеет ровно перечисленные в Solution 15 top-level полей (включая `fragments`), без legacy
- 0 ad-hoc helper-методов на `AnalysisData` сверх accessor-ов к этим полям
- 0 helper-методов с префиксами `is_` / `has_` / `needs_` / `class_` / `static_` / `attr_` / `bind_` / `expr_` / `directive_` на `Ctx`/`AnalysisData` (Decision 52)
- 0 чтений `IgnoreData` (как типа) из `svelte_codegen_client` / `svelte_transform`. `IgnoreData` доступна только внутри `svelte_check`. Decision 48
- 0 случаев `view.is_ignored(id, "code-string")` в codegen / transform; runtime-влияющие ignore-коды читаются как поля payload (`hydration_html_changed_ignored`, `await_reactivity_loss_ignored`)
- 0 AST-classification walks в `svelte_transform` для rune-распознавания; rune-classification идёт исключительно через `data.reactivity.rune_calls()` (Decision 49)
- 0 ИЛИ-цепочек в `svelte_codegen_client/src/lib.rs::generate` для решений уровня "shape компонент-функции"; вместо них — матч по `RuntimePlan::fn_params` / `epilogue` / bitflags / `stores_setup` (Decision 47)
- 0 AST-walks в codegen за component-level фактами (`has_bubble_events`, `has_legacy_slots` определялись через walk root fragment / all nodes — теперь приватные intermediates `RuntimePlanBuilder`-а, в публичный API не попадают)
- `crates/svelte_analyze/src/validate/` каталог удалён, `crates/svelte_check/` существует и содержит все правила (включая `IgnoreData`-сканер и `warning_filter`)
- `just test-compiler && just test-diagnostics && just clippy-strict` зелёные

## Out of Scope

1. **Рефакторинг `svelte_ast`.** Parent-link / parent-fragment-id в `Node` — отдельный долгосрочный пункт `debt.md`. После него `TemplateTopology` исчезает, но это работа уровня parser-а / AST крейта, не analyze.

2. **Compiler-level concurrency.** Параллельный analyze N компонентов через rayon — задача `svelte_compiler` / `napi_compiler` / `wasm_compiler`-оркестратора. Этот PRD только обеспечивает, что один analyze-вызов остаётся `Send + чисто функциональным`.

3. **Within-component concurrency.** Точечный `rayon::join` (например, CSS pipeline параллельно с Phase 4) — потенциально в будущем по результатам профайлинга. Не закладывается архитектурно в этом PRD.

4. **Property-based / fuzz тесты на cluster builder-ы.** End-to-end snapshot и unit-тесты — достаточны для acceptance этого PRD.

5. **Изменение публичного API `svelte_diagnostics`.** Checker-крейт использует существующие `Diagnostic` / `Severity` типы.

6. **Миграция transform-а как самостоятельный документ.** Transform работает per-reference через `ReactivitySemantics` и не классифицирует expression-узлы как единицы. Изменения transform-крейта в рамках этого PRD узкие: (a) точечный rewrite-а `is_pickled_await(span.start)` → лукап по NodeId (decision 24); (b) переход на `data.reactivity.rune_calls()` для rune-classification вместо локальных AST-walks (decision 49); (c) чтение `await_reactivity_loss_ignored` множества из `BlockerData` вместо обращения к `IgnoreData` по строковому коду (decision 48). Не входит в этот PRD: ScriptTargets-store как отдельное поле `AnalysisData` — решено, что не нужен (текущего публичного API `ReactivitySemantics` достаточно). Decisions уровня AST-shape (assign vs assign_async, dev wraps, console.log rewrite) делаются transform-ом single-match без предсчёта.

7. **Полный перенос codegen `prepare`/`hoist` в analyzer.** Рассматривался; отвергнут. `prepare.rs` и hoisting эффективны за один проход в codegen, перенос целиком даёт перенос работы без сокращения. Анализатор пред-вычисляет узкие факты, помогающие codegen-prepare-pipeline-у (slot_target, wrapped_in_css_container, FragmentSemantics — context-bundle), но walk фрагмента остаётся в codegen.

8. **Pre-compute `delegated_events` set в analyzer.** Рассматривался; отвергнут. Это codegen-state-аккумулятор по факту emit-а событий — не «решение», просто bookkeeping. Перенос в analyzer создал бы дублирующий event-walk без выгоды для consumer-а.

9. **Изменение CSS preprocessor / output формата.** `CssAnalysisBuilder` поглощает существующие 3 css-pass-а без изменения CSS-семантики или output-формата.

10. **Property тесты на фрагменты codegen-а.** Codegen меняется только в части переключения с deprecated AnalysisData helper-ов на cluster API; behavior сохраняется (проверяется существующими `compiler_tests/cases2/`).

11. **Изменение представления async (separate cluster).** Async-поля живут в payload каждого кластера. Это уже зафиксировано в `SEMANTIC_LAYER_ARCHITECTURE.md` и не пересматривается в этом PRD.

12. **Удаление `ComponentSemantics` крейта.** Crate `svelte_component_semantics` остаётся как есть. Изменения только в обёртке `ComponentScoping` в analyze (она растворяется).

13. **Per-text `ws_only` pre-compute.** Рассматривался; отвергнут. Whitespace trimming делается posicionно (is_first/is_last/prev_is_expr/next_is_expr) — расчёт зависит от соседей, и сводный per-text bool не сокращает работу codegen-prepare-а.

14. **Группировка template-side полей `AnalysisData` под `template:` подструктуру.** Рассматривался; отвергнут. Текущий плоский layout с 15 полями удобнее для destructure (`let { elements, topology, fragments } = analysis;`) и не нагружает доступ дополнительной ступенью.

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
- Размер `types/data/` в analyze: 18 side-tables → cluster store-структуры + top-level факты в финальной форме `AnalysisData` (15 полей, см. Solution).
- Количество ad-hoc helper-методов на `AnalysisData`: 70 → 0 (только accessor-ы к 15 полям).
- Количество helper-методов с префиксами `is_`/`has_`/`needs_`/`class_`/`static_`/`attr_`/`bind_`/`expr_`/`directive_` на `Ctx`: текущая base-line → 0 (Decision 52).
- Количество `&&`-цепочек, читающих ≥2 стора, в `svelte_codegen_client`: текущая base-line → 0.
- Количество ИЛИ-цепочек ≥3 источников в `svelte_codegen_client/src/lib.rs::generate` (`fn_params`, exports composition, prelude imports): текущая base-line → 0 (Decision 47).
- Количество AST-walks в codegen за component-level фактами (`has_bubble_events`, `has_legacy_slots`): 2 → 0 (поглощены `RuntimePlanBuilder`).
- Количество AST-walks в transform за rune-classification: текущая base-line → 0 (Decision 49; всё через `data.reactivity.rune_calls()`).
- Количество `view.is_ignored(id, "code-string")`-вызовов в codegen + transform: текущая base-line (2: hydration_html_changed, await_reactivity_loss) → 0 (Decision 48).
- Количество template walks в analyze: текущая (≥6) → ≤ 6 (по одному на cluster Phase 4 + один на TemplateIndices + один на FragmentSemantics в Phase 3).

### Риски

1. **Большие cluster payload-ы.** Thick payload приводит к крупным enum-вариантам с много полей. Компенсация: каждый variant отвечает на конкретный consumer-сайт, поля документируются inline в коде. При признаках «слишком большой variant» — кандидат на расщепление variant-а или на explicit `manual exception` от Thick правила.
2. **Long-running migration.** Между шагами в коде живут две формы. Защита: `#[deprecated]` warning + `debt.md` строки + acceptance gate на тесты + правило «удалить старое до закрытия cluster-миграции».
3. **Boundary `svelte_check`.** Чтобы checker мог читать всё нужное через публичный API без `pub(crate)` хаков, придётся явно расширить публичный API analyze. Это может потребовать дополнительных аксессоров, что увеличит surface. Защита: каждый новый pub-метод проходит ревью.
4. **`PickledAwaitOffsets` миграция transform-а.** Transform лукапит по `span.start`, что требует пере-привязки на NodeId. Узкая правка в `transform/template_rewrites.rs`, не блокирующая, но требующая внимательности.
5. **Удаление `ComponentScoping`.** Обёртка имеет помимо delegation-методов несколько кэшей и helper-ов. Каждый — point-by-point переезжает либо в `ComponentSemantics`, либо в `TemplateTopology`. Возможны регрессии перформанса если кэш был критичный.
6. **Расширение `RuntimePlan` (Decision 47) затрагивает всю сборку компонент-функции в codegen `lib.rs::generate`.** Migration этого шага — массивный refactor одного файла, могущий ломать несвязанные тесты на промежуточных состояниях. Защита: миграция атомарным шагом (один builder-расширение + один codegen-переход), с обязательным green `compiler_tests/cases2` перед merge.
7. **Появление нового runtime-влияющего ignore-кода в reference-компиляторе.** Если в Svelte upstream появится новый `<!-- svelte-ignore X -->`, влияющий на shape JS-output, потребуется добавить новое поле в payload по Decision 48. Защита: правило фиксируется в Decision 48, review-чек ловит попытки добавить `is_ignored(id, "X")` в codegen / transform.

### Ссылки

- `CLAUDE.md` — догмы и глоссарий.
- `ARCHITECTURE.md` — инварианты по крейтам.
- `CODEBASE_MAP.md` — текущая карта типов.
- `SEMANTIC_LAYER_ARCHITECTURE.md` — рабочий документ semantic layer (обновляется при принятии PRD).
- `debt.md` — известный технический долг (получает новые строки на каждый умирающий side-table).
- `ROADMAP.md` — статус портирования.
- `crates/svelte_codegen_client/src/template/each_block.rs` — reference implementation для consumer-side decision composition (`gen_each_block` / `EachPlan`).
