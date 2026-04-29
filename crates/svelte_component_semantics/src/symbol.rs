use compact_str::CompactString;
use oxc_span::Span;
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::scope::ScopeId;
use oxc_syntax::symbol::{SymbolFlags, SymbolId};

pub mod state {

    pub const MUTATED: u32 = 1 << 0;

    pub const MEMBER_MUTATED: u32 = 1 << 1;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SymbolOwner {
    ModuleScript,

    InstanceScript,

    Template,

    Synthetic,
}

pub(crate) struct SymbolTable {
    names: Vec<CompactString>,
    spans: Vec<Span>,
    flags: Vec<SymbolFlags>,
    scope_ids: Vec<ScopeId>,
    declaration_node_ids: Vec<OxcNodeId>,
    resolved_references: Vec<Vec<ReferenceId>>,
    state: Vec<u32>,
    owners: Vec<SymbolOwner>,

    name_index: rustc_hash::FxHashMap<CompactString, SymbolId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            spans: Vec::new(),
            flags: Vec::new(),
            scope_ids: Vec::new(),
            declaration_node_ids: Vec::new(),
            resolved_references: Vec::new(),
            state: Vec::new(),
            owners: Vec::new(),
            name_index: rustc_hash::FxHashMap::default(),
        }
    }

    pub fn create_symbol(
        &mut self,
        name: CompactString,
        span: Span,
        flags: SymbolFlags,
        scope_id: ScopeId,
        node_id: OxcNodeId,
        owner: SymbolOwner,
    ) -> SymbolId {
        let id = SymbolId::from_usize(self.names.len());
        self.name_index.entry(name.clone()).or_insert(id);
        self.names.push(name);
        self.spans.push(span);
        self.flags.push(flags);
        self.scope_ids.push(scope_id);
        self.declaration_node_ids.push(node_id);
        self.resolved_references.push(Vec::new());
        self.state.push(0);
        self.owners.push(owner);
        id
    }

    pub fn symbol_name(&self, id: SymbolId) -> &str {
        &self.names[id.index()]
    }

    pub fn symbol_span(&self, id: SymbolId) -> Span {
        self.spans[id.index()]
    }

    pub fn symbol_flags(&self, id: SymbolId) -> SymbolFlags {
        self.flags[id.index()]
    }

    pub fn symbol_scope_id(&self, id: SymbolId) -> ScopeId {
        self.scope_ids[id.index()]
    }

    pub fn symbol_declaration(&self, id: SymbolId) -> OxcNodeId {
        self.declaration_node_ids[id.index()]
    }

    pub fn symbol_owner(&self, id: SymbolId) -> SymbolOwner {
        self.owners[id.index()]
    }

    pub fn is_mutated(&self, id: SymbolId) -> bool {
        self.state[id.index()] & state::MUTATED != 0
    }

    pub fn has_state(&self, id: SymbolId, bit: u32) -> bool {
        self.state[id.index()] & bit != 0
    }

    pub fn set_state(&mut self, id: SymbolId, bit: u32) {
        self.state[id.index()] |= bit;
    }

    pub fn add_resolved_reference(
        &mut self,
        symbol_id: SymbolId,
        reference_id: ReferenceId,
        is_write: bool,
    ) {
        self.resolved_references[symbol_id.index()].push(reference_id);
        if is_write {
            self.state[symbol_id.index()] |= state::MUTATED;
        }
    }

    pub fn get_resolved_reference_ids(&self, id: SymbolId) -> &[ReferenceId] {
        &self.resolved_references[id.index()]
    }

    pub fn symbol_ids(&self) -> impl Iterator<Item = SymbolId> {
        (0..self.names.len()).map(SymbolId::from_usize)
    }

    pub fn symbol_names(&self) -> impl Iterator<Item = &str> {
        self.names.iter().map(|n| n.as_str())
    }

    pub fn find_by_name(&self, name: &str) -> Option<SymbolId> {
        self.name_index.get(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_symbol(table: &mut SymbolTable, name: &str) -> SymbolId {
        table.create_symbol(
            name.into(),
            Span::default(),
            SymbolFlags::empty(),
            ScopeId::from_usize(0),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        )
    }

    #[test]
    fn create_and_query() {
        let mut t = SymbolTable::new();
        let id = make_symbol(&mut t, "foo");

        assert_eq!(t.symbol_name(id), "foo");
        assert_eq!(t.symbol_scope_id(id), ScopeId::from_usize(0));
        assert!(!t.is_mutated(id));
        assert_eq!(t.symbol_owner(id), SymbolOwner::InstanceScript);
    }

    #[test]
    fn eager_mutation_on_write_ref() {
        let mut t = SymbolTable::new();
        let id = make_symbol(&mut t, "x");
        let ref_id = ReferenceId::from_usize(0);

        t.add_resolved_reference(id, ref_id, false);
        assert!(!t.is_mutated(id));

        let ref_id2 = ReferenceId::from_usize(1);
        t.add_resolved_reference(id, ref_id2, true);
        assert!(t.is_mutated(id));
    }

    #[test]
    fn resolved_references_tracked() {
        let mut t = SymbolTable::new();
        let id = make_symbol(&mut t, "x");

        assert!(t.get_resolved_reference_ids(id).is_empty());

        let r0 = ReferenceId::from_usize(0);
        let r1 = ReferenceId::from_usize(1);
        t.add_resolved_reference(id, r0, false);
        t.add_resolved_reference(id, r1, true);

        assert_eq!(t.get_resolved_reference_ids(id), &[r0, r1]);
    }

    #[test]
    fn symbol_flags() {
        let mut t = SymbolTable::new();
        let id = t.create_symbol(
            "imp".into(),
            Span::default(),
            SymbolFlags::Import,
            ScopeId::from_usize(0),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );
        assert!(t.symbol_flags(id).contains(SymbolFlags::Import));
    }

    #[test]
    fn owner_tracking() {
        let mut t = SymbolTable::new();
        let m = t.create_symbol(
            "a".into(),
            Span::default(),
            SymbolFlags::empty(),
            ScopeId::from_usize(0),
            OxcNodeId::DUMMY,
            SymbolOwner::ModuleScript,
        );
        let i = t.create_symbol(
            "b".into(),
            Span::default(),
            SymbolFlags::empty(),
            ScopeId::from_usize(0),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );
        let tpl = t.create_symbol(
            "c".into(),
            Span::default(),
            SymbolFlags::empty(),
            ScopeId::from_usize(0),
            OxcNodeId::DUMMY,
            SymbolOwner::Template,
        );
        let syn = t.create_symbol(
            "d".into(),
            Span::default(),
            SymbolFlags::empty(),
            ScopeId::from_usize(0),
            OxcNodeId::DUMMY,
            SymbolOwner::Synthetic,
        );

        assert_eq!(t.symbol_owner(m), SymbolOwner::ModuleScript);
        assert_eq!(t.symbol_owner(i), SymbolOwner::InstanceScript);
        assert_eq!(t.symbol_owner(tpl), SymbolOwner::Template);
        assert_eq!(t.symbol_owner(syn), SymbolOwner::Synthetic);
    }

    #[test]
    fn symbol_ids_iterator() {
        let mut t = SymbolTable::new();
        make_symbol(&mut t, "a");
        make_symbol(&mut t, "b");
        make_symbol(&mut t, "c");

        let ids: Vec<_> = t.symbol_ids().collect();
        assert_eq!(ids.len(), 3);
        assert_eq!(t.symbol_name(ids[0]), "a");
        assert_eq!(t.symbol_name(ids[2]), "c");
    }

    #[test]
    fn symbol_names_iterator() {
        let mut t = SymbolTable::new();
        make_symbol(&mut t, "x");
        make_symbol(&mut t, "y");

        let names: Vec<_> = t.symbol_names().collect();
        assert_eq!(names, vec!["x", "y"]);
    }
}
