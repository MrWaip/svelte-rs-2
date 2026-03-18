use rustc_hash::{FxHashMap, FxHashSet};

pub use oxc_semantic::{ScopeId, SymbolId};
use oxc_semantic::{NodeId as OxcNodeId, Reference as OxcReference, ReferenceFlags as OxcReferenceFlags, ScopeFlags, Scoping, SymbolFlags};

use svelte_ast::{Component, Fragment, Node, NodeId};
use svelte_js::RuneKind;

use crate::data::AnalysisData;

pub struct Rune {
    pub kind: RuneKind,
    /// Symbols referenced in the init expression. Only populated for Derived/DerivedBy.
    pub derived_deps: Vec<SymbolId>,
}

/// Unified scope tree for script + template, wrapping `oxc_semantic::Scoping`.
///
/// Script declarations come from OXC's `SemanticBuilder`. Template-introduced
/// bindings (each-block context/index) are added via `add_scope` / `add_binding`.
pub struct ComponentScoping {
    scoping: Scoping,
    runes: FxHashMap<SymbolId, Rune>,
    /// Our AST NodeId → OXC ScopeId for scope-introducing nodes (each blocks, snippets, await then).
    node_scopes: FxHashMap<NodeId, ScopeId>,
    /// AwaitBlock NodeId → ScopeId for the catch fragment (separate from node_scopes which stores then scope).
    await_catch_scopes: FxHashMap<NodeId, ScopeId>,
    // SymbolId-keyed classification fields (single source of truth for semantic decisions)
    prop_source_syms: FxHashSet<SymbolId>,
    prop_non_source_names: FxHashMap<SymbolId, String>,
    /// sym_id → base_name (e.g. SymbolId of "count" → "count")
    store_syms: FxHashMap<SymbolId, String>,
    known_values: FxHashMap<SymbolId, String>,
    snippet_param_syms: FxHashSet<SymbolId>,
}

impl ComponentScoping {
    /// Create from a pre-built OXC Scoping (produced by `analyze_script_with_scoping`).
    pub fn from_scoping(scoping: Scoping) -> Self {
        Self {
            scoping,
            runes: FxHashMap::default(),
            node_scopes: FxHashMap::default(),
            await_catch_scopes: FxHashMap::default(),
            prop_source_syms: FxHashSet::default(),
            prop_non_source_names: FxHashMap::default(),
            store_syms: FxHashMap::default(),
            known_values: FxHashMap::default(),
            snippet_param_syms: FxHashSet::default(),
        }
    }

    /// Create an empty scoping (no script block).
    /// Parses an empty source via `svelte_js` to get a valid root scope from OXC.
    pub fn empty() -> Self {
        let (_info, scoping) = svelte_js::analyze_script_with_scoping("", 0, false)
            .expect("empty script should parse");
        Self::from_scoping(scoping)
    }

    // -- Scope management --

    pub fn root_scope_id(&self) -> ScopeId {
        self.scoping.root_scope_id()
    }

