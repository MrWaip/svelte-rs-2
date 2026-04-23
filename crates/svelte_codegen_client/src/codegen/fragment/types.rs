use bitflags::bitflags;
use smallvec::SmallVec;
use svelte_ast::NodeId;

use crate::codegen::data_structures::ConcatPart;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ContentStrategy {
    Empty,

    SingleStatic,

    SingleExpr(NodeId),

    SingleConcat,

    SingleElement(NodeId),

    SingleBlock(NodeId),

    CssWrappedComponent(NodeId),

    ControlledEach(NodeId),

    Multi {
        count: u16,
        has_elements: bool,
        has_blocks: bool,
        has_text: bool,
        first_is_block: bool,
        first_is_text_like: bool,
    },
}

bitflags! {

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
     pub(super) struct ChildrenFlags: u8 {
        const HAS_TEXT     = 1 << 0;
        const HAS_EXPR     = 1 << 1;
        const HAS_CONCAT   = 1 << 2;
        const HAS_ELEMENT  = 1 << 3;
        const HAS_BLOCK    = 1 << 4;
    }
}

#[derive(Copy, Clone)]
pub(super) enum StrategyKind {
    SingleElement,
    Multi,
}

impl StrategyKind {
    pub(super) fn of(strategy: &ContentStrategy) -> Self {
        match strategy {
            ContentStrategy::Multi { .. } => StrategyKind::Multi,
            _ => StrategyKind::SingleElement,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Child {
    Text(ConcatPart),
    Expr(NodeId),
    Concat(SmallVec<[ConcatPart; 4]>),
    Node(NodeId),
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(super) struct HoistedBucket {
    pub snippets: SmallVec<[NodeId; 2]>,
    pub const_tags: SmallVec<[NodeId; 2]>,
    pub debug_tags: SmallVec<[NodeId; 2]>,
    pub svelte_head: SmallVec<[NodeId; 1]>,
    pub svelte_window: SmallVec<[NodeId; 1]>,
    pub svelte_document: SmallVec<[NodeId; 1]>,
    pub svelte_body: SmallVec<[NodeId; 1]>,
    pub titles: SmallVec<[NodeId; 1]>,
}

impl HoistedBucket {
    pub(super) fn is_empty(&self) -> bool {
        self.snippets.is_empty() && self.is_empty_ignoring_snippets()
    }

    pub(super) fn is_empty_ignoring_snippets(&self) -> bool {
        self.const_tags.is_empty()
            && self.debug_tags.is_empty()
            && self.svelte_head.is_empty()
            && self.svelte_window.is_empty()
            && self.svelte_document.is_empty()
            && self.svelte_body.is_empty()
            && self.titles.is_empty()
    }
}

pub(super) enum BufItem {
    Text(ConcatPart),
    Expr(NodeId),
}

pub(super) enum HoistedKind {
    NotHoisted,
    Snippet,
    ConstTag,
    DebugTag,
    SvelteHead,
    SvelteWindow,
    SvelteDocument,
    SvelteBody,
    TitleInsideHead,
    Comment,
    Error,
}

pub(super) enum ChildAnchor {
    ElementChild { parent_var: String },
    FragmentFirstChild { frag_var: String },
    RawIdent(String),
}
