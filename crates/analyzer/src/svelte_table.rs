use std::collections::HashMap;

use oxc_index::IndexVec;
use oxc_semantic::{
    NodeId, Reference, ReferenceFlags, ReferenceId, ScopeTree, SymbolId, SymbolTable,
};

use crate::compute_optimization::OptimizationResult;

#[derive(Debug)]
pub struct Rune {
    pub mutated: bool,
    pub kind: RuneKind,
}

#[derive(Debug)]
pub enum RuneKind {
    State,
}

#[derive(Debug, Default)]
pub struct SvelteTable {
    pub(crate) symbols: SymbolTable,
    pub(crate) scopes: ScopeTree,
    pub(crate) runes: HashMap<SymbolId, Rune>,
    pub(crate) optimizations: HashMap<NodeId, OptimizationResult>,
    pub(crate) need_binding_group: bool,
}

impl SvelteTable {
    pub fn new(symbols: SymbolTable, scopes: ScopeTree) -> Self {
        return Self {
            runes: HashMap::default(),
            scopes,
            symbols,
            optimizations: HashMap::new(),
            need_binding_group: false,
        };
    }

    pub fn add_rune(&mut self, id: SymbolId, kind: RuneKind) {
        self.runes.insert(
            id,
            Rune {
                kind,
                mutated: self.symbol_is_mutated(id),
            },
        );
    }

    pub fn add_root_scope_reference(
        &mut self,
        name: &str,
        flags: ReferenceFlags,
    ) -> Option<ReferenceId> {
        let symbol_id = self.scopes.get_root_binding(name);

        if let Some(symbol_id) = symbol_id {
            let mut reference = Reference::new(NodeId::DUMMY, flags);
            reference.set_symbol_id(symbol_id);
            let reference_id = self.symbols.create_reference(reference);
            self.symbols.add_resolved_reference(symbol_id, reference_id);
            self.sync_rune(symbol_id);

            return Some(reference_id);
        }

        return None;
    }

    fn sync_rune(&mut self, symbol_id: SymbolId) {
        let mutated = self.symbol_is_mutated(symbol_id);
        let option = self.runes.get_mut(&symbol_id);

        if let Some(rune) = option {
            rune.mutated = mutated;
        }
    }

    pub fn symbol_is_mutated(&self, symbol_id: SymbolId) -> bool {
        return self.symbols.symbol_is_mutated(symbol_id);
    }

    pub fn is_rune_reference(&self, reference_id: ReferenceId) -> bool {
        return self.get_rune_by_reference(reference_id).is_some();
    }

    pub fn get_rune_by_reference(&self, reference_id: ReferenceId) -> Option<&Rune> {
        let reference = self.symbols.get_reference(reference_id);
        let symbol_id = reference.symbol_id();

        if symbol_id.is_none() {
            return None;
        }

        return self.runes.get(&symbol_id.unwrap());
    }

    pub fn get_rune_by_symbol_id(&self, symbol_id: SymbolId) -> Option<&Rune> {
        return self.runes.get(&symbol_id);
    }

    pub fn root_scope_id(&self) -> oxc_semantic::ScopeId {
        return self.scopes.root_scope_id();
    }

    pub fn add_optimization(&mut self, node_id: NodeId, opt: OptimizationResult) {
        self.optimizations.insert(node_id, opt);
    }

    pub fn get_optimization(&self, node_id: NodeId) -> Option<&OptimizationResult> {
        return self.optimizations.get(&node_id);
    }

    pub fn need_binding_group(&self) -> bool {
        return self.need_binding_group;
    }

    pub fn set_need_binding_group(&mut self) {
        self.need_binding_group = true;
    }
}
