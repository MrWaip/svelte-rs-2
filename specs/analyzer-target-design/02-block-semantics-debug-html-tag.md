# BlockSemantics: `HtmlTag` variant + namespace bitsets removal + `DebugTagData` cleanup

## Parent

`specs/analyzer-target-design.md`

## What to build

`{@html}` получает variant `BlockSemantics::HtmlTag(HtmlTagSemantics)` с pre-computed payload. Codegen `emit_html_tag` мигрирует с трёх отдельных лукапов (`view.html_tag_in_svg`, `view.html_tag_in_mathml`, `view.is_ignored(_, "hydration_html_changed")`) на чтение одного payload-варианта.

`{@debug}` **variant в `BlockSemantics` не получает**. AST-узел `DebugTag { id, span, identifier_refs }` уже самодостаточен: codegen `emit_hoisted_debug_tag` берёт имя из `source_text(span)`, выражение — из `parsed.take_expr(ident_ref.id())`. Pre-compute нечего, добавление variant'а было бы голым зеркалом AST. Decision 9 парент-спеки переформулируется: `{@html}` — variant, `{@debug}` — остаётся AST-only.

`DebugTagData` side-table удаляется как мёртвый: используется только одним юнит-тестом, codegen уже читает AST напрямую.

Параллельно удаляется дублирующая namespace-инфраструктура для `{@html}`: после миграции namespace живёт внутри `HtmlTagSemantics.parent_strategy`, поэтому `data.elements.html_tag_in_svg/mathml` bitsets, `pending_html_tags` collector в `template_side_tables`, post-walk join в `executor.rs`, и accessors на `codegen_view` — всё удаляется. Один источник правды.

`is_controlled` (controlled-fragment shape — единственный ребёнок RegularElement) **остаётся в codegen**. Это fragment-shape fact, не block-semantics — переедет в `FragmentSemanticsStore` спекой 06. До тех пор `parent_strategy` хранит только namespace.

