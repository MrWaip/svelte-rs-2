# `FragmentSemanticsStore` utility store

## Parent

`specs/analyzer-target-design.md`

## What to build

Новый utility store `FragmentSemanticsStore` рядом с `TemplateTopology` / `TemplateElementIndex`. Не «кластер per Svelte kind» (Decision 1 не нарушается). Хранит per-fragment context-bundle, накапливаемый сверху-вниз по template-walk-у.

Поля:

- `preserve_ws: bool` (composition `preserve_whitespace || inside_pre || inside_textarea || inside_script` родителя)
- `inside_pre: bool`
- `inside_textarea: bool`
- `inside_script: bool`
- `inside_head: bool`
- `namespace: Namespace` (Html / Svg / MathML)
- `parent_tag: Option<&str>` (имя родительского HTML-тега для специальных правил вроде `parent="pre"` whitespace-strip)
- `needs_text_first_next: bool` (derive из `FragmentRole`)

`children_summary` фрагмента (`has_children`, `non_trivial_child_count`, `has_expression_child`, `has_animate_child`, `has_rich_content_by_id`) сворачивается в payload родителя fragment-а (`Element` / `Block` / Component-root) — не в `FragmentSemanticsStore`, а в кластеры родителя через built-in summary поле.

`FragmentSemanticsBuilder` — отдельный walk сверху-вниз с накоплением context-флагов в Phase 3. Не объединяется с `TemplateIndicesBuilder` (cohesion: один index, один context-bundle).

Codegen `prepare`/`FragmentCtx` перестаёт протаскивать флаги через рекурсию — читает `data.fragments.get(fid)` одним лукапом на каждый fragment-вход.

`FragmentFacts`, `RichContentFacts`, `FragmentNamespaces`, `fragment_blockers` удаляются.

Decision basis: §22 + §51.

## Acceptance criteria

- [ ] `FragmentSemanticsBuilder` существует и регистрируется в Phase 3 как отдельный walk
- [ ] `FragmentSemanticsStore` — top-level поле `AnalysisData::fragments`
- [ ] Все поля context-bundle (preserve_ws, inside_pre/textarea/script/head, namespace, parent_tag, needs_text_first_next) пред-вычислены
- [ ] `children_summary` свёрнут в payload родителя fragment-а в кластерах Element / Block
- [ ] Codegen `prepare`/`FragmentCtx` читает `data.fragments.get(fid)` вместо протаскивания флагов через рекурсивный context
- [ ] `FragmentFacts` удалён
- [ ] `RichContentFacts` удалён
- [ ] `FragmentNamespaces` удалён
- [ ] `fragment_blockers` side-table удалён
- [ ] `passes/fragment_topology.rs` и часть `passes/template_side_tables.rs` поглощены `TemplateIndicesBuilder` (если ещё не сделано в более ранних слайсах)
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

- `05-element-semantics-cluster.md`
