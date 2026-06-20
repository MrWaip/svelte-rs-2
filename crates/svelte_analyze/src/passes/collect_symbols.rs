use oxc_ast::ast::{Expression, IdentifierReference};
use oxc_ast_visit::Visit;
use svelte_ast::{Attribute, NodeId, RenderTag, StyleDirectiveValue};
use svelte_span::Span;

use crate::types::data::AnalysisData;
use crate::types::markers::ScopingBuilt;
use crate::walker::{TemplateVisitor, VisitContext};

pub(crate) fn make_visitor(_scoping: ScopingBuilt) -> CollectSymbolsVisitor {
    CollectSymbolsVisitor {
        pending_shorthand: None,
        pending_clsx: false,
    }
}

pub(crate) struct CollectSymbolsVisitor {
    pending_shorthand: Option<(NodeId, String)>,
    pending_clsx: bool,
}

impl TemplateVisitor for CollectSymbolsVisitor {
    fn visit_expression(&mut self, _node_id: NodeId, _span: Span, _ctx: &mut VisitContext<'_, '_>) {
    }

    fn visit_js_expression(
        &mut self,
        node_id: NodeId,
        expr: &Expression<'_>,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        if expression_uses_legacy_slots(expr) {
            ctx.data.output.needs_sanitized_legacy_slots = true;
        }
        classify_shorthand(node_id, expr, &mut self.pending_shorthand, ctx.data);
        classify_clsx(node_id, expr, &mut self.pending_clsx, ctx.data);
    }

    fn visit_render_tag(&mut self, _tag: &RenderTag, _ctx: &mut VisitContext<'_, '_>) {}

    fn visit_attribute(&mut self, attr: &Attribute, _ctx: &mut VisitContext<'_, '_>) {
        set_pending_flags(attr, &mut self.pending_shorthand, &mut self.pending_clsx);
    }
}

fn expression_uses_legacy_slots(expr: &Expression<'_>) -> bool {
    struct Probe(bool);
    impl<'a> Visit<'a> for Probe {
        fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
            if ident.name.as_str() == "$$slots" {
                self.0 = true;
            }
        }
    }
    let mut p = Probe(false);
    p.visit_expression(expr);
    p.0
}

fn classify_shorthand(
    _node_id: NodeId,
    expr: &Expression<'_>,
    pending: &mut Option<(NodeId, String)>,
    data: &mut AnalysisData,
) {
    if let Some((attr_id, name)) = pending.take()
        && let Expression::Identifier(ident) = expr.get_inner_expression()
        && ident.name.as_str() == name
    {
        data.elements.flags.expression_shorthand.insert(attr_id);
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
        expr.get_inner_expression(),
        Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::RegExpLiteral(_)
            | Expression::TemplateLiteral(_)
            | Expression::BinaryExpression(_)
    ) {
        data.elements.flags.needs_clsx.insert(node_id);
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
