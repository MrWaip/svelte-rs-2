# `AttributeSemantics` cluster

## Parent

`specs/analyzer-target-design.md`

## What to build

Новый кластер `AttributeSemantics`. Идентичность — `NodeId` attribute/directive-узла. Любая работа с атрибутом начинается с одного варианта enum `data.attributes.get(attr_id)`.

Кластер строится по правилу **«variant = pre-computed decision»**: variant создаётся только тогда, когда у потребителя есть compose-логика на ≥2 источника (Decision 50 parent PRD). Любой attribute, для которого решение тривиально-derive-уемо из AST, в кластер не заводится — codegen матчится по AST-kind в `NonSpecial`-ветке.

### Scope

`AttributeSemanticsBuilder` покрывает все attribute/directive-сайты компонента (HTML element / Component / SvelteWindow / SvelteDocument / SvelteBody / SvelteBoundary). Параллельно `ExpressionSemanticsBuilder` расширяется и обслуживает все expression-сайты, на которые ссылаются attribute payload-ы:

- attribute value-expression (`<div foo={expr}>`)
- attribute concat-parts (`<div foo="a {expr}">`)
- spread (`<div {...expr}>`)
- bind value (`bind:value={expr}` на любом host-е)
- event handler (`on:click={expr}`, `onclick={expr}`)
- use/transition/animate argument
- attach value
- class/style directive expression
- boundary handler value (`<svelte:boundary onerror={expr}>`)

**Out of scope для `ExpressionSemanticsBuilder`:** block-defining expressions (each-collection, if-condition, await-promise, key, html-tag, render-args, const-tag-init). Они живут под `BlockSemantics` payload-ом — async_kind/blockers/store/etc. там уже зафиксированы и в `ExpressionSemantics` не дублируются.

### Public payload shape

```rust
pub enum AttributeSemantics {
    #[default]
    NonSpecial,

    ElementBind(ElementBindSemantics),
    WindowBind(WindowBindSemantics),
    DocumentBind(DocumentBindSemantics),
    ComponentBind(ComponentBindSemantics),

    Event(EventSemantics),
    ComponentProp(ComponentPropSemantics),
    BoundaryProp(BoundaryPropSemantics),
}
```

Bind на разных host-ах — отдельные variants. Property-enum в каждом — узкий per-host (Window-host не может нести Element-property и наоборот → невалидные комбинации запрещены типом). `HtmlBindKind` общий — bind может идти на store-subscribed expression вне зависимости от host-а.

```rust
pub struct ElementBindSemantics {
    pub property: ElementBindPropertyKind,
    pub expression: NodeId,
    pub kind: HtmlBindKind,
    pub blockers: SmallVec<[u32; 2]>,
    pub parent_each_blocks: SmallVec<[NodeId; 4]>,    // только для group-варианта
    pub group_value_attr: Option<NodeId>,             // только для group-варианта
}

pub struct WindowBindSemantics {
    pub property: WindowBindKind,
    pub expression: NodeId,
    pub kind: HtmlBindKind,
    pub blockers: SmallVec<[u32; 2]>,
}

pub struct DocumentBindSemantics {
    pub property: DocumentBindKind,
    pub expression: NodeId,
    pub kind: HtmlBindKind,
    pub blockers: SmallVec<[u32; 2]>,
}

pub enum HtmlBindKind {
    Plain,
    StoreSubscribed { store_symbol: SymbolId },
}
```

`ElementBindPropertyKind` — узкое подмножество текущего `BindPropertyKind` без Window/Document/ComponentProp вариантов (Value/Checked/Group/Files/Indeterminate/Open/This/ContentEditable/ElementSize/ResizeObserver/Media/ImageNaturalSize/Focused). `WindowBindKind`/`DocumentBindKind` — текущие узкие enum-ы из `template_data.rs`. `<svelte:body>` не несёт bind-директив (reference forbids), variant не вводится — только event-handler-ы попадают в `Event`-variant.

```rust
pub struct ComponentBindSemantics {
    pub expression: NodeId,
    pub kind: ComponentBindKind,
    pub each_context_vars: SmallVec<[SymbolId; 4]>,
}

pub enum ComponentBindKind {
    Expression,
    Identifier { symbol: SymbolId },
    This { symbol: Option<SymbolId> },
}
```

