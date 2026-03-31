# SIMPLIFY_REVIEW

Единый документ по упрощению архитектуры `analyze → transform → codegen`, отсортированный по долгосрочному эффекту.

## Audit coverage

Покрытие ревью:
- `crates/svelte_analyze/src` (31 files)
- `crates/svelte_transform/src` (3 files)
- `crates/svelte_codegen_client/src` (33 files)

Итого: **67 файлов**, ~**21,225 LOC**.

---

## Prioritized simplifications (sorted by effect)

### 1) Typed pass graph for `svelte_analyze` (maximum effect)

**Problem**
- Порядок пассов и их зависимости зашиты вручную в orchestration-функции.
- Инварианты между пассами выражены комментариями, а не контрактами.

**Change**
- Ввести декларативные pass-descriptor’ы: `requires` / `produces`.
- Собирать порядок выполнения из dependency graph.

**Primary targets**
- `crates/svelte_analyze/src/lib.rs`
- `crates/svelte_analyze/src/passes/mod.rs`

**Expected effect**
- Самое сильное снижение регрессий и стоимости портирования новых фич.

---

### 2) Split codegen `Ctx` into immutable `CodegenQuery` + mutable `CodegenState`

**Problem**
- `Ctx` одновременно: AST/analysis query facade + mutable emission state + async/event plumbing.
- Высокая связность и тяжёлый cognitive load.

**Change**
- `CodegenQuery<'a>`: только read-only semantic questions.
- `CodegenState<'a>`: только mutable generation state.
- Переходно оставить thin `Ctx`-wrapper.

**Primary targets**
- `crates/svelte_codegen_client/src/context.rs`
- `crates/svelte_codegen_client/src/template/*.rs`

**Expected effect**
- Значительное упрощение codegen-модулей и более дешёвые future-refactor’ы.

---

### 3) Stabilize analyze→codegen contract via `CodegenView`

**Problem**
- Codegen использует смешанный стиль API: часть через прокси, часть через прямой `ctx.analysis.*`.
- Внутренняя раскладка `AnalysisData` протекает в downstream.

**Change**
- В analyze добавить единый контракт `CodegenView` для downstream-запросов.
- Считать nested tables implementation detail analyze.

**Primary targets**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/context.rs`

**Expected effect**
- Сильное снижение coupling между фазами и churn при изменении data-модели.

---

### 4) Consolidate render-tag data into single `RenderTagPlan`

**Problem**
- Render-tag знания распылены по нескольким side tables (`mode`, `arg_infos`, `prop_sources`).
- Codegen должен синхронизировать их на лету.

**Change**
- Один `NodeTable<RenderTagPlan>` со всем необходимым для эмиссии.

**Primary targets**
- analyze render-tag passes
- `crates/svelte_codegen_client/src/template/render_tag.rs`

**Expected effect**
- Устранение класса ошибок рассинхронизации таблиц.

---

### 5) Replace offset plumbing with typed expression handles

**Problem**
- `node_expr_offsets/attr_expr_offsets` + `span.start` leaking в transform/codegen.
- Это технический долг API-границы parser↔analyze↔codegen.

**Change**
- Typed handles (`ExprHandle`, `StmtHandle`) вместо raw offsets в downstream.

**Primary targets**
- parser types / `ParserResult`
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_transform/src/lib.rs`
- `crates/svelte_codegen_client/src/context.rs`

**Expected effect**
- Существенно более понятный и менее хрупкий inter-phase API.

---

### 6) Unified expression-dependencies API (`ExprDeps`) for Node/Attr

**Problem**
- Дублирование API для blockers/memo/await между node-expression и attr-expression путями.

**Change**
- Ввести единый query по `ExprSite` (`Node`/`Attr`) с нормализованным `ExprDeps`.

