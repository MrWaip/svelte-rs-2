use super::*;
use svelte_ast::{ExprRef, StmtRef};

#[inline]
pub(crate) fn dispatch_expr(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    expr_ref: &ExprRef,
    ctx: &mut VisitContext<'_, '_>,
) {
    let span = expr_ref.span;
    for v in visitors.iter_mut() {
        v.visit_expression(id, span, ctx);
    }
    if let Some(expr) = ctx.parsed.and_then(|p| p.expr(expr_ref.id())) {
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
    expr_ref: Option<&ExprRef>,
    ctx: &mut VisitContext<'_, '_>,
) {
    if let Some(expr_ref) = expr_ref {
        dispatch_expr(visitors, id, expr_ref, ctx);
    }
}

#[inline]
pub(crate) fn dispatch_stmt(
    visitors: &mut [&mut dyn TemplateVisitor],
    id: NodeId,
    stmt_ref: &StmtRef,
    ctx: &mut VisitContext<'_, '_>,
) {
    let span = stmt_ref.span;
    for v in visitors.iter_mut() {
        v.visit_statement(id, span, ctx);
    }
    if let Some(stmt) = ctx.parsed.and_then(|p| p.stmt(stmt_ref.id())) {
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
    stmt_ref: Option<&StmtRef>,
    ctx: &mut VisitContext<'_, '_>,
) {
    if let Some(stmt_ref) = stmt_ref {
        dispatch_stmt(visitors, id, stmt_ref, ctx);
    }
}

pub(crate) fn dispatch_concat_exprs(
    visitors: &mut [&mut dyn TemplateVisitor],
    parts: &[ConcatPart],
    ctx: &mut VisitContext<'_, '_>,
) {
    for part in parts {
        if let ConcatPart::Dynamic { id, expr, .. } = part {
            dispatch_expr(visitors, *id, expr, ctx);
        }
    }
}
