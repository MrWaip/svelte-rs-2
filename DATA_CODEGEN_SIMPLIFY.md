# data.rs × codegen — simplification opportunities

Фокус: упростить стык `crates/svelte_analyze/src/types/data.rs` и потребление в `crates/svelte_codegen_client`.

## Что видно сейчас

### 1) Codegen читает `AnalysisData` и через `Ctx`-обертки, и напрямую
- В `Ctx` есть много прокси-методов, но в template/script модулях остаются частые прямые обращения `ctx.analysis.*`.
- Это смешивает 2 API-стиля одновременно и усложняет эволюцию data-модели.

### 2) Несколько сущностей передаются как "рассыпанные" таблицы
- Пример: render-tag логика живет в нескольких таблицах (`render_tag_*`) и собирается на стороне codegen.
- Аналогично для async/blockers есть несколько близких методов для node/attr.

### 3) Есть offset-plumbing для expr/stmt lookup
- `node_expr_offsets` + `attr_expr_offsets` + `parsed.exprs/stmts` по `span.start`.
- Для codegen это лишняя техническая деталь парсера.

---

## Приоритетные упрощения (по пользе)

### P1. Ввести единый `CodegenView` поверх `AnalysisData`

**Идея**
- В analyze сделать слой запросов для codegen (без раскрытия внутренней структуры таблиц):

```rust
pub struct CodegenView<'a> { data: &'a AnalysisData }

impl<'a> CodegenView<'a> {
    pub fn runtime_plan(&self) -> RuntimePlan;
    pub fn render_tag_plan(&self, id: NodeId) -> RenderTagPlanRef<'a>;
    pub fn expr_plan(&self, id: NodeId) -> ExprPlanRef<'a>;
    pub fn attr_expr_plan(&self, id: NodeId) -> ExprPlanRef<'a>;
}
```

**Польза**
- codegen больше не зависит от раскладки `AnalysisData` полей.
- будущие refactor’ы таблиц не "размазываются" по `template/*`.

**Где внедрять**
- `crates/svelte_analyze/src/types/data.rs`
- `crates/svelte_codegen_client/src/context.rs`

---

### P2. Склеить render-tag side tables в один `RenderTagPlan`

**Сейчас раздельно**
- `render_tag_callee_mode`
- `render_tag_arg_infos`
- `render_tag_prop_sources`

**Предложение**
- единый `NodeTable<RenderTagPlan>`:

```rust
pub struct RenderTagPlan {
    pub callee_mode: RenderTagCalleeMode,
    pub args: Vec<RenderTagArgPlan>,
}

pub struct RenderTagArgPlan {
    pub expr: ExpressionInfo,
    pub prop_source_sym: Option<SymbolId>,
}
```

**Польза**
- codegen получает один "готовый пакет" без синхронизации 2-3 таблиц по индексу.
- меньше потенциальных рассинхронов.

**Где внедрять**
- analyze: построение в pass’ах render-tag
- codegen: `template/render_tag.rs`

---

### P3. Унифицировать blockers API для node/attr через общий `ExprDeps`

**Сейчас**
- `expression_blockers(id)` и `attr_expression_blockers(id)`
- `expr_has_blockers(id)` (только для node)

**Предложение**
- общий enum ключа + единый API:

```rust
pub enum ExprSite { Node(NodeId), Attr(NodeId) }

pub struct ExprDeps {
    pub has_await: bool,
    pub blocker_indices: SmallVec<[u32; 2]>,
    pub has_calls: bool,
    pub needs_memo: bool,
}

pub fn expr_deps(&self, site: ExprSite) -> ExprDeps;
```

**Польза**
- меньше дублирования в template/modules (`if`, `each`, `events`, `attributes`, `expression`).
- единый контракт для async/memo решений.

---

### P4. Убрать вычисление prelude/runtime-флагов из codegen `lib.rs`

**Сейчас в codegen считаются:**
- `needs_push`, `has_exports`, `has_bindable`, `has_ce_props`, `has_stores`

**Предложение**
- перенести в analyze в готовую структуру:

```rust
pub struct RuntimePlan {
    pub needs_push: bool,
    pub has_stores: bool,
    pub has_exports: bool,
    pub has_bindable: bool,
    pub has_ce_props: bool,
}
```

**Польза**
- один источник истины для runtime wiring.
- меньше cross-phase recombination в codegen.

---

### P5. Заменить offset API на typed expression handles

**Сейчас**
- `node_expr_offset/attr_expr_offset` + `parsed.exprs.get/remove(offset)`.

**Предложение**
- typed handle:

```rust
pub struct ExprHandle(u32);
pub struct StmtHandle(u32);
```

- analyze хранит `NodeId -> ExprHandle`, `AttrId -> ExprHandle`.
- parser скрывает внутренний `span.start` как implementation detail.

**Польза**
- проще читать codegen/transform, меньше "магии offset".

---

### P6. Для const-tag добавить предвычисленные SymbolId в analyze

**Сейчас**
- в codegen есть повторные `find_binding(scope_id, name)` по строкам const-tag имен.

**Предложение**
- в `ConstTagData` хранить resolved `Vec<SymbolId>` рядом с именами.
- codegen берет symbols напрямую без строкового поиска.

**Польза**
- меньше runtime-поисков/условной логики в codegen.
- stronger boundary: analyze владеет symbol resolution.

---

## Быстрый безопасный план внедрения (3 PR)

1. **PR-A (низкий риск):** `RuntimePlan` + `CodegenView::runtime_plan()` + адаптация `codegen/lib.rs`.
2. **PR-B (средний риск):** `RenderTagPlan` + миграция `template/render_tag.rs`.
3. **PR-C (средний риск):** unified `ExprDeps` API и перевод `if/each/events/attributes` на него.

## Почему это даст наибольший эффект

- Снижает количество мест, где codegen «знает» внутренности `AnalysisData`.
- Убирает дублирующие контракты (несколько таблиц/методов для одной концепции).
- Делает изменения в analyze дешевле и безопаснее для downstream.
