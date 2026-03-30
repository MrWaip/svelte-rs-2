use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{ConcatPart, NodeId, StyleDirective};
use svelte_span::Span;

use super::node_table::{NodeBitSet, NodeTable};
use crate::scope::{ComponentScoping, SymbolId};
use super::script::{ExportInfo, ScriptInfo};

pub use svelte_parser::ParserResult;

// ---------------------------------------------------------------------------
// AwaitBindingInfo / DestructureKind — binding patterns for await blocks
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum AwaitBindingInfo {
    Simple(String),
    Destructured { kind: DestructureKind, names: Vec<String> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestructureKind {
    Object,
    Array,
}

impl AwaitBindingInfo {
    pub fn names(&self) -> &[String] {
        match self {
            Self::Simple(name) => std::slice::from_ref(name),
            Self::Destructured { names, .. } => names,
        }
    }
}

// BindingNameCollector and DestructureBindingCollector moved to passes/build_scoping.rs

// ---------------------------------------------------------------------------
// Expression analysis types (created in js_analyze, stored in AnalysisData)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ExpressionInfo {
    pub kind: ExpressionKind,
    /// Resolved SymbolIds referenced in this expression.
    /// Populated by `resolve_references` from OXC reference_id on AST nodes.
    pub ref_symbols: SmallVec<[SymbolId; 2]>,
    /// Expression references a `$X` store subscription.
    pub has_store_ref: bool,
    pub has_side_effects: bool,
    pub has_call: bool,
    /// Expression contains an `await` keyword (requires `experimental.async`).
    pub has_await: bool,
    /// Set when the expression contains `$effect.pending()` — forces the expression to be dynamic.
    pub has_state_rune: bool,
    /// Set when the expression contains a deep mutation on a `$`-prefixed identifier
    /// (e.g., `$store.field = val` or `$store.count++`). Used to determine if component
    /// needs `$.push/$.pop` for `$.store_mutate` support.
    pub has_store_member_mutation: bool,
    /// Expression requires component context (unsafe member/call/new on import/prop).
    /// Aggregated into `AnalysisData::needs_context` for `$.push`/`$.pop`.
    pub needs_context: bool,
    /// Dynamic in template or element-attribute context.
    /// For `expressions`: template semantics (state runes, dynamic bindings, stores, class fields).
    /// For `attr_expressions`: element-attribute semantics (prop_non_source OR is_dynamic_by_id).
    pub is_dynamic: bool,
    /// Dynamic in component/boundary attribute context (Svelte's `has_state` semantics).
    /// Any reference to a rune or non-root-scope binding.
    /// Only meaningful for `attr_expressions`; for regular `expressions`, equals `is_dynamic`.
    pub has_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    Identifier(CompactString),
    Literal,
    CallExpression { callee: CompactString },
    MemberExpression,
    ArrowFunction,
    Assignment,
    Other,
}

impl ExpressionKind {
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::Identifier(_) | Self::MemberExpression)
    }
}

// ---------------------------------------------------------------------------
// FragmentKey — typed key for lowered_fragments and content_types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentKey {
    Root,
    Element(NodeId),
    ComponentNode(NodeId),
    IfConsequent(NodeId),
    IfAlternate(NodeId),
    EachBody(NodeId),
    EachFallback(NodeId),
    SnippetBody(NodeId),
    KeyBlockBody(NodeId),
    SvelteHeadBody(NodeId),
    SvelteElementBody(NodeId),
    SvelteBoundaryBody(NodeId),
    AwaitPending(NodeId),
    AwaitThen(NodeId),
    AwaitCatch(NodeId),
}

impl FragmentKey {
    pub fn is_each_body(&self) -> bool {
        matches!(self, Self::EachBody(_))
    }

    /// Whether a text-first fragment needs `$.next()` to skip the anchor comment.
    /// Matches the reference compiler's `is_text_first` parent whitelist:
    /// Fragment, EachBlock, SnippetBlock, Component, SvelteBoundary, SvelteComponent, SvelteSelf.
    pub fn needs_text_first_next(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::EachBody(_)
                | Self::EachFallback(_)
                | Self::SnippetBody(_)
                | Self::ComponentNode(_)
                | Self::SvelteBoundaryBody(_)
        )
    }

    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::Root => None,
            Self::Element(id) | Self::ComponentNode(id)
            | Self::IfConsequent(id) | Self::IfAlternate(id)
            | Self::EachBody(id) | Self::EachFallback(id)
            | Self::SnippetBody(id) | Self::KeyBlockBody(id)
            | Self::SvelteHeadBody(id) | Self::SvelteElementBody(id)
            | Self::SvelteBoundaryBody(id)
            | Self::AwaitPending(id) | Self::AwaitThen(id) | Self::AwaitCatch(id) => Some(*id),
        }
    }
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Grouped sub-structures
// ---------------------------------------------------------------------------

/// Pre-collected class directive data for codegen (avoids re-traversal).
pub struct ClassDirectiveInfo {
    pub id: NodeId,
    pub name: String,
    pub has_expression: bool,
}

/// Pre-classified component attribute for codegen consumption.
#[derive(Clone)]
pub struct ComponentPropInfo {
    pub kind: ComponentPropKind,
    pub is_dynamic: bool,
}

#[derive(Clone)]
pub enum ComponentPropKind {
    /// `name="value"` — static string
    String { name: String, value_span: Span },
    /// `name` — boolean attribute
    Boolean { name: String },
    /// `name={expr}` — expression (possibly shorthand, possibly memoized)
    Expression {
        name: String,
        attr_id: NodeId,
        shorthand: bool,
        needs_memo: bool,
    },
    /// `name="text{expr}text"` — template concatenation
    Concatenation {
        name: String,
        attr_id: NodeId,
        parts: Vec<ConcatPart>,
    },
    /// `{name}` — shorthand
    Shorthand { attr_id: NodeId, name: String },
    /// `bind:this={expr}`
    BindThis { bind_id: NodeId },
    /// `bind:name` or `bind:name={expr}` — component prop binding (not bind:this)
    Bind {
        name: String,
        bind_id: NodeId,
        mode: ComponentBindMode,
    },
    /// `{...spread}` — spread attribute on component
    Spread { attr_id: NodeId },
    /// `{@attach fn}` — attachment on component, generates `$.attachment()` computed property
    Attach { attr_id: NodeId },
}

