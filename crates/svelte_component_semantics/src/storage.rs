use compact_str::CompactString;
use oxc_ast::{AstKind, ast::IdentifierReference};
use oxc_span::{GetSpan, Span};
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::scope::{ScopeFlags, ScopeId};
use oxc_syntax::symbol::{SymbolFlags, SymbolId};
use rustc_hash::{FxHashMap, FxHashSet};
use std::marker::PhantomData;
use svelte_ast::FragmentKey;

use crate::symbol::SymbolOwner;

use crate::reference::{Reference, ReferenceTable};
use crate::scope::ScopeTable;
use crate::symbol::SymbolTable;

/// Generic JS AST storage keyed by the component-wide remapped `OxcNodeId`.
///
/// This stays generic on purpose: it mirrors JS node identity and parentage
/// without carrying any Svelte-specific meaning.
pub struct JsStorage<'a> {
    nodes: Vec<Option<JsNode<'a>>>,
    parent_ids: Vec<Option<OxcNodeId>>,
}

/// JS node metadata stored alongside `ComponentSemantics`.
///
/// The canonical node identity is the remapped component `OxcNodeId` used as
/// the lookup key in `JsStorage`; the embedded `AstKind` still points at the
/// original OXC AST node.
#[derive(Clone, Copy)]
pub struct JsNode<'a> {
    kind: AstKind<'a>,
    scope_id: ScopeId,
}

impl<'a> JsStorage<'a> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            parent_ids: Vec::new(),
        }
    }

    pub fn node(&self, id: OxcNodeId) -> Option<&JsNode<'a>> {
        self.nodes.get(id.index()).and_then(Option::as_ref)
    }

    pub fn parent_id(&self, id: OxcNodeId) -> Option<OxcNodeId> {
        self.parent_ids.get(id.index()).copied().flatten()
    }

    pub fn kind(&self, id: OxcNodeId) -> Option<AstKind<'a>> {
        self.node(id).map(JsNode::kind)
    }

    pub(crate) fn record_node(
        &mut self,
        id: OxcNodeId,
        kind: AstKind<'a>,
        scope_id: ScopeId,
        parent_id: Option<OxcNodeId>,
    ) {
        let index = id.index();
        if self.nodes.len() <= index {
            self.nodes.resize_with(index + 1, || None);
            self.parent_ids.resize(index + 1, None);
        }

        if let Some(existing) = self.nodes[index] {
            // The component semantic builder can legitimately re-enter the same
            // OXC node through custom visitor paths, but it must resolve to the
            // exact same canonical component node slot every time.
            debug_assert!(
                std::mem::discriminant(&existing.kind()) == std::mem::discriminant(&kind)
                    && existing.kind().span() == kind.span()
                    && existing.scope_id() == scope_id
                    && self.parent_ids[index] == parent_id,
                "component node ids should map to a single canonical JS node: id={:?}, existing_kind={:?}, new_kind={:?}, existing_span={:?}, new_span={:?}, existing_scope={:?}, new_scope={:?}, existing_parent={:?}, new_parent={:?}",
                id,
                existing.kind(),
                kind,
                existing.kind().span(),
                kind.span(),
                existing.scope_id(),
                scope_id,
                self.parent_ids[index],
                parent_id
            );
            return;
        }

        self.nodes[index] = Some(JsNode { kind, scope_id });
        self.parent_ids[index] = parent_id;
    }
}

impl Default for JsStorage<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> JsNode<'a> {
    pub fn kind(&self) -> AstKind<'a> {
        self.kind
    }

    pub fn scope_id(&self) -> ScopeId {
        self.scope_id
    }
}

/// Component-wide semantic graph — the single source of truth for scopes,
/// symbols, and references across module script, instance script, and template.
pub struct ComponentSemantics<'a> {
    pub(crate) scopes: ScopeTable,
    pub(crate) symbols: SymbolTable,
    pub(crate) references: ReferenceTable,
    pub(crate) js: JsStorage<'a>,
    /// References created from template expressions (not from script AST).
    template_reference_ids: FxHashSet<ReferenceId>,
    /// Unresolved references at root scope, keyed by name.
    root_unresolved_references: FxHashMap<CompactString, Vec<ReferenceId>>,
    /// Maps template fragment keys to their scopes.
    fragment_scopes: FxHashMap<FragmentKey, ScopeId>,
    /// O(1) membership check for template-introduced scopes.
    /// Built by `build_template_scope_set()` after all template scopes are registered.
    template_scope_set: FxHashSet<ScopeId>,
    /// The module script root scope, if present.
    module_scope_id: Option<ScopeId>,
    /// The instance script root scope, if present.
    instance_scope_id: Option<ScopeId>,
    ast_lifetime: PhantomData<&'a ()>,
}

