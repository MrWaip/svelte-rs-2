use oxc_ast::ast::*;

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
    if let Expression::Identifier(id) = e
        && id.name == "undefined"
    {
        return false;
    }
    true
}

pub fn find_expr_root_name<'b>(expr: &'b Expression) -> Option<&'b str> {
    find_expr_root_identifier(expr).map(|id| id.name.as_str())
}

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

pub fn replace_expr_root_in_assign_target<'a>(
    target: &mut AssignmentTarget<'a>,
    replacement: Expression<'a>,
) {
    if let Some(member) = target.as_member_expression_mut() {
        replace_expr_root(member.object_mut(), replacement);
    }
}

pub fn replace_expr_root_in_simple_target<'a>(
    target: &mut SimpleAssignmentTarget<'a>,
    replacement: Expression<'a>,
) {
    if let Some(member) = target.as_member_expression_mut() {
        replace_expr_root(member.object_mut(), replacement);
    }
}
