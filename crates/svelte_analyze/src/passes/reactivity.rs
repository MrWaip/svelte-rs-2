use svelte_span::Span;
use svelte_ast::{
    Attribute, AwaitBlock, ConstTag, NodeId,
};
use crate::types::data::{AnalysisData, ExpressionKind};
use crate::walker::{ParentKind, TemplateVisitor, VisitContext};

pub(crate) struct ReactivityVisitor;

impl ReactivityVisitor {
    pub(crate) fn new() -> Self {
        Self
    }

    fn expr_is_dynamic(
        &self,
        node_id: &NodeId,
        data: &AnalysisData,
    ) -> bool {
        if let Some(info) = data.expressions.get(*node_id) {
            if info.has_state_rune || info.needs_context {
                return true;
            }

            // MemberExpressions: any resolved local binding → dynamic.
            // Mirrors reference is_pure(): only globals (no binding) are pure.
            if matches!(info.kind, ExpressionKind::MemberExpression) {
                return info.references.iter().any(|r| {
                    if data.scoping.is_store_ref(&r.name) {
                        return true;
                    }
                    r.symbol_id.is_some()
                });
            }

            return info.references.iter().any(|r| {
                // Store subscriptions ($count) are always dynamic
                if data.scoping.is_store_ref(&r.name) {
                    return true;
                }
                if let Some(sym_id) = r.symbol_id {
                    if data.scoping.is_dynamic_by_id(sym_id) {
                        return true;
                    }
                    // When class state fields exist, member access on local bindings
                    // is potentially reactive (getters call $.get internally).
                    if data.has_class_state_fields
                        && data.scoping.symbol_scope_id(sym_id) == data.scoping.root_scope_id()
                        && !data.scoping.is_rune(sym_id)
                    {
                        return true;
                    }
                }
                false
            });
        }
        false
    }
}

impl TemplateVisitor for ReactivityVisitor {
    fn visit_await_block(&mut self, block: &AwaitBlock, ctx: &mut VisitContext<'_>) {
        // Await blocks are always dynamic
        ctx.data.dynamic_nodes.insert(block.id);
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, ctx: &mut VisitContext<'_>) {
        if self.expr_is_dynamic(&tag.id, ctx.data) {
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
            let in_component = ctx.ancestors()
                .nth(1) // grandparent (skip immediate attr parent)
                .is_some_and(|gp| matches!(gp.kind, ParentKind::ComponentNode | ParentKind::SvelteBoundary));

            if in_component {
                if component_attr_id_is_dynamic(node_id, ctx.data) {
                    ctx.data.element_flags.dynamic_attrs.insert(node_id);
                }
            } else if let Some(el_id) = ctx.nearest_element() {
                if attr_id_is_dynamic(node_id, ctx.data) {
                    ctx.data.element_flags.dynamic_attrs.insert(node_id);
                    ctx.data.element_flags.needs_ref.insert(el_id);
                    if matches!(parent_kind, Some(ParentKind::ClassDirective)) {
                        ctx.data.element_flags.has_dynamic_class_directives.insert(el_id);
                    }
                }
            }
            // else: SvelteWindow/Document/Body attrs — not processed (matches original)
        } else if !matches!(parent_kind, Some(ParentKind::SvelteElement)) {
            // SvelteElement tag expression is not classified as dynamic_node (matches original)
            if self.expr_is_dynamic(&node_id, ctx.data) {
                ctx.data.dynamic_nodes.insert(node_id);
            }
        }
    }
}

// Component props are dynamic for *any* rune reference (mutated or not).
// This matches Svelte's `has_state` semantics — component props use getters
// so the child component can re-read the value reactively.
fn component_attr_id_is_dynamic(attr_id: NodeId, data: &AnalysisData) -> bool {
    if let Some(info) = data.attr_expressions.get(attr_id) {
        return info.references.iter().any(|r| {
            if let Some(sym_id) = r.symbol_id {
                let root = data.scoping.root_scope_id();
                if data.scoping.symbol_scope_id(sym_id) != root {
                    return true;
                }
                if data.scoping.is_rune(sym_id) {
                    return true;
                }
            }
            false
        });
    }
    false
}

// Per-variant visit methods already exclude String/Boolean, so this
// takes a NodeId directly.
fn attr_id_is_dynamic(attr_id: NodeId, data: &AnalysisData) -> bool {
    if let Some(info) = data.attr_expressions.get(attr_id) {
        return info.references.iter().any(|r| {
            let Some(sym_id) = r.symbol_id else { return false; };
            // Non-source props are always dynamic (accessed as $$props.name)
            if data.scoping.prop_non_source_name(sym_id).is_some() {
                return true;
            }
            data.scoping.is_dynamic_by_id(sym_id)
        });
    }
    false
}
