use std::collections::{HashMap, HashSet};

use svelte_ast::NodeId;
use svelte_js::{ExpressionInfo, RuneKind, ScriptInfo};

use crate::scope::ComponentScoping;

// ---------------------------------------------------------------------------
// FragmentKey — typed key for lowered_fragments and content_types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentKey {
    Root,
    Element(NodeId),
    IfConsequent(NodeId),
    IfAlternate(NodeId),
    EachBody(NodeId),
    EachFallback(NodeId),
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

pub struct AnalysisData {
    /// Lowered + trimmed representation of each fragment.
    pub lowered_fragments: HashMap<FragmentKey, LoweredFragment>,
    /// Parsed JS metadata for ExpressionTag nodes (and IfBlock/EachBlock test expressions).
    pub expressions: HashMap<NodeId, ExpressionInfo>,
    /// Parsed JS metadata for element attribute expressions: (element_id, attr_index).
    pub attr_expressions: HashMap<(NodeId, usize), ExpressionInfo>,
    /// Parsed script block declarations.
    pub script: Option<ScriptInfo>,
    /// Unified scope tree for script + template (oxc-based).
    pub scoping: ComponentScoping,
    /// Element attributes that reference rune symbols: (element NodeId, attr index).
    pub dynamic_attrs: HashSet<(NodeId, usize)>,
    /// Nodes (ExpressionTag / IfBlock / EachBlock) that reference rune symbols.
    pub dynamic_nodes: HashSet<NodeId>,
    /// Elements that need a JS variable reference for reactive updates.
    pub node_needs_ref: HashSet<NodeId>,
    /// Content classification for each fragment.
    pub content_types: HashMap<FragmentKey, ContentType>,
    /// Compile-time known values for const declarations with literal initializers.
    pub known_values: HashMap<String, String>,
    /// NodeIds of IfBlocks whose alternate is an elseif (single IfBlock with elseif: true).
    pub alt_is_elseif: HashSet<NodeId>,
    /// Props analysis (from $props() destructuring).
    pub props: Option<PropsAnalysis>,

    // -- Cached sets for codegen (populated after mutations pass) --
    /// All rune symbol names (precomputed).
    pub rune_names: HashSet<String>,
    /// All mutated rune names (script assignments + bind directives).
    pub mutated_runes: HashSet<String>,
    /// Rune names mutated only via bind directives.
    pub bind_mutated_runes: HashSet<String>,
}

impl AnalysisData {
    pub fn new() -> Self {
        Self {
            lowered_fragments: HashMap::new(),
            expressions: HashMap::new(),
            attr_expressions: HashMap::new(),
            script: None,
            scoping: ComponentScoping::empty(),
            dynamic_attrs: HashSet::new(),
            dynamic_nodes: HashSet::new(),
            node_needs_ref: HashSet::new(),
            content_types: HashMap::new(),
            known_values: HashMap::new(),
            alt_is_elseif: HashSet::new(),
            props: None,
            rune_names: HashSet::new(),
            mutated_runes: HashSet::new(),
            bind_mutated_runes: HashSet::new(),
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
        self.rune_names.contains(name)
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
    /// An IfBlock (has its own sub-fragments in lowered_fragments).
    IfBlock(NodeId),
    /// An EachBlock (has its own sub-fragments in lowered_fragments).
    EachBlock(NodeId),
    /// Adjacent text nodes and expression tags grouped together.
    TextConcat { parts: Vec<ConcatPart> },
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
