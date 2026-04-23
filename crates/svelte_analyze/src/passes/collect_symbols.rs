use oxc_ast::ast::{Expression, IdentifierReference};
use oxc_ast_visit::Visit;
use smallvec::SmallVec;
use svelte_ast::{Attribute, EachBlock, NodeId, RenderTag, StyleDirectiveValue};
use svelte_span::Span;

use crate::passes::js_analyze;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{AnalysisData, ExpressionInfo, ExpressionKind};
use crate::walker::{TemplateVisitor, VisitContext};

/// Create a CollectSymbolsVisitor for use after TemplateSemanticVisitor.
/// Consumes `ScopingBuilt` marker to enforce ordering.
pub(crate) fn make_visitor(_scoping: crate::types::markers::ScopingBuilt) -> CollectSymbolsVisitor {
    CollectSymbolsVisitor {
        pending_render_tag: None,
        pending_shorthand: None,
        pending_clsx: false,
    }
}

pub(crate) struct CollectSymbolsVisitor {
    pending_render_tag: Option<NodeId>,
    pending_shorthand: Option<(NodeId, String)>,
    pending_clsx: bool,
}

// ---------------------------------------------------------------------------
// TemplateVisitor impl
// ---------------------------------------------------------------------------

impl TemplateVisitor for CollectSymbolsVisitor {
    fn visit_each_block(&mut self, block: &EachBlock, ctx: &mut VisitContext<'_, '_>) {
        if let Some(key_id) = block.key_id {
            ctx.data
                .blocks
                .each_context
                .record_key_node_id(block.id, key_id);
        }
    }

    fn visit_expression(&mut self, _node_id: NodeId, _span: Span, _ctx: &mut VisitContext<'_, '_>) {
    }

    fn visit_js_expression(
        &mut self,
        node_id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let mut info = build_expression_info(expr, &mut ctx.data.scoping);
        if info.uses_legacy_slots() {
            ctx.data.output.needs_sanitized_legacy_slots = true;
        }
        classify_shorthand(node_id, expr, &mut self.pending_shorthand, ctx.data);
        classify_clsx(node_id, expr, &mut self.pending_clsx, ctx.data);
        classify_render_tag(
            node_id,
            expr,
            &mut info,
            &mut self.pending_render_tag,
            ctx.data,
        );
        store_expression_info(node_id, info, ctx);
    }

    fn visit_render_tag(&mut self, tag: &RenderTag, _ctx: &mut VisitContext<'_, '_>) {
        self.pending_render_tag = Some(tag.id);
    }

    fn visit_const_tag(&mut self, tag: &svelte_ast::ConstTag, ctx: &mut VisitContext<'_, '_>) {
        // Build ExpressionInfo for the @const init expression so that
        // mark_const_tag_bindings can read ref_symbols for derived_deps.
        // Borrow `parsed` directly (the field, not the ctx.parsed() method)
        // to avoid pinning all of ctx as immutable while we mutate
        // ctx.data.scoping below.
        let init_expr =
            ctx.parsed
                .and_then(|p| p.stmt(tag.decl.id()))
                .and_then(|stmt| match stmt {
                    oxc_ast::ast::Statement::VariableDeclaration(decl) => {
                        decl.declarations.first().and_then(|d| d.init.as_ref())
                    }
                    _ => None,
                });
        if let Some(init_expr) = init_expr {
            let info = build_expression_info(init_expr, &mut ctx.data.scoping);
            ctx.data.expressions.insert(tag.id, info);
        }
    }

    fn visit_attribute(&mut self, attr: &Attribute, _ctx: &mut VisitContext<'_, '_>) {
        set_pending_flags(attr, &mut self.pending_shorthand, &mut self.pending_clsx);
    }

    fn leave_concatenation_attribute(
        &mut self,
        attr: &svelte_ast::ConcatenationAttribute,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        merge_concat_expression_info(&attr.parts, attr.id, ctx);
    }

    fn leave_style_directive(
        &mut self,
        dir: &svelte_ast::StyleDirective,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        if let StyleDirectiveValue::Concatenation(parts) = &dir.value {
            merge_concat_expression_info(parts, dir.id, ctx);
        }
    }
}

// ---------------------------------------------------------------------------
// Extracted steps — each is a named function for flat visit_js_expression
// ---------------------------------------------------------------------------

/// Build a complete ExpressionInfo: analyze flags + collect resolved symbols in one pass.
pub(crate) fn build_expression_info(
    expr: &Expression<'_>,
    scoping: &mut ComponentScoping,
) -> ExpressionInfo {
    let mut info = js_analyze::analyze_expression(expr);
    info.set_ref_symbols(collect_ref_symbols(expr, scoping));
    info
}

