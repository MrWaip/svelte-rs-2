use rustc_hash::{FxHashMap, FxHashSet};

pub use oxc_semantic::{ScopeId, SymbolId};
use oxc_semantic::{NodeId as OxcNodeId, Reference as OxcReference, ReferenceFlags as OxcReferenceFlags, ScopeFlags, Scoping, SymbolFlags};

use svelte_ast::{Attribute, Component, Fragment, Node, NodeId};
use svelte_parser::RuneKind;

use crate::data::{AnalysisData, EachBlockData, FragmentKey};

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
    snippet_name_syms: FxHashSet<SymbolId>,
    each_block_syms: FxHashSet<SymbolId>,
    /// Each-block vars that do NOT need `$.get()` wrapping (key_is_item optimization).
    each_non_reactive_syms: FxHashSet<SymbolId>,
    /// FragmentKey → ScopeId for scope-introducing fragments (IfBlock branches, KeyBlock, etc.)
    fragment_scopes: FxHashMap<FragmentKey, ScopeId>,
    /// SymbolId → parent ConstTag NodeId for destructured const bindings
    const_alias_tags: FxHashMap<SymbolId, NodeId>,
    /// Arrow function span.start → child ScopeId (for arrow params inside JS expressions)
    arrow_scopes: FxHashMap<u32, ScopeId>,
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
            snippet_name_syms: FxHashSet::default(),
            each_block_syms: FxHashSet::default(),
            each_non_reactive_syms: FxHashSet::default(),
            fragment_scopes: FxHashMap::default(),
            const_alias_tags: FxHashMap::default(),
            arrow_scopes: FxHashMap::default(),
        }
    }

    /// Create an empty scoping (no script block).
    pub fn empty() -> Self {
        let alloc = oxc_allocator::Allocator::default();
        let result = oxc_parser::Parser::new(&alloc, "", oxc_span::SourceType::default()).parse();
        let sem = oxc_semantic::SemanticBuilder::new().build(&result.program);
        Self::from_scoping(sem.semantic.into_scoping())
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

    /// Get the declared name of a symbol.
    pub fn symbol_name(&self, id: SymbolId) -> &str {
        self.scoping.symbol_name(id)
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
            && !self.is_snippet_param(sym_id)
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

    pub fn arrow_scope(&self, span_start: u32) -> Option<ScopeId> {
        self.arrow_scopes.get(&span_start).copied()
    }

    /// True if this symbol was declared as an arrow function parameter.
    pub fn is_arrow_param(&self, sym_id: SymbolId) -> bool {
        let scope = self.symbol_scope_id(sym_id);
        self.arrow_scopes.values().any(|&s| s == scope)
    }

    pub fn register_arrow_scope(&mut self, span_start: u32, parent: ScopeId, param_names: &[String]) -> ScopeId {
        let scope = self.add_child_scope(parent);
        for name in param_names {
            self.add_binding(scope, name);
        }
        self.arrow_scopes.insert(span_start, scope);
        scope
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

// ---------------------------------------------------------------------------
// Build scoping pass
// ---------------------------------------------------------------------------

/// Build the unified ComponentScoping from script + template.
///
/// The `Scoping` is already initialized by `parse_js` (via `analyze_script_with_scoping`),
/// so this pass only marks runes and walks the template to add each-block scopes.
pub(crate) fn build_scoping(component: &Component, data: &mut AnalysisData) -> crate::markers::ScopingBuilt {
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
    let mut snippet_params = std::mem::take(&mut data.snippets.params);
    walk_template_scopes(&component.fragment, component, &mut data.scoping, root, &const_tag_names, &mut snippet_params, &mut data.each_blocks);
    data.const_tags.names = const_tag_names;
    data.snippets.params = snippet_params;
    crate::markers::ScopingBuilt::new()
}

fn walk_template_scopes(
    fragment: &Fragment,
    component: &Component,
    scoping: &mut ComponentScoping,
    current_scope: ScopeId,
    const_tag_names: &FxHashMap<NodeId, Vec<String>>,
    snippet_params: &mut FxHashMap<NodeId, Vec<String>>,
    each_blocks: &mut EachBlockData,
) {
    for node in &fragment.nodes {
        match node {
            Node::EachBlock(block) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_node_scope(block.id, child_scope);

                let context_text = component.source_text(block.context_span);
                let is_destructured = context_text.starts_with('{') || context_text.starts_with('[');

                if is_destructured {
                    // Destructured context: use $$item as internal name
                    let ctx_sym = scoping.add_binding(child_scope, "$$item");
                    scoping.mark_each_block_var(ctx_sym);
                    each_blocks.context_names.insert(block.id, "$$item".to_string());
                    each_blocks.is_destructured.insert(block.id);

                    // Extract binding names from pattern and register in scope
                    let bindings = extract_destructuring_bindings(context_text);
                    for (name, has_default) in &bindings {
                        let sym = scoping.add_binding(child_scope, name);
                        if *has_default {
                            // With default → $.derived_safe_equal → $.get() read
                            // Leave unmarked (non-root falls through to $.get() in transform)
                        } else {
                            // Without default → thunk → name() call
                            scoping.mark_snippet_param(sym);
                        }
                    }
                } else {
                    let ctx_sym = scoping.add_binding(child_scope, context_text);
                    scoping.mark_each_block_var(ctx_sym);
                    each_blocks.context_names.insert(block.id, context_text.to_string());

                    // key_is_item optimization: context var doesn't need $.get()
                    if let Some(key_span) = block.key_span {
                        let key_text = component.source_text(key_span).trim();
                        let ctx_trimmed = context_text.trim();
                        if key_text == ctx_trimmed && is_simple_identifier(key_text) {
                            scoping.mark_each_non_reactive(ctx_sym);
                            each_blocks.key_is_item.insert(block.id);
                        }
                    }
                }

                // Add optional index variable binding and store name in side table
                if let Some(idx_span) = block.index_span {
                    let idx_name = component.source_text(idx_span);
                    let idx_sym = scoping.add_binding(child_scope, idx_name);
                    scoping.mark_each_block_var(idx_sym);
                    each_blocks.index_names.insert(block.id, idx_name.to_string());
                }

                // Detect animate: directive on direct child elements
                if block.body.nodes.iter().any(|node| {
                    if let Node::Element(el) = node {
                        el.attributes.iter().any(|a| matches!(a, Attribute::AnimateDirective(_)))
                    } else {
                        false
                    }
                }) {
                    each_blocks.has_animate.insert(block.id);
                }

                walk_template_scopes(&block.body, component, scoping, child_scope, const_tag_names, snippet_params, each_blocks);

                // Fallback uses parent scope (no context/index vars)
                if let Some(fb) = &block.fallback {
                    walk_template_scopes(fb, component, scoping, current_scope, const_tag_names, snippet_params, each_blocks);
                }
            }
            Node::Element(el) => {
                walk_template_scopes(&el.fragment, component, scoping, current_scope, const_tag_names, snippet_params, each_blocks);
            }
            Node::ComponentNode(cn) => {
                walk_template_scopes(&cn.fragment, component, scoping, current_scope, const_tag_names, snippet_params, each_blocks);
            }
            Node::IfBlock(block) => {
                let cons_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::IfConsequent(block.id), cons_scope);
                walk_template_scopes(&block.consequent, component, scoping, cons_scope, const_tag_names, snippet_params, each_blocks);
                if let Some(alt) = &block.alternate {
                    let alt_scope = scoping.add_child_scope(current_scope);
                    scoping.set_fragment_scope(FragmentKey::IfAlternate(block.id), alt_scope);
                    walk_template_scopes(alt, component, scoping, alt_scope, const_tag_names, snippet_params, each_blocks);
                }
            }
            Node::SnippetBlock(block) => {
                // Snippet name is a normal binding in the parent scope
                let name_sym = scoping.add_binding(current_scope, &block.name);
                scoping.mark_snippet_name(name_sym);
                // Create a child scope for snippet params — they shadow outer bindings.
                let snippet_scope = scoping.add_child_scope(current_scope);
                scoping.set_node_scope(block.id, snippet_scope);
                // Read pre-computed params (populated by parser)
                if let Some(params) = snippet_params.get(&block.id) {
                    for param in params {
                        let sym_id = scoping.add_binding(snippet_scope, param);
                        scoping.mark_snippet_param(sym_id);
                    }
                }
                walk_template_scopes(&block.body, component, scoping, snippet_scope, const_tag_names, snippet_params, each_blocks);
            }
            Node::KeyBlock(block) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::KeyBlockBody(block.id), child_scope);
                walk_template_scopes(&block.fragment, component, scoping, child_scope, const_tag_names, snippet_params, each_blocks);
            }
            Node::SvelteHead(head) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::SvelteHeadBody(head.id), child_scope);
                walk_template_scopes(&head.fragment, component, scoping, child_scope, const_tag_names, snippet_params, each_blocks);
            }
            Node::SvelteElement(el) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::SvelteElementBody(el.id), child_scope);
                walk_template_scopes(&el.fragment, component, scoping, child_scope, const_tag_names, snippet_params, each_blocks);
            }
            Node::SvelteBoundary(b) => {
                let child_scope = scoping.add_child_scope(current_scope);
                scoping.set_fragment_scope(FragmentKey::SvelteBoundaryBody(b.id), child_scope);
                walk_template_scopes(&b.fragment, component, scoping, child_scope, const_tag_names, snippet_params, each_blocks);
            }
            Node::ConstTag(tag) => {
                if let Some(names) = const_tag_names.get(&tag.id) {
                    let is_destructured = names.len() > 1;
                    for name in names {
                        let sym_id = scoping.add_binding(current_scope, name);
                        scoping.mark_rune(sym_id, RuneKind::Derived);
                        if is_destructured {
                            scoping.mark_const_alias(sym_id, tag.id);
                        }
                    }
                }
            }
            Node::AwaitBlock(block) => {
                // Pending fragment: child scope for const alias isolation
                if let Some(ref p) = block.pending {
                    let pending_scope = scoping.add_child_scope(current_scope);
                    scoping.set_fragment_scope(FragmentKey::AwaitPending(block.id), pending_scope);
                    walk_template_scopes(p, component, scoping, pending_scope, const_tag_names, snippet_params, each_blocks);
                }

                // Then fragment: create child scope if value binding exists
                if let Some(ref t) = block.then {
                    if let Some(val_span) = block.value_span {
                        let then_scope = scoping.add_child_scope(current_scope);
                        scoping.set_node_scope(block.id, then_scope);
                        let binding_text = component.source_text(val_span);
                        for name in svelte_parser::parse_await_binding(binding_text).names() {
                            scoping.add_binding(then_scope, name);
                        }
                        walk_template_scopes(t, component, scoping, then_scope, const_tag_names, snippet_params, each_blocks);
                    } else {
                        walk_template_scopes(t, component, scoping, current_scope, const_tag_names, snippet_params, each_blocks);
                    }
                }

                // Catch fragment: create child scope if error binding exists
                if let Some(ref c) = block.catch {
                    if let Some(err_span) = block.error_span {
                        let catch_scope = scoping.add_child_scope(current_scope);
                        scoping.set_await_catch_scope(block.id, catch_scope);
                        let binding_text = component.source_text(err_span);
                        for name in svelte_parser::parse_await_binding(binding_text).names() {
                            scoping.add_binding(catch_scope, name);
                        }
                        walk_template_scopes(c, component, scoping, catch_scope, const_tag_names, snippet_params, each_blocks);
                    } else {
                        walk_template_scopes(c, component, scoping, current_scope, const_tag_names, snippet_params, each_blocks);
                    }
                }
            }
            Node::SvelteWindow(_) | Node::SvelteDocument(_) | Node::SvelteBody(_) => {}
            Node::ExpressionTag(_) | Node::Text(_) | Node::Comment(_) | Node::RenderTag(_) | Node::HtmlTag(_) | Node::DebugTag(_) | Node::Error(_) => {}
        }
    }
}

