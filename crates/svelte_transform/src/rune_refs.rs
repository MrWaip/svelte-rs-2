//! Build rune-transformed AST nodes ($.get/$.set/$.update) using OXC AstBuilder.

use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast::{AstBuilder, NONE};
use oxc_span::SPAN;
use oxc_syntax::operator::AssignmentOperator;

/// Build a plain identifier expression for the given name.
/// Used by legacy `$$props` / `$$restProps` rewrites that swap one identifier for another.
pub fn make_ident<'a>(alloc: &'a Allocator, name: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    ast.expression_identifier(SPAN, ast.atom(name))
}

/// Build `$.get(name)` call expression.
pub fn make_rune_get<'a>(alloc: &'a Allocator, name: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "get");
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
    ast.expression_call(SPAN, callee, NONE, ast.vec1(name_arg), false)
}

/// Build `$.safe_get(name)` call expression.
/// Used instead of `$.get` for `var`-declared state, because `var` hoisting means the binding
/// may be read before the initializer runs and the signal is set.
pub fn make_rune_safe_get<'a>(alloc: &'a Allocator, name: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "safe_get");
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
        let delta =
            Argument::from(ast.expression_numeric_literal(SPAN, -1.0, None, NumberBase::Decimal));
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
    Expression::StaticMemberExpression(
        ast.alloc(ast.static_member_expression(SPAN, get_call, property, false)),
    )
}

/// Build `$$props.name` — static member expression.
pub fn make_props_access<'a>(alloc: &'a Allocator, prop_name: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let object = ast.expression_identifier(SPAN, ast.atom("$$props"));
    let property = ast.identifier_name(SPAN, ast.atom(prop_name));
    Expression::StaticMemberExpression(
        ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
    )
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
    ast.expression_call(
        SPAN,
        eager_callee,
        NONE,
        ast.vec1(Argument::from(pending_ref)),
        false,
    )
}

/// Build `() => expr`, with unthunk optimization for zero-arg calls to identifiers.
fn make_thunk<'a>(ast: &AstBuilder<'a>, expr: Expression<'a>) -> Expression<'a> {
    // Optimize `() => fn()` → `fn` when fn is a zero-arg call to an identifier/member
    if let Expression::CallExpression(call) = &expr {
        if call.arguments.is_empty()
            && !call.optional
            && matches!(
                &call.callee,
                Expression::Identifier(_) | Expression::StaticMemberExpression(_)
            )
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
    let body = ast.alloc(ast.function_body(
        SPAN,
        ast.vec(),
        ast.vec1(ast.statement_expression(SPAN, expr)),
    ));
    ast.expression_arrow_function(SPAN, true, false, NONE, params, NONE, body)
}

/// Build `$.store_set(base_name, value)` call expression.
pub fn make_store_set<'a>(
    alloc: &'a Allocator,
    base_name: &str,
    value: Expression<'a>,
) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "store_set");
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(base_name)));
    let value_arg = Argument::from(value);
    ast.expression_call(
        SPAN,
        callee,
        NONE,
        ast.vec_from_array([name_arg, value_arg]),
        false,
    )
}

/// Build `$.update_store(base_name, $dollar_name()[, -1])` or `$.update_pre_store(...)`.
pub fn make_store_update<'a>(
    alloc: &'a Allocator,
    base_name: &str,
    dollar_name: &str,
    is_prefix: bool,
    is_increment: bool,
) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let fn_name = if is_prefix {
        "update_pre_store"
    } else {
        "update_store"
    };
    let callee = make_dollar_member(&ast, fn_name);
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(base_name)));
    let thunk_call = make_thunk_call(alloc, dollar_name);
    let thunk_arg = Argument::from(thunk_call);

    let args = if is_increment {
        ast.vec_from_array([name_arg, thunk_arg])
    } else {
        let delta =
            Argument::from(ast.expression_numeric_literal(SPAN, -1.0, None, NumberBase::Decimal));
        ast.vec_from_array([name_arg, thunk_arg, delta])
    };

    ast.expression_call(SPAN, callee, NONE, args, false)
}

/// Build `$.untrack(name)` — wraps a thunk identifier for deep mutation root replacement.
pub fn make_untrack<'a>(alloc: &'a Allocator, dollar_name: &str) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "untrack");
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(dollar_name)));
    ast.expression_call(SPAN, callee, NONE, ast.vec1(name_arg), false)
}

