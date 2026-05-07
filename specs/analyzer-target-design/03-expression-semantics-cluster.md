# `ExpressionSemantics` cluster (5-й кластер)

## Parent

`specs/analyzer-target-design.md`

## What to build

Новый кластер `ExpressionSemantics` — пятый кластер per Svelte kind. Идентичность — `NodeId` конкретного OXC `Expression`-узла, на который ссылается template-сайт типа text-emission или attribute/directive-value-emission.

### Scope

`ExpressionSemantics` хранит payload **только** для следующих сайтов:

- standalone `{expr}` в фрагменте (`ExpressionTag.expression`)
- value-expression любого attribute / directive: `attr={expr}`, `class:foo={expr}`, `style:foo={expr}`, `bind:foo={expr}`, `on:click={expr}`, `use:action={expr}`, `transition:t={expr}`, `animate:a={expr}`, `attach:a={expr}`, и т.д.
- каждая `Concatenation::Dynamic { id }`-часть концат-атрибута (`<div class="a {b}{c}">` → entries для NodeId-ов `b` и `c`)

Block-defining expressions **не в scope**: each-collection, each-key, if-condition, await-promise, render-args/callee, const-tag-init, html-tag-expression. Их async/blocker-факты уже живут в `BlockSemantics`-вариантах (`EachAsyncKind::Async { blockers }`, `IfAsyncKind::Async { blockers }`, `RenderAsyncKind::Async { blockers }`, `AwaitWrapper::AsyncWrap { blockers }`, `ConstTagAsyncKind::Async { blockers }`, `KeyAsyncKind::Async { blockers }`). Дублировать их в Expression payload — лишний precompute. Codegen этих сайтов читает `BlockSemantics`, не `ExpressionSemantics`.

`{@debug}`-identifier-refs и boundary-handler callbacks — `NonSpecial` (codegen читает напрямую из AST, payload не нужен).

### Public payload shape

```rust
pub enum ExpressionSemantics {
    #[default]
    NonSpecial,
    Expression(ExpressionData),
}

pub struct ExpressionData {
    pub kind: ExprKind,
    pub legacy_wrap: LegacyWrap,
    pub memoization: Memoization,
    pub references: SmallVec<[SymbolId; 2]>,
}

pub enum ExprKind {
    Static,
    Dynamic,
    Async {
        has_await: bool,
        is_pickled: bool,
        blockers: SmallVec<[u32; 2]>,
    },
}

pub enum LegacyWrap {
    None,
    CoarseWrap,
    SanitizedProps,
    CoarseAndSanitized,
}

pub enum Memoization {
    None,
    SyncMemo,
    AsyncMemo,
}
```

Никаких bitflags в публичном payload-е, никаких bool-полей вне enum-вариантов, никаких строк. Identity — `NodeId`/`SymbolId`.

`ExpressionInfo` (legacy bag-of-facts с полями `kind`, `expr_role`, `ref_symbols`, `uses_legacy_*`, `has_*`) и `ExpressionKind` (top-level дискриминанта-обёртка) удаляются как типы. Top-level shape потребитель/builder матчит по OXC `Expression` напрямую.

### Поглощения side-tables

- `PickledAwaitOffsets` удаляется. Факт переезжает в `ExprKind::Async { is_pickled }`-вариант. `transform/template_rewrites.rs` лукапит через NodeId: `data.expressions.get(await_expr.node_id())` → матч на `ExprKind::Async { is_pickled }`.
- Per-expression `dynamism: bool` поглощается `ExprKind` (Static vs Dynamic vs Async).
- `expr_deps`/`expr_blockers`/`expr_has_await`/`expr_has_blockers` поглощаются `ExprKind::Async { has_await, blockers }`. Sync-выражения не несут пустых SmallVec — type-level гарантия.
- `needs_clsx` уезжает в `AttributeSemantics` (parent-context-зависимый факт class-attr-position, не expression-факт).
- `is_expression_shorthand` уезжает в `AttributeSemantics` (атрибутный shorthand `<input {value}>` ⇔ `value={value}`).
- Аггрегированные attribute-level факты (`has_call`/`has_side_effects`/`is_function`/`is_import` под NodeId самого Attribute-узла) уезжают в `AttributeSemantics` payload как поля enum-вариантов. Концепт `merge_concat_expression_info` исчезает: каждая `Dynamic { id }`-часть концата — отдельный entry в `ExpressionSemantics`, агрегация по атрибуту делается `AttributeSemanticsBuilder`-ом.

