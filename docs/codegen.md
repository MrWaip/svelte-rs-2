# PRD: Кодген (корневой)

label: codegen
topics: codegen, emit, dumb codegen, JS printing, template/from_html, hoisted, runtime calls $.*, fragment/blocks/attributes visitors

Корневой PRD для слоя кодгена (`svelte_codegen_client`).
Догма: **dumb codegen** — один запрос к анализу на один use case → одно однозначное решение эмита.

## Назначение

Обходит (проанализированный + трансформированный) AST и эмитит Svelte runtime JS. Без пере-walk'ов за смыслом, без сшивания compound-флагов.

## Inputs / outputs

- Входы: `&Component` (template AST), `&AnalysisData`, трансформированные `oxc::Program`-ы.
- Выход: `oxc::Program` Svelte runtime client JS, готовый к печати.

## Архитектурные инварианты

1. **Нет нового анализа.** Нет пере-walk'ов AST для переклассификации узлов.
2. **Нет диагностик.**
3. **Codegen всё ещё владеет layout-only вычислениями**, зависящими от окружающего emit-контекста (sibling-layout, anchors, ident-generation). Это не «анализ смысла» — локальные printing-решения.

## Локальные emit-time анализы

Один анализ-образный проход, живущий в кодгене by design — зависит от emit-контекста (preserve_whitespace, окружающие теги), важен только для печати.

**Fragment prepare** — `crates/svelte_codegen_client/src/codegen/fragment/prepare.rs`. За один проход по детям фрагмента: hoist структурных узлов в bucket; trim/normalize whitespace по правилам Svelte; coalesce смежных `Text` + `{expression}` в один `Concat`; классификация детей в `ContentStrategy` (`Empty`, `SingleStatic`, `SingleExpr`, `SingleConcat`, `SingleElement`, `SingleBlock`, `Multi { … }`), чтобы родитель-эмиттер взял одну ветку без доп-инспекции.

## Anchors

Кодген трекает текущий **fragment anchor** — DOM-референс, относительно которого дети append/insert. `FragmentAnchor`-варианты (`data_structures.rs`): `Root` (`$$anchor`-параметр), `CallbackParam { name, append_inside }` (тело блока), `Child { parent_var }`, `SiblingVar { var }`. Handling — в `codegen/anchor.rs`. Block-эмиттеры резервируют anchor-ident'ы up-front (`pending_anchor_idents`) и коммитят при материализации comment-anchor'а.

## Анти-паттерны

- Пере-walk AST за фактом, которым анализ уже владеет.
- Compound-ветвление на raw-bool'ах / индексах для выбора emit-формы — решение в анализ как именованный вариант.
- Эмит диагностик из кодгена.
- Перезапуск fragment-prepare-проходов вне кодгена (emit-context-specific).

## Связь с другими документами

- `context.md` §«Догмы», §«Hoisting», §«Эмит-форма семантики», §«Анализ в кодгене».
- `block-semantics.md`, `attribute-semantics.md`, `expression-semantics.md` — поставщики готовых emit-решений.
- `transform.md` — даёт трансформированные `oxc::Program`-ы.
- `supporting-crates.md` — `svelte_ast_builder`, `svelte_emit_builders`, `svelte_transform_css`.
- `compiler.md` — место кодгена в пайплайне.