/// Build `$.store_mutate(base_name, mutation, untracked)` call expression.
pub fn make_store_mutate<'a>(
    alloc: &'a Allocator,
    base_name: &str,
    mutation: Expression<'a>,
    untracked: Expression<'a>,
) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, "store_mutate");
    let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(base_name)));
    let mutation_arg = Argument::from(mutation);
    let untracked_arg = Argument::from(untracked);
    ast.expression_call(
        SPAN,
        callee,
        NONE,
        ast.vec_from_array([name_arg, mutation_arg, untracked_arg]),
        false,
    )
}

/// Expand a compound assignment operator into a binary/logical expression.
/// For plain `=`, returns `right` unchanged.
pub fn build_compound_value<'a>(
    alloc: &'a Allocator,
    operator: AssignmentOperator,
    left_read: Expression<'a>,
    right: Expression<'a>,
) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    if operator.is_assign() {
        return right;
    }
    if let Some(bin_op) = operator.to_binary_operator() {
        ast.expression_binary(SPAN, left_read, bin_op, right)
    } else if let Some(log_op) = operator.to_logical_operator() {
        ast.expression_logical(SPAN, left_read, log_op, right)
    } else {
        unreachable!("all compound assignment operators are either binary or logical")
    }
}

/// Check whether the value should be proxied (wrapped for reactivity tracking).
pub fn should_proxy(e: &Expression) -> bool {
    if e.is_literal() {
        return false;
    }
    if matches!(
        e,
        Expression::TemplateLiteral(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::BinaryExpression(_)
    ) {
        return false;
    }
    if let Expression::Identifier(id) = e {
        if id.name == "undefined" {
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Member chain helpers (shared between template transform and script codegen)
// ---------------------------------------------------------------------------

/// Walk an Expression member chain to find the root identifier name.
pub fn find_expr_root_name<'b>(expr: &'b Expression) -> Option<&'b str> {
    find_expr_root_identifier(expr).map(|id| id.name.as_str())
}

/// Walk an Expression member chain to find the root identifier reference node.
pub fn find_expr_root_identifier<'b, 'a>(
    expr: &'b Expression<'a>,
) -> Option<&'b oxc_ast::ast::IdentifierReference<'a>> {
    let mut current = expr;
    loop {
        match current {
            Expression::Identifier(id) => return Some(id),
            _ => current = current.as_member_expression()?.object(),
        }
    }
}

/// Replace the root identifier in an Expression member chain with a replacement.
pub fn replace_expr_root<'a>(expr: &mut Expression<'a>, replacement: Expression<'a>) {
    let mut current = expr;
    loop {
        if matches!(current, Expression::Identifier(_)) {
            *current = replacement;
            return;
        }
        match current.as_member_expression_mut() {
            Some(member) => current = member.object_mut(),
            None => return,
        }
    }
}

/// Replace root identifier in an AssignmentTarget member chain.
pub fn replace_expr_root_in_assign_target<'a>(
    target: &mut AssignmentTarget<'a>,
    replacement: Expression<'a>,
) {
    if let Some(member) = target.as_member_expression_mut() {
        replace_expr_root(member.object_mut(), replacement);
    }
}

/// Replace root identifier in a SimpleAssignmentTarget member chain.
pub fn replace_expr_root_in_simple_target<'a>(
    target: &mut SimpleAssignmentTarget<'a>,
    replacement: Expression<'a>,
) {
    if let Some(member) = target.as_member_expression_mut() {
        replace_expr_root(member.object_mut(), replacement);
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Build `$.method` static member expression (public for use in ExprTransformer).
pub(crate) fn make_dollar_member<'a>(ast: &AstBuilder<'a>, method: &str) -> Expression<'a> {
    let object = ast.expression_identifier(SPAN, ast.atom("$"));
    let property = ast.identifier_name(SPAN, ast.atom(method));
    Expression::StaticMemberExpression(
        ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
    )
}

/// Build `$.<method>(arg)` — single-argument call on the $-runtime object.
pub fn make_dollar_call<'a>(
    alloc: &'a Allocator,
    method: &str,
    arg: Expression<'a>,
) -> Expression<'a> {
    let ast = AstBuilder::new(alloc);
    let callee = make_dollar_member(&ast, method);
    ast.expression_call(SPAN, callee, NONE, ast.vec1(Argument::from(arg)), false)
}
