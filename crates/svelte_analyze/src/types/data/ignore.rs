use super::*;

#[derive(Debug, Default)]
pub struct IgnoreData {
    node_snapshot: FxHashMap<NodeId, u32>,
    snapshots: Vec<FxHashSet<String>>,
    intern: FxHashMap<Vec<String>, u32>,
}

impl IgnoreData {
    pub fn new() -> Self {
        let empty_set = FxHashSet::default();
        let mut intern = FxHashMap::default();
        intern.insert(Vec::new(), 0);
        Self {
            node_snapshot: FxHashMap::default(),
            snapshots: vec![empty_set],
            intern,
        }
    }

    pub fn is_ignored(&self, node_id: NodeId, code: &str) -> bool {
        self.node_snapshot
            .get(&node_id)
            .and_then(|&idx| self.snapshots.get(idx as usize))
            .is_some_and(|set| set.contains(code))
    }

    pub(crate) fn intern_snapshot(&mut self, codes: &FxHashSet<String>) -> u32 {
        let mut sorted: Vec<String> = codes.iter().cloned().collect();
        sorted.sort();
        if let Some(&idx) = self.intern.get(&sorted) {
            return idx;
        }
        let idx = self.snapshots.len() as u32;
        self.snapshots.push(codes.clone());
        self.intern.insert(sorted, idx);
        idx
    }

    pub(crate) fn set_snapshot(&mut self, node_id: NodeId, idx: u32) {
        if idx != 0 {
            self.node_snapshot.insert(node_id, idx);
        }
    }
}
