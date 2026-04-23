use svelte_ast::{FragmentId, FragmentRole, NodeId};

pub struct FragmentLayout {
    pub items: Vec<NodeId>,
    pub owner: Option<NodeId>,
    pub role: FragmentRole,
}

#[derive(Default)]
pub struct FragmentLayouts {
    entries: Vec<Option<FragmentLayout>>,
}

impl FragmentLayouts {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, id: FragmentId, layout: FragmentLayout) {
        let idx = id.0 as usize;
        if self.entries.len() <= idx {
            self.entries.resize_with(idx + 1, || None);
        }
        self.entries[idx] = Some(layout);
    }

    pub fn get(&self, id: FragmentId) -> Option<&FragmentLayout> {
        self.entries.get(id.0 as usize)?.as_ref()
    }

    pub fn items(&self, id: FragmentId) -> Option<&[NodeId]> {
        self.get(id).map(|l| l.items.as_slice())
    }
}