**Primary targets**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/template/{if_block,each_block,events,attributes,expression}.rs`

**Expected effect**
- Меньше duplicated logic в template codegen и проще глобально менять async/memo policy.

---

### 7) Move runtime prelude decisions to analyze (`RuntimePlan`)

**Problem**
- Codegen recompute’ит aggregate-флаги (`needs_push`, exports/bindable/stores/ce-props).
- Это cross-phase recombination, которую лучше предвычислять в analyze.

**Change**
- Добавить `RuntimePlan` в analyze и потреблять в codegen как готовое решение.

**Primary targets**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/lib.rs`

**Expected effect**
- Упрощение entry codegen path + единый источник истины для runtime wiring.

---

### 8) Centralize scope fallback policy in transform (`TemplateScopeResolver`)

**Problem**
- Повторяющиеся `unwrap_or(scope)` для child-scope резолва по многим node-kind’ам.

**Change**
- Одна сущность `TemplateScopeResolver` с явными методами по типам child-секции.

**Primary targets**
- `crates/svelte_transform/src/lib.rs`

**Expected effect**
- Снижение вероятности subtle scope bugs и проще поддержка новых node-kind’ов.

---

### 9) Pass-bundle API for analyze multi-visitor walks

**Problem**
- В одном месте вручную собираются наборы visitor’ов, зависимости не типизированы.

**Change**
- Именованные bundles (`SemanticBundle`, `ReactivityBundle`, `ElementBundle`) + constructor checks.

**Primary targets**
- `crates/svelte_analyze/src/lib.rs`
- `crates/svelte_analyze/src/walker.rs`

**Expected effect**
- Более прозрачная архитектура анализатора и локализация ответственности.

---

### 10) Normalize async emission policy into explicit `AsyncEmissionPlan`

**Problem**
- В template модулях повторяется похожая схема: `has_await`, `needs_async`, thunk, wrapper.

**Change**
- Один унифицированный `AsyncEmissionPlan` и helper для эмиссии.

**Primary targets**
- `crates/svelte_codegen_client/src/template/*.rs`
- `crates/svelte_codegen_client/src/context.rs`

**Expected effect**
- Меньше divergence между блоками (`if/each/key/html/render/svelte_element`).

---

### 11) Pre-resolve const-tag `SymbolId` in analyze

**Problem**
- Codegen делает повторные string-based scope lookups для const-tag names.

**Change**
- В `ConstTagData` хранить рядом с именами resolved `Vec<SymbolId>`.

**Primary targets**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/template/const_tag.rs`

**Expected effect**
- Упрощение const-tag codegen и улучшение boundary hygiene.

---

### 12) Decompose selected mega-files by concern (lower but real effect)

**Problem**
- Очень крупные файлы затрудняют review/navigation.

**Change**
- Разбить по функциональным срезам без изменения поведения.

**Primary targets**
- `crates/svelte_codegen_client/src/builder.rs`
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/script/traverse.rs`
- `crates/svelte_analyze/src/passes/js_analyze.rs`

**Expected effect**
- Лучшая поддерживаемость и более быстрые ревью.

---

## Rollout order (pragmatic)

1. #1 + #2 + #3 (архитектурные safety rails)
2. #4 + #5 + #6 + #7 (контракт и data-flow упрощение)
3. #8 + #9 + #10 + #11 (локальная консолидация)
4. #12 (структурная декомпозиция)

## Notes
- Документ intentionally ориентирован на long-term benefit.
- No generated snapshots touched.

---

## Proven compiler practices from mature projects (and how to apply here)

Ниже не «общие советы», а практики, которые десятилетиями обкатаны в production-компиляторах (Rust/LLVM/Swift/TypeScript и др.) и обычно дают максимальный выигрыш в maintainability.

### A) Query-based architecture (incremental, memoized semantic queries)

**Почему это работает в зрелых компиляторах**
- Вместо «огромного пайплайна» система строится как граф запросов: каждый запрос имеет четкие входы/выходы и кэш.
- Это снижает связанность между фазами и упрощает инкрементальность.

**Как применить в svelte-rs**
- Эволюционно поверх `AnalysisData` ввести query-surface (`CodegenView` + typed pass outputs).
- На первом этапе кэш может быть тривиальным (in-memory на compile run), без сложной инкрементальности.

