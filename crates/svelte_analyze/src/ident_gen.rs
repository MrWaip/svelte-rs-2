use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};

/// Shared unique identifier generator. Produces `prefix`, `prefix_1`, `prefix_2`, etc.
/// Checks generated names against a `conflicts` set (script-level identifiers).
///
/// Uses CompactString to avoid heap allocations for short identifiers (<=24 bytes
/// are stored inline on the stack).
pub struct IdentGen {
    counters: FxHashMap<CompactString, u32>,
    conflicts: FxHashSet<CompactString>,
}

impl IdentGen {
    pub fn new() -> Self {
        Self {
            counters: FxHashMap::default(),
            conflicts: FxHashSet::default(),
        }
    }

    /// Create with a pre-populated set of names to avoid (e.g. all script-level identifiers).
    pub fn with_conflicts(conflicts: FxHashSet<String>) -> Self {
        Self {
            counters: FxHashMap::default(),
            conflicts: conflicts.into_iter().map(CompactString::from).collect(),
        }
    }

    pub fn gen(&mut self, prefix: &str) -> String {
        let key = CompactString::from(prefix);
        let count = self.counters.entry(key).or_insert(0);

        let mut name = if *count == 0 {
            CompactString::from(prefix)
        } else {
            CompactString::from(format!("{}_{}", prefix, count))
        };
        *count += 1;

        while self.conflicts.contains(name.as_str()) {
            let c = self.counters.get_mut(prefix).unwrap();
            name = CompactString::from(format!("{}_{}", prefix, c));
            *c += 1;
        }

        self.conflicts.insert(name.clone());
        name.into()
    }
}
