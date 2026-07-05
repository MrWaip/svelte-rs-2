label: analyze-js-visitor-bundle

# PRD: analyze-js-visitor-bundle — один обход JS-AST на пасс, N визиторов

Дочерний дизайн слоя `analyze` (`analyze.md`). Родительский инвариант — **build order как DAG по
зависимостям данных** и **валидация (3.C) read-only после подсистем** — остаётся неприкосновенным.
Оригинала для этого нет: это внутренняя оптимизация архитектуры, не вопрос парити выходного JS.

## Проблема

Слой анализа обходит **OXC-дерево** (instance `<script>` + module `<script>`) многократно: каждый
`impl oxc_ast_visit::Visit` заводит свой полный `visit_program`. В `svelte_analyze` — 38 таких impl,
и только стадия `VALIDATION_STAGE` прогоняет по instance-программе ~15 независимых обходов.

Профиль подтверждает: `oxc_ast_visit::generated::visit::walk::walk_expression` (6.08% агрегата,
до 10.9% в template-heavy кейсах) + `Visit::visit_expression` (3.48%) — это стоимость самого
диспетчера обхода, умноженная на число проходов. Каждый лишний проход платит за `walk_expression` /
`walk_statement` заново.

Драйверы из профиля — `RuneValidator`, `StateRefLocallyValidator`, `ScriptSemanticCollector`,
`CollectSymbolsVisitor`, `expression_semantics::builder::walker::compute`,
`TemplateBuildContext::visit_js_expression`.

## Ограничение (жёсткое)

**Пассы и подсистемы не сливаются.** `ComponentSemantics`, `ReactivitySemantics`,
`ExpressionSemantics`, валидация — остаются отдельными пассами с текущим build order и границами
данных. Сливаются только **обходы AST внутри одной фазы/пасса**, не имеющие взаимной зависимости по
данным. Единица слияния — traversal, не пасс.

## Уже существующий образец — template-сторона

Обходы **template-AST** уже слиты, и механизм ровно тот, что нужен:

- `walker::TemplateVisitor` (`walker/visitor.rs`) — проектный fan-out trait: набор `visit_*`-хуков с
  пустыми дефолтами.
- `walker::walk_template` + `walker::dispatch` — один обход дерева; на каждом узле
  `for v in visitors.iter_mut() { v.visit_X(...) }` раздаёт событие всем визиторам среза
  `&mut [&mut dyn TemplateVisitor; N]`.
- `passes::bundles::*Bundle` — пасс собирает bundle, берёт `bundle.visitors()` и вызывает
  `run_parsed_template_bundle` (`executor.rs:12`), который делает **один** `walk_template` на N
  визиторов.

Так работают пассы `TemplateSideTables`, `CollectSymbols`, `TemplateClassificationWalk`,
`ValidateTemplate` — каждый прогоняет свой набор `TemplateVisitor` за один обход шаблона.

**Симметрия отсутствует на JS-стороне.** Для OXC `Program` fan-out-обёртки нет: каждый визитор зовёт
собственный `visit_program`. Дизайн — перенести тот же паттерн на JS-обход.

## Целевая форма

Ввести JS-аналог template-механизма:

1. **`JsVisitor`** — проектный fan-out trait (по образцу `TemplateVisitor`), но над OXC-узлами. Хуки
   только на те виды узлов, что реально нужны потребителям (union текущих валидаторов): как минимум
   `visit_call_expression`, `visit_identifier_reference`, `visit_member_expression`,
   `visit_declaration` / `visit_variable_declarator`, `visit_new_expression`, `visit_function` /
   `visit_arrow` (для отслеживания function-depth), плюс `enter/leave`-пары там, где нужен контекст
   вложенности. Все хуки — с пустым дефолтом.

2. **`walk_program(program, &mut [&mut dyn JsVisitor])`** — единственный драйвер обхода OXC-дерева.
   Внутри — один `oxc_ast_visit::Visit`-impl (fan-out-адаптер), который на каждом релевантном узле
   раздаёт событие всем `JsVisitor` среза и ровно один раз рекурсивно спускается (`walk_*`). Контекст
   (function-depth, scope-flags, runes-флаг, ссылка на `AnalysisData`) держит сам драйвер и отдаёт
   визиторам — аналог `VisitContext`.

3. **Bundles по пассу.** Пасс собирает bundle своих `JsVisitor` и делает один `walk_program`.
   Первый и главный — `JsValidationBundle` для пасса `Validate`.

Форма 1:1 повторяет template-сторону, поэтому в архитектуру ничего нового не вносит — только
устраняет асимметрию.

## Scope по фазам

### Фаза 1 (первичная) — пасс `Validate`

`validate::validate` (`validate/mod.rs`) сейчас внутри одного пасса запускает независимые полные
обходы instance-программы: `legacy::validate_legacy_diagnostics`, `runes::validate`,
`stores::validate`, `PerfClassWarningValidator`, `experimental_async::validate_instance_program`,
`class_state_fields::validate`, `typescript::validate`, `non_reactive_update::validate`. Все —
read-only над `AnalysisData` (уже построена) и над `program`, все эмитят в `diags`, взаимных
зависимостей по данным нет → это листья одной фазы, законно сливаются.

