pub mod builder;
pub mod data;

pub use builder::build;
pub use data::{ExprKind, ExpressionData, ExpressionSemantics, LegacyWrap, Memoization};

use svelte_ast::NodeId;

#[derive(Debug, Default, Clone)]
pub struct ExpressionSemanticsStore {
    entries: Vec<ExpressionSemantics>,
}

impl ExpressionSemanticsStore {
    pub fn new(node_count: u32) -> Self {
        let mut entries = Vec::with_capacity(node_count as usize);
        entries.resize_with(node_count as usize, ExpressionSemantics::default);
        Self { entries }
    }

    pub fn get(&self, id: NodeId) -> &ExpressionSemantics {
        self.entries
            .get(id.0 as usize)
            .unwrap_or(&ExpressionSemantics::NonSpecial)
    }

    pub(crate) fn set(&mut self, id: NodeId, value: ExpressionSemantics) {
        let idx = id.0 as usize;
        if idx >= self.entries.len() {
            self.entries
                .resize_with(idx + 1, ExpressionSemantics::default);
        }
        self.entries[idx] = value;
    }
}
