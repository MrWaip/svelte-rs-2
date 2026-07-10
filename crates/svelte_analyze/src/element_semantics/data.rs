use smallvec::SmallVec;
use svelte_ast::{NodeId, OxcNodeId};

use crate::types::data::ContentEditableKind;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ElementSemantics {
    #[default]
    None,

    RegularElement(RegularElementSemantics),

    Boundary(BoundarySemantics),

    SvelteElement(SvelteElementSemantics),

    LegacySlot(LegacySlotSemantics),

    LegacyComponentSlots(LegacyComponentSlotsSemantics),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegacySlotSemantics {
    pub name: String,
    pub has_fallback: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegacyComponentSlotsSemantics {
    pub default_slot: LegacyDefaultSlot,
    pub default_wrapper: Option<NodeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegacyDefaultSlot {
    #[default]
    ChildrenProp,
    SlotDefaultInvalid,
    SlotDefault,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegularElementSemantics {
    pub async_kind: ElementAsyncKind,
    pub value_role: ElementValueRole,
    pub replay_events: SmallVec<[ElementReplayEvent; 2]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementReplayEvent {
    Load,
    Error,
}

impl ElementReplayEvent {
    pub fn attribute_name(self) -> &'static str {
        match self {
            ElementReplayEvent::Load => "onload",
            ElementReplayEvent::Error => "onerror",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ElementValueRole {
    #[default]
    Plain,
    Select {
        rich: bool,
    },
    Option {
        value: Option<NodeId>,
        rich: bool,
    },
    TextareaValue {
        body: TextareaBody,
    },
    ContentEditable {
        bind_id: NodeId,
        kind: ContentEditableKind,
    },
    RichContainer,
    RawText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextareaBody {
    Single(OxcNodeId),
    Segments(Vec<TextareaSegment>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextareaSegment {
    Text(String),
    Expression { node_id: NodeId, oxc_id: OxcNodeId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SvelteElementSemantics {
    pub async_kind: ElementAsyncKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElementAsyncKind {
    Sync,

    Awaited { blockers: SmallVec<[u32; 2]> },

    Deferred { blockers: SmallVec<[u32; 2]> },
}

impl ElementAsyncKind {
    pub fn blockers(&self) -> &[u32] {
        match self {
            ElementAsyncKind::Sync => &[],
            ElementAsyncKind::Awaited { blockers } | ElementAsyncKind::Deferred { blockers } => {
                blockers
            }
        }
    }

    pub fn awaited(&self) -> bool {
        matches!(self, ElementAsyncKind::Awaited { .. })
    }

    pub fn is_sync(&self) -> bool {
        matches!(self, ElementAsyncKind::Sync)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundarySemantics {
    pub failed: BoundaryBranch,
    pub pending: BoundaryBranch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryBranch {
    None,
    Snippet(NodeId),
    Attribute(NodeId),
}

#[derive(Debug, Default, Clone)]
pub struct ElementSemanticsStore {
    entries: Vec<ElementSemantics>,
}

impl ElementSemanticsStore {
    pub(crate) fn new(node_count: u32) -> Self {
        let mut entries = Vec::with_capacity(node_count as usize);
        entries.resize_with(node_count as usize, ElementSemantics::default);
        Self { entries }
    }

    pub fn query(&self, id: NodeId) -> &ElementSemantics {
        self.entries
            .get(id.0 as usize)
            .unwrap_or(&ElementSemantics::None)
    }

    pub(crate) fn set(&mut self, id: NodeId, value: ElementSemantics) {
        let idx = id.0 as usize;
        if idx >= self.entries.len() {
            self.entries.resize_with(idx + 1, ElementSemantics::default);
        }
        self.entries[idx] = value;
    }
}
