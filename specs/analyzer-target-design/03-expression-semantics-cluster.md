# `ExpressionSemantics` cluster (5-й кластер)

## Parent

`specs/analyzer-target-design.md`

## What to build

Новый кластер `ExpressionSemantics` — пятый кластер per Svelte kind. Идентичность — `NodeId` конкретного `ExpressionTag`-узла, сидящего как **child фрагмента**.

### Scope

`ExpressionSemantics` хранит payload **только** для fragment-child `ExpressionTag`-узлов:

- standalone `<div>{expr}</div>` (ExpressionTag в children фрагмента)
- coalesced concat-parts типа `<div>hi {a} {b}!</div>` — каждая ConcatPart::Expr ссылается на NodeId всё того же fragment-child ExpressionTag

**Out of scope полностью:**

- value-expression любого attribute/directive (`attr={expr}`, `class:foo={expr}`, `style:foo={expr}`, `bind:foo={expr}`, `on:click={expr}`, `use:action={expr}`, `transition:t={expr}`, `animate:a={expr}`, `attach:a={expr}`)
- attribute concat-parts (`<div class="a {b}">`)
- spread (`<div {...x}>`)
- block-defining expressions (each-collection, if-condition, await-promise, render-args, const-tag-init, key, html-tag-expression)
- `{@debug}` identifier-refs, boundary-handler callbacks

Эти сайты обслуживаются текущей инфраструктурой (`ExpressionInfo`, `expr_*`-helpers, `PickledAwaitOffsets`) и переезжают в `AttributeSemantics` (spec 04). До spec 04 они работают как раньше.

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

### Builder

`ExpressionSemanticsBuilder`:

```rust
pub fn build<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    component_semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    blockers: &BlockerData,
    pickled_await_offsets: &PickledAwaitOffsets,
    node_count: u32,
) -> ExpressionSemanticsStore;
```

Алгоритм (один template walk + sub-walk на subtree fragment-child ExpressionTag + inline derive):

1. Walk template, заходит **только** в fragment-child `ExpressionTag`. В attribute/directive value-expression и в block-defining expression-сайты builder не заходит.
2. Sub-visitor (`ExprRawFactsCollector`, builder-local) на subtree ExpressionTag-а накапливает:
    - `references: SmallVec<[SymbolId; 2]>` — resolved symbols через `ComponentSemantics::get_reference(ref_id).symbol_id()`, deduplicated by encounter order (DFS preorder). Store-references резолвятся в store_symbol через `ReactivitySemantics::reference_semantics(ref_id)`.
    - Builder-internal bitflags `ExpressionFlags { HAS_CALL, HAS_AWAIT, HAS_STATE_RUNE, HAS_STORE_REF, HAS_SIDE_EFFECTS, USES_LEGACY_SANITIZED_PROPS, NEEDS_CONTEXT, IS_DYNAMIC }`.
    - `top_level_shape` (builder-local) для `legacy_wrap`-композиции.
3. Inline derive четырёх public решений из raw facts:
    - `ExprKind::Async { has_await, is_pickled, blockers }` если `flags.HAS_AWAIT` или `references.iter().filter_map(|s| blockers.symbol_blocker(s)).next().is_some()`. Иначе `Dynamic` если `flags & (IS_DYNAMIC | HAS_STATE_RUNE | HAS_STORE_REF)`. Иначе `Static`. `is_pickled` определяется лукапом `pickled_await_offsets.contains(await_expr.span.start)` для AwaitExpression-узлов внутри ExpressionTag-subtree.
    - `LegacyWrap` через `component_semantics.runes()` (если runes-mode → всегда `None`) + top-level shape (`MemberExpression | Assignment | Update`) + `flags.HAS_CALL` + `flags.USES_LEGACY_SANITIZED_PROPS`. Все три bool-а композируются в один из четырёх вариантов.
    - `Memoization` из `flags.HAS_CALL || flags.HAS_AWAIT` + `flags.HAS_AWAIT` (sync vs async bucket).
    - `references` копируется из collector-а.

Никаких post-walk фаз: каждое решение зависит только от raw-фактов конкретного fragment-child ExpressionTag + global tables (`BlockerData`, `ComponentSemantics`, `ReactivitySemantics`, `PickledAwaitOffsets`).

### Consumer contract

Codegen для in-scope сайтов (`emit_text_set`, fragment-child concat-parts в `emit_concat_set`):

```rust
match data.expressions.get(node_id) {
    ExpressionSemantics::NonSpecial => { static path, see AST }
    ExpressionSemantics::Expression(sem) => {
        let expr = match sem.legacy_wrap {
            LegacyWrap::None => expr,
            LegacyWrap::CoarseWrap => emit_coarse(expr, &sem.references),
            LegacyWrap::SanitizedProps => emit_sanitized(expr),
            LegacyWrap::CoarseAndSanitized => emit_coarse(emit_sanitized(expr), &sem.references),
        };
        let effective = match sem.memoization {
            Memoization::None => expr,
            Memoization::SyncMemo => push_sync_memo(...),
            Memoization::AsyncMemo => push_async_memo(...),
        };
        let is_dyn = matches!(sem.kind, ExprKind::Dynamic | ExprKind::Async { .. });
    }
}
```