impl<'a> ComponentSemantics<'a> {
    /// Create an empty semantic graph with a single root scope.
    pub fn new() -> Self {
        let mut scopes = ScopeTable::new();
        scopes.add_scope(None, ScopeFlags::Top | ScopeFlags::Function);
        Self {
            scopes,
            symbols: SymbolTable::new(),
            references: ReferenceTable::new(),
            js: JsStorage::new(),
            template_reference_ids: FxHashSet::default(),
            root_unresolved_references: FxHashMap::default(),
            fragment_scopes: FxHashMap::default(),
            template_scope_set: FxHashSet::default(),
            module_scope_id: None,
            instance_scope_id: None,
            ast_lifetime: PhantomData,
        }
    }

    // -- Scope queries --

    pub fn root_scope_id(&self) -> ScopeId {
        ScopeId::new(0)
    }

    pub fn module_scope_id(&self) -> Option<ScopeId> {
        self.module_scope_id
    }

    pub fn instance_scope_id(&self) -> Option<ScopeId> {
        self.instance_scope_id
    }

    pub fn scope_parent_id(&self, id: ScopeId) -> Option<ScopeId> {
        self.scopes.scope_parent_id(id)
    }

    pub fn scope_flags(&self, id: ScopeId) -> ScopeFlags {
        self.scopes.scope_flags(id)
    }

    pub fn add_scope(&mut self, parent: ScopeId, flags: ScopeFlags) -> ScopeId {
        self.scopes.add_scope(Some(parent), flags)
    }

