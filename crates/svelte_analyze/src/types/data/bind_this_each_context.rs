use smallvec::SmallVec;
use svelte_ast::NodeId;

use super::NodeTable;
use crate::scope::SymbolId;

pub struct BindThisEachContext {
    by_attr: NodeTable<SmallVec<[SymbolId; 4]>>,
}

impl BindThisEachContext {
    pub fn new(node_count: u32) -> Self {
        Self {
            by_attr: NodeTable::new(node_count),
        }
    }

    pub fn set(&mut self, attr_id: NodeId, syms: SmallVec<[SymbolId; 4]>) {
        self.by_attr.insert(attr_id, syms);
    }

    pub fn get(&self, attr_id: NodeId) -> Option<&[SymbolId]> {
        self.by_attr.get(attr_id).map(|syms| syms.as_slice())
    }
}