    pub fn add_child_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scoping
            .add_scope(Some(parent), OxcNodeId::DUMMY, ScopeFlags::empty())
    }

    pub fn node_scope(&self, node_id: NodeId) -> Option<ScopeId> {
        self.node_scopes.get(&node_id).copied()
    }

    pub fn set_node_scope(&mut self, node_id: NodeId, scope_id: ScopeId) {
        self.node_scopes.insert(node_id, scope_id);
    }

    pub fn set_await_catch_scope(&mut self, node_id: NodeId, scope_id: ScopeId) {
        self.await_catch_scopes.insert(node_id, scope_id);
    }

    pub fn await_catch_scope(&self, node_id: NodeId) -> Option<ScopeId> {
        self.await_catch_scopes.get(&node_id).copied()
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

    // -- Rune tracking --

    pub fn mark_rune(&mut self, id: SymbolId, kind: RuneKind) {
        self.runes.insert(id, Rune { kind, derived_deps: Vec::new() });
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

    /// Register a template reference into OXC Scoping.
    /// For Write/ReadWrite refs, `symbol_is_mutated()` will return true.
    pub fn register_template_reference(&mut self, sym_id: SymbolId, flags: OxcReferenceFlags) {
        let scope = self.scoping.symbol_scope_id(sym_id);
        let reference = OxcReference::new(OxcNodeId::DUMMY, scope, flags);
        let ref_id = self.scoping.create_reference(reference);
        self.scoping.add_resolved_reference(sym_id, ref_id);
    }

    // -- Convenience: SymbolId-based dynamism check --

    /// Check if a symbol is dynamic (by SymbolId, without name resolution).
    pub fn is_dynamic_by_id(&self, sym_id: SymbolId) -> bool {
        self.is_dynamic_by_id_inner(sym_id, 0)
    }

    fn is_dynamic_by_id_inner(&self, sym_id: SymbolId, depth: u8) -> bool {
        if depth > 16 {
            return true;
        }
        if let Some(rune) = self.runes.get(&sym_id) {
            if rune.kind == RuneKind::State && !self.is_mutated(sym_id) {
                return false;
            }
            if rune.kind.is_derived() {
                if rune.derived_deps.is_empty() {
                    return true; // deps unknown (e.g. $derived.by) — assume dynamic
                }
                return rune.derived_deps.iter().any(|&dep| self.is_dynamic_by_id_inner(dep, depth + 1));
            }
            return true;
        }
        // Non-root-scope binding (each block context/index) is always dynamic
        self.symbol_scope_id(sym_id) != self.root_scope_id()
    }

    // -- SymbolId-keyed classification: write --

    pub fn mark_prop_source(&mut self, sym_id: SymbolId) {
        self.prop_source_syms.insert(sym_id);
    }

    pub fn mark_prop_non_source(&mut self, sym_id: SymbolId, prop_name: String) {
        self.prop_non_source_names.insert(sym_id, prop_name);
    }

    pub fn mark_store(&mut self, sym_id: SymbolId, base_name: String) {
        self.store_syms.insert(sym_id, base_name);
    }

    pub fn set_known_value(&mut self, sym_id: SymbolId, value: String) {
        self.known_values.insert(sym_id, value);
    }

    pub fn mark_snippet_param(&mut self, sym_id: SymbolId) {
        self.snippet_param_syms.insert(sym_id);
    }

    // -- SymbolId-keyed classification: read --

    pub fn is_prop_source(&self, sym_id: SymbolId) -> bool {
        self.prop_source_syms.contains(&sym_id)
    }

    pub fn prop_non_source_name(&self, sym_id: SymbolId) -> Option<&str> {
        self.prop_non_source_names.get(&sym_id).map(|s| s.as_str())
    }

    pub fn is_store(&self, sym_id: SymbolId) -> bool {
        self.store_syms.contains_key(&sym_id)
    }

    /// Returns the store_syms map for codegen iteration (sym_id → base_name).
    pub fn store_symbols(&self) -> &FxHashMap<SymbolId, String> {
        &self.store_syms
    }

    pub fn known_value_by_sym(&self, sym_id: SymbolId) -> Option<&str> {
        self.known_values.get(&sym_id).map(|s| s.as_str())
    }

    pub fn is_snippet_param(&self, sym_id: SymbolId) -> bool {
        self.snippet_param_syms.contains(&sym_id)
    }

    pub fn is_import(&self, sym_id: SymbolId) -> bool {
        self.scoping.symbol_flags(sym_id).contains(SymbolFlags::Import)
    }

}

// ---------------------------------------------------------------------------
// Build scoping pass
// ---------------------------------------------------------------------------

/// Build the unified ComponentScoping from script + template.
///
/// The `Scoping` is already initialized by `parse_js` (via `analyze_script_with_scoping`),
/// so this pass only marks runes and walks the template to add each-block scopes.
pub fn build_scoping(component: &Component, data: &mut AnalysisData) -> crate::markers::ScopingBuilt {
    // Mark runes from script declarations
    if let Some(script_info) = &data.script {
        for decl in &script_info.declarations {
            if let Some(rune_kind) = decl.is_rune {
                let root = data.scoping.root_scope_id();
                if let Some(sym_id) = data.scoping.find_binding(root, &decl.name) {
                    data.scoping.mark_rune(sym_id, rune_kind);
                    if rune_kind.is_derived()
                        && !decl.rune_init_refs.is_empty()
                    {
                        let deps: Vec<SymbolId> = decl.rune_init_refs.iter()
                            .filter_map(|name| data.scoping.find_binding(root, name))
                            .collect();
                        if !deps.is_empty() {
                            data.scoping.set_derived_deps(sym_id, deps);
                        }
                    }
                }
            }
        }
    }

    // Walk template to add each-block/snippet scopes and const bindings.
    // Take both tables temporarily to avoid split borrow on `data`.
    let root = data.scoping.root_scope_id();
    let const_tag_names = std::mem::take(&mut data.const_tags.names);
    let snippet_params = std::mem::take(&mut data.snippets.params);
    walk_template_scopes(&component.fragment, component, &mut data.scoping, root, &const_tag_names, &snippet_params);
    data.const_tags.names = const_tag_names;
    data.snippets.params = snippet_params;
    crate::markers::ScopingBuilt::new()
}

/// Extract binding names from a pattern string.
/// Simple identifiers return themselves; destructured patterns like `{ name, age }` extract identifiers.
fn extract_binding_names(pattern: &str) -> Vec<String> {
    let trimmed = pattern.trim();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        // Destructured — extract identifier names.
        // Simple approach: strip braces/brackets, split by comma, strip defaults
        let inner = &trimmed[1..trimmed.len()-1];
        inner.split(',')
            .filter_map(|s| {
                // Handle `name = default` or just `name`
                let s = s.split('=').next().unwrap_or("").trim();
                // Handle `key: alias` (rename)
                let s = if let Some((_key, alias)) = s.split_once(':') {
                    alias.trim()
                } else {
                    s
                };
                if s.is_empty() { None } else { Some(s.to_string()) }
            })
            .collect()
    } else {
        vec![trimmed.to_string()]
    }
}

