use oxc_semantic::ScopeId;
use svelte_ast::{
    Attribute, BindDirective, ComponentNode, EachBlock, Element, ExpressionTag, IfBlock, NodeId,
    RenderTag, SnippetBlock,
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
            .and_then(|id| data.snippet_params.get(id))
            .map_or(&[], |v| v.as_slice())
    }

    fn expr_is_dynamic(
        &self,
        node_id: &NodeId,
        data: &AnalysisData,
        scope: ScopeId,
    ) -> bool {
        if let Some(info) = data.expressions.get(node_id) {
            let params = self.current_snippet_params(data);
            return info.references.iter().any(|r| {
                // Snippet params are always dynamic (they're thunks)
                if params.iter().any(|p| p == &r.name) {
                    return true;
                }
                data.scoping.is_dynamic_ref(scope, &r.name)
            });
        }
        false
    }
}

impl TemplateVisitor for ReactivityVisitor {
    fn visit_expression_tag(&mut self, tag: &ExpressionTag, scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&tag.id, data, scope) {
            data.dynamic_nodes.insert(tag.id);
        }
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&tag.id, data, scope) {
            data.dynamic_nodes.insert(tag.id);
        }
    }

    fn visit_attribute(&mut self, attr: &Attribute, idx: usize, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
        if attr_is_dynamic(attr, idx, el.id, data, scope) {
            data.dynamic_attrs.insert((el.id, idx));
            data.node_needs_ref.insert(el.id);
        }
    }

    fn visit_bind_directive(&mut self, _dir: &BindDirective, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        // Bind directives always need a DOM ref
        data.node_needs_ref.insert(el.id);
    }

    fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {
        if self.expr_is_dynamic(&block.id, data, scope) {
            data.dynamic_nodes.insert(block.id);
        }
    }

    fn visit_each_block(&mut self, block: &EachBlock, body_scope: ScopeId, data: &mut AnalysisData) {
        // body_scope is fine for the expression check — find_binding walks up parent scopes,
        // so `items` in `{#each items as item}` resolves correctly from the child scope.
        if self.expr_is_dynamic(&block.id, data, body_scope) {
            data.dynamic_nodes.insert(block.id);
        }
    }

    fn visit_component_attribute(&mut self, attr: &Attribute, idx: usize, cn: &ComponentNode, scope: ScopeId, data: &mut AnalysisData) {
        if component_attr_is_dynamic(attr, idx, cn.id, data, scope) {
            data.dynamic_attrs.insert((cn.id, idx));
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
    current_scope: ScopeId,
) -> bool {
    if matches!(
        attr,
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
    ) {
        return false;
    }
    if let Some(info) = data.attr_expressions.get(&(owner_id, attr_idx)) {
        return info.references.iter().any(|r| {
            // Any rune (even unmutated $state) or non-root binding makes it dynamic
            if let Some(sym_id) = data.scoping.find_binding(current_scope, &r.name) {
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
    attr_idx: usize,
    el_id: NodeId,
    data: &AnalysisData,
    current_scope: ScopeId,
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
            // Delegate to is_dynamic_ref for rune/scope checks
            data.scoping.is_dynamic_ref(current_scope, &r.name)
        });
    }
    false
}
