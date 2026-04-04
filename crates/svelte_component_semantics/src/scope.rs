use compact_str::CompactString;
use oxc_syntax::scope::{ScopeFlags, ScopeId};
use oxc_syntax::symbol::SymbolId;
use rustc_hash::FxHashMap;

/// Struct-of-arrays storage for scopes in a component.
pub(crate) struct ScopeTable {
    parent_ids: Vec<Option<ScopeId>>,
    flags: Vec<ScopeFlags>,
    bindings: Vec<FxHashMap<CompactString, SymbolId>>,
}

impl ScopeTable {
    pub fn new() -> Self {
        Self {
            parent_ids: Vec::new(),
            flags: Vec::new(),
            bindings: Vec::new(),
        }
    }

    pub fn add_scope(&mut self, parent: Option<ScopeId>, flags: ScopeFlags) -> ScopeId {
        let id = ScopeId::from_usize(self.parent_ids.len());
        self.parent_ids.push(parent);
        self.flags.push(flags);
        self.bindings.push(FxHashMap::default());
        id
    }

    pub fn scope_parent_id(&self, id: ScopeId) -> Option<ScopeId> {
        self.parent_ids[id.index()]
    }

    pub fn set_scope_parent_id(&mut self, id: ScopeId, parent: Option<ScopeId>) {
        self.parent_ids[id.index()] = parent;
    }

    pub fn scope_flags(&self, id: ScopeId) -> ScopeFlags {
        self.flags[id.index()]
    }

    /// Look up a binding in this scope only (no parent walk).
    pub fn get_binding(&self, scope: ScopeId, name: &str) -> Option<SymbolId> {
        self.bindings[scope.index()].get(name).copied()
    }

    /// Walk the parent chain to find a binding.
    pub fn find_binding(&self, mut scope: ScopeId, name: &str) -> Option<SymbolId> {
        loop {
            if let Some(sym) = self.bindings[scope.index()].get(name) {
                return Some(*sym);
            }
            scope = self.parent_ids[scope.index()]?;
        }
    }

    pub fn add_binding(&mut self, scope: ScopeId, name: CompactString, symbol: SymbolId) {
        self.bindings[scope.index()].insert(name, symbol);
    }

    /// Find the nearest ancestor scope (inclusive) with the Function flag.
    pub fn find_function_scope(&self, mut scope: ScopeId) -> ScopeId {
        loop {
            if self.flags[scope.index()].is_function() {
                return scope;
            }
            match self.parent_ids[scope.index()] {
                Some(parent) => scope = parent,
                None => return scope,
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn sym(n: usize) -> SymbolId {
        SymbolId::from_usize(n)
    }

    #[test]
    fn add_scope_and_parent() {
        let mut t = ScopeTable::new();
        let root = t.add_scope(None, ScopeFlags::Top);
        let child = t.add_scope(Some(root), ScopeFlags::empty());

        assert_eq!(t.scope_parent_id(root), None);
        assert_eq!(t.scope_parent_id(child), Some(root));
        assert_eq!(t.scope_flags(root), ScopeFlags::Top);
    }

    #[test]
    fn find_binding_walks_parents() {
        let mut t = ScopeTable::new();
        let root = t.add_scope(None, ScopeFlags::Top);
        let child = t.add_scope(Some(root), ScopeFlags::empty());

        t.add_binding(root, "x".into(), sym(0));

        assert_eq!(t.find_binding(child, "x"), Some(sym(0)));
        assert_eq!(t.find_binding(root, "x"), Some(sym(0)));
        assert_eq!(t.find_binding(child, "y"), None);
    }

    #[test]
    fn get_binding_no_walk() {
        let mut t = ScopeTable::new();
        let root = t.add_scope(None, ScopeFlags::Top);
        let child = t.add_scope(Some(root), ScopeFlags::empty());

        t.add_binding(root, "x".into(), sym(0));

        assert_eq!(t.get_binding(root, "x"), Some(sym(0)));
        assert_eq!(t.get_binding(child, "x"), None);
    }

    #[test]
    fn shadowing() {
        let mut t = ScopeTable::new();
        let root = t.add_scope(None, ScopeFlags::Top);
        let child = t.add_scope(Some(root), ScopeFlags::empty());

        t.add_binding(root, "x".into(), sym(0));
        t.add_binding(child, "x".into(), sym(1));

        assert_eq!(t.find_binding(child, "x"), Some(sym(1)));
        assert_eq!(t.find_binding(root, "x"), Some(sym(0)));
    }

    #[test]
    fn find_function_scope() {
        let mut t = ScopeTable::new();
        let root = t.add_scope(None, ScopeFlags::Top | ScopeFlags::Function);
        let block = t.add_scope(Some(root), ScopeFlags::empty());
        let inner_fn = t.add_scope(Some(block), ScopeFlags::Function);
        let inner_block = t.add_scope(Some(inner_fn), ScopeFlags::empty());

        assert_eq!(t.find_function_scope(inner_block), inner_fn);
        assert_eq!(t.find_function_scope(inner_fn), inner_fn);
        assert_eq!(t.find_function_scope(block), root);
    }

    #[test]
    fn set_scope_parent_id() {
        let mut t = ScopeTable::new();
        let a = t.add_scope(None, ScopeFlags::Top);
        let b = t.add_scope(None, ScopeFlags::Top);
        let c = t.add_scope(Some(a), ScopeFlags::empty());

        t.set_scope_parent_id(c, Some(b));
        assert_eq!(t.scope_parent_id(c), Some(b));
    }
}
