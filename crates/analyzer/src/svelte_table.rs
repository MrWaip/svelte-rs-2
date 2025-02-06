use std::{collections::HashMap, ptr::addr_of};

use ast::ExpressionFlags;
use oxc_ast::ast::Expression;
use oxc_semantic::{
    NodeId, Reference, ReferenceFlags, ReferenceId, ScopeTree, SymbolId, SymbolTable,
};

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
pub struct SvelteTable<'a> {
    pub(crate) symbols: SymbolTable,
    pub(crate) scopes: ScopeTree,
    pub(crate) runes: HashMap<SymbolId, Rune>,
    expressions_flags: HashMap<*const Expression<'a>, ExpressionFlags>,
}

impl<'a> SvelteTable<'a> {
    pub fn new(symbols: SymbolTable, scopes: ScopeTree) -> Self {
        return Self {
            expressions_flags: HashMap::default(),
            runes: HashMap::default(),
            scopes,
            symbols,
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

    pub fn add_expression_flag(&mut self, expression: &Expression<'a>, flags: ExpressionFlags) {
        let pointer = addr_of!(*expression);
        self.expressions_flags.insert(pointer, flags);
    }

    pub fn get_expression_flag(&self, expression: &Expression<'a>) -> Option<&ExpressionFlags> {
        let pointer = addr_of!(*expression);

        return self.expressions_flags.get(&pointer);
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

    pub fn merge_expression_flags(&self, flags: Vec<Option<&ExpressionFlags>>) -> ExpressionFlags {
        let mut aggregated = ExpressionFlags::empty();

        if flags.is_empty() {
            return aggregated;
        }

        for option in flags {
            if let Some(flag) = option {
                if flag.has_call {
                    aggregated.has_call = true;
                }

                if flag.has_state {
                    aggregated.has_state = true;
                }
            }
        }

        return aggregated;
    }
}
