use rustc_hash::FxHashMap;
use svelte_ast::OxcNodeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AwaitSemantics {
    #[default]
    Detached,

    TerminalInFragmentInterpolation,

    TerminalInConstruct,

    NonTerminal,
}

#[derive(Default)]
pub struct AwaitSemanticsStore {
    entries: FxHashMap<OxcNodeId, AwaitSemantics>,
}

impl AwaitSemanticsStore {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn query(&self, id: OxcNodeId) -> AwaitSemantics {
        self.entries.get(&id).copied().unwrap_or_default()
    }

    pub(crate) fn set(&mut self, id: OxcNodeId, value: AwaitSemantics) {
        self.entries.insert(id, value);
    }
}
