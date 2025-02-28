use oxc_semantic::{ScopeTree, SymbolTable};

pub struct HirAnalyses {
    scope: ScopeTree,
    symbols: SymbolTable,
}

impl HirAnalyses {
    pub fn new(scope: ScopeTree, symbols: SymbolTable) -> Self {
        Self { scope, symbols }
    }
}
