pub mod builder;
pub mod data;

pub use builder::build;
pub use data::{
    Evaluation, ExpressionData, ExpressionSemantics, KnownValue, LegacyWrap, SyntheticPropsCarrier,
    ValueClass, Volatility,
};

use bitflags::bitflags;
use rustc_hash::FxHashMap;
use svelte_ast::NodeId;
use svelte_component_semantics::OxcNodeId;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub(crate) struct ContextSignal: u8 {
        const IMPORT_OR_PROP_MEMBER = 1 << 0;
        const REST_PROP_MEMBER      = 1 << 1;
        const STORE_MUTATION        = 1 << 2;
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExpressionSemanticsStore {
    entries: Vec<ExpressionSemantics>,
    by_oxc: FxHashMap<OxcNodeId, ExpressionSemantics>,
    context_signals: ContextSignal,
}

impl ExpressionSemanticsStore {
    pub(crate) fn new(node_count: u32) -> Self {
        let mut entries = Vec::with_capacity(node_count as usize);
        entries.resize_with(node_count as usize, ExpressionSemantics::default);
        Self {
            entries,
            by_oxc: FxHashMap::default(),
            context_signals: ContextSignal::empty(),
        }
    }

    pub fn get(&self, id: NodeId) -> &ExpressionSemantics {
        self.entries
            .get(id.0 as usize)
            .unwrap_or(&ExpressionSemantics::NonSpecial)
    }

    pub fn get_by_oxc(&self, id: OxcNodeId) -> &ExpressionSemantics {
        self.by_oxc
            .get(&id)
            .unwrap_or(&ExpressionSemantics::NonSpecial)
    }

    pub fn is_context_required(&self) -> bool {
        !self.context_signals.is_empty()
    }

    pub(crate) fn set(&mut self, id: NodeId, value: ExpressionSemantics) {
        let idx = id.0 as usize;
        if idx >= self.entries.len() {
            self.entries
                .resize_with(idx + 1, ExpressionSemantics::default);
        }
        self.entries[idx] = value;
    }

    pub(crate) fn set_by_oxc(&mut self, id: OxcNodeId, value: ExpressionSemantics) {
        self.by_oxc.insert(id, value);
    }

    pub(crate) fn note_context(&mut self, signal: ContextSignal) {
        self.context_signals |= signal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_store_does_not_require_context() {
        let store = ExpressionSemanticsStore::new(0);
        assert!(!store.is_context_required());
    }

    #[test]
    fn noting_any_signal_marks_store_as_requiring_context() {
        let mut store = ExpressionSemanticsStore::new(0);
        store.note_context(ContextSignal::REST_PROP_MEMBER);
        assert!(store.is_context_required());
    }
}
