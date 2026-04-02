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

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut().filter_map(|slot| slot.as_mut())
    }

    pub fn keys(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_ref().map(|_| NodeId(i as u32)))
    }

    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_ref().map(|v| (NodeId(i as u32), v)))
    }

    /// Drain all populated entries, yielding (NodeId, T) pairs and clearing the table.
    pub fn drain(&mut self) -> impl Iterator<Item = (NodeId, T)> + '_ {
        self.0
            .iter_mut()
            .enumerate()
            .filter_map(|(i, slot)| slot.take().map(|v| (NodeId(i as u32), v)))
    }
}

/// Dense bitset indexed by `NodeId`. Uses u64-packed words for 8x less memory
/// and faster iteration than `Vec<bool>`.
#[derive(Debug, Clone)]
pub struct NodeBitSet {
    words: Vec<u64>,
    len: usize,
}

impl NodeBitSet {
    pub fn new(node_count: u32) -> Self {
        let len = node_count as usize;
        let word_count = (len + 63) / 64;
        Self {
            words: vec![0u64; word_count],
            len,
        }
    }

    #[inline]
    pub fn contains(&self, id: &NodeId) -> bool {
        let idx = id.0 as usize;
        let word = idx / 64;
        let bit = idx % 64;
        self.words.get(word).is_some_and(|w| w & (1u64 << bit) != 0)
    }

    pub fn insert(&mut self, id: NodeId) {
        let idx = id.0 as usize;
        let word = idx / 64;
        let bit = idx % 64;
        if word >= self.words.len() {
            self.words.resize(word + 1, 0);
        }
        self.words[word] |= 1u64 << bit;
        if idx >= self.len {
            self.len = idx + 1;
        }
    }

    pub fn remove(&mut self, id: &NodeId) -> bool {
        let idx = id.0 as usize;
        let word = idx / 64;
        let bit = idx % 64;
        if let Some(w) = self.words.get_mut(word) {
            let mask = 1u64 << bit;
            let was = *w & mask != 0;
            *w &= !mask;
            was
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    pub fn iter(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.words.iter().enumerate().flat_map(|(word_idx, &word)| {
            let base = (word_idx * 64) as u32;
            BitIter { word, base }
        })
    }
}

/// Iterator over set bits in a single u64 word.
struct BitIter {
    word: u64,
    base: u32,
}

impl Iterator for BitIter {
    type Item = NodeId;

    #[inline]
    fn next(&mut self) -> Option<NodeId> {
        if self.word == 0 {
            return None;
        }
        let bit = self.word.trailing_zeros();
        self.word &= self.word - 1; // clear lowest set bit
        Some(NodeId(self.base + bit))
    }
}
