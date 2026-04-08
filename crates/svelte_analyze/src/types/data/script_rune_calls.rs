use oxc_syntax::node::NodeId as OxcNodeId;
use rustc_hash::FxHashMap;

use crate::types::script::RuneKind;

#[derive(Default)]
pub struct ScriptRuneCalls {
    by_node: FxHashMap<OxcNodeId, RuneKind>,
}

impl ScriptRuneCalls {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn record(&mut self, node: OxcNodeId, kind: RuneKind) {
        self.by_node.insert(node, kind);
    }

    pub fn kind(&self, node: OxcNodeId) -> Option<RuneKind> {
        self.by_node.get(&node).copied()
    }

    pub fn is_rune_call(&self, node: OxcNodeId) -> bool {
        self.by_node.contains_key(&node)
    }

    pub fn iter(&self) -> impl Iterator<Item = (OxcNodeId, RuneKind)> + '_ {
        self.by_node.iter().map(|(&node, &kind)| (node, kind))
    }
}
