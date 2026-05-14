# Удаление `ScriptRuneCalls` + class-field rune-семантика в `DeclaratorSemantics`

## Parent

`specs/analyzer-target-design.md`

## What to build

`ScriptRuneCalls` (per-`OxcNodeId` rune-classification map), pass `passes/js_analyze/script_runes.rs` и весь связанный offset-плумбинг удаляются целиком. Class-field rune-инициализации (`class A { x = $state() }` и `class A { constructor() { this.x = $state() } }`) получают семантику в `DeclaratorSemantics` через два новых варианта: `ClassFieldState(ClassFieldStateSemantics)` и `ClassFieldDerived(ClassFieldDerivedSemantics)`. Запись делает `reactivity_semantics::builder_v2` через новый `visit_class`-обход. Ключ — `OxcNodeId` `PropertyDefinition` для синтаксических полей или `OxcNodeId` `AssignmentExpression` для constructor-присваиваний.

Свободные rune-вызовы (`$effect`/`$effect.pre`/`$effect.root`/`$effect.tracking`/`$inspect`/`$inspect.with`/`$host`/`$state.snapshot` как expression-уровня) распознаются transform-ом локально через `pub`-экспорт `detect_rune_from_call(call)` из `svelte_analyze` — чистая функция от `&CallExpression`, без хранения и кэша.

Transform-сайты, ныне зовущие `rune_kind_from_expr`:
- **Sites 1, 6** (variable-declarator-rune-init): доверяют `binding_semantics(sym)`, fallback на expression-классификацию удаляется. Если builder_v2 пропустил классификацию — это дефект analyzer-а, а не повод для подпорки в transform.
- **Sites 2, 3, 4, 5** (class field, constructor): читают `declarator_semantics(node)` и матчатся по `ClassFieldState`/`ClassFieldDerived`.
- **Site 7** (`rewrite_call_expression`): локальный вызов `detect_rune_from_call(call)`.

Helper-ы `rune_kind_from_expr` и `script_rune_call_node_id` в transform-е удаляются. Вся offset-инфраструктура (`instance_node_id_offset`, `module_node_id_offset`, `script_node_id_offset` плюс соответствующие accessors / проброс через codegen-pipeline) — мёртвый код: после `BuildComponentSemantics.JsSemanticVisitor::set_node_id` каждый OXC-узел уже хранит сквозной глобальный `NodeId`, и offset-сложение симметрично записывалось/читалось без полезного эффекта. Удаляется в этой же спеке.

### Форма новых вариантов `DeclaratorSemantics`

```rust
pub enum DeclaratorSemantics {
    None,
    PropsIdentifier { sym: SymbolId },
    PropsObject { leaves: SmallVec<[SymbolId; 4]>, has_rest: bool },
    LegacyStateDestructure { leaves: SmallVec<[SymbolId; 4]> },
    LetCarrier { carrier_symbol: SymbolId },
    ClassFieldState(ClassFieldStateSemantics),
    ClassFieldDerived(ClassFieldDerivedSemantics),
}

pub struct ClassFieldStateSemantics {
    pub kind: StateKind,
    pub proxied: bool,
}

pub struct ClassFieldDerivedSemantics {
    pub kind: DerivedKind,
    pub lowering: DerivedLowering,
}
```

`StateKind`, `DerivedKind`, `DerivedLowering` переиспользуются из существующих типов (variable-declarator State/Derived). Имя поля / `is_private` / `is_static` в семантику не пишутся — transform читает их из самого `PropertyDefinition` / `MemberExpression` AST-узла (правило: никаких строк в семантиках, никаких булевых дублирующих AST-флагов).

### Edge-cases классификации (1-в-1 как `original/compiler/phases/2-analyze/visitors/ClassBody.js`)

- **Static class field** (`static x = $state()`): не регистрируется. Игнорируется на уровне семантики; диагностика — отдельная зона.
- **Computed key**:
  - `PropertyDefinition` с `computed == true` (включая `["x"] = $state()`): не регистрируется.
  - В constructor `this[<expr>] = $state()`: регистрируется **только** если `<expr>` — string literal (имя берётся из literal-value); иначе игнорируется.
