use crate::expression_semantics::{ExpressionSemantics, ExpressionSemanticsStore};
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::types::data::{BlockerData, FragmentNamespaces, IgnoreData, JsAst};

use super::super::{BlockSemanticsStore, SnippetPlacement};
use super::common::declarator_from_stmt;
use super::declaration_group::DeclarationOwner;

use oxc_ast::ast::IdentifierReference;
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeId;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{Attribute, BindDirective, Component, EachBlock, FragmentId, Node, NodeId};
use svelte_component_semantics::{ComponentSemantics, ReferenceId, SymbolId, walk_bindings};

pub(super) fn populate(
    component: &Component,
    parsed: &JsAst<'_>,
    semantics: &ComponentSemantics<'_>,
    reactivity: &ReactivitySemantics,
    expressions: &ExpressionSemanticsStore,
    fragment_namespaces: &FragmentNamespaces,
    ignore_data: &IgnoreData,
    blocker_data: &BlockerData,
    dev: bool,
    store: &mut BlockSemanticsStore,
) {
    let mut ctx = Ctx {
        component,
        parsed,
        semantics,
        reactivity,
        expressions,
        fragment_namespaces,
        ignore_data,
        blocker_data,
        dev,
        current_fragment_id: component.root,
        non_root_depth: 0,
        snippet_scopes: Vec::new(),
        snippet_name_syms: FxHashSet::default(),
        store,
        each_stack: SmallVec::new(),
        bind_group_hits: FxHashSet::default(),
        declaration_owners: FxHashMap::default(),
        declaration_group_stack: SmallVec::new(),
    };
    ctx.enter_declaration_group();
    for &node_id in component.store.fragment_nodes(component.root) {
        ctx.visit_node(node_id);
    }
    ctx.leave_declaration_group(component.root);

    finalize_hoistable(
        &ctx.snippet_scopes,
        &ctx.snippet_name_syms,
        semantics,
        reactivity,
        ctx.store,
    );
}

