use oxc_ast::NONE;
use oxc_ast::ast::{Argument, Expression};
use oxc_span::SPAN;
use svelte_ast_builder::Builder;

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