### Builder

`ExpressionSemanticsBuilder` зеркалит `BlockSemanticsBuilder`:

```rust
pub fn build<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    component_semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    blockers: &BlockerData,
    node_count: u32,
) -> ExpressionSemanticsStore;
```

Алгоритм (один template walk + sub-walk на subtree expression-сайта + inline derive):

1. Walk template, для каждого in-scope expression-сайта вызывает sub-visitor по AST-поддереву.
2. Sub-visitor (`ExprRawFactsCollector`, builder-local) накапливает:
    - `references: SmallVec<[SymbolId; 2]>` — resolved symbols через `ComponentSemantics::get_reference(ref_id).symbol_id()`, deduplicated by encounter order (DFS preorder).
    - Builder-internal bitflags `ExpressionFlags { HAS_CALL, HAS_AWAIT, HAS_STATE_RUNE, HAS_STORE_REF, HAS_SIDE_EFFECTS, HAS_STORE_MEMBER_MUTATION, USES_LEGACY_SLOTS, USES_LEGACY_SANITIZED_PROPS, IS_PICKLED_AWAIT, NEEDS_CONTEXT, IS_DYNAMIC }` через AST-node-kinds + `ReactivitySemantics::binding_semantics(sym)` запросы на каждый identifier.
    - `top_level_shape` (builder-local) для `legacy_wrap`-композиции.
3. Inline derive четырёх public решений из raw facts:
    - `ExprKind::Async { has_await, is_pickled, blockers }` если `flags.HAS_AWAIT` или `references.iter().filter_map(|s| blockers.symbol_blocker(s)).next().is_some()`. Иначе `Dynamic` если `flags & (IS_DYNAMIC | HAS_STATE_RUNE | HAS_STORE_REF)`. Иначе `Static`. `is_pickled` = `flags.IS_PICKLED_AWAIT`.
    - `LegacyWrap` через `component_semantics.runes()` (если runes-mode → всегда `None`) + top-level shape (`MemberExpression | Assignment | Update`) + `flags.HAS_CALL` + `flags.USES_LEGACY_SANITIZED_PROPS`. Все три bool-а композируются в один из четырёх вариантов.
    - `Memoization` из `flags.HAS_CALL || flags.HAS_AWAIT` + `flags.HAS_AWAIT` (sync vs async bucket).
    - `references` копируется из collector-а.

Никаких post-walk фаз: каждое решение зависит только от raw-фактов конкретного expression-а + global tables (`BlockerData`, `ComponentSemantics`, `ReactivitySemantics`), которые уже готовы в Phase 1/2. Между expression-ами зависимостей нет.

### Consumer contract

Codegen для in-scope сайтов:

```rust
match data.expressions.get(node_id) {
    ExpressionSemantics::NonSpecial => { /* see AST, no precompute */ }
    ExpressionSemantics::Expression(sem) => {
        match sem.legacy_wrap { LegacyWrap::None => expr, LegacyWrap::CoarseWrap => emit_coarse(expr, &sem.references), ... }
        match sem.memoization { Memoization::None => emit_inline(expr), Memoization::SyncMemo => push_sync(...), Memoization::AsyncMemo => push_async(...) }
        match sem.kind { ExprKind::Static => ..., ExprKind::Dynamic => ..., ExprKind::Async { blockers, .. } => emit_with_blockers(blockers) }
    }
}
```

Никаких `&&`-цепочек на `runes() && needs_legacy_coarse_wrap()`. Никаких `is_dynamic(id) || expr_has_await(id) || expr_has_blockers(id)` композиций.

