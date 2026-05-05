use oxc_ast::ast::{BinaryOperator, Expression};

use svelte_ast_builder::{Arg, Builder};

pub(crate) fn wrap_binary_equals_dev<'a>(b: &Builder<'a>, expr: &mut Expression<'a>) -> bool {
    let Expression::BinaryExpression(bin) = expr else {
        return false;
    };
    let (callee, with_false) = match bin.operator {
        BinaryOperator::StrictEquality => ("$.strict_equals", false),
        BinaryOperator::StrictInequality => ("$.strict_equals", true),
        BinaryOperator::Equality => ("$.equals", false),
        BinaryOperator::Inequality => ("$.equals", true),
        _ => return false,
    };
    let left = b.move_expr(&mut bin.left);
    let right = b.move_expr(&mut bin.right);
    if with_false {
        *expr = b.call_expr(
            callee,
            [Arg::Expr(left), Arg::Expr(right), Arg::Bool(false)],
        );
    } else {
        *expr = b.call_expr(callee, [Arg::Expr(left), Arg::Expr(right)]);
    }
    true
}
