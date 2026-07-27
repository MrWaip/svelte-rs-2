use compact_str::CompactString;
use smallvec::SmallVec;
use svelte_ast::{NodeId, OxcNodeId};

use crate::types::data::ContentEditableKind;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ElementSemantics {
    #[default]
    None,

    RegularElement(RegularElementSemantics),

    HeadTitle,

    Boundary(BoundarySemantics),

    SvelteElement(SvelteElementSemantics),

    LegacySlot(LegacySlotSemantics),

    Component(ComponentElementSemantics),
}

impl ElementSemantics {
    pub fn property_reset(&self) -> ElementPropertyReset {
        match self {
            ElementSemantics::RegularElement(sem) => sem.property_reset,
            ElementSemantics::None
            | ElementSemantics::HeadTitle
            | ElementSemantics::Boundary(_)
            | ElementSemantics::SvelteElement(_)
            | ElementSemantics::LegacySlot(_)
            | ElementSemantics::Component(_) => ElementPropertyReset::None,
        }
    }

    pub fn is_script(&self) -> bool {
        match self {
            ElementSemantics::RegularElement(sem) => sem.is_script,
            ElementSemantics::None
            | ElementSemantics::HeadTitle
            | ElementSemantics::Boundary(_)
            | ElementSemantics::SvelteElement(_)
            | ElementSemantics::LegacySlot(_)
            | ElementSemantics::Component(_) => false,
        }
    }

    pub fn async_kind(&self) -> &ElementAsyncKind {
        match self {
            ElementSemantics::RegularElement(sem) => &sem.async_kind,
            ElementSemantics::SvelteElement(sem) => &sem.tag_async_kind,
            ElementSemantics::Component(sem) => &sem.async_kind,
            ElementSemantics::LegacySlot(sem) => &sem.async_kind,
            ElementSemantics::None
            | ElementSemantics::HeadTitle
            | ElementSemantics::Boundary(_) => &ElementAsyncKind::Sync,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegacySlotSemantics {
    pub name: String,
    pub has_fallback: bool,
    pub async_kind: ElementAsyncKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentElementSemantics {
    pub async_kind: ElementAsyncKind,
    pub legacy_slots: LegacyComponentSlotsSemantics,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegacyComponentSlotsSemantics {
    pub default_slot: LegacyDefaultSlot,
    pub default_wrapper: Option<NodeId>,
    pub default_let_owner: Option<NodeId>,
    pub default_let_scope_owners: SmallVec<[NodeId; 2]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegacyDefaultSlot {
    #[default]
    ChildrenProp,
    SlotDefaultInvalid,
    SlotDefault,
    OwnLetDisplaced,
    SlotDefaultSlottedLet,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegularElementSemantics {
    pub name: CompactString,
    pub async_kind: ElementAsyncKind,
    pub value_role: ElementValueRole,
    pub replay_events: SmallVec<[ElementReplayEvent; 2]>,
    pub opaque_content: bool,
    pub property_reset: ElementPropertyReset,
    pub is_script: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementPropertyReset {
    None,
    Dir,
    LazyLoadingImg,
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
    Single { node_id: NodeId, oxc_id: OxcNodeId },
    Static(String),
    Segments(Vec<TextareaSegment>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextareaSegment {
    Text(String),
    Expression { node_id: NodeId, oxc_id: OxcNodeId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SvelteElementSemantics {
    pub tag_async_kind: ElementAsyncKind,
    pub attributes_async_kind: ElementAsyncKind,
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
    pub pending_needs_nullish_guard: bool,
    pub failed_snippet: Option<NodeId>,
    pub pending_snippet: Option<NodeId>,
}

impl BoundarySemantics {
    pub fn is_prop_snippet(&self, id: NodeId) -> bool {
        self.failed_snippet == Some(id) || self.pending_snippet == Some(id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryBranch {
    None,
    Snippet(NodeId),
    Attribute(NodeId),
}

#[derive(Debug, Default, Clone)]
pub struct ElementSemanticsStore {
    entries: rustc_hash::FxHashMap<u32, ElementSemantics>,
}

impl ElementSemanticsStore {
    pub(crate) fn new(node_count: u32) -> Self {
        let cap = node_count as usize / 4;
        Self {
            entries: rustc_hash::FxHashMap::with_capacity_and_hasher(cap, Default::default()),
        }
    }

    pub fn query(&self, id: NodeId) -> &ElementSemantics {
        self.entries.get(&id.0).unwrap_or(&ElementSemantics::None)
    }

    pub(crate) fn set(&mut self, id: NodeId, value: ElementSemantics) {
        self.entries.insert(id.0, value);
    }
}
