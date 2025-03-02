use std::{collections::HashMap, env::consts::OS};

use hir::OwnerId;
use oxc_semantic::{ScopeTree, SymbolTable};

use crate::bitflags::{OwnerContentType, OwnerContentTypeFlags};

pub struct HirAnalyses {
    scope: ScopeTree,
    symbols: SymbolTable,
    content_types: HashMap<OwnerId, OwnerContentType>,
}

impl HirAnalyses {
    pub fn new(symbols: SymbolTable, scope: ScopeTree) -> Self {
        Self {
            scope,
            symbols,
            content_types: HashMap::new(),
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
}
