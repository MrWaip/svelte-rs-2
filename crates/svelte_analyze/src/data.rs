use std::collections::{HashMap, HashSet};

use svelte_span::Span;
use svelte_ast::NodeId;
use svelte_js::{DeclarationKind, ExpressionInfo, RuneKind, ScriptInfo};

// ---------------------------------------------------------------------------
// SymbolId — typed index into symbols Vec
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub u32);

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
    /// All top-level symbols declared in the script block.
    pub symbols: Vec<SymbolInfo>,
    /// Index into `symbols` by name.
    pub symbol_by_name: HashMap<String, SymbolId>,
    /// Which symbols are Svelte runes.
    pub runes: HashMap<SymbolId, RuneKind>,
    /// Element attributes that reference rune symbols: (element NodeId, attr index).
    pub dynamic_attrs: HashSet<(NodeId, usize)>,
    /// Nodes (ExpressionTag / IfBlock / EachBlock) that reference rune symbols.
    pub dynamic_nodes: HashSet<NodeId>,
    /// Elements that need a JS variable reference for reactive updates.
    pub node_needs_ref: HashSet<NodeId>,
    /// Content classification for each fragment.
    pub content_types: HashMap<FragmentKey, ContentType>,
}

impl AnalysisData {
    pub fn new() -> Self {
        Self {
            lowered_fragments: HashMap::new(),
            expressions: HashMap::new(),
            attr_expressions: HashMap::new(),
            script: None,
            symbols: Vec::new(),
            symbol_by_name: HashMap::new(),
            runes: HashMap::new(),
            dynamic_attrs: HashSet::new(),
            dynamic_nodes: HashSet::new(),
            node_needs_ref: HashSet::new(),
            content_types: HashMap::new(),
        }
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
// SymbolInfo — a declared symbol from the script block
// ---------------------------------------------------------------------------

pub struct SymbolInfo {
    pub name: String,
    pub span: Span,
    pub kind: DeclarationKind,
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
