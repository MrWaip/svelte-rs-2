use compact_str::CompactString;
use oxc_ast::{AstKind, ast::IdentifierReference};
use oxc_span::{GetSpan, Span};
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::scope::{ScopeFlags, ScopeId};
use oxc_syntax::symbol::{SymbolFlags, SymbolId};
use rustc_hash::{FxHashMap, FxHashSet};
use std::marker::PhantomData;

use crate::symbol::SymbolOwner;

use crate::reference::{Reference, ReferenceTable};
use crate::scope::ScopeTable;
use crate::symbol::SymbolTable;

pub struct JsStorage<'a> {
    nodes: Vec<Option<JsNode<'a>>>,
    parent_ids: Vec<Option<OxcNodeId>>,
}

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

pub struct ComponentSemantics<'a> {
    pub(crate) scopes: ScopeTable,
    pub(crate) symbols: SymbolTable,
    pub(crate) references: ReferenceTable,
    pub(crate) js: JsStorage<'a>,

    template_reference_ids: FxHashSet<ReferenceId>,

    root_unresolved_references: FxHashMap<CompactString, Vec<ReferenceId>>,

    store_candidate_refs: Vec<(SymbolId, ReferenceId)>,

    fragment_scopes: Vec<Option<ScopeId>>,

    template_scope_set: FxHashSet<ScopeId>,

    module_scope_id: Option<ScopeId>,

    instance_scope_id: Option<ScopeId>,
    ast_lifetime: PhantomData<&'a ()>,
}

impl<'a> ComponentSemantics<'a> {
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
            store_candidate_refs: Vec::new(),
            fragment_scopes: Vec::new(),
            template_scope_set: FxHashSet::default(),
            module_scope_id: None,
            instance_scope_id: None,
            ast_lifetime: PhantomData,
        }
    }

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

    pub fn has_symbol_state(&self, id: SymbolId, bit: u32) -> bool {
        self.symbols.has_state(id, bit)
    }

    pub fn set_symbol_state(&mut self, id: SymbolId, bit: u32) {
        self.symbols.set_state(id, bit);
    }

    pub fn symbols_with_state(&self, bit: u32) -> impl Iterator<Item = SymbolId> + '_ {
        self.symbols
            .symbol_ids()
            .filter(move |&id| self.symbols.has_state(id, bit))
    }

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

    pub fn is_instance_reference(&self, id: ReferenceId) -> bool {
        let Some(instance) = self.instance_scope_id else {
            return false;
        };
        let Some(sym) = self.references.get(id).symbol_id() else {
            return false;
        };
        self.symbols.symbol_scope_id(sym) == instance
    }

    pub fn is_module_reference(&self, id: ReferenceId) -> bool {
        let Some(module) = self.module_scope_id else {
            return false;
        };
        let Some(sym) = self.references.get(id).symbol_id() else {
            return false;
        };
        self.symbols.symbol_scope_id(sym) == module
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

    pub fn add_resolved_reference(&mut self, symbol_id: SymbolId, reference_id: ReferenceId) {
        let is_write = self.references.get(reference_id).is_write();
        self.symbols
            .add_resolved_reference(symbol_id, reference_id, is_write);
    }

    pub fn mark_symbol_member_mutated(&mut self, symbol_id: SymbolId) {
        self.symbols
            .set_state(symbol_id, crate::symbol::state::MEMBER_MUTATED);
    }

    pub fn is_member_mutated(&self, id: SymbolId) -> bool {
        self.symbols
            .has_state(id, crate::symbol::state::MEMBER_MUTATED)
    }

    pub fn is_mutated_any(&self, id: SymbolId) -> bool {
        self.is_mutated(id) || self.is_member_mutated(id)
    }

    pub fn get_resolved_reference_ids(&self, id: SymbolId) -> &[ReferenceId] {
        self.symbols.get_resolved_reference_ids(id)
    }

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

                AstKind::VariableDeclarator(_) => {
                    return Some(self.symbol_name(sym_id));
                }
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

    pub fn find_function_scope(&self, scope: ScopeId) -> ScopeId {
        self.scopes.find_function_scope(scope)
    }

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

    pub fn is_expr_local(&self, sym: SymbolId) -> bool {
        let scope = self.symbols.symbol_scope_id(sym);
        !self.is_component_top_level_scope(scope) && !self.template_scope_set.contains(&scope)
    }

    pub fn symbol_ids(&self) -> impl Iterator<Item = SymbolId> {
        self.symbols.symbol_ids()
    }

    pub fn symbol_names(&self) -> impl Iterator<Item = &str> {
        self.symbols.symbol_names()
    }

    pub fn collect_import_syms(&self) -> FxHashSet<SymbolId> {
        self.symbols
            .symbol_ids()
            .filter(|&id| self.symbols.symbol_flags(id).contains(SymbolFlags::Import))
            .collect()
    }

    pub fn find_symbol_by_name(&self, name: &str) -> Option<SymbolId> {
        self.symbols.find_by_name(name)
    }

    pub fn collect_all_symbol_names(&self) -> FxHashSet<CompactString> {
        self.symbols
            .symbol_names()
            .map(CompactString::from)
            .collect()
    }

    pub fn collect_component_top_level_symbol_names(&self) -> FxHashSet<CompactString> {
        self.symbol_ids()
            .filter(|&sym| self.is_component_top_level_symbol(sym))
            .map(|sym| CompactString::from(self.symbol_name(sym)))
            .collect()
    }

    pub fn set_fragment_scope_by_id(&mut self, id: svelte_ast::FragmentId, scope: ScopeId) {
        let idx = id.0 as usize;
        if self.fragment_scopes.len() <= idx {
            self.fragment_scopes.resize(idx + 1, None);
        }
        self.fragment_scopes[idx] = Some(scope);
    }

    pub fn fragment_scope_by_id(&self, id: svelte_ast::FragmentId) -> Option<ScopeId> {
        self.fragment_scopes.get(id.0 as usize).copied().flatten()
    }

    pub fn build_template_scope_set(&mut self) {
        self.template_scope_set = self.fragment_scopes.iter().filter_map(|s| *s).collect();
    }

    pub fn is_template_scope(&self, scope: ScopeId) -> bool {
        self.template_scope_set.contains(&scope)
    }

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

    pub fn root_unresolved_references(&self) -> &FxHashMap<CompactString, Vec<ReferenceId>> {
        &self.root_unresolved_references
    }

    pub(crate) fn add_store_candidate_ref(&mut self, sym: SymbolId, ref_id: ReferenceId) {
        self.store_candidate_refs.push((sym, ref_id));
    }

    pub fn store_candidate_refs(&self) -> &[(SymbolId, ReferenceId)] {
        &self.store_candidate_refs
    }

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

        let read_ref =
            sem.create_reference(Reference::new(OxcNodeId::DUMMY, root, ReferenceFlags::Read));
        sem.add_resolved_reference(sym, read_ref);
        assert!(!sem.is_mutated(sym));

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

        assert_eq!(sem.function_depth(root), 1);

        assert_eq!(sem.function_depth(block), 1);

        assert_eq!(sem.function_depth(fn_scope), 2);

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
