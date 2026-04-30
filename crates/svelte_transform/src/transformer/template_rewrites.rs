use oxc_ast::ast::Expression;
use oxc_traverse::TraverseCtx;

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
        if t.dispatch_identifier_update(it) {
            return;
        }
        t.dispatch_member_update(it, ctx);
        return;
    }

    if matches!(it, Expression::AssignmentExpression(_)) {
        t.dispatch_member_assignment(it, false, ctx);
    }
}

pub(crate) fn rewrite_template_exit<'a>(
    t: &mut ComponentTransformer<'_, 'a>,
    it: &mut Expression<'a>,
) {
    t.rewrite_shared_call(it, false);

    let analysis = t.analysis.expect("Template mode requires analysis");

    if let Expression::AwaitExpression(await_expr) = it {
        let ignored = t.template_owner_node.is_some_and(|id| {
            analysis
                .output
                .ignore_data
                .is_ignored(id, "await_reactivity_loss")
        });
        let is_pickled = analysis.is_pickled_await(await_expr.span.start);

        let ast = t.b.ast;
        let arg = std::mem::replace(
            &mut await_expr.argument,
            ast.expression_identifier(oxc_span::SPAN, ast.atom("")),
        );

        if is_pickled {
            let save_call = t.make_dollar_call("save", arg);
            await_expr.argument = save_call;
            let Expression::AwaitExpression(_) = &*it else {
                unreachable!()
            };
            let awaited =
                std::mem::replace(it, ast.expression_identifier(oxc_span::SPAN, ast.atom("")));
            *it = ast.expression_call(oxc_span::SPAN, awaited, oxc_ast::NONE, ast.vec(), false);
            return;
        } else if t.dev && !ignored {
            let track_call = t.make_dollar_call("track_reactivity_loss", arg);
            await_expr.argument = track_call;
            let Expression::AwaitExpression(_) = &*it else {
                unreachable!()
            };
            let awaited =
                std::mem::replace(it, ast.expression_identifier(oxc_span::SPAN, ast.atom("")));
            *it = ast.expression_call(oxc_span::SPAN, awaited, oxc_ast::NONE, ast.vec(), false);
            return;
        } else {
            await_expr.argument = arg;
            return;
        }
    }

    t.rewrite_prop_update_ownership_exit(it);

    let suppress_proxy = t.in_bind_setter_traverse;
    t.dispatch_identifier_assignment(it, suppress_proxy);
}
