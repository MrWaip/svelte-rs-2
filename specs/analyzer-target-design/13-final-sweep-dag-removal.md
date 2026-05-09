# Final sweep: orphan passes deletion + DAG resolver removal + `analyze()` rewrite

## Parent

`specs/analyzer-target-design.md`

## What to build

Финальный sweep — удаление всего наследия passes-эпохи и переписка `analyze()` на 6 явных фаз без условной логики выбора порядка.

Удаляются целиком:

- `passes/bind_semantics`
- `passes/build_component_semantics`
- `passes/bundles`
- `passes/collect_symbols`
- `passes/content_types`
- `passes/dynamism`
- `passes/element_flags`
- `passes/executor`
- `passes/fragment_topology`
- `passes/js_analyze` (все 6 sub-passes)
- `passes/post_resolve`
- `passes/template_side_tables`

Удаляется DAG-резолвер `passes/mod.rs` целиком: `PassKey`, `DataToken`, `PassDescriptor`, `resolve_execution_order`, `default_stage_execution_order`, `PRE_TEMPLATE_SCRIPT_STAGE`, `INDEX_BUILD_STAGE`, `POST_TEMPLATE_ANALYSIS_STAGE`, `TEMPLATE_EXECUTION_STAGE`, `VALIDATION_STAGE`.

`analyze()` переписывается как линейный список из 6 фаз с фиксированным порядком, выраженным через явные вызовы builder-функций. Зависимости между фазами обеспечиваются Rust типизацией (input — выход предыдущей фазы):

1. Phase 1 — scope & scripts
2. Phase 2 — reactivity
3. Phase 3 — template-walk indices
4. Phase 4 — clusters
5. Phase 5 — global rollup
6. Phase 6 — checker (отдельный entrypoint в crate `svelte_check`)

`utils/` остаётся как shared библиотека helper-ов (`script_info`, `simple_expression`, `attributes`, `binding_pattern`, `ce_config`, `events`, `html_tree_validation`, `ident_gen`, `legacy_slot`, `property_key`, `var_decl_kind`).

Этот слайс проверяет финальные acceptance criteria PRD-уровня (см. секцию "Финальные acceptance criteria" в spec'е).

Decision basis: §11 + §30 + §31 + §32 + §38.

## Acceptance criteria

- [ ] `crates/svelte_analyze/src/passes/` каталог удалён целиком
- [ ] DAG-резолвер `passes/mod.rs` удалён (PassKey/DataToken/PassDescriptor/`resolve_execution_order`/stages)
- [ ] `analyze()` — линейный список 6 фаз без условной логики выбора порядка
- [ ] Зависимости между фазами проверяются Rust типизацией, не runtime-валидатором
- [ ] `utils/` сохранён
- [ ] `AnalysisData` имеет ровно 15 top-level полей (scoping, reactivity, blocks, attributes, elements, expressions, fragments, topology, element_index, script_declarations, blockers, custom_element, runtime, css, component_name)
- [ ] 0 ad-hoc helper-методов на `AnalysisData` сверх accessor-ов к этим полям
- [ ] 0 helper-методов с префиксами `is_` / `has_` / `needs_` / `class_` / `static_` / `attr_` / `bind_` / `expr_` / `directive_` на `Ctx`/`AnalysisData`
- [ ] 0 чтений `ExpressionInfo` (как типа) из codegen и transform
- [ ] 0 чтений deprecated side-tables (`BindSemanticsData`, `DynamismData`, `ElementFlags`, etc.) из любого внешнего крейта
- [ ] 0 чтений `IgnoreData` (как типа) из `svelte_codegen_client` / `svelte_transform`
- [ ] 0 случаев `view.is_ignored(id, "code-string")` в codegen / transform
- [ ] 0 AST-classification walks в `svelte_transform` для rune-распознавания
- [ ] 0 ИЛИ-цепочек в `svelte_codegen_client/src/lib.rs::generate` для решений уровня "shape компонент-функции"
- [ ] 0 AST-walks в codegen за component-level фактами (`has_bubble_events`, `has_legacy_slots`)
- [ ] `CODEBASE_MAP.md` обновлён под финальную форму
- [ ] `SEMANTIC_LAYER_ARCHITECTURE.md` обновлён
- [ ] `debt.md` — все строки про удаляемые side-tables / passes сняты; долгосрочный пункт про parent-link в svelte_ast остаётся
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный

## Blocked by

- `01-script-rune-calls-side-output.md`
- `06-fragment-semantics-store.md`
- `07-runtime-plan-extension.md`
- `09-svelte-check-crate.md`
- `10-css-analysis-builder.md`
- `11-phase-1-narrow-builders.md`
- `12-codegen-view-component-scoping-removal.md`
