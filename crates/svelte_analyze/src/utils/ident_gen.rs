use std::fmt::Write;

use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};

fn build_name(prefix: &str, n: u32) -> CompactString {
    let mut buf = CompactString::with_capacity(prefix.len() + 4);
    buf.push_str(prefix);
    buf.push('_');
    let _ = write!(buf, "{}", n);
    buf
}

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
        let key = CompactString::from(prefix);
        let count = self.counters.entry(key).or_insert(0);

        let mut name = if *count == 0 {
            CompactString::from(prefix)
        } else {
            build_name(prefix, *count)
        };
        *count += 1;

        while self.conflicts.contains(name.as_str()) {
            let c = self
                .counters
                .get_mut(prefix)
                .expect("counter was inserted for this prefix above");
            name = build_name(prefix, *c);
            *c += 1;
        }

        self.conflicts.insert(name.clone());
        name.into()
    }
}
