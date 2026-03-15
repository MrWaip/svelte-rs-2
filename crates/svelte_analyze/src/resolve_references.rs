use oxc_semantic::{ReferenceFlags as OxcReferenceFlags, ScopeId};
use svelte_ast::{
    Attribute, BindDirective, ComponentNode, EachBlock, Element, ExpressionTag, HtmlTag, IfBlock,
    KeyBlock, NodeId, RenderTag,
};

use crate::data::AnalysisData;
use crate::walker::{self, TemplateVisitor};

use svelte_ast::Component;

/// Resolve all template references to SymbolIds and register write references in OXC Scoping.
/// After this pass, `symbol_is_mutated()` covers both script and template mutations.
pub fn resolve_references(component: &Component, data: &mut AnalysisData) {
    let root_scope = data.scoping.root_scope_id();
    let mut visitor = ResolveReferencesVisitor { component };
    walker::walk_template(&component.fragment, data, root_scope, &mut visitor);
    data.cache_rune_sets();
}

struct ResolveReferencesVisitor<'a> {
    component: &'a Component,
}

impl ResolveReferencesVisitor<'_> {
    /// Resolve references in an expression (by NodeId in `data.expressions`).
    fn resolve_expr_refs(&self, node_id: NodeId, scope: ScopeId, data: &mut AnalysisData) {
        let Some(mut info) = data.expressions.remove(&node_id) else {
            return;
        };
        for r in &mut info.references {
            if let Some(sym_id) = data.scoping.find_binding(scope, &r.name) {
                r.symbol_id = Some(sym_id);
                if r.flags == svelte_js::ReferenceFlags::Write
                    || r.flags == svelte_js::ReferenceFlags::ReadWrite
                {
                    data.scoping
                        .register_template_reference(sym_id, OxcReferenceFlags::Write);
                }
            }
        }
        data.expressions.insert(node_id, info);
    }

    /// Resolve references in an attribute expression.
    fn resolve_attr_refs(
        &self,
        owner_id: NodeId,
        attr_idx: usize,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        let key = (owner_id, attr_idx);
        let Some(mut info) = data.attr_expressions.remove(&key) else {
            return;
        };
        for r in &mut info.references {
            if let Some(sym_id) = data.scoping.find_binding(scope, &r.name) {
                r.symbol_id = Some(sym_id);
                if r.flags == svelte_js::ReferenceFlags::Write
                    || r.flags == svelte_js::ReferenceFlags::ReadWrite
                {
                    data.scoping
                        .register_template_reference(sym_id, OxcReferenceFlags::Write);
                }
            }
        }
        data.attr_expressions.insert(key, info);
    }
}

impl TemplateVisitor for ResolveReferencesVisitor<'_> {
    fn visit_expression_tag(
        &mut self,
        tag: &ExpressionTag,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.resolve_expr_refs(tag.id, scope, data);
    }

    fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {
        self.resolve_expr_refs(block.id, scope, data);
    }

    fn visit_each_block(
        &mut self,
        block: &EachBlock,
        body_scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        // The collection expression uses the parent scope (body_scope's parent),
        // but find_binding walks up, so body_scope works correctly.
        self.resolve_expr_refs(block.id, body_scope, data);
    }

    fn visit_key_block(&mut self, block: &KeyBlock, scope: ScopeId, data: &mut AnalysisData) {
        self.resolve_expr_refs(block.id, scope, data);
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {
        self.resolve_expr_refs(tag.id, scope, data);
    }

    fn visit_html_tag(&mut self, tag: &HtmlTag, scope: ScopeId, data: &mut AnalysisData) {
        self.resolve_expr_refs(tag.id, scope, data);
    }

    fn visit_attribute(
        &mut self,
        _attr: &Attribute,
        idx: usize,
        el: &Element,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.resolve_attr_refs(el.id, idx, scope, data);
    }

    fn visit_component_attribute(
        &mut self,
        _attr: &Attribute,
        idx: usize,
        cn: &ComponentNode,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.resolve_attr_refs(cn.id, idx, scope, data);
    }

    fn visit_bind_directive(
        &mut self,
        dir: &BindDirective,
        _el: &Element,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        let name = if dir.shorthand {
            dir.name.as_str()
        } else if let Some(span) = dir.expression_span {
            self.component.source_text(span).trim()
        } else {
            return;
        };
        if let Some(sym_id) = data.scoping.find_binding(scope, name) {
            // bind: is always a ReadWrite reference
            data.scoping
                .register_template_reference(sym_id, OxcReferenceFlags::Write);
        }
    }
}
