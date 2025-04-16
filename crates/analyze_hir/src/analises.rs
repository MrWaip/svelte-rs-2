use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use hir::{ExpressionId, NodeId, OwnerId};
use oxc_ast::ast::Span;
use oxc_semantic::{
    NodeId as OxcNodeId, Reference, ReferenceId, ScopeFlags, ScopeId, ScopeTree, SymbolFlags,
    SymbolId, SymbolTable,
};
use oxc_span::SPAN;

use crate::{
    analyze_script::SvelteRune,
    bitflags::{ExpressionFlags, OwnerContentType, OwnerContentTypeFlags},
    indentifier_gen::IdentifierGen,
};

pub struct HirAnalyses {
    scope: RefCell<ScopeTree>,
    symbols: RefCell<SymbolTable>,
    content_types: HashMap<OwnerId, OwnerContentType>,
    identifier_generators: RefCell<HashMap<String, IdentifierGen>>,
    dynamic_nodes: HashSet<NodeId>,
    runes: HashMap<SymbolId, SvelteRune>,
    expression_flags: HashMap<ExpressionId, ExpressionFlags>,
}

impl HirAnalyses {
    pub fn new(symbols: SymbolTable, scope: ScopeTree) -> Self {
        Self {
            scope: RefCell::new(scope),
            symbols: RefCell::new(symbols),
            content_types: HashMap::new(),
            identifier_generators: RefCell::new(HashMap::new()),
            dynamic_nodes: HashSet::new(),
            runes: HashMap::new(),
            expression_flags: HashMap::new(),
        }
    }

    pub fn take_scoping(&self) -> (SymbolTable, ScopeTree) {
        (self.symbols.take(), self.scope.take())
    }

    pub fn set_scoping(&self, symbols: SymbolTable, scope: ScopeTree) {
        self.symbols.replace(symbols);
        self.scope.replace(scope);
    }

    pub fn set_content_type(&mut self, owner_id: OwnerId, content_type: OwnerContentType) {
        self.content_types.insert(owner_id, content_type);
    }

    pub fn get_content_type(&self, owner_id: &OwnerId) -> &OwnerContentType {
        self.content_types.get(owner_id).unwrap()
    }

    pub fn get_common_content_type(&self, owner_id: &OwnerId) -> OwnerContentTypeFlags {
        self.get_content_type(owner_id).as_common_or_empty()
    }

    pub fn mark_node_as_dynamic(&mut self, node_id: NodeId) {
        self.dynamic_nodes.insert(node_id);
    }

    pub fn is_dynamic(&self, node_id: &NodeId) -> bool {
        self.dynamic_nodes.contains(node_id)
    }

    pub fn add_rune(&mut self, symbol_id: SymbolId, rune: SvelteRune) {
        self.runes.insert(symbol_id, rune);
    }

    pub fn get_rune(&self, symbol_id: SymbolId) -> Option<&SvelteRune> {
        self.runes.get(&symbol_id)
    }

    pub fn generate_ident(&self, preferable_name: &str) -> String {
        let mut identifiers = self.identifier_generators.borrow_mut();

        let generator = if let Some(generator) = identifiers.get_mut(preferable_name) {
            generator
        } else {
            let generator = IdentifierGen::new(preferable_name.to_string());
            identifiers.insert(preferable_name.to_string(), generator);

            identifiers.get_mut(preferable_name).unwrap()
        };

        let mut identifier: String;

        loop {
            identifier = generator.next();
            let binding = self.scope.borrow().get_root_binding(&identifier);

            if binding.is_none() {
                break;
            }
        }

        let root_scope_id = self.scope.borrow().root_scope_id();

        let symbol_id = self.symbols.borrow_mut().create_symbol(
            Span::default(),
            &identifier,
            SymbolFlags::empty(),
            root_scope_id,
            OxcNodeId::DUMMY,
        );

        self.scope
            .borrow_mut()
            .add_binding(root_scope_id, &identifier, symbol_id);

        identifier
    }

    fn sync_rune(&mut self, symbol_id: SymbolId) {
        let mutated = self.symbol_is_mutated(symbol_id);
        let option = self.runes.get_mut(&symbol_id);

        if let Some(rune) = option {
            rune.mutated = mutated;
        }
    }

    pub fn symbol_is_mutated(&self, symbol_id: SymbolId) -> bool {
        return self.symbols.borrow().symbol_is_mutated(symbol_id);
    }

    pub fn root_scope_id(&self) -> ScopeId {
        return self.scope.borrow().root_scope_id();
    }

    pub(crate) fn add_reference(
        &mut self,
        name: &str,
        flags: oxc_semantic::ReferenceFlags,
        scope_id: ScopeId,
    ) -> Option<ReferenceId> {
        let symbol_id = self.scope.borrow().get_binding(scope_id, name);

        if let Some(symbol_id) = symbol_id {
            let mut reference = Reference::new(OxcNodeId::DUMMY, flags);
            reference.set_symbol_id(symbol_id);
            let reference_id = self.symbols.borrow_mut().create_reference(reference);
            self.symbols
                .borrow_mut()
                .add_resolved_reference(symbol_id, reference_id);

            self.sync_rune(symbol_id);

            return Some(reference_id);
        }

        None
    }

    pub fn get_rune_by_reference(&self, reference_id: ReferenceId) -> Option<&SvelteRune> {
        let binding = self.symbols.borrow();
        let reference = binding.get_reference(reference_id);
        let symbol_id = reference.symbol_id();

        symbol_id?;

        self.runes.get(&symbol_id.unwrap())
    }

    pub(crate) fn add_scope(&self) -> ScopeId {
        let root_scope = self.root_scope_id();
        return self.scope.borrow_mut().add_scope(
            Some(root_scope),
            OxcNodeId::DUMMY,
            ScopeFlags::empty(),
        );
    }

    pub(crate) fn add_binding(&mut self, name: &str, scope_id: ScopeId) -> SymbolId {
        let symbol_id = self.symbols.borrow_mut().create_symbol(
            SPAN,
            name,
            SymbolFlags::empty(),
            scope_id,
            OxcNodeId::DUMMY,
        );
        self.scope
            .borrow_mut()
            .add_binding(scope_id, name, symbol_id);

        return symbol_id;
    }

    pub(crate) fn set_expression_flags(
        &mut self,
        expression_id: ExpressionId,
        expression_flags: ExpressionFlags,
    ) {
        self.expression_flags
            .insert(expression_id, expression_flags);
    }

    pub fn get_expression_flags(&self, expression_id: ExpressionId) -> &ExpressionFlags {
        return self.expression_flags.get(&expression_id).unwrap();
    }
}
