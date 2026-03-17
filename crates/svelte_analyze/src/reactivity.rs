use oxc_semantic::ScopeId;
use svelte_ast::{
    Attribute, BindDirective, ComponentNode, ConstTag, EachBlock, Element, ExpressionTag, HtmlTag,
    IfBlock, KeyBlock, NodeId, RenderTag, SnippetBlock, UseDirective,
};
use crate::data::AnalysisData;
use crate::walker::TemplateVisitor;

pub(crate) struct ReactivityVisitor {
    /// Stack of snippet NodeIds for nested snippets (avoids cloning param vecs).
    snippet_id_stack: Vec<NodeId>,
}

impl ReactivityVisitor {
    pub(crate) fn new() -> Self {
        Self {
            snippet_id_stack: Vec::new(),
        }
    }

    fn current_snippet_params<'a>(&self, data: &'a AnalysisData) -> &'a [String] {
        self.snippet_id_stack
            .last()
            .and_then(|id| data.snippets.params.get(id))
            .map_or(&[], |v| v.as_slice())
    }

    /// Check if an attribute expression references a snippet parameter.
    fn attr_refs_snippet_param(&self, owner_id: &NodeId, attr_idx: usize, data: &AnalysisData) -> bool {
        let params = self.current_snippet_params(data);
        if params.is_empty() {
            return false;
        }
        if let Some(info) = data.attr_expressions.get(&(*owner_id, attr_idx)) {
            return info.references.iter().any(|r| params.iter().any(|p| p == &r.name));
        }
        false
    }

    fn expr_is_dynamic(
        &self,
        node_id: &NodeId,
        data: &AnalysisData,
    ) -> bool {
        if let Some(info) = data.expressions.get(node_id) {
            let params = self.current_snippet_params(data);
            return info.references.iter().any(|r| {
                // Snippet params are always dynamic (they're thunks)
                if params.iter().any(|p| p == &r.name) {
                    return true;
                }
                // Store subscriptions ($count) are always dynamic
                if is_store_ref(&r.name, data) {
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

    fn visit_attribute(&mut self, attr: &Attribute, idx: usize, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        let is_dynamic = attr_is_dynamic(attr, idx, el.id, data)
            || self.attr_refs_snippet_param(&el.id, idx, data);
        if is_dynamic {
            data.element_flags.dynamic_attrs.insert((el.id, idx));
            data.element_flags.needs_ref.insert(el.id);
        }
    }

    fn visit_bind_directive(&mut self, _dir: &BindDirective, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        data.element_flags.needs_ref.insert(el.id);
    }

    fn visit_use_directive(&mut self, _dir: &UseDirective, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
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

    fn visit_component_attribute(&mut self, attr: &Attribute, idx: usize, cn: &ComponentNode, _scope: ScopeId, data: &mut AnalysisData) {
        if component_attr_is_dynamic(attr, idx, cn.id, data) {
            data.element_flags.dynamic_attrs.insert((cn.id, idx));
        }
    }

    fn visit_snippet_block(&mut self, block: &SnippetBlock, _scope: ScopeId, _data: &mut AnalysisData) {
        self.snippet_id_stack.push(block.id);
    }

    fn leave_snippet_block(&mut self, _block: &SnippetBlock, _scope: ScopeId, _data: &mut AnalysisData) {
        self.snippet_id_stack.pop();
    }
}

// Component props are dynamic for *any* rune reference (mutated or not).
// This matches Svelte's `has_state` semantics — component props use getters
// so the child component can re-read the value reactively.
fn component_attr_is_dynamic(
    attr: &Attribute,
    attr_idx: usize,
    owner_id: NodeId,
    data: &AnalysisData,
) -> bool {
    if matches!(
        attr,
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
    ) {
        return false;
    }
    if let Some(info) = data.attr_expressions.get(&(owner_id, attr_idx)) {
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

/// Check if a reference name is a store subscription (`$X` where X is in store_subscriptions).
fn is_store_ref(name: &str, data: &AnalysisData) -> bool {
    if name.starts_with('$') && name.len() > 1 {
        return data.store_subscriptions.contains(&name[1..]);
    }
    false
}

// Attributes are dynamic for any rune reference or prop access — they can
// change between renders and need a template_effect to stay up-to-date.
fn attr_is_dynamic(
    attr: &Attribute,
    attr_idx: usize,
    el_id: NodeId,
    data: &AnalysisData,
) -> bool {
    if matches!(
        attr,
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
    ) {
        return false;
    }
    if let Some(info) = data.attr_expressions.get(&(el_id, attr_idx)) {
        return info.references.iter().any(|r| {
            // Non-source props are always dynamic (accessed as $$props.name)
            if let Some(pa) = &data.props {
                if pa.props.iter().any(|p| p.local_name == r.name && !p.is_prop_source) {
                    return true;
                }
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