Decision basis: §9 (переформулирован) + §21 (DebugTagData растворяется не в variant'е, а удаляется как side-table) + §48 (`hydration_html_changed_ignored` как поле payload).

### Форма нового variant'а

```rust
pub enum BlockSemantics {
    NonSpecial,
    Each(EachBlockSemantics),
    Await(AwaitBlockSemantics),
    Snippet(SnippetBlockSemantics),
    ConstTag(ConstTagBlockSemantics),
    Render(RenderTagBlockSemantics),
    If(IfBlockSemantics),
    Key(KeyBlockSemantics),
    HtmlTag(HtmlTagSemantics),
}

pub struct HtmlTagSemantics {
    pub parent_strategy: HtmlTagNamespace,
    pub hydration_html_changed_ignored: bool,
}

pub enum HtmlTagNamespace {
    Html,
    Svg,
    MathMl,
}
```

`expression_node_id` в payload не кладётся — выражение уже доступно как `tag.expression: ExprRef` напрямую из AST.

`hydration_html_changed_ignored` хранит **финальный ответ** с уже запечённым `state.dev`: `dev && ignore_data.is_ignored(id, "hydration_html_changed")`. Codegen читает поле без `state.dev`-гейта. Analyzer уже владеет `dev` (`data.script.dev`, проставляется из `AnalyzeOptions.dev`).

### Расширение входа `block_semantics::build()`

Сигнатура `crates/svelte_analyze/src/block_semantics/builder/mod.rs::build` принимает три новых параметра:

- `&FragmentNamespaces` — для namespace по `current_fragment_id`. Уже доступен как `data.template.fragment_namespaces` (заполняется `template_side_tables::collect_fragment_namespaces` до `BuildBlockSemantics`).
- `&IgnoreData` — для `is_ignored(id, "hydration_html_changed")`. Уже доступен как `data.output.ignore_data` (заполняется `ScanIgnoreComments` до `BuildBlockSemantics`).
- `dev: bool` — для запекания в `hydration_html_changed_ignored`.

Pass `BuildBlockSemantics` (`passes/executor.rs:219-228`) пробрасывает эти аргументы.

### Walker-изменения

`block_semantics/builder/walker.rs::Ctx`:
- Добавить поле `current_fragment_id: FragmentId`. Push/pop в `visit_fragment`.
- Прокинуть `fragment_namespaces: &'c FragmentNamespaces`, `ignore_data: &'c IgnoreData`, `dev: bool`.

`Ctx::visit_node` — добавить arm `Node::HtmlTag(tag) => super::html_tag::populate(self, tag)`.

Новый файл `block_semantics/builder/html_tag.rs`:
```rust
pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &HtmlTag) {
    let parent_strategy = match ctx.fragment_namespaces.get(ctx.current_fragment_id) {
        Some(Namespace::Svg) => HtmlTagNamespace::Svg,
        Some(Namespace::Mathml) => HtmlTagNamespace::MathMl,
        _ => HtmlTagNamespace::Html,
    };
    let hydration_html_changed_ignored =
        ctx.dev && ctx.ignore_data.is_ignored(tag.id, "hydration_html_changed");
    ctx.store.set(tag.id, BlockSemantics::HtmlTag(HtmlTagSemantics {
        parent_strategy,
        hydration_html_changed_ignored,
    }));
}
```

### Codegen dispatch

`crates/svelte_codegen_client/src/codegen/blocks/dispatch.rs:99` сейчас обходит `BlockSemantics` mismatch-check для HtmlTag (единственный block-узел без него). Унифицируется по образцу If/Each/Await/Key/Render/Snippet:

```rust
Node::HtmlTag(_) => {
    let sem = match self.ctx.query.analysis.block_semantics(id) {
        BlockSemantics::HtmlTag(s) => s.clone(),
        _ => {
            return CodegenError::unexpected_block_semantics(id, "HtmlTag expected HtmlTag");
        }
    };
    self.emit_html_tag(state, ctx, id, sem)
}
```

`emit_html_tag` в `codegen/blocks/html_tag.rs` принимает `sem: HtmlTagSemantics` параметром. Удаляются три источника:

```rust
let is_svg = !is_controlled && matches!(sem.parent_strategy, HtmlTagNamespace::Svg);
let is_mathml = !is_controlled && matches!(sem.parent_strategy, HtmlTagNamespace::MathMl);
let hydration_ignored = sem.hydration_html_changed_ignored;
```

`is_controlled` остаётся вычисляемым на лету из `FragmentAnchor` — это fragment-shape, не block-semantics.

### Удаляемые сущности (namespace duplication)

- `crates/svelte_analyze/src/types/data/analysis.rs`: поля `html_tag_in_svg`, `html_tag_in_mathml` (`NodeBitSet`) + конструктор-инициализация + методы `html_tag_in_svg(id)`, `html_tag_in_mathml(id)`.
- `crates/svelte_analyze/src/types/data/codegen_view.rs`: accessors `html_tag_in_svg`, `html_tag_in_mathml`.
- `crates/svelte_analyze/src/passes/bundles.rs`: поле `pending_html_tags` + `take_pending_html_tags`.
- `crates/svelte_analyze/src/passes/template_side_tables.rs`: `pending_html_tags: Vec<(NodeId, FragmentId)>` поле visitor'а; push в `visit_html_tag` (~line 580). Сам `visit_html_tag` остаётся (record_node_parent и др.) — почистить только html-tag-specific код.
- `crates/svelte_analyze/src/passes/executor.rs:149-159`: post-walk join `pending_html_tags` × `fragment_namespaces`.

### Удаляемые сущности (`DebugTagData` cleanup, независимая часть)

- `crates/svelte_analyze/src/types/data/template_data.rs:66-86`: `DebugTagData` целиком (struct + impl Default + impl).
- `crates/svelte_analyze/src/types/data/analysis.rs:96,107`: поле `template.debug_tags` + конструктор-инициализация.
- `crates/svelte_analyze/src/types/data/codegen_view.rs:384-389`: `debug_tags_for_fragment_by_id`.
- `crates/svelte_analyze/src/passes/bundles.rs:20,37-39`: `debug_tag_buckets` + `take_debug_tag_buckets`.
- `crates/svelte_analyze/src/passes/template_side_tables.rs:25,664-670`: поле `debug_tag_buckets`; visitor `visit_debug_tag` целиком (после удаления push'а тело пустое — снести).
- `crates/svelte_analyze/src/passes/executor.rs:132-139`: post-walk join `debug_tag_buckets`.
- `crates/svelte_analyze/src/lib.rs:27`, `types/mod.rs:10`, `types/data/mod.rs:68`: re-export'ы `DebugTagData`.

### Тесты

Compiler-тесты (`tasks/compiler_tests/cases2/html_tag*`, `cases2/html_tag_hydration_ignore`, `cases2/debug_*`) — без изменений. Они и есть гарантия корректности end-to-end (если `parent_strategy` или `hydration_html_changed_ignored` посчитан неправильно, выходной JS поедет, тест упадёт).

Unit-тесты в `crates/svelte_analyze/src/tests.rs` — новый модуль `block_semantics_html_tag_tests` по образцу `block_semantics_each_tests:5934`. Кейсы:

1. `{@html x}` в обычном HTML → `parent_strategy = Html`, `hydration_html_changed_ignored = false`.
2. `<svg>{@html x}</svg>` → `Svg`.
3. `<math>{@html x}</math>` → `MathMl`.
4. `<svg><foreignObject>{@html x}</foreignObject></svg>` → `Html` (foreignObject выходит из svg-namespace).
5. `<!-- svelte-ignore hydration_html_changed --><div>{@html x}</div>` с `dev: true` → `hydration_html_changed_ignored = true`.
6. То же с `dev: false` → `hydration_html_changed_ignored = false` (запечённый dev).
7. Без ignore-pragma в dev → `hydration_html_changed_ignored = false`.

Существующие тесты:
- `tests.rs:2848 html_tag_namespace_flags_preserved` — переписать на чтение `BlockSemantics::HtmlTag.parent_strategy`.
- `tests.rs:2750-2753` (assert на `data.html_tag_in_svg`/`mathml`) — удалить, покрытие достаточно через новые unit-тесты + compiler-тесты.
- `tests.rs:2807-2813 debug_tag_ids_collected_for_fragment` — удалить (читал `data.template.debug_tags.by_fragment_id`, side-table удаляется; покрытие debug-tag поведения остаётся в `cases2/debug_*`).

### Правки документации

- `SEMANTIC_LAYER_ARCHITECTURE.md` строки 162–163 (HtmlTag/DebugTag out-of-scope): HtmlTag → in-scope как `BlockSemantics::HtmlTag`; DebugTag → остаётся out-of-scope с пояснением «AST самодостаточен, variant не нужен; `DebugTagData` side-table удалён».
- `specs/analyzer-target-design.md` Decision 9 (строка 187): `{@html}` — variant `BlockSemantics::HtmlTag` с payload `{ parent_strategy: HtmlTagNamespace, hydration_html_changed_ignored: bool }`. `{@debug}` — variant не создаётся (AST самодостаточен), `DebugTagData` side-table удалён как мёртвый.
- `specs/analyzer-target-design.md` Decision 21 (строка 224): `DebugTagData` — растворяется не в variant'е, а удаляется как side-table.

### Технический долг

`debt.md` — две записи:

- **`HtmlTagSemantics.parent_strategy` без `is_controlled`**. `is_controlled` (controlled-fragment shape — единственный ребёнок RegularElement) остаётся в codegen до спеки 06 «FragmentSemanticsStore». Когда controlled-fact переедет в fragment-кластер, `parent_strategy` либо расширится, либо `is_controlled` останется в fragment-cluster — решить тогда же.
- **`BlockSemanticsBuilder` зависит от `dev: bool`**. После запекания `dev` в `hydration_html_changed_ignored`, результат `BlockSemanticsStore` зависит от compile-time-режима. Сейчас на одну компиляцию один codegen-режим, поэтому ок. Если в будущем потребуется reuse analyze-результата между dev и prod-проходами codegen'а, придётся вынести `dev` обратно в codegen или дублировать analyze.

## Acceptance criteria

- [ ] `BlockSemantics::HtmlTag(HtmlTagSemantics)` вариант существует
- [ ] `HtmlTagSemantics { parent_strategy: HtmlTagNamespace, hydration_html_changed_ignored: bool }` определён
- [ ] `HtmlTagNamespace { Html, Svg, MathMl }` определён
- [ ] `block_semantics::build()` принимает `&FragmentNamespaces`, `&IgnoreData`, `dev: bool`
- [ ] `BuildBlockSemantics` pass пробрасывает новые аргументы
- [ ] Walker `Ctx` отслеживает `current_fragment_id`
- [ ] `block_semantics/builder/html_tag.rs::populate` вычисляет namespace из `fragment_namespaces` и ignore-fact c запечённым `dev`
- [ ] `dispatch.rs` обрабатывает `Node::HtmlTag` через `match BlockSemantics::HtmlTag` (как Each/If/etc.)
- [ ] `emit_html_tag` принимает `sem: HtmlTagSemantics` параметром, читает `sem.parent_strategy` и `sem.hydration_html_changed_ignored`
- [ ] 0 вхождений `is_ignored(_, "hydration_html_changed")` в `crates/svelte_codegen_client/`
- [ ] 0 вхождений `view.html_tag_in_svg` / `view.html_tag_in_mathml` в `crates/svelte_codegen_client/`
- [ ] `data.elements.html_tag_in_svg`, `data.elements.html_tag_in_mathml` (поля + методы) удалены
- [ ] `codegen_view.rs::html_tag_in_svg`, `html_tag_in_mathml` accessors удалены
- [ ] `pending_html_tags` поле visitor'а + `take_pending_html_tags` + post-walk join в `executor.rs` удалены
- [ ] `DebugTagData` struct удалён
- [ ] `analysis.rs::template.debug_tags` поле удалено
- [ ] `codegen_view.rs::debug_tags_for_fragment_by_id` удалён
- [ ] `debug_tag_buckets` field + `take_debug_tag_buckets` + `visit_debug_tag` (если станет пустым) + post-walk join удалены
- [ ] Все re-exports `DebugTagData` (`lib.rs`, `types/mod.rs`, `types/data/mod.rs`) удалены
- [ ] Существующий тест `html_tag_namespace_flags_preserved` переписан на `BlockSemantics::HtmlTag.parent_strategy`
- [ ] Тест `tests.rs:2750-2753` (assert на bitsets) удалён
- [ ] Тест `debug_tag_ids_collected_for_fragment` удалён
- [ ] Unit-тесты `block_semantics_html_tag_tests` (7 кейсов из списка выше) добавлены и зелёные
- [ ] `SEMANTIC_LAYER_ARCHITECTURE.md` строки 162–163 обновлены (HtmlTag → in-scope; DebugTag — пояснение «AST-only, side-table удалён»)
- [ ] `analyzer-target-design.md` Decision 9 и Decision 21 переформулированы согласно реальному решению (DebugTag-variant не создаётся)
- [ ] `debt.md` дополнен двумя записями (`is_controlled` в codegen; `BlockSemanticsBuilder` зависит от `dev`)
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный

## Blocked by

None — can start immediately. `FragmentNamespaces` и `IgnoreData` уже доступны в `AnalysisData` к моменту запуска `BuildBlockSemantics` (текущий pipeline order: `TemplateSideTables` → `ScanIgnoreComments` → ... → `BuildBlockSemantics`).
