use rustc_hash::{FxHashMap, FxHashSet};
use svelte_ast::NodeId;
use svelte_component_semantics::{
    ComponentSemantics, OxcNodeId, Reference, ReferenceId, ScopeFlags, SymbolFlags, SymbolOwner,
};

use crate::types::data::FragmentKey;
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

/// Compatibility shim over `svelte_component_semantics::ComponentSemantics`.
///
/// This keeps the existing analyze/codegen query surface while the new
/// component-wide semantics storage becomes the single source of truth.
pub struct ComponentScoping {
    semantics: ComponentSemantics,
    runes: FxHashMap<SymbolId, Rune>,
    prop_source_syms: FxHashSet<SymbolId>,
    prop_non_source_names: FxHashMap<SymbolId, String>,
    store_syms: FxHashSet<SymbolId>,
    known_values: FxHashMap<SymbolId, String>,
    getter_syms: FxHashSet<SymbolId>,
    snippet_param_syms: FxHashSet<SymbolId>,
    snippet_name_syms: FxHashSet<SymbolId>,
    each_block_syms: FxHashSet<SymbolId>,
    each_rest_syms: FxHashSet<SymbolId>,
    each_non_reactive_syms: FxHashSet<SymbolId>,
    each_index_non_dynamic_syms: FxHashSet<SymbolId>,
    const_alias_tags: FxHashMap<SymbolId, NodeId>,
    dynamic_sym_cache: Option<FxHashSet<SymbolId>>,
    rest_prop_sym: Option<SymbolId>,
    rest_prop_excluded: FxHashSet<String>,
    var_state_syms: FxHashSet<SymbolId>,
}

impl ComponentScoping {
    pub fn new_empty() -> Self {
        Self::from_semantics(ComponentSemantics::new())
    }

