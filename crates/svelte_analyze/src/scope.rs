use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::{Deref, DerefMut};
use svelte_ast::NodeId;
use svelte_component_semantics::{ComponentSemantics, OxcNodeId, SymbolFlags, SymbolOwner};

use crate::types::script::RuneKind;

pub use svelte_component_semantics::{ScopeId, SymbolId};

pub struct Rune {
    pub kind: RuneKind,
    /// Symbols referenced in the init expression. Only populated for Derived/DerivedBy.
    pub derived_deps: Vec<SymbolId>,
    /// True when $state/$state.raw init is proxyable (array/object/non-primitive).
    /// Proxy-candidate state is reactive even without reassignment.
    pub is_proxy_init: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemplateBindingReadKind {
    Identifier,
    ThunkCall,
    RuneGet,
    RuneSafeGet,
    SlotLetCarrierMember,
    PropsAccess,
}

/// Per-symbol classification bits stored in `ComponentSemantics::state`.
/// Bits 0–7 are reserved for core semantics (MUTATED, etc.).
mod sym_class {
    pub const PROP_SOURCE: u32 = 1 << 8;
    pub const STORE: u32 = 1 << 9;
    pub const GETTER: u32 = 1 << 10;
    pub const SNIPPET_PARAM: u32 = 1 << 11;
    pub const SNIPPET_NAME: u32 = 1 << 12;
    pub const EACH_BLOCK_VAR: u32 = 1 << 13;
    pub const EACH_REST: u32 = 1 << 14;
    pub const EACH_NON_REACTIVE: u32 = 1 << 15;
    pub const EACH_INDEX_NON_DYNAMIC: u32 = 1 << 16;
    pub const VAR_STATE: u32 = 1 << 17;
    pub const TEMPLATE_DECLARATION: u32 = 1 << 18;
}

/// Svelte component scoping — wraps `ComponentSemantics` with Svelte-specific
/// classification (runes, props, stores, etc.).
///
/// All `ComponentSemantics` methods are available via `Deref`/`DerefMut`.
pub struct ComponentScoping {
    semantics: ComponentSemantics,
    runes: FxHashMap<SymbolId, Rune>,
    prop_non_source_names: FxHashMap<SymbolId, String>,
    known_values: FxHashMap<SymbolId, String>,
    const_alias_tags: FxHashMap<SymbolId, NodeId>,
    slot_let_attr_carriers: FxHashMap<NodeId, SymbolId>,
    slot_let_binding_carriers: FxHashMap<SymbolId, SymbolId>,
    dynamic_sym_cache: Option<FxHashSet<SymbolId>>,
    rest_prop_sym: Option<SymbolId>,
    rest_prop_excluded: FxHashSet<String>,
}

impl Deref for ComponentScoping {
    type Target = ComponentSemantics;
    fn deref(&self) -> &ComponentSemantics {
        &self.semantics
    }
}

impl DerefMut for ComponentScoping {
    fn deref_mut(&mut self) -> &mut ComponentSemantics {
        &mut self.semantics
    }
}

impl ComponentScoping {
    pub fn new_empty() -> Self {
        Self::from_semantics(ComponentSemantics::new())
    }

    pub fn from_semantics(semantics: ComponentSemantics) -> Self {
        Self {
            semantics,
            runes: FxHashMap::default(),
            prop_non_source_names: FxHashMap::default(),
            known_values: FxHashMap::default(),
            const_alias_tags: FxHashMap::default(),
            slot_let_attr_carriers: FxHashMap::default(),
            slot_let_binding_carriers: FxHashMap::default(),
            dynamic_sym_cache: None,
            rest_prop_sym: None,
            rest_prop_excluded: FxHashSet::default(),
        }
    }

    pub fn into_semantics(self) -> ComponentSemantics {
        self.semantics
    }

    pub fn semantics(&self) -> &ComponentSemantics {
        &self.semantics
    }

