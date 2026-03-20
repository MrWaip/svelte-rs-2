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

/// Build `$.set(name, value[, true])` call expression.
/// If `proxy` is true, adds a third `true` argument.
pub fn make_rune_set<'a>(
    alloc: &'a Allocator,
    name: &str,
    value: Expression<'a>,
    proxy: bool,
) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "set");
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
    let value_arg = Argument::from(value);
    if proxy {
        let true_arg = Argument::from(ast.expression_boolean_literal(SPAN, true));
        ast.expression_call(SPAN, callee, NONE, ast.vec_from_array([name_arg, value_arg, true_arg]), false)
    } else {
        ast.expression_call(SPAN, callee, NONE, ast.vec_from_array([name_arg, value_arg]), false)
    }
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

/// Build `$.get(signal_name).prop` — member access on a derived signal.
pub fn make_member_get<'a>(alloc: &'a Allocator, signal_name: &str, prop: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let get_call = make_rune_get(alloc, signal_name);
    let property = ast.identifier_name(SPAN, ast.atom(prop));
    Expression::StaticMemberExpression(ast.alloc(
        ast.static_member_expression(SPAN, get_call, property, false),
    ))
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

/// Build `$.eager(() => expr)` — wrap the argument in a thunk and call $.eager.
/// Applies unthunk optimization: `() => fn()` → `fn` when fn is a zero-arg call to an identifier.
pub fn make_eager_thunk<'a>(alloc: &'a Allocator, arg: Expression<'a>) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "eager");
    let thunked = make_thunk(&ast, arg);
    ast.expression_call(SPAN, callee, NONE, ast.vec1(Argument::from(thunked)), false)
}

/// Build `$.eager($.pending)` — thunk-optimized form of `$.eager(() => $.pending())`.
pub fn make_eager_pending<'a>(alloc: &'a Allocator) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let eager_callee = make_dollar_member(&ast, "eager");
    // $.pending (identifier, not call — thunk optimization of () => $.pending())
    let pending_ref = make_dollar_member(&ast, "pending");
    ast.expression_call(SPAN, eager_callee, NONE, ast.vec1(Argument::from(pending_ref)), false)
}

/// Build `() => expr`, with unthunk optimization for zero-arg calls to identifiers.
fn make_thunk<'a>(ast: &AstBuilder<'a>, expr: Expression<'a>) -> Expression<'a> {
    // Optimize `() => fn()` → `fn` when fn is a zero-arg call to an identifier/member
    if let Expression::CallExpression(call) = &expr {
        if call.arguments.is_empty()
            && !call.optional
            && matches!(&call.callee, Expression::Identifier(_) | Expression::StaticMemberExpression(_))
        {
            // Return just the callee
            if let Expression::CallExpression(call) = expr {
                return call.unbox().callee;
            }
        }
    }
    // Build `() => expr` — expression-body arrow function
    let params = ast.alloc_formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        ast.vec(),
        NONE,
    );
    let body = ast.alloc(ast.function_body(SPAN, ast.vec(), ast.vec1(
        ast.statement_expression(SPAN, expr),
    )));
    ast.expression_arrow_function(SPAN, true, false, NONE, params, NONE, body)
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