fn finalize_hoistable(
    snippet_scopes: &[SnippetScope],
    snippet_name_syms: &FxHashSet<SymbolId>,
    semantics: &ComponentSemantics<'_>,
    reactivity: &ReactivitySemantics,
    store: &mut BlockSemanticsStore,
) {
    if snippet_scopes.is_empty() {
        return;
    }

    let mut scope_to_block: rustc_hash::FxHashMap<ScopeId, (NodeId, bool)> =
        rustc_hash::FxHashMap::default();
    let mut symbol_to_block: rustc_hash::FxHashMap<SymbolId, NodeId> =
        rustc_hash::FxHashMap::default();
    for entry in snippet_scopes {
        scope_to_block.insert(entry.body_scope, (entry.block_id, entry.top_level));
        symbol_to_block.insert(entry.name_symbol, entry.block_id);
    }

    let mut tainted: FxHashSet<NodeId> = FxHashSet::default();
    let mut edges: Vec<(NodeId, NodeId)> = Vec::new();

    for idx in 0..semantics.references_len() {
        let ref_id = ReferenceId::from_usize(idx);

        let mut referenced_snippet: Option<NodeId> = None;
        if !reactivity
            .reference_semantics(ref_id)
            .is_store_subscription()
        {
            if !semantics.is_instance_reference(ref_id) {
                continue;
            }

            if let Some(sym) = semantics.get_reference(ref_id).symbol_id() {
                if snippet_name_syms.contains(&sym) {
                    match symbol_to_block.get(&sym) {
                        Some(&target) => referenced_snippet = Some(target),
                        None => continue,
                    }
                } else if reactivity.binding_semantics(sym).is_maybe_reactive() {
                    continue;
                }
            }
        }

        let mut scope = Some(semantics.get_reference(ref_id).scope_id());
        while let Some(s) = scope {
            if let Some(&(block_id, _)) = scope_to_block.get(&s) {
                match referenced_snippet {
                    Some(target) if target != block_id => edges.push((block_id, target)),
                    Some(_) => {}
                    None => {
                        tainted.insert(block_id);
                    }
                }
            }
            scope = semantics.scope_parent_id(s);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for &(referrer, target) in &edges {
            if tainted.contains(&target) && tainted.insert(referrer) {
                changed = true;
            }
        }
    }

    for entry in snippet_scopes {
        if !entry.top_level {
            continue;
        }
        let placement = if tainted.contains(&entry.block_id) {
            SnippetPlacement::InstanceLevel
        } else {
            SnippetPlacement::ModuleLevel
        };
        store.set_snippet_placement(entry.block_id, placement);
    }
}

#[derive(Copy, Clone)]
pub(super) struct SnippetScope {
    pub(super) block_id: NodeId,
    pub(super) name_symbol: SymbolId,
    pub(super) body_scope: ScopeId,
    pub(super) top_level: bool,
}

pub(super) struct Ctx<'c, 'a> {
    pub(super) component: &'c Component,
    pub(super) parsed: &'c JsAst<'a>,
    pub(super) semantics: &'c ComponentSemantics<'a>,
    pub(super) reactivity: &'c ReactivitySemantics,
    pub(super) expressions: &'c ExpressionSemanticsStore,
    pub(super) fragment_namespaces: &'c FragmentNamespaces,
    pub(super) ignore_data: &'c IgnoreData,
    pub(super) blocker_data: &'c BlockerData,
    pub(super) dev: bool,
    pub(super) current_fragment_id: FragmentId,

    pub(super) non_root_depth: u32,

    pub(super) snippet_scopes: Vec<SnippetScope>,

    pub(super) snippet_name_syms: FxHashSet<SymbolId>,
    pub(super) store: &'c mut BlockSemanticsStore,

    each_stack: SmallVec<[EachFrame; 4]>,

    bind_group_hits: FxHashSet<NodeId>,

    pub(super) declaration_owners: FxHashMap<SymbolId, DeclarationOwner>,

    declaration_group_stack: SmallVec<[SmallVec<[NodeId; 2]>; 4]>,
}

#[derive(Clone)]
struct EachFrame {
    block_id: NodeId,
    introduced: SmallVec<[SymbolId; 4]>,
}

impl<'a> Ctx<'_, 'a> {
    fn visit_node(&mut self, id: NodeId) {
        let node = self.component.store.get(id);
        match node {
            Node::EachBlock(block) => super::each::populate(self, block),
            Node::AwaitBlock(block) => super::await_::populate(self, block),
            Node::Element(el) => {
                self.check_bind_group_in_attrs(&el.attributes);
                self.visit_fragment(el.fragment);
            }
            Node::SlotElementLegacy(el) => self.visit_fragment(el.fragment),
            Node::ComponentNode(cn) => {
                self.visit_fragment(cn.fragment);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    self.visit_fragment(fid);
                }
            }
            Node::SvelteSelf(cn) => {
                self.visit_fragment(cn.fragment);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    self.visit_fragment(fid);
                }
            }
            Node::SvelteComponentLegacy(cn) => {
                self.visit_fragment(cn.fragment);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    self.visit_fragment(fid);
                }
            }
            Node::IfBlock(block) => super::if_::populate(self, block),
            Node::SnippetBlock(block) => super::snippet::populate(self, block),
            Node::ConstTag(tag) => super::const_tag::populate(self, tag),
            Node::DeclarationTag(tag) => super::declaration_tag::populate(self, tag),
            Node::RenderTag(tag) => super::render::populate(self, tag),
            Node::KeyBlock(block) => super::key::populate(self, block),
            Node::HtmlTag(tag) => super::html_tag::populate(self, tag),
            Node::SvelteHead(el) => self.visit_fragment(el.fragment),
            Node::SvelteFragmentLegacy(el) => self.visit_fragment(el.fragment),
            Node::SvelteElement(el) => {
                self.check_bind_group_in_attrs(&el.attributes);
                self.visit_fragment(el.fragment);
            }
            Node::SvelteBoundary(el) => self.visit_fragment(el.fragment),
            _ => {}
        }
    }

    pub(super) fn visit_fragment(&mut self, fragment_id: svelte_ast::FragmentId) {
        self.non_root_depth += 1;
        let prev_fragment_id = self.current_fragment_id;
        self.current_fragment_id = fragment_id;
        self.enter_declaration_group();
        let component = self.component;
        for &id in component.fragment_nodes(fragment_id) {
            self.visit_node(id);
        }
        self.leave_declaration_group(fragment_id);
        self.current_fragment_id = prev_fragment_id;
        self.non_root_depth -= 1;
    }

    pub(super) fn enter_declaration_group(&mut self) {
        self.declaration_group_stack.push(SmallVec::new());
    }

    pub(super) fn leave_declaration_group(&mut self, fragment_id: svelte_ast::FragmentId) {
        let Some(members) = self.declaration_group_stack.pop() else {
            return;
        };
        if members.is_empty() {
            return;
        }
        self.store
            .set_fragment_declaration_group(fragment_id, members);
    }

    pub(super) fn declaration_group_is_open(&self) -> bool {
        self.declaration_group_stack
            .last()
            .is_some_and(|members| !members.is_empty())
    }

    pub(super) fn push_declaration_group_member(&mut self, node_id: NodeId) {
        let Some(members) = self.declaration_group_stack.last_mut() else {
            return;
        };
        let opens_group = members.is_empty();
        members.push(node_id);
        if opens_group {
            self.store
                .open_fragment_declaration_group(self.current_fragment_id);
        }
    }

    pub(super) fn push_each_frame(
        &mut self,
        block_id: NodeId,
        introduced: SmallVec<[SymbolId; 4]>,
    ) {
        self.each_stack.push(EachFrame {
            block_id,
            introduced,
        });
    }

    pub(super) fn pop_each_frame(&mut self) {
        self.each_stack.pop();
    }

    pub(super) fn each_has_group_binding(&self, block_id: NodeId) -> bool {
        self.bind_group_hits.contains(&block_id)
    }

    pub(super) fn collect_each_introduced_symbols(
        &self,
        block: &EachBlock,
        item_sym: Option<SymbolId>,
        pattern_fallback: bool,
        index_sym: Option<SymbolId>,
    ) -> SmallVec<[SymbolId; 4]> {
        let mut out: SmallVec<[SymbolId; 4]> = SmallVec::new();
        if let Some(sym) = item_sym {
            out.push(sym);
        } else if pattern_fallback
            && let Some(decl) = block
                .context
                .as_ref()
                .and_then(|r| self.parsed.stmt(r.id()))
                .and_then(declarator_from_stmt)
        {
            walk_bindings(&decl.id, |v| out.push(v.symbol));
        }
        if let Some(sym) = index_sym {
            out.push(sym);
        }
        out
    }

    fn check_bind_group_in_attrs(&mut self, attrs: &[Attribute]) {
        if self.each_stack.is_empty() {
            return;
        }
        for attr in attrs {
            let Attribute::BindDirective(dir) = attr else {
                continue;
            };
            if dir.name != "group" {
                continue;
            }
            self.attribute_bind_group(dir);
        }
    }

    fn attribute_bind_group(&mut self, dir: &BindDirective) {
        let Some(expr) = self.parsed.expr(dir.expression.id()) else {
            return;
        };
        let mut collector = RefCollector { refs: Vec::new() };
        collector.visit_expression(expr);
        let mut ids: SmallVec<[SymbolId; 8]> = SmallVec::new();
        for ref_id in collector.refs {
            let Some(sym) = self.semantics.get_reference(ref_id).symbol_id() else {
                continue;
            };
            if !ids.contains(&sym) {
                ids.push(sym);
            }
        }
        for i in (0..self.each_stack.len()).rev() {
            let frame = &self.each_stack[i];
            let owns_any = ids.iter().any(|sym| frame.introduced.contains(sym));
            if !owns_any {
                continue;
            }
            let block_id = frame.block_id;
            self.bind_group_hits.insert(block_id);
            for sym in self.each_collection_symbols(block_id) {
                if !ids.contains(&sym) {
                    ids.push(sym);
                }
            }
        }
    }

    fn each_collection_symbols(&self, block_id: NodeId) -> SmallVec<[SymbolId; 4]> {
        let ExpressionSemantics::Expression(data) = self.expressions.get(block_id) else {
            return SmallVec::new();
        };
        data.references.iter().copied().collect()
    }
}

struct RefCollector {
    refs: Vec<ReferenceId>,
}

impl<'a> Visit<'a> for RefCollector {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
}
