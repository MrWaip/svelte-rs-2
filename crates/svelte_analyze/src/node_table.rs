use svelte_ast::NodeId;

/// Dense lookup table indexed by `NodeId`. Replaces `FxHashMap<NodeId, T>`
/// for tables where keys are sequential node IDs from the parser.
#[derive(Debug, Clone)]
pub struct NodeTable<T>(Vec<Option<T>>);

impl<T> NodeTable<T> {
    pub fn new(node_count: u32) -> Self {
        let mut v = Vec::with_capacity(node_count as usize);
        v.resize_with(node_count as usize, || None);
        Self(v)
    }

    #[inline]
    pub fn get(&self, id: NodeId) -> Option<&T> {
        self.0.get(id.0 as usize).and_then(|slot| slot.as_ref())
    }

    #[inline]
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        self.0.get_mut(id.0 as usize).and_then(|slot| slot.as_mut())
    }

    pub fn insert(&mut self, id: NodeId, value: T) {
        let idx = id.0 as usize;
        if idx >= self.0.len() {
            self.0.resize_with(idx + 1, || None);
        }
        self.0[idx] = Some(value);
    }

    pub fn remove(&mut self, id: NodeId) -> Option<T> {
        let idx = id.0 as usize;
        self.0.get_mut(idx).and_then(|slot| slot.take())
    }

    #[inline]
    pub fn contains_key(&self, id: NodeId) -> bool {
        self.get(id).is_some()
    }

    /// Get-or-insert-default, returns `&mut T`. Replaces `.entry(id).or_default()`.
    pub fn get_or_default(&mut self, id: NodeId) -> &mut T
    where
        T: Default,
    {
        let idx = id.0 as usize;
        if idx >= self.0.len() {
            self.0.resize_with(idx + 1, || None);
        }
        self.0[idx].get_or_insert_with(T::default)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter().filter_map(|slot| slot.as_ref())
    }

    pub fn keys(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.0.iter().enumerate().filter_map(|(i, slot)| {
            slot.as_ref().map(|_| NodeId(i as u32))
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> {
        self.0.iter().enumerate().filter_map(|(i, slot)| {
            slot.as_ref().map(|v| (NodeId(i as u32), v))
        })
    }

    /// Drain all populated entries, yielding (NodeId, T) pairs and clearing the table.
    pub fn drain(&mut self) -> impl Iterator<Item = (NodeId, T)> + '_ {
        self.0.iter_mut().enumerate().filter_map(|(i, slot)| {
            slot.take().map(|v| (NodeId(i as u32), v))
        })
    }
}

/// Dense bitset indexed by `NodeId`. Replaces `FxHashSet<NodeId>`.
#[derive(Debug, Clone)]
pub struct NodeBitSet(Vec<bool>);

impl NodeBitSet {
    pub fn new(node_count: u32) -> Self {
        Self(vec![false; node_count as usize])
    }

    #[inline]
    pub fn contains(&self, id: &NodeId) -> bool {
        self.0.get(id.0 as usize).copied().unwrap_or(false)
    }

    pub fn insert(&mut self, id: NodeId) {
        let idx = id.0 as usize;
        if idx >= self.0.len() {
            self.0.resize(idx + 1, false);
        }
        self.0[idx] = true;
    }

    pub fn remove(&mut self, id: &NodeId) -> bool {
        let idx = id.0 as usize;
        if let Some(slot) = self.0.get_mut(idx) {
            let was = *slot;
            *slot = false;
            was
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.0.iter().any(|&b| b)
    }

    pub fn iter(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.0.iter().enumerate().filter_map(|(i, &b)| {
            if b { Some(NodeId(i as u32)) } else { None }
        })
    }
}
