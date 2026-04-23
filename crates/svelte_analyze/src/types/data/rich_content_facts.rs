use svelte_ast::FragmentId;

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
    entries: Vec<Option<RichContentFactsEntry>>,
}

impl Default for RichContentFacts {
    fn default() -> Self {
        Self::new()
    }
}

impl RichContentFacts {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn record(&mut self, id: FragmentId, entry: RichContentFactsEntry) {
        let idx = id.0 as usize;
        if self.entries.len() <= idx {
            self.entries.resize(idx + 1, None);
        }
        self.entries[idx] = Some(entry);
    }

    pub fn entry_by_id(&self, id: FragmentId) -> Option<&RichContentFactsEntry> {
        self.entries.get(id.0 as usize)?.as_ref()
    }

    pub fn has_rich_content_by_id(&self, id: FragmentId, parent: RichContentParentKind) -> bool {
        self.entry_by_id(id)
            .is_some_and(|entry| entry.has_rich_content(parent))
    }
}
