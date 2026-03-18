use oxc_ast::ast::Expression;
use rustc_hash::{FxHashMap, FxHashSet};
use svelte_ast::NodeId;
use svelte_js::{ExpressionInfo, ScriptInfo};

use crate::scope::ComponentScoping;

// ---------------------------------------------------------------------------
// ParsedExprs — parsed JS expression ASTs in a shared OXC allocator
// ---------------------------------------------------------------------------

/// Parsed JS expression ASTs, stored in a shared OXC allocator.
/// Separate from AnalysisData to avoid lifetime propagation.
pub struct ParsedExprs<'a> {
    /// Template expressions: ExpressionTag, IfBlock test, EachBlock expr, RenderTag, HtmlTag.
    pub exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Attribute expressions, keyed by attribute NodeId.
    pub attr_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// ConcatenationAttribute dynamic parts: (attr_id, part_index).
    pub concat_part_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,
    /// EachBlock key expressions: keyed by EachBlock NodeId.
    pub key_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Pre-parsed script Program AST. Consumed by codegen via `Option::take()`.
    pub script_program: Option<oxc_ast::ast::Program<'a>>,
}

impl<'a> ParsedExprs<'a> {
    pub fn new() -> Self {
        Self {
            exprs: FxHashMap::default(),
            attr_exprs: FxHashMap::default(),
            concat_part_exprs: FxHashMap::default(),
            key_exprs: FxHashMap::default(),
            script_program: None,
        }
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
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Grouped sub-structures
// ---------------------------------------------------------------------------

/// Per-element flags populated by ElementFlagsVisitor, reactivity, and needs_var passes.
pub struct ElementFlags {
    pub(crate) has_spread: FxHashSet<NodeId>,
    pub(crate) has_class_directives: FxHashSet<NodeId>,
    pub(crate) has_class_attribute: FxHashSet<NodeId>,
    pub(crate) needs_clsx: FxHashSet<NodeId>,
    /// Pre-extracted static class attribute value (avoids span→string conversion in codegen).
    pub(crate) static_class: FxHashMap<NodeId, String>,
    pub(crate) has_style_directives: FxHashSet<NodeId>,
    /// Pre-extracted static style attribute value (avoids span→string conversion in codegen).
    pub(crate) static_style: FxHashMap<NodeId, String>,
    pub(crate) needs_input_defaults: FxHashSet<NodeId>,
    pub(crate) needs_var: FxHashSet<NodeId>,
    pub(crate) needs_ref: FxHashSet<NodeId>,
    pub(crate) dynamic_attrs: FxHashSet<NodeId>,
}

impl ElementFlags {
    pub fn new() -> Self {
        Self {
            has_spread: FxHashSet::default(),
            has_class_directives: FxHashSet::default(),
            has_class_attribute: FxHashSet::default(),
            needs_clsx: FxHashSet::default(),
            static_class: FxHashMap::default(),
            has_style_directives: FxHashSet::default(),
            static_style: FxHashMap::default(),
            needs_input_defaults: FxHashSet::default(),
            needs_var: FxHashSet::default(),
            needs_ref: FxHashSet::default(),
            dynamic_attrs: FxHashSet::default(),
        }
    }

    pub fn has_spread(&self, id: NodeId) -> bool { self.has_spread.contains(&id) }
    pub fn has_class_directives(&self, id: NodeId) -> bool { self.has_class_directives.contains(&id) }
    pub fn has_class_attribute(&self, id: NodeId) -> bool { self.has_class_attribute.contains(&id) }
    pub fn needs_clsx(&self, id: NodeId) -> bool { self.needs_clsx.contains(&id) }
    pub fn has_style_directives(&self, id: NodeId) -> bool { self.has_style_directives.contains(&id) }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool { self.needs_input_defaults.contains(&id) }
    pub fn needs_var(&self, id: NodeId) -> bool { self.needs_var.contains(&id) }
    pub fn needs_ref(&self, id: NodeId) -> bool { self.needs_ref.contains(&id) }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool { self.dynamic_attrs.contains(&id) }
    pub fn static_class(&self, id: NodeId) -> Option<&str> { self.static_class.get(&id).map(|s| s.as_str()) }
    pub fn static_style(&self, id: NodeId) -> Option<&str> { self.static_style.get(&id).map(|s| s.as_str()) }
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
    pub(crate) params: FxHashMap<NodeId, Vec<String>>,
    pub(crate) hoistable: FxHashSet<NodeId>,
}

impl SnippetData {
    pub fn new() -> Self {
        Self {
            params: FxHashMap::default(),
            hoistable: FxHashSet::default(),
        }
    }

    pub fn params(&self, id: NodeId) -> Option<&Vec<String>> { self.params.get(&id) }
    pub fn is_hoistable(&self, id: NodeId) -> bool { self.hoistable.contains(&id) }
}

/// ConstTag analysis: declared names and per-fragment grouping.
pub struct ConstTagData {
    pub(crate) names: FxHashMap<NodeId, Vec<String>>,
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl ConstTagData {
    pub fn new() -> Self {
        Self {
            names: FxHashMap::default(),
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn names(&self, id: NodeId) -> Option<&Vec<String>> { self.names.get(&id) }
    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.by_fragment.get(key) }
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

pub struct AnalysisData {
    /// Parsed JS metadata for ExpressionTag nodes (and IfBlock/EachBlock test expressions).
    pub expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Parsed JS metadata for attribute expressions, keyed by attribute NodeId.
    pub attr_expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Parsed script block declarations.
    pub script: Option<ScriptInfo>,
    /// Unified scope tree for script + template (oxc-based).
    pub scoping: ComponentScoping,
    /// Nodes (ExpressionTag / IfBlock / EachBlock) that reference rune symbols.
    pub dynamic_nodes: FxHashSet<NodeId>,
    /// NodeIds of IfBlocks whose alternate is an elseif (single IfBlock with elseif: true).
    pub alt_is_elseif: FxHashSet<NodeId>,
    /// Props analysis (from $props() destructuring).
    pub props: Option<PropsAnalysis>,
    /// Exported names from `export const/function/class` or `export { ... }`.
    pub exports: Vec<svelte_js::ExportInfo>,
    /// Component needs runtime context (`$.push`/`$.pop`), e.g. has `$effect` calls.
    pub needs_context: bool,

    /// Per-element flags (spread, class/style directives, needs_var, etc.).
    pub element_flags: ElementFlags,
    /// Fragment lowering and content classification.
    pub fragments: FragmentData,
    /// Snippet parameters and hoistability.
    pub snippets: SnippetData,
    /// ConstTag declared names and per-fragment grouping.
    pub const_tags: ConstTagData,
}

impl AnalysisData {
    pub fn new() -> Self {
        Self {
            expressions: FxHashMap::default(),
            attr_expressions: FxHashMap::default(),
            script: None,
            scoping: ComponentScoping::empty(),
            dynamic_nodes: FxHashSet::default(),
            alt_is_elseif: FxHashSet::default(),
            props: None,
            exports: Vec::new(),
            needs_context: false,
            element_flags: ElementFlags::new(),
            fragments: FragmentData::new(),
            snippets: SnippetData::new(),
            const_tags: ConstTagData::new(),
        }
    }
}

impl AnalysisData {
    pub fn is_dynamic(&self, id: NodeId) -> bool { self.dynamic_nodes.contains(&id) }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool { self.alt_is_elseif.contains(&id) }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> { self.expressions.get(&id) }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> { self.attr_expressions.get(&id) }

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

#[derive(Clone)]
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
    /// Adjacent text nodes and expression tags grouped together.
    TextConcat { parts: Vec<ConcatPart>, has_expr: bool },
}

impl FragmentItem {
    /// Returns `true` if this is a standalone `{expression}` with no surrounding text.
    /// Used by codegen to pass `is_text: true` to `$.child()` / `$.sibling()`.
    pub fn is_standalone_expr(&self) -> bool {
        matches!(self, FragmentItem::TextConcat { parts, .. }
            if parts.len() == 1 && matches!(parts[0], ConcatPart::Expr(_)))
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
            | FragmentItem::SvelteElement(id) => *id,
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

#[derive(Clone)]
pub enum ConcatPart {
    /// Static text content (possibly trimmed).
    Text(String),
    /// Expression tag node id.
    Expr(NodeId),
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
    /// Needs `$.prop()` signal wrapper (vs direct `$$props.name` access).
    pub is_prop_source: bool,
    /// Default value requires lazy evaluation (`() => expr`).
    /// True when `default_text` is present and is not a simple expression.
    pub is_lazy_default: bool,
}

// ---------------------------------------------------------------------------
// ContentStrategy — classification of what a fragment contains, with embedded data
// ---------------------------------------------------------------------------

/// Block-type discriminant for ContentStrategy::SingleBlock.
/// Mirrors FragmentItem block variants so classify_items encodes the type once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleBlockKind {
    IfBlock(NodeId),
    EachBlock(NodeId),
    HtmlTag(NodeId),
    KeyBlock(NodeId),
    RenderTag(NodeId),
    ComponentNode(NodeId),
    SvelteElement(NodeId),
}

impl SingleBlockKind {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::IfBlock(id) | Self::EachBlock(id) | Self::HtmlTag(id)
            | Self::KeyBlock(id) | Self::RenderTag(id) | Self::ComponentNode(id)
            | Self::SvelteElement(id) => *id,
        }
    }
}

/// Describes the content of a fragment. Carries item data so codegen does not
/// need to re-inspect the lowered fragment for common decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentStrategy {
    Empty,
    /// Only static text, no expressions. Contains the pre-extracted text value.
    Static(String),
    /// Exactly one element node. Contains its NodeId.
    SingleElement(NodeId),
    /// Exactly one block node (IfBlock, EachBlock, etc.). Contains the block kind with its NodeId.
    SingleBlock(SingleBlockKind),
    /// Text with expressions, or a mix of elements/blocks/text.
    Dynamic { has_elements: bool, has_blocks: bool, has_text: bool },
}