Transform для `is_pickled_await`-rewrite:

```rust
if let ExpressionSemantics::Expression(ExpressionData { kind: ExprKind::Async { is_pickled: true, .. }, .. }) = data.expressions.get(await_expr.node_id()) {
    rewrite_pickled(await_expr);
}
```

Лукап по `NodeId`, не `span.start`.

### Cleanup

После миграции удаляются:

- `crates/svelte_analyze/src/types/data/expr.rs` (`ExpressionInfo`, `ExpressionKind`, `ExprRole`, `ExprSite`, `ExprDeps`).
- `crates/svelte_analyze/src/types/data/pickled_await_offsets.rs` (`PickledAwaitOffsets`).
- `crates/svelte_analyze/src/passes/js_analyze/expression_info.rs` (включая `analyze_expression`, `needs_context`-pass-фрагменты, attribute-merge).
- `merge_concat_expression_info` и shadow-aggregation в `collect_symbols.rs`.
- Helper-методы на `Ctx`/`AnalysisData` с префиксом `expr_` (`expr_deps`, `expr_role`, `expr_is_async`, `expr_has_await`, `expr_has_blockers`).
- `attr_expression(id)` accessor (single-store решает `or_else`-цепочку в `regular.rs:237`).
- `is_pickled_await(offset: u32)` accessor; transform лукапит через NodeId.

Decision basis: §8 (cluster ownership), §10 (async as field, not peer cluster), §18 (ExpressionInfo dissolves), §24 (PickledAwaitOffsets absorbed), §50 (precompute rule), §52 (helper-mapping).

## Acceptance criteria

- [ ] `ExpressionSemanticsBuilder` существует, регистрируется в `analyze()` (Phase 4 или раньше — Expression больше не зависит от `BlockSemanticsStore`).
- [ ] `ExpressionSemanticsStore` — top-level поле `AnalysisData::expressions`. `attr_expressions` удалён.
- [ ] Public payload — ровно `{ kind, legacy_wrap, memoization, references }` per `ExpressionData`, `NonSpecial`-default для out-of-scope NodeId.
- [ ] `ExprKind::Async { has_await, is_pickled, blockers }` — единственное место хранения per-expression async-фактов.
- [ ] `LegacyWrap`-вариант выставляется builder-ом с учётом `component_semantics.runes()` — codegen `runes()`-чек удалён.
- [ ] Codegen pipeline для in-scope сайтов читает `data.expressions.get(id)` единым матчем по варианту, без `&&`-цепочек на ≥2 сторов.
- [ ] 0 чтений `ExpressionInfo` (как типа) из codegen и transform.
- [ ] `ExpressionInfo`, `ExpressionKind`, `ExprRole`, `ExprSite`, `ExprDeps` удалены.
- [ ] `PickledAwaitOffsets` удалён. Transform лукапит через `data.expressions.get(node_id)`.
- [ ] 0 helper-методов с префиксом `expr_` на `Ctx`/`AnalysisData`.
- [ ] 0 строковых полей и 0 публичных bitflag-полей в payload (`ExpressionFlags` — builder-local scratch only).
- [ ] `merge_concat_expression_info` удалён, каждая `Dynamic { id }` — отдельный entry в store.
- [ ] `needs_clsx` и `is_expression_shorthand` мигрированы в `AttributeSemantics` (acceptance этих миграций — в spec 04).
- [ ] Юнит-тесты на `ExpressionSemanticsBuilder` per поле payload (`kind`-варианты, `legacy_wrap`-варианты, `memoization`-варианты, `references`-сборка).
- [ ] `just test-compiler` зелёный.
- [ ] `just test-diagnostics` зелёный.
- [ ] `just clippy-strict` зелёный.
- [ ] Запись в `debt.md` снята / обновлена.

## Blocked by

- `02-block-semantics-debug-html-tag.md` (per PRD Decision 39 — Block доделывается раньше; после narrow scope Expression формально не зависит от BlockSemantics, но migration sequence сохраняется ради staged removal).
