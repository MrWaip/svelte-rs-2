use svelte_ast::FragmentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FragmentWhitespace {
    Preserve,
    Collapse,
    Remove,
}

impl FragmentWhitespace {
    pub fn is_preserved(self) -> bool {
        matches!(self, FragmentWhitespace::Preserve)
    }

    pub fn is_removable(self) -> bool {
        matches!(self, FragmentWhitespace::Remove)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentSemantics {
    pub whitespace: FragmentWhitespace,
}

impl FragmentSemantics {
    const DEFAULT: FragmentSemantics = FragmentSemantics {
        whitespace: FragmentWhitespace::Collapse,
    };
}

#[derive(Default)]
pub struct FragmentSemanticsStore {
    by_id: Vec<Option<FragmentSemantics>>,
}

impl FragmentSemanticsStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn record(&mut self, id: FragmentId, semantics: FragmentSemantics) {
        let idx = id.0 as usize;
        if self.by_id.len() <= idx {
            self.by_id.resize(idx + 1, None);
        }
        self.by_id[idx] = Some(semantics);
    }

    pub fn query(&self, id: FragmentId) -> FragmentSemantics {
        self.by_id
            .get(id.0 as usize)
            .copied()
            .flatten()
            .unwrap_or(FragmentSemantics::DEFAULT)
    }
}