fn walk_template_scopes(
    fragment: &Fragment,
    component: &Component,
    scoping: &mut ComponentScoping,
    current_scope: ScopeId,
    const_tag_names: &FxHashMap<NodeId, Vec<String>>,
    snippet_params: &FxHashMap<NodeId, Vec<String>>,
) {
    for node in &fragment.nodes {
        match node {
            Node::EachBlock(block) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_node_scope(block.id, child_scope);

                // Add context variable binding
                let context_name = component.source_text(block.context_span);
                scoping.add_binding(child_scope, context_name);

                // Add optional index variable binding
                if let Some(idx_span) = block.index_span {
                    let idx_name = component.source_text(idx_span);
                    scoping.add_binding(child_scope, idx_name);
                }

                walk_template_scopes(&block.body, component, scoping, child_scope, const_tag_names, snippet_params);

                // Fallback uses parent scope (no context/index vars)
                if let Some(fb) = &block.fallback {
                    walk_template_scopes(fb, component, scoping, current_scope, const_tag_names, snippet_params);
                }
            }
            Node::Element(el) => {
                walk_template_scopes(&el.fragment, component, scoping, current_scope, const_tag_names, snippet_params);
            }
            Node::ComponentNode(cn) => {
                walk_template_scopes(&cn.fragment, component, scoping, current_scope, const_tag_names, snippet_params);
            }
            Node::IfBlock(block) => {
                walk_template_scopes(&block.consequent, component, scoping, current_scope, const_tag_names, snippet_params);
                if let Some(alt) = &block.alternate {
                    walk_template_scopes(alt, component, scoping, current_scope, const_tag_names, snippet_params);
                }
            }
            Node::SnippetBlock(block) => {
                // Create a child scope for snippet params — they shadow outer bindings.
                let snippet_scope = scoping.add_child_scope(current_scope);
                scoping.set_node_scope(block.id, snippet_scope);
                if let Some(params) = snippet_params.get(&block.id) {
                    for param in params {
                        let sym_id = scoping.add_binding(snippet_scope, param);
                        scoping.mark_snippet_param(sym_id);
                    }
                }
                walk_template_scopes(&block.body, component, scoping, snippet_scope, const_tag_names, snippet_params);
            }
            Node::KeyBlock(block) => {
                walk_template_scopes(&block.fragment, component, scoping, current_scope, const_tag_names, snippet_params);
            }
            Node::SvelteHead(head) => {
                walk_template_scopes(&head.fragment, component, scoping, current_scope, const_tag_names, snippet_params);
            }
            Node::SvelteElement(el) => {
                walk_template_scopes(&el.fragment, component, scoping, current_scope, const_tag_names, snippet_params);
            }
            Node::SvelteBoundary(b) => {
                walk_template_scopes(&b.fragment, component, scoping, current_scope, const_tag_names, snippet_params);
            }
            Node::ConstTag(tag) => {
                if let Some(names) = const_tag_names.get(&tag.id) {
                    for name in names {
                        let sym_id = scoping.add_binding(current_scope, name);
                        scoping.mark_rune(sym_id, RuneKind::Derived);
                    }
                }
            }
            Node::AwaitBlock(block) => {
                // Pending fragment uses parent scope (no bindings)
                if let Some(ref p) = block.pending {
                    walk_template_scopes(p, component, scoping, current_scope, const_tag_names, snippet_params);
                }

                // Then fragment: create child scope if value binding exists
                if let Some(ref t) = block.then {
                    if let Some(val_span) = block.value_span {
                        let then_scope = scoping.add_child_scope(current_scope);
                        scoping.set_node_scope(block.id, then_scope);
                        let binding_text = component.source_text(val_span);
                        // For destructured bindings like `{ name, age }`, extract identifiers
                        for name in extract_binding_names(binding_text) {
                            scoping.add_binding(then_scope, &name);
                        }
                        walk_template_scopes(t, component, scoping, then_scope, const_tag_names, snippet_params);
                    } else {
                        walk_template_scopes(t, component, scoping, current_scope, const_tag_names, snippet_params);
                    }
                }

                // Catch fragment: create child scope if error binding exists
                if let Some(ref c) = block.catch {
                    if let Some(err_span) = block.error_span {
                        let catch_scope = scoping.add_child_scope(current_scope);
                        // Store catch scope separately — we use await_catch_scopes map
                        scoping.set_await_catch_scope(block.id, catch_scope);
                        let binding_text = component.source_text(err_span);
                        for name in extract_binding_names(binding_text) {
                            scoping.add_binding(catch_scope, &name);
                        }
                        walk_template_scopes(c, component, scoping, catch_scope, const_tag_names, snippet_params);
                    } else {
                        walk_template_scopes(c, component, scoping, current_scope, const_tag_names, snippet_params);
                    }
                }
            }
            Node::SvelteWindow(_) | Node::SvelteDocument(_) | Node::SvelteBody(_) => {}
            Node::ExpressionTag(_) | Node::Text(_) | Node::Comment(_) | Node::RenderTag(_) | Node::HtmlTag(_) | Node::DebugTag(_) | Node::Error(_) => {}
        }
    }
}
