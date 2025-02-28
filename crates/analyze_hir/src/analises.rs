use oxc_semantic::{ScopeTree, SymbolTable};

pub struct HirAnalises {
    scope: ScopeTree,
    symbols: SymbolTable,
}

impl HirAnalises {
    pub fn new(scope: ScopeTree, symbols: SymbolTable) -> Self {
        Self { scope, symbols }
    }
}
