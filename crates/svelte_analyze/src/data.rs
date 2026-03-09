use std::collections::{HashMap, HashSet};

use span::Span;
use svelte_ast::NodeId;
use svelte_js::{DeclarationKind, ExpressionInfo, RuneKind, ScriptInfo};

// ---------------------------------------------------------------------------
// Sentinel NodeId for the root component fragment
// ---------------------------------------------------------------------------

/// Key for the root `Component::fragment` in `lowered_fragments` and `content_types`.
pub const ROOT_FRAGMENT_ID: NodeId = NodeId(u32::MAX);

/// Key for the alternate branch of an IfBlock.
/// Convention: block_id.0 | ALTERNATE_MASK.
/// Safe because the parser's NodeIdAllocator never produces IDs >= 2^30.
const ALTERNATE_MASK: u32 = 1 << 30;

pub fn alternate_id(block_id: NodeId) -> NodeId {
    NodeId(block_id.0 | ALTERNATE_MASK)
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

pub struct AnalysisData {
    /// Lowered + trimmed representation of each fragment, keyed by owner NodeId.
    pub lowered_fragments: HashMap<NodeId, LoweredFragment>,
    /// Parsed JS metadata for ExpressionTag nodes (and IfBlock/EachBlock test expressions).
    pub expressions: HashMap<NodeId, ExpressionInfo>,
    /// Parsed script block declarations.
    pub script: Option<ScriptInfo>,
    /// All top-level symbols declared in the script block.
    pub symbols: Vec<SymbolInfo>,
    /// Index into `symbols` by name.
    pub symbol_by_name: HashMap<String, usize>,
    /// Which symbols (by index into `symbols`) are Svelte runes.
    pub runes: HashMap<usize, RuneKind>,
    /// Element attributes that reference rune symbols: (element NodeId, attr index).
    pub dynamic_attrs: HashSet<(NodeId, usize)>,
    /// Nodes (ExpressionTag / IfBlock / EachBlock) that reference rune symbols.
    pub dynamic_nodes: HashSet<NodeId>,
    /// Elements that need a JS variable reference for reactive updates.
    pub node_needs_ref: HashSet<NodeId>,
    /// Content classification for each fragment owner.
    pub content_types: HashMap<NodeId, ContentType>,
}

impl AnalysisData {
    pub fn new() -> Self {
        Self {
            lowered_fragments: HashMap::new(),
            expressions: HashMap::new(),
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

pub enum FragmentItem {
    /// A standalone element node.
    Element(NodeId),
    /// An IfBlock or EachBlock (has its own sub-fragments in lowered_fragments).
    Block(NodeId),
    /// Adjacent text nodes and expression tags grouped together.
    TextConcat { parts: Vec<ConcatPart> },
}

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
    /// Mix of elements, blocks, and/or text.
    Mixed,
}
