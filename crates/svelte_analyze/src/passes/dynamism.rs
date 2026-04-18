//! Dynamism classification — «should a template fragment / attribute / component
//! be wrapped in reactive code at codegen time?».
//!
//! Single source of truth for three orthogonal consumer questions:
//! - `is_dynamic_node(id)` — fragment item needs `$.template_effect(...)` wrap.
//! - `is_dynamic_attr(id)`  — attribute needs per-attr effect wrap (separate
//!   from the node-level question because an element can be static overall but
//!   hold a single dynamic `class:foo={bar}` directive).
//! - `is_dynamic_component(id)` — component call site needs `$.component(...)`
//!   (dotted callee / `<svelte:component>` / reactive-bound callee). Orthogonal
//!   to `is_dynamic_node` on the same component: a dynamic callee with static
//!   props does not need a template_effect wrap around the call.
//!
//! All three sets are populated by one template-walker pass that consults
//! `ReactivitySemantics::reference_semantics(ref_id)` for per-reference answers
//! and `DeclarationSemantics` for component-binding classification.

use svelte_ast::{
    Attribute, AwaitBlock, ComponentNode, ConstTag, NodeId, SlotElementLegacy, SVELTE_COMPONENT,
};
use svelte_span::Span;

use crate::types::data::{DeclarationSemantics, ParentKind};
use crate::types::node_table::NodeBitSet;
use crate::walker::{TemplateVisitor, VisitContext};

/// Storage for dynamism classification. Three disjoint BitSets because the
/// consumer questions are semantically different even when they touch the
/// same NodeId (see module docs).
#[derive(Debug)]
pub struct DynamismData {
    dynamic_nodes: NodeBitSet,
    dynamic_attrs: NodeBitSet,
    dynamic_components: NodeBitSet,
}

impl DynamismData {
    pub fn new(node_count: u32) -> Self {
        Self {
            dynamic_nodes: NodeBitSet::new(node_count),
            dynamic_attrs: NodeBitSet::new(node_count),
            dynamic_components: NodeBitSet::new(node_count),
        }
    }

    pub fn is_dynamic_node(&self, id: NodeId) -> bool {
        self.dynamic_nodes.contains(&id)
    }

    pub fn is_dynamic_attr(&self, id: NodeId) -> bool {
        self.dynamic_attrs.contains(&id)
    }

    pub fn is_dynamic_component(&self, id: NodeId) -> bool {
        self.dynamic_components.contains(&id)
    }

    pub(crate) fn mark_dynamic_node(&mut self, id: NodeId) {
        self.dynamic_nodes.insert(id);
    }

    pub(crate) fn mark_dynamic_attr(&mut self, id: NodeId) {
        self.dynamic_attrs.insert(id);
    }

    pub(crate) fn mark_dynamic_component(&mut self, id: NodeId) {
        self.dynamic_components.insert(id);
    }
}

/// Single template walker that populates `DynamismData`'s three BitSets.
///
/// Logic mirrors the legacy `reactivity.rs` visitor (for `dynamic_nodes` +
/// `dynamic_attrs`) and the component-branch of `element_flags.rs` (for
/// `dynamic_components`). Source of truth for per-reference dynamism is
/// `ExpressionInfo::is_dynamic` / `has_state` (for now — until Step 4 moves
/// the per-reference query onto `ReferenceSemantics` directly and removes
/// the cached flags).
pub(crate) struct DynamismVisitor;

impl DynamismVisitor {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl TemplateVisitor for DynamismVisitor {
    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_, '_>) {
        ctx.data.dynamism.mark_dynamic_node(block.id);
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_, '_>) {
        if ctx
            .data
            .expressions
            .get(tag.id)
            .is_some_and(|info| info.is_dynamic())
        {
            ctx.data.dynamism.mark_dynamic_node(tag.id);
        }
    }

    fn visit_slot_element_legacy(&mut self, el: &SlotElementLegacy, ctx: &mut VisitContext<'_, '_>) {
        ctx.data.dynamism.mark_dynamic_node(el.id);
    }

    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_, '_>) {
        // `needs_ref` lives alongside dynamism because the condition "this
        // element hosts a bindable / reactive attr" is what later triggers
        // both the effect wrap and the element reference emission.
        if ParentKind::from_attr(attr).is_some_and(|k| k.needs_element_ref()) {
            if let Some(el_id) = ctx.data.nearest_element(attr.id()) {
                ctx.data.elements.flags.needs_ref.insert(el_id);
            }
        }
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_, '_>) {
        let data = &*ctx.data;
        let uses_runes = data.uses_runes();
        let base_name = cn
            .name
            .split('.')
            .next()
            .unwrap_or_else(|| cn.name.as_str());
        if let Some(sym_id) = data.scoping.find_binding(ctx.scope, base_name) {
            if uses_runes && is_reactive_component_binding(data, sym_id) {
                ctx.data.dynamism.mark_dynamic_component(cn.id);
            }
        }
        if uses_runes && cn.name.contains('.') {
            ctx.data.dynamism.mark_dynamic_component(cn.id);
        }
        if cn.name == SVELTE_COMPONENT {
            ctx.data.dynamism.mark_dynamic_component(cn.id);
        }
    }

    fn visit_expression(&mut self, node_id: NodeId, _span: Span, ctx: &mut VisitContext<'_, '_>) {
        let parent = ctx.data.expr_parent(node_id);
        let parent_kind = parent.map(|p| p.kind);

        if parent_kind.is_some_and(|k| k.is_attr()) {
            let attr_id = parent.map_or(node_id, |p| p.id);
            let in_component = ctx
                .data
                .expr_ancestors(node_id)
                .nth(1)
                .is_some_and(|gp| {
                    matches!(
                        gp.kind,
                        ParentKind::ComponentNode | ParentKind::SvelteBoundary
                    )
                });

            let is_dynamic = ctx.data.attr_expressions.get(node_id).is_some_and(|info| {
                if in_component {
                    info.has_state()
                } else {
                    info.is_dynamic()
                }
            });

            if is_dynamic {
                ctx.data.dynamism.mark_dynamic_attr(attr_id);
                if !in_component {
                    if let Some(el_id) = ctx.data.nearest_element_for_expr(node_id) {
                        ctx.data.elements.flags.needs_ref.insert(el_id);
                        if matches!(parent_kind, Some(ParentKind::ClassDirective)) {
                            ctx.data
                                .elements
                                .flags
                                .has_dynamic_class_directives
                                .insert(el_id);
                        }
                    }
                }
            }
        } else if !(matches!(parent_kind, Some(ParentKind::SvelteElement))
            && parent.is_some_and(|p| p.id == node_id))
        {
            if ctx
                .data
                .expressions
                .get(node_id)
                .is_some_and(|info| info.is_dynamic())
            {
                ctx.data.dynamism.mark_dynamic_node(node_id);
            }
        }
    }
}

/// A component binding is «reactive» for the purposes of dynamic-component
/// lowering when its declaration is anything other than `NonReactive` or
/// `Unresolved`. Kept identical to the legacy `element_flags` helper so the
/// parallel pass produces bit-for-bit the same classification.
fn is_reactive_component_binding(
    data: &crate::AnalysisData<'_>,
    sym: crate::scope::SymbolId,
) -> bool {
    !matches!(
        data.declaration_semantics(data.scoping.symbol_declaration(sym)),
        DeclarationSemantics::NonReactive | DeclarationSemantics::Unresolved,
    )
}