**Один lookup** на сайт. **Один центральный** match по варианту. Внутри `Expression(sem)`-арм-а — линейная логика, читающая только поля `sem`. Никаких `&&`-цепочек на ≥2 сторов. Никаких partial-`if let Expression(data)` внутри helper-ов.

`is_pickled_await`-rewrite в transform для AwaitExpression-узлов **внутри fragment-child ExpressionTag** может лукапить через NodeId; для AwaitExpression в out-of-scope сайтах (атрибуты) — продолжает работать `PickledAwaitOffsets`. Полное удаление `PickledAwaitOffsets` — задача spec 04.

### Cleanup в этом спринте

После миграции in-scope сайтов:

- Codegen-сайты `emit_text_set` и concat-parts в `emit_concat_set` читают **только** `data.expressions.get(id)`.
- Helper-ы, доступ к которым теперь идёт через payload (`is_dynamic(id)` для fragment-child ExpressionTag, `expr_has_await(id)`, `expr_has_blockers(id)`, `expr_deps(id)`, `attr_expression(id)` — но только для fragment-child ExpressionTag), **не удаляются** глобально: их продолжают читать out-of-scope сайты (атрибутные). Удаление этих helper-ов — задача spec 04.

### Cleanup в spec 04

Эти пункты явно **переезжают в spec 04** (атрибуты):

- Удаление `crates/svelte_analyze/src/types/data/expr.rs` (`ExpressionInfo`, `ExpressionKind`, `ExprRole`, `ExprSite`, `ExprDeps`).
- Удаление `crates/svelte_analyze/src/types/data/pickled_await_offsets.rs` (`PickledAwaitOffsets`).
- Удаление `crates/svelte_analyze/src/passes/js_analyze/expression_info.rs`.
- Удаление `merge_concat_expression_info`.
- Удаление `expr_*`-helper-ов (`expr_deps`, `expr_role`, `expr_is_async`, `expr_has_await`, `expr_has_blockers`).
- Удаление `attr_expression(id)`-accessor.
- Миграция `transform/template_rewrites.rs` на NodeId-лукап для всех `await`-сайтов (включая атрибуты).

Decision basis: §8 (cluster ownership), §10 (async as field, not peer cluster), §18 (ExpressionInfo dissolves), §24 (PickledAwaitOffsets absorbed), §50 (precompute rule), §52 (helper-mapping).

## Acceptance criteria

- [ ] `ExpressionSemanticsBuilder` существует, регистрируется в `analyze()`.
- [ ] `ExpressionSemanticsStore` — top-level поле `AnalysisData::expressions_v2`.
- [ ] Public payload — ровно `{ kind, legacy_wrap, memoization, references }` per `ExpressionData`, `NonSpecial`-default для out-of-scope NodeId.
- [ ] `ExprKind::Async { has_await, is_pickled, blockers }` — единственное место хранения per-expression async-фактов **для fragment-child ExpressionTag**.
- [ ] `LegacyWrap`-вариант выставляется builder-ом с учётом `component_semantics.runes()` — codegen `runes()`-чек удалён в in-scope сайтах.
- [ ] In-scope codegen-сайты (`emit_text_set`, fragment-child concat-parts в `emit_concat_set`) читают `data.expressions.get(id)` единым центральным матчем по варианту, без `&&`-цепочек на ≥2 сторов и без partial-`if let`-ов внутри helper-ов.
- [ ] 0 строковых полей и 0 публичных bitflag-полей в payload (`ExpressionFlags` — builder-local scratch only).
- [ ] Builder walker не заходит в attribute/directive value-expression и в block-defining expression-subtrees.
- [ ] Юнит-тесты на `ExpressionSemanticsBuilder` per поле payload (`kind`-варианты, `legacy_wrap`-варианты, `memoization`-варианты, `references`-сборка).
- [ ] `just test-compiler` зелёный.
- [ ] `just test-diagnostics` зелёный.
- [ ] `just clippy-strict` зелёный.
- [ ] Запись в `debt.md` снята / обновлена.

## Blocked by

- `02-block-semantics-debug-html-tag.md` (per PRD Decision 39 — Block доделывается раньше; после narrow scope Expression формально не зависит от BlockSemantics, но migration sequence сохраняется ради staged removal).

## Unblocks

- `04-attribute-semantics-cluster.md` (spec 04) — финальное удаление `ExpressionInfo`/`expr_*`/`PickledAwaitOffsets` плюс миграция всех out-of-scope expression-сайтов идёт там.