/// Getter/setter pattern for component bind directives.
#[derive(Clone, Copy, Debug)]
pub enum ComponentBindMode {
    /// `$bindable()` prop — `name()` / `name($$value)`
    PropSource,
    /// `$state`/`$derived` rune — `$.get(name)` / `$.set(name, $$value)`
    Rune,
    /// Plain variable — `name` / `name = $$value`
    Plain,
}

/// Pre-computed render tag callee routing mode.
///
/// Replaces three separate bool flags (`is_dynamic`, `is_chain`, `callee_is_getter`)
/// with a single enum that codegen can `match` on directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTagCalleeMode {
    /// callee(anchor, ...args) — normal binding, regular call
    Direct,
    /// callee?.(anchor, ...args) — normal binding, optional chain
    Chain,
    /// $.snippet(anchor, thunk(callee), ...args) — non-normal binding
    DynamicRegular,
    /// $.snippet(anchor, () => callee ?? $.noop, ...args) — non-normal + optional chain
    DynamicChain,
}

impl RenderTagCalleeMode {
    pub fn is_dynamic(self) -> bool {
        matches!(self, Self::DynamicRegular | Self::DynamicChain)
    }
    pub fn is_chain(self) -> bool {
        matches!(self, Self::Chain | Self::DynamicChain)
    }
}

/// Pre-computed event handler delegation vs direct binding decision.
#[derive(Debug, Clone, Copy)]
pub enum EventHandlerMode {
    /// Event uses delegation: `$.delegated(name, el, handler, [void 0, true])`
    Delegated { passive: bool },
    /// Event uses direct binding: `$.event(name, el, handler, [capture?, passive?])`
    Direct { capture: bool, passive: bool },
}

/// Per-element flags populated by ElementFlagsVisitor, reactivity, and needs_var passes.
pub struct ElementFlags {
    pub(crate) has_spread: NodeBitSet,
    pub(crate) class_attr_id: NodeTable<NodeId>,
    pub(crate) class_directive_info: NodeTable<Vec<ClassDirectiveInfo>>,
    pub(crate) needs_clsx: NodeBitSet,
    /// Pre-extracted static class attribute value (avoids span→string conversion in codegen).
    pub(crate) static_class: NodeTable<String>,
    pub(crate) style_directives: NodeTable<Vec<StyleDirective>>,
    /// Pre-extracted static style attribute value (avoids span→string conversion in codegen).
    pub(crate) static_style: NodeTable<String>,
    pub(crate) needs_input_defaults: NodeBitSet,
    pub(crate) needs_var: NodeBitSet,
    pub(crate) needs_ref: NodeBitSet,
    pub(crate) dynamic_attrs: NodeBitSet,
    /// Elements with both `contenteditable="true"` and `bind:innerHTML|innerText|textContent`.
    /// Text children use `nodeValue=` init instead of `$.set_text()` update.
    pub(crate) bound_contenteditable: NodeBitSet,
    pub(crate) has_use_directive: NodeBitSet,
    pub(crate) has_dynamic_class_directives: NodeBitSet,
    /// Attribute/directive whose expression is a simple identifier matching the name
    /// (e.g., `class:foo={foo}`, `style:color={color}`). Enables property shorthand in output.
    pub(crate) expression_shorthand: NodeBitSet,
    /// Pre-classified component attributes for codegen (avoids two-pass pattern).
    pub(crate) component_props: NodeTable<Vec<ComponentPropInfo>>,
    /// Pre-computed event handler delegation routing (avoids on-the-fly decision in codegen).
    pub(crate) event_handler_mode: NodeTable<EventHandlerMode>,
}

impl ElementFlags {
    pub fn new(node_count: u32) -> Self {
        Self {
            has_spread: NodeBitSet::new(node_count),
            class_attr_id: NodeTable::new(node_count),
            class_directive_info: NodeTable::new(node_count),
            needs_clsx: NodeBitSet::new(node_count),
            static_class: NodeTable::new(node_count),
            style_directives: NodeTable::new(node_count),
            static_style: NodeTable::new(node_count),
            needs_input_defaults: NodeBitSet::new(node_count),
            needs_var: NodeBitSet::new(node_count),
            needs_ref: NodeBitSet::new(node_count),
            dynamic_attrs: NodeBitSet::new(node_count),
            bound_contenteditable: NodeBitSet::new(node_count),
            has_use_directive: NodeBitSet::new(node_count),
            has_dynamic_class_directives: NodeBitSet::new(node_count),
            expression_shorthand: NodeBitSet::new(node_count),
            component_props: NodeTable::new(node_count),
            event_handler_mode: NodeTable::new(node_count),
        }
    }

