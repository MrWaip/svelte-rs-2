# Kill FragmentItem

Infrastructure slice. Устраняет `FragmentItem` как codegen-диспатчер и перекладывает fragment-level работу по правильным владельцам, плюс вводит новый модуль `codegen/`, изолированный от legacy `template/`.

Контекст: `SEMANTIC_LAYER_ARCHITECTURE.md` → «Prerequisite: Kill FragmentItem» (строки 446-546).

## Проблема

`FragmentItem` смешивает две несвязанные задачи:

1. **Node-kind дискриминация** (`Element`, `IfBlock`, `EachBlock`, `ComponentNode`, …) — дублирует `block_semantics(id)` и `element_shape_semantics(id)`.
2. **Настоящая lowering-работа** — склейка подряд идущих Text+ExpressionTag в `TextConcat`, фильтрация hoisted-узлов, whitespace collapse.

Последствие: диспатчер в codegen начинается с `match FragmentItem::*`, после чего semantic-query вложены внутри. Root Consumer Migration Rule нарушается по построению.

Кроме того, вокруг `FragmentItem` живёт `ContentStrategy` (enum формы содержимого per fragment) и side-tables (`debug_tags`, `title_elements`, `component_snippets`, `local_snippets`, `const_tags.by_fragment`, `has_dynamic_children`) — часть из них оказывается ненужной как публичный интерфейс.

## Целевая архитектура: разделение ответственности

| Задача | Слой | Обоснование |
|---|---|---|
| AST с текстом как написан | parser | исходный материал |
| `{expr}` реактивность, блок-семантика, element-shape | analyze (существующие кластеры) | семантика, несколько consumer'ов |
| `SnippetBlockSemantics.hoistable: bool` | analyze (block_semantics) | композитное семантическое решение |
| JS-rewrites (runes, await, stores) | transform | единственный mutator AST |
| Trim edges + collapse middle | codegen (prepare) | локальная нормализация ради вывода, один consumer |
| Hoisted фильтрация / сбор в bucket | codegen (prepare) | тривиальный match по типу AST-узла |
| Hoisted emit | codegen (hoisted) | единственный consumer |
| ContentStrategy (Empty/Single*/Multi) | codegen (prepare) | решение перед обходом, один consumer |
| Group Text+Expr → Concat | codegen (prepare) | та же нормализация |
| Итоговый match + emit | codegen (emit_fragment + on_*) | целевой JS |

### Единое правило

> Analyze держит данные **только если их читает другой pass analyze** либо **несколько слоёв вне analyze**.
> Иначе — codegen собирает при обходе.

Применение:
- **const_tags** — публичный side-table удаляется. Reactivity builder intra-analyze обходит template один раз (через существующий `visit_const_tag`) и собирает плоский `Vec<NodeId>` как побочный эффект. В `finish()` пробегается по этому списку, выставляет `ConstDeclarationSemantics::ConstTag.reactive`. Локальный вектор дропается. Codegen per-fragment список получает сам в `prepare` через `HoistedBucket.const_tags`.

  **Почему это отдельный пост-шаг reactivity builder'а:** реактивность `{@const x = expr}` зависит от классификации script-level деклараций, на которые ссылается `expr`. Цепочка `let a = $state(0); let b = $derived(a); {@const x = b}` требует, чтобы реактивность `b` была вычислена **до** решения про `x`. Основной fix-point реактивности стабилизируется первым, потом классифицируются const_tag'и.

- **snippets / debug_tags / title_elements** — читает только codegen. В analyze не нужны.
- **SnippetBlockSemantics.hoistable** — композитный семантический flag, остаётся в block_semantics.
- **ConstTagBlockSemantics** (`decl_node_id`, `async_kind`) — остаётся в block_semantics. Codegen читает payload по NodeId из bucket'а.

## Архитектура codegen/ (финальная)

Модуль `crates/svelte_codegen_client/src/codegen/` **строго изолирован** от `template/`:
- `use crate::template::*` и `use super::template::*` внутри `codegen/` — **запрещены**.
- `template/*` может ре-экспортировать из `codegen/` (обратная зависимость).
- Общая логика (HTML-генератор, template_effect, memo) **копируется** в `codegen/` с адаптацией под наши типы (`FragmentCtx`, `ConcatPart`, `FragmentOutput`). Legacy остаётся заморожен до удаления.

### Типы

