use oxc_ast::ast::{Expression, IdentifierReference};
use oxc_ast_visit::Visit;
use oxc_semantic::{
    NodeId as OxcNodeId, Reference as OxcReference, ReferenceFlags as OxcReferenceFlags, ScopeId,
};
use smallvec::SmallVec;
use svelte_ast::{BindDirective, Component, NodeId, RenderTag};

use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::AnalysisData;
use crate::walker::{TemplateVisitor, VisitContext};

/// Create a CollectSymbolsVisitor for use after TemplateSemanticVisitor.
/// Consumes `ScopingBuilt` marker to enforce ordering.
pub(crate) fn make_visitor(
    component: &Component,
    _scoping: crate::types::markers::ScopingBuilt,
) -> CollectSymbolsVisitor<'_> {
    CollectSymbolsVisitor { component }
}

pub(crate) struct CollectSymbolsVisitor<'a> {
    component: &'a Component,
}

/// Collect resolved SymbolIds from OXC references set by TemplateSemanticVisitor.
/// Walks the parsed expression AST, reads reference_id → symbol_id from each
/// IdentifierReference, and stores deduplicated SymbolIds on ExpressionInfo.
fn collect_ref_symbols(expr: &Expression<'_>, scoping: &ComponentScoping) -> SmallVec<[SymbolId; 2]> {
    struct Collector<'s> {
        scoping: &'s ComponentScoping,
        symbols: SmallVec<[SymbolId; 2]>,
    }
    impl<'a> Visit<'a> for Collector<'_> {
        fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
            if let Some(ref_id) = ident.reference_id.get() {
                let reference = self.scoping.get_reference(ref_id);
                if let Some(sym_id) = reference.symbol_id() {
                    if !self.symbols.contains(&sym_id) {
                        self.symbols.push(sym_id);
                    }
                }
            }
        }
    }
    let mut collector = Collector { scoping, symbols: SmallVec::new() };
    collector.visit_expression(expr);
    collector.symbols
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

/// If `name` is `$X`, mark `X` as a store subscription.
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

impl TemplateVisitor for CollectSymbolsVisitor<'_> {
    fn visit_js_expression(
        &mut self,
        node_id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_>,
    ) {
        let symbols = collect_ref_symbols(expr, &ctx.data.scoping);
        // Store detection from expression identifiers
        struct StoreDetector<'s> {
            scoping: &'s mut ComponentScoping,
        }
        impl<'a> Visit<'a> for StoreDetector<'_> {
            fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
                try_mark_store(ident.name.as_str(), self.scoping);
            }
        }
        let mut detector = StoreDetector { scoping: &mut ctx.data.scoping };
        detector.visit_expression(expr);

        if ctx.parent().is_some_and(|p| p.kind.is_attr()) {
            if let Some(info) = ctx.data.attr_expressions.get_mut(node_id) {
                info.ref_symbols = symbols;
            }
        } else if let Some(info) = ctx.data.expressions.get_mut(node_id) {
            info.ref_symbols = symbols;
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

impl CollectSymbolsVisitor<'_> {
    fn resolve_bind(&self, dir: &BindDirective, scope: ScopeId, data: &mut AnalysisData) {
        let name = if dir.shorthand {
            dir.name.as_str()
        } else if let Some(span) = dir.expression_span {
            self.component.source_text(span).trim()
        } else {
            return;
        };
        // bind: is always a write reference — create OXC reference for mutation tracking
        if let Some(sym_id) = data.scoping.find_binding(scope, name) {
            let mut reference = OxcReference::new(OxcNodeId::DUMMY, scope, OxcReferenceFlags::Write);
            reference.set_symbol_id(sym_id);
            let ref_id = data.scoping.create_reference(reference);
            data.scoping.add_resolved_reference(sym_id, ref_id);
        }
    }
}
