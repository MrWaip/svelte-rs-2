use rustc_hash::{FxHashMap, FxHashSet};
pub use oxc_semantic::{ScopeId, SymbolId};
use oxc_semantic::{NodeId as OxcNodeId, Reference as OxcReference, ReferenceId, ScopeFlags, Scoping, SymbolFlags};

use svelte_ast::NodeId;
use crate::types::script::RuneKind;

use crate::types::data::FragmentKey;

pub struct Rune {
    pub kind: RuneKind,
    /// Symbols referenced in the init expression. Only populated for Derived/DerivedBy.
    pub derived_deps: Vec<SymbolId>,
    /// True when $state/$state.raw init is proxyable (array/object/non-primitive).
    /// Proxy-candidate state is reactive even without reassignment.
    pub is_proxy_init: bool,
}

/// Unified scope tree for script + template, wrapping `oxc_semantic::Scoping`.
///
/// Script declarations come from OXC's `SemanticBuilder`. Template-introduced
/// bindings (each-block context/index) are added via `add_scope` / `add_binding`.
pub struct ComponentScoping {
    scoping: Scoping,
    template_reference_ids: FxHashSet<ReferenceId>,
    runes: FxHashMap<SymbolId, Rune>,
    // SymbolId-keyed classification fields (single source of truth for semantic decisions)
    prop_source_syms: FxHashSet<SymbolId>,
    prop_non_source_names: FxHashMap<SymbolId, String>,
    /// sym_id → base_name (e.g. SymbolId of "count" → "count")
    store_syms: FxHashSet<SymbolId>,
    known_values: FxHashMap<SymbolId, String>,
    getter_syms: FxHashSet<SymbolId>,
    snippet_name_syms: FxHashSet<SymbolId>,
    each_block_syms: FxHashSet<SymbolId>,
    /// Each-block vars that do NOT need `$.get()` wrapping (key_is_item optimization).
    each_non_reactive_syms: FxHashSet<SymbolId>,
    /// Unified scope index: FragmentKey → ScopeId for all template-introduced scopes
    /// (EachBody, SnippetBody, IfConsequent, IfAlternate, AwaitThen, AwaitCatch, etc.)
    fragment_scopes: FxHashMap<FragmentKey, ScopeId>,
    /// Derived from fragment_scopes — O(1) membership check for template scopes.
    /// Built by `build_template_scope_set()` after `build_scoping`.
    template_scope_set: FxHashSet<ScopeId>,
    /// SymbolId → parent ConstTag NodeId for destructured const bindings
    const_alias_tags: FxHashMap<SymbolId, NodeId>,
    /// Pre-computed set of dynamic rune symbols (populated by `precompute_dynamic_cache`).
    /// When `Some`, `is_dynamic_by_id` uses O(1) lookup instead of recursive walk.
    dynamic_sym_cache: Option<FxHashSet<SymbolId>>,
    /// SymbolId of the rest variable from `let { ...props } = $props()`
    rest_prop_sym: Option<SymbolId>,
    /// Prop names explicitly destructured before the rest element (excluded from rewriting)
    rest_prop_excluded: FxHashSet<String>,
    /// Symbols declared with `var` (as opposed to `let`/`const`) that are state runes.
    /// `var`-declared state must use `$.safe_get` instead of `$.get` because `var` hoisting
    /// means the binding may be read before its initializer runs.
    var_state_syms: FxHashSet<SymbolId>,
}

impl ComponentScoping {
    /// Create ComponentScoping from an optional OXC Scoping.
    ///
    /// - `Some(scoping)` — script block was parsed, use its scope tree (already has a root scope).
    /// - `None` — no script block; creates a minimal Scoping with just a root scope.
    pub fn new(scoping: Option<Scoping>) -> Self {
        let scoping = scoping.unwrap_or_else(|| {
            let mut s = Scoping::default();
            // Scoping::default() has no scopes, but root_scope_id() returns ScopeId(0).
            // Add a root scope so ScopeId(0) exists and child scopes get a valid parent.
            s.add_scope(None, OxcNodeId::DUMMY, ScopeFlags::empty());
            s
        });
        Self {
            scoping,
            template_reference_ids: FxHashSet::default(),
            runes: FxHashMap::default(),
            prop_source_syms: FxHashSet::default(),
            prop_non_source_names: FxHashMap::default(),
            store_syms: FxHashSet::default(),
            known_values: FxHashMap::default(),
            getter_syms: FxHashSet::default(),
            snippet_name_syms: FxHashSet::default(),
            each_block_syms: FxHashSet::default(),
            each_non_reactive_syms: FxHashSet::default(),
            fragment_scopes: FxHashMap::default(),
            template_scope_set: FxHashSet::default(),
            const_alias_tags: FxHashMap::default(),
            dynamic_sym_cache: None,
            rest_prop_sym: None,
            rest_prop_excluded: FxHashSet::default(),
            var_state_syms: FxHashSet::default(),
        }
    }