`each_context_vars` — список each-context symbol-ов, references на которые сидят в expression-е bind-а. Заменяет helper `bind_each_context(id)`. Кладётся только на ComponentBind (единственный consumer — `bind_this.rs` на компоненте). `parent_each_blocks` для bind:group — на ElementBind (отдельное решение, отдельная shape).

```rust
pub struct EventSemantics {
    pub modifiers: EventModifier,
    pub emit: EventEmit,
}

pub enum EventEmit {
    HtmlDelegated { expression: NodeId, handler_emit: HandlerEmit },
    HtmlDirect    { expression: NodeId, handler_emit: HandlerEmit },
    HtmlBubble,
    Component     { expression: NodeId, handler_emit: HandlerEmit },
}

pub enum HandlerEmit {
    Direct,
    Wrapped,
}
```

`EventEmit` — сводный enum. Невалидные комбинации (Bubble + handler) запрещены типом (паттерн EpilogueKind из RuntimePlan, Decision 47 parent PRD). `HandlerEmit` — pre-computed решение «оборачивать ли handler в стрелку», поглощает helper-ы `attr_is_function` + `attr_is_import` + AST-shape матч (Identifier vs Arrow vs Function vs other) + `dev`-flag. Сырых флагов наружу не отдаём.

`EventModifier` — текущий bitflags-тип, переезжает в payload без изменений.

```rust
pub enum ComponentPropSemantics {
    Expression(ComponentPropExpressionSemantics),
    Concat(ComponentPropConcatSemantics),
}

pub struct ComponentPropExpressionSemantics {
    pub expression: NodeId,
    pub memo: ComponentPropMemo,
    pub shorthand: bool,
}

pub struct ComponentPropConcatSemantics {
    pub memo: ComponentPropMemo,
}

pub enum ComponentPropMemo {
    Inline,
    Getter,
    Derived,
}
```

Static / Boolean component-prop — `NonSpecial`. Codegen в `component_props/dispatch.rs` под NonSpecial-веткой матчит AST (`StringAttribute` / `BooleanAttribute`) и эмитит без semantic-лукапа.

```rust
pub struct BoundaryPropSemantics {
    pub emit: BoundaryPropEmit,
}

pub enum BoundaryPropEmit {
    KeyValue,
    Getter,
}
```

`BoundaryPropEmit::Getter` — pre-computed композиция `is_dynamic || is_import`, поглощает текущий код в `containers/svelte_boundary.rs`. Сырых флагов наружу не отдаём.

### Variants, которые НЕ заводятся

| Attribute / directive | Почему |
|---|---|
| `<div foo="x">` (StringAttribute на element) | shape тривиально-derive-уем из AST, codegen читает `value_span` |
| `<div foo>` (BooleanAttribute) | то же |
| `<div foo={x}>` (ExpressionAttribute, не event) | facts живут в `ExpressionSemantics` по NodeId expression-а |
| `<div foo="a {x}">` (ConcatenationAttribute) | parts в AST, expression-факты в `ExpressionSemantics` |
| `<div {...x}>` / `<Comp {...x}>` (Spread) | один NodeId, никакой композиции |
| `class:foo={x}` (ClassDirective per-attr) | name/expression в AST; per-element агрегация — в `ElementSemantics` (spec 05) |
| `style:foo={x}` (StyleDirective per-attr) | то же; per-element агрегация — в `ElementSemantics` |
| `use:action={x}` / `transition:t={x}` / `animate:a={x}` | name_ref + expression в AST; transition direction/global тривиально-derive-уемы |
| `attach:foo={x}` | один NodeId, никакой композиции |

Codegen для всех этих случаев матчит `AttributeSemantics::NonSpecial`-ветку и идёт в sub-match по AST-kind.

### Consumer contract

**Один dispatch-сайт** на каждый consumer-context. `data.attributes.get(...)` вызывается **только** в этих файлах:

- `crates/svelte_codegen_client/src/codegen/attributes/dispatch.rs` — HTML element / SvelteElement / SvelteWindow / SvelteDocument / SvelteBody.
- `crates/svelte_codegen_client/src/codegen/component_props/dispatch.rs` — Component / SvelteSelf / SvelteComponent.
- `crates/svelte_codegen_client/src/codegen/containers/svelte_boundary.rs` — SvelteBoundary attributes.
- `crates/svelte_transform/src/transformer/template_rewrites.rs` — pickled-await rewrite (через top-level `pickled_awaits`-set, не через `data.attributes`).

