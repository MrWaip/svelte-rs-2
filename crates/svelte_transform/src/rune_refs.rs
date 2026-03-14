//! Build rune-transformed AST nodes ($.get/$.set/$.update) using OXC AstBuilder.

use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast::{AstBuilder, NONE};
use oxc_span::SPAN;

/// Build `$.get(name)` call expression.
pub fn make_rune_get<'a>(alloc: &'a Allocator, name: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "get");
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
    ast.expression_call(SPAN, callee, NONE, ast.vec1(name_arg), false)
}

/// Build `$.set(name, value)` call expression.
pub fn make_rune_set<'a>(
    alloc: &'a Allocator,
    name: &str,
    value: Expression<'a>,
) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "set");
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
    let value_arg = Argument::from(value);
    ast.expression_call(SPAN, callee, NONE, ast.vec_from_array([name_arg, value_arg]), false)
}

/// Build `$.update(name)` or `$.update_pre(name)` call expression.
pub fn make_rune_update<'a>(
    alloc: &'a Allocator,
    name: &str,
    is_prefix: bool,
    is_increment: bool,
) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let fn_name = if is_prefix { "update_pre" } else { "update" };
    let callee = make_dollar_member(&ast, fn_name);
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));

    let args = if is_increment {
        ast.vec1(name_arg)
    } else {
        let delta = Argument::from(ast.expression_numeric_literal(SPAN, -1.0, None, NumberBase::Decimal));
        ast.vec_from_array([name_arg, delta])
    };

    ast.expression_call(SPAN, callee, NONE, args, false)
}

/// Build `name()` — thunk call for snippet params and prop sources.
pub fn make_thunk_call<'a>(alloc: &'a Allocator, name: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = ast.expression_identifier(SPAN, ast.atom(name));
    ast.expression_call(SPAN, callee, NONE, ast.vec(), false)
}

/// Build `$$props.name` — static member expression.
pub fn make_props_access<'a>(alloc: &'a Allocator, prop_name: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let object = ast.expression_identifier(SPAN, ast.atom("$$props"));
    let property = ast.identifier_name(SPAN, ast.atom(prop_name));
    Expression::StaticMemberExpression(ast.alloc(
        ast.static_member_expression(SPAN, object, property, false),
    ))
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Build `$.method` static member expression.
fn make_dollar_member<'a>(ast: &AstBuilder<'a>, method: &str) -> Expression<'a> {
    let object = ast.expression_identifier(SPAN, ast.atom("$"));
    let property = ast.identifier_name(SPAN, ast.atom(method));
    Expression::StaticMemberExpression(ast.alloc(
        ast.static_member_expression(SPAN, object, property, false),
    ))
}
