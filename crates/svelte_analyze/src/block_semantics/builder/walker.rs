use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::types::data::{BlockerData, JsAst};

use super::super::BlockSemanticsStore;
use super::common::declarator_from_stmt;

use oxc_ast::ast::IdentifierReference;
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeId;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use svelte_ast::{Attribute, BindDirective, Component, EachBlock, Node, NodeId};
use svelte_component_semantics::{ComponentSemantics, ReferenceId, SymbolId, walk_bindings};

pub(super) fn populate(
    component: &Component,
    parsed: &JsAst<'_>,
    semantics: &ComponentSemantics<'_>,
    reactivity: &ReactivitySemantics,
    blockers: &BlockerData,
    store: &mut BlockSemanticsStore,
) {
    let mut ctx = Ctx {
        component,
        parsed,
        semantics,
        reactivity,
        blockers,
        non_root_depth: 0,
        snippet_scopes: Vec::new(),
        snippet_name_syms: FxHashSet::default(),
        store,
        each_stack: SmallVec::new(),
        bind_group_hits: FxHashSet::default(),
    };
    let root_nodes_len = component.store.fragment_nodes(component.root).len();
    for i in 0..root_nodes_len {
        let node_id = component.store.fragment_nodes(component.root)[i];
        ctx.visit_node(node_id);
    }

    finalize_hoistable(
        &ctx.snippet_scopes,
        &ctx.snippet_name_syms,
        semantics,
        ctx.store,
    );
}

fn finalize_hoistable(
    snippet_scopes: &[SnippetScope],
    snippet_name_syms: &FxHashSet<SymbolId>,
    semantics: &ComponentSemantics<'_>,
    store: &mut BlockSemanticsStore,
) {
    if snippet_scopes.is_empty() {
        return;
    }

    let mut scope_to_block: rustc_hash::FxHashMap<ScopeId, (NodeId, bool)> =
        rustc_hash::FxHashMap::default();
    for entry in snippet_scopes {
        scope_to_block.insert(entry.body_scope, (entry.block_id, entry.top_level));
    }

    let mut tainted: FxHashSet<NodeId> = FxHashSet::default();

    for idx in 0..semantics.references_len() {
        let ref_id = ReferenceId::from_usize(idx);
        if !semantics.is_instance_reference(ref_id) {
            continue;
        }

        if let Some(sym) = semantics.get_reference(ref_id).symbol_id()
            && snippet_name_syms.contains(&sym)
        {
            continue;
        }

        let mut scope = Some(semantics.get_reference(ref_id).scope_id());
        while let Some(s) = scope {
            if let Some(&(block_id, _)) = scope_to_block.get(&s) {
                tainted.insert(block_id);
            }
            scope = semantics.scope_parent_id(s);
        }
    }

    for entry in snippet_scopes {
        if !entry.top_level {
            continue;
        }
        if !tainted.contains(&entry.block_id) {
            store.set_snippet_hoistable(entry.block_id, true);
        }
    }
}

#[derive(Copy, Clone)]
pub(super) struct SnippetScope {
    pub(super) block_id: NodeId,
    pub(super) body_scope: ScopeId,
    pub(super) top_level: bool,
}

pub(super) struct Ctx<'c, 'a> {
    pub(super) component: &'c Component,
    pub(super) parsed: &'c JsAst<'a>,
    pub(super) semantics: &'c ComponentSemantics<'a>,
    pub(super) reactivity: &'c ReactivitySemantics,
    pub(super) blockers: &'c BlockerData,

    pub(super) non_root_depth: u32,

    pub(super) snippet_scopes: Vec<SnippetScope>,

    pub(super) snippet_name_syms: FxHashSet<SymbolId>,
    pub(super) store: &'c mut BlockSemanticsStore,

    each_stack: SmallVec<[EachFrame; 4]>,

    bind_group_hits: FxHashSet<NodeId>,
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
            Node::IfBlock(block) => super::if_::populate(self, block),
            Node::SnippetBlock(block) => super::snippet::populate(self, block),
            Node::ConstTag(tag) => super::const_tag::populate(self, tag),
            Node::RenderTag(tag) => super::render::populate(self, tag),
            Node::KeyBlock(block) => super::key::populate(self, block),
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
        let len = self.component.fragment_nodes(fragment_id).len();
        for i in 0..len {
            let id = self.component.fragment_nodes(fragment_id)[i];
            self.visit_node(id);
        }
        self.non_root_depth -= 1;
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
        for ref_id in collector.refs {
            let Some(sym) = self.semantics.get_reference(ref_id).symbol_id() else {
                continue;
            };
            for frame in &self.each_stack {
                if frame.introduced.contains(&sym) {
                    self.bind_group_hits.insert(frame.block_id);
                }
            }
        }
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
