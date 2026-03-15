use oxc_ast::ast::Expression;
use rustc_hash::{FxHashMap, FxHashSet};

use svelte_ast::NodeId;
use svelte_js::{ExpressionInfo, RuneKind, ScriptInfo};
use svelte_span::Span;

use crate::scope::ComponentScoping;

// ---------------------------------------------------------------------------
// ParsedExprs — parsed JS expression ASTs in a shared OXC allocator
// ---------------------------------------------------------------------------

/// Parsed JS expression ASTs, stored in a shared OXC allocator.
/// Separate from AnalysisData to avoid lifetime propagation.
pub struct ParsedExprs<'a> {
    /// Template expressions: ExpressionTag, IfBlock test, EachBlock expr, RenderTag, HtmlTag.
    pub exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Attribute expressions: (owner_element_id, attr_index).
    pub attr_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,
    /// ConcatenationAttribute dynamic parts: (owner_id, attr_index, part_index).
    pub concat_part_exprs: FxHashMap<(NodeId, usize, usize), Expression<'a>>,
    /// Pre-parsed script Program AST. Consumed by codegen via `Option::take()`.
    pub script_program: Option<oxc_ast::ast::Program<'a>>,
}

impl<'a> ParsedExprs<'a> {
    pub fn new() -> Self {
        Self {
            exprs: FxHashMap::default(),
            attr_exprs: FxHashMap::default(),
            concat_part_exprs: FxHashMap::default(),
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
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

pub struct AnalysisData {
    /// Lowered + trimmed representation of each fragment.
    pub lowered_fragments: FxHashMap<FragmentKey, LoweredFragment>,
    /// Parsed JS metadata for ExpressionTag nodes (and IfBlock/EachBlock test expressions).
    pub expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Parsed JS metadata for element attribute expressions: (element_id, attr_index).
    pub attr_expressions: FxHashMap<(NodeId, usize), ExpressionInfo>,
    /// Parsed script block declarations.
    pub script: Option<ScriptInfo>,
    /// Unified scope tree for script + template (oxc-based).
    pub scoping: ComponentScoping,
    /// Element attributes that reference rune symbols: (element NodeId, attr index).
    pub dynamic_attrs: FxHashSet<(NodeId, usize)>,
    /// Nodes (ExpressionTag / IfBlock / EachBlock) that reference rune symbols.
    pub dynamic_nodes: FxHashSet<NodeId>,
    /// Elements that need a JS variable reference for reactive updates.
    pub node_needs_ref: FxHashSet<NodeId>,
    /// Content classification for each fragment.
    pub content_types: FxHashMap<FragmentKey, ContentType>,
    /// Compile-time known values for const declarations with literal initializers.
    pub known_values: FxHashMap<String, String>,
    /// NodeIds of IfBlocks whose alternate is an elseif (single IfBlock with elseif: true).
    pub alt_is_elseif: FxHashSet<NodeId>,
    /// Props analysis (from $props() destructuring).
    pub props: Option<PropsAnalysis>,
    /// Exported names from `export const/function/class` or `export { ... }`.
    pub exports: Vec<svelte_js::ExportInfo>,
    /// Snippet parameter names, keyed by snippet NodeId.
    pub snippet_params: FxHashMap<NodeId, Vec<String>>,
    /// Top-level snippets whose bodies don't reference component-scoped variables.
    pub hoistable_snippets: FxHashSet<NodeId>,

    // -- Cached sets for codegen (populated after mutations pass) --
    /// All rune symbol names with their kinds (precomputed).
    pub rune_names: FxHashMap<String, RuneKind>,
    /// All mutated rune names (script assignments + bind directives).
    pub mutated_runes: FxHashSet<String>,
    /// Rune names mutated only via bind directives.
    pub bind_mutated_runes: FxHashSet<String>,
    /// Elements that need a DOM variable during traversal (precomputed for codegen).
    pub elements_needing_var: FxHashSet<NodeId>,

    // -- Element flags (populated by ElementFlagsVisitor) --
    /// Elements that have at least one spread attribute.
    pub element_has_spread: FxHashSet<NodeId>,
    /// Elements that have at least one class directive.
    pub element_has_class_directives: FxHashSet<NodeId>,
    /// Static class attribute Span for elements (value_span of StringAttribute named "class").
    pub element_static_class: FxHashMap<NodeId, Span>,
    /// Input elements that need `$.remove_input_defaults` (have bind or value attr).
    pub needs_input_defaults: FxHashSet<NodeId>,
    /// Fragments whose items contain at least one dynamic child.
    pub fragment_has_dynamic_children: FxHashSet<FragmentKey>,

    /// Component needs runtime context (`$.push`/`$.pop`), e.g. has `$effect` calls.
    pub needs_context: bool,
}

impl AnalysisData {
    pub fn new() -> Self {
        Self {
            lowered_fragments: FxHashMap::default(),
            expressions: FxHashMap::default(),
            attr_expressions: FxHashMap::default(),
            script: None,
            scoping: ComponentScoping::empty(),
            dynamic_attrs: FxHashSet::default(),
            dynamic_nodes: FxHashSet::default(),
            node_needs_ref: FxHashSet::default(),
            content_types: FxHashMap::default(),
            known_values: FxHashMap::default(),
            alt_is_elseif: FxHashSet::default(),
            props: None,
            exports: Vec::new(),
            snippet_params: FxHashMap::default(),
            hoistable_snippets: FxHashSet::default(),
            rune_names: FxHashMap::default(),
            mutated_runes: FxHashSet::default(),
            bind_mutated_runes: FxHashSet::default(),
            elements_needing_var: FxHashSet::default(),
            element_has_spread: FxHashSet::default(),
            element_has_class_directives: FxHashSet::default(),
            element_static_class: FxHashMap::default(),
            needs_input_defaults: FxHashSet::default(),
            fragment_has_dynamic_children: FxHashSet::default(),
            needs_context: false,
        }
    }

    /// Populate cached rune name sets from ComponentScoping.
    /// Call after build_scoping + detect_mutations.
    pub fn cache_rune_sets(&mut self) {
        self.rune_names = self.scoping.rune_names();
        self.mutated_runes = self.scoping.mutated_rune_names();
        self.bind_mutated_runes = self.scoping.bind_mutated_rune_names();
    }
}

impl AnalysisData {
    pub fn is_rune(&self, name: &str) -> bool {
        self.rune_names.contains_key(name)
    }

    pub fn is_mutable_rune(&self, name: &str) -> bool {
        self.mutated_runes.contains(name)
    }

    pub fn rune_kind(&self, name: &str) -> Option<RuneKind> {
        let root = self.scoping.root_scope_id();
        let sym_id = self.scoping.find_binding(root, name)?;
        self.scoping.rune_kind(sym_id)
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
// ContentType — classification of what a fragment contains
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Empty,
    /// Only static text, no expressions.
    StaticText,
    /// Text and/or expressions (at least one expression).
    DynamicText,
    /// Exactly one element, nothing else.
    SingleElement,
    /// Exactly one block (IfBlock or EachBlock), nothing else.
    SingleBlock,
    /// Mix of elements, blocks, and/or text.
    Mixed,
}