Sub-emitter функции (`emit_element_bind`, `emit_event`, `emit_component_prop_expression`, `emit_boundary_prop`, …) **принимают payload-структуру параметром** и AST-узел отдельно. Внутри sub-emitter-а **запрещено**:

- повторно вызывать `data.attributes.get(...)`;
- читать helper-ы `ctx.attr_*` / `ctx.bind_*` / `ctx.expr_*` / `ctx.event_handler_mode` / `ctx.event_modifiers` (helper-ы удалены, см. cleanup).

При нарушении инварианта (variant payload не соответствует AST-kind, например ElementBind на ComponentNode) consumer возвращает `CodegenError::semantic_mismatch(...)`. Никаких `unreachable!()`, silent `return`-ов, fallback-веток.

Пример dispatch-а в `attributes/dispatch.rs`:

```rust
for attr in attributes {
    match data.attributes.get(attr.id()) {
        AttributeSemantics::ElementBind(b)  => self.emit_element_bind(state, owner_id, owner_var, b, attr)?,
        AttributeSemantics::WindowBind(b)   => self.emit_window_bind(state, owner_var, b)?,
        AttributeSemantics::DocumentBind(b) => self.emit_document_bind(state, owner_var, b)?,
        AttributeSemantics::BodyBind(b)     => self.emit_body_bind(state, owner_var, b)?,
        AttributeSemantics::Event(ev)       => self.emit_event(state, owner_id, owner_var, ev, attr)?,

        AttributeSemantics::ComponentBind(_)
        | AttributeSemantics::ComponentProp(_)
        | AttributeSemantics::BoundaryProp(_) => {
            return CodegenError::semantic_mismatch(attr.id(), "component-only semantics on HTML element");
        }

        AttributeSemantics::NonSpecial => match attr {
            Attribute::StringAttribute(a)        => self.emit_static_attribute(state, owner_id, owner_var, a)?,
            Attribute::BooleanAttribute(a)       => self.emit_boolean_attribute(state, owner_var, a),
            Attribute::ExpressionAttribute(a)    => self.emit_expression_attribute(state, owner_id, owner_var, a)?,
            Attribute::ConcatenationAttribute(a) => self.emit_concat_attribute(state, owner_id, owner_var, a)?,
            Attribute::UseDirective(d)           => self.emit_use_directive(state, owner_id, owner_var, d)?,
            Attribute::TransitionDirective(d)    => self.emit_transition_directive(state, owner_id, owner_var, d)?,
            Attribute::AnimateDirective(d)       => self.emit_animate_directive(state, owner_id, owner_var, d)?,
            Attribute::AttachTag(a)              => self.emit_attach_tag(state, owner_id, owner_var, a)?,
            Attribute::OnDirectiveLegacy(d)      => self.emit_on_directive_legacy(state, owner_id, owner_var, d)?,
            Attribute::SpreadAttribute(_)
            | Attribute::ClassDirective(_)
            | Attribute::StyleDirective(_)       => continue, // aggregated separately at element-level
            Attribute::BindDirective(_)
            | Attribute::LetDirectiveLegacy(_)   => return CodegenError::semantic_mismatch(attr.id(), "..."),
        }
    }
}
```

### Builder

```text
crates/svelte_analyze/src/attribute_semantics/
├── mod.rs                         pub use
├── data.rs                        AttributeSemantics + payload structs
└── builder/
    ├── mod.rs                     pub fn build(...) -> AttributeSemanticsStore
    ├── walker.rs                  template walker
    ├── element_bind.rs            ElementBind / Window / Document / Body classify
    ├── component.rs               ComponentBind / ComponentProp classify
    ├── event.rs                   Event classify (handler_emit + EventEmit)
    └── boundary.rs                BoundaryProp classify
```

Сигнатура:

```rust
pub fn build<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    component_semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    expressions: &ExpressionSemanticsStore,
    blocker_data: &BlockerData,
    topology: &TemplateTopology,
    node_count: u32,
    dev: bool,
) -> AttributeSemanticsStore;
```

Builder работает один template walk. Зависимость на `ExpressionSemanticsStore` фиксирует порядок Phase 4: Block → Expression → Attribute → Element. `dev`-флаг нужен для `HandlerEmit`/`BoundaryPropEmit` композиций.

Builder **не читает** факты из старых passes (`ElementFlags`, `BindSemanticsData`, etc.) — вся compose-логика дублируется внутри builder-а. После завершения миграции старые passes удаляются вместе со своими сторами.

### Pickled await rewrite