```rust
// Входной контекст — родитель передаёт ребёнку.
pub(crate) struct FragmentCtx<'a> {
    // Trim rules (читает только prepare)
    pub preserve_whitespace: bool,
    pub is_pre: bool,
    pub is_textarea: bool,
    pub can_remove_entirely: bool,
    pub inside_head: bool,

    // Emit rules
    pub namespace: Namespace,
    pub key: FragmentKey,
    pub source: &'a str,
    pub anchor: FragmentAnchor<'a>,
}

impl<'a> FragmentCtx<'a> {
    pub fn is_root(&self) -> bool {
        matches!(self.anchor, FragmentAnchor::Root)
    }
}

// Типизированная точка вставки для ребёнка.
pub(crate) enum FragmentAnchor<'a> {
    /// Вершина — $$anchor доступен, нужен $.next перед static head content,
    /// финальный $.append($$anchor, root) делает на верхнем уровне.
    Root,

    /// Внутри element'а — ребёнок строит первый child через $.child(parent_var).
    Child { parent_var: String },

    /// Позиция задана expression'ом напрямую. Используют блоки:
    /// {#if}/{#each}/{#await} создают `var node = $.first_child(frag)`
    /// и передают node как Explicit своим веткам.
    Explicit(Expression<'a>),

    /// Фрагмент внутри callback'а: `(anchor) => { ... }` — named slot, $.head,
    /// each item body, if consequent, snippet body. Anchor — параметр callback'а.
    CallbackParam { name: String },
}

// Локальное состояние codegen'а при обходе одного template-слота.
// Мутируется сверху вниз: родитель даёт ребёнку &mut, ребёнок пишет в него.
pub(crate) struct EmitState<'a> {
    /// HTML-буфер со стеком push/pop_element. Строится одним обходом AST.
    pub template: Template,

    /// DOM-инициализация: var node = ..., attach вызовы, sibling-cursor vars.
    pub init: Vec<Statement<'a>>,

    /// Реактивные statements. Оборачивается в $.template_effect снаружи.
    pub update: Vec<Statement<'a>>,

    /// Post-create hooks: $.effect, transitions, use: хвосты.
    pub after_update: Vec<Statement<'a>>,

    /// Namespace текущего слота.
    pub namespace: Namespace,
    pub preserve_whitespace: bool,
    pub bound_contenteditable: bool,

    /// Имя корневой JS-переменной этого слота (для упаковки снаружи).
    /// Например, `"text"` для SingleExpr или `"fragment"` для Multi.
    pub root_var: Option<String>,

    /// svelte:head / svelte:window / svelte:document / svelte:body.
    /// Текут вверх: ребёнок пушит сюда, родитель не трогает, финализация на root'е.
    pub special_elements: Vec<Statement<'a>>,

    /// Memoized attrs для template_effect_with_memo. Будущее.
    pub memo_attrs: Vec<MemoAttr<'a>>,

    /// Async blockers.
    pub script_blockers: Vec<u32>,
    pub extra_blockers: Vec<Expression<'a>>,
}

// Program-level состояние codegen'а.
pub(crate) struct Codegen<'a, 'ctx> {
    ctx: &'ctx mut Ctx<'a>,
    hoisted: Vec<Statement<'a>>,
    instance_snippets: Vec<Statement<'a>>,
    hoistable_snippets: Vec<Statement<'a>>,
}
```

### Точка входа

```rust
pub(crate) fn codegen_root_fragment<'a>(ctx: &mut Ctx<'a>) -> Result<CodegenResult<'a>> {
    let root_ctx = FragmentCtx::root(ctx, FragmentKey::Root);
    let mut cg = Codegen::new(ctx);
    let mut state = EmitState::new(root_ctx.namespace, root_ctx.preserve_whitespace);

    cg.emit_fragment(&mut state, &root_ctx, FragmentKey::Root)?;

    Ok(cg.finalize(state))
}
```

`Codegen::finalize(state)` сам решает форму упаковки по содержимому `state`:
- `state.template` не пустой → `var root = $.from_html(...)` в hoisted, `var fragment = root()` в body, `$.append($$anchor, fragment)`.
- `state.template` пустой, `state.root_var` есть → `$.append($$anchor, root_var)` (случай single-leaf-as-root: `{foo}`, `text`).
- Пустой фрагмент → ничего.

Упаковку `$.template_effect(...)` над `state.update` тоже делает `finalize` — на уровне root / callback-border.

## Инварианты

Эти шесть правил — опорные точки архитектуры. Любое нарушение = баг.

### №1 — единственная точка обработки фрагмента

Только `emit_fragment(state, ctx, key)` может:
- звать `prepare`,
- матчить `ContentStrategy`,
- гонять `process_children`,
- эмитить листовые (static / expr / concat).

