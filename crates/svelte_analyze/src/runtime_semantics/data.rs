#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChildPropMode {
    #[default]
    In,
    InOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RuntimeSemantics {
    child_prop_mode: ChildPropMode,
}

impl RuntimeSemantics {
    pub(crate) fn new(child_prop_mode: ChildPropMode) -> Self {
        Self { child_prop_mode }
    }

    pub fn child_prop_mode(self) -> ChildPropMode {
        self.child_prop_mode
    }
}

#[derive(Default)]
pub struct RuntimeSemanticsStore {
    semantics: RuntimeSemantics,
}

impl RuntimeSemanticsStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn record(&mut self, semantics: RuntimeSemantics) {
        self.semantics = semantics;
    }

    pub fn query(&self) -> RuntimeSemantics {
        self.semantics
    }
}
