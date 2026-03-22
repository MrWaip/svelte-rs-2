use rustc_hash::{FxHashMap, FxHashSet};

/// Shared unique identifier generator. Produces `prefix`, `prefix_1`, `prefix_2`, etc.
/// Checks generated names against a `conflicts` set (script-level identifiers)
/// to avoid shadowing user-declared variables.
pub struct IdentGen {
    counters: FxHashMap<String, u32>,
    conflicts: FxHashSet<String>,
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
            conflicts,
        }
    }

    pub fn gen(&mut self, prefix: &str) -> String {
        let count = self.counters.entry(prefix.to_string()).or_insert(0);
        let mut name = if *count == 0 {
            prefix.to_string()
        } else {
            format!("{}_{}", prefix, count)
        };
        *count += 1;

        while self.conflicts.contains(&name) {
            name = format!("{}_{}", prefix, *self.counters.get(prefix).unwrap());
            *self.counters.get_mut(prefix).unwrap() += 1;
        }

        self.conflicts.insert(name.clone());
        name
    }
}
