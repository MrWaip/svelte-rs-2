use rustc_hash::FxHashMap;
use std::ops::{Deref, DerefMut};
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, SymbolFlags, SymbolOwner};

pub use svelte_component_semantics::{ScopeId, SymbolId};

/// Per-symbol classification bits stored in `ComponentSemantics::state`.
/// Bits 0–7 are reserved for core semantics (MUTATED, etc.).
mod sym_class {
    // Bits 8-13 previously held `STORE` (C3), `GETTER` / `SNIPPET_PARAM` /
    // `SNIPPET_NAME` / `EACH_REST` / `EACH_NON_REACTIVE` (C5) — all moved to
    // `reactivity_semantics` as v2 side-tables.
    pub const EACH_INDEX_NON_DYNAMIC: u32 = 1 << 14;
    // Bit 15 previously held `VAR_STATE` — `$state` var-declared fact now lives
    // in `StateDeclarationSemantics::var_declared` inside `ReactivitySemantics`.
    // Bit 16 previously held `TEMPLATE_DECLARATION` — moved to
    // `reactivity_semantics` as `template_declaration_symbols` side-table.
}

/// Svelte component scoping — wraps `ComponentSemantics` with Svelte-specific
/// classification (runes, props, stores, etc.).
///
/// All `ComponentSemantics` methods are available via `Deref`/`DerefMut`.
///
/// The Svelte-specific helpers on this wrapper are transitional migration
/// surface. New semantic ownership belongs in `reactivity_semantics`, and new
/// builder code should depend on generic `ComponentSemantics` facts plus AST
/// rather than these Svelte-specific accessors.
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

    /// Synthetic binding helper used by template side-table passes.
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

    // Reactive meaning (rune classification, derived dep-graph, $state proxy
    // init) now lives exclusively in `reactivity_semantics::ReactivitySemantics`.
    // All v1 helpers (`mark_rune`, `set_derived_deps`, etc.) have been removed.

    // Removed in cluster C5: `mark_template_declaration`, `is_template_declaration`,
    // `mark_getter`, `is_getter`, `mark_snippet_param`, `is_snippet_param`,
    // `mark_snippet_name`, `is_snippet_name`, `mark_each_rest`, `is_each_rest`,
    // `mark_each_non_reactive`, `is_each_non_reactive` — all moved to
    // `reactivity_semantics::ReactivitySemantics` as v2 side-tables.

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

    // -- Convenience --

    pub fn find_binding_in_any_scope(&self, name: &str) -> Option<SymbolId> {
        self.semantics.find_symbol_by_name(name)
    }
}