- **`this.x = ...` вне constructor body** (в обычном методе, в arrow внутри constructor-а, во вложенных блоках): игнорируется. Регистрируются ровно top-level `ExpressionStatement`-ы тела constructor-а с формой `this.<key> = <expr>`.
- **Collision (case 3)** — rune-init и в `PropertyDefinition`, и в `this.x = $state()` с тем же именем: эмиттится `Diagnostic::StateFieldDuplicate { name }` (тип уже определён в `svelte_diagnostics`, но до этой спеки нигде не эмиттился). Recovery — builder пишет `ClassFieldState`/`Derived` на `assign.node_id()` и одновременно `DeclaratorSemantics::None` на `prop.node_id()` (last-wins на имя), чтобы transform не получал двух semantic-entry-ей для одного логического слота. **Diagnostic эмиттится не из builder_v2, а из validator-а `validate/class_state_fields.rs`** — отдельный walk по классам, ищет collision. Соответствует Decision 44 парент-спеки («cluster builders диагностики не эмитят»).
- **Sub-cases без коллизии**:
  - Только PropertyDefinition с rune-init (case 1) → `ClassFieldState`/`Derived` пишется на `prop.node_id()`.
  - Только constructor `this.x = $state()` (PropertyDefinition отсутствует или есть как пустой placeholder `x;`) → `ClassFieldState`/`Derived` пишется на `assign.node_id()`. Placeholder PropertyDefinition `x;` остаётся без семантики (`None`); transform позже удалит его (`state.rs:947-950`).

### Класс-обход в `builder_v2`

Один cohesive `visit_class`-метод (а не sub-pass), потому что:
- Данные пишутся в `declarator_semantics`, владельцем которого является `builder_v2`.
- Last-wins-логику коллизии (constructor перезаписывает PropertyDefinition) удобно делать в одном месте, видя оба источника одновременно.
- Единственный обход AST для записи фактов, без дублирующего walker-а.

Алгоритм builder_v2:
1. Пройти `PropertyDefinition`-ы с rune-init (non-static, non-computed): записать `ClassFieldState`/`Derived` на `prop.node_id()`, имя → `prop.node_id()` в локальный `prop_record_by_name`.
2. Найти constructor-метод (если есть). Пройти top-level `ExpressionStatement`-ы. Для `this.<key> = <expr>` (computed-rules как в reference) с rune-init:
   - Если `name ∈ prop_record_by_name` → перезаписать `declarator_semantics[prior_prop_node] = None`, затем записать `ClassFieldState`/`Derived` на `assign.node_id()` (last-wins).
   - Иначе → записать `ClassFieldState`/`Derived` на `assign.node_id()`.

Diagnostic-эмиссия отдельно — в `validate/class_state_fields.rs`. Этот validator делает свой проход по классам и эмиттит `StateFieldDuplicate` при обнаружении collision-имени между PropertyDefinition rune-init и constructor `this.x =` rune-init. Builder-у диагностики не передаются.

### Добавляемый validator

`crates/svelte_analyze/src/validate/class_state_fields.rs` — новый файл. Регистрируется в `validate/mod.rs`, вызывается из `validate_program` при `runes == true`. Walk по классам: PropertyDefinition rune-init собирает в `declared_names`; для constructor `this.<key> = $state(...)` rune-init проверяет `declared_names.contains(name)` → если да, эмиттит `Diagnostic::error(StateFieldDuplicate { name }, assign.span)`.

### Удаляемые сущности

- `crates/svelte_analyze/src/passes/js_analyze/script_runes.rs` целиком.
- `crates/svelte_analyze/src/types/data/script_rune_calls.rs` (модуль и `pub struct ScriptRuneCalls`).
- Поле `ScriptAnalysis.script_rune_calls`, accessor `AnalysisData::script_rune_calls()`.
- Поля `ScriptAnalysis.instance_node_id_offset`, `module_node_id_offset` + сеттеры в `passes/build_component_semantics.rs`.
- `codegen_view.rs` accessors: `script_rune_calls()`, `instance_script_node_id_offset()`, `module_script_node_id_offset()`.
- `svelte_codegen_client/src/script/pipeline.rs` — проброс `script_rune_calls` и offset-полей через `script_node_id_offset`.
- `svelte_codegen_client/src/context.rs` accessor `instance_script_node_id_offset()`.
- В `svelte_transform`: поля `ComponentTransformer.script_rune_calls`, `script_node_id_offset`; helpers `script_rune_call_node_id`, `rune_kind_from_expr`; их инициализация в `entry.rs` / `template_entry.rs`.
- В `svelte_transform/src/transformer/state.rs:81 rune_kind_for_declarator` — fallback-ветка `or_else(...)` уходит, метод сводится к `first_binding_symbol(...).and_then(rune_for_symbol)`.
- Тесты `tests.rs::script_rune_calls_keep_module_and_instance_programs_distinct` и `tests.rs::script_rune_calls_survive_template_node_id_activity`.

