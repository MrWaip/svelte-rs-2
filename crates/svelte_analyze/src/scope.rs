use rustc_hash::FxHashMap;
use std::ops::{Deref, DerefMut};
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, SymbolFlags, SymbolOwner};

pub use svelte_component_semantics::{ScopeId, SymbolId};

mod sym_class {

    pub const EACH_INDEX_NON_DYNAMIC: u32 = 1 << 14;
}

pub struct ComponentScoping<'a> {
    semantics: ComponentSemantics<'a>,
    known_values: FxHashMap<SymbolId, String>,
    rest_prop_sym: Option<SymbolId>,
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

    pub fn from_semantics(semantics: ComponentSemantics<'a>) -> Self {
        Self {
            semantics,
            known_values: FxHashMap::default(),
            rest_prop_sym: None,
        }
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

    pub(crate) fn set_known_value(&mut self, sym_id: SymbolId, value: String) {
        self.known_values.insert(sym_id, value);
    }

    pub(crate) fn known_value_by_sym(&self, sym_id: SymbolId) -> Option<&str> {
        self.known_values.get(&sym_id).map(|s| s.as_str())
    }

    pub(crate) fn mark_each_index_non_dynamic(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::EACH_INDEX_NON_DYNAMIC);
    }

    pub(crate) fn is_each_index_non_dynamic(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::EACH_INDEX_NON_DYNAMIC)
    }

    pub(crate) fn mark_rest_prop_sym(&mut self, sym_id: SymbolId) {
        self.rest_prop_sym = Some(sym_id);
    }

    pub(crate) fn is_rest_prop(&self, sym_id: SymbolId) -> bool {
        self.rest_prop_sym == Some(sym_id)
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