/// Single OXC walk: collect resolved SymbolIds.
fn collect_ref_symbols(
    expr: &Expression<'_>,
    scoping: &mut ComponentScoping,
) -> SmallVec<[SymbolId; 2]> {
    collect_resolved_ref_symbols(scoping, |collector| {
        collector.visit_expression(expr);
    })
}

fn store_expression_info(node_id: NodeId, info: ExpressionInfo, ctx: &mut VisitContext<'_, '_>) {
    if ctx.parent().is_some_and(|p| p.kind.is_attr()) {
        ctx.data.attr_expressions.insert(node_id, info);
    } else {
        ctx.data.expressions.insert(node_id, info);
    }
}

fn classify_shorthand(
    _node_id: NodeId,
    expr: &Expression<'_>,
    pending: &mut Option<(NodeId, String)>,
    data: &mut AnalysisData,
) {
    if let Some((attr_id, name)) = pending.take() {
        if let Expression::Identifier(ident) = expr {
            if ident.name.as_str() == name {
                data.elements.flags.expression_shorthand.insert(attr_id);
            }
        }
    }
}

fn classify_clsx(
    node_id: NodeId,
    expr: &Expression<'_>,
    pending: &mut bool,
    data: &mut AnalysisData,
) {
    if !*pending {
        return;
    }
    *pending = false;
    if !matches!(
        expr,
        Expression::StringLiteral(_)
            | Expression::TemplateLiteral(_)
            | Expression::BinaryExpression(_)
    ) {
        data.elements.flags.needs_clsx.insert(node_id);
    }
}

fn classify_render_tag(
    node_id: NodeId,
    _expr: &Expression<'_>,
    info: &mut ExpressionInfo,
    pending: &mut Option<NodeId>,
    _data: &mut AnalysisData,
) {
    if pending.take() == Some(node_id) {
        info.mark_render_tag();
    }
}

fn set_pending_flags(
    attr: &Attribute,
    pending_shorthand: &mut Option<(NodeId, String)>,
    pending_clsx: &mut bool,
) {
    match attr {
        Attribute::ExpressionAttribute(ea) => {
            *pending_shorthand = Some((ea.id, ea.name.clone()));
            if ea.name == "class" {
                *pending_clsx = true;
            }
        }
        Attribute::ClassDirective(cd) => {
            // After the shorthand rework `expression_span` is non-optional.
            // The bitset still tracks "expression text matches attribute name"
            // which is resolved by `classify_shorthand` further down the
            // pass against the already-parsed Expression.
            *pending_shorthand = Some((cd.id, cd.name.clone()));
        }
        Attribute::StyleDirective(sd) => {
            if matches!(sd.value, StyleDirectiveValue::Expression) {
                *pending_shorthand = Some((sd.id, sd.name.clone()));
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Merge ExpressionInfo from individual concat part entries into a single entry
/// for the parent attribute/directive.
fn merge_concat_expression_info(
    parts: &[svelte_ast::ConcatPart],
    parent_id: NodeId,
    ctx: &mut VisitContext<'_, '_>,
) {
    let mut merged = ExpressionInfo::new(ExpressionKind::Other);
    for part in parts {
        if let svelte_ast::ConcatPart::Dynamic { id, .. } = part {
            if let Some(info) = ctx.data.attr_expressions.get(*id) {
                merged.merge_in(info);
            }
        }
    }
    // Keep the merged parent attr/directive as a separate reverse-reference site:
    // callers may care about the holder node even when its dynamic parts were
    // already recorded individually.
    ctx.data.attr_expressions.insert(parent_id, merged);
}

struct ResolvedRefCollector<'s, 'a> {
    scoping: &'s mut ComponentScoping<'a>,
    symbols: SmallVec<[SymbolId; 2]>,
}

impl<'a> Visit<'a> for ResolvedRefCollector<'_, '_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            if let Some(sym_id) = self.scoping.get_reference(ref_id).symbol_id() {
                if !self.symbols.contains(&sym_id) {
                    self.symbols.push(sym_id);
                }
            }
        }
    }
}

fn collect_resolved_ref_symbols(
    scoping: &mut ComponentScoping<'_>,
    visit: impl FnOnce(&mut ResolvedRefCollector<'_, '_>),
) -> SmallVec<[SymbolId; 2]> {
    let mut collector = ResolvedRefCollector {
        scoping,
        symbols: SmallVec::new(),
    };
    visit(&mut collector);
    collector.symbols
}
