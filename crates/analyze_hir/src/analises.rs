use std::{cell::RefCell, collections::HashMap};

use hir::OwnerId;
use oxc_ast::ast::Span;
use oxc_semantic::{NodeId, ScopeTree, SymbolFlags, SymbolTable};

use crate::{
    bitflags::{OwnerContentType, OwnerContentTypeFlags},
    indentifier_gen::IdentifierGen,
};

pub struct HirAnalyses {
    scope: RefCell<ScopeTree>,
    symbols: RefCell<SymbolTable>,
    content_types: HashMap<OwnerId, OwnerContentType>,
    identifier_generators: RefCell<HashMap<String, IdentifierGen>>,
}

impl HirAnalyses {
    pub fn new(symbols: SymbolTable, scope: ScopeTree) -> Self {
        Self {
            scope: RefCell::new(scope),
            symbols: RefCell::new(symbols),
            content_types: HashMap::new(),
            identifier_generators: RefCell::new(HashMap::new()),
        }
    }

    pub fn set_content_type(&mut self, owner_id: OwnerId, content_type: OwnerContentType) {
        self.content_types.insert(owner_id, content_type);
    }

    fn get_content_type(&self, owner_id: &OwnerId) -> &OwnerContentType {
        return self.content_types.get(owner_id).unwrap();
    }

    pub fn get_common_content_type(&self, owner_id: &OwnerId) -> OwnerContentTypeFlags {
        return self.get_content_type(owner_id).as_common_or_empty();
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
            NodeId::DUMMY,
        );

        self.scope
            .borrow_mut()
            .add_binding(root_scope_id, &identifier, symbol_id);

        return identifier;
    }
}
