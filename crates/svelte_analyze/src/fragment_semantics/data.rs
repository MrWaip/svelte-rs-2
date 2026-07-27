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
pub enum FragmentBindings {
    None,
    Local,
}

impl FragmentBindings {
    pub fn declares_local(self) -> bool {
        matches!(self, FragmentBindings::Local)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FragmentContent {
    Markup,
    SelfContained,
}

impl FragmentContent {
    pub fn is_self_contained(self) -> bool {
        matches!(self, FragmentContent::SelfContained)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FragmentScript {
    Plain,
    ContainsScript,
}

impl FragmentScript {
    pub fn has_script(self) -> bool {
        matches!(self, FragmentScript::ContainsScript)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentSemantics {
    pub whitespace: FragmentWhitespace,
    pub bindings: FragmentBindings,
    pub script: FragmentScript,
    pub content: FragmentContent,
}

impl FragmentSemantics {
    const DEFAULT: FragmentSemantics = FragmentSemantics {
        whitespace: FragmentWhitespace::Collapse,
        bindings: FragmentBindings::None,
        script: FragmentScript::Plain,
        content: FragmentContent::Markup,
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
