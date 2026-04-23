use svelte_ast::{FragmentId, Namespace};

#[derive(Default)]
pub struct FragmentNamespaces {
    by_id: Vec<Option<Namespace>>,
}

impl FragmentNamespaces {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn record(&mut self, id: FragmentId, ns: Namespace) {
        let idx = id.0 as usize;
        if self.by_id.len() <= idx {
            self.by_id.resize(idx + 1, None);
        }
        self.by_id[idx] = Some(ns);
    }

    pub fn get(&self, id: FragmentId) -> Option<Namespace> {
        self.by_id.get(id.0 as usize).copied().flatten()
    }
}