    pub fn add_child_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scopes.add_scope(Some(parent), ScopeFlags::empty())
    }

    // -- Symbol queries --

    pub fn find_binding(&self, scope: ScopeId, name: &str) -> Option<SymbolId> {
        self.scopes.find_binding(scope, name)
    }

    pub fn get_binding(&self, scope: ScopeId, name: &str) -> Option<SymbolId> {
        self.scopes.get_binding(scope, name)
    }

    pub fn own_binding_names(&self, scope: ScopeId) -> impl Iterator<Item = &str> {
        self.scopes.own_binding_names(scope)
    }

    pub fn add_binding(
        &mut self,
        scope: ScopeId,
        name: &str,
        span: Span,
        flags: SymbolFlags,
        node_id: OxcNodeId,
        owner: SymbolOwner,
    ) -> SymbolId {
        let compact_name = CompactString::from(name);
        let sym =
            self.symbols
                .create_symbol(compact_name.clone(), span, flags, scope, node_id, owner);
        self.scopes.add_binding(scope, compact_name, sym);
        sym
    }

    pub fn symbol_name(&self, id: SymbolId) -> &str {
        self.symbols.symbol_name(id)
    }

    pub fn symbol_span(&self, id: SymbolId) -> Span {
        self.symbols.symbol_span(id)
    }

    pub fn symbol_flags(&self, id: SymbolId) -> SymbolFlags {
        self.symbols.symbol_flags(id)
    }

    pub fn symbol_scope_id(&self, id: SymbolId) -> ScopeId {
        self.symbols.symbol_scope_id(id)
    }

    pub fn symbol_declaration(&self, id: SymbolId) -> oxc_syntax::node::NodeId {
        self.symbols.symbol_declaration(id)
    }

    pub fn is_mutated(&self, id: SymbolId) -> bool {
        self.symbols.is_mutated(id)
    }

    /// Check if a symbol has a specific state bit set.
    pub fn has_symbol_state(&self, id: SymbolId, bit: u32) -> bool {
        self.symbols.has_state(id, bit)
    }

    /// Set a state bit on a symbol (bitwise OR).
    pub fn set_symbol_state(&mut self, id: SymbolId, bit: u32) {
        self.symbols.set_state(id, bit);
    }

    /// Iterate symbols that have a given state bit set.
    pub fn symbols_with_state(&self, bit: u32) -> impl Iterator<Item = SymbolId> + '_ {
        self.symbols
            .symbol_ids()
            .filter(move |&id| self.symbols.has_state(id, bit))
    }

    /// True if the symbol has any write reference other than `exclude_ref_id`.
    pub fn has_write_reference_other_than(
        &self,
        sym_id: SymbolId,
        exclude_ref_id: ReferenceId,
    ) -> bool {
        self.get_resolved_reference_ids(sym_id)
            .iter()
            .copied()
            .any(|ref_id| ref_id != exclude_ref_id && self.references.get(ref_id).is_write())
    }

    // -- Reference queries --

    pub fn create_reference(&mut self, reference: Reference) -> ReferenceId {
        self.references.create_reference(reference)
    }

    pub fn create_template_reference(&mut self, reference: Reference) -> ReferenceId {
        let id = self.references.create_reference(reference);
        self.template_reference_ids.insert(id);
        id
    }

    pub fn get_reference(&self, id: ReferenceId) -> &Reference {
        self.references.get(id)
    }

    pub fn try_get_reference(&self, id: ReferenceId) -> Option<&Reference> {
        if id.index() < self.references.len() {
            Some(self.references.get(id))
        } else {
            None
        }
    }

    pub fn references_len(&self) -> usize {
        self.references.len()
    }

    pub fn get_reference_mut(&mut self, id: ReferenceId) -> &mut Reference {
        self.references.get_mut(id)
    }

    pub fn symbol_for_reference(&self, id: ReferenceId) -> Option<SymbolId> {
        self.get_reference(id).symbol_id()
    }

    pub fn symbol_for_identifier_reference(
        &self,
        id: &IdentifierReference<'a>,
    ) -> Option<SymbolId> {
        id.reference_id
            .get()
            .and_then(|ref_id| self.symbol_for_reference(ref_id))
    }

    pub fn is_template_reference(&self, id: ReferenceId) -> bool {
        self.template_reference_ids.contains(&id)
    }

    /// Link a reference to a symbol. Sets the symbol's mutated flag if write.
    pub fn add_resolved_reference(&mut self, symbol_id: SymbolId, reference_id: ReferenceId) {
        let is_write = self.references.get(reference_id).is_write();
        self.symbols
            .add_resolved_reference(symbol_id, reference_id, is_write);
    }

    /// Force-mark a symbol as member-mutated.
    ///
    /// Template sites (e.g. `bind:value={foo.bar}`) express mutation of a
    /// symbol through a member chain — OXC reads `foo` as an object-side
    /// identifier but the template still writes via the member. Mirrors the
    /// reference compiler behaviour (`phases/scope.js::BindDirective` updates
    /// → `binding.mutated = true` for the root identifier of a member-
    /// expression bind target, kept separate from `reassigned`).
    ///
    /// Does NOT set the plain `MUTATED` flag — consumers that care about
    /// reassignment specifically (`$state`-wrapping, for example) stay
    /// unaffected; the generic "any mutation" query is `is_mutated_any`.
    pub fn mark_symbol_member_mutated(&mut self, symbol_id: SymbolId) {
        self.symbols
            .set_state(symbol_id, crate::symbol::state::MEMBER_MUTATED);
    }

    pub fn is_member_mutated(&self, id: SymbolId) -> bool {
        self.symbols.has_state(id, crate::symbol::state::MEMBER_MUTATED)
    }

    /// True when `symbol` has either been reassigned (a write reference) or
    /// member-mutated via a template site. Equivalent to
    /// reference-compiler's `binding.updated`.
    pub fn is_mutated_any(&self, id: SymbolId) -> bool {
        self.is_mutated(id) || self.is_member_mutated(id)
    }

    pub fn get_resolved_reference_ids(&self, id: SymbolId) -> &[ReferenceId] {
        self.symbols.get_resolved_reference_ids(id)
    }

    // -- JS node queries --

    pub fn js_storage(&self) -> &JsStorage<'a> {
        &self.js
    }

    pub fn js_node(&self, id: OxcNodeId) -> Option<&JsNode<'a>> {
        self.js.node(id)
    }

    pub fn js_parent_id(&self, id: OxcNodeId) -> Option<OxcNodeId> {
        self.js.parent_id(id)
    }

    pub fn js_kind(&self, id: OxcNodeId) -> Option<AstKind<'a>> {
        self.js.kind(id)
    }

    /// Static object-pattern key that introduced this binding, if any.
    ///
    /// Examples:
    /// - `localFoo` in `let { foo: localFoo } = obj` -> `Some("foo")`
    /// - `bar` in `let { bar } = obj` -> `Some("bar")`
    /// - `rest` in `let { ...rest } = obj` -> `None`
    /// - `x` in `let x = 1` -> `None`
    pub fn binding_origin_key(&self, sym_id: SymbolId) -> Option<&str> {
        let mut node_id = self.symbol_declaration(sym_id);
        loop {
            let parent_id = match self.js_parent_id(node_id) {
                Some(parent_id) => parent_id,
                None => {
                    debug_assert!(
                        false,
                        "binding_origin_key missing js parent: sym={:?}, name={}, decl_node={:?}, decl_kind={:?}",
                        sym_id,
                        self.symbol_name(sym_id),
                        node_id,
                        self.js_kind(node_id)
                    );
                    return None;
                }
            };
            match self.js_kind(parent_id)? {
                AstKind::AssignmentPattern(_) => {
                    node_id = parent_id;
                }
                AstKind::BindingProperty(prop) => {
                    if prop.computed {
                        return None;
                    }
                    return match prop.key.static_name()? {
                        std::borrow::Cow::Borrowed(name) => Some(name),
                        std::borrow::Cow::Owned(_) => None,
                    };
                }
                AstKind::BindingRestElement(_) => return None,
                other => {
                    debug_assert!(
                        false,
                        "binding_origin_key stopped on unexpected parent: sym={:?}, name={}, decl_node={:?}, parent_id={:?}, parent_kind={:?}",
                        sym_id,
                        self.symbol_name(sym_id),
                        node_id,
                        parent_id,
                        other
                    );
                    return None;
                }
            }
        }
    }

    /// Static object-pattern key for the symbol referenced by `ref_id`, if any.
    pub fn binding_origin_key_for_reference(&self, ref_id: ReferenceId) -> Option<&str> {
        let sym_id = self.symbol_for_reference(ref_id)?;
        self.binding_origin_key(sym_id)
    }

    pub fn binding_origin_key_for_identifier_reference(
        &self,
        id: &IdentifierReference<'a>,
    ) -> Option<&str> {
        let sym_id = self.symbol_for_identifier_reference(id)?;
        self.binding_origin_key(sym_id)
    }

    // -- Scope hierarchy helpers --

    pub fn find_function_scope(&self, scope: ScopeId) -> ScopeId {
        self.scopes.find_function_scope(scope)
    }

    /// Count function scopes between `scope` and the root (inclusive).
    pub fn function_depth(&self, mut scope: ScopeId) -> u32 {
        let mut depth = 0;
        loop {
            if self.scopes.scope_flags(scope).is_function() {
                depth += 1;
            }
            match self.scopes.scope_parent_id(scope) {
                Some(parent) => scope = parent,
                None => return depth,
            }
        }
    }

    pub fn is_component_top_level_scope(&self, scope: ScopeId) -> bool {
        scope == self.root_scope_id() || self.module_scope_id.is_some_and(|m| scope == m)
    }

    pub fn is_component_top_level_symbol(&self, sym: SymbolId) -> bool {
        self.is_component_top_level_scope(self.symbols.symbol_scope_id(sym))
    }

    pub fn symbol_owner(&self, id: SymbolId) -> SymbolOwner {
        self.symbols.symbol_owner(id)
    }

    /// True if this symbol was declared inside a JS construct in a template
    /// expression (arrow/function param, for-loop var, block-scoped var, catch param).
    /// Not top-level, and not a template-introduced scope (each/snippet/etc.).
    pub fn is_expr_local(&self, sym: SymbolId) -> bool {
        let scope = self.symbols.symbol_scope_id(sym);
        !self.is_component_top_level_scope(scope) && !self.template_scope_set.contains(&scope)
    }

    // -- Iterators --

    /// Iterate all symbol IDs.
    pub fn symbol_ids(&self) -> impl Iterator<Item = SymbolId> {
        self.symbols.symbol_ids()
    }

    /// Iterate all symbol names (parallel with symbol_ids).
    pub fn symbol_names(&self) -> impl Iterator<Item = &str> {
        self.symbols.symbol_names()
    }

    /// Collect all import-flagged SymbolIds.
    pub fn collect_import_syms(&self) -> FxHashSet<SymbolId> {
        self.symbols
            .symbol_ids()
            .filter(|&id| self.symbols.symbol_flags(id).contains(SymbolFlags::Import))
            .collect()
    }

    /// O(1) lookup: find any symbol with this name regardless of scope.
    pub fn find_symbol_by_name(&self, name: &str) -> Option<SymbolId> {
        self.symbols.find_by_name(name)
    }

    /// Collect all symbol names (for IdentGen conflict detection).
    pub fn collect_all_symbol_names(&self) -> FxHashSet<CompactString> {
        self.symbols
            .symbol_names()
            .map(|s| CompactString::from(s))
            .collect()
    }

    /// Collect names declared in component top-level scopes only.
    pub fn collect_component_top_level_symbol_names(&self) -> FxHashSet<CompactString> {
        self.symbol_ids()
            .filter(|&sym| self.is_component_top_level_symbol(sym))
            .map(|sym| CompactString::from(self.symbol_name(sym)))
            .collect()
    }

    // -- Fragment scopes --

    /// Register a scope for a template fragment key.
    pub fn set_fragment_scope(&mut self, key: FragmentKey, scope: ScopeId) {
        self.fragment_scopes.insert(key, scope);
    }

    /// Look up the scope for a template fragment key.
    pub fn fragment_scope(&self, key: &FragmentKey) -> Option<ScopeId> {
        self.fragment_scopes.get(key).copied()
    }

    /// Build the template_scope_set from fragment_scopes for O(1) `is_expr_local`.
    /// Call after all template scopes are registered.
    pub fn build_template_scope_set(&mut self) {
        self.template_scope_set = self.fragment_scopes.values().copied().collect();
    }

    /// Check if a scope is a template-introduced scope (from fragment_scopes).
    pub fn is_template_scope(&self, scope: ScopeId) -> bool {
        self.template_scope_set.contains(&scope)
    }

    // -- Unresolved references --

    /// Record an unresolved reference (name not found in any scope).
    pub(crate) fn add_root_unresolved_reference(
        &mut self,
        name: CompactString,
        reference_id: ReferenceId,
    ) {
        self.root_unresolved_references
            .entry(name)
            .or_default()
            .push(reference_id);
    }

    /// Get all unresolved references at root scope.
    pub fn root_unresolved_references(&self) -> &FxHashMap<CompactString, Vec<ReferenceId>> {
        &self.root_unresolved_references
    }

    // -- Builder helpers (pub(crate) for builder module) --

    pub(crate) fn set_module_scope_id(&mut self, id: ScopeId) {
        self.module_scope_id = Some(id);
    }

    pub(crate) fn set_instance_scope_id(&mut self, id: ScopeId) {
        self.instance_scope_id = Some(id);
    }

    pub(crate) fn set_scope_parent_id(&mut self, scope: ScopeId, parent: Option<ScopeId>) {
        self.scopes.set_scope_parent_id(scope, parent);
    }
    pub(crate) fn record_js_node(
        &mut self,
        id: OxcNodeId,
        kind: AstKind<'a>,
        scope_id: ScopeId,
        parent_id: Option<OxcNodeId>,
    ) {
        self.js.record_node(id, kind, scope_id, parent_id);
    }
}