`PickledAwaitOffsets` (span-keyed `FxHashSet<u32>`) удаляется. Заменяется на top-level поле `AnalysisData`:

```rust
pub struct PickledAwaits {
    node_ids: FxHashSet<OxcNodeId>,
}
impl PickledAwaits {
    pub fn contains(&self, id: OxcNodeId) -> bool;
}
```

Transform `template_rewrites.rs`:

```rust
let is_pickled = analysis.pickled_awaits.contains(await_expr.node_id());
```

`ExprKind::Async.is_pickled` поле удаляется (не имеет production-consumer-а; тесты переписываются на чтение top-level set-а).

### Cleanup в этом спринте

После миграции consumer-ов удаляются:

- `crates/svelte_analyze/src/types/data/expr.rs` целиком (`ExpressionInfo`, `ExpressionKind`, `ExprRole`, `ExprSite`, `ExprDeps`).
- `crates/svelte_analyze/src/types/data/pickled_await_offsets.rs` (`PickledAwaitOffsets`).
- `crates/svelte_analyze/src/types/data/directive_modifier_flags.rs` (тип `DirectiveModifierFlags` уезжает в payload `EventSemantics`).
- `BindSemanticsData` (`crates/svelte_analyze/src/types/data/template_data.rs`) — все поля растворены в `ElementBind*`/`ComponentBind`/`Window/Document/BodyBind` payload-ах.
- `TemplateSemanticsData::node_ref_symbols` — мигрирует на `ExpressionSemantics::references` (для NodeId expression-а).
- `crates/svelte_analyze/src/passes/js_analyze/expression_info.rs`.
- `crates/svelte_analyze/src/passes/bind_semantics.rs`.
- `merge_concat_expression_info` и shadow-aggregation в `passes/collect_symbols.rs`.
- Helper-методы на `Ctx`/`AnalysisData`: `expr_deps`, `expr_role`, `expr_is_async`, `expr_has_await`, `expr_has_blockers`, `attr_expression`, `attr_expression_blockers`, `attr_is_function`, `attr_is_import`, `bind_target_semantics`, `bind_target_symbol`, `bind_each_context`, `bind_directive`, `bind_blockers`, `bind_group_value_attr`, `parent_each_blocks`, `event_modifiers` (helper), `event_handler_mode` (helper), `is_pickled_await(offset: u32)`.
- `ExprKind::Async.is_pickled` поле.
- `attr_expressions: NodeTable<ExpressionInfo>` поле `AnalysisData`.

Decision basis: §8 (cluster ownership), §10 (async as field), §18 (ExpressionInfo dissolves), §19 (BindSemanticsData/TemplateSemanticsData/DirectiveModifierFlags растворяются), §24 (PickledAwaitOffsets), §50 (precompute rule), §52 (helper-mapping).

## Acceptance criteria

### Builder

- [ ] `AttributeSemanticsBuilder` существует и регистрируется в Phase 4 после `ExpressionSemanticsBuilder`
- [ ] `AttributeSemanticsStore` — top-level поле `AnalysisData::attributes`
- [ ] Variants реализованы: `NonSpecial`, `ElementBind`, `WindowBind`, `DocumentBind`, `ComponentBind`, `Event`, `ComponentProp`, `BoundaryProp`
- [ ] Каждый variant — реальное pre-computed решение (variant без compose не вводится)
- [ ] AST-зеркальные variants (Attach/Use/Transition/Animate/Spread/ClassDirective/StyleDirective/StringAttribute/BooleanAttribute/ConcatenationAttribute/ExpressionAttribute-без-события) НЕ создаются
- [ ] Имена атрибутов / событий / классов / style-properties / actions НЕ хранятся в payload
- [ ] `ElementBindSemantics` несёт `parent_each_blocks` и `group_value_attr` (только для group-варианта)
- [ ] `ComponentBindSemantics::This { symbol: Option<SymbolId> }` хранит resolved symbol для bind:this
- [ ] `EventSemantics` несёт `EventEmit` (HtmlDelegated/HtmlDirect/HtmlBubble/Component) и `modifiers: EventModifier`
- [ ] `HandlerEmit { Direct, Wrapped }` поглощает композицию `attr_is_function` + `attr_is_import` + AST-shape + `dev`-флага
- [ ] `BoundaryPropEmit { KeyValue, Getter }` поглощает композицию `is_dynamic || is_import`
- [ ] `ComponentPropSemantics` — два variants (`Expression`, `Concat`); Static/Boolean идут в `NonSpecial`
- [ ] Builder не читает факты из старых passes (`ElementFlags`, `BindSemanticsData`, …) — все compose-derives дублируются внутри builder-а
- [ ] Юнит-тесты на каждый variant `AttributeSemanticsBuilder` (TDD)

