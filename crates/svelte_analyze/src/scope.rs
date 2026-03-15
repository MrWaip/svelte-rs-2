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
    /// Our AST NodeId → OXC ScopeId for scope-introducing nodes (each blocks).
    node_scopes: FxHashMap<NodeId, ScopeId>,
}

impl ComponentScoping {
    /// Create from a pre-built OXC Scoping (produced by `analyze_script_with_scoping`).
    pub fn from_scoping(scoping: Scoping) -> Self {
        Self {
            scoping,
            runes: FxHashMap::default(),
            node_scopes: FxHashMap::default(),
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

    // -- Unique name generation --

    /// Generate a unique name that doesn't conflict with any binding in the scope chain.
    /// Similar to Svelte's `scope.generate()`.
    pub fn generate(&self, base: &str, scope: ScopeId) -> String {
        let mut candidate = base.to_string();
        let mut counter = 1u32;
        while self.find_binding(scope, &candidate).is_some() {
            candidate = format!("{}_{}", base, counter);
            counter += 1;
        }
        candidate
    }

    // -- Name-based lookup for codegen (works with names, not SymbolIds) --

    /// Lookup rune kind + mutated status by variable name (root scope).
    pub fn rune_info_by_name(&self, name: &str) -> Option<(RuneKind, bool)> {
        let root = self.root_scope_id();
        let sym_id = self.find_binding(root, name)?;
        let kind = self.rune_kind(sym_id)?;
        Some((kind, self.is_mutated(sym_id)))
    }

}

// ---------------------------------------------------------------------------
// Build scoping pass
// ---------------------------------------------------------------------------

/// Build the unified ComponentScoping from script + template.
///
/// The `Scoping` is already initialized by `parse_js` (via `analyze_script_with_scoping`),
/// so this pass only marks runes and walks the template to add each-block scopes.
pub fn build_scoping(component: &Component, data: &mut AnalysisData) {
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

    // Walk template to add each-block scopes and const bindings
    let root = data.scoping.root_scope_id();
    // Take temporarily to avoid split borrow on `data`
    let const_tag_names = std::mem::take(&mut data.const_tags.names);
    let destructured_ids: FxHashSet<NodeId> = data.const_tags.destructured.keys().copied().collect();
    let mut const_sym_ids: Vec<(NodeId, String, SymbolId)> = Vec::new();
    let mut generated_temps: Vec<(NodeId, String)> = Vec::new();
    walk_template_scopes(&component.fragment, component, &mut data.scoping, root, &const_tag_names, &destructured_ids, &mut const_sym_ids, &mut generated_temps);
    data.const_tags.names = const_tag_names;

    // Store generated temp names and build binding_to_temp mapping
    for (node_id, temp_name) in &generated_temps {
        if let Some(info) = data.const_tags.destructured.get_mut(node_id) {
            info.temp_name = temp_name.clone();
        }
    }
    for (node_id, _name, sym_id) in &const_sym_ids {
        if let Some(info) = data.const_tags.destructured.get(node_id) {
            if !info.temp_name.is_empty() {
                data.const_tags.binding_to_temp.insert(*sym_id, info.temp_name.clone());
            }
        }
    }
}

fn walk_template_scopes(
    fragment: &Fragment,
    component: &Component,
    scoping: &mut ComponentScoping,
    current_scope: ScopeId,
    const_tag_names: &FxHashMap<NodeId, Vec<String>>,
    destructured_ids: &FxHashSet<NodeId>,
    const_sym_ids: &mut Vec<(NodeId, String, SymbolId)>,
    generated_temps: &mut Vec<(NodeId, String)>,
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

                walk_template_scopes(&block.body, component, scoping, child_scope, const_tag_names, destructured_ids, const_sym_ids, generated_temps);

                // Fallback uses parent scope (no context/index vars)
                if let Some(fb) = &block.fallback {
                    walk_template_scopes(fb, component, scoping, current_scope, const_tag_names, destructured_ids, const_sym_ids, generated_temps);
                }
            }
            Node::Element(el) => {
                walk_template_scopes(&el.fragment, component, scoping, current_scope, const_tag_names, destructured_ids, const_sym_ids, generated_temps);
            }
            Node::ComponentNode(cn) => {
                walk_template_scopes(&cn.fragment, component, scoping, current_scope, const_tag_names, destructured_ids, const_sym_ids, generated_temps);
            }
            Node::IfBlock(block) => {
                walk_template_scopes(&block.consequent, component, scoping, current_scope, const_tag_names, destructured_ids, const_sym_ids, generated_temps);
                if let Some(alt) = &block.alternate {
                    walk_template_scopes(alt, component, scoping, current_scope, const_tag_names, destructured_ids, const_sym_ids, generated_temps);
                }
            }
            Node::SnippetBlock(block) => {
                walk_template_scopes(&block.body, component, scoping, current_scope, const_tag_names, destructured_ids, const_sym_ids, generated_temps);
            }
            Node::KeyBlock(block) => {
                walk_template_scopes(&block.fragment, component, scoping, current_scope, const_tag_names, destructured_ids, const_sym_ids, generated_temps);
            }
            Node::ConstTag(tag) => {
                if let Some(names) = const_tag_names.get(&tag.id) {
                    let is_destructured = destructured_ids.contains(&tag.id);
                    if is_destructured {
                        // Generate unique temp name, checking scope for conflicts
                        let temp_name = scoping.generate("computed_const", current_scope);
                        let temp_sym = scoping.add_binding(current_scope, &temp_name);
                        scoping.mark_rune(temp_sym, RuneKind::Derived);
                        generated_temps.push((tag.id, temp_name));
                    }
                    for name in names {
                        let sym_id = scoping.add_binding(current_scope, name);
                        if !is_destructured {
                            scoping.mark_rune(sym_id, RuneKind::Derived);
                        }
                        const_sym_ids.push((tag.id, name.clone(), sym_id));
                    }
                }
            }
            Node::ExpressionTag(_) | Node::Text(_) | Node::Comment(_) | Node::RenderTag(_) | Node::HtmlTag(_) | Node::Error(_) => {}
        }
    }
}
