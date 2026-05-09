use rustc_hash::FxHashSet;
use svelte_component_semantics::OxcNodeId;

#[derive(Default)]
pub struct PickledAwaits {
    node_ids: FxHashSet<OxcNodeId>,
}

impl PickledAwaits {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn extend_node_ids(&mut self, ids: impl IntoIterator<Item = OxcNodeId>) {
        self.node_ids.extend(ids);
    }

    pub fn contains(&self, id: OxcNodeId) -> bool {
        self.node_ids.contains(&id)
    }
}
