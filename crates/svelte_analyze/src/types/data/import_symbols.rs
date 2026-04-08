use rustc_hash::FxHashSet;

use crate::scope::SymbolId;

#[derive(Default)]
pub struct ImportSymbolSet {
    syms: FxHashSet<SymbolId>,
}

impl ImportSymbolSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn extend(&mut self, syms: impl IntoIterator<Item = SymbolId>) {
        self.syms.extend(syms);
    }

    pub fn contains(&self, sym: SymbolId) -> bool {
        self.syms.contains(&sym)
    }

    pub fn iter(&self) -> impl Iterator<Item = SymbolId> + '_ {
        self.syms.iter().copied()
    }
}
