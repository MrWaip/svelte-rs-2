use compact_str::CompactString;
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct ProxyStateInits {
    by_name: FxHashMap<CompactString, bool>,
}

impl ProxyStateInits {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn set_proxied(&mut self, name: impl Into<CompactString>, is_proxied: bool) {
        self.by_name.insert(name.into(), is_proxied);
    }

    pub fn is_proxied(&self, name: &str) -> bool {
        self.by_name.get(name).copied().unwrap_or(false)
    }
}