### ExpressionSemantics scope expansion

- [ ] `ExpressionSemanticsBuilder` заходит в attribute / directive value-expressions, attribute concat-parts, spread, bind value, event handler, use/transition/animate argument, attach value, class/style directive expression, boundary handler value
- [ ] `ExpressionSemanticsBuilder` НЕ заходит в block-defining expressions (each-collection, if-condition, await-promise, key, html-tag, render-args, const-tag-init)
- [ ] Юнит-тесты на новые expression-сайты

### Pickled awaits

- [ ] `PickledAwaitOffsets` (span-keyed `FxHashSet<u32>`) удалён
- [ ] `pickled_awaits: PickledAwaits` (OxcNodeId-keyed `FxHashSet`) — top-level поле `AnalysisData`
- [ ] Transform `template_rewrites.rs` лукапит per `OxcNodeId`
- [ ] `ExprKind::Async.is_pickled` поле удалено

### Consumer migration

- [ ] `data.attributes.get(id)` вызывается **только** в `attributes/dispatch.rs`, `component_props/dispatch.rs`, `containers/svelte_boundary.rs`
- [ ] Sub-emitter функции принимают payload-структуру параметром, не делают повторных лукапов
- [ ] Top-level dispatch — exhaustive match по `AttributeSemantics`, ветка `NonSpecial` — sub-match по AST-kind
- [ ] При несоответствии payload-а AST-kind-у consumer возвращает `CodegenError::semantic_mismatch(...)` — никаких `unreachable!()`, silent `return`-ов, fallback-веток

### Cleanup

- [ ] `BindSemanticsData` удалён
- [ ] `TemplateSemanticsData` удалён
- [ ] `DirectiveModifierFlags` (тип) удалён, `EventModifier` живёт внутри `EventSemantics`
- [ ] `ExpressionInfo`, `ExpressionKind`, `ExprRole`, `ExprSite`, `ExprDeps` удалены
- [ ] `attr_expressions: NodeTable<ExpressionInfo>` поле `AnalysisData` удалено
- [ ] `passes/js_analyze/expression_info.rs` удалён
- [ ] `passes/bind_semantics.rs` удалён
- [ ] `merge_concat_expression_info` удалён
- [ ] 0 helper-методов с префиксом `expr_` на `Ctx`/`AnalysisData`
- [ ] 0 helper-методов с префиксами `bind_` / `attr_` на `Ctx`/`AnalysisData`
- [ ] `parent_each_blocks(id)` accessor удалён
- [ ] `event_modifiers(id)` accessor удалён (поле `EventSemantics::modifiers` вместо)
- [ ] `event_handler_mode(id)` accessor удалён (поле `EventSemantics::emit` вместо)
- [ ] `attr_expression(id)` accessor удалён
- [ ] `is_pickled_await(offset: u32)` accessor удалён
- [ ] 0 чтений `ExpressionInfo` (как типа) из codegen и transform

### Test gates

- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный

## Migration sequence

1. **Stage 1 — Builder (TDD).** По `/tdd` для каждого variant: red unit-тест на форму payload → green impl. Параллельно `ExpressionSemanticsBuilder` scope expand. Старые passes продолжают работать, codegen consumer-ов не трогаем.
2. **Stage 2 — Consumer migration.** Переключаем consumer-ов на `data.attributes.get(id)`. Каждый коммит — `just test-compiler && test-diagnostics && clippy-strict` зелёные. Порядок: transform pickled rewrite → boundary container → component_props/dispatch → attributes/dispatch.
3. **Stage 3 — Legacy removal.** После того как ни один consumer не читает старые helper-ы / стора, удаляем их (см. Cleanup acceptance criteria).
4. **Stage 4 — Spec checkboxes.** Отмечаем чекбоксы в этом файле.

## Blocked by

- `03-expression-semantics-cluster.md`

## Unblocks

- `05-element-semantics-cluster.md` — `ElementSemantics` поглощает per-element агрегации (class_directive_info, style_directives, event_handler_mode для element-flags-консьюмеров вне attribute-pipeline-а), часть из которых в spec 04 остаётся за рамками.
