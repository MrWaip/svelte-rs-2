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
    pub blockers: SmallVec<[u32; 2]>,
    pub legacy_wrap: LegacyWrap,
    pub memoization: Memoization,
    pub references: SmallVec<[SymbolId; 2]>,
}

impl ExpressionData {
    pub fn has_await(&self) -> bool;
    pub fn is_dynamic(&self) -> bool;
    pub fn needs_effect(&self) -> bool;
    pub fn needs_node_memo(&self) -> bool;
}

pub enum ExprKind {
    Folded(CompactString),
    Static,
    Dynamic,
    Async { has_await: bool, is_pickled: bool },
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

Никаких bitflags в публичном payload-е, никаких bool-полей вне enum-вариантов, никаких mutable строк / hash-таблиц. `ExprKind::Folded(CompactString)` хранит compile-time-resolvable строковое значение для identifier-of-known-binding case (`scoping.known_value_by_sym`). `blockers` — индексы в `BlockerData::symbol_blockers`, общая модель для sync (`{@const}`) и async (`await`). Identity — `NodeId`/`SymbolId`.

`needs_effect = is_dynamic() || !blockers.is_empty()` — emit-decision: нужен ли `$.template_effect`/`$.update`-обёртка. Не путать с `is_dynamic` (свойство значения, не emission-стратегии).

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
3. **Folded fast-path**: если top-level expression — `Identifier` И `references.len() == 1` И `scoping.known_value_by_sym(refs[0])` → `Some(s)` → возвращаем `ExpressionData { kind: Folded(s.into()), blockers: empty, legacy_wrap: None, memoization: None, references: empty }`. Codegen увидит folded и инлайнит как литерал без emission-обёртки.
4. Inline derive четырёх public решений из raw facts:
    - `blockers: SmallVec<[u32; 2]>` собирается отдельной функцией `derive::blockers(facts, blocker_data)` — индексы из `BlockerData::symbol_blockers` для каждого ref-symbol. Хранится на верх `ExpressionData`, общая для sync и async кейсов.
    - `ExprKind::Async { has_await, is_pickled }` если `flags.HAS_AWAIT` или `!blockers.is_empty()`. Иначе `Dynamic` если `flags & (IS_DYNAMIC | HAS_STATE_RUNE | HAS_STORE_REF)`. Иначе `Static`. `is_pickled` определяется лукапом `pickled_await_offsets.contains(await_expr.span.start)` для AwaitExpression-узлов внутри ExpressionTag-subtree.
    - `LegacyWrap` через `component_semantics.runes()` (если runes-mode → всегда `None`) + top-level shape (`MemberExpression | Assignment | Update`) + `flags.HAS_CALL` + `flags.USES_LEGACY_SANITIZED_PROPS`. Все три bool-а композируются в один из четырёх вариантов.
    - `Memoization` из `flags.HAS_CALL && !references.is_empty()` (sync) или `flags.HAS_AWAIT` (async).
    - `references` копируется из collector-а.

Никаких post-walk фаз: каждое решение зависит только от raw-фактов конкретного fragment-child ExpressionTag + global tables (`BlockerData`, `ComponentSemantics`, `ReactivitySemantics`, `PickledAwaitOffsets`, `ComponentScoping`).

### Consumer contract

Все fragment-child ExpressionTag-сайты сосредоточены в одном emission entry-point — `Codegen::emit_concatenation` (`crates/svelte_codegen_client/src/codegen/concatenation.rs`). Per-part loop в `build_concatenation_parts`:

```rust
for part in parts {
    if let Some(s) = ctx.static_text_of(part) { merge_str(s); continue; }
    let ConcatPart::Expr(id) = part else { continue };

    if let ExpressionSemantics::Expression(data) = view.expression_semantics(*id)
        && let ExprKind::Folded(s) = &data.kind
    {
        merge_str(s);
        continue;
    }

    let expr = self.take_node_expr(*id)?;
    let defined = self.is_node_expr_definitely_defined(*id, &expr); // post-transform shape; см. §post-transform note
    match view.expression_semantics(*id) {
        ExpressionSemantics::NonSpecial => tpl_parts.push(TemplatePart::Expr(expr, defined)),
        ExpressionSemantics::Expression(data) => {
            let part_needs_effect = data.is_dynamic();
            let expr = self.apply_legacy_wrap(expr, data.legacy_wrap, &data.references);
            let effective = match data.memoization {
                Memoization::None      => expr,
                Memoization::SyncMemo  => push_sync_memo(...),
                Memoization::AsyncMemo => push_async_memo(...),
            };
            tpl_parts.push(TemplatePart::Expr(effective, defined));
            needs_effect |= part_needs_effect;
        }
    }
}
```

**Один lookup** на part (Folded fast-path + основной match). **Один центральный** match по варианту. Внутри `Expression(data)`-арм-а — линейная логика, читающая только поля `data`. Никаких `&&`-цепочек на ≥2 сторов. Никаких partial-`if let Expression(data)` внутри helper-ов.

**Post-transform note.** `is_definitely_defined`-проверка (для решения «нужен ли `?? ""` coalesce в template-literal») остаётся **codegen-side** в `Codegen::is_node_expr_definitely_defined`, потому что фундаментально требует **post-transform** shape JS-выражения (например, для unkeyed `{#each items as _, index}` — `index` после transform остаётся `Identifier`; для keyed `{#each items as item, i (item.id)}` — `i` превращается в `$.get(i)`, `CallExpression`). Analyze видит только pre-transform AST → информации недостаточно. Это инвариант разделения analyze ↔ codegen: pre-transform свойства живут в analyze, post-transform-shape-зависимые — в codegen.

`is_pickled_await`-rewrite в transform для AwaitExpression-узлов **внутри fragment-child ExpressionTag** может лукапить через NodeId; для AwaitExpression в out-of-scope сайтах (атрибуты) — продолжает работать `PickledAwaitOffsets`. Полное удаление `PickledAwaitOffsets` — задача spec 04.

### Cleanup в этом спринте

После миграции in-scope сайтов:

- Все fragment-child ExpressionTag-эмиссии локализованы в `concatenation.rs::emit_concatenation` (4 anchor-варианта: `SiblingTextNode`, `SingleFragmentChild`, `SingleFragmentRoot`, `SingleFragmentCallbackParam`). Старые `emit_text_set`, `emit_concat_set`, `emit_expr_node_in_fragment`, `emit_concat_node_in_fragment` удалены.
- Per-part loop читает **только** `view.expression_semantics(id)` — единый lookup на part через `Folded` fast-path + основной match.
- `Codegen::try_resolve_known_from_expr` мёртв в fragment-child пути (Folded резолвится в analyze). Helper остаётся для title/attribute путей (out of scope).
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

- [x] `ExpressionSemanticsBuilder` существует, регистрируется в `analyze()`.
- [x] `ExpressionSemanticsStore` — top-level поле `AnalysisData::expressions_v2`.
- [x] Public payload — `{ kind, blockers, legacy_wrap, memoization, references }` per `ExpressionData`, `NonSpecial`-default для out-of-scope NodeId.
- [x] `blockers` — общая модель индексов в `BlockerData::symbol_blockers`, единое место хранения per-expression blocker-фактов (sync `{@const}` + async `await`) для fragment-child ExpressionTag. `ExprKind::Async { has_await, is_pickled }` хранит per-expression async-флаги.
- [x] `ExprKind::Folded(CompactString)` — fast-path для compile-time-resolvable identifier-of-known-binding (через `scoping.known_value_by_sym`).
- [x] `LegacyWrap`-вариант выставляется builder-ом с учётом `component_semantics.runes()` — codegen `runes()`-чек удалён в in-scope сайтах.
- [x] Все fragment-child ExpressionTag-эмиссии локализованы в `concatenation.rs::emit_concatenation`. Per-part loop — единый центральный match (Folded fast-path + основной), без `&&`-цепочек на ≥2 сторов и без partial-`if let`-ов внутри helper-ов.
- [x] 0 mutable строк / hash-таблиц в payload, 0 публичных bitflag-полей. `ExpressionFlags` — builder-local scratch only. `Folded(CompactString)` — immutable string variant, допустим как эквивалент enum-литерала.
- [x] Builder walker не заходит в attribute/directive value-expression и в block-defining expression-subtrees.
- [x] Юнит-тесты на `ExpressionSemanticsBuilder` per поле payload (`kind`-варианты, `legacy_wrap`-варианты, `memoization`-варианты, `references`-сборка).
- [x] `just test-compiler` зелёный (676 + 422).
- [x] `just test-diagnostics` зелёный.
- [x] `just clippy-strict` зелёный.
- [ ] Запись в `debt.md` («`expr_deps` builder leaks analysis work into codegen») — для fragment-child пути закрыто, для attribute/title пути остаётся актуальной. Обновить в рамках spec 04.

## Blocked by

- `02-block-semantics-debug-html-tag.md` (per PRD Decision 39 — Block доделывается раньше; после narrow scope Expression формально не зависит от BlockSemantics, но migration sequence сохраняется ради staged removal).

## Unblocks

- `04-attribute-semantics-cluster.md` (spec 04) — финальное удаление `ExpressionInfo`/`expr_*`/`PickledAwaitOffsets` плюс миграция всех out-of-scope expression-сайтов идёт там.
