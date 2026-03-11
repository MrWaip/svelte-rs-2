use std::collections::{HashMap, HashSet};

use oxc_semantic::{
    NodeId as OxcNodeId, Reference, ReferenceFlags as OxcRefFlags, ScopeFlags, ScopeId, Scoping,
    SymbolFlags, SymbolId,
};

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
    runes: HashMap<SymbolId, RuneKind>,
    /// Which symbols are mutated via bind directives.
    bind_mutated: HashSet<SymbolId>,
    /// Our AST NodeId → OXC ScopeId for scope-introducing nodes (each blocks).
    node_scopes: HashMap<NodeId, ScopeId>,
}

impl ComponentScoping {
    /// Create from a pre-built OXC Scoping (produced by `analyze_script_with_scoping`).
    pub fn from_scoping(scoping: Scoping) -> Self {
        Self {
            scoping,
            runes: HashMap::new(),
            bind_mutated: HashSet::new(),
            node_scopes: HashMap::new(),
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

    pub fn symbol_name(&self, id: SymbolId) -> &str {
        self.scoping.symbol_name(id)
    }

    pub fn symbol_is_mutated(&self, id: SymbolId) -> bool {
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

    pub fn rune_kind(&self, id: SymbolId) -> Option<RuneKind> {
        self.runes.get(&id).copied()
    }

    pub fn is_rune(&self, id: SymbolId) -> bool {
        self.runes.contains_key(&id)
    }

    pub fn mark_bind_mutated(&mut self, id: SymbolId) {
        self.bind_mutated.insert(id);
    }

    /// Add a write reference to a symbol so `symbol_is_mutated()` returns true.
    pub fn mark_symbol_mutated(&mut self, sym_id: SymbolId) {
        let scope = self.scoping.symbol_scope_id(sym_id);
        let mut reference = Reference::new(OxcNodeId::DUMMY, scope, OxcRefFlags::Write);
        reference.set_symbol_id(sym_id);
        let ref_id = self.scoping.create_reference(reference);
        self.scoping.add_resolved_reference(sym_id, ref_id);
    }

    // -- Convenience: scope-aware resolution --

    /// Check if a reference is dynamic in the given scope.
    /// A reference is dynamic if it resolves to a rune or a non-root-scope binding
    /// (e.g., each-block variable).
    pub fn is_dynamic_ref(&self, scope: ScopeId, name: &str) -> bool {
        let Some(sym_id) = self.find_binding(scope, name) else {
            return false;
        };
        if self.runes.contains_key(&sym_id) {
            return true;
        }
        // Non-root-scope binding (each block context/index) is always dynamic
        self.symbol_scope_id(sym_id) != self.root_scope_id()
    }

    // -- Bulk queries for codegen compatibility --

    /// All rune symbol names.
    pub fn rune_names(&self) -> HashSet<String> {
        self.runes
            .keys()
            .map(|&id| self.scoping.symbol_name(id).to_string())
            .collect()
    }

    /// Names of all mutated runes (script assignments via OXC semantic + bind mutations).
    pub fn mutated_rune_names(&self) -> HashSet<String> {
        self.runes
            .keys()
            .filter(|&&id| self.scoping.symbol_is_mutated(id) || self.bind_mutated.contains(&id))
            .map(|&id| self.scoping.symbol_name(id).to_string())
            .collect()
    }

    /// Names of runes mutated only via bind directives.
    pub fn bind_mutated_rune_names(&self) -> HashSet<String> {
        self.bind_mutated
            .iter()
            .filter(|id| self.runes.contains_key(id))
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
            Node::IfBlock(block) => {
                walk_template_scopes(&block.consequent, component, scoping, current_scope);
                if let Some(alt) = &block.alternate {
                    walk_template_scopes(alt, component, scoping, current_scope);
                }
            }
            Node::ExpressionTag(_) | Node::Text(_) | Node::Comment(_) => {}
        }
    }
}
