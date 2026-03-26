use oxc_semantic::{ReferenceFlags as OxcReferenceFlags, ScopeId};
use svelte_ast::{BindDirective, Component, NodeId, RenderTag};
use svelte_span::Span;

use crate::ComponentScoping;
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
            if r.flags.is_write() {
                data.scoping.register_template_reference(sym_id, OxcReferenceFlags::Write);
            }
        }
    }
    // Store detection: $X → mark X as store (after mutable borrow of info is released)
    let info = data.expressions.get(node_id).unwrap();
    for r in &info.references {
        try_mark_store(&r.name, &mut data.scoping);
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
            if r.flags.is_write() {
                data.scoping.register_template_reference(sym_id, OxcReferenceFlags::Write);
            }
        }
    }
    let info = data.attr_expressions.get(attr_id).unwrap();
    for r in &info.references {
        try_mark_store(&r.name, &mut data.scoping);
    }
}

/// If `name` is `$X`, mark `X` as a store subscription (if it's a root-scope non-rune binding).
fn try_mark_store(name: &str, scoping: &mut ComponentScoping) {
    if !name.starts_with('$') || name.len() <= 1 {
        return;
    }
    let base = &name[1..];
    let root = scoping.root_scope_id();
    let Some(sym_id) = scoping.find_binding(root, base) else {
        return;
    };
    if !scoping.is_rune(sym_id) && !scoping.is_store(sym_id) {
        scoping.mark_store(sym_id, base.to_string());
    }
}

/// Resolve script-level store subscriptions from OXC unresolved references.
/// Called after the template walk, when ComponentScoping is fully built.
pub(crate) fn resolve_script_stores(data: &mut AnalysisData) {
    let candidates: Vec<String> = match &data.script {
        Some(s) => s.store_candidates.iter().map(|n| format!("${n}")).collect(),
        None => return,
    };
    for name in &candidates {
        try_mark_store(name, &mut data.scoping);
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
