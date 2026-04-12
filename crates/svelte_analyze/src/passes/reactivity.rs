use crate::walker::{ParentKind, TemplateVisitor, VisitContext};
use svelte_ast::{Attribute, AwaitBlock, ConstTag, NodeId, SlotElementLegacy};
use svelte_span::Span;

pub(crate) struct ReactivityVisitor;

impl ReactivityVisitor {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl TemplateVisitor for ReactivityVisitor {
    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_>) {
        // Await blocks are always dynamic
        ctx.data.output.dynamic_nodes.insert(block.id);
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_>) {
        if ctx
            .data
            .expressions
            .get(tag.id)
            .is_some_and(|info| info.is_dynamic)
        {
            ctx.data.output.dynamic_nodes.insert(tag.id);
        }
    }

    fn visit_slot_element_legacy(&mut self, el: &SlotElementLegacy, ctx: &mut VisitContext<'_>) {
        ctx.data.output.dynamic_nodes.insert(el.id);
    }

    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_>) {
        if ParentKind::from_attr(attr).is_some_and(|k| k.needs_element_ref()) {
            if let Some(el_id) = ctx.data.nearest_element(attr.id()) {
                ctx.data.elements.flags.needs_ref.insert(el_id);
            }
        }
    }

    fn visit_expression(&mut self, node_id: NodeId, _span: Span, ctx: &mut VisitContext<'_>) {
        let parent = ctx.data.expr_parent(node_id);
        let parent_kind = parent.map(|p| p.kind);

        if parent_kind.is_some_and(|k| k.is_attr()) {
            // For concat parts, use the parent attribute's id for dynamic_attrs marking
            let attr_id = parent.map_or(node_id, |p| p.id);
            let in_component = ctx
                .data
                .expr_ancestors(node_id)
                .nth(1) // grandparent (skip immediate attr parent)
                .is_some_and(|gp| {
                    matches!(
                        gp.kind,
                        ParentKind::ComponentNode | ParentKind::SvelteBoundary
                    )
                });

            let is_dynamic = ctx.data.attr_expressions.get(node_id).is_some_and(|info| {
                if in_component {
                    info.has_state
                } else {
                    info.is_dynamic
                }
            });

            if is_dynamic {
                ctx.data.elements.flags.dynamic_attrs.insert(attr_id);
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
            // else: SvelteWindow/Document/Body attrs — not processed (matches original)
        } else if !(matches!(parent_kind, Some(ParentKind::SvelteElement))
            && parent.is_some_and(|p| p.id == node_id))
        {
            // SvelteElement tag expression (where node_id == parent el.id) is not
            // classified as dynamic_node. Child expressions have their own id.
            if ctx
                .data
                .expressions
                .get(node_id)
                .is_some_and(|info| info.is_dynamic)
            {
                ctx.data.output.dynamic_nodes.insert(node_id);
            }
        }
    }
}