    pub fn has_spread(&self, id: NodeId) -> bool {
        self.has_spread.contains(&id)
    }
    pub fn has_class_directives(&self, id: NodeId) -> bool {
        self.class_directive_info.contains_key(id)
    }
    pub fn has_class_attribute(&self, id: NodeId) -> bool {
        self.class_attr_id.contains_key(id)
    }
    pub fn class_attr_id(&self, id: NodeId) -> Option<NodeId> {
        self.class_attr_id.get(id).copied()
    }
    pub fn class_directive_info(&self, id: NodeId) -> Option<&[ClassDirectiveInfo]> {
        self.class_directive_info.get(id).map(|v| v.as_slice())
    }
    pub fn needs_clsx(&self, id: NodeId) -> bool {
        self.needs_clsx.contains(&id)
    }
    pub fn has_style_directives(&self, id: NodeId) -> bool {
        self.style_directives.contains_key(id)
    }
    pub fn style_directives(&self, id: NodeId) -> &[StyleDirective] {
        self.style_directives.get(id).map_or(&[], |v| v.as_slice())
    }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool {
        self.needs_input_defaults.contains(&id)
    }
    pub fn needs_var(&self, id: NodeId) -> bool {
        self.needs_var.contains(&id)
    }
    pub fn needs_ref(&self, id: NodeId) -> bool {
        self.needs_ref.contains(&id)
    }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool {
        self.dynamic_attrs.contains(&id)
    }
    pub fn static_class(&self, id: NodeId) -> Option<&str> {
        self.static_class.get(id).map(|s| s.as_str())
    }
    pub fn static_style(&self, id: NodeId) -> Option<&str> {
        self.static_style.get(id).map(|s| s.as_str())
    }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool {
        self.bound_contenteditable.contains(&id)
    }
    pub fn has_use_directive(&self, id: NodeId) -> bool {
        self.has_use_directive.contains(&id)
    }
    pub fn has_dynamic_class_directives(&self, id: NodeId) -> bool {
        self.has_dynamic_class_directives.contains(&id)
    }
    /// Whether class attribute handling needs state (dynamic class attr or dynamic class directives).
    pub fn class_needs_state(&self, element_id: NodeId) -> bool {
        let class_attr_dynamic = self
            .class_attr_id
            .get(element_id)
            .is_some_and(|&attr_id| self.dynamic_attrs.contains(&attr_id));
        class_attr_dynamic || self.has_dynamic_class_directives.contains(&element_id)
    }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool {
        self.expression_shorthand.contains(&id)
    }
    pub fn component_props(&self, id: NodeId) -> &[ComponentPropInfo] {
        self.component_props.get(id).map_or(&[], |v| v.as_slice())
    }
    pub fn event_handler_mode(&self, attr_id: NodeId) -> Option<EventHandlerMode> {
        self.event_handler_mode.get(attr_id).copied()
    }
}

/// Fragment lowering results and content classification.
pub struct FragmentData {
    pub(crate) lowered: FxHashMap<FragmentKey, LoweredFragment>,
    pub(crate) content_types: FxHashMap<FragmentKey, ContentStrategy>,
    pub(crate) has_dynamic_children: FxHashSet<FragmentKey>,
    /// Pre-computed blocker indices per fragment (experimental.async).
    /// Aggregated from all text expression parts in the fragment.
    pub(crate) fragment_blockers: FxHashMap<FragmentKey, SmallVec<[u32; 2]>>,
}

impl FragmentData {
    pub fn new() -> Self {
        Self {
            lowered: FxHashMap::default(),
            content_types: FxHashMap::default(),
            has_dynamic_children: FxHashSet::default(),
            fragment_blockers: FxHashMap::default(),
        }
    }

    pub fn with_capacity(estimated_fragments: usize) -> Self {
        Self {
            lowered: FxHashMap::with_capacity_and_hasher(estimated_fragments, Default::default()),
            content_types: FxHashMap::with_capacity_and_hasher(
                estimated_fragments,
                Default::default(),
            ),
            has_dynamic_children: FxHashSet::with_capacity_and_hasher(
                estimated_fragments / 4,
                Default::default(),
            ),
            fragment_blockers: FxHashMap::default(),
        }
    }

    pub fn content_type(&self, key: &FragmentKey) -> ContentStrategy {
        self.content_types
            .get(key)
            .cloned()
            .unwrap_or(ContentStrategy::Empty)
    }

    pub fn has_dynamic_children(&self, key: &FragmentKey) -> bool {
        self.has_dynamic_children.contains(key)
    }

    pub fn lowered(&self, key: &FragmentKey) -> Option<&LoweredFragment> {
        self.lowered.get(key)
    }

    /// Pre-computed blocker indices for a fragment's text expressions.
    pub fn fragment_blockers(&self, key: &FragmentKey) -> &[u32] {
        self.fragment_blockers.get(key).map_or(&[], |v| v.as_slice())
    }
}

/// Snippet analysis: hoistability and component-snippet grouping.
pub struct SnippetData {
    pub(crate) hoistable: NodeBitSet,
    /// Key: ComponentNode NodeId → snippet NodeIds declared in its fragment
    pub(crate) component_snippets: NodeTable<Vec<NodeId>>,
    /// Pre-computed param names from the parsed `const name = (a, b) => {}` arrow.
    pub(crate) params: NodeTable<Vec<String>>,
}

impl SnippetData {
    pub fn new(node_count: u32) -> Self {
        Self {
            hoistable: NodeBitSet::new(node_count),
            component_snippets: NodeTable::new(node_count),
            params: NodeTable::new(node_count),
        }
    }

    pub fn is_hoistable(&self, id: NodeId) -> bool {
        self.hoistable.contains(&id)
    }
    pub fn component_snippets(&self, id: NodeId) -> &[NodeId] {
        self.component_snippets
            .get(id)
            .map_or(&[], |v| v.as_slice())
    }
    pub fn params(&self, id: NodeId) -> &[String] {
        self.params.get(id).map_or(&[], |v| v.as_slice())
    }
}