### Новые тесты на `ReactivitySemantics`

В `crates/svelte_analyze/src/tests.rs` добавляются:
- class field `$state(0)` → `ClassFieldState { kind: State, proxied: true }` на `prop.node_id()`.
- class field `$state.raw(0)` → `ClassFieldState { kind: StateRaw, proxied: false }`.
- class field `$derived(...)` → `ClassFieldDerived { kind: Derived, lowering: Sync }`.
- class field `$derived.by(...)` → `ClassFieldDerived { kind: DerivedBy, lowering: Sync }`.
- constructor case 2: `x;` placeholder + `this.x = $state()` → `ClassFieldState` на `assign.node_id()`, на `prop.node_id()` — `None`.
- collision case 3: rune-init в обоих → `Diagnostic::StateFieldDuplicate { name: "x" }` эмиттится; последняя запись (constructor) сохраняется в `declarator_semantics`.
- static field — `declarator_semantics(node)` возвращает `None`.
- computed-non-literal в constructor (`this[fn()] = $state()`) — `None`.
- non-constructor `this.x = $state()` (в method body) — `None`.

## Acceptance criteria

- [x] `DeclaratorSemantics` расширен вариантами `ClassFieldState(ClassFieldStateSemantics)`, `ClassFieldDerived(ClassFieldDerivedSemantics)`. Без строковых полей. Без дублирования AST-флагов (`is_private`, `is_static`).
- [x] `builder_v2::visit_class` реализован, поведение точно зеркалит reference (`original/compiler/phases/2-analyze/visitors/ClassBody.js`) для static / computed / non-constructor / collision случаев.
- [x] `Diagnostic::StateFieldDuplicate` эмиттится из `validate/class_state_fields.rs` при collision-case 3 (первая emission-точка в кодовой базе). По Decision 44 парент-спеки cluster builders диагностики не эмитят — collision-detection делает отдельный validator-walk.
- [x] `passes/js_analyze/script_runes.rs` удалён.
- [x] `ScriptRuneCalls` struct, модуль, и все `pub use`-ы удалены.
- [x] Поля `script.script_rune_calls`, `script.instance_node_id_offset`, `script.module_node_id_offset` + соответствующие сеттеры удалены.
- [x] Accessors `data.script_rune_calls()`, `codegen_view.rs::script_rune_calls()`, `instance_script_node_id_offset()`, `module_script_node_id_offset()`, `Context::instance_script_node_id_offset()` удалены.
- [x] `script_node_id_offset`, `script_rune_calls`-плумбинг через `svelte_codegen_client/src/script/pipeline.rs`, `svelte_transform/src/transformer/{entry,model,template_entry}.rs` удалён.
- [x] `script_rune_call_node_id`, `rune_kind_from_expr` в `svelte_transform/src/transformer/state.rs` удалены.
- [x] `rune_kind_for_declarator` упрощён до symbol-only (без expression-fallback).
- [x] `detect_rune_from_call` экспортирован как `pub` из `svelte_analyze`.
- [x] Transform sites 1, 6 (`state.rs:91`, `runes.rs:60`): без fallback на expression-классификацию.
- [x] Transform sites 2, 3, 4, 5 (`state.rs:816, 861, 940, 1002`): читают `declarator_semantics(node)` и матчатся по `ClassFieldState`/`ClassFieldDerived`.
- [x] Transform site 7 (`runes.rs:282`): локальный `detect_rune_from_call(call)`.
- [x] Старые тесты `script_rune_calls_keep_module_and_instance_programs_distinct`, `script_rune_calls_survive_template_node_id_activity` удалены.
- [x] Новые unit-тесты на `ReactivitySemantics` (см. список выше) добавлены и зелёные.
- [x] `just test-compiler` зелёный.
- [x] `just test-diagnostics` зелёный (включая parity-case `tasks/diagnostic_tests/cases/runes/state_field_duplicate/` против reference `svelte/compiler`).
- [x] `just clippy-strict` зелёный.
- [x] `debt.md` обновлён: запись про `ScriptRuneCalls` снята; offset-инфраструктура снята.

## Blocked by

None — can start immediately.
