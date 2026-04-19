use super::*;

// Re-export from svelte_ast — the canonical definition.
pub use svelte_ast::FragmentKey;

/// Extension methods on FragmentKey used only by analyze/codegen.
pub trait FragmentKeyExt {
    fn needs_text_first_next(&self) -> bool;
}

impl FragmentKeyExt for FragmentKey {
    fn needs_text_first_next(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::EachBody(_)
                | Self::EachFallback(_)
                | Self::SnippetBody(_)
                | Self::ComponentNode(_)
                | Self::NamedSlot(_, _)
                | Self::SvelteBoundaryBody(_)
        )
    }
}

pub struct FragmentData {
    pub(crate) lowered: FxHashMap<FragmentKey, LoweredFragment>,
    pub(crate) content_types: FxHashMap<FragmentKey, ContentStrategy>,
    pub(crate) has_dynamic_children: FxHashSet<FragmentKey>,
    pub(crate) fragment_blockers: FxHashMap<FragmentKey, SmallVec<[u32; 2]>>,
}

impl Default for FragmentData {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn fragment_blockers(&self, key: &FragmentKey) -> &[u32] {
        self.fragment_blockers
            .get(key)
            .map_or(&[], |v| v.as_slice())
    }
}

pub struct LoweredFragment {
    pub items: Vec<FragmentItem>,
}

/// Legacy per-fragment dispatcher enum. Conflates two unrelated things:
///
/// - Real lowering facts with no AST analog: `TextConcat` (merged
///   text+expression runs), hoisted-node filtering, whitespace
///   normalization. These stay useful.
/// - Node-kind discrimination that now duplicates Block Semantics
///   (`*Block`, `*Tag`) and ElementShape Semantics (`Element`,
///   `ComponentNode`, `SvelteElement`, `SvelteBoundary`,
///   `SlotElementLegacy`, `SvelteFragmentLegacy`). Consumers should
///   ask the semantic cluster for node-kind decisions, not pattern-
///   match on this enum.
///
/// Marked deprecated so new consumer code does not grow more call
/// sites. Planned removal: the "Kill `FragmentItem`" slice documented
/// in `SEMANTIC_LAYER_ARCHITECTURE.md` — it replaces the dispatcher
/// with a semantic-first walk over Svelte AST plus a separate
/// representation for `TextConcat` and hoisted-node filtering.
#[deprecated(note = "FragmentItem duplicates Block / ElementShape Semantics for \
            node-kind discrimination. New consumers must ask the \
            owning semantic cluster instead of pattern-matching this \
            enum. See SEMANTIC_LAYER_ARCHITECTURE.md ('Prerequisite: \
            Kill FragmentItem').")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FragmentItem {
    Element(NodeId),
    /// LEGACY(svelte4): lowered `<slot>` item. Deprecated in Svelte 5, remove in Svelte 6.
    SlotElementLegacy(NodeId),
    /// LEGACY(svelte4): lowered `<svelte:fragment>` wrapper item. Deprecated in Svelte 5, remove in Svelte 6.
    SvelteFragmentLegacy(NodeId),
    ComponentNode(NodeId),
    IfBlock(NodeId),
    EachBlock(NodeId),
    RenderTag(NodeId),
    HtmlTag(NodeId),
    KeyBlock(NodeId),
    SvelteElement(NodeId),
    SvelteBoundary(NodeId),
    AwaitBlock(NodeId),
    TextConcat {
        parts: Vec<LoweredTextPart>,
        has_expr: bool,
    },
}

impl FragmentItem {
    pub fn is_standalone_expr(&self) -> bool {
        matches!(self, FragmentItem::TextConcat { parts, .. }
            if parts.len() == 1 && matches!(parts[0], LoweredTextPart::Expr(_)))
    }

    pub fn element_like_id(&self) -> Option<NodeId> {
        match self {
            FragmentItem::Element(id)
            | FragmentItem::SlotElementLegacy(id)
            | FragmentItem::SvelteFragmentLegacy(id) => Some(*id),
            _ => None,
        }
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            FragmentItem::Element(id)
            | FragmentItem::SlotElementLegacy(id)
            | FragmentItem::SvelteFragmentLegacy(id)
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
    pub fn first_element_id(&self) -> Option<NodeId> {
        self.items.first()?.element_like_id()
    }

    pub fn first_if_block_id(&self) -> Option<NodeId> {
        match self.items.first()? {
            FragmentItem::IfBlock(id) => Some(*id),
            _ => None,
        }
    }

    pub fn first_each_block_id(&self) -> Option<NodeId> {
        match self.items.first()? {
            FragmentItem::EachBlock(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredTextPart {
    TextSpan(svelte_span::Span),
    TextOwned(String),
    Expr(NodeId),
}

impl LoweredTextPart {
    pub fn text_value<'a>(&'a self, source: &'a str) -> Option<&'a str> {
        match self {
            LoweredTextPart::TextSpan(span) => Some(span.source_text(source)),
            LoweredTextPart::TextOwned(s) => Some(s.as_str()),
            LoweredTextPart::Expr(_) => None,
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(
            self,
            LoweredTextPart::TextSpan(_) | LoweredTextPart::TextOwned(_)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentStrategy {
    Empty,
    Static(String),
    SingleElement(NodeId),
    SingleBlock(FragmentItem),
    DynamicText,
    Mixed {
        has_elements: bool,
        has_blocks: bool,
        has_text: bool,
    },
}