/// ConstTag analysis: declared names and per-fragment grouping.
pub struct ConstTagData {
    pub(crate) names: NodeTable<Vec<String>>,
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl ConstTagData {
    pub fn new(node_count: u32) -> Self {
        Self {
            names: NodeTable::new(node_count),
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn names(&self, id: NodeId) -> Option<&Vec<String>> {
        self.names.get(id)
    }
    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

/// DebugTag per-fragment grouping.
pub struct DebugTagData {
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl DebugTagData {
    pub fn new() -> Self {
        Self {
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

/// TitleElement per-fragment grouping (<title> inside <svelte:head>).
pub struct TitleElementData {
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl TitleElementData {
    pub fn new() -> Self {
        Self {
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

/// Each-block analysis data — side tables populated during build_scoping.
pub struct EachBlockData {
    /// SymbolId of the index variable.
    pub(crate) index_syms: NodeTable<SymbolId>,
    /// Reverse lookup: index SymbolId → block NodeId. Built from index_syms before Walk 3.
    pub(crate) index_sym_to_block: FxHashMap<SymbolId, NodeId>,
    /// NodeId of the key expression (block.id → key_id).
    pub(crate) key_node_ids: NodeTable<NodeId>,
    /// Key expression references the index variable (needs index param in key arrow).
    pub(crate) key_uses_index: NodeBitSet,
    /// Context is a destructuring pattern (`{ name, value }` or `[a, b]`).
    pub(crate) is_destructured: NodeBitSet,
    /// Body expressions reference the index variable (needs index param in render fn).
    pub(crate) body_uses_index: NodeBitSet,
    /// Key expression is the same simple identifier as the context variable.
    pub(crate) key_is_item: NodeBitSet,
    /// Body contains an element with an `animate:` directive.
    pub(crate) has_animate: NodeBitSet,
    /// Context variable name: simple identifier name or `"$$item"` for destructured.
    pub(crate) context_names: NodeTable<String>,
}

impl EachBlockData {
    pub fn new(node_count: u32) -> Self {
        Self {
            index_syms: NodeTable::new(node_count),
            index_sym_to_block: FxHashMap::default(),
            key_node_ids: NodeTable::new(node_count),
            key_uses_index: NodeBitSet::new(node_count),
            is_destructured: NodeBitSet::new(node_count),
            body_uses_index: NodeBitSet::new(node_count),
            key_is_item: NodeBitSet::new(node_count),
            has_animate: NodeBitSet::new(node_count),
            context_names: NodeTable::new(node_count),
        }
    }

    /// Build reverse lookup from index_syms. Call after Walk 2 populates index_syms.
    pub(crate) fn build_index_lookup(&mut self) {
        self.index_sym_to_block = self.index_syms.iter()
            .map(|(block_id, &sym)| (sym, block_id))
            .collect();
    }

    pub fn index_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.index_syms.get(id).copied()
    }

    pub fn key_uses_index(&self, id: NodeId) -> bool {
        self.key_uses_index.contains(&id)
    }
    pub fn is_destructured(&self, id: NodeId) -> bool {
        self.is_destructured.contains(&id)
    }
    pub fn body_uses_index(&self, id: NodeId) -> bool {
        self.body_uses_index.contains(&id)
    }
    pub fn key_is_item(&self, id: NodeId) -> bool {
        self.key_is_item.contains(&id)
    }
    pub fn has_animate(&self, id: NodeId) -> bool {
        self.has_animate.contains(&id)
    }
    pub fn context_name(&self, id: NodeId) -> &str {
        self.context_names.get(id).map_or("$$item", |s| s.as_str())
    }
}

/// Await block binding patterns, parsed via OXC in the `parse_js` pass.
pub struct AwaitBindingData {
    /// Then binding info, keyed by AwaitBlock NodeId.
    pub(crate) values: NodeTable<AwaitBindingInfo>,
    /// Catch binding info, keyed by AwaitBlock NodeId.
    pub(crate) errors: NodeTable<AwaitBindingInfo>,
}

impl AwaitBindingData {
    pub fn new(node_count: u32) -> Self {
        Self {
            values: NodeTable::new(node_count),
            errors: NodeTable::new(node_count),
        }
    }

    pub fn value(&self, id: NodeId) -> Option<&AwaitBindingInfo> {
        self.values.get(id)
    }
    pub fn error(&self, id: NodeId) -> Option<&AwaitBindingInfo> {
        self.errors.get(id)
    }
}

/// Pre-computed bind/directive semantics for codegen.
///
/// Eliminates string-based symbol re-resolution in codegen: instead of
/// `source_text(span) → find_binding → is_rune && is_mutated`, codegen
/// reads pre-computed flags keyed by NodeId.
pub struct BindSemanticsData {
    /// Bind directives / class directives / style directives whose target
    /// is a mutable rune (needs `$.get()`/`$.set()` instead of plain access).
    /// Key: directive NodeId.
    pub(crate) mutable_rune_targets: NodeBitSet,
    /// Nodes whose expression resolves to a prop source
    /// (each_block collection, render_tag argument identifiers).
    /// Key: EachBlock NodeId or RenderTag argument NodeId.
    pub(crate) prop_source_nodes: NodeBitSet,
    /// Pre-computed each-block variable names referenced in bind:this expressions.
    /// Key: BindDirective NodeId. Value: names of each-block vars used in the expression.
    pub(crate) bind_each_context: NodeTable<Vec<String>>,
    /// Elements that have a `bind:group` directive.
    /// Their `value` attribute uses the `__value` pattern instead of `$.set_value`.
    pub(crate) has_bind_group: NodeBitSet,
    /// bind:group directive → NodeId of the value attribute on the same element (if any).
    /// Used to build the getter thunk that evaluates the value expression.
    pub(crate) bind_group_value_attr: NodeTable<NodeId>,
    /// bind:group directive → ancestor each block NodeIds whose context vars
    /// appear in the binding expression (inner-to-outer order).
    pub(crate) parent_each_blocks: NodeTable<Vec<NodeId>>,
    /// Each blocks that need a generated `$$index` parameter for group binding.
    pub(crate) contains_group_binding: NodeBitSet,
    /// Pre-computed blocker indices for bind directive targets (experimental.async).
    pub(crate) bind_blockers: NodeTable<SmallVec<[u32; 2]>>,
}

impl BindSemanticsData {
    pub fn new(node_count: u32) -> Self {
        Self {
            mutable_rune_targets: NodeBitSet::new(node_count),
            prop_source_nodes: NodeBitSet::new(node_count),
            bind_each_context: NodeTable::new(node_count),
            has_bind_group: NodeBitSet::new(node_count),
            bind_group_value_attr: NodeTable::new(node_count),
            parent_each_blocks: NodeTable::new(node_count),
            contains_group_binding: NodeBitSet::new(node_count),
            bind_blockers: NodeTable::new(node_count),
        }
    }

    pub fn is_mutable_rune_target(&self, id: NodeId) -> bool {
        self.mutable_rune_targets.contains(&id)
    }

    pub fn is_prop_source(&self, id: NodeId) -> bool {
        self.prop_source_nodes.contains(&id)
    }

    pub fn each_context(&self, id: NodeId) -> Option<&Vec<String>> {
        self.bind_each_context.get(id)
    }

    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.has_bind_group.contains(&id)
    }

    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.bind_group_value_attr.get(id).copied()
    }

    pub fn parent_each_blocks(&self, id: NodeId) -> Option<&Vec<NodeId>> {
        self.parent_each_blocks.get(id)
    }

    pub fn contains_group_binding(&self, id: NodeId) -> bool {
        self.contains_group_binding.contains(&id)
    }

    /// Pre-computed blocker indices for a bind directive target.
    pub fn bind_blockers(&self, id: NodeId) -> &[u32] {
        self.bind_blockers.get(id).map_or(&[], |v| v.as_slice())
    }
}

// ---------------------------------------------------------------------------
// IgnoreData — svelte-ignore suppression tracking
// ---------------------------------------------------------------------------

/// Per-node ignore snapshots for `<!-- svelte-ignore -->` comment suppression.
/// Snapshots are interned: most nodes share the same (empty) set.
#[derive(Debug, Default)]
pub struct IgnoreData {
    /// NodeId → snapshot index into `snapshots`.
    node_snapshot: FxHashMap<NodeId, u32>,
    /// Interned ignore sets. Index 0 is always the empty set.
    snapshots: Vec<FxHashSet<String>>,
    /// Dedup map: sorted codes → snapshot index.
    intern: FxHashMap<Vec<String>, u32>,
}

impl IgnoreData {
    pub fn new() -> Self {
        let empty_set = FxHashSet::default();
        let mut intern = FxHashMap::default();
        intern.insert(Vec::new(), 0);
        Self {
            node_snapshot: FxHashMap::default(),
            snapshots: vec![empty_set],
            intern,
        }
    }

    /// Check if a warning code is ignored for the given node.
    pub fn is_ignored(&self, node_id: NodeId, code: &str) -> bool {
        self.node_snapshot
            .get(&node_id)
            .and_then(|&idx| self.snapshots.get(idx as usize))
            .is_some_and(|set| set.contains(code))
    }

    /// Intern a set of ignored codes and return the snapshot index.
    pub(crate) fn intern_snapshot(&mut self, codes: &FxHashSet<String>) -> u32 {
        let mut sorted: Vec<String> = codes.iter().cloned().collect();
        sorted.sort();
        if let Some(&idx) = self.intern.get(&sorted) {
            return idx;
        }
        let idx = self.snapshots.len() as u32;
        self.snapshots.push(codes.clone());
        self.intern.insert(sorted, idx);
        idx
    }

    /// Record the ignore snapshot for a node.
    pub(crate) fn set_snapshot(&mut self, node_id: NodeId, idx: u32) {
        if idx != 0 {
            self.node_snapshot.insert(node_id, idx);
        }
    }
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

pub struct AnalysisData {
    /// Parsed JS metadata for ExpressionTag nodes (and IfBlock/EachBlock test expressions).
    pub expressions: NodeTable<ExpressionInfo>,
    /// Parsed JS metadata for attribute expressions, keyed by attribute NodeId.
    pub attr_expressions: NodeTable<ExpressionInfo>,
    /// Parsed script block declarations.
    pub script: Option<ScriptInfo>,
    /// Unified scope tree for script + template (oxc-based).
    pub scoping: ComponentScoping,
    /// Nodes (ExpressionTag / IfBlock / EachBlock) that reference rune symbols.
    pub dynamic_nodes: NodeBitSet,
    /// NodeIds of IfBlocks whose alternate is an elseif (single IfBlock with elseif: true).
    pub alt_is_elseif: NodeBitSet,
    /// Props analysis (from $props() destructuring).
    pub props: Option<PropsAnalysis>,
    /// Binding name from `const id = $props.id()`.
    pub props_id: Option<String>,
    /// Exported names from `export const/function/class` or `export { ... }`.
    pub exports: Vec<ExportInfo>,
    /// Component needs runtime context (`$.push`/`$.pop`), e.g. has `$effect` calls.
    pub needs_context: bool,
    /// Script contains class declarations with `$state`/`$state.raw` fields.
    /// When true, member access on local bindings is treated as dynamic.
    pub has_class_state_fields: bool,

    /// Per-element flags (spread, class/style directives, needs_var, etc.).
    pub element_flags: ElementFlags,
    /// Fragment lowering and content classification.
    pub fragments: FragmentData,
    /// Snippet parameters and hoistability.
    pub snippets: SnippetData,
    /// ConstTag declared names and per-fragment grouping.
    pub const_tags: ConstTagData,
    /// DebugTag per-fragment grouping.
    pub debug_tags: DebugTagData,
    /// TitleElement per-fragment grouping (<title> inside <svelte:head>).
    pub title_elements: TitleElementData,
    /// Each-block context/index names.
    pub each_blocks: EachBlockData,
    /// Per-argument `has_call` flags for render tag expressions (keyed by RenderTag NodeId).
    pub render_tag_arg_has_call: NodeTable<Vec<bool>>,
    /// Per-argument prop-source SymbolId for render tags.
    /// Some(sym) = prop-source arg (pass getter directly), None = not a prop-source.
    pub render_tag_prop_sources: NodeTable<Vec<Option<SymbolId>>>,
    /// Callee SymbolId for render tags (resolved during resolve_references).
    pub(crate) render_tag_callee_sym: NodeTable<SymbolId>,
    /// Intermediate: render tags with ChainExpression callee (`{@render fn?.()}`).
    /// Consumed by `resolve_render_tag_dynamic` to compute `render_tag_callee_mode`.
    pub(crate) render_tag_is_chain: NodeBitSet,
    /// Pre-computed render tag callee routing (replaces separate is_dynamic/is_chain/is_getter flags).
    pub render_tag_callee_mode: NodeTable<RenderTagCalleeMode>,
    /// Source offsets for template node expressions (NodeId → span.start).
    /// Populated during extract_all_expressions, consumed by codegen for O(1) ParsedExprs lookup.
    pub node_expr_offsets: NodeTable<u32>,
    /// Source offsets for attribute expressions (NodeId → span.start).
    pub attr_expr_offsets: NodeTable<u32>,
    /// Await block binding patterns (then/catch), parsed via OXC.
    pub await_bindings: AwaitBindingData,
    /// Pre-computed bind/directive semantics (mutable rune targets, prop sources).
    pub bind_semantics: BindSemanticsData,
    /// Pre-computed import SymbolIds from root scope (O(1) lookup in codegen).
    pub import_syms: FxHashSet<SymbolId>,
    /// Whether this component is compiled as a custom element.
    /// When true, all props become prop sources with getter/setter exports.
    pub custom_element: bool,
    /// Parsed custom element config (from object expression form).
    pub ce_config: Option<svelte_parser::ParsedCeConfig>,
    /// $state/$state.raw declarations with proxyable init (array/object/non-primitive).
    /// Keyed by declaration name. Computed in analyze_script, consumed in build_scoping.
    pub(crate) proxy_state_inits: FxHashMap<compact_str::CompactString, bool>,
    /// True when script contains deep mutations on `$`-prefixed identifiers
    /// (e.g., `$store.field = val`). Triggers `$.push/$.pop` for `$.store_mutate`.
    pub(crate) has_store_member_mutations: bool,
    /// Blocker tracking for `experimental.async`: which script bindings depend on async operations.
    pub(crate) blocker_data: BlockerData,
    /// svelte-ignore suppression data (per-node ignore snapshots).
    pub ignore_data: IgnoreData,
}

// ---------------------------------------------------------------------------
// Blocker tracking (experimental.async)
// ---------------------------------------------------------------------------

/// Per-symbol blocker info for instance body async splitting.
/// A blocker index N means the binding is written by the N-th async thunk,
/// so template reads must wait for `$$promises[N]`.
#[derive(Debug, Default)]
pub struct BlockerData {
    /// SymbolId → blocker index (the index into the $$promises array).
    pub(crate) symbol_blockers: FxHashMap<SymbolId, u32>,
    /// Number of async thunks in the instance body.
    pub(crate) async_thunk_count: u32,
    /// Whether the instance body has any async statements.
    pub(crate) has_async: bool,
    /// Index of the first non-import statement with top-level await.
    /// Indices count only non-import statements (1:1 with `ScriptOutput.body`).
    pub(crate) first_await_index: Option<usize>,
    /// Per-statement metadata for statements at/after `first_await_index`.
    /// Indexed as `stmt_metas[i - first_await_index]` where `i` is the
    /// non-import statement index.
    pub(crate) stmt_metas: Vec<AsyncStmtMeta>,
}

/// Pre-computed metadata for one non-import statement in the async split region.
/// Produced by `calculate_instance_blockers`, consumed by codegen's `split_async_instance_body`.
#[derive(Debug, Clone)]
pub struct AsyncStmtMeta {
    /// Statement has top-level await (not inside nested functions).
    pub(crate) has_await: bool,
    /// Binding names that need hoisting for this statement.
    /// For variables: all declarator bindings except function-valued inits.
    /// For classes: the class name.
    pub(crate) hoist_names: Vec<String>,
}

impl AsyncStmtMeta {
    pub fn has_await(&self) -> bool {
        self.has_await
    }
    pub fn hoist_names(&self) -> &[String] {
        &self.hoist_names
    }
}

impl BlockerData {
    /// Whether the instance body has any async statements.
    pub fn has_async(&self) -> bool {
        self.has_async
    }

    /// Get blocker index for a symbol, if any.
    pub fn symbol_blocker(&self, sym: SymbolId) -> Option<u32> {
        self.symbol_blockers.get(&sym).copied()
    }

    /// Index of the first non-import statement with top-level await.
    pub fn first_await_index(&self) -> Option<usize> {
        self.first_await_index
    }

    /// Get metadata for a non-import statement at the given index.
    /// Only valid for indices `>= first_await_index`.
    pub fn stmt_meta(&self, stmt_index: usize) -> Option<&AsyncStmtMeta> {
        let first = self.first_await_index?;
        self.stmt_metas.get(stmt_index - first)
    }
}

impl AnalysisData {
    /// Create AnalysisData with all fields defaulted.
    /// `scoping` is left uninitialized — caller must assign it before use.
    pub(crate) fn new_empty(node_count: u32) -> Self {
        Self {
            expressions: NodeTable::new(node_count),
            attr_expressions: NodeTable::new(node_count),
            script: None,
            scoping: ComponentScoping::new(None),
            dynamic_nodes: NodeBitSet::new(node_count),
            alt_is_elseif: NodeBitSet::new(node_count),
            props: None,
            props_id: None,
            exports: Vec::new(),
            needs_context: false,
            has_class_state_fields: false,
            element_flags: ElementFlags::new(node_count),
            fragments: FragmentData::with_capacity(node_count as usize / 3),
            snippets: SnippetData::new(node_count),
            const_tags: ConstTagData::new(node_count),
            debug_tags: DebugTagData::new(),
            title_elements: TitleElementData::new(),
            each_blocks: EachBlockData::new(node_count),
            render_tag_arg_has_call: NodeTable::new(node_count),
            render_tag_prop_sources: NodeTable::new(node_count),
            render_tag_callee_sym: NodeTable::new(node_count),
            render_tag_is_chain: NodeBitSet::new(node_count),
            render_tag_callee_mode: NodeTable::new(node_count),
            node_expr_offsets: NodeTable::new(node_count),
            attr_expr_offsets: NodeTable::new(node_count),
            await_bindings: AwaitBindingData::new(node_count),
            bind_semantics: BindSemanticsData::new(node_count),
            import_syms: FxHashSet::default(),
            custom_element: false,
            ce_config: None,
            proxy_state_inits: FxHashMap::default(),
            has_store_member_mutations: false,
            blocker_data: BlockerData::default(),
            ignore_data: IgnoreData::new(),
        }
    }
}

impl AnalysisData {
    pub fn blocker_data(&self) -> &BlockerData {
        &self.blocker_data
    }
    pub fn is_dynamic(&self, id: NodeId) -> bool {
        self.dynamic_nodes.contains(&id)
    }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool {
        self.alt_is_elseif.contains(&id)
    }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.expressions.get(id)
    }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.attr_expressions.get(id)
    }
    pub fn node_expr_offset(&self, id: NodeId) -> u32 {
        *self
            .node_expr_offsets
            .get(id)
            .unwrap_or_else(|| panic!("no expr offset for node {:?}", id))
    }
    pub fn attr_expr_offset(&self, id: NodeId) -> u32 {
        *self
            .attr_expr_offsets
            .get(id)
            .unwrap_or_else(|| panic!("no expr offset for attr {:?}", id))
    }
    /// Whether the attribute's expression references an imported symbol (first reference).
    /// Import identifiers may be live bindings — codegen needs getters/wrapping.
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.attr_expressions
            .get(attr_id)
            .and_then(|info| info.ref_symbols.first())
            .is_some_and(|&sym| self.import_syms.contains(&sym))
    }
    pub fn render_tag_arg_has_call(&self, id: NodeId) -> Option<&[bool]> {
        self.render_tag_arg_has_call.get(id).map(|v| v.as_slice())
    }
    pub fn render_tag_prop_sources(&self, id: NodeId) -> Option<&[Option<SymbolId>]> {
        self.render_tag_prop_sources.get(id).map(|v| v.as_slice())
    }
    pub fn render_tag_callee_mode(&self, id: NodeId) -> RenderTagCalleeMode {
        self.render_tag_callee_mode
            .get(id)
            .copied()
            .unwrap_or(RenderTagCalleeMode::Direct)
    }

    /// Component attribute needs `$.derived()` memoization:
    /// has a function call, OR is a non-simple dynamic expression.
    pub fn component_attr_needs_memo(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(attr_id).is_some_and(|e| {
            e.has_call || (!e.kind.is_simple() && self.element_flags.is_dynamic_attr(attr_id))
        })
    }

    /// Expression has a function call or await AND references to resolved bindings — needs `$.derived` wrapping.
    pub fn needs_expr_memoization(&self, id: NodeId) -> bool {
        self.expressions
            .get(id)
            .is_some_and(|e| (e.has_call || e.has_await) && !e.ref_symbols.is_empty())
    }

    /// Check if an expression has blockers (references bindings with blocker metadata).
    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        if !self.blocker_data.has_async {
            return false;
        }
        self.expressions.get(id).is_some_and(|info| {
            info.ref_symbols.iter().any(|sym| self.blocker_data.symbol_blockers.contains_key(sym))
        })
    }

    /// Collect unique blocker indices referenced by an expression's dependencies.
    /// Returns sorted, deduplicated blocker indices.
    pub fn expression_blockers(&self, id: NodeId) -> SmallVec<[u32; 2]> {
        let mut result = SmallVec::new();
        if !self.blocker_data.has_async {
            return result;
        }
        if let Some(info) = self.expressions.get(id) {
            for sym in &info.ref_symbols {
                if let Some(&idx) = self.blocker_data.symbol_blockers.get(sym) {
                    if !result.contains(&idx) {
                        result.push(idx);
                    }
                }
            }
        }
        result.sort_unstable();
        result
    }

    /// Collect unique blocker indices referenced by an attribute expression's dependencies.
    /// Same as `expression_blockers()` but reads from `attr_expressions` (directives, not template).
    pub fn attr_expression_blockers(&self, id: NodeId) -> SmallVec<[u32; 2]> {
        let mut result = SmallVec::new();
        if !self.blocker_data.has_async {
            return result;
        }
        if let Some(info) = self.attr_expressions.get(id) {
            for sym in &info.ref_symbols {
                if let Some(&idx) = self.blocker_data.symbol_blockers.get(sym) {
                    if !result.contains(&idx) {
                        result.push(idx);
                    }
                }
            }
        }
        result.sort_unstable();
        result
    }

    /// Known compile-time value for a name at root scope (looks up SymbolId internally).
    pub fn known_value(&self, name: &str) -> Option<&str> {
        let root = self.scoping.root_scope_id();
        let sym_id = self.scoping.find_binding(root, name)?;
        self.scoping.known_value_by_sym(sym_id)
    }

    /// Check if any expression in a fragment (recursively for elements) references
    /// any of the given symbols. Used to decide whether snippet bodies need
    /// duplicated @const tags in boundary codegen.
    pub fn fragment_references_any_symbol(&self, key: &FragmentKey, syms: &FxHashSet<SymbolId>) -> bool {
        if syms.is_empty() { return false; }
        let Some(fragment) = self.fragments.lowered(key) else { return false };
        for item in &fragment.items {
            match item {
                FragmentItem::TextConcat { parts, .. } => {
                    for part in parts {
                        if let LoweredTextPart::Expr(id) = part {
                            if self.expressions.get(*id).is_some_and(|info| info.ref_symbols.iter().any(|s| syms.contains(s))) {
                                return true;
                            }
                        }
                    }
                }
                FragmentItem::Element(el_id) => {
                    if self.fragment_references_any_symbol(&FragmentKey::Element(*el_id), syms) {
                        return true;
                    }
                }
                FragmentItem::IfBlock(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self.fragment_references_any_symbol(&FragmentKey::IfConsequent(*id), syms)
                        || self.fragment_references_any_symbol(&FragmentKey::IfAlternate(*id), syms) {
                        return true;
                    }
                }
                FragmentItem::EachBlock(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self.fragment_references_any_symbol(&FragmentKey::EachBody(*id), syms)
                        || self.fragment_references_any_symbol(&FragmentKey::EachFallback(*id), syms) {
                        return true;
                    }
                }
                FragmentItem::RenderTag(id) | FragmentItem::HtmlTag(id) => {
                    if self.node_expr_references_syms(*id, syms) {
                        return true;
                    }
                }
                FragmentItem::KeyBlock(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self.fragment_references_any_symbol(&FragmentKey::KeyBlockBody(*id), syms) {
                        return true;
                    }
                }
                FragmentItem::SvelteElement(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self.fragment_references_any_symbol(&FragmentKey::SvelteElementBody(*id), syms) {
                        return true;
                    }
                }
                FragmentItem::SvelteBoundary(id) => {
                    if self.fragment_references_any_symbol(&FragmentKey::SvelteBoundaryBody(*id), syms) {
                        return true;
                    }
                }
                FragmentItem::ComponentNode(id) => {
                    if self.fragment_references_any_symbol(&FragmentKey::ComponentNode(*id), syms) {
                        return true;
                    }
                }
                FragmentItem::AwaitBlock(id) => {
                    if self.node_expr_references_syms(*id, syms) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if a node's expression references any of the given symbols.
    fn node_expr_references_syms(&self, id: NodeId, syms: &FxHashSet<SymbolId>) -> bool {
        self.expressions.get(id).is_some_and(|info| info.ref_symbols.iter().any(|s| syms.contains(s)))
    }
}

// ---------------------------------------------------------------------------
// LoweredFragment — trimmed + grouped representation of a fragment
// ---------------------------------------------------------------------------

pub struct LoweredFragment {
    pub items: Vec<FragmentItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FragmentItem {
    /// A standalone element node.
    Element(NodeId),
    /// A component instantiation.
    ComponentNode(NodeId),
    /// An IfBlock (has its own sub-fragments in lowered_fragments).
    IfBlock(NodeId),
    /// An EachBlock (has its own sub-fragments in lowered_fragments).
    EachBlock(NodeId),
    /// A RenderTag ({@render snippet(args)}).
    RenderTag(NodeId),
    /// An HtmlTag ({@html expr}).
    HtmlTag(NodeId),
    /// A KeyBlock ({#key expr}...{/key}).
    KeyBlock(NodeId),
    /// A SvelteElement (<svelte:element this={tag}>).
    SvelteElement(NodeId),
    /// A SvelteBoundary (<svelte:boundary>).
    SvelteBoundary(NodeId),
    /// An AwaitBlock ({#await expr}...{/await}).
    AwaitBlock(NodeId),
    /// Adjacent text nodes and expression tags grouped together.
    TextConcat {
        parts: Vec<LoweredTextPart>,
        has_expr: bool,
    },
}

impl FragmentItem {
    /// Returns `true` if this is a standalone `{expression}` with no surrounding text.
    /// Used by codegen to pass `is_text: true` to `$.child()` / `$.sibling()`.
    pub fn is_standalone_expr(&self) -> bool {
        matches!(self, FragmentItem::TextConcat { parts, .. }
            if parts.len() == 1 && matches!(parts[0], LoweredTextPart::Expr(_)))
    }

    /// Extract the `NodeId` from any single-node variant.
    /// Panics on `TextConcat` (which has no single id).
    pub fn node_id(&self) -> NodeId {
        match self {
            FragmentItem::Element(id)
            | FragmentItem::ComponentNode(id)
            | FragmentItem::IfBlock(id)
            | FragmentItem::EachBlock(id)
            | FragmentItem::RenderTag(id)
            | FragmentItem::HtmlTag(id)
            | FragmentItem::KeyBlock(id)
            | FragmentItem::SvelteElement(id)
            | FragmentItem::SvelteBoundary(id)
            | FragmentItem::AwaitBlock(id) => *id,
            FragmentItem::TextConcat { .. } => panic!("TextConcat has no single NodeId"),
        }
    }
}

impl LoweredFragment {
    /// Get the first item's NodeId if it is an Element.
    pub fn first_element_id(&self) -> Option<NodeId> {
        match self.items.first()? {
            FragmentItem::Element(id) => Some(*id),
            _ => None,
        }
    }

    /// Get the first item's NodeId if it is an IfBlock.
    pub fn first_if_block_id(&self) -> Option<NodeId> {
        match self.items.first()? {
            FragmentItem::IfBlock(id) => Some(*id),
            _ => None,
        }
    }

    /// Get the first item's NodeId if it is an EachBlock.
    pub fn first_each_block_id(&self) -> Option<NodeId> {
        match self.items.first()? {
            FragmentItem::EachBlock(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredTextPart {
    /// Unmodified text — reference source via span (zero-alloc).
    TextSpan(svelte_span::Span),
    /// Trimmed/modified text that differs from source (heap-allocated).
    TextOwned(String),
    /// Expression tag node id.
    Expr(NodeId),
}

impl LoweredTextPart {
    /// Get text value, resolving spans against source.
    pub fn text_value<'a>(&'a self, source: &'a str) -> Option<&'a str> {
        match self {
            LoweredTextPart::TextSpan(span) => Some(span.source_text(source)),
            LoweredTextPart::TextOwned(s) => Some(s.as_str()),
            LoweredTextPart::Expr(_) => None,
        }
    }

    /// Returns true if this is a text part (span or owned).
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            LoweredTextPart::TextSpan(_) | LoweredTextPart::TextOwned(_)
        )
    }
}

// ---------------------------------------------------------------------------
// PropsAnalysis — analysis of $props() destructuring
// ---------------------------------------------------------------------------

pub struct PropsAnalysis {
    pub props: Vec<PropAnalysis>,
    pub has_bindable: bool,
    /// `const props = $props()` — identifier pattern, not destructured
    pub is_identifier_pattern: bool,
}

pub struct PropAnalysis {
    pub local_name: String,
    pub prop_name: String,
    pub default_span: Option<svelte_span::Span>,
    pub default_text: Option<String>,
    pub is_bindable: bool,
    pub is_rest: bool,
    /// Default value requires lazy evaluation (`() => expr`).
    /// True when `default_text` is present and is not a simple expression.
    pub is_lazy_default: bool,
    /// Prop needs `$.prop()` source (has default, is mutated, or custom element).
    pub is_prop_source: bool,
    /// Prop's rune symbol is mutated (reassigned somewhere in script/template).
    pub is_mutated: bool,
    /// Prop name starts with `$$` — reserved, excluded from CE accessors.
    pub is_reserved: bool,
}

// ---------------------------------------------------------------------------
// ContentStrategy — classification of what a fragment contains, with embedded data
// ---------------------------------------------------------------------------

/// Describes the content of a fragment. Carries item data so codegen does not
/// need to re-inspect the lowered fragment for common decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentStrategy {
    Empty,
    /// Only static text, no expressions. Contains the pre-extracted text value.
    Static(String),
    /// Exactly one element node. Contains its NodeId.
    SingleElement(NodeId),
    /// Exactly one block node (IfBlock, EachBlock, etc.). Stores the FragmentItem directly.
    SingleBlock(FragmentItem),
    /// Text with expressions (no elements or blocks).
    DynamicText,
    /// Mix of elements, blocks, and/or text.
    Mixed {
        has_elements: bool,
        has_blocks: bool,
        has_text: bool,
    },
}
