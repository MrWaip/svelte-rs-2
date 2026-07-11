use std::mem;

use oxc_ast::{NONE, ast::Expression};
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;

use super::equals::wrap_binary_equals_dev;
use super::model::ComponentTransformer;

pub(crate) fn rewrite_template_enter<'a>(
    t: &mut ComponentTransformer<'_, 'a>,
    it: &mut Expression<'a>,
    is_lhs: bool,
    ctx: &mut TraverseCtx<'a, ()>,
) {
    if t.rewrite_rest_prop_member(it, is_lhs) {
        return;
    }

    if matches!(it, Expression::Identifier(_)) {
        t.dispatch_identifier_read(it);
        return;
    }

    if matches!(it, Expression::UpdateExpression(_)) {
        if t.dispatch_identifier_update(it, ctx) {
            return;
        }
        t.dispatch_member_update(it, ctx);
        return;
    }

    if matches!(it, Expression::AssignmentExpression(_)) {
        if super::state_legacy::is_destructure_assignment_lhs(it) {
            t.destructure_lhs_depth += 1;
        }
        t.dispatch_member_assignment(it, false, ctx);
    }
}

pub(crate) fn rewrite_template_exit<'a>(
    t: &mut ComponentTransformer<'_, 'a>,
    it: &mut Expression<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) {
    t.rewrite_call_expression(it);

    if t.dev
        && let Some(replacement) = t.transform_console_log(it)
    {
        *it = replacement;
        return;
    }

    let analysis = t.analysis.expect("Template mode requires analysis");

    if let Expression::AwaitExpression(await_expr) = it {
        let ignored = t.template_owner_node.is_some_and(|id| {
            analysis
                .output
                .ignore_data
                .is_ignored(id, "await_reactivity_loss")
        });
        let is_pickled = analysis.pickled_awaits.contains(await_expr.node_id());

        let ast = t.b.ast;
        let arg = mem::replace(
            &mut await_expr.argument,
            ast.expression_identifier(SPAN, ast.atom("")),
        );

        if is_pickled {
            let save_call = t.make_dollar_call("save", arg);
            await_expr.argument = save_call;
            let Expression::AwaitExpression(_) = &*it else {
                unreachable!()
            };
            let awaited = mem::replace(it, ast.expression_identifier(SPAN, ast.atom("")));
            *it = ast.expression_call(SPAN, awaited, NONE, ast.vec(), false);
            return;
        } else if t.dev && !ignored {
            let track_call = t.make_dollar_call("track_reactivity_loss", arg);
            await_expr.argument = track_call;
            let Expression::AwaitExpression(_) = &*it else {
                unreachable!()
            };
            let awaited = mem::replace(it, ast.expression_identifier(SPAN, ast.atom("")));
            *it = ast.expression_call(SPAN, awaited, NONE, ast.vec(), false);
            return;
        } else {
            await_expr.argument = arg;
            return;
        }
    }

    if t.rewrite_destructure_assignment_exit(it, ctx) {
        return;
    }

    t.rewrite_prop_update_ownership_exit(it);

    t.dispatch_identifier_assignment(it, ctx);

    if t.dev {
        wrap_binary_equals_dev(t.b, it);
    }
}
