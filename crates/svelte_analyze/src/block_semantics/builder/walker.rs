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
use crate::types::data::{BlockerData, JsAst};

use super::super::BlockSemanticsStore;
use super::common::declarator_from_stmt;

use oxc_ast::ast::IdentifierReference;
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeId;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use svelte_ast::{Attribute, BindDirective, Component, EachBlock, Node, NodeId};
use svelte_component_semantics::{walk_bindings, ComponentSemantics, ReferenceId, SymbolId};

/// Entry point: run the single cluster-wide template walk.
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

/// Flip `SnippetBlockSemantics.hoistable` to `true` for every top-level
/// snippet whose body contains no reference to an instance-scope symbol
/// (i.e. nothing declared in `<script>`). Walk scope-chain from each
/// reference's own scope up to the component root; if the chain passes
/// through a collected snippet body scope and the reference resolves to
/// an instance-scope symbol — that snippet is tainted.
fn finalize_hoistable(
    snippet_scopes: &[SnippetScope],
    snippet_name_syms: &FxHashSet<SymbolId>,
    semantics: &ComponentSemantics<'_>,
    store: &mut BlockSemanticsStore,
) {
    if snippet_scopes.is_empty() {
        return;
    }

    // Reverse lookup: body scope id → (block id, top-level flag). Scope ids
    // are unique per snippet body so no collision is possible.
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
        // Sibling snippet references live in instance scope too (every
        // `{#snippet foo}` declares `foo` at the component function
        // level). Calling one snippet from another must not taint the
        // caller — match the legacy behaviour where only script-authored
        // bindings counted.
        if let Some(sym) = semantics.get_reference(ref_id).symbol_id() {
            if snippet_name_syms.contains(&sym) {
                continue;
            }
        }
        // Walk up the scope chain from the reference's own scope; if we
        // hit any snippet body scope along the way that snippet transitively
        // reads an instance-scope symbol. Mark **every** snippet we pass
        // through (a ref nested inside snippet A inside snippet B taints
        // both — though B's top-level status is what matters for hoisting).
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
            // Nested snippets are never hoistable — populator already seeded
            // `hoistable: false`; skip.
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
    /// Nesting counter updated as the walker descends into container
    /// nodes (elements, blocks, components, slots, etc.). 0 means
    /// "currently iterating the component fragment root" — the only
    /// position where a `{#snippet}` counts as top-level for hoisting.
    pub(super) non_root_depth: u32,
    /// Body-scope snapshot for every `{#snippet}` encountered during the
    /// walk. Consumed by `finalize_hoistable` after the walk completes:
    /// for each ref in the component that resolves to an instance-scope
    /// symbol we look up its scope chain against this table and taint
    /// the enclosing snippet.
    pub(super) snippet_scopes: Vec<SnippetScope>,
    /// Symbols that name component snippets (the `foo` in
    /// `{#snippet foo(...)}`). Registered by the snippet populator during
    /// the walk. Used by `finalize_hoistable` to exclude references to
    /// sibling snippets from the instance-scope taint set: calling one
    /// snippet from another doesn't make the caller instance-bound.
    pub(super) snippet_name_syms: FxHashSet<SymbolId>,
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

    /// Descend into a fragment's children by FragmentId.
    pub(super) fn visit_fragment(&mut self, fragment_id: svelte_ast::FragmentId) {
        self.non_root_depth += 1;
        let len = self.component.fragment_nodes(fragment_id).len();
        for i in 0..len {
            let id = self.component.fragment_nodes(fragment_id)[i];
            self.visit_node(id);
        }
        self.non_root_depth -= 1;
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
                .context
                .as_ref()
                .and_then(|r| self.parsed.stmt(r.id()))
                .and_then(declarator_from_stmt)
            {
                walk_bindings(&decl.id, |v| out.push(v.symbol));
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
