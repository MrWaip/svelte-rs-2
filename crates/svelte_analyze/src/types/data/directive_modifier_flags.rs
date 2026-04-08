use super::NodeTable;
use bitflags::bitflags;
use svelte_ast::NodeId;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct EventModifier: u16 {
        const ONCE = 1 << 0;
        const CAPTURE = 1 << 1;
        const PREVENT_DEFAULT = 1 << 2;
        const STOP_PROPAGATION = 1 << 3;
        const STOP_IMMEDIATE_PROPAGATION = 1 << 4;
        const NONPASSIVE = 1 << 5;
        const PASSIVE = 1 << 6;
        const TRUSTED = 1 << 7;
        const SELF = 1 << 8;
        const GLOBAL = 1 << 9;
    }
}

pub struct DirectiveModifierFlags {
    flags: NodeTable<EventModifier>,
}

impl DirectiveModifierFlags {
    pub fn new(node_count: u32) -> Self {
        Self {
            flags: NodeTable::new(node_count),
        }
    }

    pub(crate) fn record(&mut self, id: NodeId, flags: EventModifier) {
        if !flags.is_empty() {
            self.flags.insert(id, flags);
        }
    }

    pub fn get(&self, id: NodeId) -> EventModifier {
        self.flags.get(id).copied().unwrap_or_default()
    }

    pub fn has(&self, id: NodeId, modifier: EventModifier) -> bool {
        self.get(id).contains(modifier)
    }
}