fn is_simple_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

/// Extract binding names and default status from a destructuring pattern.
/// Handles `{ a, b = 5 }` and `[a, b]` patterns.
pub(crate) fn extract_destructuring_bindings(pattern: &str) -> Vec<(String, bool)> {
    let s = pattern.trim();
    let inner = if s.starts_with('{') && s.ends_with('}') {
        &s[1..s.len() - 1]
    } else if s.starts_with('[') && s.ends_with(']') {
        &s[1..s.len() - 1]
    } else {
        return vec![];
    };

    let mut result = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();

    for ch in inner.chars() {
        match ch {
            '{' | '[' | '(' => { depth += 1; current.push(ch); }
            '}' | ']' | ')' => { depth -= 1; current.push(ch); }
            ',' if depth == 0 => {
                process_destructured_binding(&current, &mut result);
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        process_destructured_binding(&current, &mut result);
    }

    result
}

fn process_destructured_binding(part: &str, result: &mut Vec<(String, bool)>) {
    let part = part.trim();
    if part.is_empty() { return; }

    // Find first '=' at depth 0
    let mut depth = 0i32;
    let mut eq_pos = None;
    for (i, ch) in part.char_indices() {
        match ch {
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth -= 1,
            '=' if depth == 0 => { eq_pos = Some(i); break; }
            _ => {}
        }
    }

    let (name_part, has_default) = if let Some(pos) = eq_pos {
        (&part[..pos], true)
    } else {
        (part, false)
    };

    let name_part = name_part.trim();
    // Handle rest (...)
    let name_part = name_part.strip_prefix("...").unwrap_or(name_part);
    // Handle rename (prop: alias)
    let name_part = if let Some(idx) = name_part.rfind(':') {
        name_part[idx + 1..].trim()
    } else {
        name_part
    };

    if is_simple_identifier(name_part) {
        result.push((name_part.to_string(), has_default));
    }
}
