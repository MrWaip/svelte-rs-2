use super::*;

#[inline]
pub(crate) fn dispatch_expr(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Span,
    ctx: &mut VisitContext<'_, '_>,
) {
    for v in visitors.iter_mut() {
        v.visit_expression(id, span, ctx);
    }
    let parsed = ctx.parsed;
    if let Some(expr) = parsed
        .and_then(|p| p.expr_handle(span.start))
        .and_then(|h| parsed.and_then(|p| p.expr(h)))
    {
        for v in visitors.iter_mut() {
            v.visit_js_expression(id, expr, ctx);
        }
    }
    for v in visitors.iter_mut() {
        v.leave_expression(id, span, ctx);
    }
}

#[inline]
pub(crate) fn dispatch_opt_expr(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Option<Span>,
    ctx: &mut VisitContext<'_, '_>,
) {
    if let Some(span) = span {
        dispatch_expr(visitors, id, span, ctx);
    }
}

#[inline]
pub(crate) fn dispatch_stmt(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Span,
    ctx: &mut VisitContext<'_, '_>,
) {
    for v in visitors.iter_mut() {
        v.visit_statement(id, span, ctx);
    }
    let parsed = ctx.parsed;
    if let Some(stmt) = parsed
        .and_then(|p| p.stmt_handle(span.start))
        .and_then(|h| parsed.and_then(|p| p.stmt(h)))
    {
        for v in visitors.iter_mut() {
            v.visit_js_statement(id, stmt, ctx);
        }
    }
    for v in visitors.iter_mut() {
        v.leave_statement(id, span, ctx);
    }
}

#[inline]
pub(crate) fn dispatch_opt_stmt(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    span: Option<Span>,
    ctx: &mut VisitContext<'_, '_>,
) {
    if let Some(span) = span {
        dispatch_stmt(visitors, id, span, ctx);
    }
}

pub(crate) fn dispatch_concat_exprs(
    visitors: &mut [&mut dyn TemplateVisitor],
    parts: &[ConcatPart],
    ctx: &mut VisitContext<'_, '_>,
) {
    for part in parts {
        if let ConcatPart::Dynamic { id, span } = part {
            dispatch_expr(visitors, *id, *span, ctx);
        }
    }
}
