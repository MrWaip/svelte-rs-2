use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{ConcatPart, NodeId, StyleDirective};
use svelte_span::Span;

use crate::node_table::{NodeBitSet, NodeTable};
use crate::scope::{ComponentScoping, SymbolId};
use crate::script_types::{ExportInfo, ScriptInfo};

pub use svelte_parser::ParsedExprs;

// ---------------------------------------------------------------------------
// Expression analysis types (created in js_analyze, stored in AnalysisData)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ExpressionInfo {
    pub kind: ExpressionKind,
    pub references: SmallVec<[Reference; 2]>,
    pub has_side_effects: bool,
    pub has_call: bool,
    /// Set when the expression contains `$effect.pending()` — forces the expression to be dynamic.
    pub has_state_rune: bool,
    /// Set when the expression contains a deep mutation on a `$`-prefixed identifier
    /// (e.g., `$store.field = val` or `$store.count++`). Used to determine if component
    /// needs `$.push/$.pop` for `$.store_mutate` support.
    pub has_store_member_mutation: bool,
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub(crate) name: CompactString,
    pub(crate) span: Span,
    pub(crate) flags: ReferenceFlags,
    /// Resolved after `resolve_references` pass. `None` for globals/unresolved.
    pub(crate) symbol_id: Option<SymbolId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceFlags {
    Read,
    Write,
    ReadWrite,
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
    Expression { name: String, attr_id: NodeId, shorthand: bool, needs_memo: bool },
    /// `name="text{expr}text"` — template concatenation
    Concatenation { name: String, attr_id: NodeId, parts: Vec<ConcatPart> },
    /// `{name}` — shorthand
    Shorthand { attr_id: NodeId, name: String },
    /// `bind:this={expr}`
    BindThis { bind_id: NodeId },
    /// `bind:name` or `bind:name={expr}` — component prop binding (not bind:this)
    Bind { name: String, bind_id: NodeId, mode: ComponentBindMode },
    /// `{...spread}` — tracked but skipped in prop building
    Spread,
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

    pub fn has_spread(&self, id: NodeId) -> bool { self.has_spread.contains(&id) }
    pub fn has_class_directives(&self, id: NodeId) -> bool { self.class_directive_info.contains_key(id) }
    pub fn has_class_attribute(&self, id: NodeId) -> bool { self.class_attr_id.contains_key(id) }
    pub fn class_attr_id(&self, id: NodeId) -> Option<NodeId> { self.class_attr_id.get(id).copied() }
    pub fn class_directive_info(&self, id: NodeId) -> Option<&[ClassDirectiveInfo]> { self.class_directive_info.get(id).map(|v| v.as_slice()) }
    pub fn needs_clsx(&self, id: NodeId) -> bool { self.needs_clsx.contains(&id) }
    pub fn has_style_directives(&self, id: NodeId) -> bool { self.style_directives.contains_key(id) }
    pub fn style_directives(&self, id: NodeId) -> &[StyleDirective] { self.style_directives.get(id).map_or(&[], |v| v.as_slice()) }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool { self.needs_input_defaults.contains(&id) }
    pub fn needs_var(&self, id: NodeId) -> bool { self.needs_var.contains(&id) }
    pub fn needs_ref(&self, id: NodeId) -> bool { self.needs_ref.contains(&id) }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool { self.dynamic_attrs.contains(&id) }
    pub fn static_class(&self, id: NodeId) -> Option<&str> { self.static_class.get(id).map(|s| s.as_str()) }
    pub fn static_style(&self, id: NodeId) -> Option<&str> { self.static_style.get(id).map(|s| s.as_str()) }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool { self.bound_contenteditable.contains(&id) }
    pub fn has_use_directive(&self, id: NodeId) -> bool { self.has_use_directive.contains(&id) }
    pub fn has_dynamic_class_directives(&self, id: NodeId) -> bool { self.has_dynamic_class_directives.contains(&id) }
    /// Whether class attribute handling needs state (dynamic class attr or dynamic class directives).
    pub fn class_needs_state(&self, element_id: NodeId) -> bool {
        let class_attr_dynamic = self.class_attr_id.get(element_id)
            .is_some_and(|&attr_id| self.dynamic_attrs.contains(&attr_id));
        class_attr_dynamic || self.has_dynamic_class_directives.contains(&element_id)
    }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool { self.expression_shorthand.contains(&id) }
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
}