**Где начать**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_analyze/src/lib.rs`
- `crates/svelte_codegen_client/src/context.rs`

---

### B) Data-oriented execution plans (NOT a global IR)

**Почему это работает**
- Успешные компиляторы редко гоняют «сырые AST + флаги» до финального backend.
- Но это не обязательно означает единый монолитный IR: часто хватает целевых plan-структур с четкими инвариантами.

**Как применить**
- В этом проекте не нужен отдельный общий IR-слой.
- Нужны узкие typed plans в `AnalysisData` (например, `RenderTagPlan`, `ExprDeps`, `RuntimePlan`) как контракт для client codegen.
- Для SSR — отдельные SSR-oriented plans/queries, без требования совпадать с client-метаданными.

**Где начать**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/template/*.rs`

---


### IR position for this repo (client + SSR)

- **Не вводить единый кросс-таргет IR как цель сам по себе.**
- Держать общими только базовые семантические факты, где это действительно выгодно.
- Поверх них иметь **два target-specific projection слоя**:
  - client plans (DOM/runtime emission),
  - SSR plans (server rendering semantics).
- Если метаданные расходятся — это нормально; синхронизировать нужно инварианты поведения, а не форму структур.

### C) Make invalid states unrepresentable (enum-first + typestate)

**Почему это работает**
- В образцовых компиляторах огромная доля багов убирается типами, а не тестами.
- Enum/typestate отрезают невозможные комбинации на уровне компиляции.

**Как применить**
- Продолжить замену boolean-clusters на domain enums (`AsyncEmissionPlan`, unified deps/memo modes).
- Для фаз — использовать маркеры готовности (по аналогии с уже существующими marker-типами).

