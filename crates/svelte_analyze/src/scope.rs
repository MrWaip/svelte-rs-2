use std::ops::{Deref, DerefMut};
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, SymbolFlags, SymbolOwner};

pub use svelte_component_semantics::{ScopeId, SymbolId};

mod symbol_class {

    pub const EACH_INDEX_NON_DYNAMIC: u32 = 1 << 14;
}

pub struct ComponentScoping<'a> {
    semantics: ComponentSemantics<'a>,
}

impl<'a> Deref for ComponentScoping<'a> {
    type Target = ComponentSemantics<'a>;
    fn deref(&self) -> &ComponentSemantics<'a> {
        &self.semantics
    }
}

impl<'a> DerefMut for ComponentScoping<'a> {
    fn deref_mut(&mut self) -> &mut ComponentSemantics<'a> {
        &mut self.semantics
    }
}

impl<'a> ComponentScoping<'a> {
    pub fn new_empty() -> Self {
        Self::from_semantics(ComponentSemantics::new())
    }

    pub fn with_capacity(node_count: usize) -> Self {
        Self::from_semantics(ComponentSemantics::with_capacity(node_count))
    }

    pub fn from_semantics(semantics: ComponentSemantics<'a>) -> Self {
        Self { semantics }
    }

    pub fn into_semantics(self) -> ComponentSemantics<'a> {
        self.semantics
    }

    pub fn semantics(&self) -> &ComponentSemantics<'a> {
        &self.semantics
    }

    pub fn semantics_mut(&mut self) -> &mut ComponentSemantics<'a> {
        &mut self.semantics
    }

    pub fn add_synthetic_binding(&mut self, scope: ScopeId, name: &str) -> SymbolId {
        self.semantics.add_binding(
            scope,
            name,
            oxc_span::SPAN,
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::Synthetic,
        )
    }

    pub fn is_import(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .symbol_flags(sym_id)
            .contains(SymbolFlags::Import)
    }

    pub fn is_reexported_specifier_local(&self, sym_id: SymbolId) -> bool {
        self.semantics.is_reexported_specifier_local(sym_id)
    }

    pub(crate) fn mark_each_index_non_dynamic(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, symbol_class::EACH_INDEX_NON_DYNAMIC);
    }

    pub(crate) fn is_each_index_non_dynamic(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, symbol_class::EACH_INDEX_NON_DYNAMIC)
    }

    pub fn add_unique_synthetic_binding(
        &mut self,
        scope: ScopeId,
        preferred_name: &str,
    ) -> SymbolId {
        let mut name = preferred_name.to_string();
        let mut suffix = 0u32;
        while self.find_binding_in_any_scope(&name).is_some() {
            suffix += 1;
            name.clear();
            name.push_str(preferred_name);
            name.push('_');
            name.push_str(&suffix.to_string());
        }
        self.add_synthetic_binding(scope, &name)
    }

    pub fn find_binding_in_any_scope(&self, name: &str) -> Option<SymbolId> {
        self.semantics.find_symbol_by_name(name)
    }
}
