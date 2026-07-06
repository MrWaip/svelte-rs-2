use oxc_ast::NONE;
use oxc_ast::ast::{Argument, Expression};
use oxc_span::SPAN;
use svelte_ast_builder::Builder;

pub fn is_simple_expression(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::Identifier(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_) => true,
        Expression::ParenthesizedExpression(inner) => is_simple_expression(&inner.expression),
        Expression::ConditionalExpression(cond) => {
            is_simple_expression(&cond.test)
                && is_simple_expression(&cond.consequent)
                && is_simple_expression(&cond.alternate)
        }
        Expression::BinaryExpression(bin) => {
            is_simple_expression(&bin.left) && is_simple_expression(&bin.right)
        }
        Expression::LogicalExpression(log) => {
            is_simple_expression(&log.left) && is_simple_expression(&log.right)
        }
        _ => false,
    }
}

pub fn dollar_member<'a>(b: &Builder<'a>, method: &str) -> Expression<'a> {
    let ast = b.ast;
    let object = ast.expression_identifier(SPAN, ast.atom("$"));
    let property = ast.identifier_name(SPAN, ast.atom(method));
    Expression::StaticMemberExpression(
        ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
    )
}

pub fn thunk_call<'a>(b: &Builder<'a>, name: &str) -> Expression<'a> {
    let ast = b.ast;
    let callee = ast.expression_identifier(SPAN, ast.atom(name));
    ast.expression_call(SPAN, callee, NONE, ast.vec(), false)
}

pub fn untrack_ident<'a>(b: &Builder<'a>, name: &str) -> Expression<'a> {
    let ast = b.ast;
    let callee = dollar_member(b, "untrack");
    let arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
    ast.expression_call(SPAN, callee, NONE, ast.vec1(arg), false)
}
