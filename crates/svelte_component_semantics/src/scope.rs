use compact_str::CompactString;
use oxc_syntax::scope::{ScopeFlags, ScopeId};
use oxc_syntax::symbol::SymbolId;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

const INLINE_BINDINGS: usize = 8;

#[allow(clippy::large_enum_variant)]
enum ScopeBindings {
    Small(SmallVec<[(CompactString, SymbolId); INLINE_BINDINGS]>),
    Large(Box<FxHashMap<CompactString, SymbolId>>),
}

impl ScopeBindings {
    fn new() -> Self {
        Self::Small(SmallVec::new())
    }

    fn get(&self, name: &str) -> Option<SymbolId> {
        match self {
            Self::Small(v) => v
                .iter()
                .find(|(k, _)| k.as_str() == name)
                .map(|(_, s)| *s),
            Self::Large(m) => m.get(name).copied(),
        }
    }

    fn insert(&mut self, name: CompactString, symbol: SymbolId) {
        match self {
            Self::Small(v) => {
                if let Some(slot) = v.iter_mut().find(|(k, _)| *k == name) {
                    slot.1 = symbol;
                    return;
                }
                if v.len() >= INLINE_BINDINGS {
                    let mut map: FxHashMap<CompactString, SymbolId> =
                        FxHashMap::with_capacity_and_hasher(
                            INLINE_BINDINGS * 2,
                            Default::default(),
                        );
                    for (k, s) in v.drain(..) {
                        map.insert(k, s);
                    }
                    map.insert(name, symbol);
                    *self = Self::Large(Box::new(map));
                } else {
                    v.push((name, symbol));
                }
            }
            Self::Large(m) => {
                m.insert(name, symbol);
            }
        }
    }

    fn names(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        match self {
            Self::Small(v) => Box::new(v.iter().map(|(k, _)| k.as_str())),
            Self::Large(m) => Box::new(m.keys().map(|k| k.as_str())),
        }
    }
}

pub(crate) struct ScopeTable {
    parent_ids: Vec<Option<ScopeId>>,
    flags: Vec<ScopeFlags>,
    bindings: Vec<ScopeBindings>,
}

impl ScopeTable {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parent_ids: Vec::with_capacity(capacity),
            flags: Vec::with_capacity(capacity),
            bindings: Vec::with_capacity(capacity),
        }
    }

    pub fn add_scope(&mut self, parent: Option<ScopeId>, flags: ScopeFlags) -> ScopeId {
        let id = ScopeId::from_usize(self.parent_ids.len());
        self.parent_ids.push(parent);
        self.flags.push(flags);
        self.bindings.push(ScopeBindings::new());
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

    pub fn get_binding(&self, scope: ScopeId, name: &str) -> Option<SymbolId> {
        self.bindings[scope.index()].get(name)
    }

    pub fn own_binding_names(&self, scope: ScopeId) -> impl Iterator<Item = &str> {
        self.bindings[scope.index()].names()
    }

    pub fn find_binding(&self, mut scope: ScopeId, name: &str) -> Option<SymbolId> {
        loop {
            if let Some(sym) = self.bindings[scope.index()].get(name) {
                return Some(sym);
            }
            scope = self.parent_ids[scope.index()]?;
        }
    }

    pub fn add_binding(&mut self, scope: ScopeId, name: CompactString, symbol: SymbolId) {
        self.bindings[scope.index()].insert(name, symbol);
    }

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
        let mut t = ScopeTable::with_capacity(0);
        let root = t.add_scope(None, ScopeFlags::Top);
        let child = t.add_scope(Some(root), ScopeFlags::empty());

        assert_eq!(t.scope_parent_id(root), None);
        assert_eq!(t.scope_parent_id(child), Some(root));
        assert_eq!(t.scope_flags(root), ScopeFlags::Top);
    }

    #[test]
    fn find_binding_walks_parents() {
        let mut t = ScopeTable::with_capacity(0);
        let root = t.add_scope(None, ScopeFlags::Top);
        let child = t.add_scope(Some(root), ScopeFlags::empty());

        t.add_binding(root, "x".into(), sym(0));

        assert_eq!(t.find_binding(child, "x"), Some(sym(0)));
        assert_eq!(t.find_binding(root, "x"), Some(sym(0)));
        assert_eq!(t.find_binding(child, "y"), None);
    }

    #[test]
    fn get_binding_no_walk() {
        let mut t = ScopeTable::with_capacity(0);
        let root = t.add_scope(None, ScopeFlags::Top);
        let child = t.add_scope(Some(root), ScopeFlags::empty());

        t.add_binding(root, "x".into(), sym(0));

        assert_eq!(t.get_binding(root, "x"), Some(sym(0)));
        assert_eq!(t.get_binding(child, "x"), None);
    }

    #[test]
    fn shadowing() {
        let mut t = ScopeTable::with_capacity(0);
        let root = t.add_scope(None, ScopeFlags::Top);
        let child = t.add_scope(Some(root), ScopeFlags::empty());

        t.add_binding(root, "x".into(), sym(0));
        t.add_binding(child, "x".into(), sym(1));

        assert_eq!(t.find_binding(child, "x"), Some(sym(1)));
        assert_eq!(t.find_binding(root, "x"), Some(sym(0)));
    }

    #[test]
    fn find_function_scope() {
        let mut t = ScopeTable::with_capacity(0);
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
        let mut t = ScopeTable::with_capacity(0);
        let a = t.add_scope(None, ScopeFlags::Top);
        let b = t.add_scope(None, ScopeFlags::Top);
        let c = t.add_scope(Some(a), ScopeFlags::empty());

        t.set_scope_parent_id(c, Some(b));
        assert_eq!(t.scope_parent_id(c), Some(b));
    }

    #[test]
    fn promote_small_to_large() {
        let mut t = ScopeTable::with_capacity(0);
        let root = t.add_scope(None, ScopeFlags::Top);
        let total = INLINE_BINDINGS + 4;
        for i in 0..total {
            t.add_binding(root, format!("v{i}").into(), sym(i));
        }
        for i in 0..total {
            assert_eq!(t.find_binding(root, &format!("v{i}")), Some(sym(i)));
        }
        assert_eq!(t.find_binding(root, "missing"), None);
    }

    #[test]
    fn small_overwrite_keeps_inline() {
        let mut t = ScopeTable::with_capacity(0);
        let root = t.add_scope(None, ScopeFlags::Top);
        t.add_binding(root, "x".into(), sym(0));
        t.add_binding(root, "x".into(), sym(7));
        assert_eq!(t.find_binding(root, "x"), Some(sym(7)));
    }

    #[test]
    fn own_binding_names_yields_inserted() {
        let mut t = ScopeTable::with_capacity(0);
        let root = t.add_scope(None, ScopeFlags::Top);
        t.add_binding(root, "a".into(), sym(0));
        t.add_binding(root, "b".into(), sym(1));
        let mut names: Vec<&str> = t.own_binding_names(root).collect();
        names.sort();
        assert_eq!(names, vec!["a", "b"]);
    }
}
