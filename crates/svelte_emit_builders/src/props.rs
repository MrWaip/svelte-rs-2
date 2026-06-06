use oxc_ast::ast::Expression;
use oxc_span::SPAN;
use svelte_ast_builder::Builder;

pub fn props_member<'a>(b: &Builder<'a>, prop_name: &str) -> Expression<'a> {
    let ast = b.ast;
    let object = ast.expression_identifier(SPAN, ast.atom("$$props"));
    let property = ast.identifier_name(SPAN, ast.atom(prop_name));
    Expression::StaticMemberExpression(
        ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
    )
}

pub fn props_computed_access<'a>(b: &Builder<'a>, prop_name: &str) -> Expression<'a> {
    let ast = b.ast;
    let object = ast.expression_identifier(SPAN, ast.atom("$$props"));
    let property = b.str_expr(prop_name);
    Expression::ComputedMemberExpression(
        ast.alloc(ast.computed_member_expression(SPAN, object, property, false)),
    )
}
