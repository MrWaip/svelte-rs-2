pub mod builder;
pub mod data;

pub use builder::build;
pub use data::{
    Evaluation, ExpressionData, ExpressionSemantics, KnownValue, LegacyWrap, Suspension,
    SyntheticPropsCarrier, ValueClass, Volatility,
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
        const UNSAFE_CALLEE_OR_NEW  = 1 << 3;
        const SCRIPT_CONTEXT        = 1 << 4;
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExpressionSemanticsStore {
    entries: FxHashMap<u32, u32>,
    by_oxc: FxHashMap<OxcNodeId, u32>,
    values: Vec<ExpressionSemantics>,
    context_signals: ContextSignal,
}

const NON_SPECIAL: &ExpressionSemantics = &ExpressionSemantics::NonSpecial;

impl ExpressionSemanticsStore {
    pub(crate) fn new(node_count: u32) -> Self {
        let cap = node_count as usize / 4;
        Self {
            entries: FxHashMap::with_capacity_and_hasher(cap, Default::default()),
            by_oxc: FxHashMap::with_capacity_and_hasher(cap, Default::default()),
            values: Vec::with_capacity(cap),
            context_signals: ContextSignal::empty(),
        }
    }

    pub fn get(&self, id: NodeId) -> &ExpressionSemantics {
        match self.entries.get(&id.0) {
            Some(slot) => &self.values[*slot as usize],
            None => NON_SPECIAL,
        }
    }

    pub fn get_by_oxc(&self, id: OxcNodeId) -> &ExpressionSemantics {
        match self.by_oxc.get(&id) {
            Some(slot) => &self.values[*slot as usize],
            None => NON_SPECIAL,
        }
    }

    pub fn is_context_required(&self) -> bool {
        !self.context_signals.is_empty()
    }

    fn push_value(&mut self, value: ExpressionSemantics) -> u32 {
        let slot = self.values.len() as u32;
        self.values.push(value);
        slot
    }

    pub(crate) fn set(&mut self, id: NodeId, value: ExpressionSemantics) {
        match self.entries.get(&id.0) {
            Some(slot) => self.values[*slot as usize] = value,
            None => {
                let slot = self.push_value(value);
                self.entries.insert(id.0, slot);
            }
        }
    }

    pub(crate) fn set_by_oxc(&mut self, id: OxcNodeId, value: ExpressionSemantics) {
        match self.by_oxc.get(&id) {
            Some(slot) => self.values[*slot as usize] = value,
            None => {
                let slot = self.push_value(value);
                self.by_oxc.insert(id, slot);
            }
        }
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
