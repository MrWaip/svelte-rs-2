use oxc_ast::NONE;
use oxc_ast::ast::{Argument, Expression};
use oxc_span::SPAN;
use svelte_ast_builder::Builder;

use crate::runtime::dollar_member;

pub fn rune_get<'a>(b: &Builder<'a>, name: &str) -> Expression<'a> {
    let ast = b.ast;
    let callee = dollar_member(b, "get");
    let arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
    ast.expression_call(SPAN, callee, NONE, ast.vec1(arg), false)
}

pub fn rune_safe_get<'a>(b: &Builder<'a>, name: &str) -> Expression<'a> {
    let ast = b.ast;
    let callee = dollar_member(b, "safe_get");
    let arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
    ast.expression_call(SPAN, callee, NONE, ast.vec1(arg), false)
}

pub fn rune_set<'a>(
    b: &Builder<'a>,
    name: &str,
    value: Expression<'a>,
    proxy: bool,
) -> Expression<'a> {
    let ast = b.ast;
    let callee = dollar_member(b, "set");
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
    let value_arg = Argument::from(value);
    if proxy {
        let true_arg = Argument::from(ast.expression_boolean_literal(SPAN, true));
        ast.expression_call(
            SPAN,
            callee,
            NONE,
            ast.vec_from_array([name_arg, value_arg, true_arg]),
            false,
        )
    } else {
        ast.expression_call(
            SPAN,
            callee,
            NONE,
            ast.vec_from_array([name_arg, value_arg]),
            false,
        )
    }
}

pub fn member_get_via_get<'a>(
    b: &Builder<'a>,
    signal_name: &str,
    prop: &str,
) -> Expression<'a> {
    let ast = b.ast;
    let get_call = rune_get(b, signal_name);
    let property = ast.identifier_name(SPAN, ast.atom(prop));
    Expression::StaticMemberExpression(
        ast.alloc(ast.static_member_expression(SPAN, get_call, property, false)),
    )
}
