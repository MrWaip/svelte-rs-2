use oxc_semantic::ScopeId;
use svelte_ast::{
    AnimateDirective, AttachTag, Attribute, AwaitBlock, BindDirective, ClassDirective,
    ConcatenationAttribute, ComponentNode, ConstTag, EachBlock,
    Element, ExpressionAttribute, ExpressionTag, HtmlTag, IfBlock, KeyBlock, NodeId, RenderTag,
    Shorthand, SpreadAttribute, StyleDirective, SvelteBoundary,
    TransitionDirective, UseDirective,
};
use crate::data::{AnalysisData, ExpressionKind};
use crate::walker::TemplateVisitor;

pub(crate) struct ReactivityVisitor {
    /// Track current element for directive visits that need to set needs_ref
    current_element_id: Option<NodeId>,
}

impl ReactivityVisitor {
    pub(crate) fn new() -> Self {
        Self { current_element_id: None }
    }

    fn mark_element_needs_ref(&self, data: &mut AnalysisData) {
        if let Some(el_id) = self.current_element_id {
            data.element_flags.needs_ref.insert(el_id);
        }
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

    fn visit_element(&mut self, el: &Element, _scope: ScopeId, _data: &mut AnalysisData) {
        self.current_element_id = Some(el.id);
    }

    fn leave_element(&mut self, _el: &Element, _scope: ScopeId, _data: &mut AnalysisData) {
        self.current_element_id = None;
    }

    fn visit_expression_attribute(&mut self, attr: &ExpressionAttribute, _scope: ScopeId, data: &mut AnalysisData) {
        if attr_id_is_dynamic(attr.id, data) {
            data.element_flags.dynamic_attrs.insert(attr.id);
            self.mark_element_needs_ref(data);
        }
    }

    fn visit_concatenation_attribute(&mut self, attr: &ConcatenationAttribute, _scope: ScopeId, data: &mut AnalysisData) {
        if attr_id_is_dynamic(attr.id, data) {
            data.element_flags.dynamic_attrs.insert(attr.id);
            self.mark_element_needs_ref(data);
        }
    }

    fn visit_spread_attribute(&mut self, attr: &SpreadAttribute, _scope: ScopeId, data: &mut AnalysisData) {
        if attr_id_is_dynamic(attr.id, data) {
            data.element_flags.dynamic_attrs.insert(attr.id);
            self.mark_element_needs_ref(data);
        }
    }

    fn visit_shorthand(&mut self, attr: &Shorthand, _scope: ScopeId, data: &mut AnalysisData) {
        if attr_id_is_dynamic(attr.id, data) {
            data.element_flags.dynamic_attrs.insert(attr.id);
            self.mark_element_needs_ref(data);
        }
    }

    fn visit_class_directive(&mut self, dir: &ClassDirective, _scope: ScopeId, data: &mut AnalysisData) {
        if attr_id_is_dynamic(dir.id, data) {
            data.element_flags.dynamic_attrs.insert(dir.id);
            self.mark_element_needs_ref(data);
            if let Some(el_id) = self.current_element_id {
                data.element_flags.has_dynamic_class_directives.insert(el_id);
            }
        }
    }

    fn visit_style_directive(&mut self, dir: &StyleDirective, _scope: ScopeId, data: &mut AnalysisData) {
        if attr_id_is_dynamic(dir.id, data) {
            data.element_flags.dynamic_attrs.insert(dir.id);
            self.mark_element_needs_ref(data);
        }
    }

    fn visit_bind_directive(&mut self, _dir: &BindDirective, _scope: ScopeId, data: &mut AnalysisData) {
        self.mark_element_needs_ref(data);
    }

    fn visit_use_directive(&mut self, _dir: &UseDirective, _scope: ScopeId, data: &mut AnalysisData) {
        self.mark_element_needs_ref(data);
    }

    fn visit_transition_directive(&mut self, _dir: &TransitionDirective, _scope: ScopeId, data: &mut AnalysisData) {
        self.mark_element_needs_ref(data);
    }

    fn visit_animate_directive(&mut self, _dir: &AnimateDirective, _scope: ScopeId, data: &mut AnalysisData) {
        self.mark_element_needs_ref(data);
    }

    fn visit_attach_tag(&mut self, _tag: &AttachTag, _scope: ScopeId, data: &mut AnalysisData) {
        self.mark_element_needs_ref(data);
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

    fn visit_component_node(&mut self, cn: &ComponentNode, _scope: ScopeId, data: &mut AnalysisData) {
        for attr in &cn.attributes {
            if component_attr_is_dynamic(attr, data) {
                data.element_flags.dynamic_attrs.insert(attr.id());
            }
        }
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
    if let Some(info) = data.attr_expressions.get(attr.id()) {
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
fn attr_id_is_dynamic(
    attr_id: NodeId,
    data: &AnalysisData,
) -> bool {
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
