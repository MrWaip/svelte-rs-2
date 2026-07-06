use smallvec::SmallVec;
use svelte_ast::NodeId;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ElementSemantics {
    #[default]
    None,

    RegularElement(RegularElementSemantics),

    Boundary(BoundarySemantics),

    SvelteElement(SvelteElementSemantics),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegularElementSemantics {
    pub async_kind: ElementAsyncKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SvelteElementSemantics {
    pub async_kind: ElementAsyncKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElementAsyncKind {
    Awaited { blockers: SmallVec<[u32; 2]> },

    Deferred { blockers: SmallVec<[u32; 2]> },
}

impl ElementAsyncKind {
    pub fn blockers(&self) -> &[u32] {
        match self {
            ElementAsyncKind::Awaited { blockers } | ElementAsyncKind::Deferred { blockers } => {
                blockers
            }
        }
    }

    pub fn awaited(&self) -> bool {
        matches!(self, ElementAsyncKind::Awaited { .. })
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