    // -- Scope management --

    pub fn root_scope_id(&self) -> ScopeId {
        self.scoping.root_scope_id()
    }

    pub fn add_child_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scoping
            .add_scope(Some(parent), OxcNodeId::DUMMY, ScopeFlags::empty())
    }

    // -- Symbol management --

    /// Declare a new binding in a scope. Creates symbol + adds binding.
    pub fn add_binding(&mut self, scope: ScopeId, name: &str) -> SymbolId {
        let ident: oxc_span::Ident<'_> = name.into();
        let symbol_id = self.scoping.create_symbol(
            oxc_span::SPAN,
            ident,
            SymbolFlags::empty(),
            scope,
            OxcNodeId::DUMMY,
        );
        self.scoping.add_binding(scope, name.into(), symbol_id);
        symbol_id
    }

    /// Find a binding by name, walking up parent scopes.
    pub fn find_binding(&self, scope: ScopeId, name: &str) -> Option<SymbolId> {
        self.scoping.find_binding(scope, name.into())
    }

    /// Check if a symbol has any mutation (script + template, all via OXC references).
    pub fn is_mutated(&self, id: SymbolId) -> bool {
        self.scoping.symbol_is_mutated(id)
    }

    /// Get the scope a symbol was declared in.
    pub fn symbol_scope_id(&self, id: SymbolId) -> ScopeId {
        self.scoping.symbol_scope_id(id)
    }

    /// Get the declared name of a symbol.
    pub fn symbol_name(&self, id: SymbolId) -> &str {
        self.scoping.symbol_name(id)
    }

    pub fn function_depth(&self, mut scope: ScopeId) -> usize {
        let mut depth = 0usize;
        loop {
            if self.scoping.scope_flags(scope).is_function() {
                depth += 1;
            }
            let Some(parent) = self.scoping.scope_parent_id(scope) else {
                break;
            };
            scope = parent;
        }
        depth
    }

    pub fn is_template_reference(&self, ref_id: ReferenceId) -> bool {
        self.template_reference_ids.contains(&ref_id)
    }

    // -- Rune tracking --

    pub fn mark_rune(&mut self, id: SymbolId, kind: RuneKind) {
        self.runes.insert(id, Rune { kind, derived_deps: Vec::new(), is_proxy_init: false });
    }

    pub fn mark_rune_with_proxy(&mut self, id: SymbolId, kind: RuneKind, is_proxy_init: bool) {
        self.runes.insert(id, Rune { kind, derived_deps: Vec::new(), is_proxy_init });
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

    /// True when a `$state`/`$state.raw` init argument is non-primitive (proxy candidate).
    /// Proxy-candidate state is still reactive through property mutations even without reassignment.
    pub fn is_proxy_init_state(&self, id: SymbolId) -> bool {
        self.runes.get(&id).is_some_and(|r| r.is_proxy_init)
    }

    /// Mark a state rune symbol as `var`-declared.
    /// `var`-declared state requires `$.safe_get` instead of `$.get` because var hoisting means
    /// the binding may be read before its initializer has run.
    pub fn mark_var_state(&mut self, id: SymbolId) {
        self.var_state_syms.insert(id);
    }

    /// True when the symbol is a state rune declared with `var`.
    pub fn is_var_declared_state(&self, id: SymbolId) -> bool {
        self.var_state_syms.contains(&id)
    }

    /// Store a Reference object and return its ReferenceId.
    pub fn create_reference(&mut self, reference: OxcReference) -> ReferenceId {
        self.scoping.create_reference(reference)
    }

    /// Template references are synthesized by template visitors and do not map
    /// to script AST nodes in OXC's semantic node table.
    pub fn create_template_reference(&mut self, reference: OxcReference) -> ReferenceId {
        let ref_id = self.scoping.create_reference(reference);
        self.template_reference_ids.insert(ref_id);
        ref_id
    }

    /// Record that a ReferenceId resolves to a SymbolId.
    pub fn add_resolved_reference(&mut self, sym_id: SymbolId, ref_id: ReferenceId) {
        self.scoping.add_resolved_reference(sym_id, ref_id);
    }

    /// Get a reference by ReferenceId.
    pub fn get_reference(&self, ref_id: ReferenceId) -> &OxcReference {
        self.scoping.get_reference(ref_id)
    }

    // -- Convenience: SymbolId-based dynamism check --

    /// Check if a symbol is dynamic (by SymbolId, without name resolution).
    /// After `precompute_dynamic_cache()`, rune checks are O(1) set lookups.
    pub fn is_dynamic_by_id(&self, sym_id: SymbolId) -> bool {
        if let Some(cache) = &self.dynamic_sym_cache {
            // Cached path: rune dynamism pre-computed, scope check is O(1)
            if cache.contains(&sym_id) {
                return true;
            }
            // Non-rune symbol: dynamic iff declared in non-root scope
            if !self.runes.contains_key(&sym_id) {
                return self.symbol_scope_id(sym_id) != self.root_scope_id();
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
            // Unmutated primitive $state is a compile-time constant (not dynamic).
            // Unmutated proxy-candidate $state (array/object) is still reactive via proxy.
            if rune.kind == RuneKind::State && !self.is_mutated(sym_id) && !rune.is_proxy_init {
                return false;
            }
            if rune.kind.is_derived() {
                if rune.derived_deps.is_empty() {
                    return true; // deps unknown (e.g. $derived.by) — assume dynamic
                }
                return rune.derived_deps.iter().any(|&dep| self.is_dynamic_by_id_uncached(dep, depth + 1));
            }
            return true;
        }
        // Non-root-scope binding (each block context/index) is always dynamic
        self.symbol_scope_id(sym_id) != self.root_scope_id()
    }

    /// Pre-compute dynamic status for all rune symbols with memoization.
    /// After this call, `is_dynamic_by_id` uses O(1) lookups for runes.
    pub fn precompute_dynamic_cache(&mut self) {
        let mut memo: FxHashMap<SymbolId, bool> = FxHashMap::default();
        let rune_ids: Vec<SymbolId> = self.runes.keys().copied().collect();
        for sym_id in rune_ids {
            self.compute_dynamic_memoized(sym_id, &mut memo, 0);
        }
        let dynamic_set = memo.into_iter()
            .filter(|(_, is_dyn)| *is_dyn)
            .map(|(sym, _)| sym)
            .collect();
        self.dynamic_sym_cache = Some(dynamic_set);
    }

    /// Add blocked symbols to the dynamic cache and propagate to derived symbols.
    /// Blocked symbols change when their async promise resolves, so they are reactive.
    pub fn mark_blocked_symbols_dynamic(&mut self, symbol_blockers: &rustc_hash::FxHashMap<oxc_semantic::SymbolId, u32>) {
        let Some(cache) = &mut self.dynamic_sym_cache else { return };
        for &sym in symbol_blockers.keys() {
            cache.insert(sym);
        }
        // Chains like a→b→c require iterating until stable — a single pass misses transitive deps.
        let rune_ids: Vec<(SymbolId, Vec<SymbolId>)> = self.runes.iter()
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
            if !changed { break; }
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
                    // Collect deps to avoid borrow conflict with &self
                    let deps: Vec<SymbolId> = rune.derived_deps.clone();
                    deps.iter().any(|&dep| self.compute_dynamic_memoized(dep, memo, depth + 1))
                }
            } else {
                true
            }
        } else {
            self.symbol_scope_id(sym_id) != self.root_scope_id()
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

    pub fn mark_snippet_name(&mut self, sym_id: SymbolId) {
        self.snippet_name_syms.insert(sym_id);
    }

    pub fn mark_each_block_var(&mut self, sym_id: SymbolId) {
        self.each_block_syms.insert(sym_id);
    }

    /// Mark each-block var as non-reactive (key_is_item optimization — no `$.get()` wrapping).
    pub fn mark_each_non_reactive(&mut self, sym_id: SymbolId) {
        self.each_non_reactive_syms.insert(sym_id);
    }

    pub fn mark_rest_prop(&mut self, sym_id: SymbolId, excluded: FxHashSet<String>) {
        self.rest_prop_sym = Some(sym_id);
        self.rest_prop_excluded = excluded;
    }

    // -- SymbolId-keyed classification: read --

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

    /// Returns store SymbolIds for codegen iteration.
    pub fn store_symbol_ids(&self) -> &FxHashSet<SymbolId> {
        &self.store_syms
    }

    pub fn known_value_by_sym(&self, sym_id: SymbolId) -> Option<&str> {
        self.known_values.get(&sym_id).map(|s| s.as_str())
    }

    pub fn is_getter(&self, sym_id: SymbolId) -> bool {
        self.getter_syms.contains(&sym_id)
    }

    pub fn is_snippet_name(&self, sym_id: SymbolId) -> bool {
        self.snippet_name_syms.contains(&sym_id)
    }

    pub fn is_each_block_var(&self, sym_id: SymbolId) -> bool {
        self.each_block_syms.contains(&sym_id)
    }

    /// Each-block var that does NOT need `$.get()` (key_is_item optimization in runes mode).
    pub fn is_each_non_reactive(&self, sym_id: SymbolId) -> bool {
        self.each_non_reactive_syms.contains(&sym_id)
    }

    /// A binding is "normal" if it's a regular const/let/function — not a rune, prop,
    /// snippet param, each var, or store subscription.
    pub fn is_normal_binding(&self, sym_id: SymbolId) -> bool {
        !self.is_rune(sym_id)
            && !self.is_prop_source(sym_id)
            && self.prop_non_source_name(sym_id).is_none()
            && !self.is_getter(sym_id)
            && !self.is_each_block_var(sym_id)
            && !self.is_store(sym_id)
    }

    pub(crate) fn set_fragment_scope(&mut self, key: FragmentKey, scope_id: ScopeId) {
        self.fragment_scopes.insert(key, scope_id);
    }

    pub fn fragment_scope(&self, key: &FragmentKey) -> Option<ScopeId> {
        self.fragment_scopes.get(key).copied()
    }

    pub(crate) fn mark_const_alias(&mut self, sym_id: SymbolId, tag_id: NodeId) {
        self.const_alias_tags.insert(sym_id, tag_id);
    }

    pub fn const_alias_tag(&self, sym_id: SymbolId) -> Option<NodeId> {
        self.const_alias_tags.get(&sym_id).copied()
    }

    /// True if this symbol was declared inside a template expression
    /// (arrow/function param, for-loop var, block-scoped var, catch param).
    /// True if this symbol was declared inside a JS construct in a template expression
    /// (arrow/function param, for-loop var, block-scoped var, catch param).
    /// Derived: non-root scope that isn't a template-introduced scope.
    pub fn is_expr_local(&self, sym_id: SymbolId) -> bool {
        let scope = self.symbol_scope_id(sym_id);
        scope != self.root_scope_id() && !self.template_scope_set.contains(&scope)
    }

    /// Build the template_scope_set from fragment_scopes. Must be called after build_scoping.
    pub(crate) fn build_template_scope_set(&mut self) {
        self.template_scope_set = self.fragment_scopes.values().copied().collect();
    }

    pub fn is_import(&self, sym_id: SymbolId) -> bool {
        self.scoping.symbol_flags(sym_id).contains(SymbolFlags::Import)
    }

    /// Collect all import-flagged SymbolIds from root scope.
    /// Used to pre-compute the set once during analysis rather than per-lookup in codegen.
    pub fn collect_import_syms(&self) -> FxHashSet<SymbolId> {
        let root = self.root_scope_id();
        self.scoping.iter_bindings_in(root)
            .filter(|&sym_id| self.scoping.symbol_flags(sym_id).contains(SymbolFlags::Import))
            .collect()
    }

    /// Collect all symbol names from all scopes (for IdentGen conflict detection).
    pub fn collect_all_symbol_names(&self) -> FxHashSet<String> {
        self.scoping.symbol_names().map(|s| s.to_string()).collect()
    }

    /// Check if a name is a store subscription (`$X` where `X` is marked as store in root scope).
    pub fn is_store_ref(&self, name: &str) -> bool {
        self.store_base_name(name).is_some()
    }

    /// Returns the base name (without `$` prefix) if this is a store reference.
    pub fn store_base_name<'n>(&self, name: &'n str) -> Option<&'n str> {
        if name.starts_with('$') && name.len() > 1 {
            let base = &name[1..];
            let root = self.root_scope_id();
            if self.find_binding(root, base).is_some_and(|sym| self.is_store(sym)) {
                return Some(base);
            }
        }
        None
    }

}
