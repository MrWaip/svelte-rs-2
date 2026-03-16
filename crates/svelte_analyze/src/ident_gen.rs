use rustc_hash::FxHashMap;

/// Shared unique identifier generator. Produces `prefix`, `prefix_1`, `prefix_2`, etc.
pub struct IdentGen {
    counters: FxHashMap<String, u32>,
}

impl IdentGen {
    pub fn new() -> Self {
        Self {
            counters: FxHashMap::default(),
        }
    }

    pub fn gen(&mut self, prefix: &str) -> String {
        let count = if let Some(c) = self.counters.get_mut(prefix) {
            let n = *c;
            *c += 1;
            n
        } else {
            self.counters.insert(prefix.to_string(), 1);
            0
        };
        if count == 0 {
            prefix.to_string()
        } else {
            format!("{}_{}", prefix, count)
        }
    }
}
