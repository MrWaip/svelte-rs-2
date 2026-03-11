use oxc_semantic::ScopeId;
use svelte_ast::{
    Attribute, EachBlock, Element, ExpressionTag, IfBlock, NodeId, RenderTag, SnippetBlock,
};

use crate::data::AnalysisData;
use crate::walker::TemplateVisitor;

pub(crate) struct ReactivityVisitor {
    /// Stack of snippet parameter names for nested snippets.
    snippet_params_stack: Vec<Vec<String>>,
}

impl ReactivityVisitor {
    pub(crate) fn new() -> Self {
        Self {
            snippet_params_stack: Vec::new(),
        }
    }

    fn current_snippet_params(&self) -> &[String] {
        self.snippet_params_stack.last().map_or(&[], |v| v.as_slice())
    }

    fn expr_is_dynamic(
        &self,
        node_id: &NodeId,
        data: &AnalysisData,
        scope: ScopeId,
    ) -> bool {
        if let Some(info) = data.expressions.get(node_id) {
            let params = self.current_snippet_params();
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

    fn visit_element(&mut self, el: &Element, scope: ScopeId, data: &mut AnalysisData) {
        let mut needs_ref = false;
        for (attr_idx, attr) in el.attributes.iter().enumerate() {
            if attr_is_dynamic(attr, attr_idx, el.id, data, scope) {
                data.dynamic_attrs.insert((el.id, attr_idx));
                needs_ref = true;
            }
            if matches!(attr, Attribute::BindDirective(_)) {
                needs_ref = true;
            }
        }
        if needs_ref {
            data.node_needs_ref.insert(el.id);
        }
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

    fn visit_snippet_block(&mut self, block: &SnippetBlock, _scope: ScopeId, data: &mut AnalysisData) {
        let params = data
            .snippet_params
            .get(&block.id)
            .cloned()
            .unwrap_or_default();
        self.snippet_params_stack.push(params);
    }

    fn leave_snippet_block(&mut self, _block: &SnippetBlock, _scope: ScopeId, _data: &mut AnalysisData) {
        self.snippet_params_stack.pop();
    }
}

// Attributes are dynamic only for *mutated* runes — unlike expressions where
// any rune makes the expression dynamic. This is because attributes generate
// `$.attr_effect()` which requires a signal that can change.
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
            let root = data.scoping.root_scope_id();
            if let Some(sym_id) = data.scoping.find_binding(current_scope, &r.name) {
                // Non-root scope binding (each-block var) is always dynamic
                if data.scoping.symbol_scope_id(sym_id) != root {
                    return true;
                }
                // Root-scope rune that is mutated
                if data.scoping.is_rune(sym_id) && data.scoping.is_mutated(sym_id) {
                    return true;
                }
            }
            false
        });
    }
    false
}