Никакой другой модуль codegen не имеет права это делать. `element.rs`, `component.rs`, `svelte_element.rs`, `block/*.rs` — **вообще не знают** про `prepare`, `ContentStrategy`, `Child`, `HoistedBucket`.

### №2 — `&mut EmitState` сверху вниз

Ребёнок пишет в `state` родителя. `FragmentOutput` больше нет. Результат для родителя — побочный эффект в `state` плюс лёгкий возврат типа `String` (имя var'а) когда нужно.

### №3 — композиция только через эмиттеры владельцев

Любой эмиттер владельца узла (element, component, svelte:element, svelte:boundary, block) **обязан** вызывать `emit_fragment` для своих детей. Прямой `prepare` / `process_children` / дублирующий обход — запрещён.

Общая форма:

```rust
fn emit_element(&mut self, state, ctx, el_id) -> Result<String> {
    state.template.push_element(name, is_html);
    let el_name = self.gen_ident(name);

    self.emit_attrs(state, el_id, &attrs)?;      // ← общий эмиттер атрибутов

    let child_ctx = ctx.child_of_element(
        name, FragmentKey::Element(el_id), ns,
        FragmentAnchor::Child { parent_var: el_name.clone() },
    );
    self.emit_fragment(state, &child_ctx, FragmentKey::Element(el_id))?;  // ← детей

    state.template.pop_element();
    Ok(el_name)
}
```

### №4 — `emit_fragment` знает всё про anchor

```rust
fn emit_fragment(&mut self, state, ctx, key) -> Result<()> {
    let (children, strategy, bucket) = prepare(...);   // единственный prepare

    match strategy {
        Empty => {}
        SingleStatic(part)  => self.emit_static_leaf(state, ctx, part),
        SingleExpr(id)      => self.emit_expr_leaf(state, ctx, id)?,
        SingleConcat(parts) => self.emit_concat_leaf(state, ctx, parts)?,
        SingleElement(id)   => self.emit_element_in_fragment(state, ctx, id)?,
        SingleBlock(id)     => self.emit_block(state, ctx, id)?,
        Multi               => self.process_children(state, ctx, &children)?,
    }

    self.emit_hoisted(state, ctx, &bucket)
}
```

Листовые и Multi сами решают, что делать под `ctx.anchor` (Root/CallbackParam vs Child) — читают поле из ctx. Решение встроено в каждый leaf/multi-emitter, а не вынесено наверх.

### №5 — `process_children` — внутренний helper

`process_children` — приватный helper `emit_fragment` **только** для `Multi`. Снаружи не вызывается. Элементы/компоненты/блоки зовут `emit_fragment` для своих детей, который сам решит, что Multi → process_children.

### №6 — атрибуты через один `emit_attrs`

```rust
fn emit_attrs(&mut self, state, owner_id, attrs) -> Result<()>
```

Единственный путь обработки атрибутов для любого владельца (Element, SvelteElement, Component, SvelteBoundary). Static пишет в `state.template.set_attribute`, dynamic — в `state.init` / `state.update` / `state.after_update`.

## Грепы-детекторы регресса

После рефактора и на каждом коммите:

```
grep -r 'prepare(' crates/.../codegen/
  → 1 совпадение, только в emit_fragment.rs.

grep -r 'ContentStrategy' crates/.../codegen/
  → только prepare.rs (определение) + emit_fragment.rs (единственный match).

grep -r 'process_children' crates/.../codegen/
  → только emit_fragment.rs (вызов) + process_children.rs (тело).

grep -r 'FragmentOutput' crates/.../codegen/
  → 0. Удалён.

grep -r '&mut FragmentOutput' crates/.../codegen/
  → 0.

grep -rE '\.(unwrap|expect)\(|unreachable!' crates/.../codegen/ | grep -v _tests.rs
  → 0. Только CodegenError через ?.
```

Нарушение любого грепа = нарушение инвариантов. Не коммитим.

## Переток наверх (special_elements и т.п.)

`EmitState` — локальное состояние слота, но поля `special_elements` / `memo_attrs` / `script_blockers` / `extra_blockers` **текут** снизу вверх через `state` родителя. Ребёнок пушит в них напрямую, родитель не трогает — только финализация на root'е инжектит `special_elements` в body перед `$.append`.

Альтернатива — иметь отдельный `state` на границе element'а, тогда эти поля пришлось бы мёрджить в parent state руками. Не стоит — накладно и добавляет точек ошибки. `EmitState` уникален на весь root-фрагмент; template-буфер маркируется через push/pop_element-стек.

## Program-scoped исключения (ctx.state)

Не всё ходит через `EmitState`. Program-scoped состояние (один на компонент) живёт в `ctx.state`:
- `ctx.state.delegated_events` — события коллектятся за весь компонент.
- `ctx.state.needs_binding_group` / `group_index_names` — bind:group (отложено).
- `ctx.state.const_tag_blockers` — async const blockers.

Это НЕ нарушение №2 — агрегировать такое через `state` родителя бессмысленно (читатели — братья-фрагменты, не родитель).

## Правила trim (prepare, не меняются)

Реализовано в `codegen/prepare.rs`, покрыто 14 юнит-тестами (`codegen::prepare_tests`).

- **Edges:** leading/trailing ws-only Text — удалить. Leading/trailing WS внутри крайнего не-ws Text — срезать.
- **Middle:** ws-only Text между двумя не-Expression соседями — коллапсировать в `" "`. Между Expression и чем угодно — сохранить (семантический spacing).
- **`preserve_whitespace`** (ctx флаг от родителя `<pre>` / `<textarea>` / опции компиляции) — шаги 2-5 пропускаются.
- **`can_remove_entirely`** (ctx флаг от `<select>`, `<tr>`, `<table>`, `<colgroup>`, `<tbody>`, `<thead>`, `<tfoot>`, `<datalist>`, `<svg>` без `<text>`) — ws-only Text удаляется целиком вместо коллапса.
- **Pre-quirk:** первый `"\n"` / `"\r\n"` внутри `<pre>` / `<textarea>` дропается.

`is_hoisted` — чистый match по AST Node enum:
```rust
fn is_hoisted(node: &Node) -> bool {
    matches!(node,
        Node::SnippetBlock(_) | Node::ConstTag(_) | Node::DebugTag(_)
        | Node::SvelteHead(_) | Node::SvelteWindow(_)
        | Node::SvelteDocument(_) | Node::SvelteBody(_))
}
```
Title element (`<title>` внутри `<svelte:head>`) — обрабатывается через ctx-флаг `inside_head`.

## Child / ContentStrategy (в codegen/prepare)

```rust
pub(crate) enum Child {
    Text(ConcatPart),          // одиночный Text-узел
    Expr(NodeId),              // одиночный ExpressionTag
    Concat(SmallVec<[ConcatPart; 4]>),  // ≥2 смешанных Text+Expr
    Node(NodeId),              // всё остальное
}

pub(crate) enum ConcatPart {
    Static(Span),              // zero-copy span
    StaticOwned(String),       // текст после whitespace-collapse
    Expr(NodeId),
}

pub(crate) enum ContentStrategy {
    Empty,
    SingleStatic,
    SingleExpr(NodeId),
    SingleConcat,
    SingleElement(NodeId),
    SingleBlock(NodeId),
    Multi { count: u16, has_elements: bool, has_blocks: bool, has_text: bool },
}
```

## Файлы в codegen/

```
codegen/
  mod.rs              — Codegen struct + codegen_root_fragment + finalize
  state.rs            — CodegenResult (внешний контракт для lib.rs::generate)
  error.rs            — CodegenError + constructor-методы

  prepare.rs          — Child / ConcatPart / ContentStrategy / prepare / HoistedBucket / resolve_fragment
  prepare_tests.rs    — юнит-тесты prepare

  fragment_ctx.rs     — FragmentCtx + FragmentAnchor
  emit_state.rs       — EmitState (template, init, update, after_update, root_var, special_elements, ...)
  template.rs         — Template (html-буфер со стеком push/pop_element, set_attribute, as_html)
  namespace.rs        — from_namespace / inherited_fragment_namespace

  expr.rs             — Codegen::take_node_expr / take_attr_expr / take_stmt_for

  emit_fragment.rs    — emit_fragment(&mut state, ctx, key) — ЕДИНСТВЕННЫЙ prepare + match
                        + внутренние emit_*_leaf (static/expr/concat)
                        + process_children (приватный helper для Multi)
                        + emit_hoisted

  element.rs          — emit_element(&mut state, ctx, el_id) → String
                        push_element → emit_attrs → emit_fragment(child_ctx) → pop_element

  attributes.rs       — emit_attrs(&mut state, owner_id, attrs) — единая точка для любого владельца

  component.rs        — (будущее) emit_component(&mut state, ctx, id) — props через emit_attrs, slots через emit_fragment
  svelte_element.rs   — (будущее) emit_svelte_element(&mut state, ctx, id)
  svelte_boundary.rs  — (будущее)

  block/
    if_block.rs       — (будущее) emit_if_block(&mut state, ctx, id)
    each_block.rs     — (будущее) ветки consequent/alternate через emit_fragment с CallbackParam
    ...
```

Правило: один файл — один эмиттер владельца узла. Эмиттер не знает про prepare/ContentStrategy, только вызывает `emit_fragment` для своих детей.

## Контракт входа / выхода

```rust
pub(crate) fn codegen_root_fragment<'a>(ctx: &mut Ctx<'a>) -> Result<CodegenResult<'a>>;

#[derive(Default)]
pub(crate) struct CodegenResult<'a> {
    pub hoisted: Vec<Statement<'a>>,
    pub body: Vec<Statement<'a>>,
    pub instance_snippets: Vec<Statement<'a>>,
    pub hoistable_snippets: Vec<Statement<'a>>,
}
```

Это **форма для `lib.rs::generate`** (4-tuple без кортежа). Собирается один раз в `Codegen::finalize(root_output)` — больше нигде `CodegenResult` не существует.

## Error-model

```rust
pub(crate) enum CodegenError {
    FragmentNotFound(FragmentKey),
    UnexpectedChild { expected: &'static str, got: &'static str },
    UnexpectedNode { node_id: NodeId, expected: &'static str },
    UnexpectedBlockSemantics { node_id: NodeId, got: &'static str },
    MissingExpression(NodeId),
    MissingExpressionDeps(NodeId),
}

impl CodegenError {
    pub(crate) fn fragment_not_found<T>(key: FragmentKey) -> Result<T>;
    pub(crate) fn unexpected_child<T>(expected: &'static str, got: &'static str) -> Result<T>;
    // ... — все ветки возвращают сразу Result<T> для эргономики.
}

pub(crate) type Result<T> = std::result::Result<T, CodegenError>;
```

Без lifetime'ов. Все `on_*` / `emit_fragment` возвращают `Result<FragmentOutput<'a>>`. Оператор `?` работает везде.

### Абсолютный запрет на `.unwrap()` / `.expect()` / `unreachable!()` в `codegen/`

**Внутри `codegen/*` production-кода — zero tolerance**. Это касается и копий из legacy: если в оригинале был `.expect("...")` — при переносе он **заменяется** на:
- `CodegenError::...` возврат через `?` — если ошибка возможна (missing expression, unexpected node).
- Явную обработку `Option` через `let Some(...) = ... else { ... }` / `match` — если вариант действительно недостижим по контракту выше (тогда возврат `CodegenError::UnexpectedNode` / `UnexpectedBlockSemantics` — **как структурированная ошибка**, не как panic).
- Для `fmt::Write` в `String` (copy-paste с `.expect("writing to String never fails")`) — использовать `String::push_str` + `format!` либо `let _ = write!(...)` (явное игнорирование гарантированно-успешной операции).

**Исключения** (строго ограниченные):
1. `codegen/prepare_tests.rs` и любые `#[cfg(test)]` модули — тесты используют `.expect` / `unreachable!` как штатный инструмент (`assert!`).
2. Один костыль в `template/mod.rs::gen_root_fragment`:
   ```rust
   let result = crate::codegen::codegen_root_fragment(ctx).expect("codegen failed");
   ```
   Этот `.expect` вне `codegen/`, временный, удаляется когда `lib.rs::generate` начнёт пробрасывать `Result` наружу.

**Проверка**: `grep -E '\.(unwrap|expect)\(|unreachable!' crates/svelte_codegen_client/src/codegen/` **вне `_tests.rs` файлов** должен возвращать **0 совпадений**. Это входит в §«Критерий готовности» каждого шага.

## Фичефлаг

Env-переменная `SVELTE_RS_CODEGEN_V2=1` (или `=true`). По умолчанию — legacy.

- `just test-compiler-v2` и `just test-case-v2` — корпус compiler_tests с переменной.
- Программно флаг включить нельзя — избежать divergence между юнит-тестами и интеграционными.
- Удаление флага — финальный коммит после зелёного `test-compiler-v2` + `test-diagnostics`.

## Доступ к данным анализа

Через `self.ctx.*` (тот же канал, что у legacy). **Не** оборачиваем `AnalysisData` в фасад, **не** кэшируем результаты внутри `Codegen`/`FragmentCtx`.

Два шаблона:

### 1. Короткие методы-хелперы на `Codegen` (правило: добавляем когда вызов встречается 3+ раз)

```rust
impl<'a, 'ctx> Codegen<'a, 'ctx> {
    #[inline] fn b(&self) -> &Builder<'a> { &self.ctx.state.b }
    #[inline] fn node(&self, id: NodeId) -> &Node { self.ctx.query.component.store.get(id) }
    #[inline] fn block_sem(&self, id: NodeId) -> &BlockSemantics { self.ctx.query.analysis.block_semantics(id) }
    #[inline] fn element_shape(&self, id: NodeId) -> &ElementShapeSemantics { self.ctx.query.analysis.element_shape(id) }
    #[inline] fn expr_flags(&self, id: NodeId) -> &ExpressionFlags { self.ctx.query.analysis.expression_flags(id) }
}
```

### 2. Локальные `let`-алиасы в начале `on_*` (когда читается много разных полей)

```rust
fn on_single_element(&mut self, ctx: &FragmentCtx<'a>, el_id: NodeId) -> Result<FragmentOutput<'a>> {
    let analysis = &self.ctx.query.analysis;
    let view = &self.ctx.query.view;
    let store = &self.ctx.query.component.store;
    // ...
}
```

**Borrow-правило**: `let analysis = &self.ctx.query.analysis` держит immutable borrow на `self`, поэтому после него нельзя `self.hoisted.push(...)`. Лечение — сначала читаем и **clone / извлекаем enum-вариант / bool / id** в локальные переменные, borrow дропается, потом мутируем.

### Доступ к expressions через `Codegen::take_node_expr`

Для вытаскивания OXC Expression из `ExprHandle` есть отдельный хелпер:

```rust
impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn take_node_expr(&mut self, id: NodeId) -> Result<Expression<'a>>;
    pub(super) fn take_attr_expr(&mut self, attr_id: NodeId) -> Result<Expression<'a>>;
    pub(super) fn take_stmt_for(&mut self, id: NodeId) -> Result<Statement<'a>>;
}
```

Эти методы **скрывают** `ExprHandle` / `StmtHandle` / `ctx.state.parsed.take_expr()`. Когда схема хранения expressions будет отрефакторена — меняется только `codegen/expr.rs`, все `on_*` остаются нетронутыми.

## Что умирает / переезжает

| Сущность | Судьба |
|---|---|
| `FragmentItem` (enum, 13 вариантов) | удаляется |
| `LoweredFragment { items: Vec<FragmentItem> }` | удаляется |
| `LoweredTextPart` | удаляется (заменяется `ConcatPart`) |
| Старая `ContentStrategy` (в analyze) | удаляется, переезжает в codegen с новыми вариантами |
| `has_dynamic_children: FxHashSet<FragmentKey>` | удаляется из analyze — вычисляется в `prepare` через `ChildrenFlags` |
| `fragment_blockers: FxHashMap<FragmentKey, ...>` | **остаётся в analyze** — async-семантика поверх `BlockerData`, в codegen переносить нельзя |
| `FragmentsData.snippets.component_snippets` / `local_snippets` | удаляются из публичного API — codegen собирает в bucket |
| `FragmentsData.snippets.component_named_slots` | **остаётся** — семантическая классификация по атрибуту `slot`, не hoisted |
| `FragmentsData.debug_tags.by_fragment` | удаляется |
| `FragmentsData.title_elements.by_fragment` | удаляется |
| `FragmentsData.const_tags.by_fragment` | удаляется публичный side-table. Reactivity builder собирает плоский список intra-analyze walk'ом. Codegen собирает per-fragment список в bucket в `prepare`. |
| `SnippetBlockSemantics.hoistable: bool` | остаётся (композитный семантический flag) |
| `ConstTagBlockSemantics` | остаётся как есть |
| `passes/lower.rs::trim_text` | переезжает в codegen (`prepare.rs`) |
| `passes/content_types.rs` | удаляется — `ContentStrategy` считается в codegen при prepare |
| `CollectConstTagFragments` pass | исчезает — reactivity builder делает intra-analyze walk, codegen собирает per-fragment в `prepare` |

## Миграционный план

Миграция выполняется по кускам, каждый коммит зелёным.

**Правило коммитов.** После каждой пачки изменений — обязательно:
1. `just test-compiler` — зелёный (legacy не сломано).
2. `just test-diagnostics` — зелёный.
3. Коммит.

Не накапливать несколько пачек в одну зелёную сборку. Если тесты упали — чинить в той же пачке.

### Текущее состояние (baseline)

Реализовано:
- `codegen/prepare.rs` + 14 юнит-тестов (100%).
- `codegen/namespace.rs` — дубль `from_namespace` / `inherited_fragment_namespace` без `use template::*`.
- `codegen/fragment_ctx.rs` — `FragmentCtx` (старая форма, без `anchor` поля).
- `codegen/expr.rs` — `take_node_expr`, `take_attr_expr`, `take_stmt_for`.
- `codegen/emit_fragment.rs` — диспатч на `on_*` через старый `FragmentBuilder`.
- `on_empty`, `on_single_static`, `on_single_expr`, `on_single_concat` — реализованы для **root-случая** (без anchor-веток).
- `codegen/html.rs` — stub (`todo!`). Перенос `fragment_html` / `element_html` произойдёт в Шаге 2 уже под новые типы (`FragmentOutput`, anchor-aware).

Тесты: **36/551 v2** / 551/551 legacy / 337/337 diagnostics.

### Шаг 1 — финальный дизайн-рефактор

Перевести существующий скелет на целевые типы:
1. Создать `codegen/fragment_output.rs` — `FragmentOutput`.
2. Расширить `FragmentCtx` полем `anchor: FragmentAnchor<'a>` + убрать `is_root: bool` (derive).
3. Изменить сигнатуру `emit_fragment` и всех `on_*` на `-> Result<FragmentOutput<'a>>`.
4. Удалить `FragmentBuilder` — не нужен.
5. Переписать 4 существующих `on_*` (empty/static/expr/concat) под anchor-aware + `FragmentOutput`.
6. Переписать `codegen_root_fragment` через `Codegen::finalize(root_output)`.

Критерий готовности: **36/551 v2 сохраняется** (регрессий нет). Legacy 551/551.

### Шаг 2 — `on_single_element` (Element-only, без атрибутов, с рекурсией)

Element-ветка диспатча: `var el = $.child(parent_var)` (nested) или `var el = root()` с template-const (Root/CallbackParam). Рекурсия в дочерний фрагмент через `self.emit_fragment(FragmentKey::Element(id), child_ctx)` с `FragmentAnchor::Child { parent_var }`. Ожидание: кейсы `<div></div>`, `<div>hi</div>`, `<div>{x}</div>`, `<div><p>hi</p></div>` зеленеют в v2.

### Шаг 3 — `process_attrs` в `codegen/attrs.rs` (инкрементно)

Статические (`class="foo"`, `disabled`) уже в `element_html`. Инкрементно добавлять:
- `ExpressionAttribute` (`class={x}`) → `$.set_attribute` в update.
- `BindDirective` → `$.bind_*` в init или after_update.
- `ClassDirective` / `StyleDirective` / `UseDirective` / `OnDirective` / `AttachTag` / `TransitionDirective` / `AnimateDirective`.

Каждый тип — отдельный коммит.

### Шаг 4 — `emit_template_effect_with_memo` в `codegen/effect.rs`

Перенос из `template/expression.rs`. Типы `MemoAttr`, `MemoAttrUpdate`, `TemplateMemoState` — рядом.

### Шаг 5 — `on_single_block` + `dispatch_node` для block'ов

If / Each / Await / Key / Render / HtmlTag. Каждый — отдельный коммит. Внутри: `var frag = $.comment(); var node = $.first_child(frag);` → `gen_if_block(ctx, id, sem, FragmentAnchor::Explicit(node_expr))`. Ветки consequent/alternate/then/catch/body эмитятся как callback'и с `FragmentAnchor::CallbackParam { name: "$$anchor" }`.

### Шаг 6 — `on_mixed` с sibling cursor

Локальный курсор `prev_ident` / `sibling_offset` в `on_mixed`. Каждому ребёнку — свой `FragmentAnchor::Explicit($.child(frag))` для первого, `$.sibling(prev)` для следующих. Родительский html — через `build_fragment_html`.

### Шаг 7 — Компоненты (`on_component`)

Через `dispatch_node` → `ElementShape::Component`. Props через `prop_object.rs`. Named slots — отдельный emit с `FragmentAnchor::CallbackParam`. CSS wrapper (`component_css_props`) — отдельный путь.

### Шаг 8 — Special элементы

`svelte:head` / `svelte:window` / `svelte:document` / `svelte:body` через `emit_hoisted` в `FragmentOutput.special_elements`. `svelte:element` / `svelte:boundary` — отдельные dispatch-ветки (не через tpl-const).

### Шаг 9 — Оставшееся

Controlled each fragments, bind:this / bind:group (program-state), delegated events, DEV-режим (`add_svelte_meta`, `add_locations`), async const tags (`const_tag_blockers` full), snippets/render_tag.

### Шаг 10 — Удаление analyze-side

Когда `test-compiler-v2` зелёный целиком:

1. Удалить `LoweredFragment`, `FragmentItem`, `LoweredTextPart`, старый `ContentStrategy` из `svelte_analyze`.
2. Удалить `has_dynamic_children`, публичные `component_snippets` / `local_snippets` / `debug_tags` / `title_elements`. `component_named_slots` оставить. `fragment_blockers` оставить.
3. Убрать публичный `const_tags.by_fragment`. Reactivity builder переходит на intra-analyze walk.
4. Удалить `passes/lower.rs`, `passes/content_types.rs`, pass `CollectConstTagFragments`.

### Шаг 11 — Удаление legacy template/

1. Убрать env-флаг `SVELTE_RS_CODEGEN_V2` — codegen v2 становится default'ом.
2. Удалить `crates/svelte_codegen_client/src/template/` целиком.
3. Переназвать `codegen/` → `template/` (или оставить как есть).

### Шаг 12 — Проверка traversal budget

Пересчитать traversal-бюджет (`SEMANTIC_LAYER_ARCHITECTURE.md` строки 129-138). Ожидание: template walk'ов в analyze меньше минимум на 2 (`LowerTemplate` + `ClassifyRemainingFragments` сняты).

## Проверки до коммита (для каждого шага)

1. **Pre-Code Shape Check.** Псевдокод `on_*` линейный, без вложенных ветвлений с fallback'ом.
2. **Callsite coverage.** Каждый `FragmentKey` вариант проходит через `resolve_fragment` + `emit_fragment` с корректным `FragmentAnchor`.
3. **Reference JS line-by-line.** Перед реализацией `on_X` для строгого кейса — **прогнать конкретный `.svelte` через reference компилятор (или через legacy), прочитать reference JS построчно**, зафиксировать чего ждать. Без этого — не начинать.

## Решённые вопросы

### fragment_blockers — остаётся в analyze

Transform не читает (grep `fragment_blockers` в `svelte_transform` → 0 файлов). Единственные consumer'ы — codegen.

**Но переносить в codegen не надо.** `fragment_blockers` вычисляются на основе `BlockerData` (script-level async analysis) + async-статуса выражений в поддеревьях фрагмента. Это семантический анализ поверх async-семантики. Если переложить в codegen — codegen начнёт делать async-анализ, что нарушит границу. Пусть analyze продолжает вычислять, codegen читает через готовый accessor (`self.ctx.fragment_blockers(&key)` возвращает `&[u32]`, ребёнок кладёт в `FragmentOutput.script_blockers`).

### component_named_slots — остаётся в analyze (семантика компонента)

Это не hoisted из fragment flow. `<div slot="header">` внутри `<MyComponent>` остаётся в AST children компонента, но переинтерпретируется codegen'ом как слот для prop'а `header`. Это семантическая классификация компонентных children по атрибуту `slot`, не вытаскивание узлов.

Решение: остаётся как есть в `SnippetData.component_named_slots`. Когда доедет ElementShape кластер — переедет в `ElementShapeSemantics::Component` payload как поле `named_slots: Vec<(NodeId, FragmentKey)>`. Bucket-подход в `prepare` к этому не применим — узел из AST children не удаляется.

### css_prune — не зависит от FragmentItem

Grep по `css_prune.rs`: `FragmentItem` / `LoweredFragment` / `TextConcat` / `ContentStrategy` — 0 совпадений. CSS pruning ходит по Svelte AST напрямую, не через lowered форму. Удаление `lower.rs` на CSS pipeline не влияет.

### `FragmentSetup::PostCreate` не нужен в новой архитектуре

Legacy использует для named slot с `:let` директивами. В новой архитектуре родитель просто кладёт slot-let stmts **в свой `FragmentOutput.init` перед** дочерним telом (полученным через `emit_fragment`). Не нужен enum-параметр.

### `ctx.state.pending_const_blockers` — legacy-мёртвый код

Пишется в legacy, но читающих мест не нашлось. В новой архитектуре не воспроизводим.

### `ctx.state.bound_contenteditable` — выводится из analysis

Вместо stack-флага в state читаем `self.ctx.is_bound_contenteditable(el_id)` на месте в `on_single_element`. Не нужен как state.

### Async const tag blockers — остаются в `ctx.state`

`ctx.state.const_tag_blockers: FxHashMap<SymbolId, (String, usize)>` — program-scoped (один `$$promises` массив на всю функцию компонента). Пишется в `gen_const_tags`, читается в `expression.rs` при построении blockers для expressions. В новой архитектуре остаётся в `ctx.state` — агрегировать через `FragmentOutput` бессмысленно (читатели — братья-фрагменты, не родитель).