**Где начать**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/template/*.rs`
- `crates/svelte_analyze/src/types/markers.rs`

---

### D) Deterministic pipeline + golden tests as contract

**Почему это работает**
- Компиляторы высокого качества делают выход строго детерминированным.
- Snapshot/golden tests становятся «юридическим контрактом» поведения.

**Как применить**
- Для каждой архитектурной миграции (query/plan) делать behavior-preserving PR-цепочки.
- Добавить thin regression suites для точек риска: async-blockers, render-tag mode, const-tag symbol resolution.

**Где начать**
- `tasks/compiler_tests/cases2/*`
- `crates/svelte_codegen_client/src/template/*.rs`

---

### E) Diagnostics as first-class pipeline product

**Почему это работает**
- В зрелых системах диагностика проектируется как продукт архитектуры, не побочный эффект.
- Это стабилизирует refactor’ы: ошибки/варнинги остаются согласованными между фазами.

**Как применить**
- Привязать diagnostics к query/plan слоям, чтобы источник решения был явным.
- Убедиться, что suppress/ignore semantics не размазаны по нескольким местам.

**Где начать**
- `crates/svelte_analyze/src/walker.rs`
- `crates/svelte_analyze/src/validate/*`
- `crates/svelte_analyze/src/types/data.rs` (`ignore_data`)

---

### F) Explicit ownership boundaries (no semantic rediscovery downstream)

**Почему это работает**
- Лучшие компиляторы жестко держат ownership: semantic facts определяются upstream, downstream только применяет.

**Как применить**
- Зафиксировать правило: если codegen начинает «догадываться» о семантике, добавлять новый analysis-plan вместо локального workaround.
- Формализовать это в review checklist (в PR template/qa workflow).

**Где начать**
- `CLAUDE.md` boundary rules
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/context.rs`

---

### G) Compiler engineering practice: many small safe steps

**Почему это работает**
- Большие «rewrite PR» почти всегда ухудшают качество и тормозят команду.
- Зрелые проекты делают архитектурные миграции цепочкой маленьких PR с compatibility shims.

**Как применить (конкретно для этого проекта)**
1. Добавить query wrappers без изменения behavior.
2. Перевести 1 узкий path (например, `render_tag`) на новый план.
3. Удалить старые поля/ветки только после green snapshots.

**Где начать**
- `crates/svelte_codegen_client/src/template/render_tag.rs`
- `crates/svelte_analyze/src/types/data.rs`

---

## Practical recommendation

Если цель — «не повторить спагетти», наибольший ROI даст комбинация:
1. **#1 Typed pass graph**,
2. **#3 Stable CodegenView contract**,
3. **#4 RenderTagPlan consolidation**,
4. **#5 Typed handles вместо offsets**.

Это вместе дает тот самый «compiler-grade backbone»: явные зависимости, строгий контракт между фазами и меньше неявной логики в backend.

---

## Grouped execution plan (sorted by effect, with combinable workstreams)

Ниже — не просто приоритеты, а **сгруппированные пачки изменений**, которые имеет смысл делать вместе в одном потоке работ.

### Group A — Foundation contracts (Maximum effect)

**Includes:**
- #1 Typed pass graph
- #3 Stable `CodegenView` contract
- #2 Split `Ctx` into `CodegenQuery` + `CodegenState`

**Почему вместе**
- Это один системный контур: явные зависимости analyze + стабильный API к codegen + разделение read/write responsibilities.
- Если делать раздельно, будет много временных адаптеров и лишняя churn-нагрузка.

**Suggested order inside group**
1. `CodegenView` wrappers (без behavior change)
2. typed pass descriptors/graph
3. разделение `Ctx` (с temporary compatibility layer)

**Primary impact**
- Резкое снижение архитектурной связанности и «спагетти-эффекта».

---

### Group B — Data model normalization (Very high effect)

**Includes:**
- #4 `RenderTagPlan` consolidation
- #6 unified `ExprDeps`
- #7 `RuntimePlan`
- #11 pre-resolved const-tag `SymbolId`

**Почему вместе**
- Все 4 пункта про один и тот же принцип: **codegen должен читать готовые semantic plans, а не собирать их сам**.
- Дает максимальный выигрыш по чистоте контракта analyze→codegen.

**Suggested order inside group**
1. `RuntimePlan` (самый безопасный)
2. `RenderTagPlan`
3. unified `ExprDeps`
4. const-tag symbol pre-resolution

**Primary impact**
- Сильное уменьшение cross-phase recomputation и локальных workaround’ов.

---

### Group C — API hygiene + traversal safety (High effect)

**Includes:**
- #5 typed expression handles (вместо offsets)
- #8 `TemplateScopeResolver`
- #9 pass-bundle API

**Почему вместе**
- Это пакет про «безопасную механику»: меньше хрупких ключей, меньше copy-paste fallback logic, более явные блоки обходов.

**Suggested order inside group**
1. typed handles
2. `TemplateScopeResolver`
3. pass-bundles

**Primary impact**
- Существенно меньше accidental bugs в проходах и child-scope резолве.

---

### Group D — Local consolidation and maintainability (Medium effect)

**Includes:**
- #10 `AsyncEmissionPlan`
- #12 mega-file decomposition

**Почему вместе**
- После стабилизации контрактов эти изменения становятся дешевыми и безопасными.
- До стабилизации могут дать шумные диффы без системной пользы.

**Primary impact**
- Лучшая читабельность и скорость ревью/онбординга.

---

## TL;DR: best execution sequence

1. **Group A** (архитектурный каркас)
2. **Group B** (нормализация semantic plans)
3. **Group C** (механическая безопасность API/обходов)
4. **Group D** (локальная шлифовка)

Эта последовательность дает максимальный эффект раньше и минимизирует стоимость миграции.

---

## Implementation playbook (how to execute this in real PRs)

Ниже — практический план внедрения, чтобы roadmap не остался документом.

### 0) Execution rules (before any refactor PR)

1. **Behavior-preserving by default**: сначала контракты/структуры, потом удаление legacy-path.
2. **One architectural intent per PR**: не смешивать, например, `RenderTagPlan` и `typed handles` в одном PR.
3. **Golden-first safety**: любой шаг проходит через существующие snapshot/компиляторные тесты.
4. **Compatibility window**: временно держать старый и новый путь параллельно, пока не мигрированы все call-sites.

---

### Phase 1 — Lay foundation (Group A, 2–3 weeks)

**Goal**: стабилизировать границы и API между analyze/codegen.

**PR-1: Introduce `CodegenView` facade (no behavior change)**
- Добавить read-only facade поверх `AnalysisData`.
- Перевести только несколько безопасных вызовов в `codegen/lib.rs` на facade.
- Оставить старые поля/доступы нетронутыми.

**PR-2: Typed pass descriptors in analyze**
- Описать существующие пассы как descriptors (`requires/produces`).
- Выполнять в прежнем порядке, но уже через таблицу зависимостей.

**PR-3: Split `Ctx` internals (`Query` + `State`) with shim**
- Ввести новые структуры, но оставить backward-compatible методы `Ctx`.
- Мигрировать 1-2 template модуля как proof-of-safety.

**Exit criteria**
- Нет изменения snapshot output.
- В codegen меньше прямых обращений к `ctx.analysis.*` в целевых модулях.

---

### Phase 2 — Normalize semantic plans (Group B, 2–4 weeks)

**Goal**: убрать cross-phase recomputation.

**PR-4: `RuntimePlan`**
- Перенести `needs_push/has_exports/has_bindable/has_stores/has_ce_props` в analyze.
- Codegen читает один готовый plan.

**PR-5: `RenderTagPlan` consolidation**
- Склеить `mode + arg infos + prop sources` в один план.
- Перевести `template/render_tag.rs` полностью на него.

**PR-6: unified `ExprDeps`**
- Единый API для blockers/memo/await по Node/Attr.
- Перевести `if/each/events/attributes/expression` постепенно.

**PR-7: pre-resolved const-tag SymbolId**
- Перенести scope lookup из codegen в analyze side tables.

**Exit criteria**
- Уменьшилось число повторных semantic checks в codegen.
- Все мигрированные модули используют plan/query API, не field-level wiring.

---

### Phase 3 — Mechanical safety cleanup (Group C, 2–3 weeks)

**Goal**: убрать хрупкие технические контракты.

**PR-8: typed expression handles**
- Ввести `ExprHandle/StmtHandle`.
- Скрыть `span.start` как internal parser detail.

**PR-9: `TemplateScopeResolver`**
- Централизовать fallback policy для child scopes в transform.

**PR-10: pass-bundle API**
- Формализовать группы visitor’ов analyze.

**Exit criteria**
- Нет `unwrap_or(scope)`-копипасты в ключевых ветках transform.
- Analyze orchestration читается через bundles/descriptors, а не через ad-hoc сборку.

---

### Phase 4 — Consolidation (Group D, ongoing)

**Goal**: улучшить читабельность и стоимость поддержки.

**PR-11: `AsyncEmissionPlan`**
- Унифицировать async emission strategy в template codegen.

**PR-12+: mega-file decomposition**
- Разделять крупные файлы по функциональным срезам, без behavior change.

**Exit criteria**
- Повторяющиеся async-ветки сведены к единому паттерну.
- Новые фичи добавляются в меньшее число мест.

---

## Tracking & governance

### Weekly architecture check (30 min)

- Какие PR закрыли пункты roadmap (по ID: #1..#12)?
- Где остаются временные compatibility shims?
- Какие прямые обращения `ctx.analysis.*` ещё не мигрированы?

### Suggested KPIs

1. Кол-во прямых `ctx.analysis.*` обращений в `template/*`.
2. Кол-во мест, где codegen recompute’ит semantic policy.
3. Кол-во fallback scope lookups в transform.
4. Размер diff’ов на новые фичи (должен снижаться после Phases 1–3).

### Stop conditions (to avoid rewrite trap)

- Если PR затрагивает >2 групп одновременно — разбить.
- Если шаг меняет behavior и архитектуру одновременно — разделить на 2 PR.
- Если coverage падает/тесты нестабильны — заморозить migration, чинить базу.

---

## Fast-start recommendation (next 2 PRs)

1. **Сделать PR-1 (`CodegenView` facade, no behavior change)**.
2. **Сразу после — PR-4 (`RuntimePlan`)** как самый безопасный measurable win.

Это даст быструю пользу и создаст правильный ритм миграции.

---

## Kickoff: how to start implementation from this plan

Да — лучше стартовать **от текущего PR/ветки с roadmap**, но выполнять кодовые изменения отдельными малыми PR поверх него.

### Recommended branch flow

1. Merge (or keep as base) this docs PR.
2. Create feature branch for first implementation slice:
   - `git checkout -b refactor/codegen-view-phase1`
3. Делать только один шаг из playbook за ветку.

### First coding session (recommended)

**Take PR-1 from Phase 1**: `CodegenView` facade, без изменения поведения.

Минимальный scope:
- add `CodegenView` API in analyze
- wire it into codegen context
- migrate only safe reads in `codegen/lib.rs`
- keep compatibility layer (old access still works)

### Session checklist

- Before coding: read `README.md`, `CLAUDE.md`, `SIMPLIFY_REVIEW.md`.
- During coding: no mixed intents (только PR-1 scope).
- Validation: run focused tests + snapshots used by current workflow.
- Exit: same output, no snapshot drift, small diff.

### What NOT to do in first PR

- не трогать одновременно `RenderTagPlan` и `typed handles`
- не делать split `Ctx` полностью в первой итерации
- не удалять старые поля, пока все call-sites не мигрированы

Эта тактика дает быстрый старт без риска «большого переписывания».

---

## Detailed PR templates (so implementation steps are unambiguous)

Ниже детализированы первые PR из playbook в формате: **цель → изменения → файлы → проверки → done**.

### PR-1: `CodegenView` facade (no behavior change)

**Goal**
- Ввести стабильный read-only API между analyze и codegen без изменения логики генерации.

**Exact changes**
1. Добавить `CodegenView<'a>` в analyze (`types/data.rs` или соседний модуль query facade).
2. Экспортировать минимальный набор методов, который уже нужен в `codegen/lib.rs`.
3. В `context.rs` добавить `query: CodegenView<'a>` либо методы-делегаты на facade.
4. Перевести только безопасные read-only места в `codegen/lib.rs`.
5. Не удалять старые доступы к `analysis` (compatibility window).

**Target files (initial slice)**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_analyze/src/lib.rs` (если нужен re-export)
- `crates/svelte_codegen_client/src/context.rs`
- `crates/svelte_codegen_client/src/lib.rs`

**Checks**
- `just test-compiler`
- `just test-analyzer`
- `just test-case <1-2 затронутых кейса>`

**Done when**
- Output полностью идентичен.
- В `codegen/lib.rs` есть первые мигрированные чтения через facade.
- Нет удаления старых полей/API.

---

### PR-2: typed pass descriptors (analyze orchestration)

**Goal**
- Сделать зависимости пассов явными, сохранив текущий execution order.

**Exact changes**
1. Описать существующие пассы descriptor-структурами (`requires/produces`).
2. В orchestration использовать descriptors, но оставить прежний порядок исполнения.
3. Добавить assert/guard на missing prerequisites в debug path.

**Target files**
- `crates/svelte_analyze/src/lib.rs`
- `crates/svelte_analyze/src/passes/mod.rs`
- (опционально) новый файл `crates/svelte_analyze/src/passes/descriptors.rs`

**Checks**
- `just test-analyzer`
- `just test-compiler`

**Done when**
- Все старые пассы подключены через descriptors.
- Никакой behavioral drift в снапшотах.

---

### PR-3: split `Ctx` internals (`CodegenQuery` + `CodegenState`) with shim

**Goal**
- Разделить read-only семантические запросы и mutable state codegen.

**Exact changes**
1. Ввести структуры `CodegenQuery` и `CodegenState`.
2. Перенести поля `Ctx` по ответственности (без удаления старых методов).
3. Мигрировать 1-2 template модуля (например, `if_block.rs`, `render_tag.rs`) на новый доступ.

**Target files**
- `crates/svelte_codegen_client/src/context.rs`
- `crates/svelte_codegen_client/src/template/if_block.rs`
- `crates/svelte_codegen_client/src/template/render_tag.rs`

**Checks**
- `just test-compiler`
- `just test-case <if/render focused cases>`

**Done when**
- Новые структуры используются минимум в 1-2 модулях.
- Старый API остаётся как shim.

---

### PR-4: `RuntimePlan`

**Goal**
- Убрать recomputation runtime-флагов в codegen entry.

**Exact changes**
1. Добавить `RuntimePlan` в analyze.
2. Вычислять план в analyze после всех зависимых пассов.
3. В `codegen/lib.rs` заменить локальные вычисления на чтение `RuntimePlan`.

**Target files**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_analyze/src/lib.rs`
- `crates/svelte_codegen_client/src/lib.rs`

**Checks**
- `just test-analyzer`
- `just test-compiler`

**Done when**
- В `codegen/lib.rs` больше нет локальной сборки этих флагов.
- Все флаги читаются из одного плана.

---

### PR-5: `RenderTagPlan`

**Goal**
- Свести render-tag решения к одному plan-объекту.

**Exact changes**
1. Добавить `RenderTagPlan` и `RenderTagArgPlan` в analyze data layer.
2. Заполнять его в существующих render-tag related pass’ах.
3. Перевести `template/render_tag.rs` на новый источник.
4. Держать старые таблицы временно, пока все call-sites не мигрированы.

**Target files**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_analyze/src/passes/js_analyze.rs` (или соответствующие render-tag steps)
- `crates/svelte_codegen_client/src/template/render_tag.rs`

**Checks**
- `just test-case render_tag`
- `just test-compiler`

**Done when**
- `render_tag.rs` читает единый plan.
- Старые таблицы не используются в новом пути render-tag.

---

### PR-6: unified `ExprDeps`

**Goal**
- Убрать дублированные node/attr решения по blockers/memo/await.

**Exact changes**
1. Ввести `ExprSite` + `ExprDeps` query API.
2. Перевести последовательно: `if_block` → `each_block` → `events` → `attributes` → `expression`.
3. Оставить адаптеры к старым методам до полной миграции.

**Target files**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/template/if_block.rs`
- `crates/svelte_codegen_client/src/template/each_block.rs`
- `crates/svelte_codegen_client/src/template/events.rs`
- `crates/svelte_codegen_client/src/template/attributes.rs`
- `crates/svelte_codegen_client/src/template/expression.rs`

**Checks**
- `just test-compiler`
- `just test-case <async/blockers related>`

**Done when**
- В перечисленных модулях нет локальной дублированной логики вычисления deps.
- Решения берутся через `ExprDeps`.

---

## PR author checklist (copy into each PR body)

- [ ] Scope затрагивает только 1 шаг roadmap.
- [ ] Нет изменения behavior/snapshots (или изменение изолировано отдельным PR).
- [ ] Есть compatibility path (если миграция неполная).
- [ ] Есть явный plan удаления shim в следующем PR.
- [ ] Запущены целевые тесты (`just test-compiler` + focused test-case).

---

## Completeness self-check (to ensure nothing was lost)

Проверено, что документ по-прежнему содержит все ключевые блоки:

- [x] Audit coverage
- [x] Prioritized 12 simplifications
- [x] Proven compiler practices
- [x] Grouped execution plan (A–D)
- [x] Phase-by-phase implementation playbook
- [x] Governance/KPI/stop conditions
- [x] Kickoff branch flow
- [x] Detailed PR templates (PR-1..PR-6)

If any section changes in future edits, update this checklist in the same commit.
