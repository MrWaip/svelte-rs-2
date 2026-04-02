use crate::walker::{ParentKind, TemplateVisitor, VisitContext};
use svelte_ast::{Attribute, AwaitBlock, ConstTag, NodeId};
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
        ctx.data.dynamic_nodes.insert(block.id);
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_>) {
        if ctx
            .data
            .expressions
            .get(tag.id)
            .is_some_and(|info| info.is_dynamic)
        {
            ctx.data.dynamic_nodes.insert(tag.id);
        }
    }

    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_>) {
        if ParentKind::from_attr(attr).is_some_and(|k| k.needs_element_ref()) {
            if let Some(el_id) = ctx.nearest_element() {
                ctx.data.element_flags.needs_ref.insert(el_id);
            }
        }
    }

    fn visit_expression(&mut self, node_id: NodeId, _span: Span, ctx: &mut VisitContext<'_>) {
        let parent_kind = ctx.parent().map(|p| p.kind);

        if parent_kind.is_some_and(|k| k.is_attr()) {
            // For concat parts, use the parent attribute's id for dynamic_attrs marking
            let attr_id = ctx.parent().map_or(node_id, |p| p.id);
            let in_component = ctx
                .ancestors()
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
                ctx.data.element_flags.dynamic_attrs.insert(attr_id);
                if !in_component {
                    if let Some(el_id) = ctx.nearest_element() {
                        ctx.data.element_flags.needs_ref.insert(el_id);
                        if matches!(parent_kind, Some(ParentKind::ClassDirective)) {
                            ctx.data
                                .element_flags
                                .has_dynamic_class_directives
                                .insert(el_id);
                        }
                    }
                }
            }
            // else: SvelteWindow/Document/Body attrs — not processed (matches original)
        } else if !(matches!(parent_kind, Some(ParentKind::SvelteElement))
            && ctx.parent().is_some_and(|p| p.id == node_id))
        {
            // SvelteElement tag expression (where node_id == parent el.id) is not
            // classified as dynamic_node. Child expressions have their own id.
            if ctx
                .data
                .expressions
                .get(node_id)
                .is_some_and(|info| info.is_dynamic)
            {
                ctx.data.dynamic_nodes.insert(node_id);
            }
        }
    }
}
