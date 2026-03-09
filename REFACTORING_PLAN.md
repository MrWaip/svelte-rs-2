# Рефакторинг Svelte-RS: План и прогресс

### Stage 3c: Тесты в стиле cases2 ✅
### Stage 3d-1: Text whitespace trimming ✅
### Stage 3d-2: Dynamic text API ✅
### Stage 3d-3: else-if codegen ✅
### Stage 3d-4: Whitespace between siblings in templates ✅

### Stage 3d-5: Boolean attribute format + shorthand attr as dynamic ⏳
Тесты: `element_attributes`
Проблема: `visible` → должно быть `visible=""` в шаблоне; `{description}` shorthand не попадает в `$.set_attribute`

### Stage 3d-6: `$.proxy` + TypeScript stripping ⏳
Тесты: `state_runes`
Проблема: объектные `$state({})` / `$state(varRef)` → `$.proxy(...)`, TypeScript type declarations попадают в output

### Stage 3d-7: bind directive codegen ⏳
Тесты: `bind_directives`
Проблема: `$.bind_value`, `$.remove_input_defaults`, `$.state` для rune-переменных

### Stage 3d-8: Element children traversal ⏳
Тесты: `nested_resets`, `elements_childs`
Проблема: `textContent =` для single-expr children, `$.reset`, правильный обход child/sibling

### Stage 3d-9: each block with Mixed content ⏳
Тесты: `each_block`
Проблема: Mixed content type внутри each item body

### Stage 3d-10: smoke (интеграция) ⏳
Тесты: `smoke`, `elements_childs`
После исправления всего выше

---

### Stage 3e: Миграция cases/ → cases2/

Для каждой группы: скопировать `case.svelte` в `cases2/<name>/`, сгенерировать свежий `case-svelte.js` официальным Svelte, добавить тест в `test_v3.rs`, починить codegen до зелёного теста.

**Stage 3e-1: Script & imports** ⏳
Кейсы: `only_script`, `hoist_imports`

**Stage 3e-2: Static content** ⏳
Кейсы: `static_attributes`, `static_interpolation`, `only_compressible_nodes`, `element_without_new_line`

**Stage 3e-3: Rune interactions** ⏳
Кейсы: `access_to_muted_state_rune`, `interpolation_rune_handling`, `attibute_rune_handling`, `rune_update`, `assign_in_template`

**Stage 3e-4: Directives** ⏳
Кейсы: `class_directive`

**Stage 3e-5: Misc / complex** ⏳
Кейсы: `element_with_interpolation`, `big`

**Stage 3e-6: Evaluate potential dupes** ⏳
Кейсы из cases/: `single_text`, `single_element_node`, `if_block`, `if_else` — сравнить с имеющимися cases2/, мигрировать если есть отличия
