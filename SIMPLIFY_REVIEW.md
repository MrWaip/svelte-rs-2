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

### B) Data-oriented IR between stages (small explicit intermediate representations)

**Почему это работает**
- Успешные компиляторы редко гоняют «сырые AST + флаги» до финального backend.
- Они вводят 1–2 небольших IR с инвариантами, чтобы downstream не переоткрывал семантику.

**Как применить**
- Ввести mini-IR для template emission plan (например, `RenderTagPlan`, `ExprDeps`, `RuntimePlan`).
- Цель: codegen потребляет планы, а не вычисляет политику.

**Где начать**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/template/*.rs`

---

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
- Для каждой архитектурной миграции (query/plan/IR) делать behavior-preserving PR-цепочки.
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
