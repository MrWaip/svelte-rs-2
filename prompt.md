# Task: доработка side-table индексов в `svelte_analyze`

  ## Контекст

  Crate `svelte_analyze` хранит производные данные в side-tables (`crates/svelte_analyze/src/types/data/`). Codegen
  читает их через `AnalysisData` и **не должен re-traverse AST** для классификации. Базовые структуры —
  `NodeTable<T>` / `NodeBitSet` (dense, индексируемые по `svelte_ast::NodeId`) в
  `crates/svelte_analyze/src/types/node_table.rs`. Есть уже: `ElementFacts` (+`AttrIndex`), `FragmentFacts`,
  `TemplateElementIndex`, `TemplateTopology`, `EachContextIndex`, `RichContentFacts`, `TemplateSemanticsData`,
  `BindSemanticsData`, `AwaitBindingData`, `ExpressionInfo`, `RuntimePlan`, `SnippetData`.

  Архитектурные правила (из `CLAUDE.md`): immutable AST; classifications → analyze, не codegen; SymbolId over
  strings; никакого codegen-caching через re-traversal.

  ## Жёсткое правило структуры (обязательно для всех пунктов ниже)

  **В `AnalysisData` не должно появляться новых сырых `FxHashMap` / `FxHashSet` / `Vec<T>` полей.** Каждый индекс —
  **отдельная именованная структура** в своём модуле `crates/svelte_analyze/src/types/data/<name>.rs`, со своим
  `pub`-API и инкапсулированным хранилищем. Снаружи видны только типизированные методы (`has(...)`, `get(...)`,
  `iter(...)`, `record(...)`), не внутренние коллекции.

  Конкретно:
  - Поле в `AnalysisData` — всегда `some_index: SomeIndex`, никогда `some_map: FxHashMap<K, V>`.
  - Конструктор индекса — `SomeIndex::new(node_count)` или `SomeIndex::default()`.
  - Все мутации — через методы индекса (`record_*`, `mark_*`, `set_*`), без прямого доступа к внутренним полям извне
  модуля.
  - Все чтения — через методы индекса, возвращающие типизированные значения или `impl Iterator`. Никаких
  `&FxHashMap<...>` в сигнатурах public API.
  - Внутри индекс использует `NodeTable` / `NodeBitSet` / `FxHashMap` / `Vec` как приватные поля — это implementation
   detail, не контракт.
  - `AnalysisData::<method>` — тонкий delegate в индекс, не прямой доступ к коллекции. Либо вообще убрать delegate и
  требовать `data.some_index.method(...)` на call-site.

  **Существующие нарушения этого правила, которые надо починить в рамках этого таска:**
  - `import_syms: FxHashSet<SymbolId>` (`analysis.rs:36`) → `ImportSymbolSet` с методами `contains(sym)`, `iter()`,
  `insert(sym)`.
  - `proxy_state_inits: FxHashMap<CompactString, bool>` (`analysis.rs:42`) → `ProxyStateInits` с типизированным API;
  ключ — string literal рунайм-вызова, это ок как внутреннее хранилище, но снаружи должен быть метод
  `is_proxied(name: &str) -> bool` и `set_proxied(name, bool)`.
  - `pickled_await_offsets: FxHashSet<u32>` (`analysis.rs:46`) → `PickledAwaitOffsets` с методами
  `contains_offset(u32)` / `insert_offset(u32)` (плюс TODO: в идеале key'ить не через span-offset, а через stable id
  — см. B6).
  - `script_rune_call_kinds: FxHashMap<u32, RuneKind>` (`analysis.rs:45`) → см. B6, там он целиком переезжает в новый
   `ScriptRuneCalls` и меняет ключ на OXC NodeId.

  Эти четыре сейчас принимаются через `pub(crate)` поля и пробрасываются наружу через helper-методы, возвращающие
  `&FxHashMap<...>` (`analysis.rs:116`, `codegen_view.rs:46`). В рамках рефакторинга **сигнатуры методов codegen-view
   должны стать типизированными** (`script_rune_calls(&self) -> &ScriptRuneCalls`), не возвращать сырой map.

  ---

  ## Часть A. Фиксы существующих индексов

  ### A1. `EachContextIndex`: убрать String-прокси для символов
  `crates/svelte_analyze/src/types/data/each_context_index.rs`

  - `bind_this_context: NodeTable<Vec<String>>` → `NodeTable<SmallVec<[SymbolId; 4]>>`.
    Источник данных: `passes/bind_semantics.rs:37-46` — там уже есть `SmallVec<SymbolId>` (`info.ref_symbols`),
  который сейчас конвертится в `String` через `scoping.symbol_name(sym).to_string()` перед записью. Убрать конверсию,
   consumer сам лукапит имена на чтении.
  - `context_names: NodeTable<String>` **оставить как `CompactString`** — это output-имя переменной для codegen
  (`each_block.rs:57`), не прокси символа. Смена `String` → `CompactString` убирает аллокацию на коротких именах
  (`item`, `row`). Дефолт `"$$item"` не писать в таблицу: `record_context_name(block.id, "$$item".to_string())` в
  `template_side_tables.rs:542` → оставить `None`, литерал уже возвращается в fallback'е `context_name()`
  (each_context_index.rs:118).

  ### A2. `EachContextIndex`: вынести чужеродные поля
  - `parent_each_blocks: NodeTable<Vec<NodeId>>` — это не «each context», а ancestor-запрос для `bind:this`. Удалить
  поле, заменить helper'ом `AnalysisData::enclosing_each_blocks(attr_id)` поверх
  `TemplateTopology::ancestors(...).filter(|p| p.kind == ParentKind::EachBlock)`.

  ### A3. `EachContextIndex`: схлопнуть 6 `NodeBitSet` + 4 `NodeTable` в `EachBlockEntry`
  Each-блоков в компоненте единицы–десятки, а сейчас на каждый флаг выделяется `node_count/8` байт. Переделать в
  `NodeTable<EachBlockEntry>` с bitflags-полем для boolean-флагов (`key_uses_index`, `is_destructured`,
  `body_uses_index`, `key_is_item`, `has_animate`, `contains_group_binding`). `EachBlockEntry` должен содержать
  только each-block-keyed данные (`bind_this_context` отдельно — ключ attr_id, не block_id).

  Если `bind_this_context` остаётся отдельной таблицей — вынести в **отдельный типизированный индекс**
  `BindThisEachContext` в своём модуле, не в `EachContextIndex`.

  ### A4. `TemplateElementIndex::class_candidates` / `id_candidates` — O(N²) hot path
  `crates/svelte_analyze/src/types/data/template_element_index.rs:244-256`

  Сейчас копирует `maybe_matches_class` в SmallVec и возвращает по значению — для компонента с N спредов и M
  CSS-селекторов это N·M аллокаций. Переделать на `impl Iterator<Item = NodeId> + '_` через `chain(static,
  maybe_matches)`. Consumer сам собирает если нужно.

  ### A5. `TemplateElementIndex`: прочие мелочи
  - `by_tag` / `by_static_class` / `by_static_id`: `Vec<NodeId>` → `SmallVec<[NodeId; 4]>`.
  - `last_child_by_parent: FxHashMap<Option<NodeId>, NodeId>` — временное состояние билдера. После `record`'ов
  обнулить через `mem::take`, не держать в структуре на lifetime анализа.
  - `previous_siblings(id)` (строка ~258) — возвращать итератор, не SmallVec.

  ### A6. Консолидация атрибутного прохода `ElementFacts` vs `TemplateElementIndex`
  `element_facts.rs:12-22` и `template_element_index.rs:33-69` параллельно ходят по одним и тем же
  `element.attributes` ради частично пересекающихся фактов. Объединить: один проход строит `ElementFactsEntry` с
  полями `{ attr_index, has_spread, has_runtime_attrs, has_dynamic_class, has_dynamic_id, static_id, static_classes
  }`; `TemplateElementIndex::record` принимает уже готовый entry и строит только reverse-мапы.

  ---

  ## Часть B. Новые индексы

  Каждый пункт — **новый модуль `crates/svelte_analyze/src/types/data/<name>.rs`** со своей структурой. `mod.rs`
  добавляет `pub use`. Поле в `AnalysisData` — одно, типа этой структуры.

  ### B1. `DirectiveModifierFlags` ★ низкий риск, высокий повтор

  Новый модуль: `crates/svelte_analyze/src/types/data/directive_modifier_flags.rs`.

  bitflags! { pub struct EventModifier: u16 { ... } }

  pub struct DirectiveModifierFlags {
      flags: NodeTable,
  }

  impl DirectiveModifierFlags {
      pub fn new(node_count: u32) -> Self { ... }
      pub(crate) fn record(&mut self, id: NodeId, flags: EventModifier) { ... }
      pub fn get(&self, id: NodeId) -> EventModifier { ... }  // empty по умолчанию
      pub fn has(&self, id: NodeId, modifier: EventModifier) -> bool { ... }
  }

  Флаги: `once`, `capture`, `preventDefault`, `stopPropagation`, `passive`, `nonpassive`, `trusted`, `self`,
  `global`. Вычисляются один раз в пассе (рядом с `ElementFactsEntry` или в `passes/element_flags.rs`).

  Заменяет повторяющийся паттерн `modifiers.iter().any(|m| m == "X")`:
  - `passes/template_validation.rs:933` (`passive`)
  - `passes/template_validation.rs:934` (`nonpassive`)
  - `passes/template_validation.rs:2373` (`!= "once"`)
  - `passes/element_flags.rs:255` (`once`)
  - `codegen_client/template/events/actions.rs:102` (`global`)

  ### B2. `NamespaceKind` + `is_void` per element ★ закрывает boundary violation

  **Не новый модуль** — поля добавляются в существующий `ElementFactsEntry`. Но они должны быть **именованным типом
  `NamespaceKind`**, не парой bool'ов:

  #[derive(Copy, Clone, Eq, PartialEq)]
  pub enum NamespaceKind { Html, Svg, MathMl, ForeignObject }

  В `ElementFactsEntry` добавить: `namespace: NamespaceKind`, `is_void: bool`, `is_custom_element: bool`. API —
  методы `ElementFacts::namespace(id)`, `is_void(id)`, `is_custom_element(id)`.

  Посчитать в пассе один раз с учётом **inherited namespace от родителя** через `TemplateTopology`. После этого
  `svelte_ast::is_svg()` / `is_mathml()` / `is_void()` становятся internal утилитами для построения индекса, в
  codegen не зовутся.

  Сейчас codegen **сам** вычисляет namespace inheritance (`codegen_client/template/mod.rs:91-118`) — это boundary
  violation.

  Доказательства повторения:
  - `codegen_client/template/html_tag.rs:22,34,41-42,68,75-76`
  - `codegen_client/template/svelte_element.rs:47,106,132,156`
  - `codegen_client/template/html.rs:123` (`is_void`)
  - `codegen_client/template/mod.rs:112-118` (inheritance — violation)
  - `passes/lower.rs:6`

  ### B3. Использовать существующий `AttrIndex` в codegen

  `AttrIndex` построен в `ElementFactsEntry.attr_index`, но помечен `pub(crate)` и **не вызывается из codegen**.
  Codegen делает ручные линейные сканы:
  - `codegen_client/template/attributes.rs:295-302` — `.iter().find(|attr| attr.id() == class_attr_id)`
  - `codegen_client/template/element.rs:83-94`

  Сделать `AttrIndex` публичным через `ElementFacts::attr_index(id) -> Option<&AttrIndex>`. Заменить линейные сканы
  на `attr_index.first(&el.attributes, "name")` / `attr_index.all(...)`. Если нужен find-by-id — добавить метод
  `AttrIndex::find_by_id(attr_id)` через внутреннее поле `by_id: FxHashMap<NodeId, u16>` (private, доступ только
  через метод).

  Это «починить забытое», не новый индекс.

  ### B4. `SymbolRefIndex` — reverse `SymbolId → [ExprNodeId]` ★ архитектурная дырка

  Новый модуль: `crates/svelte_analyze/src/types/data/symbol_ref_index.rs`.

  pub struct SymbolRefIndex {
      refs: Vec<SmallVec<[NodeId; 4]>>,  // private, индексируется SymbolId.as_usize()
  }

  impl SymbolRefIndex {
      pub fn new() -> Self { ... }
      pub(crate) fn record(&mut self, sym: SymbolId, expr_id: NodeId) { ... }
      pub fn references(&self, sym: SymbolId) -> &[NodeId] { ... }
      pub fn reference_count(&self, sym: SymbolId) -> usize { ... }
      pub fn is_referenced(&self, sym: SymbolId) -> bool { ... }
  }

  Сейчас есть тол Task: доделать рефакторинг side-table индексов в svelte_analyze

  Частичная реализация лежит в рабочем дереве (B7, B6, A1, A2, A4 и куски A5/B3 сделаны). Этот таск закрывает остаток плюс один баг в уже
  сделанном.

  Архитектурные правила те же, что в прошлом проходе: immutable AST; classifications → analyze, не codegen; SymbolId over strings; каждый
  индекс — именованная структура в своём модуле crates/svelte_analyze/src/types/data/<name>.rs, никаких сырых FxHashMap / FxHashSet / Vec<T>
  полей в AnalysisData.

  ---
  0. БАГФИКС (сначала). ScriptRuneCalls — коллизия NodeId между instance и module script

  crates/svelte_analyze/src/passes/js_analyze/script_runes.rs

  Сейчас коллектор вызывает visit_program сначала на parsed.program, потом на parsed.module_program, и каждый раз пишет
  self.data.script_rune_calls.record(call.node_id(), kind) без offset'а. OXC назначает OxcNodeId per-parse начиная с 0 — диапазоны
  пересекаются, module-script перетрёт instance-script в FxHashMap<OxcNodeId, RuneKind>.

  Прецедент корректного подхода: crates/svelte_component_semantics/src/builder/js_visitor.rs:58-104 — new_with_offset + remap_node_id.
  ComponentSemanticsBuilder уже выставляет для module script node_id_offset = max_node_id_seen(instance) + 1.

  Нужно:
  1. В ScriptRuneCalls (types/data/script_rune_calls.rs) ключ оставить OxcNodeId, но запись для module-script должна идти через тот же offset,
   что использует ComponentSemanticsBuilder. Offset должен прокидываться через analysis-пайплайн, а не пересчитываться в script_runes.rs
  (тогда есть риск рассинхрона с semantics). Добавить в ScriptInfo или AnalysisData поле module_script_node_id_offset: u32, заполняемое на
  этапе build_component_semantics, и читать его в collect_script_rune_call_kinds.
  2. Коллектор принимает offset и при обходе parsed.module_program делает OxcNodeId::from_usize(raw.index() + offset) перед записью.
  3. Codegen (script/state.rs:1358-1368 — script_rune_call_node_id) при lookup'е для module-script должен применить тот же offset. Сейчас
  rune_kind_from_expr одинаково работает для instance и module script, нужно пробросить флаг «это module script» или offset через
  ScriptTransformer и прибавлять его перед index.kind(...).

  Альтернативный (более простой) путь, если offset пробрасывать дорого: хранить в ScriptRuneCalls не одну мапу, а две (instance:
  FxHashMap<OxcNodeId, RuneKind>, module: FxHashMap<OxcNodeId, RuneKind>) и public API принимает ScriptProgramKind наряду с OxcNodeId. Решение
   на твоё усмотрение — но никакого span.start обратно, и результат должен быть типизированным.

  Regression test: добавить компонент с rune-вызовом одновременно в instance script и module script ($effect.root в module + $state в instance
   — валидный случай) и убедиться, что оба классифицируются правильно. Файл теста — по месту других script_runes кейсов в
  svelte_analyze/src/tests.rs.

  ---
  1. A3. EachContextIndex → NodeTable<EachBlockEntry> с bitflags

  crates/svelte_analyze/src/types/data/each_context_index.rs

  Сейчас на каждый флаг each-блока выделяется NodeBitSet размером node_count/8 — each-блоков единицы, overhead несоразмерен. Схлопнуть:

  bitflags! {
      pub struct EachBlockFlags: u8 {
          const KEY_USES_INDEX = 1 << 0;
          const IS_DESTRUCTURED = 1 << 1;
          const BODY_USES_INDEX = 1 << 2;
          const KEY_IS_ITEM = 1 << 3;
          const HAS_ANIMATE = 1 << 4;
          const CONTAINS_GROUP_BINDING = 1 << 5;
          const NEEDS_COLLECTION_ID = 1 << 6;
      }
  }

  struct EachBlockEntry {
      flags: EachBlockFlags,
      context_name: Option<CompactString>,
      index_sym: Option<SymbolId>,
      key_node_id: Option<NodeId>,
  }

  pub struct EachContextIndex {
      entries: NodeTable<EachBlockEntry>,
      index_sym_to_block: FxHashMap<SymbolId, NodeId>,
      bind_this_context: BindThisEachContext,  // см. ниже
  }

  - bind_this_context — отдельный именованный индекс BindThisEachContext в своём модуле types/data/bind_this_each_context.rs. Ключ — attr_id
  (не block_id), поэтому он не должен жить в EachBlockEntry. API: set(attr_id, SmallVec<[SymbolId; 4]>), get(attr_id) -> Option<&[SymbolId]>.
  - Все существующие mark_*/record_*/getter методы EachContextIndex сохранить с той же сигнатурой (это call-site compatibility) — внутри они
  просто мутируют EachBlockFlags / EachBlockEntry.
  - После рефакторинга поле в AnalysisData остаётся each_context: EachContextIndex, типизированное API наружу — без изменений для большинства
  call-site'ов.

  ---
  2. A5. TemplateElementIndex::last_child_by_parent — временное состояние билдера

  crates/svelte_analyze/src/types/data/template_element_index.rs:121,136,171-174

  last_child_by_parent: FxHashMap<Option<NodeId>, NodeId> используется только во время record(...) для связывания sibling'ов. После окончания
  пасса оно мёртвый груз на lifetime анализа.

  Вынести в состояние билдера: либо параметр record, либо временная структура TemplateElementIndexBuilder с методом finish(self) ->
  TemplateElementIndex, которая держит last_child_by_parent и next_sibling/previous_sibling во время построения, а в finish отдаёт готовый
  TemplateElementIndex без этого поля. Выбирай вариант по месту вызова в passes/template_side_tables.rs.

  ---
  3. A6. Консолидация ElementFacts vs TemplateElementIndex проходов по атрибутам

  crates/svelte_analyze/src/types/data/element_facts.rs:12-22 и template_element_index.rs:33-69 сейчас оба независимо итерируют
  element.attributes и частично пересекаются (has_spread, has_dynamic_class, has_dynamic_id).

  Сделать:
  1. Один проход по attributes строит ElementFactsEntry со всеми полями сразу: { attr_index: AttrIndex, has_spread, has_runtime_attrs,
  has_dynamic_class, has_dynamic_id, static_id: Option<CompactString>, static_classes: SmallVec<[CompactString; 2]> }.
  2. ElementFactsEntry::build становится единственным местом, где происходит for attr in attrs { match ... } для этих фактов.
  TemplateElementEntry::build удаляется — TemplateElementIndex либо становится чистым reverse-index'ом поверх готового ElementFactsEntry, либо
   схлапывается в ElementFacts.
  3. TemplateElementIndex::record принимает &ElementFactsEntry (по ссылке на уже существующую запись в ElementFacts) и строит только
  by_tag/by_static_class/by_static_id/maybe_matches_*/previous_sibling/next_sibling.
  4. Public API TemplateElementIndex::tag_name/static_id/has_static_class/may_match_* перевести на чтение из ElementFactsEntry вместо
  TemplateElementEntry. TemplateElementEntry после этого должен либо исчезнуть, либо стать чистым reverse-index-специфичным (только
  sibling'и).
  5. Порядок вызовов в passes/template_side_tables.rs / passes/element_flags.rs — ElementFacts пишется до TemplateElementIndex, и последний
  читает готовую запись.

  Цель — каждое HTML-element посещение делает один цикл по attributes для фактов (AttrIndex build — отдельно, он и так один проход).

  ---
  4. B1. DirectiveModifierFlags — per-attribute bitflags для event/bind модификаторов

  Новый модуль: crates/svelte_analyze/src/types/data/directive_modifier_flags.rs.

  bitflags! {
      pub struct EventModifier: u16 {
          const ONCE = 1 << 0;
          const CAPTURE = 1 << 1;
          const PREVENT_DEFAULT = 1 << 2;
          const STOP_PROPAGATION = 1 << 3;
          const STOP_IMMEDIATE_PROPAGATION = 1 << 4;
          const PASSIVE = 1 << 5;
          const NONPASSIVE = 1 << 6;
          const TRUSTED = 1 << 7;
          const SELF = 1 << 8;
          const GLOBAL = 1 << 9;
      }
  }

  pub struct DirectiveModifierFlags {
      flags: NodeTable<EventModifier>,
  }

  impl DirectiveModifierFlags {
      pub fn new(node_count: u32) -> Self { ... }
      pub(crate) fn record(&mut self, id: NodeId, flags: EventModifier) { ... }
      pub fn get(&self, id: NodeId) -> EventModifier { ... }  // empty если нет записи
      pub fn has(&self, id: NodeId, modifier: EventModifier) -> bool { ... }
  }

  Заполнять в том же пассе, где билдится ElementFactsEntry (либо рядом в passes/element_flags.rs). Ключ — NodeId атрибута (OnDirective.id /
  BindDirective.id). Строковый парсинг modifiers — только здесь, один раз; на чтении — bitflags тест.

  Заменить строковые проверки на call-site'ах:
  - crates/svelte_analyze/src/passes/template_validation.rs:933 — PASSIVE
  - crates/svelte_analyze/src/passes/template_validation.rs:934 — NONPASSIVE
  - crates/svelte_analyze/src/passes/template_validation.rs:2373 — != ONCE
  - crates/svelte_analyze/src/passes/element_flags.rs:255 — ONCE
  - crates/svelte_codegen_client/src/template/events/actions.rs:102 — GLOBAL (через CodegenView::event_modifiers(attr_id))

  Добавить delegate AnalysisData::event_modifiers(attr_id) -> EventModifier и CodegenView::event_modifiers(attr_id) -> EventModifier.

  ---
  5. B2. NamespaceKind + is_void per element — закрыть boundary violation

  Не новый модуль — расширяем ElementFactsEntry в types/data/element_facts.rs.

  #[derive(Copy, Clone, Eq, PartialEq, Debug)]
  pub enum NamespaceKind { Html, Svg, MathMl, ForeignObject }

  В ElementFactsEntry добавить:
  - namespace: NamespaceKind
  - is_void: bool
  - is_custom_element: bool

  Public API ElementFacts: namespace(id) -> Option<NamespaceKind>, is_void(id) -> bool, is_custom_element(id) -> bool. Проброс через
  AnalysisData/CodegenView.

  Ключевой момент: namespace вычисляется с учётом inherited namespace от родителя через TemplateTopology::ancestors(...). То есть
  ElementFacts::record теперь принимает родительский NamespaceKind (или помимо record — отдельный проход, который идёт top-down через topology
   и проставляет namespace каждому element'у). Порядок пассов: сначала TemplateTopology, потом ElementFacts (если не так уже).

  svelte_ast::is_svg() / is_mathml() / is_void() остаются internal утилитами для построения этого поля — вне analyze их не зовут.

  Call-site'ы которые должны переключиться на ElementFacts::namespace/is_void:
  - crates/svelte_codegen_client/src/template/html_tag.rs:22,34,41-42,68,75-76
  - crates/svelte_codegen_client/src/template/svelte_element.rs:47,106,132,156
  - crates/svelte_codegen_client/src/template/html.rs:123 (is_void)
  - crates/svelte_codegen_client/src/template/mod.rs:91-118 — это boundary violation, codegen сам считает inheritance. Удалить, читать
  ctx.view.namespace(el_id).
  - crates/svelte_analyze/src/passes/lower.rs:6

  После этого grep по svelte_ast::is_svg / is_mathml / is_void из svelte_codegen_client должен дать 0 матчей.

  ---
  6. B4. SymbolRefIndex — reverse SymbolId → [ExprNodeId]

  Новый модуль: crates/svelte_analyze/src/types/data/symbol_ref_index.rs.

  pub struct SymbolRefIndex {
      refs: Vec<SmallVec<[NodeId; 4]>>,
  }

  impl SymbolRefIndex {
      pub fn new() -> Self { Self { refs: Vec::new() } }
      pub(crate) fn record(&mut self, sym: SymbolId, expr_id: NodeId) { ... }
      pub fn references(&self, sym: SymbolId) -> &[NodeId] { ... }
      pub fn reference_count(&self, sym: SymbolId) -> usize { ... }
      pub fn is_referenced(&self, sym: SymbolId) -> bool { ... }
  }

  Заполняется в том же пассе, который строит ExpressionInfo — когда записывается ExpressionInfo.ref_symbols, каждый элемент дополнительно
  дублируется в SymbolRefIndex::record(sym, expr_id). Никакого отдельного прохода не вводить.

  Найти точку записи ref_symbols в passes/js_analyze/ и passes/template_side_tables.rs — и там же вызвать data.symbol_refs.record(...).

  Поле в AnalysisData: pub symbol_refs: SymbolRefIndex. Проброс в CodegenView. Тестировать на компоненте с unused $state — ручной ассерт через
   новый метод AnalysisData::symbol_refs.is_referenced(sym).

  Существующие 80+ forward-итераций по ref_symbols не трогать — это отдельная ось.

  ---
  7. B5. SubtreeContainsFlags — propagated-up флаги поддерева

  Новый модуль: crates/svelte_analyze/src/types/data/subtree_flags.rs.

  bitflags! {
      pub struct SubtreeFlags: u8 {
          const AWAIT = 1 << 0;
          const BIND_GROUP = 1 << 1;
          const ANIMATE = 1 << 2;
          const SLOT = 1 << 3;
          const DYNAMIC_ATTRS = 1 << 4;
      }
  }

  pub struct SubtreeContainsIndex {
      flags: NodeTable<SubtreeFlags>,
  }

  impl SubtreeContainsIndex {
      pub fn new(node_count: u32) -> Self { ... }
      pub(crate) fn record(&mut self, id: NodeId, flags: SubtreeFlags) { ... }
      pub fn contains(&self, id: NodeId, flag: SubtreeFlags) -> bool { ... }
      pub fn flags(&self, id: NodeId) -> SubtreeFlags { ... }
  }

  Заполнение — bottom-up одним проходом. Встроить в существующий walker.rs (composite visitor): на leave_* узла OR'им его собственные флаги +
  флаги детей в родителя. Детали — опираться на TemplateTopology::parent, записывать в flags на leave.

  Конкретные source-of-truth для каждого флага:
  - AWAIT — узел или его поддерево содержит AwaitBlock.
  - BIND_GROUP — поддерево содержит BindDirective { name: "group", .. }.
  - ANIMATE — поддерево содержит AnimateDirective.
  - SLOT — поддерево содержит <slot> (legacy) или {@render} (не уверен, проверь по reference compiler).
  - DYNAMIC_ATTRS — поддерево содержит элемент с has_runtime_attrs || has_spread.

  После введения:
  - EachContextIndex::contains_group_binding + mark_contains_group_binding удалить из EachContextIndex и EachBlockFlags.
  - Ручное проталкивание в passes/bind_semantics.rs:132-172 (где сейчас mark_contains_group_binding(each_id) лепится через ancestors +
  сравнение scope'ов) — удалить. Вместо него — при leave each-блока, subtree_flags.contains(each_id, SubtreeFlags::BIND_GROUP).
  - FragmentFacts::has_direct_animate_child — не трогать (это direct-only, другое свойство). Но любые call-site'ы, которым нужен «где-то в
  поддереве», переключить на SubtreeFlags::ANIMATE.
  - Ad-hoc рекурсивные обходы в passes/template_validation.rs (поищи по fn walk_ / ручные match'и с рекурсией) — заменить на
  subtree_flags.contains(id, ...), где это семантически эквивалентно.

  ---
  8. Мелкие чистки в уже сделанном

  8.1. crates/svelte_analyze/src/types/data/analysis.rs:117-122 — два метода script_rune_calls + script_rune_call_kind. Второй нигде не
  используется (grep), удалить. Call-site должен ходить data.script_rune_calls().kind(node).

  8.2. crates/svelte_analyze/src/types/data/import_symbols.rs:28-32 и proxy_state_inits.rs:23-27 — impl From<FxHashSet<..>> / impl
  From<FxHashMap<..>> открывают публичный escape hatch для сырой коллекции. Если не используется извне crate'а — сделать pub(crate) fn
  from_raw(...) или удалить. Проверь grep на ImportSymbolSet::from/ProxyStateInits::from.

  8.3. AnalysisData::parent_each_blocks (analysis.rs:353-377) — промежуточный SmallVec<[SymbolId; 4]> на hot path. Проинлайнить фильтр в
  .any(...) внутри filter_map, убрать referenced_syms целиком:

  pub fn parent_each_blocks(&self, id: NodeId) -> SmallVec<[NodeId; 4]> {
      let Some(info) = self.attr_expressions.get(id) else { return SmallVec::new(); };
      self.ancestors(id)
          .filter(|p| p.kind == ParentKind::EachBlock)
          .filter_map(|parent| {
              let body_scope = self.each_body_scope(parent.id, self.scoping.root_scope_id());
              info.ref_symbols.iter().any(|&sym| {
                  self.scoping.is_each_block_var(sym)
                      && self.scoping.symbol_scope_id(sym) == body_scope
              }).then_some(parent.id)
          })
          .collect()
  }

  ---
  Порядок выполнения

  Строго в таком порядке (каждый пункт — отдельный коммит, зелёный cargo check -p svelte_analyze -p svelte_codegen_client + релевантные
  unit-тесты):

  1. §0 багфикс ScriptRuneCalls NodeId collision (+ regression test)
  2. §8 чистки (механический sweep, разгружает diff последующих пунктов)
  3. §1 A3 EachBlockEntry bitflags + BindThisEachContext
  4. §2 A5 last_child_by_parent → builder
  5. §3 A6 ElementFacts/TemplateElementIndex консолидация (базис для §4–§5)
  6. §4 B1 DirectiveModifierFlags
  7. §5 B2 NamespaceKind + is_void (удаляет boundary violation в codegen)
  8. §6 B4 SymbolRefIndex
  9. §7 B5 SubtreeContainsFlags (удаляет contains_group_binding из §3)

  После каждого пункта — прогон cargo test -p svelte_analyze и cargo test -p svelte_codegen_client. Если падают golden-тесты codegen —
  смотреть diff output'а, не подгонять ассерты.

  Жёсткие запреты (повтор)

  - **Никаких новых сырых FxHashMap / FxHashSet / Vec<T> полей в AnalysisData.

⏺ Промпт: доделать работу по prompt.md

  Контекст

  В ветке claude/diagnose-example-component-CwOuL уже начат рефакторинг side-table индексов в svelte_analyze по задаче из prompt.md. Часть
  работы сделана, часть — нет, и в сделанной части есть один реальный баг. Нужно довести до конца, сохранив уже существующие изменения.

  Сначала прочитай prompt.md целиком — там архитектурные правила и обоснования. Затем выполни пункты ниже. Жёсткое правило из prompt.md
  остаётся в силе: никаких новых FxHashMap / FxHashSet / Vec<T> полей в AnalysisData — каждый индекс это отдельная именованная структура в
  своём модуле crates/svelte_analyze/src/types/data/<name>.rs с типизированным API.

  Перед кодом всегда проверяй cargo check -p svelte_analyze -p svelte_codegen_client. В конце — прогнать тесты: cargo test -p svelte_analyze
  -p svelte_parser (компиляторные тесты пропускай).

  ---
  1. БАГ: коллизия NodeId в script_runes.rs (сделать первым)

  crates/svelte_analyze/src/passes/js_analyze/script_runes.rs:22-27 вызывает collector.visit_program(...) для parsed.program и
  parsed.module_program и пишет call.node_id() без offset'а. OXC назначает NodeId per-parse с нуля, диапазоны пересекаются, второй проход
  затирает первый в FxHashMap<OxcNodeId, RuneKind>.

  Для module script уже есть механика offset'а: crates/svelte_component_semantics/src/builder/js_visitor.rs:58-104 (new_with_offset +
  remap_node_id). Нужно получить тот же offset, что используется в ComponentSemanticsBuilder для module-программы, и применить его в
  коллекторе перед записью.

  - Найди, где в analyze-пайплайне уже вычисляется / хранится этот offset (grep node_id_offset в svelte_component_semantics и в
  svelte_analyze).
  - Прокинь его в collect_script_rune_call_kinds или храни рядом с ParserResult.
  - В Collector::visit_call_expression применяй offset только для module-программы.
  - Альтернативный вариант если offset нигде не шарится снаружи: обрабатывай module_program отдельным проходом с явным offset'ом, равным
  max(instance_program.node_id) + 1. Выбери тот, который не требует дублировать логику offset'а в analyze.

  Тест: добавь компонент с runes и в instance, и в module script, и проверь, что classify-запросы из codegen возвращают правильный RuneKind
  для обоих.

  ---
  2. Удалить мёртвый delegate в AnalysisData

  crates/svelte_analyze/src/types/data/analysis.rs:120-122 — метод script_rune_call_kind(&self, node: OxcNodeId) нигде не используется
  (проверь grep'ом). Удали его. Оставь только script_rune_calls(&self) -> &ScriptRuneCalls, call-site'ы уже зовут
  data.script_rune_calls().kind(node).

  ---
  3. Сузить escape hatches в новых обёртках

  crates/svelte_analyze/src/types/data/import_symbols.rs:28-32 и proxy_state_inits.rs:23-27 экспортируют impl From<FxHashSet<..>> / impl
  From<FxHashMap<..>> как pub. Grep'ом проверь, используются ли они вне crate. Если нет — замени на pub(crate) конструктор (from_raw) или
  удали вовсе и переделай запись через insert/set_proxied в месте заполнения.

  ---
  4. A3 — EachContextIndex → NodeTable<EachBlockEntry> + bitflags

  crates/svelte_analyze/src/types/data/each_context_index.rs сейчас держит 6 NodeBitSet + 4 NodeTable на компонент, где each-блоков единицы.
  Переделай:

  bitflags! {
      pub struct EachBlockFlags: u8 {
          const KEY_USES_INDEX      = 1 << 0;
          const IS_DESTRUCTURED     = 1 << 1;
          const BODY_USES_INDEX     = 1 << 2;
          const KEY_IS_ITEM         = 1 << 3;
          const HAS_ANIMATE         = 1 << 4;
          const CONTAINS_GROUP_BIND = 1 << 5;
          const NEEDS_COLLECTION_ID = 1 << 6;
      }
  }

  pub struct EachBlockEntry {
      flags: EachBlockFlags,
      context_name: Option<CompactString>,
      index_sym: Option<SymbolId>,
      key_node_id: Option<NodeId>,
  }

  Поле bind_this_context: NodeTable<SmallVec<[SymbolId; 4]>> ключится по attr_id, а не по block_id — вынеси его в отдельный модуль
  bind_this_each_context.rs со своей структурой BindThisEachContext. В AnalysisData будет два поля: each_context: EachContextIndex и
  bind_this_each_context: BindThisEachContext.

  Сохрани все существующие публичные методы (index_sym, key_node_id, key_uses_index, is_destructured, body_uses_index, key_is_item,
  has_animate, contains_group_binding, needs_collection_id, context_name, bind_this_context) с той же сигнатурой — внутренняя перестройка не
  должна ломать call-site'ы в codegen.

  index_sym_to_block: FxHashMap<SymbolId, NodeId> оставь как есть — это отдельный reverse-lookup, не флаги.

  ---
  5. A5 — last_child_by_parent обнуление

  crates/svelte_analyze/src/types/data/template_element_index.rs:121,136 — это временное состояние билдера. После того как record больше не
  будет вызываться, обнули через mem::take.

  Вариант: метод TemplateElementIndex::finalize(&mut self), который делает self.last_child_by_parent = FxHashMap::default();. Вызови его в
  конце паса, который заполняет индекс (grep TemplateElementIndex::record → найдёшь пас). Если единственный вызов — record, альтернатива:
  std::mem::take(&mut self.last_child_by_parent) в конце цикла по элементам прямо в пасе.

  ---
  6. A6 — консолидация ElementFacts и TemplateElementIndex проходов

  Сейчас element_facts.rs:12-22 и template_element_index.rs:33-69 независимо итерируются по element.attributes ради частично пересекающихся
  фактов. Объедини:

  - Расширь ElementFactsEntry полями static_id: Option<CompactString>, static_classes: SmallVec<[CompactString; 2]>, has_dynamic_id: bool,
  has_dynamic_class: bool (то, что сейчас в TemplateElementEntry).
  - TemplateElementEntry оставь только с tag_name + parent_element и ссылкой/копией нужных полей, или сделай TemplateElementEntry тонкой
  обёрткой поверх ElementFactsEntry.
  - TemplateElementIndex::record принимает готовый &ElementFactsEntry и строит только reverse-мапы (by_tag, by_static_class, by_static_id,
  maybe_matches_*, previous_sibling, next_sibling).
  - Проход, заполняющий оба индекса (grep element_facts.record и template_elements.record), должен один раз построить ElementFactsEntry и
  передать ссылку в оба.

  Сохрани все публичные методы обоих индексов — никаких изменений в call-site'ах codegen.

  ---
  7. B1 — DirectiveModifierFlags

  Новый модуль crates/svelte_analyze/src/types/data/directive_modifier_flags.rs:

  bitflags! {
      pub struct EventModifier: u16 {
          const ONCE            = 1 << 0;
          const CAPTURE         = 1 << 1;
          const PREVENT_DEFAULT = 1 << 2;
          const STOP_PROPAGATION = 1 << 3;
          const PASSIVE         = 1 << 4;
          const NONPASSIVE      = 1 << 5;
          const TRUSTED         = 1 << 6;
          const SELF            = 1 << 7;
          const GLOBAL          = 1 << 8;
      }
  }

  pub struct DirectiveModifierFlags {
      flags: NodeTable<EventModifier>,
  }

  impl DirectiveModifierFlags {
      pub fn new(node_count: u32) -> Self;
      pub(crate) fn record(&mut self, id: NodeId, flags: EventModifier);
      pub fn get(&self, id: NodeId) -> EventModifier; // empty по умолчанию
      pub fn has(&self, id: NodeId, modifier: EventModifier) -> bool;
  }

  Заполняй в том же пасе, где строится ElementFactsEntry (или в passes/element_flags.rs — посмотри, где сейчас обрабатываются OnDirective).
  Один проход по modifiers: &[String], маппинг имени → флаг.

  Замени все использования modifiers.iter().any(|m| m == "X"):
  - passes/template_validation.rs:933 — passive
  - passes/template_validation.rs:934 — nonpassive
  - passes/template_validation.rs:2373 — != "once"
  - passes/element_flags.rs:255 — once
  - svelte_codegen_client/template/events/actions.rs:102 — global

  Добавь поле directive_modifiers: DirectiveModifierFlags в AnalysisData. Проброс через CodegenView если нужен codegen site'у.

  ---
  8. B2 — NamespaceKind + is_void в ElementFactsEntry

  Не отдельный модуль — поля добавляются в существующий ElementFactsEntry. В element_facts.rs:

  #[derive(Copy, Clone, Eq, PartialEq, Debug)]
  pub enum NamespaceKind { Html, Svg, MathMl, ForeignObject }

  Добавь в ElementFactsEntry: namespace: NamespaceKind, is_void: bool, is_custom_element: bool. Публичные методы ElementFacts::namespace(id)
  -> NamespaceKind, is_void(id) -> bool, is_custom_element(id) -> bool.

  Namespace считай с учётом inheritance от родителя через TemplateTopology — это ключевой момент. Сейчас codegen сам это делает в
  crates/svelte_codegen_client/src/template/mod.rs:91-118 (boundary violation). Логика такая:
  - <svg> → Svg
  - <math> → MathMl
  - <foreignObject> внутри Svg → ForeignObject (переключает контекст обратно на HTML-like)
  - иначе — inherited от родителя; корень — Html.

  Строй в отдельном пасе или в том же, где ElementFactsEntry, но ПОСЛЕ того как TemplateTopology построен — нужен parent lookup. Проверь,
  какой пас заполняет element_facts сейчас и где его место в executor.rs.

  Утилиты svelte_ast::is_svg() / is_mathml() / is_void() после этого зовутся только внутри билдера индекса, не из codegen. Замени
  использования в:
  - codegen_client/template/html_tag.rs:22,34,41-42,68,75-76
  - codegen_client/template/svelte_element.rs:47,106,132,156
  - codegen_client/template/html.rs:123
  - codegen_client/template/mod.rs:112-118 — удалить локальное вычисление inheritance, звать ctx.view.element_facts(id).namespace()
  - passes/lower.rs:6 — если там тоже есть ручное вычисление, заменить

  Проброс через CodegenView: namespace(id) -> NamespaceKind, is_void(id) -> bool, is_custom_element(id) -> bool.

  ---
  9. B5 — SubtreeContainsFlags

  Новый модуль crates/svelte_analyze/src/types/data/subtree_flags.rs:

  bitflags! {
      pub struct SubtreeFlags: u8 {
          const AWAIT          = 1 << 0;
          const BIND_GROUP     = 1 << 1;
          const ANIMATE        = 1 << 2;
          const SLOT           = 1 << 3;
          const DYNAMIC_ATTRS  = 1 << 4;
      }
  }

  pub struct SubtreeContainsIndex {
      flags: NodeTable<SubtreeFlags>,
  }

  impl SubtreeContainsIndex {
      pub fn new(node_count: u32) -> Self;
      pub(crate) fn set(&mut self, id: NodeId, flags: SubtreeFlags);
      pub(crate) fn add(&mut self, id: NodeId, flags: SubtreeFlags); // OR-in
      pub fn contains(&self, id: NodeId, flag: SubtreeFlags) -> bool;
      pub fn flags(&self, id: NodeId) -> SubtreeFlags;
  }

  Заполнение bottom-up: отдельный пас после того, как дерево уже обойдено forward-ом (нужно знать факты per-node). Варианты:
  1. Обход template_topology по постфиксу: для каждого leaf node — посчитать локальные флаги; потом пройти по всем нодам от leaves к корню,
  OR-ить child flags в parent.
  2. Если walker уже делает leave-коллбэки — воткнуться туда.

  Смотри crates/svelte_analyze/src/walker.rs — есть ли leave_* методы? Если есть, используй их: на leave_* для ребёнка делаешь parent_flags |=
   child_flags.

  После введения:
  - EachContextIndex::contains_group_binding удали (из bitflags EachBlockFlags тоже) — замени на subtree_flags.contains(each_id,
  SubtreeFlags::BIND_GROUP). Обнови AnalysisData::contains_group_binding.
  - passes/bind_semantics.rs:168 mark_contains_group_binding → вместо этого просто помечай локальный флаг на attr/element, а propagation
  сделает отдельный пас.
  - FragmentFacts::has_direct_animate_child оставь как есть (direct, не subtree) — это другой вопрос.

  ---
  10. B4 — SymbolRefIndex (reverse SymbolId → [NodeId])

  Новый модуль crates/svelte_analyze/src/types/data/symbol_ref_index.rs:

  pub struct SymbolRefIndex {
      refs: Vec<SmallVec<[NodeId; 4]>>, // private, индекс — SymbolId.as_usize()
  }

  impl SymbolRefIndex {
      pub fn new(symbol_count: usize) -> Self;
      pub(crate) fn record(&mut self, sym: SymbolId, expr_id: NodeId);
      pub fn references(&self, sym: SymbolId) -> &[NodeId];
      pub fn reference_count(&self, sym: SymbolId) -> usize;
      pub fn is_referenced(&self, sym: SymbolId) -> bool;
  }

  Заполнение: в том же проходе, где строится ExpressionInfo и пишется ref_symbols. Найди место (grep ref_symbols.push или ExpressionInfo {
  ref_symbols:), рядом с записью в forward-direction добавь symbol_refs.record(sym, expr_node_id). Никакого отдельного прохода.

  Размер: SymbolRefIndex::new(scoping.symbol_count()) — получи количество символов после того как ComponentScoping полностью построен.

  Не добавляй call-site'ов использования — это задел. Но документируй в rustdoc модуля, для чего.

  ---
  Порядок выполнения

  1. Баг в script_runes.rs (#1) — корректность.
  2. Удалить мёртвый delegate (#2) и сузить escape hatches (#3) — тривиально, 5 минут.
  3. A5 finalize (#5) — тривиально.
  4. A3 EachContextIndex рефакторинг (#4) — средне, изолировано.
  5. A6 консолидация проходов (#6) — средне, изолировано.
  6. B1 DirectiveModifierFlags (#7) — легко, много call-site'ов.
  7. B2 NamespaceKind (#8) — важно, закрывает boundary violation.
  8. B5 SubtreeContainsFlags (#9) — требует B2 сделанным для паттерна пассов.
  9. B4 SymbolRefIndex (#10) — легко, задел без call-site'ов.

  После каждого пункта: cargo check -p svelte_analyze -p svelte_codegen_client. В конце всего: cargo test -p svelte_analyze -p svelte_parser.

  Правила, которые НЕ нарушать

  - Никаких сырых FxHashMap/FxHashSet/Vec<T> полей в AnalysisData.
  - Никаких String-lookup'ов по символам — только SymbolId.
  - Никакого re-traversal AST в codegen ради классификации — всё в analyze side-tables.
  - OXC Visit для любого обхода JS AST, никакого ручного match'а.
  - Если что-то уже используется через grep — сохраняй публичную сигнатуру метода, меняй только внутренности.
  - Если застрял на подходе после 2 попыток — останавливайся, не пытайся обойти. Фиксируй в commit'е то, что работает, и докладывай что не
  получилось.о forward direction: `ExpressionInfo.ref_symbols: SmallVec<SymbolId>`. Обратного вопроса нет →
  невозможно без полного обхода:
  - Unused state runes detection
  - Реактивный reachability
  - Оптимизация `$.template_effect`
  - Диагностики "unused prop"

  Заполняется **в том же проходе**, что строит `ExpressionInfo` — при записи `ref_symbols` дублировать в
  `SymbolRefIndex::record`. Без отдельного прохода. 83+ существующих forward-итераций не меняются.

  ### B5. `SubtreeContainsFlags` — propagated-up флаги поддерева

  Новый модуль: `crates/svelte_analyze/src/types/data/subtree_flags.rs`.

  bitflags! {
      pub struct SubtreeFlags: u8 {
          const AWAIT = ...;
          const BIND_GROUP = ...;
          const ANIMATE = ...;
          const SLOT = ...;
          const DYNAMIC_ATTRS = ...;
      }
  }

  pub struct SubtreeContainsIndex {
      flags: NodeTable,
  }

  impl SubtreeContainsIndex {
      pub fn new(node_count: u32) -> Self { ... }
      pub(crate) fn record(&mut self, id: NodeId, flags: SubtreeFlags) { ... }
      pub fn contains(&self, id: NodeId, flag: SubtreeFlags) -> bool { ... }
      pub fn flags(&self, id: NodeId) -> SubtreeFlags { ... }
  }

  Bottom-up одним проходом через существующий walker: при выходе из ребёнка — OR'им его флаги в родителя.

  Закрывает паттерн «ручное проталкивание флага вверх»:
  - `passes/bind_semantics.rs:170` — `mark_contains_group_binding(each_id)` (ручное проталкивание одного флага,
  прибито гвоздями к each)
  - `FragmentFacts::has_direct_animate_child` — только direct children
  - `passes/template_validation.rs` — ad-hoc рекурсивные обходы

  После введения — `EachContextIndex::contains_group_binding` удаляется, становится `subtree_flags.contains(each_id,
  SubtreeFlags::BIND_GROUP)`.

  ### B6. `ScriptRuneCalls` — замена `script_rune_call_kinds` на типизированный индекс ★ тривиальный фикс

  Новый модуль: `crates/svelte_analyze/src/types/data/script_rune_calls.rs`.

  pub struct ScriptRuneCalls {
      by_node: FxHashMap<OxcNodeId, RuneKind>,  // private
  }

  impl ScriptRuneCalls {
      pub fn new() -> Self { ... }
      pub(crate) fn record(&mut self, node: OxcNodeId, kind: RuneKind) { ... }
      pub fn kind(&self, node: OxcNodeId) -> Option { ... }
      pub fn is_rune_call(&self, node: OxcNodeId) -> bool { ... }
      pub fn iter(&self) -> impl Iterator<Item = (OxcNodeId, RuneKind)> + '_ { ... }
  }

  Сейчас: `crates/svelte_analyze/src/passes/js_analyze/script_runes.rs:17` —
  `self.data.script_rune_call_kinds.insert(call.span.start, kind)`. Тип `FxHashMap<u32, RuneKind>` keyed by
  **span.start** — span-based lookup, ноль type safety, нарушение SymbolId-правила по духу.

  `ident.node_id` / `call.node_id` в OXC AST уже популяется parser'ом, и `ComponentSemanticsBuilder::js_visitor`
  (`crates/svelte_component_semantics/src/builder/js_visitor.rs:94-104`) уже глобализует их через `node_id_offset`.

  Переключить ключ на `oxc_syntax::node::NodeId`. **Не `NodeTable`** — density OXC NodeId после offset-мерджа под
  вопросом; hashmap безопаснее без измерений.

  Затронутые чтения (сейчас принимают `&FxHashMap<u32, RuneKind>` — обновить сигнатуры на `&ScriptRuneCalls` и
  методы):
  - `analysis.rs:116,120` — delegate-методы убрать или переписать на типизированные
  - `codegen_view.rs:46` — `script_rune_calls(&self) -> &ScriptRuneCalls`
  - `codegen_client/context.rs:521` — то же
  - `script/state.rs:85`, `script/pipeline.rs:74,165,173,198,235,255,292`, `script/model.rs:120`, `lib.rs:69` —
  сигнатуры меняются с `&FxHashMap<u32, RuneKind>` на `&ScriptRuneCalls`

  Это также шаблон-прецедент для будущих script-phase side-tables (per-CallExpression / per-Identifier классификация)
   — они должны keyed'иться через OXC NodeId и быть типизированными структурами.

  ### B7. Санация существующих сырых коллекций в `AnalysisData`

  Четыре поля `analysis.rs:36,42,45,46` — `import_syms`, `proxy_state_inits`, `script_rune_call_kinds`,
  `pickled_await_offsets` — сейчас сырые `FxHashSet` / `FxHashMap`. Завернуть каждое в именованную структуру в своём
  модуле:

  - `ImportSymbolSet` (`types/data/import_symbols.rs`) — wraps `FxHashSet<SymbolId>`, API: `contains`, `iter`,
  `insert`.
  - `ProxyStateInits` (`types/data/proxy_state_inits.rs`) — wraps `FxHashMap<CompactString, bool>`, API:
  `is_proxied(name)`, `set_proxied(name, bool)`.
  - `ScriptRuneCalls` — см. B6 (здесь же и ключ меняется).
  - `PickledAwaitOffsets` (`types/data/pickled_await_offsets.rs`) — wraps `FxHashSet<u32>`. TODO-комментарий в
  модуле: «в идеале key'ить по OXC NodeId, не span-offset — требует синхронизации с точкой записи».

  Все текущие helper-методы в `analysis.rs` / `codegen_view.rs`, возвращающие `&FxHashMap<...>` / `&FxHashSet<...>`,
  должны получить типизированный return type (`&ImportSymbolSet` и т.д.) или быть удалены в пользу
  `data.import_symbols.method(...)`.

  ---

  ## Приоритеты (по убыванию пользы / объёма работы)

  1. **B1 DirectiveModifierFlags** — ~час, 5 сайтов, задел на event/a11y
  2. **B6 ScriptRuneCalls (OxcNodeId key + типизация)** — ~20 строк + обход сигнатур, прецедент
  3. **B7 санация сырых полей `AnalysisData`** — механический рефакторинг, обязательное основание для всего
  остального
  4. **A1 bind_this_context SymbolId + A2 parent_each_blocks удалить**
  5. **A4 O(N²) class_candidates/id_candidates → iterator**
  6. **B3 AttrIndex в codegen**
  7. **B2 NamespaceKind + is_void**