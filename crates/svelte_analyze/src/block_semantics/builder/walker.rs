//! Cluster-wide template walker.
//!
//! One traversal of the Svelte template populates every migrated Block
//! Semantics kind. The walker owns the shared state (component / parsed
//! / semantics / reactivity / blockers / store) plus cross-kind scratch
//! (today: an each-block stack used for scope-qualified `bind:group`
//! attribution). Per-kind population logic lives in sibling modules
//! (`super::each`, `super::await_`, future `super::if_`, ...) as free
//! functions that take `&mut Ctx`.

use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::types::data::{BlockerData, ParserResult};

use super::super::BlockSemanticsStore;
use super::common::{collect_binding_pattern_symbols, declarator_from_stmt};

use oxc_ast::ast::IdentifierReference;
use oxc_ast_visit::Visit;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use svelte_ast::{Attribute, BindDirective, Component, EachBlock, Node, NodeId};
use svelte_component_semantics::{ComponentSemantics, ReferenceId, SymbolId};

/// Entry point: run the single cluster-wide template walk.
pub(super) fn populate(
    component: &Component,
    parsed: &ParserResult<'_>,
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
        store,
        each_stack: SmallVec::new(),
        bind_group_hits: FxHashSet::default(),
    };
    for &node_id in &component.fragment.nodes {
        ctx.visit_node(node_id);
    }
}

pub(super) struct Ctx<'c, 'a> {
    pub(super) component: &'c Component,
    pub(super) parsed: &'c ParserResult<'a>,
    pub(super) semantics: &'c ComponentSemantics<'a>,
    pub(super) reactivity: &'c ReactivitySemantics,
    pub(super) blockers: &'c BlockerData,
    pub(super) store: &'c mut BlockSemanticsStore,
    /// Stack of enclosing each-blocks during the walk. Each frame
    /// carries the symbols the each introduces in its body scope
    /// (item / index / destructured leaves). Used to attribute
    /// `bind:group={...}` directives to the correct enclosing each
    /// frame (Svelte's scope-qualified rule). Managed by
    /// [`Self::push_each_frame`] / [`Self::pop_each_frame`] — kept on
    /// the walker rather than the each populator because its lifetime
    /// is the traversal, not one `visit_each` call.
    each_stack: SmallVec<[EachFrame; 4]>,
    /// Set of each-block node ids that contain a `bind:group` whose
    /// expression references one of their introduced symbols.
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
                self.visit_fragment(&el.fragment.nodes);
            }
            Node::SlotElementLegacy(el) => self.visit_fragment(&el.fragment.nodes),
            Node::ComponentNode(cn) => self.visit_fragment(&cn.fragment.nodes),
            Node::IfBlock(block) => {
                self.visit_fragment(&block.consequent.nodes);
                if let Some(alt) = &block.alternate {
                    self.visit_fragment(&alt.nodes);
                }
            }
            Node::SnippetBlock(block) => self.visit_fragment(&block.body.nodes),
            Node::KeyBlock(block) => self.visit_fragment(&block.fragment.nodes),
            Node::SvelteHead(el) => self.visit_fragment(&el.fragment.nodes),
            Node::SvelteFragmentLegacy(el) => self.visit_fragment(&el.fragment.nodes),
            Node::SvelteElement(el) => {
                self.check_bind_group_in_attrs(&el.attributes);
                self.visit_fragment(&el.fragment.nodes);
            }
            Node::SvelteBoundary(el) => self.visit_fragment(&el.fragment.nodes),
            _ => {}
        }
    }

    pub(super) fn visit_fragment(&mut self, nodes: &[NodeId]) {
        for &id in nodes {
            self.visit_node(id);
        }
    }

    /// Push a new each frame around a sub-traversal. The each populator
    /// calls this before recursing into the block body so any
    /// `bind:group` directive encountered below can be attributed to
    /// this frame (see [`Self::check_bind_group_in_attrs`]).
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

    /// True iff a `bind:group={expr}` encountered during the body walk
    /// of the given each block referenced a symbol this each introduces.
    pub(super) fn each_has_group_binding(&self, block_id: NodeId) -> bool {
        self.bind_group_hits.contains(&block_id)
    }

    /// Collect leaf identifiers introduced by `{#each ... as <pattern>[, <index>]}`
    /// into a flat `SymbolId` list. Helper for [`Self::push_each_frame`].
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
        } else if pattern_fallback {
            if let Some(decl) = block
                .context_span
                .and_then(|cs| self.parsed.stmt_handle(cs.start))
                .and_then(|h| declarator_from_stmt(self.parsed.stmt(h)?))
            {
                collect_binding_pattern_symbols(&decl.id, &mut out);
            }
        }
        if let Some(sym) = index_sym {
            out.push(sym);
        }
        out
    }

    /// Scan element attributes for a `bind:group={...}` directive. If
    /// found, walk its expression and, for each enclosing each on the
    /// stack whose `introduced` symbols match any referenced symbol,
    /// record a hit.
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
        let Some(expr) = self
            .parsed
            .expr_handle(dir.expression_span.start)
            .and_then(|h| self.parsed.expr(h))
        else {
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