Переписать каждый из них как `impl JsVisitor` и прогнать одним `walk_program` через
`JsValidationBundle`. То же самое отдельно для `module_program`. Пасс `Validate` **остаётся одним
пассом** на прежнем месте в `VALIDATION_STAGE` — меняется только число обходов внутри него: ~15 → 1
(instance) + 1 (module).

### Фаза 2 — множественные валидаторы рун

`runes::validate` сам по себе разворачивается в несколько полных обходов (`RuneValidator`,
`StateRefLocallyValidator`, `RestPropAccessValidator`, `ConstTagRuneProbe` — `visit_program` на
строках 87/143/397/978). Это обходы внутри одной подсистемы валидации → их `JsVisitor`-формы кладутся
в тот же `JsValidationBundle`, что и Фаза 1.

### Фаза 3 (опционально, позже) — обходы внутри билдер-пассов и embedded-JS

- Билдер-пассы (`BuildReactivitySemantics`/`build_v2` и др.) внутри себя тоже делают несколько
  JS-обходов; те, что внутри **одного** пасса и без взаимной зависимости, — кандидаты на тот же
  bundle. Пассы между собой при этом **не** сливаются (build order сохраняется).
- Template-embedded JS: `dispatch.rs:16` отдаёт `visit_js_expression(id, expr, ctx)` целым
  выражением — рекурсии внутрь OXC-поддерева нет, поэтому каждый заинтересованный `TemplateVisitor`
  обходит одно и то же выражение сам. Точка расширения `JsVisitor`-механизма на embedded-выражения.

Фаза 3 не входит в первичную поставку и оценивается отдельно после замера Фаз 1–2.

## Инварианты, которые дизайн обязан сохранить

1. **Пассы/подсистемы не сливаются.** Ключи пассов, стадии, build order (`passes/mod.rs`) не
   меняются. Меняется только тело отдельных пассов (число обходов внутри).
2. **Read-only.** `JsVisitor` не мутируют ни AST, ни подсистемы — как и текущие валидаторы (догма
   3.C).
3. **Парити диагностик по составу.** Тесты диагностик сортируют обе стороны по
   `(severity, code, start, end)` и сравнивают (`test_diagnostics.rs:106`), поэтому **порядок эмита
   не важен** — важен идентичный *набор* диагностик.
4. **Не пере-выводить факты подсистем.** `JsVisitor`-валидаторы читают `AnalysisData`, как сейчас; их
   логика на узле дословно переносится, не переписывается.

## Риск: связанность по `span_already_taken`

Единственная межвалидаторная зависимость от порядка — `span_already_taken(diags, span)`
(`validate/mod.rs:111`, единственное использование — `runes.rs:379`): валидатор смотрит, не занят ли
спан диагностикой, эмитнутой **другим** валидатором ранее. При последовательных полных обходах «ранее»
= «в предыдущем полном проходе»; при слитом обходе интерливинг меняется, и результат проверки может
измениться.

Мутация: убрать инлайн-зависимость от интерливинга — арбитраж спанов вынести в **пост-walk шаг**
реконсиляции над собранным `diags` (dedup по спану с фиксированным precedence валидаторов), а не
проверять «занятость» во время обхода. Это делает набор диагностик инвариантным к порядку и снимает
единственную преграду слияния. Precedence при этом фиксируется явно (порядок в bundle / таблица
приоритетов), а не неявным порядком пассов.

## Non-goals

- Не менять выходной JS и набор диагностик (только их производство).
- Не трогать template-сторону (уже слита) кроме опциональной Фазы 3.
- Не сливать пассы и подсистемы, не менять build order.
- Не вводить параллелизм — цель — убрать повторные обходы, не распараллелить их.

## Ожидаемый эффект

Стадия валидации перестаёт быть N-кратным обходом instance-программы. Прямая цель — срезать долю
`walk_expression`/`walk_statement`/`visit_expression`, приходящуюся на валидацию (основной вклад в
~9.5% агрегата этих функций). Точная величина — по замеру после Фазы 1 на драйвер-кейсе
`template_case_02` (`just bench-flame …/template/case_02.svelte`).

## Связь с другими документами

- `analyze.md` — §Build order, §Валидация (3.C): инварианты, которые дизайн сохраняет.
- `component-semantics.md`, `reactivity-semantics.md`, `expression-semantics.md` — подсистемы,
  чьи билдер-обходы затрагивает опциональная Фаза 3 (читать при заходе в неё).
- `walker/visitor.rs`, `walker/dispatch.rs`, `walker/traverse.rs`, `passes/bundles.rs`,
  `passes/executor.rs` — образец fan-out-паттерна, который переносится на JS-сторону.
