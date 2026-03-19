use oxc_semantic::ScopeId;
use svelte_ast::{
    AnimateDirective, AttachTag, Attribute, AwaitBlock, BindDirective, ComponentNode, ConstTag, EachBlock,
    Element, ExpressionTag, HtmlTag, IfBlock, KeyBlock, NodeId, RenderTag, SvelteBoundary,
    TransitionDirective, UseDirective,
};
use crate::data::AnalysisData;
use crate::walker::TemplateVisitor;

pub(crate) struct ReactivityVisitor;

impl ReactivityVisitor {
    fn expr_is_dynamic(
        &self,
        node_id: &NodeId,
        data: &AnalysisData,
    ) -> bool {
        if let Some(info) = data.expressions.get(node_id) {
            return info.references.iter().any(|r| {
                // Store subscriptions ($count) are always dynamic
                if data.scoping.is_store_ref(&r.name) {
                    return true;
                }
                if let Some(sym_id) = r.symbol_id {
                    data.scoping.is_dynamic_by_id(sym_id)
                } else {
                    false
                }
            });
        }
        false
    }
}

impl TemplateVisitor for ReactivityVisitor {
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, _scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&tag.id, data) {
            data.dynamic_nodes.insert(tag.id);
        }
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, _scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&tag.id, data) {
            data.dynamic_nodes.insert(tag.id);
        }
    }

    fn visit_html_tag(&mut self, tag: &HtmlTag, _scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&tag.id, data) {
            data.dynamic_nodes.insert(tag.id);
        }
    }

    fn visit_const_tag(&mut self, tag: &ConstTag, _scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&tag.id, data) {
            data.dynamic_nodes.insert(tag.id);
        }
    }

    fn visit_attribute(&mut self, attr: &Attribute, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        let attr_id = attr.id();
        if attr_is_dynamic(attr, data) {
            data.element_flags.dynamic_attrs.insert(attr_id);
            data.element_flags.needs_ref.insert(el.id);
        }
    }

    fn visit_bind_directive(&mut self, _dir: &BindDirective, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        data.element_flags.needs_ref.insert(el.id);
    }

    fn visit_use_directive(&mut self, _dir: &UseDirective, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        data.element_flags.needs_ref.insert(el.id);
    }

    fn visit_transition_directive(&mut self, _dir: &TransitionDirective, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        data.element_flags.needs_ref.insert(el.id);
    }

    fn visit_animate_directive(&mut self, _dir: &AnimateDirective, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        data.element_flags.needs_ref.insert(el.id);
    }

    fn visit_attach_tag(&mut self, _tag: &AttachTag, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        data.element_flags.needs_ref.insert(el.id);
    }

    fn visit_if_block(&mut self, block: &IfBlock, _scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&block.id, data) {
            data.dynamic_nodes.insert(block.id);
        }
    }

    fn visit_each_block(&mut self, block: &EachBlock, _parent_scope: ScopeId, _body_scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&block.id, data) {
            data.dynamic_nodes.insert(block.id);
        }
    }

    fn visit_key_block(&mut self, block: &KeyBlock, _scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&block.id, data) {
            data.dynamic_nodes.insert(block.id);
        }
    }

    fn visit_await_block(&mut self, block: &AwaitBlock, _scope: ScopeId, data: &mut AnalysisData) {
        // Await blocks are always dynamic
        data.dynamic_nodes.insert(block.id);
    }

    fn visit_component_attribute(&mut self, attr: &Attribute, cn: &ComponentNode, _scope: ScopeId, data: &mut AnalysisData) {
        if component_attr_is_dynamic(attr, data) {
            data.element_flags.dynamic_attrs.insert(attr.id());
        }
        let _ = cn;
    }

    fn visit_svelte_boundary(&mut self, boundary: &SvelteBoundary, _scope: ScopeId, data: &mut AnalysisData) {
        // Boundary attributes use the same has_state semantics as component props —
        // reactive expressions get getters so the boundary can re-read reactively.
        for attr in &boundary.attributes {
            if component_attr_is_dynamic(attr, data) {
                data.element_flags.dynamic_attrs.insert(attr.id());
            }
        }
    }

}

// Component props are dynamic for *any* rune reference (mutated or not).
// This matches Svelte's `has_state` semantics — component props use getters
// so the child component can re-read the value reactively.
fn component_attr_is_dynamic(
    attr: &Attribute,
    data: &AnalysisData,
) -> bool {
    if matches!(
        attr,
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
    ) {
        return false;
    }
    if let Some(info) = data.attr_expressions.get(&attr.id()) {
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


// Attributes are dynamic for any rune reference or prop access — they can
// change between renders and need a template_effect to stay up-to-date.
fn attr_is_dynamic(
    attr: &Attribute,
    data: &AnalysisData,
) -> bool {
    if matches!(
        attr,
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
    ) {
        return false;
    }
    if let Some(info) = data.attr_expressions.get(&attr.id()) {
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
