use compact_str::{CompactString, format_compact};
use rustc_hash::{FxHashMap, FxHashSet};

pub struct IdentGen {
    counters: FxHashMap<CompactString, u32>,
    conflicts: FxHashSet<CompactString>,
}

impl Default for IdentGen {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IdentGenSnapshot {
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

    pub fn with_conflicts(conflicts: FxHashSet<CompactString>) -> Self {
        Self {
            counters: FxHashMap::default(),
            conflicts,
        }
    }

    pub fn snapshot(&self) -> IdentGenSnapshot {
        IdentGenSnapshot {
            counters: self.counters.clone(),
            conflicts: self.conflicts.clone(),
        }
    }

    pub fn restore(&mut self, snap: IdentGenSnapshot) {
        self.counters = snap.counters;
        self.conflicts = snap.conflicts;
    }

    pub fn generate(&mut self, prefix: &str) -> String {
        let cs = self.generate_compact(prefix);
        cs.into()
    }

    pub fn generate_compact(&mut self, prefix: &str) -> CompactString {
        let count = if let Some(c) = self.counters.get_mut(prefix) {
            let v = *c;
            *c += 1;
            v
        } else {
            self.counters.insert(CompactString::from(prefix), 1);
            0
        };

        let name = if count == 0 {
            CompactString::from(prefix)
        } else {
            format_compact!("{prefix}_{count}")
        };

        if !self.conflicts.contains(&name) {
            self.conflicts.insert(name.clone());
            return name;
        }

        loop {
            let c = self
                .counters
                .get_mut(prefix)
                .expect("counter was inserted for this prefix above");
            let n = *c;
            *c += 1;
            let candidate = format_compact!("{prefix}_{n}");
            if !self.conflicts.contains(&candidate) {
                self.conflicts.insert(candidate.clone());
                return candidate;
            }
        }
    }
}