impl FragmentData {
    pub fn new() -> Self {
        Self {
            lowered: FxHashMap::default(),
            content_types: FxHashMap::default(),
            has_dynamic_children: FxHashSet::default(),
        }
    }

    pub fn with_capacity(estimated_fragments: usize) -> Self {
        Self {
            lowered: FxHashMap::with_capacity_and_hasher(estimated_fragments, Default::default()),
            content_types: FxHashMap::with_capacity_and_hasher(estimated_fragments, Default::default()),
            has_dynamic_children: FxHashSet::with_capacity_and_hasher(estimated_fragments / 4, Default::default()),
        }
    }

    pub fn content_type(&self, key: &FragmentKey) -> ContentStrategy {
        self.content_types.get(key).cloned().unwrap_or(ContentStrategy::Empty)
    }

    pub fn has_dynamic_children(&self, key: &FragmentKey) -> bool {
        self.has_dynamic_children.contains(key)
    }

    pub fn lowered(&self, key: &FragmentKey) -> Option<&LoweredFragment> {
        self.lowered.get(key)
    }
}

/// Snippet analysis: parameter names and hoistability.
pub struct SnippetData {
    pub(crate) params: NodeTable<Vec<String>>,
    pub(crate) hoistable: NodeBitSet,
}

impl SnippetData {
    pub fn new(node_count: u32) -> Self {
        Self {
            params: NodeTable::new(node_count),
            hoistable: NodeBitSet::new(node_count),
        }
    }

    pub fn params(&self, id: NodeId) -> Option<&Vec<String>> { self.params.get(id) }
    pub fn is_hoistable(&self, id: NodeId) -> bool { self.hoistable.contains(&id) }
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

    pub fn names(&self, id: NodeId) -> Option<&Vec<String>> { self.names.get(id) }
    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.by_fragment.get(key) }
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

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.by_fragment.get(key) }
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

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.by_fragment.get(key) }
}

/// Each-block context/index names, extracted from source text during scope building.
pub struct EachBlockData {
    pub(crate) context_names: NodeTable<String>,
    pub(crate) index_names: NodeTable<String>,
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
}

impl EachBlockData {
    pub fn new(node_count: u32) -> Self {
        Self {
            context_names: NodeTable::new(node_count),
            index_names: NodeTable::new(node_count),
            key_uses_index: NodeBitSet::new(node_count),
            is_destructured: NodeBitSet::new(node_count),
            body_uses_index: NodeBitSet::new(node_count),
            key_is_item: NodeBitSet::new(node_count),
            has_animate: NodeBitSet::new(node_count),
        }
    }

    pub fn context_name(&self, id: NodeId) -> Option<&str> {
        self.context_names.get(id).map(|s| s.as_str())
    }

    pub fn index_name(&self, id: NodeId) -> Option<&str> {
        self.index_names.get(id).map(|s| s.as_str())
    }

    pub fn key_uses_index(&self, id: NodeId) -> bool { self.key_uses_index.contains(&id) }
    pub fn is_destructured(&self, id: NodeId) -> bool { self.is_destructured.contains(&id) }
    pub fn body_uses_index(&self, id: NodeId) -> bool { self.body_uses_index.contains(&id) }
    pub fn key_is_item(&self, id: NodeId) -> bool { self.key_is_item.contains(&id) }
    pub fn has_animate(&self, id: NodeId) -> bool { self.has_animate.contains(&id) }
}

/// Await block binding patterns, parsed via OXC in the `parse_js` pass.
pub struct AwaitBindingData {
    /// Then binding info, keyed by AwaitBlock NodeId.
    pub(crate) values: NodeTable<svelte_parser::AwaitBindingInfo>,
    /// Catch binding info, keyed by AwaitBlock NodeId.
    pub(crate) errors: NodeTable<svelte_parser::AwaitBindingInfo>,
}

impl AwaitBindingData {
    pub fn new(node_count: u32) -> Self {
        Self {
            values: NodeTable::new(node_count),
            errors: NodeTable::new(node_count),
        }
    }