impl Default for ComponentSemantics<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_syntax::reference::ReferenceFlags;

    #[test]
    fn new_has_root_scope() {
        let sem = ComponentSemantics::new();
        let root = sem.root_scope_id();
        assert_eq!(root, ScopeId::new(0));
        assert!(sem.scope_flags(root).is_function());
        assert_eq!(sem.scope_parent_id(root), None);
    }

    #[test]
    fn add_binding_and_find() {
        let mut sem = ComponentSemantics::new();
        let root = sem.root_scope_id();
        let sym = sem.add_binding(
            root,
            "count",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );

        assert_eq!(sem.symbol_name(sym), "count");
        assert_eq!(sem.find_binding(root, "count"), Some(sym));
        assert_eq!(sem.find_binding(root, "other"), None);
    }

    #[test]
    fn child_scope_resolves_parent_binding() {
        let mut sem = ComponentSemantics::new();
        let root = sem.root_scope_id();
        let sym = sem.add_binding(
            root,
            "x",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );
        let child = sem.add_child_scope(root);

        assert_eq!(sem.find_binding(child, "x"), Some(sym));
    }

    #[test]
    fn mutation_tracking_via_reference() {
        let mut sem = ComponentSemantics::new();
        let root = sem.root_scope_id();
        let sym = sem.add_binding(
            root,
            "x",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );

        // Read reference — not mutated
        let read_ref =
            sem.create_reference(Reference::new(OxcNodeId::DUMMY, root, ReferenceFlags::Read));
        sem.add_resolved_reference(sym, read_ref);
        assert!(!sem.is_mutated(sym));

        // Write reference — mutated
        let write_ref = sem.create_reference(Reference::new(
            OxcNodeId::DUMMY,
            root,
            ReferenceFlags::Write,
        ));
        sem.add_resolved_reference(sym, write_ref);
        assert!(sem.is_mutated(sym));
    }

    #[test]
    fn template_reference_tracking() {
        let mut sem = ComponentSemantics::new();
        let root = sem.root_scope_id();

        let script_ref =
            sem.create_reference(Reference::new(OxcNodeId::DUMMY, root, ReferenceFlags::Read));
        let template_ref = sem.create_template_reference(Reference::new(
            OxcNodeId::DUMMY,
            root,
            ReferenceFlags::Read,
        ));

        assert!(!sem.is_template_reference(script_ref));
        assert!(sem.is_template_reference(template_ref));
    }

    #[test]
    fn function_depth() {
        let mut sem = ComponentSemantics::new();
        let root = sem.root_scope_id();
        let block = sem.add_child_scope(root);
        let fn_scope = sem.add_scope(block, ScopeFlags::Function);
        let inner_block = sem.add_child_scope(fn_scope);

        // root (function) = 1
        assert_eq!(sem.function_depth(root), 1);
        // block under root — root is function, so 1
        assert_eq!(sem.function_depth(block), 1);
        // fn_scope (function) + root (function) = 2
        assert_eq!(sem.function_depth(fn_scope), 2);
        // inner_block → fn_scope → root = 2
        assert_eq!(sem.function_depth(inner_block), 2);
    }

    #[test]
    fn component_top_level() {
        let mut sem = ComponentSemantics::new();
        let root = sem.root_scope_id();
        let module = sem.add_scope(root, ScopeFlags::Top | ScopeFlags::Function);
        sem.set_module_scope_id(module);

        let sym_root = sem.add_binding(
            root,
            "a",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );
        let sym_module = sem.add_binding(
            module,
            "b",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );
        let child = sem.add_child_scope(root);
        let sym_nested = sem.add_binding(
            child,
            "c",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );

        assert!(sem.is_component_top_level_symbol(sym_root));
        assert!(sem.is_component_top_level_symbol(sym_module));
        assert!(!sem.is_component_top_level_symbol(sym_nested));
    }

    #[test]
    fn collect_component_top_level_symbol_names_excludes_nested_scopes() {
        let mut sem = ComponentSemantics::new();
        let root = sem.root_scope_id();
        let module = sem.add_scope(root, ScopeFlags::Top | ScopeFlags::Function);
        sem.set_module_scope_id(module);

        sem.add_binding(
            root,
            "instance_top",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );
        sem.add_binding(
            module,
            "module_top",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::ModuleScript,
        );
        let nested = sem.add_child_scope(root);
        sem.add_binding(
            nested,
            "nested_local",
            Span::default(),
            SymbolFlags::empty(),
            OxcNodeId::DUMMY,
            SymbolOwner::InstanceScript,
        );

        let names = sem.collect_component_top_level_symbol_names();

        assert!(names.contains("instance_top"));
        assert!(names.contains("module_top"));
        assert!(!names.contains("nested_local"));
    }
}
