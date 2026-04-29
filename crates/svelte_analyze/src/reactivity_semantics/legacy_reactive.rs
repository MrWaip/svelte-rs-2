use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_component_semantics::OxcNodeId;

use crate::scope::SymbolId;

pub const LEGACY_REACTIVE_IMPORT_PREFIX: &str = "$$_import_";

pub fn legacy_reactive_import_wrapper_name(import_name: &str) -> String {
    format!("{LEGACY_REACTIVE_IMPORT_PREFIX}{import_name}")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegacyReactiveStatement {
    pub stmt_node: OxcNodeId,
    pub kind: LegacyReactiveKind,
    pub assignments: SmallVec<[SymbolId; 4]>,
    pub dependencies: SmallVec<[SymbolId; 8]>,
    pub uses_props: bool,
    pub uses_rest_props: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LegacyReactiveKind {
    SimpleAssignment {
        target_sym: SymbolId,
        implicit_decl: bool,
    },

    DestructureAssignment {
        target_syms: SmallVec<[SymbolId; 4]>,
        implicit_decl_syms: SmallVec<[SymbolId; 4]>,
    },

    Block,

    ExpressionOnly,

    Conditional,
}

#[derive(Clone, Debug, Default)]
pub struct LegacyReactivitySemantics {
    statements: FxHashMap<OxcNodeId, LegacyReactiveStatement>,

    topo_order: SmallVec<[OxcNodeId; 4]>,

    implicit_reactive_locals: FxHashSet<SymbolId>,

    mutated_imports: SmallVec<[SymbolId; 2]>,

    cycle_path: Option<SmallVec<[OxcNodeId; 4]>>,
}

impl LegacyReactivitySemantics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter_statements_topo(&self) -> impl Iterator<Item = &LegacyReactiveStatement> + '_ {
        self.topo_order
            .iter()
            .filter_map(|node_id| self.statements.get(node_id))
    }

    pub fn is_implicit_reactive_local(&self, sym: SymbolId) -> bool {
        self.implicit_reactive_locals.contains(&sym)
    }

    pub fn iter_mutated_imports(&self) -> impl Iterator<Item = SymbolId> + '_ {
        self.mutated_imports.iter().copied()
    }

    pub fn cycle_path(&self) -> Option<&[OxcNodeId]> {
        self.cycle_path.as_deref()
    }

    pub(crate) fn set_cycle_path(&mut self, path: SmallVec<[OxcNodeId; 4]>) {
        self.cycle_path = Some(path);
    }

    pub fn is_mutated_import(&self, sym: SymbolId) -> bool {
        self.mutated_imports.contains(&sym)
    }

    pub(crate) fn add_mutated_import(&mut self, sym: SymbolId) {
        if !self.mutated_imports.contains(&sym) {
            self.mutated_imports.push(sym);
        }
    }

    pub(crate) fn record_statement(&mut self, statement: LegacyReactiveStatement) {
        self.statements.insert(statement.stmt_node, statement);
    }

    pub(crate) fn set_topo_order(&mut self, order: SmallVec<[OxcNodeId; 4]>) {
        self.topo_order = order;
    }

    pub(crate) fn mark_implicit_reactive_local(&mut self, sym: SymbolId) {
        self.implicit_reactive_locals.insert(sym);
    }
}
