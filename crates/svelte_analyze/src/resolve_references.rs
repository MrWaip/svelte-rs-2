use oxc_semantic::{ReferenceFlags as OxcReferenceFlags, ScopeId};
use svelte_ast::{
    Attribute, BindDirective, ComponentNode, EachBlock, Element, ExpressionTag, HtmlTag, IfBlock,
    KeyBlock, NodeId, RenderTag, SvelteElement,
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
}

struct ResolveReferencesVisitor<'a> {
    component: &'a Component,
}

/// Resolve references in an expression (by NodeId in `data.expressions`).
/// NLL allows split borrows: `data.expressions` and `data.scoping` are separate fields.
fn resolve_expr_refs(node_id: NodeId, scope: ScopeId, data: &mut AnalysisData) {
    let Some(info) = data.expressions.get_mut(&node_id) else {
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
}

/// Resolve references in an attribute expression.
fn resolve_attr_refs(
    owner_id: NodeId,
    attr_idx: usize,
    scope: ScopeId,
    data: &mut AnalysisData,
) {
    let key = (owner_id, attr_idx);
    let Some(info) = data.attr_expressions.get_mut(&key) else {
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
}

impl TemplateVisitor for ResolveReferencesVisitor<'_> {
    fn visit_expression_tag(
        &mut self,
        tag: &ExpressionTag,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        resolve_expr_refs(tag.id, scope, data);
    }

    fn visit_if_block(&mut self, block: &IfBlock, scope: ScopeId, data: &mut AnalysisData) {
        resolve_expr_refs(block.id, scope, data);
    }

    fn visit_each_block(
        &mut self,
        block: &EachBlock,
        parent_scope: ScopeId,
        _body_scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        // Collection expression belongs to parent scope, not the each-block's child scope.
        resolve_expr_refs(block.id, parent_scope, data);
    }

    fn visit_key_block(&mut self, block: &KeyBlock, scope: ScopeId, data: &mut AnalysisData) {
        resolve_expr_refs(block.id, scope, data);
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, scope: ScopeId, data: &mut AnalysisData) {
        resolve_expr_refs(tag.id, scope, data);
    }

    fn visit_html_tag(&mut self, tag: &HtmlTag, scope: ScopeId, data: &mut AnalysisData) {
        resolve_expr_refs(tag.id, scope, data);
    }

    fn visit_attribute(
        &mut self,
        _attr: &Attribute,
        idx: usize,
        el: &Element,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        resolve_attr_refs(el.id, idx, scope, data);
    }

    fn visit_component_attribute(
        &mut self,
        _attr: &Attribute,
        idx: usize,
        cn: &ComponentNode,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        resolve_attr_refs(cn.id, idx, scope, data);
    }

    fn visit_bind_directive(
        &mut self,
        dir: &BindDirective,
        _el: &Element,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        self.resolve_bind(dir, scope, data);
    }

    fn visit_svelte_element(
        &mut self,
        el: &SvelteElement,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        // Resolve tag expression references (skip for static string tags)
        if !el.static_tag {
            resolve_expr_refs(el.id, scope, data);
        }
        // Walk attributes — resolve expression refs and bind directives
        for (idx, attr) in el.attributes.iter().enumerate() {
            resolve_attr_refs(el.id, idx, scope, data);
            if let Attribute::BindDirective(dir) = attr {
                self.resolve_bind(dir, scope, data);
            }
        }
    }
}

impl ResolveReferencesVisitor<'_> {
    fn resolve_bind(&self, dir: &BindDirective, scope: ScopeId, data: &mut AnalysisData) {
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