    pub fn value(&self, id: NodeId) -> Option<&svelte_parser::AwaitBindingInfo> { self.values.get(id) }
    pub fn error(&self, id: NodeId) -> Option<&svelte_parser::AwaitBindingInfo> { self.errors.get(id) }
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
    /// Intermediate: per-argument identifier name (if the arg is a plain identifier).
    /// Consumed by `resolve_render_tag_prop_sources` after props analysis.
    pub(crate) render_tag_arg_idents: NodeTable<Vec<Option<String>>>,
    /// Per-argument prop-source SymbolId for render tags.
    /// Some(sym) = prop-source arg (pass getter directly), None = not a prop-source.
    pub render_tag_prop_sources: NodeTable<Vec<Option<SymbolId>>>,
    /// Callee identifier name for render tags (only set when callee is an Identifier).
    pub(crate) render_tag_callee_name: NodeTable<String>,
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
            render_tag_arg_idents: NodeTable::new(node_count),
            render_tag_prop_sources: NodeTable::new(node_count),
            render_tag_callee_name: NodeTable::new(node_count),
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
        }
    }
}

impl AnalysisData {
    pub fn is_dynamic(&self, id: NodeId) -> bool { self.dynamic_nodes.contains(&id) }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool { self.alt_is_elseif.contains(&id) }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> { self.expressions.get(id) }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> { self.attr_expressions.get(id) }
    pub fn node_expr_offset(&self, id: NodeId) -> u32 {
        *self.node_expr_offsets.get(id).unwrap_or_else(|| panic!("no expr offset for node {:?}", id))
    }
    pub fn attr_expr_offset(&self, id: NodeId) -> u32 {
        *self.attr_expr_offsets.get(id).unwrap_or_else(|| panic!("no expr offset for attr {:?}", id))
    }
    /// Whether the attribute's expression references an imported symbol (first reference).
    /// Import identifiers may be live bindings — codegen needs getters/wrapping.
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(attr_id)
            .and_then(|info| info.references.first())
            .and_then(|r| r.symbol_id)
            .is_some_and(|sym| self.import_syms.contains(&sym))
    }
    pub fn render_tag_arg_has_call(&self, id: NodeId) -> Option<&[bool]> { self.render_tag_arg_has_call.get(id).map(|v| v.as_slice()) }
    pub fn render_tag_prop_sources(&self, id: NodeId) -> Option<&[Option<SymbolId>]> { self.render_tag_prop_sources.get(id).map(|v| v.as_slice()) }
    pub fn render_tag_callee_mode(&self, id: NodeId) -> RenderTagCalleeMode {
        self.render_tag_callee_mode.get(id).copied().unwrap_or(RenderTagCalleeMode::Direct)
    }

    /// Component attribute needs `$.derived()` memoization:
    /// has a function call, OR is a non-simple dynamic expression.
    pub fn component_attr_needs_memo(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(attr_id).is_some_and(|e|
            e.has_call || (!e.kind.is_simple() && self.element_flags.is_dynamic_attr(attr_id))
        )
    }

    /// Expression has a function call AND references to resolved bindings — needs `$.derived` wrapping.
    pub fn needs_expr_memoization(&self, id: NodeId) -> bool {
        self.expressions.get(id).is_some_and(|e|
            e.has_call && e.references.iter().any(|r| r.symbol_id.is_some())
        )
    }

    /// Known compile-time value for a name at root scope (looks up SymbolId internally).
    pub fn known_value(&self, name: &str) -> Option<&str> {
        let root = self.scoping.root_scope_id();
        let sym_id = self.scoping.find_binding(root, name)?;
        self.scoping.known_value_by_sym(sym_id)
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
    TextConcat { parts: Vec<LoweredTextPart>, has_expr: bool },
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
        matches!(self, LoweredTextPart::TextSpan(_) | LoweredTextPart::TextOwned(_))
    }
}

// ---------------------------------------------------------------------------
// PropsAnalysis — analysis of $props() destructuring
// ---------------------------------------------------------------------------

pub struct PropsAnalysis {
    pub props: Vec<PropAnalysis>,
    pub has_bindable: bool,
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
    Mixed { has_elements: bool, has_blocks: bool, has_text: bool },
}
