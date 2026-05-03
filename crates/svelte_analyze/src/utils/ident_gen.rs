use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};

fn push_u32(out: &mut String, mut n: u32) {
    if n == 0 {
        out.push('0');
        return;
    }
    let mut buf = [0u8; 10];
    let mut i = buf.len();
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    out.push_str(unsafe { std::str::from_utf8_unchecked(&buf[i..]) });
}

fn build_name_into(out: &mut String, prefix: &str, n: u32) {
    out.push_str(prefix);
    out.push('_');
    push_u32(out, n);
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
        let count = if let Some(c) = self.counters.get_mut(prefix) {
            let v = *c;
            *c += 1;
            v
        } else {
            self.counters.insert(CompactString::from(prefix), 1);
            0
        };

        let mut name = String::with_capacity(prefix.len() + 6);
        if count == 0 {
            name.push_str(prefix);
        } else {
            build_name_into(&mut name, prefix, count);
        }

        while self.conflicts.contains(name.as_str()) {
            let c = self
                .counters
                .get_mut(prefix)
                .expect("counter was inserted for this prefix above");
            let n = *c;
            *c += 1;
            name.clear();
            build_name_into(&mut name, prefix, n);
        }

        self.conflicts.insert(CompactString::from(name.as_str()));
        name
    }
}
