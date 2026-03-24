use oxc_semantic::{ReferenceFlags as OxcReferenceFlags, ScopeId};
use svelte_ast::{
    AnimateDirective, AttachTag, Attribute, BindDirective, ClassDirective, ComponentNode,
    ConcatenationAttribute, EachBlock, ExpressionAttribute, ExpressionTag, HtmlTag,
    IfBlock, KeyBlock, NodeId, OnDirectiveLegacy, RenderTag, Shorthand, SpreadAttribute,
    StyleDirective, SvelteBody, SvelteBoundary, SvelteDocument, SvelteElement, SvelteWindow,
    TransitionDirective, UseDirective,
};

use crate::data::AnalysisData;
use crate::walker::TemplateVisitor;

use svelte_ast::Component;

/// Create a ResolveReferencesVisitor for use in a composite walk.
/// Consumes `ScopingBuilt` marker to enforce ordering.
pub(crate) fn make_visitor(component: &Component, _scoping: crate::markers::ScopingBuilt) -> ResolveReferencesVisitor<'_> {
    ResolveReferencesVisitor { component }
}

pub(crate) struct ResolveReferencesVisitor<'a> {
    component: &'a Component,
}

/// Resolve references in an expression (by NodeId in `data.expressions`).
/// NLL allows split borrows: `data.expressions` and `data.scoping` are separate fields.
fn resolve_expr_refs(node_id: NodeId, scope: ScopeId, data: &mut AnalysisData) {
    let Some(info) = data.expressions.get_mut(node_id) else {
        return;
    };
    for r in &mut info.references {
        if let Some(sym_id) = data.scoping.find_binding(scope, &r.name) {
            r.symbol_id = Some(sym_id);
            if r.flags == crate::data::ReferenceFlags::Write
                || r.flags == crate::data::ReferenceFlags::ReadWrite
            {
                data.scoping
                    .register_template_reference(sym_id, OxcReferenceFlags::Write);
            }
        }
    }
}

/// Resolve references in an attribute expression.
fn resolve_attr_refs(
    attr_id: NodeId,
    scope: ScopeId,
    data: &mut AnalysisData,
) {
    let Some(info) = data.attr_expressions.get_mut(attr_id) else {
        return;
    };
    for r in &mut info.references {
        if let Some(sym_id) = data.scoping.find_binding(scope, &r.name) {
            r.symbol_id = Some(sym_id);
            if r.flags == crate::data::ReferenceFlags::Write
                || r.flags == crate::data::ReferenceFlags::ReadWrite
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

        // Store the scope for this render tag so we can resolve dynamic callees later
        // (after props analysis populates is_prop_source).
        if let Some(name) = data.render_tag_callee_name.get(tag.id) {
            if let Some(sym_id) = data.scoping.find_binding(scope, name) {
                data.render_tag_callee_sym.insert(tag.id, sym_id);
            }
        }
    }

    fn visit_html_tag(&mut self, tag: &HtmlTag, scope: ScopeId, data: &mut AnalysisData) {
        resolve_expr_refs(tag.id, scope, data);
    }

    fn visit_expression_attribute(&mut self, attr: &ExpressionAttribute, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(attr.id, scope, data);
    }
    fn visit_concatenation_attribute(&mut self, attr: &ConcatenationAttribute, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(attr.id, scope, data);
    }
    fn visit_spread_attribute(&mut self, attr: &SpreadAttribute, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(attr.id, scope, data);
    }
    fn visit_shorthand(&mut self, attr: &Shorthand, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(attr.id, scope, data);
    }
    fn visit_class_directive(&mut self, dir: &ClassDirective, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(dir.id, scope, data);
    }
    fn visit_style_directive(&mut self, dir: &StyleDirective, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(dir.id, scope, data);
    }
    fn visit_bind_directive(&mut self, dir: &BindDirective, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(dir.id, scope, data);
        self.resolve_bind(dir, scope, data);
    }
    fn visit_use_directive(&mut self, dir: &UseDirective, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(dir.id, scope, data);
    }
    fn visit_on_directive_legacy(&mut self, dir: &OnDirectiveLegacy, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(dir.id, scope, data);
    }
    fn visit_transition_directive(&mut self, dir: &TransitionDirective, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(dir.id, scope, data);
    }
    fn visit_animate_directive(&mut self, dir: &AnimateDirective, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(dir.id, scope, data);
    }
    fn visit_attach_tag(&mut self, tag: &AttachTag, scope: ScopeId, data: &mut AnalysisData) {
        resolve_attr_refs(tag.id, scope, data);
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, scope: ScopeId, data: &mut AnalysisData) {
        for attr in &cn.attributes {
            resolve_attr_refs(attr.id(), scope, data);
            if let Attribute::BindDirective(dir) = attr {
                self.resolve_bind(dir, scope, data);
            }
        }
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
        for attr in &el.attributes {
            resolve_attr_refs(attr.id(), scope, data);
            if let Attribute::BindDirective(dir) = attr {
                self.resolve_bind(dir, scope, data);
            }
        }
    }

    fn visit_svelte_window(
        &mut self,
        w: &SvelteWindow,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        for attr in &w.attributes {
            resolve_attr_refs(attr.id(), scope, data);
            if let Attribute::BindDirective(dir) = attr {
                self.resolve_bind(dir, scope, data);
            }
        }
    }

    fn visit_svelte_document(
        &mut self,
        doc: &SvelteDocument,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        for attr in &doc.attributes {
            resolve_attr_refs(attr.id(), scope, data);
            if let Attribute::BindDirective(dir) = attr {
                self.resolve_bind(dir, scope, data);
            }
        }
    }

    fn visit_svelte_body(
        &mut self,
        body: &SvelteBody,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        for attr in &body.attributes {
            resolve_attr_refs(attr.id(), scope, data);
        }
    }

    fn visit_svelte_boundary(
        &mut self,
        boundary: &SvelteBoundary,
        scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        for attr in &boundary.attributes {
            resolve_attr_refs(attr.id(), scope, data);
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
