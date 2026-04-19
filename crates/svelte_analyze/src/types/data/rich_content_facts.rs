use rustc_hash::FxHashMap;

use super::FragmentKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichContentParentKind {
    Select,
    Optgroup,
    Option,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RichContentFactsEntry {
    select: bool,
    optgroup: bool,
    option: bool,
}

impl RichContentFactsEntry {
    pub fn new(select: bool, optgroup: bool, option: bool) -> Self {
        Self {
            select,
            optgroup,
            option,
        }
    }

    pub fn has_rich_content(&self, parent: RichContentParentKind) -> bool {
        match parent {
            RichContentParentKind::Select => self.select,
            RichContentParentKind::Optgroup => self.optgroup,
            RichContentParentKind::Option => self.option,
        }
    }
}

pub struct RichContentFacts {
    entries: FxHashMap<FragmentKey, RichContentFactsEntry>,
}

impl Default for RichContentFacts {
    fn default() -> Self {
        Self::new()
    }
}

impl RichContentFacts {
    pub fn new() -> Self {
        Self {
            entries: FxHashMap::default(),
        }
    }

    pub(crate) fn record(&mut self, key: FragmentKey, entry: RichContentFactsEntry) {
        self.entries.insert(key, entry);
    }

    pub fn entry(&self, key: &FragmentKey) -> Option<&RichContentFactsEntry> {
        self.entries.get(key)
    }

    pub fn has_rich_content(&self, key: &FragmentKey, parent: RichContentParentKind) -> bool {
        self.entries
            .get(key)
            .is_some_and(|entry| entry.has_rich_content(parent))
    }
}