    pub fn semantics_mut(&mut self) -> &mut ComponentSemantics {
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

    // -- Rune tracking --

    pub fn mark_rune(&mut self, id: SymbolId, kind: RuneKind) {
        self.runes.insert(
            id,
            Rune {
                kind,
                derived_deps: Vec::new(),
                is_proxy_init: false,
            },
        );
    }

    pub fn mark_rune_with_proxy(&mut self, id: SymbolId, kind: RuneKind, is_proxy_init: bool) {
        self.runes.insert(
            id,
            Rune {
                kind,
                derived_deps: Vec::new(),
                is_proxy_init,
            },
        );
    }

    pub fn set_derived_deps(&mut self, id: SymbolId, deps: Vec<SymbolId>) {
        if let Some(rune) = self.runes.get_mut(&id) {
            rune.derived_deps = deps;
        }
    }

    pub fn rune_kind(&self, id: SymbolId) -> Option<RuneKind> {
        self.runes.get(&id).map(|r| r.kind)
    }

    pub fn is_rune(&self, id: SymbolId) -> bool {
        self.runes.contains_key(&id)
    }

    pub fn is_proxy_init_state(&self, id: SymbolId) -> bool {
        self.runes.get(&id).is_some_and(|r| r.is_proxy_init)
    }

    // -- State-bit classifications --

    pub fn mark_var_state(&mut self, id: SymbolId) {
        self.semantics.set_symbol_state(id, sym_class::VAR_STATE);
    }

    pub fn is_var_declared_state(&self, id: SymbolId) -> bool {
        self.semantics.has_symbol_state(id, sym_class::VAR_STATE)
    }

    pub fn mark_template_declaration(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::TEMPLATE_DECLARATION);
    }

    pub fn is_template_declaration(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::TEMPLATE_DECLARATION)
    }

