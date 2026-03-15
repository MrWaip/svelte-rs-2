use rustc_hash::FxHashMap;

pub use oxc_semantic::{ScopeId, SymbolId};
use oxc_semantic::{NodeId as OxcNodeId, Reference as OxcReference, ReferenceFlags as OxcReferenceFlags, ScopeFlags, Scoping, SymbolFlags};

use svelte_ast::{Component, Fragment, Node, NodeId};
use svelte_js::RuneKind;

use crate::data::AnalysisData;

/// Unified scope tree for script + template, wrapping `oxc_semantic::Scoping`.
///
/// Script declarations come from OXC's `SemanticBuilder`. Template-introduced
/// bindings (each-block context/index) are added via `add_scope` / `add_binding`.
pub struct ComponentScoping {
    scoping: Scoping,
    /// Which symbols are runes.
    runes: FxHashMap<SymbolId, RuneKind>,
    /// Our AST NodeId → OXC ScopeId for scope-introducing nodes (each blocks).
    node_scopes: FxHashMap<NodeId, ScopeId>,
    /// For $derived/$derived.by: symbols referenced in the init expression.
    derived_deps: FxHashMap<SymbolId, Vec<SymbolId>>,
}

impl ComponentScoping {
    /// Create from a pre-built OXC Scoping (produced by `analyze_script_with_scoping`).
    pub fn from_scoping(scoping: Scoping) -> Self {
        Self {
            scoping,
            runes: FxHashMap::default(),
            node_scopes: FxHashMap::default(),
            derived_deps: FxHashMap::default(),
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
        self.runes.insert(id, kind);
    }

    pub fn set_derived_deps(&mut self, id: SymbolId, deps: Vec<SymbolId>) {
        self.derived_deps.insert(id, deps);
    }

    pub fn rune_kind(&self, id: SymbolId) -> Option<RuneKind> {
        self.runes.get(&id).copied()
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
        if let Some(&kind) = self.runes.get(&sym_id) {
            if kind == RuneKind::State && !self.is_mutated(sym_id) {
                return false;
            }
            if matches!(kind, RuneKind::Derived | RuneKind::DerivedBy) {
                if let Some(deps) = self.derived_deps.get(&sym_id) {
                    return deps.iter().any(|&dep| self.is_dynamic_by_id_inner(dep, depth + 1));
                }
                return false;
            }
            return true;
        }
        // Non-root-scope binding (each block context/index) is always dynamic
        self.symbol_scope_id(sym_id) != self.root_scope_id()
    }

    // -- Bulk queries for codegen compatibility --

    /// All rune symbol names with their kinds.
    pub(crate) fn rune_names(&self) -> FxHashMap<String, RuneKind> {
        self.runes
            .iter()
            .map(|(&id, &kind)| (self.scoping.symbol_name(id).to_string(), kind))
            .collect()
    }

    /// Names of all mutated runes (script + template, all via OXC references).
    pub(crate) fn mutated_rune_names(&self) -> rustc_hash::FxHashSet<String> {
        self.runes
            .keys()
            .filter(|&&id| self.is_mutated(id))
            .map(|&id| self.scoping.symbol_name(id).to_string())
            .collect()
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
                    if matches!(rune_kind, RuneKind::Derived | RuneKind::DerivedBy)
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

    // Walk template to add each-block scopes
    let root = data.scoping.root_scope_id();
    walk_template_scopes(&component.fragment, component, &mut data.scoping, root);
}

fn walk_template_scopes(
    fragment: &Fragment,
    component: &Component,
    scoping: &mut ComponentScoping,
    current_scope: ScopeId,
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

                walk_template_scopes(&block.body, component, scoping, child_scope);

                // Fallback uses parent scope (no context/index vars)
                if let Some(fb) = &block.fallback {
                    walk_template_scopes(fb, component, scoping, current_scope);
                }
            }
            Node::Element(el) => {
                walk_template_scopes(&el.fragment, component, scoping, current_scope);
            }
            Node::ComponentNode(cn) => {
                walk_template_scopes(&cn.fragment, component, scoping, current_scope);
            }
            Node::IfBlock(block) => {
                walk_template_scopes(&block.consequent, component, scoping, current_scope);
                if let Some(alt) = &block.alternate {
                    walk_template_scopes(alt, component, scoping, current_scope);
                }
            }
            Node::SnippetBlock(block) => {
                walk_template_scopes(&block.body, component, scoping, current_scope);
            }
            Node::KeyBlock(block) => {
                walk_template_scopes(&block.fragment, component, scoping, current_scope);
            }
            Node::ExpressionTag(_) | Node::Text(_) | Node::Comment(_) | Node::RenderTag(_) | Node::HtmlTag(_) | Node::Error(_) => {}
        }
    }
}