    pub fn from_semantics(semantics: ComponentSemantics) -> Self {
        Self {
            semantics,
            runes: FxHashMap::default(),
            prop_source_syms: FxHashSet::default(),
            prop_non_source_names: FxHashMap::default(),
            store_syms: FxHashSet::default(),
            known_values: FxHashMap::default(),
            getter_syms: FxHashSet::default(),
            snippet_param_syms: FxHashSet::default(),
            snippet_name_syms: FxHashSet::default(),
            each_block_syms: FxHashSet::default(),
            each_rest_syms: FxHashSet::default(),
            each_non_reactive_syms: FxHashSet::default(),
            each_index_non_dynamic_syms: FxHashSet::default(),
            const_alias_tags: FxHashMap::default(),
            dynamic_sym_cache: None,
            rest_prop_sym: None,
            rest_prop_excluded: FxHashSet::default(),
            var_state_syms: FxHashSet::default(),
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

    // -- Scope management --

    pub fn root_scope_id(&self) -> ScopeId {
        self.semantics.root_scope_id()
    }

    pub fn add_child_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.semantics.add_child_scope(parent)
    }

    pub fn add_scope(&mut self, parent: Option<ScopeId>, flags: ScopeFlags) -> ScopeId {
        match parent {
            Some(parent) => self.semantics.add_scope(parent, flags),
            None => panic!("ComponentScoping::add_scope requires a parent scope"),
        }
    }

    pub fn module_scope_id(&self) -> Option<ScopeId> {
        self.semantics.module_scope_id()
    }

    pub fn scope_parent_id(&self, id: ScopeId) -> Option<ScopeId> {
        self.semantics.scope_parent_id(id)
    }

    pub fn scope_flags(&self, id: ScopeId) -> ScopeFlags {
        self.semantics.scope_flags(id)
    }

    // -- Symbol management --

    /// Synthetic binding helper used by some template side-table passes.
    pub fn add_binding(&mut self, scope: ScopeId, name: &str) -> SymbolId {
        self.semantics.add_binding(
            scope,
            name,
            oxc_span::SPAN,
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::Synthetic,
        )
    }

    pub fn find_binding(&self, scope: ScopeId, name: &str) -> Option<SymbolId> {
        self.semantics.find_binding(scope, name)
    }

    pub fn get_binding(&self, scope: ScopeId, name: &str) -> Option<SymbolId> {
        self.semantics.get_binding(scope, name)
    }

    pub fn is_mutated(&self, id: SymbolId) -> bool {
        self.semantics.is_mutated(id)
    }

    pub fn symbol_scope_id(&self, id: SymbolId) -> ScopeId {
        self.semantics.symbol_scope_id(id)
    }

    pub fn symbol_name(&self, id: SymbolId) -> &str {
        self.semantics.symbol_name(id)
    }

    pub fn symbol_declaration(&self, id: SymbolId) -> OxcNodeId {
        self.semantics.symbol_declaration(id)
    }

    pub fn symbol_flags(&self, id: SymbolId) -> SymbolFlags {
        self.semantics.symbol_flags(id)
    }

    pub fn is_component_top_level_scope(&self, scope_id: ScopeId) -> bool {
        self.semantics.is_component_top_level_scope(scope_id)
    }

    pub fn is_component_top_level_symbol(&self, sym_id: SymbolId) -> bool {
        self.semantics.is_component_top_level_symbol(sym_id)
    }

    pub fn function_depth(&self, scope: ScopeId) -> usize {
        self.semantics.function_depth(scope) as usize
    }

    pub fn is_template_reference(&self, ref_id: ReferenceId) -> bool {
        self.semantics.is_template_reference(ref_id)
    }

    pub fn create_reference(&mut self, reference: Reference) -> ReferenceId {
        self.semantics.create_reference(reference)
    }

    pub fn create_template_reference(&mut self, reference: Reference) -> ReferenceId {
        self.semantics.create_template_reference(reference)
    }

    pub fn add_resolved_reference(&mut self, sym_id: SymbolId, ref_id: ReferenceId) {
        self.semantics.add_resolved_reference(sym_id, ref_id);
    }

    pub fn get_reference(&self, ref_id: ReferenceId) -> &Reference {
        self.semantics.get_reference(ref_id)
    }

    pub fn try_get_reference(&self, ref_id: ReferenceId) -> Option<&Reference> {
        self.semantics.try_get_reference(ref_id)
    }

    pub fn get_reference_mut(&mut self, ref_id: ReferenceId) -> &mut Reference {
        self.semantics.get_reference_mut(ref_id)
    }

    pub fn has_write_reference_other_than(
        &self,
        sym_id: SymbolId,
        exclude_ref_id: ReferenceId,
    ) -> bool {
        self.semantics
            .get_resolved_reference_ids(sym_id)
            .iter()
            .copied()
            .any(|ref_id| {
                ref_id != exclude_ref_id && self.semantics.get_reference(ref_id).is_write()
            })
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

    pub fn mark_var_state(&mut self, id: SymbolId) {
        self.var_state_syms.insert(id);
    }

    pub fn is_var_declared_state(&self, id: SymbolId) -> bool {
        self.var_state_syms.contains(&id)
    }

    // -- Convenience: SymbolId-based dynamism check --

    pub fn is_dynamic_by_id(&self, sym_id: SymbolId) -> bool {
        if let Some(cache) = &self.dynamic_sym_cache {
            if cache.contains(&sym_id) {
                return true;
            }
            if !self.runes.contains_key(&sym_id) {
                if self.each_index_non_dynamic_syms.contains(&sym_id) {
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
        if self.each_index_non_dynamic_syms.contains(&sym_id) {
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

    // -- SymbolId-keyed classification: write --

    pub fn mark_prop_source(&mut self, sym_id: SymbolId) {
        self.prop_source_syms.insert(sym_id);
    }

    pub fn mark_prop_non_source(&mut self, sym_id: SymbolId, prop_name: String) {
        self.prop_non_source_names.insert(sym_id, prop_name);
    }

    pub fn mark_store(&mut self, sym_id: SymbolId) {
        self.store_syms.insert(sym_id);
    }

    pub fn set_known_value(&mut self, sym_id: SymbolId, value: String) {
        self.known_values.insert(sym_id, value);
    }

    pub fn mark_getter(&mut self, sym_id: SymbolId) {
        self.getter_syms.insert(sym_id);
    }

    pub fn mark_snippet_param(&mut self, sym_id: SymbolId) {
        self.snippet_param_syms.insert(sym_id);
    }

    pub fn mark_snippet_name(&mut self, sym_id: SymbolId) {
        self.snippet_name_syms.insert(sym_id);
    }

    pub fn mark_each_block_var(&mut self, sym_id: SymbolId) {
        self.each_block_syms.insert(sym_id);
    }

    pub fn mark_each_rest(&mut self, sym_id: SymbolId) {
        self.each_rest_syms.insert(sym_id);
    }

    pub fn mark_each_non_reactive(&mut self, sym_id: SymbolId) {
        self.each_non_reactive_syms.insert(sym_id);
    }

    pub fn mark_each_index_non_dynamic(&mut self, sym_id: SymbolId) {
        self.each_index_non_dynamic_syms.insert(sym_id);
    }

    pub fn mark_rest_prop(&mut self, sym_id: SymbolId, excluded: FxHashSet<String>) {
        self.rest_prop_sym = Some(sym_id);
        self.rest_prop_excluded = excluded;
    }

    // -- SymbolId-keyed classification: read --

    pub fn has_rest_prop(&self) -> bool {
        self.rest_prop_sym.is_some()
    }

    pub fn is_rest_prop(&self, sym_id: SymbolId) -> bool {
        self.rest_prop_sym == Some(sym_id)
    }

    pub fn is_rest_prop_excluded(&self, name: &str) -> bool {
        self.rest_prop_excluded.contains(name)
    }

    pub fn is_prop_source(&self, sym_id: SymbolId) -> bool {
        self.prop_source_syms.contains(&sym_id)
    }

    pub fn prop_non_source_name(&self, sym_id: SymbolId) -> Option<&str> {
        self.prop_non_source_names.get(&sym_id).map(|s| s.as_str())
    }

    pub fn is_store(&self, sym_id: SymbolId) -> bool {
        self.store_syms.contains(&sym_id)
    }

    pub fn store_symbol_ids(&self) -> &FxHashSet<SymbolId> {
        &self.store_syms
    }

    pub fn known_value_by_sym(&self, sym_id: SymbolId) -> Option<&str> {
        self.known_values.get(&sym_id).map(|s| s.as_str())
    }

    pub fn is_getter(&self, sym_id: SymbolId) -> bool {
        self.getter_syms.contains(&sym_id)
    }

    pub fn is_snippet_param(&self, sym_id: SymbolId) -> bool {
        self.snippet_param_syms.contains(&sym_id)
    }

    pub fn is_snippet_name(&self, sym_id: SymbolId) -> bool {
        self.snippet_name_syms.contains(&sym_id)
    }

    pub fn is_each_block_var(&self, sym_id: SymbolId) -> bool {
        self.each_block_syms.contains(&sym_id)
    }

    pub fn is_each_rest(&self, sym_id: SymbolId) -> bool {
        self.each_rest_syms.contains(&sym_id)
    }

    pub fn is_each_non_reactive(&self, sym_id: SymbolId) -> bool {
        self.each_non_reactive_syms.contains(&sym_id)
    }

    pub fn is_each_index_non_dynamic(&self, sym_id: SymbolId) -> bool {
        self.each_index_non_dynamic_syms.contains(&sym_id)
    }

    pub fn is_normal_binding(&self, sym_id: SymbolId) -> bool {
        !self.is_rune(sym_id)
            && !self.is_prop_source(sym_id)
            && self.prop_non_source_name(sym_id).is_none()
            && !self.is_getter(sym_id)
            && !self.is_each_block_var(sym_id)
            && !self.is_store(sym_id)
    }

    pub fn fragment_scope(&self, key: &FragmentKey) -> Option<ScopeId> {
        self.semantics.fragment_scope(key)
    }

    pub(crate) fn mark_const_alias(&mut self, sym_id: SymbolId, tag_id: NodeId) {
        self.const_alias_tags.insert(sym_id, tag_id);
    }

    pub fn const_alias_tag(&self, sym_id: SymbolId) -> Option<NodeId> {
        self.const_alias_tags.get(&sym_id).copied()
    }

    pub fn is_expr_local(&self, sym_id: SymbolId) -> bool {
        self.semantics.is_expr_local(sym_id)
    }

    pub(crate) fn build_template_scope_set(&mut self) {
        self.semantics.build_template_scope_set();
    }

    pub fn is_import(&self, sym_id: SymbolId) -> bool {
        self.semantics
            .symbol_flags(sym_id)
            .contains(SymbolFlags::Import)
    }

    pub fn collect_import_syms(&self) -> FxHashSet<SymbolId> {
        self.semantics.collect_import_syms()
    }

    pub fn collect_all_symbol_names(&self) -> FxHashSet<String> {
        self.semantics.collect_all_symbol_names()
    }

    pub fn find_binding_in_any_scope(&self, name: &str) -> Option<SymbolId> {
        self.semantics
            .symbol_ids()
            .find(|&id| self.semantics.symbol_name(id) == name)
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

    pub fn root_unresolved_references(
        &self,
    ) -> &FxHashMap<compact_str::CompactString, Vec<ReferenceId>> {
        self.semantics.root_unresolved_references()
    }
}