    pub fn mark_prop_source(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::PROP_SOURCE);
    }

    pub fn is_prop_source(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::PROP_SOURCE)
    }

    pub fn mark_prop_non_source(&mut self, sym_id: SymbolId, prop_name: String) {
        self.prop_non_source_names.insert(sym_id, prop_name);
    }

    pub fn prop_non_source_name(&self, sym_id: SymbolId) -> Option<&str> {
        self.prop_non_source_names.get(&sym_id).map(|s| s.as_str())
    }

    pub fn mark_store(&mut self, sym_id: SymbolId) {
        self.semantics.set_symbol_state(sym_id, sym_class::STORE);
    }

    pub fn is_store(&self, sym_id: SymbolId) -> bool {
        self.semantics.has_symbol_state(sym_id, sym_class::STORE)
    }

    pub fn has_stores(&self) -> bool {
        self.semantics
            .symbols_with_state(sym_class::STORE)
            .next()
            .is_some()
    }

    pub fn store_symbol_ids(&self) -> impl Iterator<Item = SymbolId> + '_ {
        self.semantics.symbols_with_state(sym_class::STORE)
    }

    pub fn set_known_value(&mut self, sym_id: SymbolId, value: String) {
        self.known_values.insert(sym_id, value);
    }

    pub fn known_value_by_sym(&self, sym_id: SymbolId) -> Option<&str> {
        self.known_values.get(&sym_id).map(|s| s.as_str())
    }

    pub fn mark_getter(&mut self, sym_id: SymbolId) {
        self.semantics.set_symbol_state(sym_id, sym_class::GETTER);
    }

    pub fn is_getter(&self, sym_id: SymbolId) -> bool {
        self.semantics.has_symbol_state(sym_id, sym_class::GETTER)
    }

    pub fn mark_snippet_param(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::SNIPPET_PARAM);
    }

    pub fn is_snippet_param(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::SNIPPET_PARAM)
    }

    pub fn mark_snippet_name(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::SNIPPET_NAME);
    }

    pub fn is_snippet_name(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::SNIPPET_NAME)
    }

    pub fn mark_each_block_var(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::EACH_BLOCK_VAR);
    }

    pub fn is_each_block_var(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::EACH_BLOCK_VAR)
    }

    pub fn mark_each_rest(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::EACH_REST);
    }

    pub fn is_each_rest(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::EACH_REST)
    }

    pub fn mark_each_non_reactive(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::EACH_NON_REACTIVE);
    }

    pub fn is_each_non_reactive(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::EACH_NON_REACTIVE)
    }

    pub fn mark_each_index_non_dynamic(&mut self, sym_id: SymbolId) {
        self.semantics
            .set_symbol_state(sym_id, sym_class::EACH_INDEX_NON_DYNAMIC);
    }

    pub fn is_each_index_non_dynamic(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .has_symbol_state(sym_id, sym_class::EACH_INDEX_NON_DYNAMIC)
    }

    pub fn mark_rest_prop(&mut self, sym_id: SymbolId, excluded: FxHashSet<String>) {
        self.rest_prop_sym = Some(sym_id);
        self.rest_prop_excluded = excluded;
    }

    pub fn has_rest_prop(&self) -> bool {
        self.rest_prop_sym.is_some()
    }

    pub fn is_rest_prop(&self, sym_id: SymbolId) -> bool {
        self.rest_prop_sym == Some(sym_id)
    }

    pub fn is_rest_prop_excluded(&self, name: &str) -> bool {
        self.rest_prop_excluded.contains(name)
    }

    pub fn is_normal_binding(&self, sym_id: SymbolId) -> bool {
        !self.is_rune(sym_id)
            && !self.is_prop_source(sym_id)
            && self.prop_non_source_name(sym_id).is_none()
            && !self.is_getter(sym_id)
            && !self.is_each_block_var(sym_id)
            && !self.is_store(sym_id)
    }

    pub fn template_binding_read_kind(&self, sym_id: SymbolId) -> TemplateBindingReadKind {
        if !self.is_component_top_level_symbol(sym_id) {
            if self.is_expr_local(sym_id)
                || self.is_snippet_name(sym_id)
                || self.is_each_non_reactive(sym_id)
            {
                return TemplateBindingReadKind::Identifier;
            }

            if self.is_getter(sym_id) {
                return TemplateBindingReadKind::ThunkCall;
            }

            if self.slot_let_binding_carrier(sym_id).is_some() {
                return TemplateBindingReadKind::SlotLetCarrierMember;
            }

            return TemplateBindingReadKind::RuneGet;
        }

        if self.is_prop_source(sym_id) {
            return TemplateBindingReadKind::ThunkCall;
        }

        if self.prop_non_source_name(sym_id).is_some() {
            return TemplateBindingReadKind::PropsAccess;
        }

        if let Some(kind) = self.rune_kind(sym_id) {
            let needs_get = self.is_mutated(sym_id) || kind.is_derived();
            if needs_get {
                return if self.is_var_declared_state(sym_id) {
                    TemplateBindingReadKind::RuneSafeGet
                } else {
                    TemplateBindingReadKind::RuneGet
                };
            }
        }

        TemplateBindingReadKind::Identifier
    }

    pub(crate) fn mark_const_alias(&mut self, sym_id: SymbolId, tag_id: NodeId) {
        self.const_alias_tags.insert(sym_id, tag_id);
    }

    pub fn const_alias_tag(&self, sym_id: SymbolId) -> Option<NodeId> {
        self.const_alias_tags.get(&sym_id).copied()
    }

    pub fn add_unique_synthetic_binding(&mut self, scope: ScopeId, preferred_name: &str) -> SymbolId {
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

    pub fn mark_slot_let_carrier(&mut self, attr_id: NodeId, carrier_sym_id: SymbolId) {
        self.slot_let_attr_carriers.insert(attr_id, carrier_sym_id);
    }

    pub fn slot_let_carrier(&self, attr_id: NodeId) -> Option<SymbolId> {
        self.slot_let_attr_carriers.get(&attr_id).copied()
    }

    pub fn mark_slot_let_binding_carrier(&mut self, sym_id: SymbolId, carrier_sym_id: SymbolId) {
        self.slot_let_binding_carriers.insert(sym_id, carrier_sym_id);
    }

    pub fn slot_let_binding_carrier(&self, sym_id: SymbolId) -> Option<SymbolId> {
        self.slot_let_binding_carriers.get(&sym_id).copied()
    }

    // -- Convenience --

    pub fn find_binding_in_any_scope(&self, name: &str) -> Option<SymbolId> {
        self.semantics.find_symbol_by_name(name)
    }

    pub fn is_store_ref(&self, name: &str) -> bool {
        self.store_base_name(name).is_some()
    }

    pub fn store_base_name<'n>(&self, name: &'n str) -> Option<&'n str> {
        if name.starts_with('$') && name.len() > 1 {
            let base = &name[1..];
            let root = self.root_scope_id();
            if self
                .find_binding(root, base)
                .is_some_and(|sym| self.is_store(sym))
            {
                return Some(base);
            }
        }
        None
    }

    // -- Dynamism --

    pub fn is_dynamic_by_id(&self, sym_id: SymbolId) -> bool {
        if let Some(cache) = &self.dynamic_sym_cache {
            if cache.contains(&sym_id) {
                return true;
            }
            if !self.runes.contains_key(&sym_id) {
                if self.is_each_index_non_dynamic(sym_id) {
                    return false;
                }
                return !self.is_component_top_level_symbol(sym_id);
            }
            return false;
        }
        self.is_dynamic_by_id_uncached(sym_id, 0)
    }

    fn is_dynamic_by_id_uncached(&self, sym_id: SymbolId, depth: u8) -> bool {
        if depth > 16 {
            return true;
        }
        if let Some(rune) = self.runes.get(&sym_id) {
            if rune.kind == RuneKind::State && !self.is_mutated(sym_id) && !rune.is_proxy_init {
                return false;
            }
            if rune.kind.is_derived() {
                if rune.derived_deps.is_empty() {
                    return true;
                }
                return rune
                    .derived_deps
                    .iter()
                    .any(|&dep| self.is_dynamic_by_id_uncached(dep, depth + 1));
            }
            return true;
        }
        if self.is_each_index_non_dynamic(sym_id) {
            return false;
        }
        !self.is_component_top_level_symbol(sym_id)
    }

    pub fn precompute_dynamic_cache(&mut self) {
        let mut memo: FxHashMap<SymbolId, bool> = FxHashMap::default();
        let rune_ids: Vec<SymbolId> = self.runes.keys().copied().collect();
        for sym_id in rune_ids {
            self.compute_dynamic_memoized(sym_id, &mut memo, 0);
        }
        let dynamic_set = memo
            .into_iter()
            .filter(|(_, is_dyn)| *is_dyn)
            .map(|(sym, _)| sym)
            .collect();
        self.dynamic_sym_cache = Some(dynamic_set);
    }

    pub fn mark_blocked_symbols_dynamic(
        &mut self,
        symbol_blockers: &rustc_hash::FxHashMap<SymbolId, u32>,
    ) {
        let Some(cache) = &mut self.dynamic_sym_cache else {
            return;
        };
        for &sym in symbol_blockers.keys() {
            cache.insert(sym);
        }
        let rune_ids: Vec<(SymbolId, Vec<SymbolId>)> = self
            .runes
            .iter()
            .filter(|(_, rune)| rune.kind.is_derived())
            .map(|(&sym, rune)| (sym, rune.derived_deps.clone()))
            .collect();
        loop {
            let mut changed = false;
            for (sym, deps) in &rune_ids {
                if !cache.contains(sym) && deps.iter().any(|dep| cache.contains(dep)) {
                    cache.insert(*sym);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }

    fn compute_dynamic_memoized(
        &self,
        sym_id: SymbolId,
        memo: &mut FxHashMap<SymbolId, bool>,
        depth: u8,
    ) -> bool {
        if let Some(&cached) = memo.get(&sym_id) {
            return cached;
        }
        if depth > 16 {
            memo.insert(sym_id, true);
            return true;
        }
        let result = if let Some(rune) = self.runes.get(&sym_id) {
            if rune.kind == RuneKind::State && !self.is_mutated(sym_id) && !rune.is_proxy_init {
                false
            } else if rune.kind.is_derived() {
                if rune.derived_deps.is_empty() {
                    true
                } else {
                    let deps: Vec<SymbolId> = rune.derived_deps.clone();
                    deps.iter()
                        .any(|&dep| self.compute_dynamic_memoized(dep, memo, depth + 1))
                }
            } else {
                true
            }
        } else {
            !self.is_component_top_level_symbol(sym_id)
        };
        memo.insert(sym_id, result);
        result
    }
}
