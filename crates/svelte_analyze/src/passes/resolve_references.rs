use oxc_semantic::{ReferenceFlags as OxcReferenceFlags, ScopeId};
use svelte_ast::{BindDirective, Component, NodeId, RenderTag};
use svelte_span::Span;

use crate::types::data::AnalysisData;
use crate::walker::{TemplateVisitor, VisitContext};

/// Create a ResolveReferencesVisitor for use in a composite walk.
/// Consumes `ScopingBuilt` marker to enforce ordering.
pub(crate) fn make_visitor(
    component: &Component,
    _scoping: crate::types::markers::ScopingBuilt,
) -> ResolveReferencesVisitor<'_> {
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
            if r.flags == crate::types::data::ReferenceFlags::Write
                || r.flags == crate::types::data::ReferenceFlags::ReadWrite
            {
                data.scoping.register_template_reference(sym_id, OxcReferenceFlags::Write);
            }
        }
    }
}

/// Resolve references in an attribute expression.
fn resolve_attr_refs(attr_id: NodeId, scope: ScopeId, data: &mut AnalysisData) {
    let Some(info) = data.attr_expressions.get_mut(attr_id) else {
        return;
    };
    for r in &mut info.references {
        if let Some(sym_id) = data.scoping.find_binding(scope, &r.name) {
            r.symbol_id = Some(sym_id);
            if r.flags == crate::types::data::ReferenceFlags::Write
                || r.flags == crate::types::data::ReferenceFlags::ReadWrite
            {
                data.scoping.register_template_reference(sym_id, OxcReferenceFlags::Write);
            }
        }
    }
}

impl TemplateVisitor for ResolveReferencesVisitor<'_> {
    fn visit_expression(&mut self, node_id: NodeId, _span: Span, ctx: &mut VisitContext<'_>) {
        if ctx.parent().is_some_and(|p| p.kind.is_attr()) {
            resolve_attr_refs(node_id, ctx.scope, ctx.data);
        } else {
            resolve_expr_refs(node_id, ctx.scope, ctx.data);
        }
    }

    fn visit_bind_directive(&mut self, dir: &BindDirective, ctx: &mut VisitContext<'_>) {
        self.resolve_bind(dir, ctx.scope, ctx.data);
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, ctx: &mut VisitContext<'_>) {
        let name = ctx.data.render_tag_callee_name.get(tag.id).cloned();
        if let Some(name) = name {
            if let Some(sym_id) = ctx.data.scoping.find_binding(ctx.scope, &name) {
                ctx.data.render_tag_callee_sym.insert(tag.id, sym_id);
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
            data.scoping.register_template_reference(sym_id, OxcReferenceFlags::Write);
        }
    }
}
