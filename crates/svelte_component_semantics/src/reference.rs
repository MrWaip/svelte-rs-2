use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::reference::{ReferenceFlags, ReferenceId};
use oxc_syntax::scope::ScopeId;
use oxc_syntax::symbol::SymbolId;

#[derive(Clone, Debug)]
pub struct Reference {
    node_id: OxcNodeId,
    scope_id: ScopeId,
    flags: ReferenceFlags,
    symbol_id: Option<SymbolId>,
}

impl Reference {
    pub fn new(node_id: OxcNodeId, scope_id: ScopeId, flags: ReferenceFlags) -> Self {
        Self {
            node_id,
            scope_id,
            flags,
            symbol_id: None,
        }
    }

    pub fn node_id(&self) -> OxcNodeId {
        self.node_id
    }

    pub fn scope_id(&self) -> ScopeId {
        self.scope_id
    }

    pub fn flags(&self) -> ReferenceFlags {
        self.flags
    }

    pub fn flags_mut(&mut self) -> &mut ReferenceFlags {
        &mut self.flags
    }

    pub fn symbol_id(&self) -> Option<SymbolId> {
        self.symbol_id
    }

    pub fn set_symbol_id(&mut self, id: SymbolId) {
        self.symbol_id = Some(id);
    }

    pub fn is_read(&self) -> bool {
        self.flags.is_read()
    }

    pub fn is_write(&self) -> bool {
        self.flags.is_write()
    }
}

pub(crate) struct ReferenceTable {
    references: Vec<Reference>,
}

impl ReferenceTable {
    pub fn new() -> Self {
        Self {
            references: Vec::new(),
        }
    }

    pub fn create_reference(&mut self, reference: Reference) -> ReferenceId {
        let id = ReferenceId::from_usize(self.references.len());
        self.references.push(reference);
        id
    }

    pub fn get(&self, id: ReferenceId) -> &Reference {
        &self.references[id.index()]
    }

    pub fn get_mut(&mut self, id: ReferenceId) -> &mut Reference {
        &mut self.references[id.index()]
    }

    pub fn len(&self) -> usize {
        self.references.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_query() {
        let mut t = ReferenceTable::new();
        let id = t.create_reference(Reference::new(
            OxcNodeId::DUMMY,
            ScopeId::from_usize(0),
            ReferenceFlags::Read,
        ));

        let r = t.get(id);
        assert!(r.is_read());
        assert!(!r.is_write());
        assert_eq!(r.symbol_id(), None);
    }

    #[test]
    fn resolve_reference() {
        let mut t = ReferenceTable::new();
        let id = t.create_reference(Reference::new(
            OxcNodeId::DUMMY,
            ScopeId::from_usize(0),
            ReferenceFlags::Write,
        ));

        let sym = SymbolId::from_usize(5);
        t.get_mut(id).set_symbol_id(sym);

        assert_eq!(t.get(id).symbol_id(), Some(sym));
        assert!(t.get(id).is_write());
    }
}
