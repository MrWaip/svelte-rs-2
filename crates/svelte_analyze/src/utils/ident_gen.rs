use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};
use std::str::from_utf8_unchecked;

fn format_name(buf: &mut [u8; 32], prefix: &str, n: u32) -> usize {
    let plen = prefix.len().min(24);
    buf[..plen].copy_from_slice(&prefix.as_bytes()[..plen]);
    buf[plen] = b'_';
    let mut pos = plen + 1;
    if n == 0 {
        buf[pos] = b'0';
        pos += 1;
    } else {
        let mut digits = [0u8; 10];
        let mut i = digits.len();
        let mut v = n;
        while v > 0 {
            i -= 1;
            digits[i] = b'0' + (v % 10) as u8;
            v /= 10;
        }
        let len = digits.len() - i;
        buf[pos..pos + len].copy_from_slice(&digits[i..]);
        pos += len;
    }
    pos
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
            let mut buf = [0u8; 32];
            let len = format_name(&mut buf, prefix, count);
            CompactString::from(unsafe { from_utf8_unchecked(&buf[..len]) })
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
            let mut buf = [0u8; 32];
            let len = format_name(&mut buf, prefix, n);
            let candidate = CompactString::from(unsafe { from_utf8_unchecked(&buf[..len]) });
            if !self.conflicts.contains(&candidate) {
                self.conflicts.insert(candidate.clone());
                return candidate;
            }
        }
    }
}
