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
