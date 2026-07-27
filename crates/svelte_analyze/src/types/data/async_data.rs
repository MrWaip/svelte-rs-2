use super::*;
use crate::expression_semantics::Suspension;
use svelte_component_semantics::OxcNodeId;

#[derive(Debug, Default)]
pub struct BlockerData {
    pub(crate) symbol_blockers: FxHashMap<SymbolId, BlockerSlot>,
    pub(crate) has_async: bool,
    pub(crate) first_await_index: Option<usize>,
    pub(crate) entries: Vec<AsyncEntry>,
    pub(crate) hoisted_names: Vec<String>,
    pub(crate) member_lookup: FxHashMap<OxcNodeId, AsyncEntryLocation>,
    pub(crate) symbol_member_lookup: FxHashMap<SymbolId, AsyncEntryLocation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockerSlot {
    pub entry: u32,
    pub member: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncEntryMemberKind {
    Declarator,
    Statement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsyncEntryLocation {
    pub entry: usize,
    pub kind: AsyncEntryMemberKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncEntryMember {
    pub(crate) node: OxcNodeId,
    pub(crate) stmt_index: usize,
    pub(crate) kind: AsyncEntryMemberKind,
    pub(crate) symbols: Vec<SymbolId>,
}

#[derive(Debug, Clone)]
pub struct AsyncEntry {
    pub(crate) members: Vec<AsyncEntryMember>,
    pub(crate) suspension: Suspension,
}

impl AsyncEntry {
    pub fn suspension(&self) -> Suspension {
        self.suspension
    }

    pub fn suspends(&self) -> bool {
        self.suspension.suspends()
    }
}

impl BlockerData {
    pub fn has_async(&self) -> bool {
        self.has_async
    }

    pub fn symbol_blocker(&self, sym: SymbolId) -> Option<BlockerSlot> {
        self.symbol_blockers.get(&sym).copied()
    }

    pub fn first_await_index(&self) -> Option<usize> {
        self.first_await_index
    }

    pub fn entries(&self) -> &[AsyncEntry] {
        &self.entries
    }

    pub fn hoisted_names(&self) -> &[String] {
        &self.hoisted_names
    }

    pub fn entry_location(&self, node: OxcNodeId) -> Option<AsyncEntryLocation> {
        if node == OxcNodeId::DUMMY {
            return None;
        }
        self.member_lookup.get(&node).copied()
    }

    pub fn entry_location_of_symbol(&self, sym: SymbolId) -> Option<AsyncEntryLocation> {
        self.symbol_member_lookup.get(&sym).copied()
    }

    pub fn entry_location_at(&self, stmt_index: usize) -> Option<AsyncEntryLocation> {
        for (entry, members) in self.entries.iter().enumerate() {
            for member in &members.members {
                if member.stmt_index == stmt_index {
                    return Some(AsyncEntryLocation {
                        entry,
                        kind: member.kind,
                    });
                }
            }
        }
        None
    }
}
